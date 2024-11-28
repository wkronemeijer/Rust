use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as _;

use super::interpreter::Interpreter;
use super::value::ValueList;

//////////
// Word //
//////////

// Still find it weird that fn(...) types aren't !Sized
pub type NativeFn = fn(&mut Interpreter) -> crate::Result;
pub type UserFn = ValueList;

#[derive(Debug, Clone)]
pub enum Word {
    Native(NativeFn),
    User(UserFn),
}

impl Word {
    pub fn run(&self, ip: &mut Interpreter) -> crate::Result {
        match self {
            Word::Native(func) => func(ip),
            Word::User(list) => ip.exec_list(list),
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Word::Native(_) => f.write_str("<native code>"),
            Word::User(list) => list.fmt(f),
        }
    }
}

////////////////
// Dictionary //
////////////////

pub type WordName = Cow<'static, str>;

pub struct Dictionary {
    word_by_name: HashMap<WordName, Word>,
}

impl Dictionary {
    pub fn new() -> Self { Dictionary { word_by_name: HashMap::new() } }

    pub fn define(&mut self, name: WordName, word: Word) -> crate::Result {
        use std::collections::hash_map::Entry::*;
        match self.word_by_name.entry(name) {
            Occupied(slot) => Err(crate::Error::NameAlreadyInUse(
                slot.key().to_string().into(),
            )),
            Vacant(slot) => Ok({
                slot.insert(word);
            }),
        }
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
        let mut keys = Vec::from_iter(self.word_by_name.keys());
        keys.sort();
        let max_len = keys.iter().map(|k| k.chars().count()).max().unwrap_or(0);

        let fmt_entry = |n: &WordName, f: &mut fmt::Formatter| -> fmt::Result {
            let key = n.to_string();
            let value = self.word_by_name.get(n).unwrap();

            let key_len = key.chars().count();
            let padding_len = (max_len - key_len).max(0);
            let padding = " ".repeat(padding_len);

            key.fmt(f)?;
            padding.fmt(f)?;
            " ".fmt(f)?;
            value.fmt(f)
        };

        let mut iter = keys.into_iter();
        if let Some(first) = iter.next() {
            fmt_entry(first, f)?;
            for rest in iter {
                f.write_char('\n')?;
                fmt_entry(rest, f)?;
            }
        }
        Ok(())
    }
}
