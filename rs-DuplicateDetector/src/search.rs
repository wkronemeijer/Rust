//! Items to find duplicates with.

use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
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
    file_count: usize,
}

impl Findings {
    /// Creates an empty findings structure.
    fn new() -> Self { Findings { entries: HashMap::new(), file_count: 0 } }

    /// The number of files visited
    pub fn file_count(&self) -> usize { self.file_count }

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

pub fn find_duplicates_mpsc(files: &[&Path], parallelism: usize) -> Findings {
    // IDEA:
    // N workers each process a chunk of the files, hashing each file
    // then send the (path, hash) through a channel
    // recv then inserts them into the result
    const CHANNEL_SIZE: usize = 1 << 8;

    let mut results = Findings::new();
    scope(|s| {
        let file_count = files.len();
        let worker_count = parallelism.max(1);
        let chunk_size = file_count.div_ceil(worker_count);
        // 10 files / 4 chunks ==> 3 files per chunk (and change)
        // div_ceil so that there are at most worker_count chunks

        let (sender, receiver) = mpsc::sync_channel(CHANNEL_SIZE);

        /////////////
        // Workers //
        /////////////

        for files_chunk in files.chunks(chunk_size) {
            let sender = sender.clone();
            s.spawn(move || {
                for &path in files_chunk {
                    let Ok(hash) = FileHash::from_contents(path) else {
                        // TODO: error channel?
                        eprintln!("failed to hash {:?}", path);
                        continue;
                    };
                    sender.send((path, hash)).unwrap();
                }
            });
        }

        ///////////////
        // Collector //
        ///////////////

        let results = &mut results;
        s.spawn(move || {
            while let Ok((path, hash)) = receiver.recv() {
                results.insert(path.to_path_buf(), hash);
            }
        });
    });
    results
}

////////////////
// Search 2.0 //
////////////////

pub fn find_duplicates_mutex(files: &[&Path], parallelism: usize) -> Findings {
    // IDEA:
    // N workers each have a chunk of paths to turn into hashes
    // (path, hash) go into a vec
    // if mutex.try_lock() {for each in vec {insert(path, hash)}}
    // Big Q is whether this is faster or not
    const BACKLOG_CAPACITY: usize = 1 << 5;
    const BACKLOG_DRAIN_THRESHOLD: usize =
        (BACKLOG_CAPACITY >> 2) + (BACKLOG_CAPACITY >> 1);

    let results = Arc::new(Mutex::new(Findings::new()));
    scope(|s| {
        let file_count = files.len();
        let worker_count = parallelism.saturating_sub(1).add(1);
        let chunk_size = file_count.div_ceil(worker_count);

        for files_chunk in files.chunks(chunk_size) {
            let results = results.clone();
            s.spawn(move || {
                let mut backlog = Vec::with_capacity(BACKLOG_CAPACITY);
                for &file in files_chunk {
                    let Ok(hash) = FileHash::from_contents(file) else {
                        // TODO: error channel?
                        eprintln!("failed to hash {:?}", file);
                        continue;
                    };
                    backlog.push((file.to_path_buf(), hash));
                    if backlog.len() >= BACKLOG_DRAIN_THRESHOLD {
                        if let Ok(mut results) = results.try_lock() {
                            for (file, hash) in backlog.drain(..) {
                                results.insert(file, hash);
                            }
                        }
                    }
                }
                if let Ok(mut results) = results.lock() {
                    for (file, hash) in backlog.drain(..) {
                        results.insert(file, hash);
                    }
                }
            });
        }
    });
    Arc::into_inner(results).unwrap().into_inner().unwrap()
}
