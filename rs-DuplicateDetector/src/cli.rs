use std::borrow::Cow;
use std::fmt;
use std::fs::canonicalize;
use std::path::Path;
use std::path::PathBuf;
use std::path::absolute;

use anyhow::bail;
use clap::Parser;

/////////////////////
// Path Formatting //
/////////////////////

#[derive(Debug, Default, Clone, Copy)]
pub enum PathStyle {
    #[default]
    Relative,
    Absolute,
    Canonical,
    // TODO: absolute file uri and canonical file uri
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

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct Cli {
    /// The directory to search.
    directory: PathBuf,

    #[arg(long)]
    parallel: bool,

    #[arg(long)]
    /// Keep path of duplicates relative.
    relative: bool,
    #[arg(long)]
    /// Make path of duplicates absolute.
    absolute: bool,
    #[arg(long)]
    /// Canonicalize path of duplicates.
    canonical: bool,
}

impl Cli {
    pub fn path_style(&self) -> crate::Result<PathStyle> {
        Ok(match (self.relative, self.absolute, self.canonical) {
            (false, false, false) => PathStyle::default(),
            (true, false, false) => PathStyle::Relative,
            (false, true, false) => PathStyle::Absolute,
            (false, false, true) => PathStyle::Canonical,
            _ => bail!("more than 1 path formatting option specified"),
        })
    }

    pub fn directory(&self) -> &Path { &self.directory }

    pub fn parallel(&self) -> bool { self.parallel }
}

// Cute but unnecessary
// pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);
