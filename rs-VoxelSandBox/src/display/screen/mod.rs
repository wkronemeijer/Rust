use glium::Program;
use glium::VertexBuffer;
use glium::backend::Facade;
use glium::implement_vertex;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::texture::CompressedTexture2d;
use glium::uniform;
use glium::uniforms::MagnifySamplerFilter;
use glium::uniforms::MinifySamplerFilter;
use glium::uniforms::Sampler;
use glium::uniforms::SamplerBehavior;
use glium::uniforms::Uniforms;

use crate::core::fused_shader::split_shader;
use crate::display::Mesh;

const SCREEN_SHADER: &str = include_str!("screen.glsl");

#[derive(Debug, Copy, Clone)]
pub struct ScreenVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}
implement_vertex!(ScreenVertex, pos, tex);

pub fn screen_uniforms<'a>(tex: &'a CompressedTexture2d) -> impl Uniforms {
    let sampler_options = SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };
    uniform! { tex: Sampler(tex, sampler_options) }
}

pub fn screen_program(gl: &impl Facade) -> crate::Result<Program> {
    println!("compiling 'screen.glsl'");
    split_shader(SCREEN_SHADER)?.into_program(gl)
}

pub type ScreenMesh = Mesh<VertexBuffer<ScreenVertex>, NoIndices>;

pub fn screen_mesh(gl: &impl Facade) -> crate::Result<ScreenMesh> {
    let top_right = ScreenVertex { pos: [1.0, 1.0], tex: [1.0, 1.0] };
    let bottom_right = ScreenVertex { pos: [1.0, -1.0], tex: [1.0, 0.0] };
    let bottom_left = ScreenVertex { pos: [-1.0, -1.0], tex: [0.0, 0.0] };
    let top_left = ScreenVertex { pos: [-1.0, 1.0], tex: [0.0, 1.0] };
    let vertices = VertexBuffer::new(gl, &[
        top_right,
        bottom_right,
        bottom_left,
        bottom_left,
        top_left,
        top_right,
    ])?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    Ok(Mesh { vertices, indices })
}
