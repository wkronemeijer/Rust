use std::fmt::Display;

use super::error::Result;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn internal() {
        assert_eq!(Value::Bool(true).try_into_int(), Ok(1));
    }
}
