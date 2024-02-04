#[derive(Debug)]
pub enum Error {
    // Singletons
    StackUnderflow,
    NumberName,

    NestedCompile,
    MissingBody,
    // With implicit context
    NameAlreadyInUse(String),
    InvalidWordName(String),
    UnknownWord(String),
    // With explicit context
    UnexpectedToken(String),
    // External
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StackUnderflow => {
                write!(f, "stack underflow")
            }
            Error::NumberName => {
                write!(f, "can not use a number as a word name")
            }
            Error::UnknownWord(word) => {
                write!(f, "unknown word: {}", word)
            }
            Error::NameAlreadyInUse(name) => {
                write!(f, "name already in use: {}", name)
            }
            Error::UnexpectedToken(token) => {
                write!(f, "unexpected token: {}", token)
            }
            Error::Other(inner) => {
                write!(f, "internal error: {}", inner)
            }
            Error::NestedCompile => {
                write!(f, "can not nest ':'")
            }
            Error::InvalidWordName(name) => {
                write!(f, "invalid word name: {}", name)
            }
            Error::MissingBody => {
                write!(f, "missing body for definition")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Other(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, value.to_string())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
