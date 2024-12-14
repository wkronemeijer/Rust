//! Implements a MineCraft-like game.

#![forbid(unsafe_code)]
// #![warn(missing_docs)]

pub mod app;
pub mod assets;
pub mod camera;
pub mod core;
pub mod display;
pub mod domain;
pub mod input;
pub mod manifest;

mod gl_types;
pub use gl_types::*;

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
