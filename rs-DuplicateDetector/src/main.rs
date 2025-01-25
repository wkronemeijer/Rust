#![forbid(unsafe_code)]

use std::num::NonZero;
use std::ops::Deref;
use std::thread::available_parallelism;
use std::time::Instant;

use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::hash_concurrent::HashFilesOptions;
use duplicate_detector::search::Findings;

macro_rules! println_time {
    ($e:expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();

        println!("{} in {}ms", stringify!($e), duration.as_millis());
        result
    }};
}

pub fn main() -> crate::Result {
    let cli = Cli::parse(); // NB: parse exits on failure
    let algo = cli.algo();
    let style = cli.style();
    let directory = cli.directory();
    let parallelism = match cli.parallel() {
        true => available_parallelism()?,
        false => NonZero::new(1).unwrap(),
    };

    ////////////
    // Search //
    ////////////

    println!("searching...");

    let files = println_time!(read_dir_all(directory)?);
    let files = Vec::from_iter(files.iter().map(Deref::deref));
    let files = files.as_slice();
    let options = HashFilesOptions { files, parallelism };
    let file_hashes = println_time!(algo.hash_files(options));
    let findings = println_time!(Findings::from_iter(file_hashes));

    println!("search complete");
    println!();

    /////////////////////
    // List duplicates //
    /////////////////////

    let mut duplicate_count = 0;
    let file_count = findings.file_count();
    debug_assert_eq!(files.len(), file_count);

    for (hash, paths) in findings.duplicates() {
        let count = paths.len();
        duplicate_count += count;
        println!("{count} file(s) with duplicate hash '{hash}':");
        for path in paths {
            println!("{}", style.try_apply(path).display());
        }
        println!();
    }

    /////////////
    // Summary //
    /////////////

    println!(
        "found {} duplicate(s) amongst {} file(s)",
        duplicate_count, file_count,
    );
    Ok(())
}
