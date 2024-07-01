use std::fmt::{self, Display, Write};
use std::ops::Deref;

use super::env::Env;
use super::value::Value;

//////////////
// WordName //
//////////////

#[derive(Debug)]
pub struct WordName(String);

impl WordName {
    fn is_valid(_: &str) -> bool {
        true
    }

    pub fn new(name: String) -> crate::Result<WordName> {
        if Self::is_valid(&name) {
            Ok(WordName(name))
        } else {
            Err(crate::Error::InvalidWordName(name))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for WordName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for WordName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///////////
// Token //
///////////

#[derive(Debug)]
pub enum Token {
    PushValue(Value),
    CallWord(WordName), // Late bound!
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::PushValue(value) => write!(f, "{}", value),
            Token::CallWord(name) => write!(f, "{}", name),
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
    pub fn new(body: NativeFunctionBody) -> Self {
        NativeFunction { body }
    }

    pub fn body(&self) -> NativeFunctionBody {
        self.body
    }
}

//////////////////
// UserFunction //
//////////////////

#[derive(Debug)]
pub struct UserFunction {
    tokens: Vec<Token>,
}

impl UserFunction {
    pub fn new() -> Self {
        UserFunction { tokens: Vec::new() }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        let iter = self.tokens.iter();
        iter
    }
}

impl Display for UserFunction {
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
    name: WordName,
    kind: WordKind,
}

impl Word {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &WordKind {
        &self.kind
    }

    pub fn native(name: WordName, body: NativeFunction) -> Word {
        Word {
            name,
            kind: WordKind::Native(body),
        }
    }

    pub fn custom(name: WordName, def: UserFunction) -> Word {
        Word {
            name,
            kind: WordKind::User(def),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref name = self.name;
        match self.kind {
            WordKind::Native(_) => write!(f, ": {} [native code] ;", name)?,
            WordKind::User(ref tokens) => write!(f, ": {} {} ;", name, tokens)?,
        }
        Ok(())
    }
}
