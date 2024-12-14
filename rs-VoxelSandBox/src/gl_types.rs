//! Re-exports scalar and vector data types using the same notation as OpenGL.
//!
//! Taken from the [OpenGL documentation](https://www.khronos.org/opengl/wiki/Data_Type_(GLSL)).
#![allow(non_camel_case_types, reason = "matches with OpenGL types")]

/////////////
// Scalars //
/////////////

pub type int = i32;
pub type uint = u32;
pub type float = f32;
pub type double = f64;

/////////////////
// i32 vectors //
/////////////////

pub type ivec2 = glam::IVec2;
pub use glam::ivec2;

pub type ivec3 = glam::IVec3;
pub use glam::ivec3;

pub type ivec4 = glam::IVec4;
pub use glam::ivec4;

/////////////////
// f32 vectors //
/////////////////

pub type vec2 = glam::Vec2;
pub use glam::vec2;

pub type vec3 = glam::Vec3;
pub use glam::vec3;

pub type vec4 = glam::Vec4;
pub use glam::vec4;

/////////////////
// f64 vectors //
/////////////////

pub type dvec2 = glam::DVec2;
pub use glam::dvec2;

pub type dvec3 = glam::DVec3;
pub use glam::dvec3;

pub type dvec4 = glam::DVec4;
pub use glam::dvec4;

//////////////////
// f32 matrices //
//////////////////

pub type mat3 = glam::Mat3;
pub use glam::mat3;

pub type mat4 = glam::Mat4;
pub use glam::mat4;
