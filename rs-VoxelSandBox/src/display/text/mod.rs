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

use crate::assets::FONT_TILESIZE;
use crate::core::fused_shader::split_shader;
use crate::display::Mesh;
use crate::mat4;
use crate::uvec2;
use crate::vec2;
use crate::vec4;

#[derive(Debug, Clone, Copy)]
pub struct TextVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}
implement_vertex!(TextVertex, pos, tex);

pub fn text_uniforms<'a>(
    font: &'a CompressedTexture2d,
    projection: mat4,
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
        projection: projection.to_cols_array_2d(),
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
#[derive(Debug, Clone)]
pub struct Label<'a> {
    /// The text to draw.
    pub text: &'a str,
    /// Where to start drawing.
    pub top_left: vec2,
    /// Height of the bounding box in **pixels**.
    pub size: vec2,
    /// Letter color. Defaults to white.
    pub color: vec4,
    /// Background color. Defaults to None, which is fully transparent.
    pub background: vec4,
}

impl<'a> Label<'a> {
    fn new(text: &'a str) -> Self {
        Label {
            text,
            top_left: vec2(0.0, 0.0),
            size: FONT_TILESIZE.as_vec2() * 2.0,
            color: vec4::ONE,       // Opaqua white
            background: vec4::ZERO, // Transparent black
        }
    }
}

impl Default for Label<'static> {
    fn default() -> Self { Label::new("") }
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

pub fn text_mesh(
    gl: &impl Facade,
    string: &str,
    atlas: &CharAtlas,
    atlas_size: vec2,
    frame_size: vec2,
    glyph_size: vec2,
    tile_size: vec2,
    text_top_left: vec2,
) -> crate::Result<TextMesh> {
    let mut vertices = Vec::<TextVertex>::new();

    let mut push_vertex = |xy: vec2, uv: vec2| {
        vertices.push(TextVertex { pos: xy.into(), tex: uv.into() });
    };

    let mut push_quad = |c: char, top_left: vec2| {
        /*
        Reference:
        S---R
        |  /|
        | / |
        |/  |
        P---Q
        */

        let tex_s = atlas.cell(c).as_vec2() * glyph_size + vec2::Y * atlas_size;
        let tex_r = tex_s + glyph_size * vec2::X;
        let tex_p = tex_s + glyph_size * vec2::Y;
        let tex_q = tex_r + glyph_size * vec2::Y;

        let pos_s = top_left;
        let pos_r = pos_s + tile_size * vec2::X;
        let pos_p = pos_s - tile_size * vec2::Y;
        let pos_q = pos_r - tile_size * vec2::Y;

        push_vertex(pos_p, tex_p / atlas_size);
        push_vertex(pos_q, tex_q / atlas_size);
        push_vertex(pos_r, tex_r / atlas_size);

        push_vertex(pos_r, tex_r / atlas_size);
        push_vertex(pos_s, tex_s / atlas_size);
        push_vertex(pos_p, tex_p / atlas_size);
    };

    let mut position = vec2(text_top_left.x, frame_size.y - text_top_left.y);
    let step = vec2::X * tile_size.x;

    for c in string.chars() {
        push_quad(c, position);
        position += step;
    }

    let vertices = VertexBuffer::immutable(gl, &vertices)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    Ok(Mesh { vertices, indices })
}
