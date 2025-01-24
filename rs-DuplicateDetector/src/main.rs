use std::ops::Deref;
use std::path::Path;
use std::thread::available_parallelism;
use std::time::Instant;

use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::core::fs::read_dir_all;
use duplicate_detector::search::find_duplicates;

pub fn main() -> crate::Result {
    let cli = Cli::parse(); // NB: parse exits on failure
    let directory = cli.directory();
    let style = cli.path_style()?;

    ////////////
    // Search //
    ////////////

    println!("searching...");
    let search_timer = Instant::now();

    let read_timer = Instant::now();
    let files = read_dir_all(directory)?;
    println!("read_dir_all() in {}ms", read_timer.elapsed().as_millis());
    let files: Vec<&Path> = files.iter().map(Deref::deref).collect();
    let files = files.as_slice();

    let find_timer = Instant::now();
    let findings = find_duplicates(files, match cli.parallel() {
        true => available_parallelism()?.get(),
        false => 1,
    });
    let search_time = search_timer.elapsed().as_millis();
    println!("find_duplicates() in {}ms", find_timer.elapsed().as_millis());

    let file_count = findings.file_count();
    println!("found {} file(s)", file_count);
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

    if duplicate_count != 0 {
        println!(
            "found {} duplicate(s) amongst {} file(s) in {}ms",
            duplicate_count, file_count, search_time,
        );
    } else {
        println!(
            "no duplicates found amongst {} file(s) in {}ms",
            file_count, search_time,
        );
    }
    Ok(())
}
