//! Stuff that should be in [`core`], but isn't.

pub mod ansi;
pub mod collections;
pub mod error;
pub mod fs;

#[macro_export]
/// Like [`dbg!`], but prints the time taken to stderr.
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
