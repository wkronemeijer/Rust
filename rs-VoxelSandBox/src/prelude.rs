#![allow(non_camel_case_types)]

pub type ivec2 = glam::IVec2;
pub type ivec3 = glam::IVec2;

pub type vec3 = glam::Vec3;
pub type vec4 = glam::Vec4;

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
