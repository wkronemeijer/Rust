use glium::backend::Facade;
use glium::implement_vertex;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::Program;
use glium::VertexBuffer;

const VERTEX_SHADER: &str = include_str!("shader.vert");
const FRAGMENT_SHADER: &str = include_str!("shader.frag");

#[derive(Debug, Clone, Copy)]
pub struct VertexAttributes {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}
implement_vertex!(VertexAttributes, pos, tex);

pub fn block_program(display: &impl Facade) -> crate::Result<Program> {
    Ok(Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None)?)
}

fn screen_quad_vertices() -> Vec<VertexAttributes> {
    let top_right = VertexAttributes { pos: [1.0, 1.0], tex: [1.0, 1.0] };
    let bottom_right = VertexAttributes { pos: [1.0, -1.0], tex: [1.0, 0.0] };
    let bottom_left = VertexAttributes { pos: [-1.0, -1.0], tex: [0.0, 0.0] };
    let top_left = VertexAttributes { pos: [-1.0, 1.0], tex: [0.0, 1.0] };
    // Default winding order is supposedly CCW, yet this still shows up
    // Isn't culling on by default as well?
    // idk
    vec![top_right, bottom_right, bottom_left, bottom_left, top_left, top_right]
}

pub fn screen_quad(
    display: &impl Facade,
) -> (VertexBuffer<VertexAttributes>, NoIndices) {
    let vertices = VertexBuffer::new(display, &screen_quad_vertices())
        .expect("create vertex buffer");
    let indices = NoIndices(PrimitiveType::TrianglesList);
    (vertices, indices)
}
