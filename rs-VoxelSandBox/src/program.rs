use std::default::Default;
use std::f32::consts::PI;

use anyhow::Context;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::texture::CompressedTexture2d;
use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::ElementState::Pressed;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::Key;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::Fullscreen;
use winit::window::Window;
use winit::window::WindowId;

use crate::assets::load_terrain_png;
use crate::manifest::APPLICATION_NAME;
use crate::mat4;
use crate::render::shader::chunk_mesh;
use crate::render::shader::chunk_program;
use crate::render::shader::chunk_uniforms;
use crate::render::shader::ChunkMesh;
use crate::render::Mesh;
use crate::vec3;
use crate::world::World;

/////////////////
// Application //
/////////////////

struct Application {
    window: Window,
    display: Display<WindowSurface>,

    program: Program,
    options: DrawParameters<'static>,

    mesh: ChunkMesh,
    terrain: CompressedTexture2d,

    world: World,
    position: vec3,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let program = chunk_program(&display)?;
        let options = DrawParameters { ..Default::default() };
        let world = World::new();

        let mesh = chunk_mesh(&world.chunk, &display)?;
        let terrain = load_terrain_png(&display)?;
        let position = vec3(10.0, 10.0, 10.0);
        Ok(Application {
            window,
            display,
            program,
            options,
            mesh,
            terrain,
            world,
            position,
        })
    }

    pub fn draw(&self) -> crate::Result {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        let Mesh { vertices, indices } = &self.mesh;

        let model = mat4::IDENTITY; // just in place for now
        let view = mat4::look_at_rh(self.position, vec3::ZERO, vec3::Z);

        let fov_y_radians = 90.0 * PI / 180.0;
        let inner_size = self.window.inner_size();
        let aspect_ratio = inner_size.width as f32 / inner_size.height as f32;
        let z_near = 0.001;
        let z_far = 1000.0;
        let projection =
            mat4::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far);

        let mvp = projection * view * model;

        let uniforms = chunk_uniforms(&self.terrain, &mvp);

        frame.draw(
            vertices,
            indices,
            &self.program,
            &uniforms,
            &self.options,
        )?;
        frame.finish()?;
        Ok(())
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
            KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => {
                    match (code, event.state) {
                        (KeyCode::Escape, Pressed) => {
                            println!(
                                "TODO: close menus (don't toggle pause menu)"
                            );
                        }
                        //
                        (KeyCode::KeyW, Pressed) => {
                            self.position += vec3::Y;
                        }
                        (KeyCode::KeyA, Pressed) => {
                            self.position += vec3::NEG_X;
                        }
                        (KeyCode::KeyS, Pressed) => {
                            self.position += vec3::NEG_Y;
                        }
                        (KeyCode::KeyD, Pressed) => {
                            self.position += vec3::X;
                        }
                        (KeyCode::KeyE, Pressed) => {
                            self.position += vec3::Z;
                        }
                        (KeyCode::KeyC, Pressed) => {
                            self.position += vec3::NEG_Z;
                        }

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
                    println!("pos = {}", self.position)
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
