use std::fmt;

#[derive(Debug)]
pub enum Cst<'s> {
    Null,
    False,
    True,
    Number(&'s str),
    /// &approx; StringLiteral
    Text(&'s str),
    Identifier(&'s str),
    List(Vec<Cst<'s>>),
    Program(Vec<Cst<'s>>),
}

#[derive(Debug)]
pub enum Ast {
    Null,
    False,
    True,
    Number(f64),
    Text(String),
    Identifier(String),
    List(Vec<Ast>),
}

impl<'s> Cst<'s> {
    pub fn format(self) -> String { self.to_string() }

    pub fn lower(self) -> Ast {
        fn map_lower(vec: Vec<Cst<'_>>) -> Vec<Ast> {
            vec.into_iter().map(Cst::lower).collect()
        }

        // TODO: escape and unescape literal strings here
        match self {
            Cst::Null => Ast::Null,
            Cst::False => Ast::False,
            Cst::True => Ast::True,
            Cst::Number(s) => Ast::Number(
                s.parse::<f64>()
                    .unwrap_or_else(|_| panic!("invalid float '{s}'")),
            ),
            // CST use #1: discard &str for Strings
            Cst::Text(s) => Ast::Text(s.to_owned()),
            Cst::Identifier(i) => Ast::Identifier(i.to_owned()),
            // CST use #2: map program and list to the same node
            Cst::List(vec) => Ast::List(map_lower(vec)),
            Cst::Program(vec) => Ast::List(map_lower(vec)),
        }
    }
}

impl<'s> fmt::Display for Cst<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_vec(vec: &Vec<Cst>, f: &mut fmt::Formatter) -> fmt::Result {
            let mut iter = vec.iter();
            if let Some(first) = iter.next() {
                first.fmt(f)?;
                for rest in iter {
                    f.write_str(" ")?;
                    rest.fmt(f)?;
                }
            }
            Ok(())
        }

        match self {
            Cst::Null => f.write_str("null"),
            Cst::False => f.write_str("false"),
            Cst::True => f.write_str("true"),
            Cst::Number(x) => write!(f, "{x}"),
            Cst::Text(s) => write!(f, "\"{s}\""),
            Cst::Identifier(i) => f.write_str(i),
            Cst::List(vec) => {
                write!(f, "[")?;
                fmt_vec(vec, f)?;
                write!(f, "]")
            }
            Cst::Program(vec) => fmt_vec(vec, f),
        }
    }
}
