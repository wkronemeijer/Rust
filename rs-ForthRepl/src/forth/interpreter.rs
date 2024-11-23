use super::dictionary::Dictionary;
use super::dictionary::Word;
use super::grammar::forestry::Cst;
use super::grammar::scanner::scan;
use super::host::Host;
use super::stack::Stack;
use crate::forth::grammar::parser::parse;
use crate::Value;

// TODO: Interpreter is really more of the equivalent of LuaState
// ForthState?
pub struct Interpreter<'a> {
    pub stack: Stack,
    pub words: Dictionary,
    // TODO: Use H, replace with fn close<H: Host>(self) -> H
    pub host: &'a mut dyn Host,
}

impl<'a> Interpreter<'a> {
    pub fn new(host: &'a mut dyn Host) -> Self {
        let stack = Stack::new();
        let words = Dictionary::new();
        let mut interpreter = Interpreter { stack, words, host };
        interpreter.register_builtins();
        interpreter
    }

    fn execute(&mut self, value: Value) -> crate::Result {
        match value {
            Value::List(ref nodes) => {
                for node in nodes.iter().cloned() {
                    match node {
                        Value::Symbol(s) => {
                            let Word::Native(func) = self.words.get(&s)?;
                            func(self)?;
                        }
                        _ => self.stack.push(node.clone()),
                    }
                }
                Ok(())
            }
            _ => Err(crate::Error::ExecuteTypeError(value.kind())),
        }
    }

    pub fn eval(&mut self, input: &str) -> crate::Result {
        let result = scan(input).and_then(parse).map(Cst::lower);
        for diag in result.report().iter() {
            println!("{diag}");
        }
        if let Some(value) = result.ok().map(Value::from_ast) {
            self.execute(value)
        } else {
            // we have already printed the diagnostics
            Ok(())
        }
    }
}
