use std::borrow::Cow;

use super::dictionary::NativeFn;
use super::dictionary::Word;
use super::grammar::ast::Ast;
use super::grammar::parser::parse;
use super::grammar::scanner::scan;
use super::interpreter::Interpreter;
use super::value::Value::*;

fn do_register(interpreter: &mut Interpreter) -> crate::Result {
    let mut define = |name: &'static str, func: NativeFn| -> crate::Result {
        interpreter.dict.define(Cow::Borrowed(name), Word::Native(func))
    };

    //////////////////////////
    // Primitive operations //
    //////////////////////////

    define("dup", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(a.clone());
        env.stack.push(a);
        Ok(())
    })?;

    define("swap", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    })?;

    define("over", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(a.clone());
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    })?;

    define("rot", |env| {
        let [a, b, c] = env.stack.parallel_pop()?;
        env.stack.push(b);
        env.stack.push(c);
        env.stack.push(a);
        Ok(())
    })?;

    define("drop", |env| {
        env.stack.pop()?;
        Ok(())
    })?;

    define("clear", |env| {
        env.stack.clear();
        Ok(())
    })?;

    define("exec", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.exec(a)
    })?;

    define("eval", |env| {
        let [source] = env.stack.parallel_pop()?;
        let source = source.into_string()?;

        let result = scan(&source).and_then(parse);
        for diag in result.report().iter() {
            println!("{diag}");

            if let Some(first) = result.report().iter().next() {
                return Err(crate::Error::ParseError(first.to_string().into()));
            }
        }
        if let Some(value) = result.ok().map(Ast::into_value) {
            env.stack.push(value);
        } else {
            // any diagnostics have already been thrown at this point
        }
        Ok(())
    })?;

    define("defun", |env| {
        let [name, body] = env.stack.parallel_pop()?;
        let name = name.into_string()?;
        let body = body.into_list()?;
        env.dict.define(Cow::Owned(name), Word::User(body))
    })?;

    //////////////////////
    // Primitive values //
    //////////////////////

    define("char", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Char(a.into_char()?));
        Ok(())
    })?;

    define("int", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Int(a.into_int()?));
        Ok(())
    })?;

    define("float", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Float(a.into_float()?));
        Ok(())
    })?;

    //////////////////
    // Input/output //
    //////////////////

    define(".", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.host.println(&a.into_string()?)
    })?;

    define("words", |env| {
        let words = &env.dict;
        env.host.println(&words.to_string())
    })?;

    //////////
    // Math //
    //////////

    define("+", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a + b));
        Ok(())
    })?;

    define("-", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a - b));
        Ok(())
    })?;

    define("*", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a * b));
        Ok(())
    })?;

    define("/", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a / b));
        Ok(())
    })?;

    ///////////
    // Logic //
    ///////////

    define("not", |env| {
        let [a] = env.stack.parallel_pop()?;
        let a = a.into_bool()?;
        env.stack.push(Bool(!a));
        Ok(())
    })?;

    define("=", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a == b));
        Ok(())
    })?;

    define("<", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a < b));
        Ok(())
    })?;

    define(">", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a > b));
        Ok(())
    })?;

    ////////
    // IO //
    ////////

    define("fs.read", |env| {
        let [filename] = env.stack.parallel_pop()?;
        let filename = filename.into_string()?;
        let contents = env.host.read_file(&filename)?;
        env.stack.push(Text(contents.into()));
        Ok(())
    })?;

    //////////////
    // Complete //
    //////////////

    Ok(())
}

impl<'h> Interpreter<'h> {
    pub(crate) fn register_builtins(&mut self) {
        do_register(self).expect("registering builtins failed");
    }
}
