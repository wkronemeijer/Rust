pub mod cli;
pub mod core;
pub mod hash;
pub mod search;

pub type Result<T = (), E = ::anyhow::Error> = ::std::result::Result<T, E>;

pub const CACHE_FILE_NAME: &str = "hash.cache.db";
