use std::collections::HashMap;

use glium::DrawParameters;
use glium::Program;
use glium::Surface;
use glium::backend::Facade;
use glium::texture::CompressedTexture2d;

use super::CharAtlas;
use super::Label;
use super::TextMesh;
use super::text_program;
use crate::assets::FONT_PNG;
use crate::assets::FONTMAP_TXT;
use crate::assets::load_png_as_texture;

//////////////////
// TextRenderer //
//////////////////

type TextCache = HashMap<String, TextMesh>;

#[derive(Debug)]
pub struct TextRenderer {
    program: Program,
    options: DrawParameters<'static>,
    font_tex: CompressedTexture2d,
    atlas: CharAtlas,

    // TODO: Use an LRU cache
    cache: TextCache,
}

impl TextRenderer {
    pub fn new(gl: &impl Facade) -> crate::Result<Self> {
        let program = text_program(gl)?;
        let options = DrawParameters { ..Default::default() };
        let font_tex = load_png_as_texture(gl, FONT_PNG)?;
        let atlas = CharAtlas::from_font_map(FONTMAP_TXT);
        let cache = TextCache::new();

        Ok(TextRenderer { program, options, font_tex, atlas, cache })
    }

    pub fn draw(
        &self,
        _frame: &mut impl Surface,
        _label: Label,
    ) -> crate::Result {
        // nothing (FOR NOW)
        // TODO: Draw some text
        Ok(())
    }
}
