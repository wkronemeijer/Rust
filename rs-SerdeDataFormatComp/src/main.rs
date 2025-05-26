use std::fmt::Write as _;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;
use std::time::SystemTime;

use serde::Deserialize;
use serde::Serialize;

//////////
// Core //
//////////

pub type Error = ::anyhow::Error;

pub type Result<T = (), E = Error> = ::core::result::Result<T, E>;

/// [RFC #60](https://rust-lang.github.io/rfcs/0060-rename-strbuf.html) was a mistake
pub type StrBuf = ::std::string::String;

//////////////
// FileHash //
//////////////

pub const HASH_BYTES: usize = 32;

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHash {
    pub time: SystemTime,

    pub method: HashAlgorithm,

    #[serde(with = "serde_bytes", rename = "hash")]
    pub bytes: [u8; HASH_BYTES],
}

//////////
// Main //
//////////

fn ascii_symbol(byte: u8) -> Option<char> {
    let c = char::from_u32(byte as u32)?;

    Some(match c {
        // '\x09' => '\u{2B7E}', // Horizontal Tab
        '\x0A' => '\u{23CE}', // Line Feed
        // '\x1B' => '\u{238B}', // Escape
        '\x20'..='\x7E' => c, // Space and ASCII Graphics
        '\x7F' => '\u{232B}', // Backspace
        _ => return None,
    })
}

fn dump_line<const C: usize>(f: &mut StrBuf, bytes: &[u8]) {
    let mut chars = ['.'; C];

    for i in 0..C {
        if let Some(byte) = bytes.get(i) {
            write!(f, "{:02X}", byte).unwrap();
            if let Some(char) = ascii_symbol(*byte) {
                chars[i] = char
            }
        } else {
            f.push(' ');
            f.push(' ');
            chars[i] = ' ';
        }
        f.push(' ');
    }

    f.extend(chars);
    f.push('\n');
}

fn hexdump<const C: usize>(bytes: &[u8]) -> StrBuf {
    let mut result = StrBuf::new();
    for byte in bytes.chunks(C) {
        dump_line::<C>(&mut result, byte);
    }
    result
}

fn psei(name: &str, bytes: &[u8]) {
    eprintln!("\x1B[1m{}\x1B[22m ({} bytes)", name, bytes.len());
    eprintln!("{}", hexdump::<24>(bytes));
}

fn main() -> crate::Result {
    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    let [a, b, c, d, e, f, g, h] = hasher.finish().to_be_bytes();

    let value = FileHash {
        time: SystemTime::now(),
        method: HashAlgorithm::Sha256,
        bytes: [
            a, b, c, d, e, f, g, h, a, b, c, d, e, f, g, h, 0, 1, 2, 3, 4, 5,
            6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ],
    };

    eprintln!("Value to be serialized: {:#?}", value);

    psei("JSON", serde_json::to_string(&value)?.as_bytes());
    psei("JSON (Pretty)", serde_json::to_string_pretty(&value)?.as_bytes());
    psei("TOML", toml::to_string(&value)?.as_bytes());
    psei("TOML (Pretty)", toml::to_string_pretty(&value)?.as_bytes());
    psei("MessagePack", rmp_serde::to_vec(&value)?.as_slice());
    psei("MessagePack (Named)", rmp_serde::to_vec_named(&value)?.as_slice());
    psei("PostCard", postcard::to_stdvec(&value)?.as_slice());

    Ok(())
}
