pub mod core;
pub mod game;
pub mod idea;
pub mod idea2;
pub mod prelude;

pub type Result<T = (), E = Box<dyn std::error::Error>> =
    std::result::Result<T, E>;
