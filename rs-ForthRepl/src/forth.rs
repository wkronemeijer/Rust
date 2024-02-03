pub mod error;
pub mod value;

use std::collections::HashMap;
use std::fmt::{Display, Write};

use self::error::ForthError;
use self::error::ForthResult;
use self::value::ForthValue;

type ForthFunc = fn(&mut ForthStack) -> ForthResult<()>;

// Note: you can just use a singleton vec
// Would simplify things for sure...

pub struct ForthWord {
    name: String,
    def: Vec<ForthFunc>,
}

impl ForthWord {
    pub fn native(name: String, body: ForthFunc) -> ForthWord {
        ForthWord {
            name,
            def: vec![body],
        }
    }

    pub fn custom(name: String, def: Vec<ForthFunc>) -> ForthWord {
        ForthWord { name, def }
    }
}

////////////////
// ForthBlock //
////////////////

pub struct ForthStack {
    vec: Vec<ForthValue>,
}

impl ForthStack {
    const AVERAGE_STACK_DEPTH: usize = 1 << 4;

    pub fn new() -> ForthStack {
        ForthStack {
            vec: Vec::with_capacity(Self::AVERAGE_STACK_DEPTH),
        }
    }

    pub fn depth(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, value: ForthValue) {
        self.vec.push(value)
    }

    pub fn pop(&mut self) -> ForthResult<ForthValue> {
        self.vec.pop().ok_or(ForthError::StackUnderflow)
    }

    fn run_word(&mut self, word: &ForthWord) -> ForthResult<()> {
        for sub in &word.def {
            sub(self)?;
        }
        Ok(())
    }
}

impl Display for ForthStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.vec.iter() {
            f.write_str(&item.to_string())?;
            f.write_char(' ')?;
            // TODO: Is there an intersperse method?
        }
        Ok(())
    }
}

///////////////////////
// Forth Interpreter //
///////////////////////

enum ForthInterpreterState {
    Interpreting,
    Primed, // needs a name
    Compiling(Vec<ForthFunc>),
}

pub struct ForthInterpreter {
    pub stack: ForthStack,
    // We want to own
    words: HashMap<String, ForthWord>,
    state: ForthInterpreterState,
}

impl ForthInterpreter {
    pub fn new() -> ForthInterpreter {
        let mut result = ForthInterpreter {
            stack: ForthStack::new(),
            words: HashMap::new(),
            state: ForthInterpreterState::Interpreting,
        };
        result.register_stdlib();
        return result;
    }

    pub fn define_word(&mut self, word: ForthWord) {
        self.words.insert(word.name.clone(), word);
    }

    // fn process_word(&mut self, word: &str) -> ForthResult<()> {
    //     if let Ok(number) = word.parse::<i32>() {
    //         self.stack.push(ForthValue::Int(number));
    //         continue;
    //     } else if word == ":" {
    //         todo!("start compilation");
    //     } else if word == ";" {
    //         todo!("end compilation");
    //     } else if let Some(word) = self.words.get(word) {
    //         // Insert compiling check
    //         self.stack.run_word(word)?;
    //     } else {
    //         return Err(ForthError::ParseError {
    //             culprit: word.to_owned(),
    //         });
    //     }
    // }

    pub fn parse(&mut self, input: &str) -> Result<(), ForthError> {
        for word in input.split_ascii_whitespace() {
            if word == ":" {
                todo!("start compilation");
            } else if word == ";" {
                todo!("end compilation");
            } else if let Ok(number) = word.parse::<i32>() {
                self.stack.push(ForthValue::Int(number));
            } else if let Some(word) = self.words.get(word) {
                // Insert compiling check
                self.stack.run_word(word)?;
            } else {
                return Err(ForthError::ParseError {
                    culprit: word.to_owned(),
                });
            }
        }
        Ok(())
    }

    pub fn print_stack(&self) {
        print!("\x1b[30m");
        if self.stack.depth() > 0 {
            print!("{}", self.stack);
        } else {
            print!("(empty stack)");
        }
        println!("\x1b[0m");
    }
}

//////////////////
// Native words //
//////////////////
// I wonder if you can move this to another file...

impl ForthInterpreter {
    fn define_native_word(&mut self, name: &'static str, body: ForthFunc) {
        self.define_word(ForthWord::native(name.to_string(), body))
    }

    fn register_stdlib(&mut self) {
        self.define_native_word("+", |inter| {
            let a = inter.pop()?.as_int()?;
            let b = inter.pop()?.as_int()?;
            inter.push(ForthValue::Int(a + b));
            Ok(())
        });

        self.define_native_word("-", |inter| {
            let a = inter.pop()?.as_int()?;
            let b = inter.pop()?.as_int()?;
            inter.push(ForthValue::Int(a - b));
            Ok(())
        });

        self.define_native_word("dup", |inter| {
            let a = inter.pop()?;
            inter.push(a);
            inter.push(a);
            Ok(())
        });

        self.define_native_word(".", |inter| {
            println!("{}", inter.pop()?);
            Ok(())
        });
    }
}
