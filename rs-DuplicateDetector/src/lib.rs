//! Helps detect duplicates.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod connection;
/// Stuff that should be in [`core`], but isn't.
pub mod core {
    pub mod ansi;
    /// Contains additional collections.
    pub mod collections {
        pub mod tinyvec;
    }
    pub mod error;
    pub mod fs;
    pub mod xattr;
}
pub mod db;
pub mod hash;
pub mod hash_concurrent;
pub mod search;
pub mod status_line;

use std::collections::HashSet;
use std::fmt::Write;
use std::fs::canonicalize;
use std::io::Write as _;
use std::io::stdin;
use std::io::stdout;
use std::ops::Deref;
use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::path::PathBuf;

use url::Url;

use crate::connection::Connection;
use crate::connection::ConnectionKind;
use crate::core::ansi::Anchor;
use crate::core::ansi::Bold;
use crate::core::fs::open_explorer;
use crate::core::fs::read_dir_all;
use crate::db::Database;
use crate::hash::FileHash;
use crate::hash::HashStyle;
use crate::hash_concurrent::HashFilesOptions;
use crate::hash_concurrent::parallel_hash_files;
use crate::search::Deduplicator;
use crate::search::PathStyle;

/////////////////
// Error types //
/////////////////

/// This libraries error type. A synonym for [`::anyhow::Error`].
pub type Error = ::anyhow::Error;

/// This libraries result type.
pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;

////////////
// Output //
////////////

fn print_findings(
    findings: &Deduplicator,
    style: StyleOptions,
    interactive: bool,
) -> crate::Result {
    let mut lines = stdin().lines();
    let ref mut entry = String::new();
    let mut first = true;
    for (hash, paths) in findings.duplicates() {
        if interactive {
            if first {
                first = false;
            } else {
                let mut stdout = stdout().lock();
                write!(stdout, "Press enter for the next set of files...")?;
                stdout.flush()?;
                let Some(Ok(_)) = lines.next() else { continue };
                write!(stdout, "\x1B[1A\x1B[2K\r")?; //Clear last line, return to 
                stdout.flush()?;
            }
            for &path in paths {
                open_explorer(path)?;
            }
        }

        entry.clear();
        let count = paths.len();
        let hash = style.hash.format(hash);
        let header = format!("{} files with hash {}", count, hash);
        writeln!(entry, "{}:", Bold(&header))?;
        for &path in paths {
            let dir = style.path.format(path.parent().unwrap());
            let file = Path::new(path.file_name().unwrap());

            let canonical_file_path = canonicalize(path)?;
            let file_url = Url::from_file_path(&canonical_file_path).unwrap();

            let canonical_dir_path = canonical_file_path.parent().unwrap();
            let dir_url = Url::from_file_path(&canonical_dir_path).unwrap();

            writeln!(
                entry,
                "{}{}{}",
                Anchor(&dir_url, dir.display()),
                MAIN_SEPARATOR,
                Anchor(&file_url, file.display()),
            )?;
        }
        println!("{}", entry.trim_ascii());
    }
    Ok(())
}

//////////
// Main //
//////////

/// Options for output formatting.
#[derive(Debug, Clone, Copy)]
pub struct StyleOptions {
    /// How to format the hashes of the duplicates found.
    pub hash: HashStyle,
    /// How to format the path to the duplicates found.
    pub path: PathStyle,
}

/// Options for finding duplicates.
pub struct Options {
    /// Where to look for duplicates.
    pub directories: Vec<PathBuf>,
    /// Options for hashing.
    pub config: HashFilesOptions,
    /// Options for output formatting.
    pub style: StyleOptions,
    /// Where to (re)store previously found information on duplicates.
    pub cache: ConnectionKind,
    /// Whether to wipe the cache before computation.
    pub clean_cache: bool,
    /// Whether to display TUI interface for duplicates
    pub interactive: bool,
}

/// Finds duplicates using the specified parameters.
pub fn run(
    Options {
        directories,
        config,
        style,
        clean_cache,
        cache,
        interactive,
    }: Options,
) -> crate::Result {
    ///////////////////////
    // Load data sources //
    ///////////////////////

    let mut index = Connection::<Database>::open(cache)?;
    if clean_cache {
        index.clear();
    }
    let disk: Vec<PathBuf> = {
        let mut all_files = Vec::new();
        for dir in &directories {
            all_files.extend(read_dir_all(dir)?);
        }
        // directories.iter().flat_map(read_dir_all).flatten()
        // ...has the same result, but ignores errors
        all_files
    };

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

    let files_to_insert: Vec<(PathBuf, FileHash)> =
        parallel_hash_files(&files_to_hash, config)?
            .into_iter()
            .map(|(path, hash)| (path.to_path_buf(), hash))
            .collect();

    ///////////////////
    // Apply changes //
    ///////////////////

    // Index can contains paths of various different directories;
    // This predicate selects those paths which are descendants of our target.
    let is_our_file =
        |file: &Path| directories.iter().any(|dir| file.starts_with(dir));
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

    print_findings(&findings, style, interactive)?;

    Ok(())
}
