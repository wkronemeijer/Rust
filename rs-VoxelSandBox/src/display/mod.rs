//! Contains all code primarily concerned with drawing to a display.

pub mod chunk;
pub mod screen;
pub mod text;

use std::f32::consts::PI;

use glium::Surface;
use glium::backend::Facade;

use self::chunk::renderer::ChunkRenderer;
use self::text::Label;
use self::text::renderer::TextRenderer;
use crate::camera::Camera;
use crate::domain::game::Game;
use crate::mat4;

//////////
// Mesh //
//////////

#[derive(Debug)]
pub struct Mesh<V, I> {
    pub vertices: V,
    pub indices: I,
}

//////////////////
// App Renderer //
//////////////////

pub struct AppRenderer {
    pub chunk: ChunkRenderer,
    pub text: TextRenderer,
}

pub const FOV_Y_RADIANS: f32 = 90.0 * PI / 180.0;

impl AppRenderer {
    pub fn new(gl: &impl Facade) -> crate::Result<Self> {
        Ok(AppRenderer {
            chunk: ChunkRenderer::new(gl)?,
            text: TextRenderer::new(gl)?,
        })
    }

    /// Prepares for a draw. This is the time to update meshes.
    pub fn pre_draw(&mut self, gl: &impl Facade, game: &Game) -> crate::Result {
        self.chunk.pre_draw(gl, game)?;
        Ok(())
    }

    fn projection(&self, (width, height): (u32, u32)) -> mat4 {
        let aspect_ratio = (width as f32) / (height as f32);
        let z_near = 0.001;
        let z_far = 1000.0;

        mat4::perspective_rh_gl(FOV_Y_RADIANS, aspect_ratio, z_near, z_far)
    }

    pub fn draw_world(
        &self,
        frame: &mut impl Surface,
        camera: &Camera,
    ) -> crate::Result {
        let projection = self.projection(frame.get_dimensions());
        let view = camera.view();
        self.chunk.draw(frame, view, projection)
    }

    pub fn draw_text(
        &self,
        frame: &mut impl Surface,
        label: Label,
    ) -> crate::Result {
        self.text.draw(frame, label)
    }
}
