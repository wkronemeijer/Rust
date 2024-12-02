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
use glium::Program;
use glium::VertexBuffer;

use crate::assets::TERRAIN_PNG_PIXEL_DIM;
use crate::ivec3;
use crate::mat4;
use crate::render::Mesh;
use crate::vec2;
use crate::vec3;
use crate::world::chunk::Chunk;
use crate::world::tile::Tile;

const VERTEX_SHADER: &str = include_str!("chunk.vert");
const FRAGMENT_SHADER: &str = include_str!("chunk.frag");

#[derive(Debug, Clone, Copy)]
pub struct ChunkVertex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
}
implement_vertex!(ChunkVertex, pos, tex);

pub fn chunk_uniforms<'a>(
    texture: &'a CompressedTexture2d,
    mvp: &'a mat4,
) -> impl Uniforms + use<'a> {
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
    println!("compiling chunk program");
    Ok(Program::from_source(gl, VERTEX_SHADER, FRAGMENT_SHADER, None)?)
}

pub type ChunkMesh = Mesh<VertexBuffer<ChunkVertex>, NoIndices>;

fn chunk_pos(pos: ivec3) -> vec3 {
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

// TODO: Use indices
// A cube is: 8 vertices, 10 edges, 6 faces/quads
// So 6 * 2 * 3 = 36 tri's (vs 8 verts + 36 indices)
// 36 --> 8 sounds like a good saving
fn add_block_vertices(
    pos: ivec3,
    tile: &Tile,
    vertices: &mut Vec<ChunkVertex>,
) {
    let mut push = |xyz: vec3, uv: vec2| {
        vertices.push(ChunkVertex { pos: xyz.into(), tex: uv.into() });
    };

    let mut push_quad = |v1: vec3,
                         v2: vec3,
                         v3: vec3,
                         v4: vec3,
                         t1: vec2,
                         t2: vec2,
                         t3: vec2,
                         t4: vec2| {
        push(v1, t1);
        push(v2, t2);
        push(v3, t3);
        push(v3, t3);
        push(v4, t4);
        push(v1, t1);
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
    let a = chunk_pos(pos);
    let b = a + vec3::X;
    let c = b + vec3::Y;
    let d = a + vec3::Y;
    let e = a + vec3::Z;
    let f = b + vec3::Z;
    let g = c + vec3::Z;
    let h = d + vec3::Z;

    let p = tile_uv(tile);
    let q = p + TILE_UV_STEP * vec2::X;
    let r = q + TILE_UV_STEP * vec2::Y;
    let s = p + TILE_UV_STEP * vec2::Y;

    push_quad(d, c, b, a, p, q, r, s); // bottom
    push_quad(e, f, g, h, p, q, r, s); // top
    push_quad(a, b, f, e, p, q, r, s); // front
    push_quad(b, c, g, f, p, q, r, s); // right
    push_quad(c, d, h, g, p, q, r, s); // back
    push_quad(d, a, e, h, p, q, r, s); // left
}

/// Generates a mesh for entire chunk.
/// Maps (0,0,0) to (0f,0f,0f), so still needs a model transform.
pub fn chunk_mesh(chunk: &Chunk, gl: &impl Facade) -> crate::Result<ChunkMesh> {
    let ref mut vertices = Vec::<ChunkVertex>::new();
    chunk.for_each_tile(|ipos, tile| {
        // TODO: check nearby tiles are all opaque
        if tile.is_visible() {
            add_block_vertices(ipos, tile, vertices);
        }
    });
    let vertices = VertexBuffer::new(gl, &vertices)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    Ok(Mesh { vertices, indices })
}
