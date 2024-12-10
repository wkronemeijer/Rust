#![forbid(unsafe_code)]

use voxelsandbox::manifest::APPLICATION_NAME;
use voxelsandbox::run;
use voxelsandbox::Result;

fn main() -> Result {
    // In the future we could parse CLI options here
    let name = APPLICATION_NAME;
    println!("starting {name}");
    run()?;
    println!("exited {name} successfully");
    Ok(())
}
