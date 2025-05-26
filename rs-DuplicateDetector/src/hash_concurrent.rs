//! Items to compute the hash of a set of files, concurrently.

use std::io::stderr;
use std::num::NonZero;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::core::error::sequence;
use crate::hash::FileHash;
use crate::hash::FileHasher;
use crate::status_line::StatusLine;

///////////////////////////
// Parameters and return //
///////////////////////////

type Item<'a> = (&'a Path, FileHash);

////////////
// Search //
////////////

// IDEA:
// N workers each process a chunk of the files, hashing each file
// then send the (path, hash) through a channel
// recv then inserts them into the result
// recv also writes updates to stderr to show progress
fn algorithm_mpsc<'a>(
    files: &'a [&'a Path],
    threads: NonZero<usize>,
) -> crate::Result<Vec<Item<'a>>> {
    const UPDATE_PERIOD: Duration = Duration::from_millis(100);
    const WAIT_PERIOD: Duration = Duration::from_millis(500);
    const CHANNEL_SIZE: usize = 1 << 10;

    let file_count = files.len();

    if file_count == 0 {
        return Ok(vec![]);
    }

    let worker_count = threads.get();
    let chunk_size = file_count.div_ceil(worker_count);
    // NB: Given a>=1 and b>=1,
    // then ceil(a / b) >= 1
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
pub struct HashFilesOptions {
    /// The number of threads to use.
    pub threads: NonZero<usize>,
}

/// Hashes multiple files in parallel.
pub fn parallel_hash_files<'a>(
    files: &'a [&'a Path],
    HashFilesOptions { threads }: HashFilesOptions,
) -> crate::Result<Vec<Item<'a>>> {
    let result = algorithm_mpsc(files, threads)?;

    let in_count = files.len();
    let out_count = result.len();
    debug_assert_eq!(
        in_count, out_count,
        "input and output count must be equal"
    );

    Ok(result)
}
