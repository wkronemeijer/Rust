use std::io::stdout;
use std::io::Write;
use std::mem::replace; // <3

use super::builtins::register_builtins;
use super::dictionary::Dictionary;
use super::env::Env;
use super::error::Result;
use super::stack::Stack;
use super::value::Value;
use super::word::Token;
use super::word::UserFunction;
use super::word::Word;

pub use super::error::Error;

enum InterpreterCommand {
    /// ':'
    StartCompile,
    Token(Token),
    /// ';'
    EndCompile,
}

enum InterpreterState {
    Interpret,
    Define, // needs a name
    Compile(String, UserFunction),
}

pub struct Interpreter {
    stack: Stack,
    // We want to own
    words: Dictionary,
    state: InterpreterState,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut result = Interpreter {
            stack: Stack::new(),
            words: Dictionary::new(),
            state: InterpreterState::Interpret,
        };
        register_builtins(&mut result.words).expect("registering builtins should not fail");
        return result;
    }

    fn parse_word(&mut self, word: &str) -> Result<Token> {
        if let Ok(number) = word.parse::<i32>() {
            Ok(Token::PushValue(Value::Int(number)))
        } else if word == "true" {
            Ok(Token::PushValue(Value::Bool(true)))
        } else if word == "false" {
            Ok(Token::PushValue(Value::Bool(false)))
        } else {
            // TODO: Maybe check with word regex?
            // Then again, "1+" is a valid word
            Ok(Token::CallWord(word.to_string()))
        }
    }

    fn parse_token(&mut self, token: &str) -> Result<InterpreterCommand> {
        if token == ":" {
            Ok(InterpreterCommand::StartCompile)
        } else if token == ";" {
            Ok(InterpreterCommand::EndCompile)
        } else {
            Ok(InterpreterCommand::Token(self.parse_word(token)?))
        }
    }

    fn parse(&mut self, input: &str) -> Result<Vec<InterpreterCommand>> {
        let commands: Result<Vec<InterpreterCommand>> = input
            .split_ascii_whitespace()
            .map(|token| self.parse_token(token))
            .collect();
        Ok(commands?)
    }

    fn execute_command(&mut self, cmd: InterpreterCommand) -> Result<()> {
        use self::InterpreterCommand::*;
        use self::InterpreterState::*;
        use self::Token::*;

        match cmd {
            StartCompile => match self.state {
                Interpret => {
                    self.state = Define;
                    Ok(())
                }
                Define => Err(Error::InvalidWordName(":".to_string())),
                Compile(_, _) => Err(Error::NestedCompile),
            },
            EndCompile => {
                let old_state = replace(&mut self.state, Interpret);
                match old_state {
                    // Ignore duplicate ';'
                    Interpret => Ok(()),
                    Define => Err(Error::MissingBody),
                    Compile(name, tokens) => {
                        self.words.define(Word::custom(name, tokens))?;
                        Ok(())
                    }
                }
            }
            Token(token) => match self.state {
                Interpret => {
                    let mut env = Env::new(&self.words, &mut self.stack);
                    env.evaluate_token(&token)
                }
                Define => match token {
                    PushValue(value) => Err(Error::InvalidWordName(value.to_string())),
                    CallWord(name) => {
                        // Continue compiling even though the name might not be usable
                        // Prevents executing the definition body
                        if !self.words.has(&name) {
                            self.state = Compile(name, UserFunction(vec![]));
                            Ok(())
                        } else {
                            self.state = Interpret;
                            Err(Error::NameAlreadyInUse(name))
                        }
                    }
                },
                Compile(_, ref mut tokens) => {
                    tokens.push(token);
                    Ok(())
                }
            },
        }
    }

    fn execute(&mut self, commands: Vec<InterpreterCommand>) -> Result<()> {
        commands
            .into_iter()
            .try_for_each(|cmd| self.execute_command(cmd))
    }

    pub fn read_and_execute(&mut self, input: &str) -> Result<()> {
        let commands = self.parse(input)?;
        self.execute(commands)?;
        Ok(())
    }

    pub fn print_prompt(&self) {
        match self.state {
            InterpreterState::Interpret => print!("> "),
            InterpreterState::Define => print!(": "),
            InterpreterState::Compile(ref name, ref tokens) => {
                print!(": {} {} ... ", name, tokens)
            }
        }
        stdout().flush().expect("couldn't flush 🤢");
    }

    pub fn print_stack(&self) {
        print!("\x1b[30m");
        let depth = self.stack.depth();
        if depth > 0 {
            print!("{}", self.stack);
        } else {
            print!("-");
        }
        println!("\x1b[0m");
    }
}
