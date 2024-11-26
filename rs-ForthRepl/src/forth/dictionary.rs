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

////////////////
// Dictionary //
////////////////

pub struct Dictionary {
    word_by_name: HashMap<Cow<'static, str>, Word>,
}

impl Dictionary {
    pub fn new() -> Self { Dictionary { word_by_name: HashMap::new() } }

    pub fn define(
        &mut self,
        name: Cow<'static, str>,
        word: Word,
    ) -> crate::Result {
        use std::collections::hash_map::Entry::*;
        match self.word_by_name.entry(name) {
            Occupied(slot) => {
                return Err(crate::Error::NameAlreadyInUse(
                    slot.key().to_string().into(),
                ))
            }
            Vacant(slot) => slot.insert(word),
        };
        Ok(())
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
