use std::ops::Deref;
use std::thread::available_parallelism;
use std::time::Instant;

use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::core::fs::read_dir_all;

macro_rules! println_time {
    ($e:expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();

        println!("{} in {}ms", stringify!($e), duration.as_millis());
        result
    }};
}

pub fn main() -> crate::Result {
    #![allow(unused_imports)]
    use duplicate_detector::search::find_duplicates_mpsc;
    use duplicate_detector::search::find_duplicates_mutex;

    let cli = Cli::parse(); // NB: parse exits on failure
    let style = cli.path_style();
    let directory = cli.directory();
    let parallelism = match cli.parallel() {
        true => available_parallelism()?.get(),
        false => 1,
    };

    ////////////
    // Search //
    ////////////

    println!("searching...");

    // Turn Vec<PathBuf>...
    let files = println_time!(read_dir_all(directory)?);
    let files = Vec::from_iter(files.iter().map(Deref::deref));
    let files = files.as_slice();
    // ...into &[&Path]
    let findings = println_time!(find_duplicates_mutex(files, parallelism));

    let file_count = findings.file_count();
    debug_assert_eq!(files.len(), file_count);

    println!("search complete");
    println!();

    /////////////////////
    // List duplicates //
    /////////////////////

    let mut duplicate_count = 0;

    for (hash, paths) in findings.iter() {
        let count = paths.len();
        if count > 1 {
            duplicate_count += count;
            println!("{count} file(s) with duplicate hash '{hash}':");
            for path in paths {
                println!("{}", style.try_apply(path).display());
            }
            println!();
        }
    }

    /////////////
    // Summary //
    /////////////

    println!(
        "found {} duplicate(s) amongst {} file(s)",
        duplicate_count, file_count,
    );
    Ok(())
}
