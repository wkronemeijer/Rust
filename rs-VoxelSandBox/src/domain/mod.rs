pub mod chunk;
pub mod tile;
pub mod world;

pub const TICK_RATE: usize = 20;
pub const SECONDS_PER_TICK: f32 = 1.0 / (TICK_RATE as f32);
