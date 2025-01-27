//! Items to compute the hash of a set of files, concurrently.

use std::fmt;
use std::num::NonZero;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread::scope;

use clap::ValueEnum;

use crate::hash::FileHash;
use crate::hash::PathWithHash;

///////////////////////////
// Parameters and return //
///////////////////////////

// pub type HashFilesParamters<'a, 'b: 'a> = &'b[&'a Path]
// I need to split the lifetime of the slice from the lifetime of the path
// ...don't know how 😅

pub struct HashFilesOptions {
    pub parallelism: NonZero<usize>,
}

pub type HashFilesResult<'a> = Vec<PathWithHash<'a>>;

////////////
// Search //
////////////

fn hash_files_mpsc<'a>(
    files: &[&'a Path],
    HashFilesOptions { parallelism, .. }: HashFilesOptions,
) -> HashFilesResult<'a> {
    // IDEA:
    // N workers each process a chunk of the files, hashing each file
    // then send the (path, hash) through a channel
    // recv then inserts them into the result
    const CHANNEL_SIZE: usize = 1 << 8;

    let file_count = files.len();
    let worker_count = parallelism.get();
    let chunk_size = file_count.div_ceil(worker_count);
    // 10 files / 4 chunks ==> 3 files per chunk (and change)
    // div_ceil so that there are at most worker_count chunks

    let mut results = Vec::with_capacity(file_count);
    scope(|s| {
        let (sender, receiver) = mpsc::sync_channel(CHANNEL_SIZE);

        // Worker
        for files_chunk in files.chunks(chunk_size) {
            let sender = sender.clone();
            s.spawn(move || {
                for &path in files_chunk {
                    let Ok(hash) = FileHash::from_contents(path) else {
                        eprintln!("failed to hash {:?}", path);
                        continue;
                    };
                    sender.send((path, hash)).unwrap();
                }
            });
        }

        // Collector
        let results = &mut results;
        s.spawn(move || {
            while let Ok(item) = receiver.recv() {
                results.push(item);
            }
        });
    });
    results
}

////////////////
// Search 2.0 //
////////////////

fn hash_files_mutex<'a>(
    files: &[&'a Path],
    HashFilesOptions { parallelism, .. }: HashFilesOptions,
) -> HashFilesResult<'a> {
    // IDEA:
    // N workers each have a chunk of paths to turn into hashes
    // (path, hash) go into a vec
    // if mutex.try_lock() {for each in vec {insert(path, hash)}}
    // Big Q is whether this is faster or not
    const BACKLOG_CAPACITY: usize = 1 << 5;
    const BACKLOG_DRAIN_THRESHOLD: usize =
        (BACKLOG_CAPACITY >> 2) + (BACKLOG_CAPACITY >> 1);
    let file_count = files.len();
    let worker_count = parallelism.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let results = Arc::new(Mutex::new(Vec::with_capacity(file_count)));
    scope(|s| {
        for files_chunk in files.chunks(chunk_size) {
            let results = results.clone();
            s.spawn(move || {
                let mut backlog = Vec::with_capacity(BACKLOG_CAPACITY);
                for &file in files_chunk {
                    let Ok(hash) = FileHash::from_contents(file) else {
                        eprintln!("failed to hash {:?}", file);
                        continue;
                    };
                    backlog.push((file, hash));
                    if backlog.len() >= BACKLOG_DRAIN_THRESHOLD {
                        // NB: non-blocking lock()
                        if let Ok(mut results) = results.try_lock() {
                            results.extend(backlog.drain(..));
                        }
                    }
                }
                // NB: blocking lock()
                if let Ok(mut results) = results.lock() {
                    results.extend(backlog.drain(..));
                }
            });
        }
    });
    // threads have joined, so there is exactly 1 reference to an unlocked mutex
    Arc::into_inner(results).unwrap().into_inner().unwrap()
}

/////////////////////////
// Choice of algorithm //
/////////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum ConcurrentHashingAlgorithmName {
    #[default]
    Mpsc,
    Mutex,
}

type ConcurrentHashingAlgorithm =
    for<'a> fn(&[&'a Path], HashFilesOptions) -> Vec<(&'a Path, FileHash)>;

impl ConcurrentHashingAlgorithmName {
    fn function(self) -> ConcurrentHashingAlgorithm {
        match self {
            Self::Mpsc => hash_files_mpsc,
            Self::Mutex => hash_files_mutex,
        }
    }

    pub fn hash_files<'a>(
        self,
        files: &[&'a Path],
        options: HashFilesOptions,
    ) -> HashFilesResult<'a> {
        let file_count = files.len();
        if file_count == 0 {
            return vec![];
        }
        let result = self.function()(files, options);
        let result_count = result.len();
        debug_assert_eq!(
            file_count, result_count,
            "input and output count must be equal"
        );
        result
    }
}

impl fmt::Display for ConcurrentHashingAlgorithmName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{self:?}").to_ascii_lowercase().fmt(f)
    }
}
