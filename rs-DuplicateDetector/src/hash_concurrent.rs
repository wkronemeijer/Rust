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
// FindDuplicatesOptions //
///////////////////////////

#[derive(Debug)]
pub struct HashFilesOptions<'a> {
    pub files: &'a [&'a Path],
    pub parallelism: NonZero<usize>,
}

pub type HashFilesResult = Vec<PathWithHash>;

////////////
// Search //
////////////

fn hash_files_mpsc(
    HashFilesOptions { files, parallelism, .. }: HashFilesOptions,
) -> HashFilesResult {
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
                        // TODO: error channel?
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
            while let Ok((path, hash)) = receiver.recv() {
                results.push((path.to_path_buf(), hash));
            }
        });
    });
    results
}

////////////////
// Search 2.0 //
////////////////

fn hash_files_mutex(
    HashFilesOptions { files, parallelism, .. }: HashFilesOptions,
) -> HashFilesResult {
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
                    backlog.push((file.to_path_buf(), hash));
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
    Arc::into_inner(results).unwrap().into_inner().unwrap()
}

/////////////////////////
// Choice of algorithm //
/////////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum ConcurrentHashingAlgorithm {
    #[default]
    Mpsc,
    Mutex,
}

impl ConcurrentHashingAlgorithm {
    pub fn hash_files(self, options: HashFilesOptions) -> HashFilesResult {
        let file_count = options.files.len();
        let result = match self {
            Self::Mpsc => hash_files_mpsc(options),
            Self::Mutex => hash_files_mutex(options),
        };
        let result_count = result.len();
        debug_assert_eq!(file_count, result_count);
        result
    }
}

impl fmt::Display for ConcurrentHashingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{self:?}").to_ascii_lowercase().fmt(f)
    }
}
