use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use glium::backend::Facade;
use glium::texture::CompressedTexture2d;

use super::CharAtlas;
use super::Label;
use super::text_program;
use crate::assets::FONT_PNG;
use crate::assets::FONT_PNG_DIMENSIONS;
use crate::assets::FONT_TILESIZE;
use crate::assets::FONTMAP_TXT;
use crate::assets::load_png_as_texture;
use crate::display::Mesh;
use crate::display::text::text_mesh;
use crate::display::text::text_uniforms;
use crate::mat4;
use crate::uvec2;

//////////////////
// TextRenderer //
//////////////////

#[derive(Debug)]
pub struct TextRenderer {
    program: Program,
    options: DrawParameters<'static>,
    font_tex: CompressedTexture2d,
    atlas: CharAtlas,
}

impl TextRenderer {
    pub fn new(gl: &impl Facade) -> crate::Result<Self> {
        let program = text_program(gl)?;
        let options = DrawParameters { ..Default::default() };
        let font_tex = load_png_as_texture(gl, FONT_PNG)?;
        let atlas = CharAtlas::from_font_map(FONTMAP_TXT);

        Ok(TextRenderer { program, options, font_tex, atlas })
    }

    pub fn draw(
        &self,
        gl: &impl Facade,
        frame: &mut impl Surface,
        label: &Label,
    ) -> crate::Result {
        let Label {
            text: string,
            top_left,
            size: tile_size,
            color,
            background,
        } = *label;

        let atlas_size = FONT_PNG_DIMENSIONS.as_vec2();
        let glyph_size = FONT_TILESIZE.as_vec2();

        let frame_size: uvec2 = frame.get_dimensions().into(); // px
        let frame_size = frame_size.as_vec2();

        let Mesh { vertices, indices } = text_mesh(
            gl,
            string,
            &self.atlas,
            atlas_size,
            frame_size,
            glyph_size,
            tile_size,
            top_left,
        )?;

        let right = frame_size.x as f32; // TODO: Insert DP transform
        let top = frame_size.y as f32;
        let proj = mat4::orthographic_rh_gl(0.0, right, 0.0, top, -1.0, 1.0);

        let uniforms = text_uniforms(&self.font_tex, proj, color, background);

        frame.draw(
            &vertices,
            indices,
            &self.program,
            &uniforms,
            &self.options,
        )?;
        // nothing (FOR NOW)
        // TODO: Draw some text
        Ok(())
    }
}
