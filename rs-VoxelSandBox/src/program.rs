use std::default::Default;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::f32::consts::TAU;

use anyhow::Context;
use glam::EulerRot;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::texture::CompressedTexture2d;
use glium::Depth;
use glium::DepthTest;
use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::event::ElementState::Pressed;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::Fullscreen;
use winit::window::Window;
use winit::window::WindowId;

use crate::assets::load_terrain_png;
use crate::display::shader::chunk_mesh;
use crate::display::shader::chunk_program;
use crate::display::shader::chunk_uniforms;
use crate::display::shader::ChunkMesh;
use crate::display::Mesh;
use crate::domain::world::World;
use crate::manifest::APPLICATION_NAME;
use crate::mat3;
use crate::mat4;
use crate::vec3;

////////////
// Camera //
////////////

/// By default looks down +Y
struct Camera {
    position: vec3,
    /// + is left, - is right
    yaw: f32,
    /// + is up, - is down
    pitch: f32,
    // no roll!
}

impl Camera {
    pub fn new() -> Self {
        Camera { position: vec3::ZERO, pitch: 0.0, yaw: 0.0 }
    }

    pub fn view(&self) -> mat4 {
        let Camera { yaw, pitch, .. } = *self;
        let dir = mat3::from_euler(EulerRot::ZXY, yaw, pitch, 0.0) * vec3::Y;
        mat4::look_to_rh(self.position, dir, vec3::Z)
    }

    fn position(&self) -> vec3 { self.position }

    fn change_position(&mut self, delta: vec3) { self.position += delta; }

    /// Rotates movement so it is in accordance with an unrotated camera. So with:
    /// * yaw == 0, it goes through unchanged  
    /// * yaw == &tau;/4 it is rotated to the left  
    /// * yaw == &tau;/2 it goes in reverse  
    fn change_rotated_position(&mut self, delta: vec3) {
        self.position += mat3::from_rotation_z(self.yaw) * delta;
    }

    fn yaw(&self) -> f32 { self.yaw }

    fn change_yaw(&mut self, delta: f32) {
        self.yaw = (self.yaw + delta).rem_euclid(TAU);
    }

    fn pitch(&self) -> f32 { self.pitch }

    fn change_pitch(&mut self, delta: f32) {
        self.pitch = (self.pitch + delta).clamp(-FRAC_PI_2, FRAC_PI_2);
    }
}

///////////
// Input //
///////////

struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,

    pub rotate_up: bool,
    pub rotate_down: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
}

/////////////////
// Application //
/////////////////

struct Application {
    window: Window,
    display: Display<WindowSurface>,
    last_cursor: PhysicalPosition<f64>,

    program: Program,
    options: DrawParameters<'static>,

    mesh: ChunkMesh,
    texture: CompressedTexture2d,

    #[expect(dead_code, reason = "just rendering for now")]
    world: World,
    camera: Camera,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let last_cursor = PhysicalPosition { x: 0.0, y: 0.0 };

        let program = chunk_program(&display)?;
        let options = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let world = World::new();
        let camera = Camera::new();

        let mesh = chunk_mesh(&world.chunk, &display)?;
        let texture = load_terrain_png(&display)?;
        Ok(Application {
            window,
            display,
            last_cursor,
            program,
            options,
            mesh,
            texture,
            world,
            camera,
        })
    }

    fn projection(&self) -> mat4 {
        let fov_y_radians = 90.0 * PI / 180.0;
        let inner_size = self.window.inner_size();
        let aspect_ratio = inner_size.width as f32 / inner_size.height as f32;
        let z_near = 0.001;
        let z_far = 1000.0;

        mat4::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far)
    }

    pub fn draw(&self) -> crate::Result {
        let mut frame = self.display.draw();

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);

        let Mesh { vertices, indices } = &self.mesh;
        let model = mat4::IDENTITY; // just in place (for now)
        let view = self.camera.view();
        let projection = self.projection();
        let mvp = projection * view * model;

        let uniforms = chunk_uniforms(&self.texture, &mvp);
        frame.draw(
            vertices,
            indices,
            &self.program,
            &uniforms,
            &self.options,
        )?;

        Ok(frame.finish()?)
    }

    pub fn tick(&mut self) {
        self.world.tick();
        // update camera pos
    }
}

