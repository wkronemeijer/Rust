//! Items to find duplicates with.

use std::collections::HashMap;
use std::path::Path;

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
