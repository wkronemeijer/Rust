use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid word name: {0}")]
    InvalidWordName(String),
    #[error("missing body for definition")]
    MissingBody,
    #[error("name already in use: {0}")]
    NameAlreadyInUse(String),
    #[error("can not nest ':'")]
    NestedCompile,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown word: {0}")]
    UnknownWord(String),
    // External
    #[allow(dead_code)]
    #[error("other: {0}")]
    Other(#[from] Box<dyn std::error::Error>),
}

pub type Result<T> = result::Result<T, Error>;
