use glium::texture::CompressedTexture2d;

use crate::core::spreading::SizeLike;
use crate::ivec2;
use crate::uvec2;
use crate::vec2;

pub struct TiledTexture {
    texture: CompressedTexture2d,

    texture_size: uvec2,
    grid_size: uvec2,
    tile_size: uvec2,

    uv_step: vec2,
}

pub struct UvQuad {
    pub top_left: vec2,
    pub top_right: vec2,
    pub bottom_left: vec2,
    pub bottom_right: vec2,
}

impl TiledTexture {
    pub fn new(texture: CompressedTexture2d, tile_size: uvec2) -> Self {
        let texture_size: uvec2 = texture.dimensions().into();
        let grid_size = texture_size.saturating_div(tile_size);
        // FIXME: grid×tile can fail to completely cover the texture
        // Code still works fine however
        let uv_step = tile_size.as_vec2() / texture_size.as_vec2();
        TiledTexture { texture, texture_size, grid_size, tile_size, uv_step }
    }

    pub fn texture(&self) -> &CompressedTexture2d { &self.texture }

    pub fn texture_size(&self) -> uvec2 { self.texture_size }

    pub fn grid_size(&self) -> uvec2 { self.grid_size }

    pub fn tile_size(&self) -> uvec2 { self.tile_size }

    pub fn get(&self, index: ivec2) -> Option<UvQuad> {
        let index = self.tile_size.wrapping_restrict(index);
        let step @ vec2 { x: x_step, y: y_step } = self.uv_step;

        let bottom_left = index.as_vec2() * step;
        let bottom_right = bottom_left + x_step * vec2::X;
        let top_left = bottom_left + y_step * vec2::Y;
        let top_right = bottom_right + y_step * vec2::Y;
        Some(UvQuad { top_left, top_right, bottom_left, bottom_right })
    }
}
