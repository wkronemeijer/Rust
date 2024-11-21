use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::mem::replace; // <3

use super::builtins::register_builtins;
use super::dictionary::Dictionary;
use super::env::Env;
use super::host::Host;
use super::parser::parse;
use super::stack::Stack;
use super::word::Token;
use super::word::UserFunction;
use super::word::Word;

pub enum InterpreterCommand {
    /// i.e. ':'
    StartCompile,
    /// e.g. "dup", "rot", "1+"
    ExecuteToken(Token),
    /// i.e. ';'
    EndCompile,
    /// Like a null terminator
    EndOfInput,
}

#[derive(Default)]
pub enum InterpreterState {
    #[default]
    Interpreting,
    DefiningPrimed, // needs a name before able to define
    Defining(String, UserFunction),
    Failing,
}

// TODO: Interpreter is really more of the equivalent of LuaState
// ForthState?
pub struct Interpreter<'a> {
    stack: Stack,
    // We explicitly want to own this ↓
    words: Dictionary,
    state: InterpreterState,
    host: &'a mut dyn Host,
}

impl<'a> Interpreter<'a> {
    pub fn new(host: &'a mut dyn Host) -> Self {
        let mut interpreter = Interpreter {
            stack: Stack::new(),
            words: Dictionary::new(),
            state: InterpreterState::default(),
            host,
        };
        register_builtins(&mut interpreter.words);
        interpreter
    }

    pub fn stack(&self) -> &Stack { &self.stack }

    pub fn words(&self) -> &Dictionary { &self.words }

    fn env(&mut self) -> Env {
        Env::new(&self.words, &mut self.stack, self.host)
    }

    fn execute_command(&mut self, cmd: InterpreterCommand) -> crate::Result {
        use InterpreterCommand::*;
        use InterpreterState::*;
        use Token::*;

        let result = match cmd {
            StartCompile => match self.state {
                Interpreting => {
                    self.state = DefiningPrimed;
                    Ok(())
                }
                DefiningPrimed => {
                    Err(crate::Error::InvalidWordName(":".into()))
                }
                Defining(_, _) => Err(crate::Error::NestedCompile),
                Failing => Ok(()),
            },
            EndCompile => match replace(&mut self.state, Interpreting) {
                // Ignore duplicate ';'
                Interpreting => Ok(()),
                DefiningPrimed => Err(crate::Error::MissingName),
                Defining(name, tokens) => {
                    self.words.define(Word::custom(name, tokens))?;
                    Ok(())
                }
                Failing => Ok(()),
            },
            ExecuteToken(token) => match self.state {
                Interpreting => self.env().evaluate_token(&token),
                DefiningPrimed => match token {
                    PushValue(value) => Err(crate::Error::InvalidWordName(
                        value.to_string().into(),
                    )),
                    CallWord(name) => {
                        // Continue compiling even though the name might not be usable
                        // Prevents executing the definition body
                        if !self.words.has(&name) {
                            self.state = Defining(name, UserFunction::new());
                            Ok(())
                        } else {
                            self.state = Interpreting;
                            Err(crate::Error::NameAlreadyInUse(name.into()))
                        }
                    }
                },
                Defining(_, ref mut tokens) => {
                    tokens.push(token);
                    Ok(())
                }
                Failing => Ok(()),
            },
            EndOfInput => match self.state {
                Failing => {
                    self.state = Interpreting;
                    Ok(())
                }
                _ => Ok(()),
            },
        };
        if let Err(_) = result {
            self.state = Failing;
        }
        result
    }

    fn execute(&mut self, commands: Vec<InterpreterCommand>) -> crate::Result {
        commands.into_iter().try_for_each(|cmd| self.execute_command(cmd))
    }

    fn recover(&mut self) {
        match self.state {
            InterpreterState::Failing => {
                self.state = InterpreterState::default();
            }
            _ => {}
        }
    }

    pub fn eval(&mut self, input: &str) -> crate::Result {
        let commands = parse(input)?;
        let result = self.execute(commands);
        self.recover();
        result
    }

    pub fn prompt(&self) -> Cow<'static, str> {
        match self.state {
            InterpreterState::Interpreting => Borrowed("> "),
            InterpreterState::DefiningPrimed => Borrowed(": "),
            InterpreterState::Defining(ref name, ref tokens) => {
                Owned(format!(": {name} {tokens} ... "))
            }
            InterpreterState::Failing => panic!("interpreter should recover"),
        }
    }
}
