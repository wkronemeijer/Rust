//! Contains additional error utilities;

use std::error::Error as StdError;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::collections::nonempty::NonEmptyVec;

////////////////////////
// Parition `Result`s //
////////////////////////

/// Splits a list of results into a list of values and errors.
pub fn partition_results<T, E, I>(iter: I) -> (Vec<T>, Vec<E>)
where I: IntoIterator<Item = Result<T, E>> {
    let iter = iter.into_iter();
    let size = iter.size_hint().0;
    let mut values = Vec::with_capacity(size);
    let mut errors = Vec::with_capacity(size);
    for item in iter {
        match item {
            Ok(v) => values.push(v),
            Err(e) => errors.push(e),
        }
    }
    values.shrink_to_fit();
    errors.shrink_to_fit();
    (values, errors)
}

/////////////////////
// Aggregate Error //
/////////////////////

#[derive(Debug)]
/// An error respresting a non-empty list of errors.
pub struct AggregateError {
    /// Invariant: never empty
    errors: NonEmptyVec<crate::Error>,
}

impl AggregateError {
    /// Creates a new aggregate error.
    pub fn new(errors: NonEmptyVec<crate::Error>) -> Self {
        AggregateError { errors }
    }
}

impl Deref for AggregateError {
    type Target = NonEmptyVec<crate::Error>;
    fn deref(&self) -> &Self::Target { &self.errors }
}

impl DerefMut for AggregateError {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.errors }
}

impl fmt::Display for AggregateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.errors.iter();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for rest in iter {
                "\n".fmt(f)?;
                rest.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl StdError for AggregateError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.errors.as_slice().first().as_ref())
    }
}
