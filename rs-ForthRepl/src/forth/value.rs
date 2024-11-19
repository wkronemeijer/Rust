use core::f32;
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
    Float(f32),
}

impl Value {
    // as_X? into_X? try_into_X? open for suggestion
    pub fn into_bool(self) -> crate::Result<bool> {
        match self {
            Null => Ok(false),
            Bool(b) => Ok(b),
            Int(i) => Ok(if i != 0 { true } else { false }),
            Float(f) => Ok(!(f.is_nan() || f == 0.0)),
        }
    }

    pub fn into_int(self) -> crate::Result<i32> {
        match self {
            Null => Ok(0),
            Bool(b) => Ok(if b { 1 } else { 0 }),
            Int(i) => Ok(i),
            Float(f) => Ok(f as i32),
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
            (Float(a), Float(b)) => a.total_cmp(b),
            // Inter-kind
            _ => self.kind().cmp(&other.kind()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Null => write!(fmt, "null"),
            Bool(b) => write!(fmt, "{b}"),
            Int(i) => write!(fmt, "{i}"),
            Float(f) => write!(fmt, "{f}"),
        }
    }
}

///////////////
// ValueKind //
///////////////

// This is the same as Value but with the arguments removed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Null,
    Bool,
    Int,
    Float,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
        }
    }
}

impl Value {
    // TODO: Use a macro for this
    pub fn kind(&self) -> ValueKind {
        match self {
            Null => ValueKind::Null,
            Bool(_) => ValueKind::Bool,
            Int(_) => ValueKind::Int,
            Float(_) => ValueKind::Float,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_to_bool() -> crate::Result {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this

        assert_eq!(Null.into_bool()?, false);

        assert_eq!(Bool(false).into_bool()?, false);
        assert_eq!(Bool(true).into_bool()?, true);

        assert_eq!(Int(0).into_bool()?, false);
        assert_eq!(Int(-1).into_bool()?, true);
        assert_eq!(Int(123).into_bool()?, true);
        assert_eq!(Int(4004).into_bool()?, true);

        assert_eq!(Float(0.0).into_bool()?, false);
        assert_eq!(Float(-0.0).into_bool()?, false);
        assert_eq!(Float(f32::NAN).into_bool()?, false);
        assert_eq!(Float(1.0).into_bool()?, true);
        assert_eq!(Float(-0.1).into_bool()?, true);
        assert_eq!(Float(f32::INFINITY).into_bool()?, true);
        assert_eq!(Float(528491.117).into_bool()?, true);

        Ok(())
    }

    #[test]
    fn x_to_int() -> crate::Result {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this
        assert_eq!(Bool(false).into_int()?, 0);
        assert_eq!(Bool(true).into_int()?, 1);
        Ok(())
    }
}
