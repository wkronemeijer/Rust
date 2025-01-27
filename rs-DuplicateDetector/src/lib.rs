#![forbid(unsafe_code)]

pub mod cli;
pub mod connection;
pub mod core;
pub mod db;
pub mod hash;
pub mod hash_concurrent;
pub mod search;

pub type Result<T = (), E = ::anyhow::Error> = ::std::result::Result<T, E>;

pub const CACHE_FILE_NAME: &str = "hash-cache.json";
