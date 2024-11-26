use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use super::value::Value::*;

///////////
// Value //
///////////

#[derive(Debug, Clone, Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Char(char),
    Int(i32),
    Float(f64),
    Symbol(Rc<String>),
    Text(Rc<String>),
    List(ValueList),
}

// Idea: use some traits and associated types to make these conversion simpler
impl Value {
    fn type_err(&self, goal: ValueKind) -> crate::Error {
        crate::Error::TypeConversion { from: self.kind(), to: goal }
    }

    // as_X? into_X? try_into_X? open for suggestion
    pub fn into_bool(self) -> crate::Result<bool> {
        Ok(match self {
            Null => false,
            Bool(b) => b,
            Char(c) => c != '\0',
            Int(i) => i != 0,
            Float(x) => !(x.is_nan() || x == 0.0),
            Symbol(_) => true,
            Text(_) => true,
            List(_) => true,
        })
    }

    // as_X? into_X? try_into_X? open for suggestion
    pub fn into_char(self) -> crate::Result<char> {
        Ok(match self {
            Null => '\0',
            Char(c) => c,
            Int(i) => char::try_from(i as u32)
                .map_err(|_| crate::Error::IntegerRange)?,
            Float(x) => char::try_from(x as u32)
                .map_err(|_| crate::Error::IntegerRange)?,
            _ => return Err(self.type_err(ValueKind::Char)),
        })
    }

    pub fn into_int(self) -> crate::Result<i32> {
        Ok(match self {
            Null => 0,
            Bool(b) => {
                if b {
                    1
                } else {
                    0
                }
            }
            Int(i) => i,
            Char(c) => u32::from(c) as i32, // NB: char::MAX <= i32::MAX
            Float(x) => x.floor() as i32,
            _ => return Err(self.type_err(ValueKind::Float)),
        })
    }

    pub fn into_float(self) -> crate::Result<f64> {
        Ok(match self {
            Null => 0.0,
            Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
            Float(x) => x,
            _ => Err(self.type_err(ValueKind::Float))?,
        })
    }

    pub fn into_string(self) -> crate::Result<String> {
        Ok(match self {
            Text(rc) => Rc::unwrap_or_clone(rc),
            _ => self.to_string(),
        })
    }

    pub fn into_list(self) -> crate::Result<Vec<Value>> {
        Ok(match self {
            List(nodes) => nodes.into_list(),
            // TODO: string -> list
            _ => return Err(self.type_err(ValueKind::List)),
        })
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
            (Char(a), Char(b)) => a.cmp(b),
            (Float(a), Float(b)) => a.total_cmp(b),
            (Text(a), Text(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            // Inter-kind
            _ => {
                let self_kind = self.kind();
                let other_kind = other.kind();
                debug_assert!(
                    self_kind != other_kind,
                    "missing a case for '{self_kind}'"
                );
                self_kind.cmp(&other_kind)
            }
        }
    }
}

// TODO: Use Debug/Display in the right places
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Null => f.write_str("null"),
            Bool(b) => b.fmt(f),
            Char(c) => write!(f, "'{c}'"),
            Int(i) => i.fmt(f),
            Float(x) => x.fmt(f),
            Text(t) => write!(f, "\"{t}\""),
            Symbol(s) => s.fmt(f),
            List(l) => {
                f.write_str("[")?;
                let mut iter = l.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;
                    for rest in iter {
                        write!(f, " {rest}")?;
                    }
                }
                f.write_str("]")?;
                Ok(())
            }
        }
    }
}

///////////////
// ValueList //
///////////////

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueList {
    values: Rc<Vec<Value>>,
}

impl ValueList {
    pub fn new() -> Self { ValueList { values: Rc::new(Vec::new()) } }

    pub fn iter(&self) -> impl Iterator<Item = &Value> { self.values.iter() }

    pub fn into_list(self) -> Vec<Value> { Rc::unwrap_or_clone(self.values) }
}

impl FromIterator<Value> for ValueList {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        ValueList { values: Rc::new(Vec::from_iter(iter)) }
    }
}

///////////////
// ValueKind //
///////////////

// This is the same as Value but with the arguments removed
// TODO: Can you use a macro for this?

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Null,
    Bool,
    Char,
    Int,
    Float,
    Symbol,
    Text,
    List,
}

impl Value {
    // TODO: Use a macro for this?
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Null => ValueKind::Null,
            Value::Bool(_) => ValueKind::Bool,
            Value::Char(_) => ValueKind::Char,
            Value::Int(_) => ValueKind::Int,
            Value::Float(_) => ValueKind::Float,
            Value::Symbol(_) => ValueKind::Symbol,
            Value::Text(_) => ValueKind::Text,
            Value::List(_) => ValueKind::List,
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool => f.write_str("bool"),
            Self::Char => f.write_str("char"),
            Self::Int => f.write_str("int"),
            Self::Float => f.write_str("float"),
            Self::Text => f.write_str("string"),
            Self::Symbol => f.write_str("symbol"),
            Self::List => f.write_str("list"),
        }
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_to_bool() -> crate::Result {
        // TODO: Waiting on assert_matches or PartialEq for io::Result to fix this

        assert_eq!(Null.into_bool()?, false);

        assert_eq!(Bool(false).into_bool()?, false);
        assert_eq!(Bool(true).into_bool()?, true);

        assert_eq!(Float(0.0).into_bool()?, false);
        assert_eq!(Float(-0.0).into_bool()?, false);
        assert_eq!(Float(f64::NAN).into_bool()?, false);
        assert_eq!(Float(1.0).into_bool()?, true);
        assert_eq!(Float(-0.1).into_bool()?, true);
        assert_eq!(Float(f64::INFINITY).into_bool()?, true);
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
