//! Items to compute the hash of a set of files, concurrently.

use std::io::stderr;
use std::num::NonZero;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::core::collections::nonempty::NonEmptySlice;
use crate::core::collections::nonempty::NonEmptyVec;
use crate::core::error::AggregateError;
use crate::core::error::partition_results;
use crate::hash::FileHash;
use crate::hash::FileHasher;
use crate::status_line::StatusLine;

///////////////////////////
// Parameters and return //
///////////////////////////

struct Options<'a> {
    pub files: NonEmptySlice<'a, &'a Path>,
    pub threads: NonZero<usize>,
}

type ReturnItem<'a> = (&'a Path, FileHash);

type Return<'a> = crate::Result<Vec<ReturnItem<'a>>>;

/// Turns a sequence of results into a result of a sequence.
///
/// Unlike the stdlib conversion method,
/// this function accumulates all errors into an [`AggregateError`].
fn sequence<T, I>(iter: I) -> crate::Result<Vec<T>>
where I: IntoIterator<Item = crate::Result<T>> {
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
    const UPDATE_PERIOD: Duration = Duration::from_millis(100);
    const WAIT_PERIOD: Duration = Duration::from_millis(500);
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
                let mut hasher = FileHasher::new();
                for &path in files_chunk {
                    let message = match hasher.from_contents(path) {
                        Ok(hash) => Ok((path, hash)),
                        Err(error) => Err(error.context(format!(
                            "failed to hash '{}'",
                            path.display()
                        ))),
                    };
                    if let Err(_) = sender.send(message) {
                        break;
                    }
                }
            });
        }

        // Collector
        let results = &mut results;
        scope.spawn(move || {
            let mut status_line = StatusLine::new(stderr().lock());
            let mut last_update = Instant::now();

            let mut update = |so_far: usize| {
                let percent = (100 * so_far / file_count).clamp(0, 100);
                status_line.writeln(&format!(
                    "hashed {} of {} file(s) ({}%)",
                    so_far, file_count, percent,
                ));
            };

            update(results.len());
            while let Ok(item) = receiver.recv() {
                results.push(item);
                if last_update.elapsed() > UPDATE_PERIOD {
                    last_update = Instant::now();
                    update(results.len());
                }
            }
            update(results.len());

            thread::sleep(WAIT_PERIOD);
            status_line.close();
        });
    });
    sequence(results)
}

/////////////////////////
// Choice of algorithm //
/////////////////////////

#[derive(Debug)]
/// Options for the algorithm.
pub struct HashFilesConfiguration {
    /// The number of threads to use.
    pub threads: NonZero<usize>,
}

impl HashFilesConfiguration {
    /// Uses the configuration to find duplicate files in a list of files.
    pub fn run<'a>(self, files: &'a [&'a Path]) -> Return<'a> {
        let HashFilesConfiguration { threads } = self;
        let Some(files) = NonEmptySlice::new(files) else { return Ok(vec![]) };
        let result = algorithm_mpsc(Options { files, threads })?;

        let in_count = files.len().get();
        let out_count = result.len();
        debug_assert_eq!(
            in_count, out_count,
            "input and output count must be equal"
        );

        Ok(result)
    }
}
