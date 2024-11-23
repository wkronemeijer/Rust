use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use super::grammar::forestry::Ast;
use crate::Error;
use crate::Value::*;

///////////
// Value //
///////////
// TODO: Maybe extract the list as some general thing?

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Char(char),
    Number(f64),
    Text(Rc<String>),
    Symbol(Rc<String>),
    List(Rc<Vec<Value>>),
}

impl Value {
    pub fn from_ast(ast: Ast) -> Value {
        match ast {
            Ast::Null => Value::Null,
            Ast::False => Value::Bool(false),
            Ast::True => Value::Bool(true),
            Ast::Number(x) => Value::Number(x),
            Ast::Text(s) => Value::Text(Rc::new(s)),
            Ast::Identifier(i) => Value::Symbol(Rc::new(i)),
            Ast::List(l) => Value::List(Rc::new(
                l.into_iter().map(Value::from_ast).collect(),
            )),
        }
    }

    // as_X? into_X? try_into_X? open for suggestion
    pub fn into_bool(self) -> crate::Result<bool> {
        Ok(match self {
            Null => false,
            Bool(b) => b,
            Char(c) => c != '\0',
            Number(x) => !(x.is_nan() || x == 0.0),
            _ => true,
        })
    }

    pub fn into_int(self) -> crate::Result<i32> {
        todo!("meant for bitwise operators (like JS)");
    }

    pub fn into_number(self) -> crate::Result<f64> {
        Ok(match self {
            Null => 0.0,
            Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
            Number(x) => x,
            _ => {
                return Err(Error::TypeConversion {
                    from: self.kind(),
                    to: ValueKind::Number,
                })
            }
        })
    }

    pub fn into_string(self) -> crate::Result<String> {
        Ok(match self {
            Text(rc) => Rc::unwrap_or_clone(rc),
            _ => self.to_string(),
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
            (Number(a), Number(b)) => a.total_cmp(b),
            (Text(a), Text(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            // Inter-kind
            _ => self.kind().cmp(&other.kind()),
        }
    }
}

// TODO: Use Debug/Display in the right places
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Null => write!(f, "null"),
            Bool(b) => write!(f, "{b}"),
            Char(c) => write!(f, "{c}"),
            Number(x) => write!(f, "{x}"),
            Text(t) => write!(f, "\"{t}\""),
            Symbol(s) => write!(f, "'{s}"),
            List(l) => {
                write!(f, "[")?;
                let mut iter = l.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{first}")?;
                    for rest in iter {
                        write!(f, " {rest}")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
        }
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
    Number,
    Text,
    Symbol,
    List,
}

impl Value {
    // TODO: Use a macro for this?
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Null => ValueKind::Null,
            Value::Bool(_) => ValueKind::Bool,
            Value::Char(_) => ValueKind::Char,
            Value::Number(_) => ValueKind::Number,
            Value::Text(_) => ValueKind::Text,
            Value::Symbol(_) => ValueKind::Symbol,
            Value::List(_) => ValueKind::List,
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool => write!(f, "boolean"),
            Self::Char => write!(f, "character"),
            Self::Number => write!(f, "number"),
            Self::Text => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
            Self::List => write!(f, "list"),
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

        assert_eq!(Number(0.0).into_bool()?, false);
        assert_eq!(Number(-0.0).into_bool()?, false);
        assert_eq!(Number(f64::NAN).into_bool()?, false);
        assert_eq!(Number(1.0).into_bool()?, true);
        assert_eq!(Number(-0.1).into_bool()?, true);
        assert_eq!(Number(f64::INFINITY).into_bool()?, true);
        assert_eq!(Number(528491.117).into_bool()?, true);

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
