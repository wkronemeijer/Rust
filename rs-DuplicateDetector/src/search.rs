use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::fs::metadata;
use std::iter;
use std::mem;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use crate::hash::FileHash;

//////////////////
// FindingEntry //
//////////////////
// ...technically the value, the entry is (Hash, FindingEntry)

// NB: Most files aren't duplicate
// so a single item stored inline helps save allocation traffic
/// Stores all paths found with the same hash.
#[derive(Debug, Default, Clone)]
pub enum FindingsEntry {
    #[default]
    Empty,
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

impl FindingsEntry {
    /// Creates an empty entry.
    pub fn new() -> Self { FindingsEntry::default() }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Single(..) => 1,
            Self::Multiple(vec) => vec.len(),
        }
    }

    fn into_push(self, new_path: PathBuf) -> Self {
        use FindingsEntry::*;
        match self {
            Empty => Single(new_path),
            Single(old_path) => Multiple(vec![old_path, new_path]),
            Multiple(mut paths) => {
                paths.push(new_path);
                Multiple(paths)
            },
        }
    }

    /// Adds a new value.
    pub fn push(&mut self, new_path: PathBuf) {
        // Is there mem::_ method that does without a useless memcpy?
        *self = mem::take(self).into_push(new_path);
    }

    /// Iterates over the paths contained.
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Path> + 'a> {
        // ...when is `gen` stabilized?
        match self {
            Self::Empty => Box::new(iter::empty()),
            Self::Single(path) => Box::new(iter::once(path).map(Deref::deref)),
            Self::Multiple(paths) => Box::new(paths.iter().map(Deref::deref)),
        }
    }
}

impl<'a> IntoIterator for &'a FindingsEntry {
    type Item = &'a Path;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

//////////////
// Findings //
//////////////

/// Stores the hashes and paths of all searched files.
pub struct Findings {
    entries: HashMap<FileHash, FindingsEntry>,
}

impl Findings {
    /// Creates an empty findings structure.
    fn new() -> Self { Findings { entries: HashMap::new() } }

    /// Registers the hash for a given path
    fn insert(&mut self, path: PathBuf, hash: FileHash) {
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

// FIXME: One circular symbolic link and this blows up
fn read_dir_all(dir: &Path) -> crate::Result<Vec<PathBuf>> {
    let mut frontier = VecDeque::new();
    let mut visited = Vec::new();

    frontier.push_back(dir.to_owned());
    while let Some(dir) = frontier.pop_front() {
        for item in fs::read_dir(dir)? {
            let path = item?.path();
            let stat = metadata(&path)?;
            if stat.is_dir() {
                frontier.push_back(path);
            } else {
                debug_assert!(stat.is_file());
                visited.push(path);
            }
        }
    }
    Ok(visited)
}

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
