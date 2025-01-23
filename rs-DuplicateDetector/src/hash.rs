use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use sha2::Digest;
use sha2::Sha256;

//////////////
// FileHash //
//////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileHash {
    // TODO: Should we derive this 32 from Sha256 type (somehow)?
    bytes: [u8; 32],
}

impl FileHash {
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

///////////////
// Hash File //
///////////////

impl FileHash {
    /// Computes the file hash of a file at the given path.
    ///
    /// Based on https://stackoverflow.com/a/71606608
    pub fn from_contents(path: &Path) -> crate::Result<FileHash> {
        const BUF_SIZE: usize = 1 << 12;
        const CHUNK_SIZE: usize = 1 << 10;

        let file = File::open(path)?;
        let mut reader = BufReader::with_capacity(BUF_SIZE, file);
        let mut hasher = Sha256::new();

        let mut buffer = [0; CHUNK_SIZE];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break
            }
            hasher.update(&buffer[..bytes_read]);
        }
        let digest = hasher.finalize();
        let bytes = digest.into();
        Ok(FileHash { bytes })
    }
}
