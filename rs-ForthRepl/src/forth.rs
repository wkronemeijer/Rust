pub mod builtins;
pub mod dictionary;
pub mod env;
pub mod error;
pub mod interpreter;
pub mod stack;
pub mod value;
pub mod word;

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;

    use super::*;
    use crate::prelude::*;

    fn new_boxed_interpreter() -> Box<Interpreter> {
        Box::new(Interpreter::new())
    }

    #[test]
    fn push_numbers() -> Result<()> {
        let mut interpreter = new_boxed_interpreter();
        interpreter.read_and_execute("12 24")?;
        assert_eq!(interpreter.stack.depth(), 2);
        Ok(())
    }
}
