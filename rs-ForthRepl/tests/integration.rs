use forth_repl::forth::host::TestHost;
use forth_repl::forth::interpreter::Interpreter;
use forth_repl::forth::value::Value::*;

#[test]
fn push_numbers() -> forth_repl::Result {
    let mut host = TestHost::new();
    let mut interpreter = Interpreter::new(&mut host);
    interpreter.eval("12 24")?;
    assert_eq!(interpreter.stack().depth(), 2);
    interpreter.eval("+")?;
    let result = interpreter.stack().peek().cloned();
    assert_eq!(result, Some(Float(36.0)));
    // TODO: check for output here
    Ok(())
}
