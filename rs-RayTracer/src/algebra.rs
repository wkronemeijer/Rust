// Bad habits, I suppose
#![allow(non_camel_case_types)]
use winit::dpi::PhysicalSize;

///////////
// Point //
///////////

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

//////////
// Size //
//////////

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub const fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }

    pub fn area(&self) -> u32 {
        // What to do about overflows...
        self.width * self.height
    }
}

impl From<PhysicalSize<u32>> for Size {
    fn from(this: PhysicalSize<u32>) -> Self {
        let PhysicalSize { width, height } = this;
        Size { width, height }
    }
}

impl From<Size> for (u32, u32) {
    fn from(this: Size) -> Self {
        (this.width, this.height)
    }
}

//////////
// Rect //
//////////

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

////////////////////////
// Vector of 2 floats //
////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct vec2f {
    pub x: f32,
    pub y: f32,
}

impl vec2f {
    pub fn new(x: f32, y: f32) -> vec2f {
        vec2f { x, y }
    }
}

impl From<vec2f> for (f32, f32) {
    fn from(value: vec2f) -> Self {
        (value.x, value.y)
    }
}

////////////////////////
// Vector of 3 floats //
////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> vec3f {
        vec3f { x, y, z }
    }

    pub fn to_color_bytes(&self) -> (u8, u8, u8) {
        fn splat(value: f32) -> u8 {
            (value.clamp(0.0, 1.0) * u8::MAX as f32).round() as u8
        }
        (splat(self.x), splat(self.y), splat(self.z))
        // THis is where the mystery bytes come from, no?
    }
}

//////////////////////////////////////////////////////
// Normalized device coordinates vector of 3 floats //
//////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub struct NdcVec2F {
    point: vec2f,
}

const MINIMUM: f32 = -1.0;
const MAXIMUM: f32 = 1.0;

impl NdcVec2F {
    pub fn new(x: f32, y: f32) -> NdcVec2F {
        NdcVec2F {
            point: vec2f::new(x.clamp(MINIMUM, MAXIMUM), y.clamp(MINIMUM, MAXIMUM)),
        }
    }
}

impl From<NdcVec2F> for vec2f {
    fn from(value: NdcVec2F) -> Self {
        value.point
    }
}
