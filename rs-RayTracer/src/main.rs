use anyhow::{Context, Ok};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::{uniform, DrawParameters, Surface};
use raytracer::algebra::Size;
use raytracer::backend::{program, render_to_texture, screen_quad};
use raytracer::tracer::{DrRayTracer, Raytracer};
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::keyboard::{KeyCode, PhysicalKey};

///////////
// Stuff //
///////////

const TITLE: &str = "Dr. Ray Tracer";
const INITIAL_SIZE: Size = Size::new(800, 600);

///////////
// Stuff //
///////////

fn render(tracer: &impl Raytracer) -> anyhow::Result<()> {
    println!("Initializing...");

    let mut size = INITIAL_SIZE;

    let event_loop = EventLoopBuilder::new()
        .build()
        .with_context(|| "event loop construction")?;
    let (window, display) = SimpleWindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(size.width, size.height)
        .build(&event_loop);

    let (vertices, indices) = screen_quad(&display);
    let program = program(&display);
    let options = DrawParameters { ..Default::default() };

    let mut texture = render_to_texture(&display, tracer, size);
    let mut draw_new_frame = false;

    println!("Starting event loop...");
    event_loop.run(move |event, target| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::RedrawRequested => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                if draw_new_frame {
                    texture = render_to_texture(&display, tracer, size);
                    draw_new_frame = false;
                }
                let uniforms = uniform! { tex: &texture };
                frame
                    .draw(&vertices, &indices, &program, &uniforms, &options)
                    .expect("drawing");
                frame.finish().expect("finish frame");
            }
            WindowEvent::Resized(new_size) => {
                display.resize(new_size.into());
                size = Size {
                    width: new_size.width,
                    height: new_size.height,
                }
            }
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::Space) => {
                    if event.state == ElementState::Pressed {
                        draw_new_frame = true;
                    }
                }
                _ => {}
            },
            WindowEvent::CloseRequested => {
                target.exit();
            }
            _ => {}
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => {}
    })?;
    println!("Exiting...");
    Ok(())
}

//////////
// Main //
//////////

fn main() -> anyhow::Result<()> {
    render(&DrRayTracer)?;
    Ok(())
}
