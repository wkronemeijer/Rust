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
    duplicate_count: usize,
    file_count: usize,
}

impl<'a> Deduplicator<'a> {
    /// Creates an empty deduplicator with atleast the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Deduplicator {
            entries: HashMap::with_capacity(cap),
            duplicate_count: 0,
            file_count: 0,
        }
    }

    /// Creates an empty findings structure.
    pub fn new() -> Self { Deduplicator::with_capacity(0) }

    /// The number of duplicates encountered
    pub fn duplicate_count(&self) -> usize { self.duplicate_count }

    /// The number of files visited
    pub fn file_count(&self) -> usize { self.file_count }

    /// Registers the hash for a given path
    pub fn insert(&mut self, (path, hash): (&'a Path, &'a FileHash)) {
        let paths = self.entries.entry(hash).or_insert_with(TinyVec::new);
        paths.push(path);
        // 0 -> 1 (+0 duplicates)
        // 1 -> 2 (+2 duplicates)
        // 2 -> 3 (+1 duplicates)
        match paths.len() {
            0 => unreachable!(),
            1 => (),
            2 => self.duplicate_count += 2,
            _ => self.duplicate_count += 1,
        }
        self.file_count += 1;
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
