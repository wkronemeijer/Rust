use anyhow::Context;
use anyhow::Ok;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::uniform;
use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use raytracer::algebra::Size;
use raytracer::backend::compile_program;
use raytracer::backend::render_to_texture;
use raytracer::backend::screen_quad;
use raytracer::backend::Vertex;
use raytracer::tracer::DrRayTracer;
use raytracer::tracer::Raytracer;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use winit::window::Fullscreen;
use winit::window::Window;
use winit::window::WindowId;
use ElementState::Pressed;

///////////
// Stuff //
///////////

const TITLE: &str = "Dr. Ray Tracer";
const INITIAL_SIZE: Size = Size::new(800, 600);

/////////////////
// Application //
/////////////////
/// Some genius decided to deprecate [EventLoop::run]...

struct Application<'a> {
    window: &'a Window,
    display: &'a Display<WindowSurface>,
    tracer: &'a dyn Raytracer,
    size: Size,

    program: Program,
    options: DrawParameters<'static>,
    vertices: VertexBuffer<Vertex>,
    indices: NoIndices,

    texture: glium::Texture2d,
    draw_new_frame: bool,
}

impl<'a> Application<'a> {
    pub fn new(
        window: &'a Window,
        display: &'a Display<WindowSurface>,
        tracer: &'a dyn Raytracer,
        size: Size,
    ) -> Self {
        let program = compile_program(display);
        let (vertices, indices) = screen_quad(display);
        let options = DrawParameters { ..Default::default() };
        let texture = render_to_texture(display, tracer, size);
        let draw_new_frame = true;
        Application {
            window,
            display,
            program,
            options,
            size,
            tracer,
            vertices,
            indices,
            texture,
            draw_new_frame,
        }
    }

    pub fn draw(&mut self) {
        let Application {
            display, vertices, indices, program, options, ..
        } = self;
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        if self.draw_new_frame {
            self.texture = render_to_texture(*display, self.tracer, self.size);
            self.draw_new_frame = false;
        }
        let uniforms = uniform! { tex: &self.texture };
        frame
            .draw(&*vertices, &*indices, &program, &uniforms, &options)
            .expect("drawing");
        frame.finish().expect("finish frame");
    }
}

impl<'a> ApplicationHandler<()> for Application<'a> {
    fn resumed(&mut self, _: &ActiveEventLoop) {} // no-op

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // TODO: Should we request a redraw here?
        self.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        use WindowEvent::*;
        match event {
            RedrawRequested => self.draw(),
            Resized(PhysicalSize { width, height }) => {
                self.display.resize((width, height));
                self.size = Size { width, height };
            }
            KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(code) => match (code, event.state) {
                    (KeyCode::Space, Pressed) => self.draw_new_frame = true,
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
            CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

//////////
// Main //
//////////

fn render(tracer: &dyn Raytracer) -> anyhow::Result<()> {
    println!("initializing...");

    let size = INITIAL_SIZE;

    let event_loop = EventLoop::builder()
        .build()
        .with_context(|| "event loop construction")?;
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(size.width, size.height)
        .build(&event_loop);

    let ref mut app = Application::new(&window, &display, tracer, size);

    println!("initialization complete");
    println!("starting event loop...");
    event_loop.run_app(app)?;
    println!("event loop terminated");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    render(&DrRayTracer)?;
    Ok(())
}
