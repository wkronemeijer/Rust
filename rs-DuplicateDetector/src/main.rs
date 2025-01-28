#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::num::NonZero;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::thread::available_parallelism;

use clap::Parser;
use duplicate_detector::CACHE_FILE_NAME;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::connection::Connection;
use duplicate_detector::core::collections::nonempty::NonEmptySlice;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::db::Database;
use duplicate_detector::hash::FileHash;
use duplicate_detector::hash_concurrent::HashFilesOptions;
use duplicate_detector::search::Deduplicator;

pub fn main() -> crate::Result {
    let cli = Cli::parse(); // NB: parse() exits on failure
    let algo = cli.algo();
    let hash_style = cli.hash_style();
    let path_style = cli.path_style();
    let directory = cli.directory();
    let cache = match cli.incremental() {
        true => Some(Path::new(CACHE_FILE_NAME)),
        false => None,
    };
    let parallelism = (cli.parallelism())
        .or_else(|| available_parallelism().ok())
        .or_else(|| NonZero::new(1))
        .unwrap();

    ///////////////////////
    // Load data sources //
    ///////////////////////

    eprintln!("searching...");

    let (mut index, err) = Connection::<Database>::open(cache);
    if let Some(e) = err {
        eprintln!("failed to open index: {}", e);
    }
    if cli.clean_index() {
        index.clear();
    }

    let disk = read_dir_all(directory)?;

    eprintln!("search complete");

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

    let new_file_hashes =
        if let Some(files_to_hash) = NonEmptySlice::new(&files_to_hash) {
            eprintln!("hashing using '{}'×{}", algo, parallelism);
            let options = HashFilesOptions { parallelism };
            let value = algo.hash_files(files_to_hash, options);
            eprintln!("hashing complete");
            value
        } else {
            vec![]
        };

    let files_to_insert: Vec<(PathBuf, FileHash)> = new_file_hashes
        .into_iter()
        .map(|(path, hash)| (path.to_path_buf(), hash))
        .collect();

    ///////////////////
    // Apply changes //
    ///////////////////

    // Index can contains paths of various different directories;
    // This predicate selects those paths which belong to the current target.
    let is_our_file = |file: &Path| file.starts_with(directory);
    let mut did_modify = false;

    for file in files_to_delete {
        if is_our_file(&file) {
            index.remove(&file);
            did_modify = true;
        }
    }

    for (path, hash) in files_to_insert {
        index.add(path, hash);
        did_modify = true;
    }

    if did_modify {
        if let Err(e) = index.save() {
            eprintln!("failed to save index: {}", e);
        }
    }

    eprintln!("finding duplicates...");
    let findings = Deduplicator::from_iter(
        index.entries().filter(|(file, _)| is_our_file(file)),
    );
    eprintln!();

    /////////////////////
    // List duplicates //
    /////////////////////

    let mut duplicate_count = 0;
    let file_count = findings.file_count();

    for (hash, paths) in findings.duplicates() {
        let count = paths.len();
        duplicate_count += count;
        println!("{count} file(s) with hash {}", hash_style.apply(hash));
        for &path in paths {
            println!("{}", path_style.apply(path).display());
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