impl ApplicationHandler for Application {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        use WindowEvent::*;
        match event {
            RedrawRequested => self.draw().expect("drawing failed"),
            Resized(inner_size) => self.display.resize(inner_size.into()),
            CursorMoved { position, .. } => {
                let PhysicalPosition { x: old_x, y: old_y } = self.last_cursor;
                let PhysicalPosition { x: new_x, y: new_y } = &position;
                let delta = (new_x - old_x, new_y - old_y);
                println!("cursor moved: {delta:?}");
                self.last_cursor = position;
            }
            KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => {
                    match (code, event.state) {
                        // Game input
                        (KeyCode::KeyW, Pressed) => {
                            self.camera.change_rotated_position(vec3::Y);
                        }
                        (KeyCode::KeyA, Pressed) => {
                            self.camera.change_rotated_position(vec3::NEG_X);
                        }
                        (KeyCode::KeyS, Pressed) => {
                            self.camera.change_rotated_position(vec3::NEG_Y);
                        }
                        (KeyCode::KeyD, Pressed) => {
                            self.camera.change_rotated_position(vec3::X);
                        }
                        (KeyCode::KeyE | KeyCode::Space, Pressed) => {
                            self.camera.change_position(vec3::Z);
                        }
                        (KeyCode::KeyQ | KeyCode::KeyC, Pressed) => {
                            self.camera.change_position(vec3::NEG_Z);
                        }

                        (KeyCode::ArrowUp, Pressed) => {
                            self.camera.change_pitch(PI / 16.0);
                        }
                        (KeyCode::ArrowDown, Pressed) => {
                            self.camera.change_pitch(-PI / 16.0);
                        }
                        (KeyCode::ArrowLeft, Pressed) => {
                            self.camera.change_yaw(PI / 16.0);
                        }
                        (KeyCode::ArrowRight, Pressed) => {
                            self.camera.change_yaw(-PI / 16.0);
                        }
                        // Debug
                        #[cfg(debug_assertions)]
                        (KeyCode::F8, Pressed) => {
                            // [Stop debugging] is mapped to F8 on my setup
                            event_loop.exit();
                        }
                        (KeyCode::F10, Pressed) => {
                            println!("TODO: toggle menu")
                        }
                        (KeyCode::F11, Pressed) => self.window.set_fullscreen(
                            match self.window.fullscreen() {
                                None => Some(Fullscreen::Borderless(None)),
                                Some(_) => None,
                            },
                        ),
                        _ => {}
                    };
                    if let Pressed = event.state {
                        println!("current position = {}", self.camera.position)
                    }
                }
                _ => {}
            },
            CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn resumed(&mut self, _: &ActiveEventLoop) {
        println!("<resumed>");
    }
}

/////////////
// Run fun //
/////////////

type WindowSize = PhysicalSize<u32>;

const TITLE: &str = APPLICATION_NAME;
const INITIAL_SIZE: WindowSize = WindowSize::new(800, 600);

pub fn run() -> crate::Result {
    println!("initializing...");

    let event_loop = EventLoop::builder()
        .build()
        .with_context(|| "event loop construction")?;

    let (window, display) = SimpleWindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(INITIAL_SIZE.width, INITIAL_SIZE.height)
        .build(&event_loop);

    let ref mut app = Application::new(window, display)?;

    println!("initialization complete");
    println!("starting event loop...");

    // (Why doesn't ↓ take &mut self?)
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(app)?;

    println!("exited event loop");
    Ok(())
}
