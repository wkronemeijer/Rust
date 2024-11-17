use std::fmt;
use std::ops::Deref;

use super::value::ValueKind;

///////////
// Error //
///////////

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid word name: {0}")]
    InvalidWordName(CompactString),
    #[error("missing name for definition")]
    MissingName,
    #[error("name already in use: {0}")]
    NameAlreadyInUse(CompactString),
    #[error("can not nest ':'")]
    NestedCompile,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unknown word: {0}")]
    UnknownWord(CompactString),
    #[error("cannot convert from {0} to {1}")]
    TypeError(ValueKind, ValueKind),
    #[error("integer overflow")]
    IntRangeError,
}

////////////
// Result //
////////////

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

///////////////////
// CompactString //
///////////////////
// Goal: reduce inline size, not allocation frequency or size
// TODO: Find a recently updated crate that does this for you

#[derive(Debug)]
pub struct CompactString {
    // Wasteful, but effective at reducing size
    boxed_string: Box<String>,
}

impl CompactString {
    pub fn from_box(string: Box<String>) -> Self {
        Self { boxed_string: string }
    }

    pub fn from_string(string: String) -> Self {
        Self { boxed_string: Box::new(string) }
    }

    pub fn from_str(str: &str) -> Self {
        Self { boxed_string: Box::new(str.to_string()) }
    }

    pub fn str(&self) -> &str { &*self.boxed_string }
}

impl Deref for CompactString {
    type Target = str;

    fn deref(&self) -> &Self::Target { self.str() }
}

impl From<&str> for CompactString {
    fn from(value: &str) -> Self { Self::from_str(value) }
}

impl From<String> for CompactString {
    fn from(value: String) -> Self { Self::from_string(value) }
}

impl fmt::Display for CompactString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.boxed_string.fmt(f)
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_string_is_compact() {
        assert!(
            size_of::<CompactString>() <= size_of::<usize>(),
            "compact string should be no larger than usize"
        );
    }
}
