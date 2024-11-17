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
    use value::Value::*;

    use super::*;

    fn interpreter() -> Box<Interpreter> { Box::new(Interpreter::new()) }

    #[test]
    fn push_numbers() -> crate::Result {
        let mut interpreter = interpreter();
        interpreter.eval("12 24")?;
        assert_eq!(interpreter.stack().depth(), 2);
        interpreter.eval("+")?;
        let result = interpreter.stack().peek().cloned();
        assert_eq!(result, Some(Int(36)));
        Ok(())
    }
}
