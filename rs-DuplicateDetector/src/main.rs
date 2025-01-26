#![forbid(unsafe_code)]

use std::num::NonZero;
use std::ops::Deref;
use std::thread::available_parallelism;
use std::time::Instant;

use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::db::ConnectionMode;
use duplicate_detector::db::db_version;
use duplicate_detector::db::init_db;
use duplicate_detector::hash_concurrent::HashFilesOptions;
use duplicate_detector::search::Findings;

macro_rules! eprintln_time {
    ($e:expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration_ms = start.elapsed().as_millis();
        eprintln!("\x1b[34m{} in {}ms\x1b[39m", stringify!($e), duration_ms);
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

    let mode = match cli.incremental() {
        true => ConnectionMode::File,
        false => ConnectionMode::Memory,
    };

    //////////////
    // Database //
    //////////////

    let db = init_db(mode)?;

    let conn = &db;

    eprintln!("using SQLite {}", db_version(conn)?);

    ////////////
    // Search //
    ////////////

    eprintln!("searching...");

    let files = eprintln_time!(read_dir_all(directory)?);
    let files = Vec::from_iter(files.iter().map(Deref::deref));
    let files = files.as_slice();
    let options = HashFilesOptions { files, parallelism };
    let file_hashes = eprintln_time!(algo.hash_files(options));
    let findings = eprintln_time!(Findings::from_iter(file_hashes));

    eprintln!("search complete");
    eprintln!();

    /////////////////////
    // List duplicates //
    /////////////////////

    let mut duplicate_count = 0;
    let file_count = findings.file_count();

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

    eprintln!(
        "found {} duplicate(s) amongst {} file(s)",
        duplicate_count, file_count,
    );
    Ok(())
}
