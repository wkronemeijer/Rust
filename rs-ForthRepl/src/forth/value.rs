use std::fmt;
use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Bool(bool),
    Int(i32),
}

impl Value {
    // as? into? try_into_int? open for suggestion
    pub fn try_into_int(self) -> Result<i32> {
        match self {
            Value::Bool(b) => Ok(if b { 1 } else { 0 }),
            Value::Int(i) => Ok(i),
        }
    }

    // as? into? try_into_int? open for suggestion
    pub fn try_into_bool(self) -> Result<bool> {
        match self {
            Value::Bool(b) => Ok(b),
            Value::Int(i) => Ok(if i != 0 { true } else { false }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_int() -> Result<()> {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this
        assert_eq!(Value::Bool(false).try_into_int()?, 0);
        assert_eq!(Value::Bool(true).try_into_int()?, 1);
        Ok(())
    }

    #[test]
    fn int_to_bool() -> Result<()> {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this
        assert_eq!(Value::Int(123).try_into_bool()?, true);
        assert_eq!(Value::Int(0).try_into_bool()?, false);
        assert_eq!(Value::Int(-1).try_into_bool()?, true);
        assert_eq!(Value::Int(4004).try_into_bool()?, true);
        Ok(())
    }
}
