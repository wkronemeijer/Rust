#![forbid(unsafe_code)]

pub mod assets;
pub mod core;
pub mod display;
pub mod domain;
pub mod manifest;
pub mod prelude;
pub mod program;

pub use prelude::*;
pub use program::run;
