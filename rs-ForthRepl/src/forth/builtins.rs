use std::borrow::Cow;

use super::dictionary::NativeFn;
use super::dictionary::Word;
use super::interpreter::Interpreter;
use super::value::Value::*;

fn do_register(ip: &mut Interpreter) -> crate::Result {
    //////////////////////////
    // Primitive operations //
    //////////////////////////

    ip.define_native("dup", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(a.clone());
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("swap", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("over", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(a.clone());
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("rot", |env| {
        let [a, b, c] = env.stack.parallel_pop()?;
        env.stack.push(b);
        env.stack.push(c);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("drop", |env| {
        env.stack.pop()?;
        Ok(())
    });

    ip.define_native("clear", |env| {
        env.stack.clear();
        Ok(())
    });

    ip.define_native("exec", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.exec(a)?;
        Ok(())
    });

    //////////////////////
    // Primitive values //
    //////////////////////

    ip.define_native("char", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Char(a.into_char()?));
        Ok(())
    });

    ip.define_native("int", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Int(a.into_int()?));
        Ok(())
    });

    ip.define_native("float", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.stack.push(Float(a.into_float()?));
        Ok(())
    });

    //////////////////
    // Input/output //
    //////////////////

    ip.define_native(".", |env| {
        let [a] = env.stack.parallel_pop()?;
        env.host.println(&a.into_string()?)
    });

    ip.define_native("words", |env| {
        let words = &env.words;
        env.host.println(&words.to_string())
    });

    //////////
    // Math //
    //////////

    ip.define_native("+", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a + b));
        Ok(())
    });

    ip.define_native("-", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a - b));
        Ok(())
    });

    ip.define_native("*", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a * b));
        Ok(())
    });

    ip.define_native("/", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        let a = a.into_float()?;
        let b = b.into_float()?;
        env.stack.push(Float(a / b));
        Ok(())
    });

    ///////////
    // Logic //
    ///////////

    ip.define_native("not", |env| {
        let [a] = env.stack.parallel_pop()?;
        let a = a.into_bool()?;
        env.stack.push(Bool(!a));
        Ok(())
    });

    ip.define_native("=", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a == b));
        Ok(())
    });

    ip.define_native("<", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a < b));
        Ok(())
    });

    ip.define_native(">", |env| {
        let [a, b] = env.stack.parallel_pop()?;
        env.stack.push(Bool(a > b));
        Ok(())
    });

    //////////////
    // Complete //
    //////////////

    Ok(())
}

impl<'h> Interpreter<'h> {
    pub(crate) fn define_native(
        &mut self,
        name: &'static str,
        function: NativeFn,
    ) {
        self.words.define(Cow::Borrowed(name), Word::Native(function))
    }

    pub(crate) fn register_builtins(&mut self) {
        do_register(self).expect("registering builtins failed");
    }
}
