use std::fmt::Display;
use std::fmt::Write;

use super::error::Error;
use super::error::Result;
use super::value::Value;

pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { values: Vec::new() }
    }

    pub fn depth(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.values.pop().ok_or(Error::StackUnderflow)
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = &mut self.values.iter();
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
