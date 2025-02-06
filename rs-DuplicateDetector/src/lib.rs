#![forbid(unsafe_code)]

pub mod connection;
pub mod core;
pub mod db;
pub mod hash;
pub mod hash_concurrent;
pub mod search;

use std::collections::HashSet;
use std::fmt::Write;
use std::fs::canonicalize;
use std::ops::Deref;
use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::path::PathBuf;

use url::Url;

use crate::connection::Connection;
use crate::connection::ConnectionKind;
use crate::core::ansi::Styleable;
use crate::core::fs::read_dir_all;
use crate::db::Database;
use crate::hash::FileHash;
use crate::hash::HashStyle;
use crate::hash_concurrent::HashFilesConfiguration;
use crate::search::Deduplicator;
use crate::search::PathStyle;

/////////////////
// Error types //
/////////////////

pub type Error = ::anyhow::Error;

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;

//////////
// Main //
//////////

pub struct Options {
    pub directory: PathBuf,
    pub config: HashFilesConfiguration,
    pub hash_style: HashStyle,
    pub path_style: PathStyle,
    pub cache: ConnectionKind,
    pub clean_cache: bool,
}

pub fn run(
    Options {
        directory,
        config,
        hash_style,
        path_style,
        clean_cache,
        cache,
    }: Options,
) -> crate::Result {
    ///////////////////////
    // Load data sources //
    ///////////////////////

    let mut index = Connection::<Database>::open(cache)?;
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

    let files_to_insert: Vec<(PathBuf, FileHash)> = config
        .run(&files_to_hash)?
        .into_iter()
        .map(|(path, hash)| (path.to_path_buf(), hash))
        .collect();

    ///////////////////
    // Apply changes //
    ///////////////////

    // Index can contains paths of various different directories;
    // This predicate selects those paths which are descendants of our target.
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
        entry.clear();
        let count = paths.len();
        let hash = hash_style.apply(hash);
        let header = format!("{} files with hash {}", count, hash);
        writeln!(entry, "{}:", header.bold())?;
        for &path in paths {
            let dir = path_style.apply(path.parent().unwrap());
            let file = Path::new(path.file_name().unwrap());

            let canonical_file_path = canonicalize(path)?;
            let file_url = Url::from_file_path(&canonical_file_path).unwrap();

            let canonical_dir_path = canonical_file_path.parent().unwrap();
            let dir_url = Url::from_file_path(&canonical_dir_path).unwrap();

            writeln!(
                entry,
                "{}{}{}",
                dir.display().link(&dir_url),
                MAIN_SEPARATOR,
                file.display().link(&file_url),
            )?;
        }
        println!("{}", entry.trim_ascii());
    }
    Ok(())
}
