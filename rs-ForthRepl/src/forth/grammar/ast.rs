use crate::forth::value::Value;

// no CST this time
// formatting Forth/Joy code can be done but it does extremely little for clarity
// and/or we can just say it is out of scope (LOL)

#[derive(Debug)]
pub enum Ast {
    Null,
    False,
    True,
    Number(f64),
    StringLiteral(String),
    Identifier(String),
    List(Vec<Ast>),
}

// Not sure where to put this function
// Homoiconicity baby

impl Ast {
    pub fn into_value(self) -> Value {
        match self {
            Ast::Null => Value::Null,
            Ast::False => Value::Bool(false),
            Ast::True => Value::Bool(true),
            Ast::Number(x) => Value::Number(x),
            Ast::StringLiteral(s) => Value::Text(Box::new(s)),
            Ast::Identifier(i) => Value::Symbol(Box::new(i)),
            Ast::List(l) => {
                Value::List(l.into_iter().map(Ast::into_value).collect())
            }
        }
    }
}
