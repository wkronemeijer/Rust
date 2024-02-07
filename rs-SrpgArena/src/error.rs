use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("other: {0}")]
    Other(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;
