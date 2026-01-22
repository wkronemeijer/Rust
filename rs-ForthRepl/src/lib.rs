#![allow(
    mismatched_lifetime_syntaxes,
    reason = "can't smell Send based on the name either"
)]

pub mod forth {
    pub mod builtins;
    pub mod dictionary;
    pub mod error;
    pub mod host;
    pub mod stack;
    pub mod state;
    pub mod value;
}
pub mod parsing {
    pub mod error;
    pub mod parser;
    pub mod result;
    pub mod scanner;
    pub mod token;
}

pub use forth::error::Error;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
