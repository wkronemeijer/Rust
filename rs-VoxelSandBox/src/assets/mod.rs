//! Functions as an asset manifest.

pub mod img;

use std::io::Cursor;

use glium::backend::Facade;
use glium::texture::CompressedTexture2d;
use glium::texture::RawImage2d;
use image::ImageFormat;
use image::load;
use img::TiledTexture;
use winit::window::Icon;

use crate::uvec2;

// This tile size/grid size thing keeps happening
// World/Chunk/Tile is the same thing but 3D

//////////////
// Textures //
//////////////

pub const FONT_PNG: &[u8] = include_bytes!("../../assets/textures/font.png");
pub const FONT_PNG_DIMENSIONS: uvec2 = uvec2(128, 256);
pub const FONT_TILESIZE: uvec2 = uvec2(8, 16);
pub const FONT_GRIDSIZE: uvec2 = uvec2(16, 16);
pub const FONTMAP_TXT: &str = include_str!("../../assets/textures/fontmap.txt");

pub const TERRAIN_PNG: &[u8] =
    include_bytes!("../../assets/textures/terrain.png");
pub const TERRAIN_PNG_DIMENSIONS: uvec2 = uvec2(256, 256);
pub const TERRAIN_PNG_PIXEL_DIM: usize = 256; // px
pub const TERRAIN_TILESIZE: uvec2 = uvec2(8, 8);
pub const TERRAIN_GRIDSIZE: uvec2 = uvec2(32, 32);

pub fn load_png_as_texture(
    gl: &impl Facade,
    bytes: &[u8],
) -> crate::Result<CompressedTexture2d> {
    let image = load(Cursor::new(bytes), ImageFormat::Png)?.to_rgba8();
    let dim = image.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dim);
    Ok(CompressedTexture2d::new(gl, raw_image)?)
}

pub fn load_terrain_texture(gl: &impl Facade) -> crate::Result<TiledTexture> {
    todo!();
}

pub fn load_font_texture(gl: &impl Facade) -> crate::Result<TiledTexture> {
    todo!();
}

///////////
// Icons //
///////////

pub const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");

pub fn png_to_icon(bytes: &[u8]) -> crate::Result<Icon> {
    let image = load(Cursor::new(bytes), ImageFormat::Png)?.to_rgba8();
    let (width, height) = image.dimensions();
    Ok(Icon::from_rgba(image.into_vec(), width, height)?)
}
