#![allow(non_camel_case_types)]

// Overlapping types with names...I might regret this

pub type ivec2 = glam::IVec2;
pub use glam::ivec2;

pub type ivec3 = glam::IVec3;
pub use glam::ivec3;

pub type ivec4 = glam::IVec4;
pub use glam::ivec4;

pub type vec2 = glam::Vec2;
pub use glam::vec2;

pub type vec3 = glam::Vec3;
pub use glam::vec3;

pub type vec4 = glam::Vec4;
pub use glam::vec4;

pub type mat3 = glam::Mat3;
pub use glam::mat3;

pub type mat4 = glam::Mat4;
pub use glam::mat4;

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
