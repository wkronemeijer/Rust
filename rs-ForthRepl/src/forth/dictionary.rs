use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as _;

use crate::forth::interpreter::Interpreter;

pub type NativeFn = fn(&mut Interpreter) -> crate::Result;
// Still find it weird that fn types aren't !Sized
// I mean, it's some region of code, right?
// Function pointers would then just be &'static fn(...)
// Equally usable
// Also opens the door for JIT stuff
// TODO: Reintroduce Word with ValueList variant

#[derive(Debug, Clone)]
pub enum Word {
    Native(NativeFn),
}

impl Word {
    pub fn run(&self, ip: &mut Interpreter) -> crate::Result {
        match self {
            Word::Native(func) => func(ip),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    word_by_name: HashMap<Cow<'static, str>, Word>,
}

impl Dictionary {
    pub fn new() -> Self { Dictionary { word_by_name: HashMap::new() } }

    pub fn define(&mut self, name: Cow<'static, str>, word: Word) {
        debug_assert!(
            !self.word_by_name.contains_key(&name),
            "'{name}' is defined more than once"
        );
        self.word_by_name.insert(name, word);
    }

    pub fn has(&self, name: &str) -> bool {
        self.word_by_name.contains_key(name)
    }

    pub fn get(&self, name: &str) -> crate::Result<Word> {
        self.word_by_name.get(name).cloned().ok_or_else(|| {
            crate::Error::UnknownWord(format!("'{name}'").into())
        })
    }
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = &mut self.word_by_name.keys();
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for rest in iter {
                f.write_char('\n')?;
                rest.fmt(f)?;
            }
        }
        Ok(())
    }
}
