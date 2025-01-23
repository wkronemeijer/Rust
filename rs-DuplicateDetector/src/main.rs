use std::env::args;
use std::fs::canonicalize;
use std::path::Path;

use anyhow::bail;
pub use duplicate_detector::Result;
use duplicate_detector::hash::FileHash;
use duplicate_detector::search::find_duplicates;

fn main_search(directory: &Path) -> crate::Result {
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
                // let path = absolute(path)?;
                let path = canonicalize(path)?;
                println!("• {}", path.display());
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

fn main_hash(file: &Path) -> crate::Result {
    let hash = FileHash::from_contents(file)?;
    println!("hash({file:?}) == {hash}");
    Ok(())
}

pub fn main() -> crate::Result {
    let args: Vec<String> = args().skip(1).collect(); // skip the executable 
    let args: Vec<&str> = args.iter().map(String::as_ref).collect();
    // ...is there a better way to be able to
    // match slices with string literals against args?
    match args[..] {
        ["search", dir] => main_search(Path::new(dir)),
        ["hash", file] => main_hash(Path::new(file)),
        _ => bail!("invalid arguments"),
    }
}
