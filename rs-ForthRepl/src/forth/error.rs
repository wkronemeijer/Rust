#[derive(Debug)]
pub enum Error {
    InvalidWordName(String),
    MissingBody,
    NameAlreadyInUse(String),
    NestedCompile,
    #[allow(dead_code)]
    Other(Box<dyn std::error::Error>),
    StackUnderflow,
    UnknownWord(String),
}

impl core::cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidWordName(l0), Self::InvalidWordName(r0)) => l0 == r0,
            (Self::NameAlreadyInUse(l0), Self::NameAlreadyInUse(r0)) => l0 == r0,
            (Self::Other(_), Self::Other(_)) => false,
            (Self::UnknownWord(l0), Self::UnknownWord(r0)) => l0 == r0,
            // FIXME: This swallows all variants, especially new variants with parameters
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StackUnderflow => {
                write!(f, "stack underflow")
            }
            Error::UnknownWord(word) => {
                write!(f, "unknown word: {}", word)
            }
            Error::NameAlreadyInUse(name) => {
                write!(f, "name already in use: {}", name)
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

pub type Result<T> = core::result::Result<T, Error>;
