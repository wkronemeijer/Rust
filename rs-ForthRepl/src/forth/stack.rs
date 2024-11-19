use std::fmt;
use std::fmt::Display;
use std::fmt::Write;

use super::value::Value;

pub struct Stack {
    list: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self { Stack { list: Vec::new() } }

    pub fn depth(&self) -> usize { self.list.len() }

    pub fn peek(&self) -> Option<&Value> { self.list.last() }

    pub fn push(&mut self, value: Value) { self.list.push(value) }

    pub fn pop(&mut self) -> crate::Result<Value> {
        self.list.pop().ok_or(crate::Error::StackUnderflow)
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = &mut self.list.iter();
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
