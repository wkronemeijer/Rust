use std::borrow::Cow;

use super::dictionary::NativeFn;
use super::dictionary::Word;
use super::interpreter::Interpreter;
use super::value::Value::*;

fn do_register(ip: &mut Interpreter) -> crate::Result {
    // TODO: this could really use a macro to pop values in stack order notation
    // Would also fix pop().convert(), pop().convert() ordering issue
    // a b -- r where (r == a + b)

    ///////////
    // Stack //
    ///////////

    ip.define_native("dup", |env| {
        let a = env.stack.pop()?;
        env.stack.push(a.clone());
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("swap", |env| {
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("over", |env| {
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
        env.stack.push(a.clone());
        env.stack.push(b);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("rot", |env| {
        let c = env.stack.pop()?;
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
        env.stack.push(b);
        env.stack.push(c);
        env.stack.push(a);
        Ok(())
    });

    ip.define_native("drop", |env| {
        env.stack.pop()?;
        Ok(())
    });

    //////////////////
    // Input/output //
    //////////////////

    ip.define_native(".", |env| {
        let a = env.stack.pop()?;
        env.host.println(&a.to_string())
    });

    ip.define_native("words", |env| {
        let words = &env.words;
        env.host.println(&words.to_string())
    });

    //////////
    // Math //
    //////////

    ip.define_native("+", |env| {
        let b = env.stack.pop()?.into_number()?;
        let a = env.stack.pop()?.into_number()?;
        env.stack.push(Number(a + b));
        Ok(())
    });

    ip.define_native("-", |env| {
        let b = env.stack.pop()?.into_number()?;
        let a = env.stack.pop()?.into_number()?;
        env.stack.push(Number(a - b));
        Ok(())
    });

    ip.define_native("*", |env| {
        let b = env.stack.pop()?.into_number()?;
        let a = env.stack.pop()?.into_number()?;
        env.stack.push(Number(a * b));
        Ok(())
    });

    ip.define_native("/", |env| {
        let b = env.stack.pop()?.into_number()?;
        let a = env.stack.pop()?.into_number()?;
        env.stack.push(Number(a / b));
        Ok(())
    });

    ///////////
    // Logic //
    ///////////

    ip.define_native("not", |env| {
        let value = env.stack.pop()?.into_bool()?;
        env.stack.push(Bool(!value));
        Ok(())
    });

    ip.define_native("=", |env| {
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
        env.stack.push(Bool(a == b));
        Ok(())
    });

    ip.define_native("<", |env| {
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
        env.stack.push(Bool(a < b));
        Ok(())
    });

    ip.define_native(">", |env| {
        let b = env.stack.pop()?;
        let a = env.stack.pop()?;
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
