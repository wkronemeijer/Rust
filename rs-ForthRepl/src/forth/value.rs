use std::cmp::Ordering;
use std::fmt;

use Value::*;

///////////
// Value //
///////////

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i32),
}

impl Value {
    // as? into? try_into_int? open for suggestion
    pub fn try_into_bool(self) -> crate::Result<bool> {
        match self {
            Null => Ok(false),
            Bool(b) => Ok(b),
            Int(i) => Ok(if i != 0 { true } else { false }),
        }
    }

    // as? into? try_into_int? open for suggestion
    pub fn try_into_int(self) -> crate::Result<i32> {
        match self {
            Null => Ok(0),
            Bool(b) => Ok(if b { 1 } else { 0 }),
            Int(i) => Ok(i),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        // Idea is to compare inner
        // Outer is compared lexicographically
        match (self, other) {
            // Intra-kind
            (Null, Null) => Ordering::Equal,
            (Bool(a), Bool(b)) => a.cmp(b),
            (Int(a), Int(b)) => a.cmp(b),
            // Inter-kind
            (Null | Bool(_), Bool(_) | Int(_)) => Ordering::Less,
            (Int(_) | Bool(_), Bool(_) | Null) => Ordering::Greater,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Null => write!(f, "null"),
            Bool(b) => write!(f, "{}", b),
            Int(i) => write!(f, "{}", i),
        }
    }
}

///////////////
// ValueKind //
///////////////

// This is the same as Value but with the arguments removed
#[derive(Debug, Clone, Copy)]
pub enum ValueKind {
    Null,
    Bool,
    Int,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueKind::Null => write!(f, "null"),
            ValueKind::Bool => write!(f, "bool"),
            ValueKind::Int => write!(f, "int"),
        }
    }
}

impl Value {
    // TODO: Use a macro for this
    pub fn kind(&self) -> ValueKind {
        match self {
            Null => ValueKind::Null,
            Bool(..) => ValueKind::Bool,
            Int(..) => ValueKind::Int,
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
