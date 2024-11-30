//! Functions as an asset manifest.

use std::io::Cursor;

use glium::backend::Facade;
use glium::texture::CompressedTexture2d;
use glium::texture::RawImage2d;
use image::load;
use image::ImageFormat;

const TERRAIN_PNG: &[u8] = include_bytes!("../../assets/textures/terrain.png");

pub const TERRAIN_PNG_PIXEL_LEN: usize = 256; // px

pub fn load_terrain_png(
    display: &impl Facade,
) -> crate::Result<CompressedTexture2d> {
    let image = load(Cursor::new(TERRAIN_PNG), ImageFormat::Png)?.to_rgba8();
    let dim = image.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dim);
    Ok(CompressedTexture2d::new(display, raw_image)?)
}
