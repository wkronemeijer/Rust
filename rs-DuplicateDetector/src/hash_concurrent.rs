//! Items to compute the hash of a set of files, concurrently.

use std::num::NonZero;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use clap::ValueEnum;
use strum::Display;

use crate::core::collections::nonempty::NonEmptySlice;
use crate::hash::FileHash;

///////////////////////////
// Parameters and return //
///////////////////////////

// pub type HashFilesParamters<'a, 'b: 'a> = &'b[&'a Path]
// I need to split the lifetime of the slice from the lifetime of the path
// ...don't know how 😅

struct Options<'a> {
    pub files: NonEmptySlice<'a, &'a Path>,
    pub threads: NonZero<usize>,
}

type Return<'a> = Vec<(&'a Path, FileHash)>;

////////////
// Search //
////////////

// IDEA:
// N workers each process a chunk of the files, hashing each file
// then send the (path, hash) through a channel
// recv then inserts them into the result
fn hash_files_mpsc(Options { files, threads, .. }: Options) -> Return {
    const CHANNEL_SIZE: usize = 1 << 8;

    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let mut results = Vec::with_capacity(file_count);
    thread::scope(|s| {
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

// IDEA:
// N workers each have a chunk of paths to turn into hashes
// (path, hash) go into a vec
// if mutex.try_lock() {for each in vec {insert(path, hash)}}
// Big Q is whether this is faster or not
fn hash_files_arc_mutex(Options { files, threads, .. }: Options) -> Return {
    const BACKLOG_CAPACITY: usize = 1 << 5;
    const BACKLOG_DRAIN_THRESHOLD: usize =
        (BACKLOG_CAPACITY >> 2) + (BACKLOG_CAPACITY >> 1);

    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let results = Arc::new(Mutex::new(Vec::with_capacity(file_count)));
    thread::scope(|s| {
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

////////////////
// Search 3.0 //
////////////////

// IDEA:
// N workers workings over a chunk of (&Path, Option<FileHash>)
// Replace None with Some
// Then assert at the end
fn hash_files_lockfree(Options { files, threads, .. }: Options) -> Return {
    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let mut results: Vec<(&Path, Option<FileHash>)> =
        files.iter().map(|&file| (file, None)).collect();

    thread::scope(|s| {
        for chunk in results.chunks_mut(chunk_size) {
            s.spawn(move || {
                for (file, hash) in chunk {
                    *hash = FileHash::from_contents(file).ok();
                }
            });
        }
    });

    results.into_iter().map(|(path, hash)| (path, hash.unwrap())).collect()
}

/////////////////////////
// Choice of algorithm //
/////////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum, Display)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum AlgorithmName {
    Mpsc,
    ArcMutex,
    #[default]
    LockFree,
}

impl AlgorithmName {
    fn implementation(self) -> fn(Options) -> Return {
        match self {
            Self::Mpsc => hash_files_mpsc,
            Self::ArcMutex => hash_files_arc_mutex,
            Self::LockFree => hash_files_lockfree,
        }
    }
}

#[derive(Debug)]
pub struct HashFilesConfiguration {
    algorithm: AlgorithmName,
    threads: NonZero<usize>,
}

impl HashFilesConfiguration {
    pub fn new(algorithm: AlgorithmName, threads: NonZero<usize>) -> Self {
        HashFilesConfiguration { algorithm, threads }
    }

    pub fn run<'a>(self, files: &'a [&'a Path]) -> Return<'a> {
        let HashFilesConfiguration { algorithm, threads } = self;
        let Some(files) = NonEmptySlice::new(files) else { return vec![] };

        let file_count = files.len().get();
        let thread_count = threads.get();
        let function = algorithm.implementation();
        let result = {
            let timer = Instant::now();
            let value = function(Options { files, threads });
            let mut time = timer.elapsed();
            if time.is_zero() {
                time += Duration::from_millis(1); // to prevent dividing by 0
            }
            let rate = {
                let file_count = file_count as f64;
                let seconds = time.as_secs_f64();
                let thread_count = thread_count as f64;
                file_count / seconds / thread_count
            };
            eprintln!(
                "hashed {} file(s) in {}ms ({:.1} files/sec/thread)",
                file_count,
                time.as_millis(),
                rate
            );
            value
        };
        let result_count = result.len();
        debug_assert_eq!(
            file_count, result_count,
            "input and output count must be equal"
        );
        result
    }
}
