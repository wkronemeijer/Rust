use clap::Parser;
pub use duplicate_detector::Result;
use duplicate_detector::cli::Cli;
use duplicate_detector::search::find_duplicates;

pub fn main() -> crate::Result {
    let cli = Cli::parse(); // NB: parse exits on failure
    let directory = cli.directory();
    let style = cli.path_style()?;

    println!("searching...");
    let findings = find_duplicates(directory)?;
    println!("search complete");
    println!();

    let mut total_count = 0;
    for (hash, paths) in findings.iter() {
        let count = paths.len();
        if count > 1 {
            total_count += count;
            println!("{count} file(s) with duplicate hash '{hash}':");
            for path in paths {
                println!("{}", style.apply(path)?.display());
            }
            println!();
        }
    }
    if total_count != 0 {
        println!("found {total_count} duplicate(s) in total");
    } else {
        println!("no duplicates found");
    }
    Ok(())
}
