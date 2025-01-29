#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::fmt::Write;
use std::num::NonZero;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::available_parallelism;

use clap::Parser;
use duplicate_detector::CACHE_FILE_NAME;
pub use duplicate_detector::Result;
use duplicate_detector::connection::Connection;
use duplicate_detector::core::collections::nonempty::NonEmptySlice;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::db::Database;
use duplicate_detector::hash::FileHash;
use duplicate_detector::hash::HashStyle;
use duplicate_detector::hash_concurrent::ConcurrentHashingAlgorithmName;
use duplicate_detector::hash_concurrent::HashFilesOptions;
use duplicate_detector::search::Deduplicator;
use duplicate_detector::search::PathStyle;

////////////////////
// CLI Parameters //
////////////////////

/// Searches for duplicates in the given directory.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
#[deny(missing_docs)]
pub struct Cli {
    /// The directory to search.
    pub directory: PathBuf,

    /// Algorithm for concurrent hashing.
    #[arg(long, default_value_t)]
    pub algo: ConcurrentHashingAlgorithmName,

    /// Number of threads to use for hashing.
    #[arg(long)]
    pub threads: Option<usize>,

    /// Format for hashes.
    #[arg(long, default_value_t)]
    pub hash_style: HashStyle,

    /// Format for paths.
    #[arg(long, default_value_t)]
    pub path_style: PathStyle,

    /// Persist files hashes.
    #[arg(long)]
    pub incremental: bool,

    /// Clean cache before processing.
    #[arg(long)]
    pub clean_cache: bool,
}

//////////
// Main //
//////////

pub fn start(
    Cli {
        directory,
        algo,
        threads,
        hash_style,
        path_style,
        incremental,
        clean_cache,
    }: Cli,
) -> crate::Result {
    ///////////////////////
    // Load data sources //
    ///////////////////////

    let mut index = Connection::<Database>::open(match incremental {
        true => Some(Path::new(CACHE_FILE_NAME)),
        false => None,
    })?;
    if clean_cache {
        index.clear();
    }
    let disk = read_dir_all(&directory)?;

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

    let files_to_hash: Vec<&Path> = new_files.into_iter().collect();

    let files_to_delete: Vec<PathBuf> =
        deleted_files.into_iter().map(|path| path.to_path_buf()).collect();

    /////////////
    // Execute //
    /////////////

    let parallelism = threads
        .and_then(NonZero::new)
        .or_else(|| available_parallelism().ok())
        .or_else(|| NonZero::new(1))
        .unwrap();

    let new_file_hashes =
        if let Some(files_to_hash) = NonEmptySlice::new(&files_to_hash) {
            let options = HashFilesOptions { parallelism };
            let value = algo.hash_files(files_to_hash, options);
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
    let is_our_file = |file: &Path| file.starts_with(&directory);
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

    /////////////////////
    // List duplicates //
    /////////////////////

    let findings = Deduplicator::from_iter(
        index.entries().filter(|(file, _)| is_our_file(file)),
    );

    let ref mut entry = String::new();
    for (hash, paths) in findings.duplicates() {
        let count = paths.len();
        let hash = hash_style.apply(hash);
        writeln!(entry, "\x1B[1m{} files with hash {}\x1B[22m:", count, hash)?;
        for &path in paths {
            let path = path_style.apply(path);
            writeln!(entry, "{}", path.display())?;
        }
        println!("{}", entry.trim_ascii());
        entry.clear();
    }

    Ok(())
}

pub fn main() -> ExitCode {
    let options = Cli::parse(); // NB: parse() exits on failure
    match start(options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\x1b[31m{}\x1b[37m", err);
            ExitCode::FAILURE
        },
    }
}
