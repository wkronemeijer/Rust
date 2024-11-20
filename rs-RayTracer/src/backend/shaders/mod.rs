use glium::backend::Facade;
use glium::implement_vertex;
use glium::Program;

const VERTEX_SHADER: &str = include_str!("shader.vert");
const FRAGMENT_SHADER: &str = include_str!("shader.frag");

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub fn compile_program(display: &impl Facade) -> Program {
    Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None)
        .expect("create program")
}
