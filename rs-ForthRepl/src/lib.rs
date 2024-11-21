mod core;
mod forth;

pub use core::CharIndex;

pub use forth::error::Error;
pub use forth::error::Result;
use forth::grammar::scanner::scan;
pub use forth::host::StandardHost;
pub use forth::host::TestHost;
pub use forth::interpreter::Interpreter;
pub use forth::value::Value;
pub use forth::value::ValueKind;

pub fn run_debug(input: &str) -> crate::Result {
    for token in scan(input) {
        println!("{token} \"{}\"", token.lexeme(input));
    }
    Ok(())
}
