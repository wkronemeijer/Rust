use std::borrow::Cow;
use std::fmt;
use std::fs::canonicalize;
use std::path::Path;
use std::path::PathBuf;
use std::path::absolute;

use clap::Parser;
use clap::ValueEnum;

use crate::hash_concurrent::ConcurrentHashingAlgorithm;

/////////////////////
// Path Formatting //
/////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum PathStyle {
    #[default]
    Relative,
    Absolute,
    Canonical,
}

impl PathStyle {
    /// Applies a formatting style.
    ///
    /// Can fail if the path is empty, the file at the path doesn't exist, etc.
    pub fn apply(self, path: &Path) -> crate::Result<Cow<Path>> {
        Ok(match self {
            Self::Relative => Cow::Borrowed(path),
            Self::Absolute => Cow::Owned(absolute(path)?),
            Self::Canonical => Cow::Owned(canonicalize(path)?),
        })
    }

    /// Tries to apply a formatting style,
    /// falling back to the original path if formatting fails.
    pub fn try_apply(self, path: &Path) -> Cow<Path> {
        match self.apply(path) {
            Ok(cow) => cow,
            Err(_) => Cow::Borrowed(path),
        }
    }
}

impl fmt::Display for PathStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{self:?}").to_ascii_lowercase().fmt(f)
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

    /// Algorithm to search with.
    #[arg(long, default_value_t)]
    algo: ConcurrentHashingAlgorithm,

    /// Formatting used for results.
    #[arg(long, default_value_t)]
    style: PathStyle,

    /// Persist files hashes.
    #[arg(long)]
    incremental: bool,

    /// Clean cache before processing.
    #[arg(long)]
    fresh: bool,

    /// Restrict to use only 1 (worker) thread.
    #[arg(long)]
    unconcurrent: bool,
}

impl Cli {
    pub fn algo(&self) -> ConcurrentHashingAlgorithm { self.algo }
    pub fn style(&self) -> PathStyle { self.style }
    pub fn purge_db(&self) -> bool { self.fresh }
    pub fn parallel(&self) -> bool { !self.unconcurrent }
    pub fn directory(&self) -> &Path { &self.directory }
    pub fn incremental(&self) -> bool { self.incremental }
}
