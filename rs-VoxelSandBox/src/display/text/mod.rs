pub mod renderer;

use std::collections::btree_map::BTreeMap;
use std::collections::btree_map::Entry::Occupied;
use std::collections::btree_map::Entry::Vacant;

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
use crate::uvec2;
use crate::vec2;
use crate::vec3;
use crate::vec4;

#[derive(Debug, Clone, Copy)]
pub struct TextVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}
implement_vertex!(TextVertex, pos, tex);

pub fn text_uniforms<'a>(
    font: &'a CompressedTexture2d,
    color: vec4,
    background: vec4,
) -> impl Uniforms {
    let font = Sampler(font, SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    });
    uniform! {
        font: font,
        color: color.to_array(),
        background: background.to_array()
    }
}

pub fn text_program(gl: &impl Facade) -> crate::Result<Program> {
    println!("compiling 'text.glsl'");
    split_shader(include_str!("text.glsl"))?.into_program(gl)
}

///////////
// Label //
///////////

/// Wrapper containing all styling information to draw a string.
#[derive(Debug)]
pub struct Label<'a> {
    /// The text to draw.
    pub text: &'a str,
    /// Where to start drawing.
    pub top_left: vec2,
    /// "Size" of the label, which refers to the height of the bounding box.
    pub size: f32,
    /// Letter color. Defaults to white.
    pub color: vec3,
    /// Background color. Defaults to None, which is fully transparent.
    pub background: Option<vec3>,
}

impl<'a> Default for Label<'a> {
    fn default() -> Self {
        Self {
            text: "",
            top_left: vec2(0.0, 0.0),
            size: 0.1,
            color: vec3(1.0, 1.0, 1.0),
            background: None,
        }
    }
}

///////////////
// CharAtlas //
///////////////

/// Maps chars to uvec2
#[derive(Debug)]
pub struct CharAtlas {
    map: BTreeMap<char, uvec2>,
}

impl CharAtlas {
    fn new() -> Self { CharAtlas { map: BTreeMap::new() } }

    fn insert_pos(&mut self, c: char, x: u32, y: u32) {
        self.map.insert(c, uvec2(x, y));
    }

    pub fn from_font_map(font_map: &str) -> Self {
        let mut result = Self::new();
        let mut x = 0;
        let mut y = 0;
        for c in font_map.chars() {
            if matches!(c, '\n') {
                y += 1;
                x = 0;
            } else {
                match result.map.entry(c) {
                    Occupied(_) => {},
                    Vacant(entry) => {
                        entry.insert(uvec2(x, y));
                    },
                }
                x += 1;
            }
        }
        result
    }

    pub fn cell(&self, c: char) -> uvec2 {
        self.map.get(&c).copied().unwrap_or_default()
    }
}

//////////////
// TextMesh //
//////////////

pub type TextMesh = Mesh<VertexBuffer<TextVertex>, NoIndices>;

/// Generates a mesh for entire chunk.
/// Maps (0,0,0) to (0f,0f,0f), so still needs a model transform.
pub fn text_mesh(
    gl: &impl Facade,
    string: &str,
    _step: u32,
) -> crate::Result<TextMesh> {
    let ref mut vertices = Vec::<TextVertex>::new();

    let mut push_char = |c: char| {};

    string.chars().for_each(push_char);

    let vertices = VertexBuffer::new(gl, &vertices)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    Ok(Mesh { vertices, indices })
}
