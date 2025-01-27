#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::num::NonZero;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::thread::available_parallelism;
use std::time::Instant;

use clap::Parser;
use duplicate_detector::CACHE_FILE_NAME;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::connection::Connection;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::db::Database;
use duplicate_detector::hash::FileHash;
use duplicate_detector::hash_concurrent::HashFilesOptions;
use duplicate_detector::search::Deduplicator;

macro_rules! time {
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
    let cache_path = match cli.incremental() {
        true => Some(Path::new(CACHE_FILE_NAME)),
        false => None,
    };
    let options = HashFilesOptions {
        parallelism: match cli.parallel() {
            true => available_parallelism()?,
            false => NonZero::new(1).unwrap(),
        },
    };

    ///////////////////////
    // Load data sources //
    ///////////////////////

    eprintln!("opening index...");

    let mut index = Connection::<Database>::open(cache_path)?;

    if cli.purge_db() {
        index.clear();
    }

    eprintln!("reading dirs...");
    let disk = time!(read_dir_all(directory)?);

    /////////////////////////////
    // Compare index with disk //
    /////////////////////////////

    // Files in the index
    let index_files: HashSet<&Path> = index.paths().collect();

    // Files on disk
    let disk_files: HashSet<&Path> =
        disk.iter().map(|path| path.deref()).collect();

    // Deleted files == Indexed files not on disk
    let deleted_files: HashSet<&Path> =
        index_files.difference(&disk_files).copied().collect();

    // New files == Disk files not indexed
    let new_files: HashSet<&Path> =
        disk_files.difference(&index_files).copied().collect();

    ////////////////////////
    // Set plan of action //
    ////////////////////////

    let files_to_delete: Vec<PathBuf> =
        deleted_files.into_iter().map(|path| path.to_path_buf()).collect();

    let files_to_hash: Vec<&Path> = new_files.into_iter().collect();

    /////////////
    // Execute //
    /////////////

    eprintln!("hashing...");
    let new_file_hashes = time!(algo.hash_files(&files_to_hash, options));

    let files_to_insert: Vec<(PathBuf, FileHash)> = new_file_hashes
        .into_iter()
        .map(|(path, hash)| (path.to_path_buf(), hash))
        .collect();

    for file in files_to_delete {
        index.remove(&file);
    }

    for (path, hash) in files_to_insert {
        index.add(path, hash);
    }

    if let Err(e) = index.save() {
        eprintln!("failed to save index: {}", e);
        // continue
    }

    let findings = time!(Deduplicator::from_iter(index.entries()));

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
