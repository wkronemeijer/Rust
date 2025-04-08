//! Implements a MineCraft-like game.

#![forbid(unsafe_code)]
// #![warn(missing_docs)]

////////////
// Macros //
////////////

#[macro_export]
macro_rules! once {
    ($body:block) => {{
        use std::sync::atomic::AtomicBool;
        use std::sync::atomic::Ordering;

        static ONCE: AtomicBool = AtomicBool::new(true);
        if ONCE.swap(false, Ordering::Acquire) {
            $body
        }
    }};
}

/////////////
// Modules //
/////////////

pub mod app;
pub mod assets;
pub mod camera;
pub mod core;
pub mod display;
pub mod domain;
pub mod input;

mod gl_types;
pub use gl_types::*;

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

pub const NAME: &str = "Voxel Sandbox";
pub const VERSION: &str = "0.0.1";
