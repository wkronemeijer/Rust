//! Contains all code for the world simulation.

pub mod chunk;
pub mod face;
pub mod tile;
pub mod world;

use std::time::Duration;

const TICK_RATE: u64 = 30;

pub const TICKS_PER_SECOND: u64 = TICK_RATE;
pub const SECONDS_PER_TICK: f32 = 1.0 / (TICK_RATE as f32);

const MICROSECONDS_PER_SECOND: u64 = 1_000_000;

pub const TICK_DURATION: Duration =
    Duration::from_micros(MICROSECONDS_PER_SECOND / TICKS_PER_SECOND);
