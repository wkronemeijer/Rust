use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;

//////////////////
// Cache Format //
//////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum, Display)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum CacheFormat {
    #[default]
    MessagePack,
    Json,
    PrettyJson,
}

impl CacheFormat {
    // Foreshadowing...
    pub fn default_file_name(self) -> &'static Path {
        Path::new(match self {
            Self::MessagePack => "hash-cache.dat",
            Self::Json => "hash-cache.json",
            Self::PrettyJson => "hash-cache-pretty.json",
        })
    }
}

// Per-backend serialize and deserialize
impl CacheFormat {
    pub fn from_slice<'a, T: Deserialize<'a>>(
        self,
        source: &'a [u8],
    ) -> crate::Result<T> {
        Ok(match self {
            Self::MessagePack => rmp_serde::from_slice(source)?,
            Self::Json => serde_json::from_slice(source)?,
            Self::PrettyJson => serde_json::from_slice(source)?,
        })
    }

    pub fn to_vec<T: Serialize + ?Sized>(
        self,
        value: &T,
    ) -> crate::Result<Vec<u8>> {
        Ok(match self {
            Self::MessagePack => rmp_serde::to_vec(value)?,
            Self::Json => serde_json::to_vec(value)?,
            Self::PrettyJson => serde_json::to_vec_pretty(value)?,
        })
    }
}

// Generic save and load
impl CacheFormat {
    /// Saves a [`Serialize`]able value to a file at the given path.
    /// Returns the number of bytes written.
    pub fn save_to_file<T: Serialize>(
        self,
        path: &Path,
        value: &T,
    ) -> crate::Result<usize> {
        let contents = self.to_vec(value)?;
        let bytes = contents.len();
        fs::write(path, contents)?;
        Ok(bytes)
    }

    /// Loads a [`Deserialize`]able value from a file at the given path.
    ///
    /// - The outer result contains IO errors, if any.
    /// - The inner option contains parsing errors, if any.
    pub fn load_from_file<T: for<'a> Deserialize<'a>>(
        self,
        path: &Path,
    ) -> crate::Result<Option<T>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(self.from_slice(&buffer).ok())
    }
}

////////////////
// Connection //
////////////////

#[derive(Debug)]
enum ConnectionKind {
    Memory,
    Disk { file: PathBuf, format: CacheFormat },
}

/// Wraps a type, providing persistence methods.
#[derive(Debug)]
pub struct Connection<T> {
    kind: ConnectionKind,
    inner: T,
}

impl<T> Connection<T> {
    pub fn close(self) {
        // no-op (for now)
    }
}

impl<T: Serialize> Connection<T> {
    pub fn save(&self) -> crate::Result {
        if let ConnectionKind::Disk { format, file } = &self.kind {
            format.save_to_file(file, &self.inner)?;
        }
        Ok(())
    }
}

impl<T: Default> Connection<T> {
    pub fn open_in_memory() -> crate::Result<Self> {
        let kind = ConnectionKind::Memory;
        let inner = T::default();
        Ok(Connection { kind, inner })
    }
}

impl<T: for<'a> Deserialize<'a> + Default> Connection<T> {
    pub fn open_from_disk<P: AsRef<Path>>(
        path: P,
        format: CacheFormat,
    ) -> crate::Result<Self> {
        let path = path.as_ref();
        let kind = ConnectionKind::Disk { file: path.to_path_buf(), format };
        let inner = format.load_from_file(&path)?.unwrap_or_else(T::default);
        Ok(Connection { kind, inner })
    }

    pub fn open<P: AsRef<Path>>(
        path: Option<P>,
        format: CacheFormat,
    ) -> crate::Result<Self> {
        match path {
            Some(path) => Self::open_from_disk(path, format),
            None => Self::open_in_memory(),
        }
    }
}

impl<T> Deref for Connection<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for Connection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
