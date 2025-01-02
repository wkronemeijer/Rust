pub mod core;
pub mod game;
pub mod ideas;

pub type Result<T = (), E = Box<dyn std::error::Error>> =
    std::result::Result<T, E>;
