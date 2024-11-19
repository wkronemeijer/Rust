pub mod builtins;
pub mod dictionary;
pub mod env;
pub mod error;
pub mod host;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod value;
pub mod word;

#[cfg(test)]
mod tests {
    use host::tests::TestHost;
    use interpreter::Interpreter;
    use value::Value::*;

    use super::*;

    #[test]
    fn push_numbers() -> crate::Result {
        let mut host = TestHost::new();
        let mut interpreter = Interpreter::new(&mut host);
        interpreter.eval("12 24")?;
        assert_eq!(interpreter.stack().depth(), 2);
        interpreter.eval("+")?;
        let result = interpreter.stack().peek().cloned();
        assert_eq!(result, Some(Int(36)));
        // TODO: check for output here
        Ok(())
    }
}
