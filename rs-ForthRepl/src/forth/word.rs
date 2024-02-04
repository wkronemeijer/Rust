use std::fmt::Display;

use super::{env::Env, error::Result, value::Value};

pub enum Token {
    PushValue(Value),
    CallWord(String), // Late bound!
}

pub type NativeFunction = fn(&mut Env) -> Result<()>;
pub type UserFunction = Vec<Token>;

pub enum Word {
    Native(String, NativeFunction),
    User(String, UserFunction),
}

impl Word {
    pub fn name(&self) -> &str {
        match self {
            Word::Native(name, _) => name,
            Word::User(name, _) => name,
        }
    }

    pub fn native(name: String, body: NativeFunction) -> Word {
        Word::Native(name, body)
    }

    pub fn custom(name: String, def: UserFunction) -> Word {
        Word::User(name, def)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Native(name, _) => write!(f, "native word '{}'", name)?,
            Word::User(name, tokens) => write!(f, "user word '{}' ({} long)", name, tokens.len())?,
        }
        Ok(())
    }
}
