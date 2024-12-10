//! Functions as an asset manifest.

use std::io::Cursor;

use glium::backend::Facade;
use glium::texture::CompressedTexture2d;
use glium::texture::RawImage2d;
use image::load;
use image::ImageFormat;
use winit::window::Icon;

const TERRAIN_PNG: &[u8] = include_bytes!("../../assets/textures/terrain.png");
const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");

pub const TERRAIN_PNG_PIXEL_DIM: usize = 256; // px

pub fn load_terrain_png(
    display: &impl Facade,
) -> crate::Result<CompressedTexture2d> {
    let image = load(Cursor::new(TERRAIN_PNG), ImageFormat::Png)?.to_rgba8();
    let dim = image.dimensions();
    let raw_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dim);
    Ok(CompressedTexture2d::new(display, raw_image)?)
}

pub fn load_icon_png() -> crate::Result<Icon> {
    let image = load(Cursor::new(ICON_PNG), ImageFormat::Png)?.to_rgba8();
    let (width, height) = image.dimensions();
    Ok(Icon::from_rgba(image.into_vec(), width, height)?)
}
