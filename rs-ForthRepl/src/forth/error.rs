#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid word name: {0}")]
    InvalidWordName(String),
    #[error("missing name for definition")]
    MissingName,
    #[error("name already in use: {0}")]
    NameAlreadyInUse(String),
    #[error("can not nest ':'")]
    NestedCompile,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown word: {0}")]
    UnknownWord(String),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
