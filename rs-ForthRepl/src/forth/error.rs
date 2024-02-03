use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ForthError {
    StackUnderflow,
    ParseError { culprit: String },
    UnknownWord { word: String },
    Other(Box<dyn Error>),
}

impl Display for ForthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForthError::StackUnderflow => write!(f, "stack underflow"),
            ForthError::ParseError { culprit } => write!(f, "parsing failed on: {}", culprit),
            ForthError::UnknownWord { word } => write!(f, "unknown word: {}", word),
            ForthError::Other(inner) => inner.fmt(f),
        }
    }
}

impl Error for ForthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ForthError::Other(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }
}

impl From<ForthError> for std::io::Error {
    fn from(value: ForthError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, value.to_string())
    }
}

pub type ForthResult<T> = core::result::Result<T, ForthError>;
