//! Stuff that should be in [`core`], but isn't.

pub mod collections {
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

        fn into_push(self, new_path: T) -> Self {
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
            *self = take(self).into_push(new_path);
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
                Single(path) => Box::new(once(path)),
                Multiple(paths) => Box::new(paths.iter()),
            }
        }
    }

    impl<'a, T> IntoIterator for &'a TinyVec<T> {
        type Item = &'a T;
        type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
        fn into_iter(self) -> Self::IntoIter { self.iter() }
    }
}

pub mod fs {
    use std::collections::VecDeque;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;

    /// Recursively reads a directory and returns a list of all files.
    /// Returned paths are relative to the given directory.
    pub fn read_dir_all(dir: &Path) -> io::Result<Vec<PathBuf>> {
        // FIXME: One circular symbolic link and this blows up
        // TODO: Maybe turns this into an iterator?
        let mut frontier = VecDeque::new();
        let mut visited = Vec::new();

        frontier.push_back(dir.to_owned());
        while let Some(dir) = frontier.pop_front() {
            for item in fs::read_dir(dir)? {
                let path = item?.path();
                let stat = fs::metadata(&path)?;
                if stat.is_dir() {
                    frontier.push_back(path);
                } else if stat.is_file() {
                    visited.push(path);
                } else {
                    // only visited if we symlink_metdata
                }
            }
        }
        Ok(visited)
    }
}
