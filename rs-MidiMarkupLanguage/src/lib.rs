#![warn(clippy::all)]

pub mod domain {
    pub mod midi;
    pub mod midi_file;
}

// TODO: Decide on how you will propagate errors
// Rust doesn't have error sets (sadly)
// a billion error enums doesn't feel good either
pub type Error = ::anyhow::Error;

pub type Result<T = (), E = crate::Error> = std::result::Result<T, E>;
