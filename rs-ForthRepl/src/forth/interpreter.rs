use super::dictionary::Dictionary;
use super::grammar::ast::Ast;
use super::grammar::parser::parse;
use super::grammar::scanner::scan;
use super::host::Host;
use super::stack::Stack;
use super::value::Value;
use super::value::ValueList;

// TODO: Interpreter is really more of the equivalent of LuaState
// ForthState? JoyState? StateOfJoy?
pub struct Interpreter<'a> {
    pub stack: Stack,
    pub dict: Dictionary,
    // TODO: Use H, replace with fn close<H: Host>(self) -> H
    pub host: &'a mut dyn Host,
}

impl<'a> Interpreter<'a> {
    pub fn new(host: &'a mut dyn Host) -> Self {
        let stack = Stack::new();
        let dict = Dictionary::new();
        let mut interpreter = Interpreter { stack, dict, host };
        interpreter.register_builtins();
        interpreter
    }

    pub fn exec_list(&mut self, list: &ValueList) -> crate::Result {
        for item in list.iter() {
            if let Value::Symbol(s) = item {
                self.dict.get(&s)?.run(self)?;
            } else {
                self.stack.push(item.clone())
            }
        }
        Ok(())
    }

    pub fn exec(&mut self, value: Value) -> crate::Result {
        let Value::List(ref nodes) = value else {
            return Err(crate::Error::ExecuteTypeError(value.kind()));
        };
        self.exec_list(nodes)
    }

    pub fn eval(&mut self, input: &str) -> crate::Result {
        let result = scan(input).and_then(parse);
        for diag in result.report().iter() {
            println!("{diag}");
        }
        if let Some(value) = result.ok().map(Ast::into_value) {
            self.exec(value)
        } else {
            // we have already printed the diagnostics
            Ok(())
        }
    }

    pub fn close(self) -> &'a mut dyn Host { self.host }
}
