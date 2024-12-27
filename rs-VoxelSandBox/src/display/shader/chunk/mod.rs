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

use crate::assets::TERRAIN_PNG_PIXEL_DIM;
use crate::display::Mesh;
use crate::display::shader::split_shader;
use crate::domain::chunk::Chunk;
use crate::domain::face::Face;
use crate::domain::handles::ChunkToTileIndex;
use crate::domain::tile::Tile;
use crate::ivec3;
use crate::mat4;
use crate::vec2;
use crate::vec3;

const CHUNK_SHADER: &str = include_str!("chunk.glsl");

#[derive(Debug, Clone, Copy)]
pub struct ChunkVertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
    pub light: f32,
}
implement_vertex!(ChunkVertex, pos, tex, light);

pub fn chunk_uniforms<'a>(
    texture: &'a CompressedTexture2d,
    mvp: &'a mat4,
) -> impl Uniforms {
    let sampler = Sampler(texture, SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    });
    let matrix_array = mvp.to_cols_array_2d();
    uniform! {
        tex: sampler,
        mvp: matrix_array,
    }
}

pub fn chunk_program(gl: &impl Facade) -> crate::Result<Program> {
    println!("compiling 'chunk.glsl'");
    split_shader(CHUNK_SHADER)?.into_program(gl)
}

pub type ChunkMesh = Mesh<VertexBuffer<ChunkVertex>, NoIndices>;

fn chunk_pos(pos: ChunkToTileIndex) -> vec3 {
    let x = pos.x as f32;
    let y = pos.y as f32;
    let z = pos.z as f32;
    vec3(x, y, z)
}

const TILE_PIXEL_DIM: f32 = 8.0;
const TILE_UV_STEP: f32 = TILE_PIXEL_DIM / (TERRAIN_PNG_PIXEL_DIM as f32);

/// Returns the bottom left corner of the tiles UV
fn tile_uv(tile: &Tile) -> vec2 {
    // OpenGL (0,0) as UV is bottom left
    // Tile tex_index 0 is top left
    let index = tile.tex_index();
    let x = (index % TERRAIN_PNG_PIXEL_DIM) as f32;
    let y = (index / TERRAIN_PNG_PIXEL_DIM) as f32;
    vec2(x * TILE_UV_STEP, 1.0 - y * TILE_UV_STEP)
}

fn get_light_level(face: Face) -> f32 {
    // as if the Sun was high up in the south
    match face {
        Face::Up => 1.0,
        Face::South => 0.9,
        Face::West => 0.8,
        Face::East => 0.7,
        Face::North => 0.6,
        Face::Down => 0.5,
    }
}

fn add_block_vertices(
    verts: &mut Vec<ChunkVertex>,
    tile: &Tile,
    pos: ChunkToTileIndex,
) {
    let mut push_tri = |xyz: vec3, uv: vec2, l: f32| {
        verts.push(ChunkVertex { pos: xyz.into(), tex: uv.into(), light: l });
    };

    /*
    Model:
               +Z
                |
                |
       H--------G
      /|       /|
     / |      / | +Y
    E--------F  | /
    |  |     |  |/
    |  D-----|--C
    | /      | /
    |/       |/
    A--------B--+X
    Texture:
    S---R
    |   |
    |   |
    |   |
    P---Q

    */
    let p = tile_uv(tile);
    let q = p + TILE_UV_STEP * vec2::X;
    let r = q + TILE_UV_STEP * vec2::Y;
    let s = p + TILE_UV_STEP * vec2::Y;

    let mut push_quad = |f: Face, vp: vec3, vq: vec3, vr: vec3, vs: vec3| {
        let l = get_light_level(f);
        push_tri(vp, p, l);
        push_tri(vq, q, l);
        push_tri(vr, r, l);
        push_tri(vr, r, l);
        push_tri(vs, s, l);
        push_tri(vp, p, l);
    };

    let a = chunk_pos(pos);
    let b = a + vec3::X;
    let c = b + vec3::Y;
    let d = a + vec3::Y;
    let e = a + vec3::Z;
    let f = b + vec3::Z;
    let g = c + vec3::Z;
    let h = d + vec3::Z;

    push_quad(Face::Up, e, f, g, h);
    push_quad(Face::Down, d, c, b, a);
    push_quad(Face::North, c, d, h, g);
    push_quad(Face::South, a, b, f, e);
    push_quad(Face::West, d, a, e, h);
    push_quad(Face::East, b, c, g, f);
}

/// Generates a mesh for entire chunk.
/// Maps (0,0,0) to (0f,0f,0f), so still needs a model transform.
pub fn chunk_mesh(gl: &impl Facade, chunk: &Chunk) -> crate::Result<ChunkMesh> {
    let ref mut vertices = Vec::<ChunkVertex>::new();
    chunk.for_each_tile(|pos, tile| {
        // TODO: check nearby tiles are all opaque
        if tile.is_visible() {
            add_block_vertices(vertices, tile, pos);
        }
    });
    let vertices = VertexBuffer::new(gl, &vertices)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    Ok(Mesh { vertices, indices })
}
