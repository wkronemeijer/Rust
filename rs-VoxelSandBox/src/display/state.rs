use std::collections::HashMap;

use glium::Surface;
use glium::backend::Facade;
use glium::draw_parameters::BackfaceCullingMode;
use glium::draw_parameters::Depth;
use glium::draw_parameters::DepthTest;
use glium::draw_parameters::DrawParameters;
use glium::program::Program;
use glium::texture::CompressedTexture2d;

use super::Mesh;
use super::shader::ChunkMesh;
use super::shader::chunk_mesh;
use super::shader::chunk_program;
use super::shader::chunk_uniforms;
use crate::assets::load_terrain_png;
use crate::domain::chunk::Chunk;
use crate::domain::game::Game;
use crate::domain::world::WorldToChunkIndex;
use crate::mat4;

/////////////////
// RenderState //
/////////////////

pub struct Renderer {
    program: Program,
    options: DrawParameters<'static>,
    chunk_meshes: HashMap<WorldToChunkIndex, ChunkMesh>,
    terrain: CompressedTexture2d,
}

impl Renderer {
    pub fn new(gl: &impl Facade) -> crate::Result<Self> {
        let program = chunk_program(gl)?;
        let options = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        let mesh = HashMap::new();
        let terrain = load_terrain_png(gl)?;

        Ok(Renderer { program, options, chunk_meshes: mesh, terrain })
    }

    fn should_remesh_chunk(&self, index: WorldToChunkIndex, _: &Chunk) -> bool {
        // TODO: Hash the contents of the chunk?
        // Or use a dirty flag
        !self.chunk_meshes.contains_key(&index)
    }

    pub fn update(&mut self, gl: &impl Facade, game: &Game) -> crate::Result {
        // TODO: Store a queue of chunks that need to be remeshed
        for chunk_idx in game.world.size.iter() {
            if let Some(chunk) = game.world.get(chunk_idx) {
                if self.should_remesh_chunk(chunk_idx, chunk) {
                    self.chunk_meshes.insert(chunk_idx, chunk_mesh(gl, chunk)?);
                }
            }
        }
        Ok(())
    }

    pub fn draw(
        &self,
        frame: &mut impl Surface,
        view: mat4,
        projection: mat4,
    ) -> crate::Result {
        for Mesh { vertices, indices } in self.chunk_meshes.values() {
            let model = mat4::IDENTITY;
            let mvp = projection * view * model;

            let uniforms = chunk_uniforms(&self.terrain, &mvp);

            frame.clear_depth(1.0);
            frame.draw(
                vertices,
                indices,
                &self.program,
                &uniforms,
                &self.options,
            )?;
        }
        Ok(())
    }
}
