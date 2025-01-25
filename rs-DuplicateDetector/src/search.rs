//! Items to find duplicates with.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::collections::TinyVec;
use crate::hash::FileHash;
use crate::hash::PathWithHash;

//////////////
// Findings //
//////////////

/// Stores the hashes and paths of all searched files.
pub struct Findings {
    entries: HashMap<FileHash, TinyVec<PathBuf>>,
    file_count: usize,
}

impl Findings {
    /// Creates an empty findings structure.
    fn new() -> Self { Findings { entries: HashMap::new(), file_count: 0 } }

    /// The number of files visited
    pub fn file_count(&self) -> usize { self.file_count }

    /// Registers the hash for a given path
    pub fn insert(&mut self, path: PathBuf, hash: FileHash) {
        self.file_count += 1;
        self.entries.entry(hash).or_insert_with(TinyVec::new).push(path);
    }

    /// Iterates over all hashes and paths.
    pub fn iter(&self) -> impl Iterator<Item = (&FileHash, &[PathBuf])> {
        self.entries.iter().map(|(k, v)| (k, v.as_slice()))
    }

    /// Iterates over all entries that have more than 1 file.
    pub fn duplicates(&self) -> impl Iterator<Item = (&FileHash, &[PathBuf])> {
        self.iter().filter(|(_, files)| files.len() > 1)
    }
}

impl FromIterator<PathWithHash> for Findings {
    fn from_iter<I: IntoIterator<Item = PathWithHash>>(iter: I) -> Self {
        let mut result = Self::new();
        for (path, hash) in iter {
            result.insert(path, hash);
        }
        result
    }
}
