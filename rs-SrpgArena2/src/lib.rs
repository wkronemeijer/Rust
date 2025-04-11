#![feature(map_many_mut)]
pub mod core;
pub mod events;
pub mod game;
pub mod ideas;
pub mod items;
pub mod rng;
pub mod stats;

pub type Error = ::anyhow::Error;

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;
