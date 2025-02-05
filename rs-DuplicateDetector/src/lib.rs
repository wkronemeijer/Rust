#![forbid(unsafe_code)]

pub mod connection;
pub mod core;
pub mod db;
pub mod hash;
pub mod hash_concurrent;
pub mod search;

pub type Error = ::anyhow::Error;

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;

#[macro_export]
macro_rules! time {
    ($e:expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();
        eprintln!(
            "\x1b[90mexecuted {} in {}ms\x1b[39m",
            stringify!($e),
            duration.as_millis()
        );
        result
    }};
}
