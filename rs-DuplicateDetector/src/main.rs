#![forbid(unsafe_code)]

use std::num::NonZero;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::available_parallelism;

use anyhow::Context;
use clap::Parser;
use directories::ProjectDirs;
use duplicate_detector::Options;
pub use duplicate_detector::Result;
use duplicate_detector::StyleOptions;
use duplicate_detector::connection::ConnectionKind;
use duplicate_detector::core::ansi::AnsiColor;
use duplicate_detector::core::ansi::Bold;
use duplicate_detector::core::ansi::ColorTarget;
use duplicate_detector::core::ansi::Colored;
use duplicate_detector::hash::HashStyle;
use duplicate_detector::hash_concurrent::HashFilesConfiguration;
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
    pub directories: Vec<PathBuf>,

    /// Number of threads to use for hashing.
    #[arg(long)]
    pub threads: Option<usize>,

    /// Display the full hash.
    #[arg(long)]
    pub long: bool,

    // TODO: Invert this, store hashes (with time) in a AppData/Local cache
    /// Persist files hashes in a file.
    #[arg(long)]
    pub incremental: bool,

    /// Display the absolute path.
    #[arg(long)]
    pub absolute: bool,

    /// Display the canonical path.
    #[arg(long)]
    pub canonical: bool,

    /// Clean cache before processing.
    #[arg(long)]
    pub clean_cache: bool,

    /// Where to store the cache.
    #[arg(long)]
    pub cache_path: Option<PathBuf>,
}

///////////
// Main* //
///////////

const ORG_NAME: &str = "Bliksem Software";
const APP_NAME: &str = "Duplicate Detector";

fn global_cache_path() -> crate::Result<PathBuf> {
    let dirs = ProjectDirs::from("frl", ORG_NAME, APP_NAME)
        .context("failed to find user directory")?;
    Ok(dirs.cache_dir().join("hash-cache.dat"))
}

pub fn start(
    Cli {
        mut directories,
        threads,
        incremental,
        clean_cache,
        cache_path,
        absolute,
        canonical,
        long,
    }: Cli,
) -> crate::Result {
    let cache = match incremental {
        true => ConnectionKind::Disk(match cache_path {
            Some(file) => file,
            None => global_cache_path()?,
        }),
        false => ConnectionKind::Memory,
    };

    let threads = threads
        .and_then(NonZero::new)
        .or_else(|| available_parallelism().ok())
        .or_else(|| NonZero::new(1))
        .unwrap();
    let config = HashFilesConfiguration { threads };

    if directories.len() == 0 {
        directories.push(Path::new(".").to_path_buf());
    }

    let style = StyleOptions {
        hash: match long {
            true => HashStyle::Full,
            false => HashStyle::Short,
        },
        path: match (absolute, canonical) {
            (_, true) => PathStyle::Canonical,
            (true, _) => PathStyle::Absolute,
            _ => PathStyle::Relative,
        },
    };

    duplicate_detector::run(Options {
        config,
        directories,
        cache,
        style,
        clean_cache,
    })
}

//////////
// Main //
//////////

pub fn main() -> ExitCode {
    let options = Cli::parse(); // NB: parse() exits on failure
    match start(options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let target = ColorTarget::Foreground;
            let color = AnsiColor::Red;
            eprint!("{}", Bold(Colored(target, color, "Error: ")));
            for cause in error.chain() {
                eprintln!("{}", Colored(target, color, &cause));
            }
            ExitCode::FAILURE
        },
    }
}
