use crate::{algebra::NdcVec2F, prelude::*};

pub struct Ray {
    pub origin: Vec3F,
    pub direction: Vec3F,
}

pub type Color = Vec3F;

pub trait Raytracer {
    fn render_fragment(&self, ndc: NdcVec2F) -> Vec3F;
}

pub struct DrRayTracer;

impl Raytracer for DrRayTracer {
    fn render_fragment(&self, ndc: NdcVec2F) -> Vec3F {
        let Vec2F { x, y } = ndc.into();
        let r = 0.5 + x / 2.0; // 0~1
        let g = 0.5 + y / 2.0; // 0~1
        let b = 0.0 + r / 2.0 + g / 2.0;
        Vec3F::new(r, g, b)
    }
}
