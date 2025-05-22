//! Items to compute the hash of a single file.

use std::fmt;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use strum::Display;

//////////////
// FileHash //
//////////////

type HashingAlgo = Sha256;
// TODO: Can we use ↑ to derive ↓?
const HASH_BYTE_SIZE: usize = 32;

/// The hashed contents of a file.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize
)]
#[repr(transparent)]
#[serde(transparent)] // TODO: Store it as a hex string, not as an array of bytes
pub struct FileHash {
    #[serde(with = "serde_bytes", rename = "hash")]
    bytes: [u8; HASH_BYTE_SIZE],
}

impl FileHash {
    /// Returns the bytes of this hash.
    pub fn bytes(&self) -> &[u8] { &self.bytes }
}

impl fmt::Display for FileHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.bytes {
            // Prefix with 0 to ensure the entire byte is printed
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

///////////////////
// Multi-hashing //
///////////////////

/// Re-usable file hasher.
pub struct FileHasher {
    hasher: HashingAlgo,
}

impl FileHasher {
    /// Creates a new re-usable file hasher.
    pub fn new() -> Self { FileHasher { hasher: HashingAlgo::new() } }

    /// Creates a hash from the contents of the file at the given path.
    pub fn from_contents(&mut self, path: &Path) -> crate::Result<FileHash> {
        let mut file = File::open(path)?;
        copy(&mut file, &mut self.hasher)?;

        // let now = SystemTime::now();
        let digest = self.hasher.finalize_reset();
        Ok(FileHash { bytes: digest.into() })
    }
}

/////////////////////
// Hash Formatting //
/////////////////////

#[derive(Debug, Default, Clone, Copy, ValueEnum, Display)]
#[clap(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
/// How to display hashes.
pub enum HashStyle {
    #[default]
    /// Truncate long hashes.
    Short,
    /// Display the full hash.
    Full,
}

impl HashStyle {
    /// Uses the style to formnat a hash.
    pub fn format(self, hash: &FileHash) -> String {
        let hash = hash.to_string();
        match self {
            // 8 ASCII chars == 8 bytes
            Self::Short => format!("{}(…)", &hash[0..8]),
            Self::Full => hash,
        }
    }
}
