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
pub struct Database {
    files: HashMap<PathBuf, FileHash>,
}

// Domain-specific methods
impl Database {
    pub fn add(&mut self, path: PathBuf, hash: FileHash) {
        self.files.insert(path, hash);
    }

    pub fn remove(&mut self, path: &Path) { self.files.remove(path); }

    pub fn clear(&mut self) { self.files.clear() }

    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.files.iter().map(|(path, _)| path.deref())
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Path, &FileHash)> {
        self.files.iter().map(|(path, hash)| (path.deref(), hash))
    }
}
