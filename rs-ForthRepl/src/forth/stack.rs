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

    /// Pops multiple values in "visual" order to make implementing native
    /// functions easier.
    ///
    /// If the stack loops like  
    /// 1 2 3 4  
    /// Then parallel_pop::<3>() will return  
    /// \[2, 3, 4\]
    pub fn parallel_pop<const N: usize>(
        &mut self,
    ) -> crate::Result<[Value; N]> {
        let mut array = [const { Value::Null }; N];
        let mut failed_to_pop = false;
        for i in (0..N).rev() {
            // this /should/ keep popping even when encountering an error
            match self.list.pop() {
                Some(value) => array[i] = value,
                None => failed_to_pop = true,
            }
        }
        if !failed_to_pop {
            Ok(array)
        } else {
            Err(crate::Error::StackUnderflow)
        }
    }
}

impl fmt::Display for Stack {
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
