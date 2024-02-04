mod builtins;
pub mod dictionary;
pub mod error;
pub mod stack;
pub mod value;

use std::fmt::Display;
use std::io::stdout;
use std::io::Write;
use std::mem::replace;

use self::builtins::register_builtins;
use self::dictionary::Dictionary;
use self::error::Error;
use self::error::Result;
use self::stack::Stack;
use self::value::Value;

type NativeFunction = fn(&mut Stack) -> Result<()>;
type UserFunction = Vec<Token>;

pub enum Token {
    PushValue(Value),
    CallWord(String), // Late bound!
}

// Should words know their own name?
// Feels like a place for other metadata too...
pub enum Word {
    Native(String, NativeFunction),
    User(String, UserFunction),
}

impl Word {
    pub fn name(&self) -> &str {
        match self {
            Word::Native(name, _) => name,
            Word::User(name, _) => name,
        }
    }

    pub fn native(name: String, body: NativeFunction) -> Word {
        Word::Native(name, body)
    }

    pub fn custom(name: String, def: UserFunction) -> Word {
        Word::User(name, def)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::Native(name, _) => write!(f, "native word '{}'", name)?,
            Word::User(name, tokens) => write!(f, "user word '{}' ({} long)", name, tokens.len())?,
        }
        Ok(())
    }
}

////////////////
// ForthBlock //
////////////////
///

struct Env<'a> {
    dict: &'a dictionary::Dictionary,
    stack: &'a mut stack::Stack,
}

impl<'a> Env<'a> {
    fn new(dict: &'a dictionary::Dictionary, stack: &'a mut stack::Stack) -> Self {
        Self { dict, stack }
    }

    fn evaluate_token(&mut self, token: &Token) -> Result<()> {
        match token {
            Token::PushValue(value) => self.stack.push(*value),
            Token::CallWord(name) => {
                let word = self.dict.get(name)?;
                self.evaluate_word(word)?;
            }
        };
        Ok(())
    }
    fn evaluate_word(&mut self, word: &Word) -> Result<()> {
        match word {
            Word::Native(_, func) => func(self.stack)?,
            Word::User(_, tokens) => tokens
                .iter()
                .try_for_each(|token| self.evaluate_token(token))?,
        }
        Ok(())
    }
}

///////////////////////
// Forth Interpreter //
///////////////////////

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
    Compile(String, Vec<Token>),
}

pub struct Interpreter {
    pub stack: stack::Stack,
    // We want to own
    pub words: dictionary::Dictionary,
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
                let new_state = Interpret;
                let old_state = replace(&mut self.state, new_state);
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
                        if !self.words.has(&name) {
                            self.state = Compile(name, vec![]);
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
            InterpreterState::Compile(_, _) => print!("& "),
        }
        stdout().flush().expect("couldn't flush 🤢");
    }

    pub fn print_stack(&self) {
        print!("\x1b[30m");
        let depth = self.stack.depth();
        print!("({}) ", depth);
        if depth > 0 {
            print!("{}", self.stack);
        } else {
            print!("-");
        }
        println!("\x1b[0m");
    }
}
