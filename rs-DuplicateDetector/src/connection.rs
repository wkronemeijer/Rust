use std::convert::Infallible;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

////////////////////
// Serde wrappers //
////////////////////

mod serde_format {
    // pub use serde_json::from_slice;
    // pub use serde_json::to_vec;
    pub use rmp_serde::from_slice;
    pub use rmp_serde::to_vec;
}

/// Saves a [`Serialize`]able value to a file at the given path.
fn save_to_file<T: Serialize>(path: &Path, value: &T) -> crate::Result {
    let contents = serde_format::to_vec(value)?;
    let bytes = contents.len();
    fs::write(path, contents)?;
    eprintln!("saved {} byte(s) to {}", bytes, path.display());
    Ok(())
}

/// Loads a [`Deserialize`]able value from a file at the given path.
///
/// - The outer result contains IO errors, if any.
/// - The inner option contains parsing errors, if any.
fn load_from_file<T: for<'a> Deserialize<'a>>(
    path: &Path,
) -> crate::Result<Option<T>> {
    let mut file =
        OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(serde_format::from_slice(&buffer).ok())
}

////////////////
// Connection //
////////////////

/// Wraps a type, providing persistence methods.
pub struct Connection<T> {
    location: Option<PathBuf>,
    inner: T,
}

impl<T> Connection<T> {
    pub fn is_virtual(&self) -> bool { self.location.is_none() }

    pub fn close(self) {
        // no-op (for now)
    }
}

impl<T: Serialize> Connection<T> {
    pub fn save(&self) -> crate::Result {
        if let Some(path) = &self.location {
            save_to_file(path, &self.inner)?;
        }
        Ok(())
    }
}

impl<T: Default> Connection<T> {
    pub fn open_in_memory() -> crate::Result<Self, Infallible> {
        let location = None;
        let inner = T::default();
        Ok(Connection { location, inner })
    }
}

impl<T: for<'a> Deserialize<'a> + Default> Connection<T> {
    pub fn open_from_disk<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let location = Some(path.to_path_buf());
        let inner = load_from_file(&path)?.unwrap_or_else(T::default);
        Ok(Connection { location, inner })
    }

    pub fn open<P: AsRef<Path>>(
        path: Option<P>,
    ) -> (Self, Option<crate::Error>) {
        let error = match path {
            Some(path) => Some(match Self::open_from_disk(path) {
                Ok(result) => return (result, None),
                Err(error) => error,
            }),
            None => None,
        };
        let Ok(value) = Self::open_in_memory();
        (value, error)
    }
}

impl<T> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for Connection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
