use std::time::Instant;

use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
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
    let findings = find_duplicates(directory)?;
    let search_time = search_timer.elapsed().as_millis();
    let file_count = findings.file_count();
    println!("searched {} file(s) in {}ms", file_count, search_time);
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
            "found {} duplicate(s) amongst {} file(s)",
            duplicate_count, file_count,
        );
    } else {
        println!("no duplicates found amongst {} file(s)", file_count);
    }
    Ok(())
}
