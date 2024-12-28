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
use crate::domain::game::Game;
use crate::domain::world::World;
use crate::domain::world::WorldToChunkIndex;
use crate::mat4;

/////////////////
// RenderState //
/////////////////

const REMESH_PER_UPDATE_LIMIT: i32 = 2;

pub struct Renderer {
    program: Program,
    options: DrawParameters<'static>,
    terrain_png: CompressedTexture2d,
    chunk_meshes: HashMap<WorldToChunkIndex, ChunkMesh>,
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
        let terrain_png = load_terrain_png(gl)?;
        let chunk_meshes = HashMap::new();
        Ok(Renderer { program, options, terrain_png, chunk_meshes })
    }

    fn should_remesh_chunk(
        &self,
        world: &World,
        idx: WorldToChunkIndex,
    ) -> bool {
        // TODO: Hash the contents of the chunk? or use a dirty flag?
        world.get_chunk(idx).is_some() && !self.chunk_meshes.contains_key(&idx)
    }

    pub fn clear_cache(&mut self) { self.chunk_meshes.clear(); }

    /// Prepares for a draw. This is the time to update meshes.
    pub fn pre_draw(&mut self, gl: &impl Facade, game: &Game) -> crate::Result {
        let world = &game.world;

        let mut remeshed_count = 0;
        for chunk_idx in game.world.chunk_size().span() {
            if self.should_remesh_chunk(world, chunk_idx) {
                let mesh = chunk_mesh(gl, world, chunk_idx)?;
                self.chunk_meshes.insert(chunk_idx, mesh);
                remeshed_count += 1;
                if remeshed_count >= REMESH_PER_UPDATE_LIMIT {
                    break;
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
        frame.clear_depth(1.0);
        for (idx, Mesh { vertices, indices }) in self.chunk_meshes.iter() {
            let chunk_origin = idx.world_origin();
            let model = mat4::from_translation(chunk_origin);
            let mvp = projection * view * model;

            let uniforms = chunk_uniforms(&self.terrain_png, &mvp);

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
