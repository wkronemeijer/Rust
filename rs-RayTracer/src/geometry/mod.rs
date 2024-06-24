use crate::prelude::*;

pub mod plane;
pub mod sphere;

pub struct HitInfo {
    pub p: vec3f,
}

pub trait Hittable {
    fn try_hit(&self) -> Option<HitInfo>;
}
