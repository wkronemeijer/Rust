//! Abstracts over storing a file on disk and in memory.

use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

//////////////////
// Cache Format //
//////////////////

/// Recover an instance of the underlying type from a slice of bytes.
fn from_bytes<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> crate::Result<T> {
    Ok(rmp_serde::from_slice(bytes)?)
}

/// Convert an instance of the underlying type into bytes.
fn to_bytes<T: Serialize + ?Sized>(value: &T) -> crate::Result<Vec<u8>> {
    Ok(rmp_serde::to_vec(value)?)
}

/// Saves a [`Serialize`]able value to a file at the given path.
/// Returns the number of bytes written.
fn save_to_file<T: Serialize>(path: &Path, value: &T) -> crate::Result<usize> {
    let contents = to_bytes(value)?;
    let bytes = contents.len();
    fs::write(path, contents)?;
    Ok(bytes)
}

/// Loads a [`Deserialize`]able value from a file at the given path.
/// - The outer result contains IO errors, if any.
/// - The inner option contains parsing errors, if any.
fn load_from_file<T: for<'a> Deserialize<'a>>(
    path: &Path,
) -> crate::Result<Option<T>> {
    let mut file =
        OpenOptions::new().read(true).write(true).create(true).open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(from_bytes(&buffer).ok())
}

////////////////
// Connection //
////////////////

#[derive(Debug)]
/// Backing store of the underlying type.
pub enum ConnectionKind {
    /// Store it in memory.
    Memory,
    /// Store it on disk.
    Disk {
        /// The location of the disk file.
        file: PathBuf,
    },
}

#[derive(Debug)]
/// Wraps a type, providing persistence methods.
pub struct Connection<T> {
    kind: ConnectionKind,
    inner: T,
}

impl<T> Connection<T> {
    /// Terminates the connection.
    pub fn close(self) {
        // no-op (for now)
    }
}

impl<T: Serialize> Connection<T> {
    /// Writes changes to the backing store.
    pub fn save(&self) -> crate::Result {
        if let ConnectionKind::Disk { file } = &self.kind {
            save_to_file(file, &self.inner)?;
        }
        Ok(())
    }
}

impl<T: Default> Connection<T> {
    /// Establishes a connection to a memory-backed store.
    pub fn open_in_memory() -> crate::Result<Self> {
        let inner = T::default();
        let kind = ConnectionKind::Memory;
        Ok(Connection { kind, inner })
    }
}

impl<T: for<'a> Deserialize<'a> + Default> Connection<T> {
    /// Establises a connection to a disk-backed store.
    fn open_from_disk(file: PathBuf) -> crate::Result<Self> {
        let inner = load_from_file(&file)?.unwrap_or_else(T::default);
        let kind = ConnectionKind::Disk { file };
        Ok(Connection { kind, inner })
    }

    /// Establishes a connection of the given kind.
    pub fn open(kind: ConnectionKind) -> crate::Result<Self> {
        match kind {
            ConnectionKind::Disk { file } => Self::open_from_disk(file),
            ConnectionKind::Memory => Self::open_in_memory(),
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
