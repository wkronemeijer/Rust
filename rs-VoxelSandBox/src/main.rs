#![forbid(unsafe_code)]

use anyhow::Context;
use glium::backend::glutin::SimpleWindowBuilder;
use voxelsandbox::NAME;
use voxelsandbox::Result;
use voxelsandbox::app::Application;
use winit::dpi::PhysicalSize;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

/////////
// Run //
/////////

type WindowSize = PhysicalSize<u32>;
const TITLE: &str = NAME;
const INITIAL_SIZE: WindowSize = WindowSize::new(800, 600);

fn run() -> crate::Result {
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

//////////
// Main //
//////////

fn main() -> Result {
    // In the future we could parse CLI options here
    let name = NAME;
    println!("starting {name}");
    run()?;
    println!("exited {name} successfully");
    Ok(())
}
