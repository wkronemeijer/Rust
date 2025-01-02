pub mod core;
pub mod events;
pub mod game;
pub mod ideas;
pub mod items;
pub mod rng;
pub mod stats;

pub type Result<T = (), E = Box<dyn std::error::Error>> =
    std::result::Result<T, E>;
