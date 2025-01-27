use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::fs::canonicalize;
use std::num::NonZero;
use std::path::Path;
use std::path::PathBuf;
use std::path::absolute;

use clap::Parser;
use clap::ValueEnum;
use strum::Display;

use crate::hash::FileHash;
use crate::hash_concurrent::ConcurrentHashingAlgorithmName;

/////////////////////
// Hash Formatting //
/////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum, Display)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum HashStyle {
    #[default]
    Short,
    Full,
}

impl HashStyle {
    pub fn apply(self, hash: &FileHash) -> String {
        let mut s = hash.to_string();
        match self {
            Self::Short => s.truncate(8), // hash string is always ASCII
            Self::Full => {},
        }
        s
    }
}

/////////////////////
// Path Formatting //
/////////////////////
///
#[derive(Debug, Default, Clone, Copy, ValueEnum, Display)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum PathStyle {
    #[default]
    Relative,
    Absolute,
    Canonical,
}

impl PathStyle {
    /// Tries to apply a formatting style.
    ///
    /// Can fail if the path is empty, the file at the path doesn't exist, etc.
    pub fn try_apply(self, path: &Path) -> crate::Result<Cow<Path>> {
        Ok(match self {
            Self::Relative => Borrowed(path),
            Self::Absolute => Owned(absolute(path)?),
            Self::Canonical => Owned(canonicalize(path)?),
        })
    }

    /// Applies a formatting style,
    /// falling back to the original path if formatting fails.
    pub fn apply(self, path: &Path) -> Cow<Path> {
        match self.try_apply(path) {
            Ok(cow) => cow,
            Err(_) => Borrowed(path),
        }
    }
}

/////////////
// Parsing //
/////////////
// See https://docs.rs/clap/latest/clap/_derive/#arg-types for help

/// Searches for duplicates in the given directory.
#[derive(Debug, Clone, Parser)]
#[command(version, about)]
#[deny(missing_docs)]
pub struct Cli {
    /// The directory to search.
    directory: PathBuf,

    /// Algorithm for concurrent hashing.
    #[arg(long, default_value_t)]
    algo: ConcurrentHashingAlgorithmName,

    /// Number of threads to use for hashing.
    #[arg(long)]
    threads: Option<usize>,

    /// Format for hashes.
    #[arg(long, default_value_t)]
    hash_style: HashStyle,

    /// Format for paths.
    #[arg(long, default_value_t)]
    path_style: PathStyle,

    /// Persist files hashes.
    #[arg(long)]
    incremental: bool,

    /// Clean cache before processing.
    #[arg(long)]
    clean_cache: bool,
}

impl Cli {
    // ...split the option name from the identifier
    pub fn algo(&self) -> ConcurrentHashingAlgorithmName { self.algo }

    pub fn hash_style(&self) -> HashStyle { self.hash_style }

    pub fn path_style(&self) -> PathStyle { self.path_style }

    pub fn clean_index(&self) -> bool { self.clean_cache }

    pub fn directory(&self) -> &Path { &self.directory }

    pub fn incremental(&self) -> bool { self.incremental }

    pub fn parallelism(&self) -> Option<NonZero<usize>> {
        self.threads.and_then(NonZero::new)
    }
}
