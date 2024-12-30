use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr as _;

use haskfmt::format;
use haskfmt::Options;
pub use haskfmt::Result;
use serde_json::value::Value;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("invalid arguments")]
    InvalidArguments,
}

fn print_formatted_file(filename: &str) -> crate::Result {
    let contents = read_to_string(filename)?;
    let json = Value::from_str(&contents)?;
    let formatted = format(&json, &Options { ..Default::default() })?;
    println!("{formatted}");
    Ok(())
}

fn main() -> crate::Result {
    let args: Vec<_> = args().skip(1).collect();
    match *args {
        [ref file] => print_formatted_file(file),
        _ => Err(crate::Error::InvalidArguments)?,
    }
}
