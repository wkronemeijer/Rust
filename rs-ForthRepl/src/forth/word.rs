use std::fmt;
use std::fmt::Write;

use super::env::Env;
use super::value::Value;

///////////
// Token //
///////////

#[derive(Debug)]
pub enum Token {
    PushValue(Value),
    CallWord(String), // Late bound!
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::PushValue(value) => write!(f, "{value}"),
            Token::CallWord(name) => write!(f, "{name}"),
        }
    }
}

/////////////////////
// Native Function //
/////////////////////

pub type NativeFunctionBody = fn(&mut Env) -> crate::Result;

#[derive(Debug)]
pub struct NativeFunction {
    body: NativeFunctionBody,
}

impl NativeFunction {
    pub fn new(body: NativeFunctionBody) -> Self { NativeFunction { body } }

    pub fn body(&self) -> NativeFunctionBody { self.body }
}

//////////////////
// UserFunction //
//////////////////

#[derive(Debug)]
pub struct UserFunction {
    tokens: Vec<Token>,
}

impl UserFunction {
    pub fn new() -> Self { UserFunction { tokens: Vec::new() } }

    pub fn push(&mut self, token: Token) { self.tokens.push(token); }

    pub fn iter(&self) -> impl Iterator<Item = &Token> { self.tokens.iter() }
}

impl fmt::Display for UserFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = &mut self.tokens.iter();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for rest in iter {
                f.write_char(' ')?;
                rest.fmt(f)?;
            }
        }
        Ok(())
    }
}

//////////
// Word //
//////////

#[derive(Debug)]
pub enum WordKind {
    Native(NativeFunction),
    User(UserFunction),
}

#[derive(Debug)]
pub struct Word {
    name: String,
    kind: WordKind,
}

impl Word {
    pub fn name(&self) -> &str { &self.name }

    pub fn kind(&self) -> &WordKind { &self.kind }

    pub fn native(name: String, body: NativeFunction) -> Self {
        Self { name, kind: WordKind::Native(body) }
    }

    pub fn custom(name: String, def: UserFunction) -> Self {
        Self { name, kind: WordKind::User(def) }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref name = self.name;
        match self.kind {
            WordKind::Native(_) => write!(f, ": {name} [native code] ;")?,
            WordKind::User(ref tokens) => write!(f, ": {name} {tokens} ;")?,
        }
        Ok(())
    }
}
