//! Items to find duplicates with.

use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::Path;
use std::path::absolute;

use clap::ValueEnum;
use strum::Display;

use crate::core::collections::tinyvec::TinyVec;
use crate::hash::FileHash;

//////////////
// Findings //
//////////////

/// Stores the hashes and paths of all searched files.
pub struct Deduplicator<'a> {
    entries: HashMap<&'a FileHash, TinyVec<&'a Path>>,
}

impl<'a> Deduplicator<'a> {
    /// Creates an empty deduplicator with atleast the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Deduplicator { entries: HashMap::with_capacity(cap) }
    }

    /// Creates an empty findings structure.
    pub fn new() -> Self { Deduplicator::with_capacity(0) }

    /// Registers the hash for a given path
    pub fn insert(&mut self, (path, hash): (&'a Path, &'a FileHash)) {
        self.entries.entry(hash).or_insert_with(TinyVec::new).push(path);
    }

    /// Iterates over all hashes and paths.
    pub fn iter(&self) -> impl Iterator<Item = (&FileHash, &[&'a Path])> {
        self.entries.iter().map(|(&k, v)| (k, v.as_slice()))
    }

    /// Iterates over all entries that have more than 1 file.
    pub fn duplicates(&self) -> impl Iterator<Item = (&FileHash, &[&'a Path])> {
        self.iter().filter(|(_, files)| files.len() > 1)
    }
}

impl<'a> FromIterator<(&'a Path, &'a FileHash)> for Deduplicator<'a> {
    fn from_iter<I: IntoIterator<Item = (&'a Path, &'a FileHash)>>(
        iter: I,
    ) -> Self {
        let iter = iter.into_iter();
        let mut result = Self::with_capacity(iter.size_hint().0);
        for item in iter {
            result.insert(item);
        }
        result
    }
}

/////////////////////
// Path Formatting //
/////////////////////

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
