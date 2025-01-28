use std::iter::empty;
use std::iter::once;
use std::mem::take;
use std::slice;

/// Like [`Vec`] but stores up to a single element inline,
/// then switches to heap allocation when more elements are added.
///
/// Similar to [`tinyvec::TinyVec`](https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html), but does not require [`Default`].
#[derive(Debug, Default, Clone)]
pub enum TinyVec<T> {
    #[default]
    Empty,
    Single(T),
    Multiple(Vec<T>),
}

use TinyVec::Empty;
use TinyVec::Multiple;
use TinyVec::Single;

impl<T> TinyVec<T> {
    /// Creates a new empty vector. Makes no allocations.
    pub fn new() -> Self { Empty }

    /// Returns the number of items in this vector.
    pub fn len(&self) -> usize {
        match self {
            Empty => 0,
            Single(..) => 1,
            Multiple(vec) => vec.len(),
        }
    }

    fn into_push(self, new: T) -> Self {
        match self {
            Empty => Single(new),
            Single(old) => Multiple(vec![old, new]),
            Multiple(mut vec) => {
                vec.push(new);
                Multiple(vec)
            },
        }
    }

    /// Adds a new value.
    pub fn push(&mut self, new: T) {
        // ...is there ::core::mem method that does in one operation?
        *self = take(self).into_push(new);
    }

    /// Returns a slice of the entire contents.
    pub fn as_slice(&self) -> &[T] {
        match self {
            Empty => &[],
            Single(item) => slice::from_ref(item),
            Multiple(vec) => vec.as_slice(),
        }
    }

    /// Consumes this vec and turns it into a proper [`Vec`].
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Empty => vec![],
            Single(item) => vec![item],
            Multiple(vec) => vec,
        }
    }
}

// TODO: Create a dedicated Iter for TinyVec
impl<T> TinyVec<T> {
    /// Iterates over the paths contained.
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        // ...when is `gen` stabilized?
        match self {
            Empty => Box::new(empty()),
            Single(item) => Box::new(once(item)),
            Multiple(vec) => Box::new(vec.iter()),
        }
    }
}

impl<'a, T> IntoIterator for &'a TinyVec<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
