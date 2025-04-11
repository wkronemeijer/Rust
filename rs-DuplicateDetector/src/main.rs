#![forbid(unsafe_code)]

use std::num::NonZero;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::available_parallelism;

use clap::Parser;
use duplicate_detector::Options;
pub use duplicate_detector::Result;
use duplicate_detector::connection::CacheFormat;
use duplicate_detector::connection::ConnectionKind;
use duplicate_detector::core::ansi::AnsiColor;
use duplicate_detector::core::ansi::ColorTarget;
use duplicate_detector::core::ansi::Colored;
use duplicate_detector::hash::HashStyle;
use duplicate_detector::hash_concurrent::AlgorithmName;
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

    /// Algorithm for concurrent hashing.
    #[arg(long, default_value_t)]
    pub algorithm: AlgorithmName,

    /// Number of threads to use for hashing.
    #[arg(long)]
    pub threads: Option<usize>,

    /// Format for hashes.
    #[arg(long, default_value_t)]
    pub hash_style: HashStyle,

    /// Format for paths.
    #[arg(long, default_value_t)]
    pub path_style: PathStyle,

    /// Persist files hashes in a file.
    #[arg(long)]
    pub incremental: bool,

    /// Clean cache before processing.
    #[arg(long)]
    pub clean_cache: bool,

    /// Where to store the cache.
    #[arg(long)]
    pub cache_path: Option<PathBuf>,

    /// Format of the cache.
    #[arg(long, default_value_t)]
    pub cache_format: CacheFormat,
}

///////////
// Main* //
///////////

pub fn start(
    Cli {
        mut directories,
        algorithm,
        threads,
        hash_style,
        path_style,
        incremental,
        clean_cache,
        cache_path,
        cache_format,
    }: Cli,
) -> crate::Result {
    let cache = match incremental {
        true => ConnectionKind::Disk {
            file: match cache_path {
                Some(file) => file,
                None => cache_format.default_file_name().to_path_buf(),
            },
            format: cache_format,
        },
        false => ConnectionKind::Memory,
    };

    let threads = threads
        .and_then(NonZero::new)
        .or_else(|| available_parallelism().ok())
        .or_else(|| NonZero::new(1))
        .unwrap();
    let config = HashFilesConfiguration::new(algorithm, threads);

    if directories.len() == 0 {
        directories.push(Path::new(".").to_path_buf());
    }

    duplicate_detector::run(Options {
        directories,
        config,
        cache,
        hash_style,
        path_style,
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
            // TODO: Read https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations to include causes as well
            eprintln!(
                "{}",
                Colored(ColorTarget::Foreground, AnsiColor::Red, &error)
            );
            ExitCode::FAILURE
        },
    }
}
