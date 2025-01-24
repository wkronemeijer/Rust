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
}

impl PathStyle {
    pub fn apply(self, path: &Path) -> crate::Result<PathBuf> {
        Ok(match self {
            Self::Relative => path.to_owned(),
            Self::Absolute => absolute(path)?,
            Self::Canonical => canonicalize(path)?,
        })
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
}

// Cute but unnecessary
// pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);
