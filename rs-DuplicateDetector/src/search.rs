//! Items to find duplicates with

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use crate::core::collections::TinyVec;
use crate::core::fs::read_dir_all;
use crate::hash::FileHash;

//////////////
// Findings //
//////////////

pub type FindingsEntry = TinyVec<PathBuf>;

/// Stores the hashes and paths of all searched files.
pub struct Findings {
    entries: HashMap<FileHash, FindingsEntry>,
    file_count: u64,
}

impl Findings {
    /// Creates an empty findings structure.
    fn new() -> Self { Findings { entries: HashMap::new(), file_count: 0 } }

    /// The number of files visited
    pub fn file_count(&self) -> u64 { self.file_count }

    /// Registers the hash for a given path
    fn insert(&mut self, path: PathBuf, hash: FileHash) {
        self.file_count += 1;
        self.entries.entry(hash).or_insert_with(FindingsEntry::new).push(path);
    }

    /// Iterates over all hashes and paths.
    pub fn iter(&self) -> impl Iterator<Item = (&FileHash, &FindingsEntry)> {
        self.entries.iter()
    }
}

////////////
// Search //
////////////

pub fn find_duplicates(dir: &Path) -> crate::Result<Findings> {
    let mut findings = Findings::new();

    let read_dir_timer = Instant::now();
    let files = read_dir_all(dir)?;
    println!("read_dir_all in {}ms", read_dir_timer.elapsed().as_millis());

    let hash_timer = Instant::now();
    for file in files {
        let hash = FileHash::from_contents(&file)?;
        findings.insert(file, hash);
    }
    println!("insert_hash in {}ms", hash_timer.elapsed().as_millis());

    Ok(findings)
}
