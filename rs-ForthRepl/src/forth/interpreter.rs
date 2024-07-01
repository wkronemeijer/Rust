use std::io::{stdout, Write};
use std::mem::replace; // <3

use super::builtins::register_builtins;
use super::dictionary::Dictionary;
use super::env::Env;
use super::stack::Stack;
use super::value::Value;
use super::word::{Token, UserFunction, Word, WordName};

enum InterpreterCommand {
    /// i.e. ':'
    StartCompile,
    /// e.g. "dup", "rot", "1+"
    Token(Token),
    /// i.e. ';'
    EndCompile,
    /// Like a null terminator
    EndOfInput,
}

#[derive(Default)]
enum InterpreterState {
    #[default]
    Interpreting,
    DefiningPrimed, // needs a name before able to define
    Defining(WordName, UserFunction),
    Failing,
}

pub struct Interpreter {
    pub stack: Stack,
    // We want to own
    pub words: Dictionary,
    state: InterpreterState,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut result = Interpreter {
            stack: Stack::new(),
            words: Dictionary::new(),
            state: InterpreterState::default(),
        };
        register_builtins(&mut result.words);
        result
    }

    fn parse_word(&self, word: &str) -> crate::Result<Token> {
        if let Ok(number) = word.parse::<i32>() {
            Ok(Token::PushValue(Value::Int(number)))
        } else if word == "true" {
            Ok(Token::PushValue(Value::Bool(true)))
        } else if word == "false" {
            Ok(Token::PushValue(Value::Bool(false)))
        } else {
            // TODO: Maybe check with word regex?
            // Then again, "1+" is a valid word
            Ok(Token::CallWord(WordName::new(word.to_string())?))
        }
    }

    fn parse_token(&self, token: &str) -> crate::Result<InterpreterCommand> {
        if token == ":" {
            Ok(InterpreterCommand::StartCompile)
        } else if token == ";" {
            Ok(InterpreterCommand::EndCompile)
        } else {
            Ok(InterpreterCommand::Token(self.parse_word(token)?))
        }
    }

    fn parse(&self, input: &str) -> crate::Result<Vec<InterpreterCommand>> {
        let commands: crate::Result<Vec<_>> = input
            .split_ascii_whitespace()
            .map(|token| self.parse_token(token))
            .collect();
        let mut commands = commands?;
        commands.push(InterpreterCommand::EndOfInput);
        Ok(commands)
    }

    fn execute_command(&mut self, cmd: InterpreterCommand) -> crate::Result {
        use self::InterpreterCommand::*;
        use self::InterpreterState::*;
        use self::Token::*;

        let result = match cmd {
            StartCompile => match self.state {
                Interpreting => {
                    self.state = DefiningPrimed;
                    Ok(())
                }
                DefiningPrimed => Err(crate::Error::InvalidWordName(":".to_string())),
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
            Token(token) => match self.state {
                Interpreting => Env::new(&self.words, &mut self.stack).evaluate_token(&token),
                DefiningPrimed => match token {
                    PushValue(value) => Err(crate::Error::InvalidWordName(value.to_string())),
                    CallWord(name) => {
                        // Continue compiling even though the name might not be usable
                        // Prevents executing the definition body
                        if !self.words.has(&name) {
                            self.state = Defining(name, UserFunction::new());
                            Ok(())
                        } else {
                            self.state = Interpreting;
                            Err(crate::Error::NameAlreadyInUse(name.into_inner()))
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
            InterpreterState::Failing => self.state = InterpreterState::default(),
            _ => {}
        }
    }

    pub fn eval(&mut self, input: &str) -> crate::Result {
        let commands = self.parse(input)?;
        let result = self.execute(commands);
        self.recover();
        result
    }

    pub fn print_prompt(&self) {
        match self.state {
            InterpreterState::Interpreting => print!("> "),
            InterpreterState::DefiningPrimed => print!(": "),
            InterpreterState::Defining(ref name, ref tokens) => {
                print!(": {} {} ... ", name, tokens)
            }
            InterpreterState::Failing => panic!("interpreter should have recovered"),
        }
        stdout().flush().expect("couldn't flush 🤢");
    }

    pub fn print_stack(&self) {
        let depth = self.stack.depth();
        if depth > 0 {
            print!("\x1b[30m");
            print!("{}", self.stack);
            println!("\x1b[0m");
        }
    }
}
