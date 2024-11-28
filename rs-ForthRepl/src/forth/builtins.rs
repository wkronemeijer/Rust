use std::borrow::Cow;

use super::dictionary::NativeFn;
use super::dictionary::Word;
use super::state::State;
use super::value::Value::*;
use crate::parsing::parser::parse;
use crate::parsing::scanner::scan;

pub(crate) fn register_builtins(interpreter: &mut State) -> crate::Result {
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

    define("drop.all", |env| {
        env.stack.clear();
        Ok(())
    })?;

    ////////////////
    // Reflection //
    ////////////////

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
        if let Some(value) = result.ok() {
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

    //////////////////
    // Control flow //
    //////////////////

    define("branch", |env| {
        let [bool, if_true, if_false] = env.stack.parallel_pop()?;
        let bool = bool.into_bool()?;
        env.exec(if bool { if_true } else { if_false })
    })?;

    define("ifte", |env| {
        let [bool_body, if_true, if_false] = env.stack.parallel_pop()?;
        let top = env.stack.pop()?;
        env.stack.push(top.clone());
        env.stack.push(top);
        env.exec(bool_body)?;
        let bool = env.stack.pop()?.into_bool()?;
        env.exec(if bool { if_true } else { if_false })
    })?;

    //////////////////////
    // Primitive values //
    //////////////////////

    define("bool", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a.into_bool()?));
        Ok(())
    })?;

    define("char", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Char(a.into_char()?));
        Ok(())
    })?;

    define("int", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Number(a.into_int()? as f64));
        Ok(())
    })?;

    define("number", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Number(a.into_float()?));
        Ok(())
    })?;

    //////////////////
    // Input/output //
    //////////////////

    define("put", |env| {
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
        env.stack.push(Number(a + b));
        Ok(())
    })?;

    define("-", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Number(a - b));
        Ok(())
    })?;

    define("*", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Number(a * b));
        Ok(())
    })?;

    define("/", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Number(a / b));
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

    define("==", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a == b));
        Ok(())
    })?;

    define("!=", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a != b));
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

    /////////////////////
    // In-code defined //
    /////////////////////

    interpreter.eval(
        r#"
"import" [fs.read eval exec] defun
    "#,
    )?;

    //////////////
    // Complete //
    //////////////

    Ok(())
}
