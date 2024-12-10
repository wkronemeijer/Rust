use std::default::Default;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::f32::consts::TAU;
use std::time::Instant;

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
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event::ElementState;
use winit::event::ElementState::Pressed;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::CursorGrabMode;
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
use crate::domain::SECONDS_PER_TICK;
use crate::domain::TICK_DURATION;
use crate::manifest::APPLICATION_NAME;
use crate::mat3;
use crate::mat4;
use crate::vec3;

////////////
// Camera //
////////////

/// By default looks down +Y
#[derive(Debug)]
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

    fn yaw(&self) -> f32 { self.yaw }

    fn pitch(&self) -> f32 { self.pitch }

    fn change_position(&mut self, delta: vec3) { self.position += delta; }

    /// Rotates movement so it is in accordance with an unrotated camera. So with:
    /// * yaw == 0, it goes through unchanged  
    /// * yaw == &tau;/4 it is rotated to the left  
    /// * yaw == &tau;/2 it goes in reverse  
    fn change_rotated_position(&mut self, wish_displacement: vec3) {
        self.position += mat3::from_rotation_z(self.yaw) * wish_displacement;
    }

    fn change_yaw(&mut self, delta: f32) {
        self.yaw = (self.yaw + delta).rem_euclid(TAU);
    }

    // -ε to prevent flipping the camera
    const MAX_PITCH: f32 = FRAC_PI_2 - f32::EPSILON;
    const MIN_PITCH: f32 = -Self::MAX_PITCH;

    fn change_pitch(&mut self, delta: f32) {
        self.pitch =
            (self.pitch + delta).clamp(Self::MIN_PITCH, Self::MAX_PITCH);
    }
}

///////////
// Input //
///////////
// TODO: Split between "continous" input checked every frame (like moving forward)
// And "event" input anywhere during the frame (like jumping)

#[derive(Debug, Default, Clone)]
struct InputState {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,

    pub rotate_up: bool,
    pub rotate_down: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
}

impl InputState {
    pub fn new() -> Self { Default::default() }

    /// Updates state based on pressed key.
    /// Returns [`Some`] if state was updated, [`None`] if the key was not recognized.
    pub fn process(&mut self, key: KeyCode, state: ElementState) -> Option<()> {
        use KeyCode::*;

        let is_pressed = matches!(state, Pressed);
        Some(match key {
            KeyW => self.move_forward = is_pressed,
            KeyS => self.move_backward = is_pressed,
            KeyA => self.move_left = is_pressed,
            KeyD => self.move_right = is_pressed,
            KeyE => self.move_up = is_pressed,
            KeyQ => self.move_down = is_pressed,

            ArrowUp => self.rotate_up = is_pressed,
            ArrowDown => self.rotate_down = is_pressed,
            ArrowLeft => self.rotate_left = is_pressed,
            ArrowRight => self.rotate_right = is_pressed,

            _ => return None,
        })
    }
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

    world: World,
    camera: Camera,

    input: InputState,

    tick_no: u64,
    last_tick: Instant,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let last_cursor = PhysicalPosition { x: 0.0, y: 0.0 };

        let world = World::new();
        let camera = Camera::new();
        let input = InputState::new();

        let program = chunk_program(&display)?;
        let options = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
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
            input,
            tick_no: 1,
            last_tick: Instant::now(),
        })
    }
}

// Drawing logic
impl Application {
    fn projection(&self) -> mat4 {
        let fov_y_radians = 90.0 * PI / 180.0;
        let inner_size = self.window.inner_size();
        let aspect_ratio = inner_size.width as f32 / inner_size.height as f32;
        let z_near = 0.001;
        let z_far = 1000.0;

        mat4::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far)
    }

    pub fn draw(&self) -> crate::Result {
        // I need to put it somewhere else

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
}

// Tick logic
impl Application {
    pub fn projected_next_tick(&self) -> Instant {
        self.last_tick + TICK_DURATION
    }

    /// Tries to tick at most once.
    pub fn try_tick(&mut self) {
        let now = Instant::now();
        if (now - self.last_tick) >= TICK_DURATION {
            self.tick();
            self.last_tick = now;
            self.tick_no += 1;
        }
    }

    const PLAYER_UNITS_PER_SECOND: f32 = 5.0;

    pub fn tick(&mut self) {
        println!("tick #{}", self.tick_no);

        self.world.tick();
        // TODO: Do we do self.camera.tick()?
        // Or tie it to an entity
        // Or both and add optional detach for debugging

        //////////////
        // Movement //
        //////////////

        let mut wishdir = vec3::ZERO;
        if self.input.move_forward {
            wishdir += vec3::Y;
        }
        if self.input.move_backward {
            wishdir -= vec3::Y;
        }
        if self.input.move_right {
            wishdir += vec3::X;
        }
        if self.input.move_left {
            wishdir -= vec3::X;
        }
        if self.input.move_up {
            wishdir += vec3::Z;
        }
        if self.input.move_down {
            wishdir -= vec3::Z;
        }
        wishdir = wishdir.normalize_or_zero(); // TODO: insert player speed
        wishdir *= Self::PLAYER_UNITS_PER_SECOND * SECONDS_PER_TICK;

        self.camera.change_rotated_position(wishdir);

        //////////////
        // Rotation //
        //////////////

        const VSENS: f32 = PI / 16.0;
        const HSENS: f32 = PI / 16.0;

        if self.input.rotate_left {
            self.camera.change_yaw(HSENS);
        }
        if self.input.rotate_right {
            self.camera.change_yaw(-HSENS);
        }
        if self.input.rotate_up {
            self.camera.change_pitch(VSENS);
        }
        if self.input.rotate_down {
            self.camera.change_pitch(-VSENS);
        }
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
            /////////////
            // Drawing //
            /////////////
            RedrawRequested => {
                self.try_tick();
                self.draw().expect("drawing failed");
            }
            Resized(inner_size) => self.display.resize(inner_size.into()),
            ///////////
            // Input //
            ///////////
            CursorMoved { position, .. } => {
                let PhysicalPosition { x: old_x, y: old_y } = self.last_cursor;
                let PhysicalPosition { x: new_x, y: new_y } = &position;
                let delta = (new_x - old_x, new_y - old_y);
                println!("cursor moved: {delta:?}");
                self.last_cursor = position;
            }
            KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => match (code, event.state) {
                    ///////////
                    // Debug //
                    ///////////
                    (KeyCode::Escape, Pressed) => {
                        if let Err(e) =
                            self.window.set_cursor_grab(CursorGrabMode::None)
                        {
                            eprintln!("failed to set cursor: {e}");
                        }
                    }
                    #[cfg(debug_assertions)]
                    (KeyCode::F8, Pressed) => event_loop.exit(),
                    (KeyCode::F10, Pressed) => {
                        println!("TODO: toggle menu")
                    }
                    (KeyCode::F11, Pressed) => self.window.set_fullscreen(
                        match self.window.fullscreen() {
                            None => Some(Fullscreen::Borderless(None)),
                            Some(_) => None,
                        },
                    ),
                    _ => {
                        if let None = self.input.process(code, event.state) {
                            println!("warning: ignoring {code:?}");
                        }
                    }
                },
                PhysicalKey::Unidentified(c) => {
                    println!("warning: unidentified key {c:?}");
                }
            },
            CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        use DeviceEvent::*;
        match event {
            MouseMotion { delta: (dx, dy) } => {
                println!("device event: {dx} {dy} under {device_id:?}")
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
