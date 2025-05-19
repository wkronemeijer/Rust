//! Manages a (virtual) file

use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::hash::FileHash;

//////////////
// Database //
//////////////

#[derive(Debug, Default, Serialize, Deserialize)]
/// Stores the mapping between path to files and those files' hashes.
pub struct Database {
    files: HashMap<PathBuf, FileHash>,
}

impl Database {
    /// Adds a record to this database.
    pub fn add(&mut self, path: PathBuf, hash: FileHash) {
        self.files.insert(path, hash);
    }
    /// Removes a record from this database.
    pub fn remove(&mut self, path: &Path) { self.files.remove(path); }

    /// Clears the entire database.
    pub fn clear(&mut self) { self.files.clear() }

    /// Returns all paths in this database.
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.files.iter().map(|(path, _)| path.deref())
    }

    /// Returns all records in this database.
    pub fn entries(&self) -> impl Iterator<Item = (&Path, &FileHash)> {
        self.files.iter().map(|(path, hash)| (path.deref(), hash))
    }
}
