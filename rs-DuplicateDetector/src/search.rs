//! Items to find duplicates with.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread::available_parallelism;
use std::thread::scope;

use crate::core::collections::TinyVec;
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

pub fn find_duplicates_parallel(files: &[&Path]) -> Findings {
    let mut results = Findings::new();
    scope(|s| {
        let threads = available_parallelism().expect("parallelism not found");
        let chunk_size = files.len().div_ceil(threads.into());
        // 10 files / 4 chunks ==> 3 files per chunk (and change)
        // ceil because chunk size is flexible, thread count is not

        const CHANNEL_SIZE: usize = 1 << 4;
        let (sender, receiver) = mpsc::sync_channel(CHANNEL_SIZE);

        /////////////
        // Workers //
        /////////////

        for files_chunk in files.chunks(chunk_size) {
            let sender = sender.clone();
            s.spawn(move || {
                for &file in files_chunk {
                    let Ok(hash) = FileHash::from_contents(file) else {
                        // TODO: error channel?
                        eprintln!("failed to hash {:?}", file);
                        continue;
                    };
                    let Ok(_) = sender.send((file, hash)) else {
                        // Technically this should never happen
                        break;
                    };
                }
            });
        }

        ///////////////
        // Collector //
        ///////////////

        let findings = &mut results;
        s.spawn(move || {
            while let Ok((path, hash)) = receiver.recv() {
                findings.insert(path.to_path_buf(), hash);
            }
        });
    });
    results
}

pub fn find_duplicates(files: &[&Path]) -> Findings {
    let mut results = Findings::new();
    for &file in files {
        let Ok(hash) = FileHash::from_contents(file) else {
            eprintln!("failed to hash {:?}", file);
            continue;
        };
        results.insert(file.to_path_buf(), hash);
    }
    results
}
