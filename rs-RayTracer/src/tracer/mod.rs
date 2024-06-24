use crate::algebra::NdcVec2F;
use crate::prelude::*;

pub struct Ray {
    pub origin: vec3f,
    pub direction: vec3f,
}

pub type Color = vec3f;

pub trait Raytracer {
    fn render_fragment(&self, ndc: NdcVec2F) -> vec3f;
}

pub struct DrRayTracer;

impl Raytracer for DrRayTracer {
    fn render_fragment(&self, ndc: NdcVec2F) -> vec3f {
        let vec2f { x, y } = ndc.into();
        let r = 0.5 + x / 2.0; // 0~1
        let g = 0.5 + y / 2.0; // 0~1
        let b = 0.0 + r / 2.0 + g / 2.0;
        vec3f::new(r, g, b)
    }
}
