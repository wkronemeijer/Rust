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

////////////////
// u8 vectors //
////////////////

pub type u8vec2 = glam::U8Vec2;
pub use glam::u8vec2;

pub type u8vec3 = glam::U8Vec3;
pub use glam::u8vec3;

pub type u8vec4 = glam::U8Vec4;
pub use glam::u8vec4;

/////////////////
// u16 vectors //
/////////////////

pub type u16vec2 = glam::U16Vec2;
pub use glam::u16vec2;

pub type u16vec3 = glam::U16Vec3;
pub use glam::u16vec3;

pub type u16vec4 = glam::U16Vec4;
pub use glam::u16vec4;

/////////////////
// u32 vectors //
/////////////////

pub type uvec2 = glam::UVec2;
pub use glam::uvec2;

pub type uvec3 = glam::UVec3;
pub use glam::uvec3;

pub type uvec4 = glam::UVec4;
pub use glam::uvec4;

/////////////////
// u64 vectors //
/////////////////

pub type u64vec2 = glam::U64Vec2;
pub use glam::u64vec2;

pub type u64vec3 = glam::U64Vec3;
pub use glam::u64vec3;

pub type u64vec4 = glam::U64Vec4;
pub use glam::u64vec4;

////////////////
// i8 vectors //
////////////////

pub type i8vec2 = glam::I8Vec2;
pub use glam::i8vec2;

pub type i8vec3 = glam::I8Vec3;
pub use glam::i8vec3;

pub type i8vec4 = glam::I8Vec4;
pub use glam::i8vec4;

/////////////////
// i16 vectors //
/////////////////

pub type i16vec2 = glam::I16Vec2;
pub use glam::i16vec2;

pub type i16vec3 = glam::I16Vec3;
pub use glam::i16vec3;

pub type i16vec4 = glam::I16Vec4;
pub use glam::i16vec4;

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
// i64 vectors //
/////////////////

pub type i64vec2 = glam::I64Vec2;
pub use glam::i64vec2;

pub type i64vec3 = glam::I64Vec3;
pub use glam::i64vec3;

pub type i64vec4 = glam::I64Vec4;
pub use glam::i64vec4;

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

//////////////////
// f64 matrices //
//////////////////

pub type dmat3 = glam::DMat3;
pub use glam::dmat3;

pub type dmat4 = glam::DMat4;
pub use glam::dmat4;
