//! Functions as an asset manifest.

use std::io::Cursor;

use glium::backend::Facade;
use glium::texture::CompressedTexture2d;
use glium::texture::RawImage2d;
use image::ImageFormat;
use image::load;
use winit::window::Icon;

pub const FONT_PNG: &[u8] = include_bytes!("../../assets/textures/font.png");

pub const TERRAIN_PNG: &[u8] =
    include_bytes!("../../assets/textures/terrain.png");
pub const TERRAIN_PNG_PIXEL_DIM: usize = 256; // px

pub const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");

pub fn png_to_texture(
    display: &impl Facade,
    terrain_png: &[u8],
) -> Result<CompressedTexture2d, anyhow::Error> {
    let image = load(Cursor::new(terrain_png), ImageFormat::Png)?.to_rgba8();
    let dim = image.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dim);
    Ok(CompressedTexture2d::new(display, raw_image)?)
}

pub fn png_to_icon(icon: &[u8]) -> crate::Result<Icon> {
    let image = load(Cursor::new(icon), ImageFormat::Png)?.to_rgba8();
    let (width, height) = image.dimensions();
    Ok(Icon::from_rgba(image.into_vec(), width, height)?)
}
