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
    file_count: usize,
}

impl<'a> Deduplicator<'a> {
    /// Creates an empty findings structure.
    fn new() -> Self { Deduplicator { entries: HashMap::new(), file_count: 0 } }

    /// The number of files visited
    pub fn file_count(&self) -> usize { self.file_count }

    /// Registers the hash for a given path
    pub fn insert(&mut self, (path, hash): (&'a Path, &'a FileHash)) {
        self.file_count += 1;
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
        let mut result = Self::new();
        for item in iter {
            result.insert(item);
        }
        result
    }
}
