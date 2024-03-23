pub mod shaders;
pub use shaders::*;

use crate::algebra::NdcVec2F;
use crate::prelude::*;
use crate::tracer::Raytracer;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{MipmapsOption, RawImage2d, Texture2dDataSink as _};
use glium::{Texture2d, VertexBuffer};
use std::borrow::Cow;
use std::time::Instant;

fn screen_quad_vertices() -> Vec<Vertex> {
    let top_right = Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    };
    let bottom_right = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    };
    let bottom_left = Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    };
    let top_left = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    };
    // Default winding order is supposedly CCW, yet this still shows up
    // Isn't culling on by default as well?
    // idk
    vec![top_right, bottom_right, bottom_left, bottom_left, top_left, top_right]
}

pub fn screen_quad(display: &impl Facade) -> (VertexBuffer<Vertex>, NoIndices) {
    let vertices = VertexBuffer::new(display, &screen_quad_vertices()).expect("create vertex buffer");
    let indices = NoIndices(PrimitiveType::TrianglesList);
    (vertices, indices)
}

fn point_to_index(size: Size, x: u32, y: u32) -> usize {
    let Size { width, .. } = size;
    (x + y * width).try_into().expect("spread index")
}

fn point_to_ndc(size: Size, ix: u32, iy: u32) -> NdcVec2F {
    let Size { width: iw, height: ih } = size;
    // Reminder: we need to go to the center of each "pixel"

    let x = ix as f32;
    let y = iy as f32;
    let w = iw as f32;
    let h = ih as f32;

    let ndc_x = 2.0 * ((x + 0.5) / w - 0.5);
    let ndc_y = 2.0 * ((y + 0.5) / h - 0.5);

    NdcVec2F::new(ndc_x, ndc_y)
}

pub fn render_to_texture(display: &impl Facade, tracer: &impl Raytracer, size: Size) -> Texture2d {
    let time = Instant::now();
    let result = {
        let Size { width, height } = size;
        type Rgb8 = (u8, u8, u8);
        let buffer = {
            let target_size: usize = size.area().try_into().expect("calculate pixel area");
            let mut vec: Vec<Rgb8> = Vec::new();
            vec.resize_with(target_size, Default::default);

            for x in 0..width {
                for y in 0..height {
                    let index = point_to_index(size, x, y);
                    let ndc = point_to_ndc(size, x, y);
                    vec[index] = tracer.render_fragment(ndc).to_color_bytes();
                }
            }
            vec
        };
        let image = RawImage2d::from_raw(Cow::Owned(buffer), width, height);
        Texture2d::with_mipmaps(display, image, MipmapsOption::NoMipmap).expect("create texture from buffer")
    };
    println!("render_texture() in {}ms", time.elapsed().as_millis());
    result
}
