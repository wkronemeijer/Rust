use std::default::Default;

use anyhow::Context;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::texture::CompressedTexture2d;
use glium::uniform;
use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use winit::application::ApplicationHandler;
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
use crate::manifest::APPLICATION_NAME;
use crate::render::shader::chunk_program;
use crate::render::shader::screen_mesh;
use crate::render::shader::screen_program;
use crate::render::shader::ScreenVertex;
use crate::render::Mesh;

/////////////////
// Application //
/////////////////

struct Application {
    window: Window,
    display: Display<WindowSurface>,

    program: Program,
    options: DrawParameters<'static>,

    vertices: VertexBuffer<ScreenVertex>,
    indices: NoIndices,

    terrain_tex: CompressedTexture2d,
}

impl Application {
    pub fn new(
        window: Window,
        display: Display<WindowSurface>,
    ) -> crate::Result<Self> {
        let program = screen_program(&display)?;
        let options = DrawParameters { ..Default::default() };
        let Mesh { vertices, indices } = screen_mesh(&display)?;
        let terrain_tex = load_terrain_png(&display)?;
        Ok(Application {
            window,
            display,
            program,
            options,
            vertices,
            indices,
            terrain_tex,
        })
    }

    pub fn draw(&self) -> crate::Result {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        let uniforms = uniform! { tex: &self.terrain_tex };
        frame.draw(
            &self.vertices,
            &self.indices,
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
                PhysicalKey::Code(code) => match (code, event.state) {
                    (KeyCode::Escape, Pressed) => {
                        println!("TODO: close menus (don't toggle pause menu)");
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
                },
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
