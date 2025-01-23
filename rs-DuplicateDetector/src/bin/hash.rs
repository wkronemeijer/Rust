use std::env::args;
use std::path::Path;

use anyhow::bail;
pub use duplicate_detector::Result;
use duplicate_detector::hash::hash_file;

fn main() -> crate::Result {
    let args: Vec<_> = args().skip(1).collect();
    match *args {
        [ref file] => {
            let file = Path::new(file);
            let hash = hash_file(file)?;
            println!("file = {file:?} \nhash = {hash}");
        },
        _ => bail!("invalid invocation"),
    }
    Ok(())
}
