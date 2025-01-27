use std::convert::Infallible;
use std::fs;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

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
    pub fn save(&mut self) -> crate::Result {
        if let Some(path) = &self.location {
            let contents = serde_json::to_string(&self.inner)?;
            fs::write(path, contents)?;
        };
        Ok(())
    }
}

impl<T: for<'a> Deserialize<'a> + Default> Connection<T> {
    fn load<P: AsRef<Path>>(path: P) -> crate::Result<T> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn open_from_disk<P: Into<PathBuf>>(path: P) -> crate::Result<Self> {
        let location = path.into();
        let inner = match Self::load(&location) {
            Ok(inner) => inner,
            Err(e) => {
                eprintln!("load error: {}", e);
                T::default()
            },
        };
        Ok(Connection { location: Some(location), inner })
    }

    pub fn open_in_memory() -> crate::Result<Self, Infallible> {
        let location = None;
        let inner = T::default();
        Ok(Connection { location, inner })
    }

    pub fn open<P: Into<PathBuf>>(path: Option<P>) -> crate::Result<Self> {
        Ok(match path {
            Some(path) => Self::open_from_disk(path)?,
            None => Self::open_in_memory()?,
        })
    }
}

impl<T> Deref for Connection<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for Connection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
