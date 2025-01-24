use std::iter;
use std::mem;

pub mod collections {
    use super::*;

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

    impl<T> TinyVec<T> {
        /// Creates a new empty vector. Makes no allocations.
        pub fn new() -> Self { TinyVec::Empty }

        /// Returns the number of items in this vector.
        pub fn len(&self) -> usize {
            match self {
                Self::Empty => 0,
                Self::Single(..) => 1,
                Self::Multiple(vec) => vec.len(),
            }
        }

        fn into_push(self, new_path: T) -> Self {
            use TinyVec::*;
            match self {
                Empty => Single(new_path),
                Single(old_path) => Multiple(vec![old_path, new_path]),
                Multiple(mut paths) => {
                    paths.push(new_path);
                    Multiple(paths)
                },
            }
        }

        /// Adds a new value.
        pub fn push(&mut self, new_path: T) {
            // ...is there ::core::mem method that does in one operation?
            *self = mem::take(self).into_push(new_path);
        }

        /// Consumes this vec and turns it into a proper [`Vec`].
        pub fn into_vec(self) -> Vec<T> {
            use TinyVec::*;
            match self {
                Empty => vec![],
                Single(item) => vec![item],
                Multiple(vec) => vec,
            }
        }
    }

    impl<T> TinyVec<T> {
        /// Iterates over the paths contained.
        pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
            // ...when is `gen` stabilized?
            match self {
                Self::Empty => Box::new(iter::empty()),
                Self::Single(path) => Box::new(iter::once(path)),
                Self::Multiple(paths) => Box::new(paths.iter()),
            }
        }
    }

    impl<'a, T> IntoIterator for &'a TinyVec<T> {
        type Item = &'a T;
        type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
        fn into_iter(self) -> Self::IntoIter { self.iter() }
    }
}
