use std::env::args;
use std::path::Path;
use std::process::Command;

use anyhow::bail;
pub use duplicate_detector::Result;

fn main() -> crate::Result {
    let mut args = args();
    let Some(dir) = args.next() else { bail!("missing executable location") };
    let Some(dir) = Path::new(&dir).parent() else { bail!("missing parent") };
    let Some(subcommand) = args.next() else { bail!("missing subcommand") };

    let Some(name) = Path::new(&subcommand).file_stem() else {
        bail!("invalid subcommand name")
    };
    let mut subcommand = dir.join(name);
    subcommand.set_extension("exe");

    let mut handle = Command::new(subcommand).args(args).spawn()?;
    handle.wait()?;
    Ok(())
}
