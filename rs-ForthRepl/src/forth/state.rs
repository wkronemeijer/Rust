use std::marker::PhantomData;

use super::builtins::register_builtins;
use super::dictionary::Dictionary;
use super::host::Host;
use super::stack::Stack;
use super::value::Value;
use super::value::ValueList;
use crate::parsing::parser::parse;
use crate::parsing::scanner::scan;

// TODO: Interpreter is really more of the equivalent of LuaState
// ForthState? JoyState? StateOfJoy?
pub struct State<'a> {
    pub(crate) stack: Stack,
    pub(crate) dict: Dictionary,
    // I've considered using a type parameter H: Host
    // But that would spray a little tiny type parameter over
    // the entire code and I didn't like that
    // Maybe I'll revisit at some point.
    pub(crate) host: &'a mut dyn Host,
    /// Prevents others in this crate from constructing a [State]
    /// (without running the builtin registration)
    private: PhantomData<()>,
}

impl<'a> State<'a> {
    pub fn new(host: &'a mut dyn Host) -> Self {
        let stack = Stack::new();
        let dict = Dictionary::new();
        let private = PhantomData;
        let mut state = State { stack, dict, host, private };
        register_builtins(&mut state).expect("registering builtins failed");
        state
    }

    pub fn stack(&self) -> &Stack { &self.stack }

    pub fn dict(&self) -> &Dictionary { &self.dict }

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
        match value {
            Value::List(ref nodes) => self.exec_list(nodes),
            _ => Err(crate::Error::ExecuteTypeError(value.kind())),
        }
    }

    pub fn eval(&mut self, input: &str) -> crate::Result {
        let result = scan(input).and_then(parse);
        for diag in result.report().iter() {
            println!("{diag}");
        }
        if let Some(value) = result.ok() {
            self.exec(value)
        } else {
            // we have already printed the diagnostics
            Ok(())
        }
    }
}
