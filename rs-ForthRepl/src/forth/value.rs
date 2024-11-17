use std::fmt;
use std::fmt::Display;

use Value::*;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i32),
}

impl Value {
    // as? into? try_into_int? open for suggestion
    pub fn try_into_int(self) -> crate::Result<i32> {
        match self {
            Bool(b) => Ok(if b { 1 } else { 0 }),
            Int(i) => Ok(i),
        }
    }

    // as? into? try_into_int? open for suggestion
    pub fn try_into_bool(self) -> crate::Result<bool> {
        match self {
            Bool(b) => Ok(b),
            Int(i) => Ok(if i != 0 { true } else { false }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bool(b) => write!(f, "{}", b),
            Int(i) => write!(f, "{}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_to_int() -> crate::Result {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this
        assert_eq!(Bool(false).try_into_int()?, 0);
        assert_eq!(Bool(true).try_into_int()?, 1);
        Ok(())
    }

    #[test]
    fn int_to_bool() -> crate::Result {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this
        assert_eq!(Int(123).try_into_bool()?, true);
        assert_eq!(Int(0).try_into_bool()?, false);
        assert_eq!(Int(-1).try_into_bool()?, true);
        assert_eq!(Int(4004).try_into_bool()?, true);
        Ok(())
    }
}
