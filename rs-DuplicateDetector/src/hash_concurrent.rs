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
use crate::core::collections::nonempty::NonEmptyVec;
use crate::core::error::AggregateError;
use crate::core::error::partition_results;
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

type ReturnItem<'a> = (&'a Path, FileHash);

type Return<'a> = crate::Result<Vec<ReturnItem<'a>>>;

fn process_one(path: &Path) -> crate::Result<ReturnItem> {
    let hash_result = FileHash::from_contents(path);
    match hash_result {
        Ok(hash) => Ok((path, hash)),
        Err(error) => {
            let context = format!("failed to hash {}", path.display());
            Err(error.context(context))
        },
    }
}

/// Turns a sequence of results into a result of a sequence.
///
/// Unlike the stdlib conversion method,
/// this function accumulates all errors into an [`AggregateError`].
fn sequence<T, I>(iter: I) -> crate::Result<Vec<T>>
where
    I: IntoIterator<Item = crate::Result<T>>,
{
    let (values, errors) = partition_results(iter);
    if let Some(errors) = NonEmptyVec::new(errors) {
        Err(crate::Error::new(AggregateError::new(errors)))
    } else {
        Ok(values)
    }
}

////////////
// Search //
////////////

// IDEA:
// N workers each process a chunk of the files, hashing each file
// then send the (path, hash) through a channel
// recv then inserts them into the result
fn algorithm_mpsc(Options { files, threads, .. }: Options) -> Return {
    const CHANNEL_SIZE: usize = 1 << 8;

    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let mut results = Vec::with_capacity(file_count);
    thread::scope(|scope| {
        let (sender, receiver) = mpsc::sync_channel(CHANNEL_SIZE);

        // Worker
        for files_chunk in files.chunks(chunk_size) {
            let sender = sender.clone();
            scope.spawn(move || {
                for &path in files_chunk {
                    sender.send(process_one(path)).unwrap();
                }
            });
        }

        // Collector
        let results = &mut results;
        scope.spawn(move || {
            while let Ok(item) = receiver.recv() {
                results.push(item);
            }
        });
    });
    sequence(results)
}

////////////////
// Search 2.0 //
////////////////

// IDEA:
// N workers each have a chunk of paths to turn into hashes
// (path, hash) go into a vec
// if mutex.try_lock() {for each in vec {insert(path, hash)}}
// Big Q is whether this is faster or not
fn algorithm_arc_mutex(Options { files, threads, .. }: Options) -> Return {
    const POOL_CAP: usize = 1 << 5;
    const POOL_DRAIN_THRESHOLD: usize = (POOL_CAP >> 2) + (POOL_CAP >> 1);

    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    let results = Arc::new(Mutex::new(Vec::with_capacity(file_count)));
    thread::scope(|scope| {
        for files_chunk in files.chunks(chunk_size) {
            let results = results.clone();
            scope.spawn(move || {
                let mut pool = Vec::with_capacity(POOL_CAP);
                for &file in files_chunk {
                    pool.push(process_one(file));
                    // NB: non-blocking lock()
                    if pool.len() >= POOL_DRAIN_THRESHOLD {
                        if let Ok(mut results) = results.try_lock() {
                            results.extend(pool.drain(..));
                        }
                    }
                }
                // NB: blocking lock()
                if let Ok(mut results) = results.lock() {
                    results.extend(pool.drain(..));
                }
            });
        }
    });
    // threads have joined, so there is exactly 1 reference to an unlocked mutex
    sequence(Arc::into_inner(results).unwrap().into_inner().unwrap())
}

////////////////
// Search 3.0 //
////////////////

// IDEA:
// N workers workings over a chunk of (&Path, Option<FileHash>)
// Replace None with Some
// Then assert at the end
fn algorithm_lock_free(Options { files, threads, .. }: Options) -> Return {
    let file_count = files.len().get();
    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);

    // None == not yet computed
    // Some(Ok(hash)) == hash computed
    // Some(Err(_)) == failed to hash
    let mut results: Vec<(&Path, Option<crate::Result<FileHash>>)> =
        files.iter().map(|&file| (file, None)).collect();

    thread::scope(|scope| {
        for chunk in results.chunks_mut(chunk_size) {
            scope.spawn(move || {
                for (file, hash) in chunk {
                    *hash = Some(FileHash::from_contents(file));
                }
            });
        }
    });

    sequence(results.into_iter().map(|(path, hash)| Ok((path, hash.unwrap()?))))
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
            Self::Mpsc => algorithm_mpsc,
            Self::ArcMutex => algorithm_arc_mutex,
            Self::LockFree => algorithm_lock_free,
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
        let Some(files) = NonEmptySlice::new(files) else { return Ok(vec![]) };

        let file_count = files.len().get();
        let thread_count = threads.get();
        let function = algorithm.implementation();
        let result = {
            let timer = Instant::now();
            let value = function(Options { files, threads })?;
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
        Ok(result)
    }
}
