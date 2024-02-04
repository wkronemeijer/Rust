use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

use super::{env::Env, error::Result, value::Value};

pub enum Token {
    PushValue(Value),
    CallWord(String), // Late bound!
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::PushValue(value) => write!(f, "{}", value),
            Token::CallWord(name) => write!(f, "{}", name),
        }
    }
}

pub type NativeFunction = fn(&mut Env) -> Result<()>;

pub struct UserFunction(pub Vec<Token>);

impl Deref for UserFunction {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|tok| {
            tok.fmt(f)?;
            f.write_char(' ')?;
            Ok(())
        })
    }
}

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
            Word::Native(name, _) => write!(f, ": {} <native code> ;", name)?,
            Word::User(name, tokens) => write!(f, ": {} {} ;", name, tokens)?,
        }
        Ok(())
    }
}
