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
    #[allow(dead_code)]
    #[error("other: {0}")]
    Other(#[from] Box<dyn std::error::Error>),
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown word: {0}")]
    UnknownWord(String),
}

pub type Result<T> = core::result::Result<T, Error>;
