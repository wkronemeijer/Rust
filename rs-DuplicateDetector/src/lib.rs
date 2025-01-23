pub mod core;
pub mod hash;
pub mod search;

pub type Result<T = (), E = ::anyhow::Error> = ::std::result::Result<T, E>;
