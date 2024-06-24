use crate::prelude::{vec2f, vec3f};

pub trait Material {
    fn get_color(&self, uv: vec2f) -> vec3f;
}
