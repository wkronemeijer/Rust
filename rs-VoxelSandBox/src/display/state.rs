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
use crate::mat4;

/////////////////
// RenderState //
/////////////////

pub struct Renderer {
    program: Program,
    options: DrawParameters<'static>,
    mesh: Option<ChunkMesh>,
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
        let mesh = None;
        let terrain = load_terrain_png(gl)?;

        Ok(Renderer { program, options, mesh, terrain })
    }

    fn should_remesh_chunk(&self, _: &Chunk) -> bool {
        // TODO: Hash the contents of the chunk?
        // Or use a dirty flag
        self.mesh.is_none()
    }

    pub fn update(&mut self, gl: &impl Facade, game: &Game) -> crate::Result {
        // TODO: Store a queue of
        let chunk = &game.world.chunk;
        if self.should_remesh_chunk(chunk) {
            self.mesh = Some(chunk_mesh(gl, chunk)?);
        }
        Ok(())
    }

    pub fn draw(
        &self,
        frame: &mut impl Surface,
        view: mat4,
        projection: mat4,
    ) -> crate::Result {
        let Some(Mesh { vertices, indices }) = &self.mesh else {
            return Ok(());
        };
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
        Ok(())
    }
}
