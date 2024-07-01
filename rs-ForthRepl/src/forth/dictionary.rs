use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Write as _};

use super::word::{NativeFunction, Word};
use crate::prelude::*;

pub struct Dictionary {
    words: HashMap<String, Word>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary { words: HashMap::new() }
    }

    pub fn define(&mut self, word: Word) -> Result<()> {
        let name = word.name().to_owned();
        if !self.words.contains_key(&name) {
            // Some version of "provide if doesnt exist"
            self.words.insert(name, word);
            Ok(())
        } else {
            Err(Error::NameAlreadyInUse(name))
        }
    }

    pub fn define_native(&mut self, name: &'static str, body: NativeFunction) -> Result<()> {
        self.define(Word::native(name.to_string(), body))
    }

    pub fn get(&self, name: &str) -> Result<&Word> {
        self.words.get(name).ok_or_else(|| Error::UnknownWord(name.to_owned()))
    }

    pub fn has(&self, name: &str) -> bool {
        self.words.contains_key(name)
    }
}

impl Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = &mut self.words.values();
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
