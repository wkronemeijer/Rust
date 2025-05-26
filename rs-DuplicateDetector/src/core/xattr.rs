//! Provides cross platform support for eXtended Attributes
//!
//! Note that on Windows, EA have very poor documentation;
//! NTFS ADS is used instead.

use std::ffi::OsStr;
use std::io;
use std::path::Path;

#[cfg(not(windows))]
compile_error!("this program requires NTFS ADS to store metadata");

// Note: Ideally these use Reader/Writer,
// but that requires more Rust-fu than I have

// Listing xattrs is also not really nice
// Most xattr libraries don't have a fallback for windows
// Will probably just be a shim for a proper xattr library...eventually.

/// Retrieves an extended attribute from a file.
pub fn get_xattr(
    path: impl AsRef<Path>,
    name: impl AsRef<OsStr>,
) -> io::Result<Vec<u8>> {
    fn inner(_path: &Path, _name: &OsStr) -> io::Result<Vec<u8>> { todo!() }
    inner(path.as_ref(), name.as_ref())
}

/// Sets an extended attribute on a file.
pub fn set_xattr(
    path: impl AsRef<Path>,
    name: impl AsRef<OsStr>,
    value: &[u8],
) -> io::Result<()> {
    fn inner(_path: &Path, _name: &OsStr, _value: &[u8]) -> io::Result<()> {
        todo!()
    }
    inner(path.as_ref(), name.as_ref(), value)
}

/// Removes an extended attribute from a file.
pub fn delete_xattr(
    path: impl AsRef<Path>,
    name: impl AsRef<OsStr>,
) -> io::Result<()> {
    fn inner(_path: &Path, _name: &OsStr) -> io::Result<()> { todo!() }
    inner(path.as_ref(), name.as_ref())
}
