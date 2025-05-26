//! Contains additional error utilities;

use std::error::Error as StdError;
use std::fmt;

/////////////////////////
// Partition `Result`s //
/////////////////////////

/// Turns a sequence of results into a result of a sequence.
///
/// Unlike the stdlib conversion method,
/// this function accumulates all errors into an [`AggregateError`].
pub fn sequence<T>(
    iter: impl IntoIterator<Item = crate::Result<T>>,
) -> crate::Result<Vec<T>> {
    let iter = iter.into_iter();
    let size_hint = iter.size_hint().0;
    let mut values = Vec::with_capacity(size_hint);
    let mut errors = Vec::with_capacity(size_hint);
    for item in iter {
        match item {
            Ok(v) => values.push(v),
            Err(e) => errors.push(e),
        }
    }
    values.shrink_to_fit();
    errors.shrink_to_fit();

    if let Some(aggregate) = AggregateError::new(errors) {
        Err(crate::Error::new(aggregate))
    } else {
        Ok(values)
    }
}

/////////////////////
// Aggregate Error //
/////////////////////

#[derive(Debug)]
/// An error respresting a non-empty list of errors.
pub struct AggregateError {
    /// Invariant: never empty
    errors: Vec<crate::Error>,
}

impl AggregateError {
    /// Creates a new aggregate error.
    pub fn new(errors: Vec<crate::Error>) -> Option<Self> {
        if errors.len() != 0 {
            Some(AggregateError { errors })
        } else {
            return None
        }
    }
}

impl fmt::Display for AggregateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        Some(self.errors.first().expect("aggregate was empty").as_ref())
    }
}
