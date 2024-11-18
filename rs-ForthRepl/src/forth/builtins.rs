use super::dictionary::Dictionary;
use super::value::Value;

fn do_register(dict: &mut Dictionary) -> crate::Result {
    use Value::*;

    ////////////////
    // Primitives //
    ////////////////

    dict.define_native("null", |env| {
        env.push(Null);
        Ok(())
    })?;

    dict.define_native("true", |env| {
        env.push(Bool(true));
        Ok(())
    })?;

    dict.define_native("false", |env| {
        env.push(Bool(false));
        Ok(())
    })?;

    ///////////
    // Stack //
    ///////////

    dict.define_native("dup", |env| {
        let a = env.pop()?;
        env.push(a.clone());
        env.push(a);
        Ok(())
    })?;

    dict.define_native("swap", |env| {
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(b);
        env.push(a);
        Ok(())
    })?;

    dict.define_native("over", |env| {
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(a.clone());
        env.push(b);
        env.push(a);
        Ok(())
    })?;

    dict.define_native("rot", |env| {
        let c = env.pop()?;
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(b);
        env.push(c);
        env.push(a);
        Ok(())
    })?;

    dict.define_native("drop", |env| {
        env.pop()?;
        Ok(())
    })?;

    //////////////////
    // Input/output //
    //////////////////

    dict.define_native(".", |env| {
        let a = env.pop()?;
        env.println(&format!("{a}"))
    })?;

    dict.define_native("words", |env| {
        let dict = env.dict();
        env.println(&format!("{dict}"))
    })?;

    //////////
    // Math //
    //////////

    dict.define_native("+", |env| {
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a.checked_add(b).ok_or(crate::Error::IntegerRange)?));
        Ok(())
    })?;

    dict.define_native("-", |env| {
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a.checked_sub(b).ok_or(crate::Error::IntegerRange)?));
        Ok(())
    })?;

    dict.define_native("*", |env| {
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a.checked_mul(b).ok_or(crate::Error::IntegerRange)?));
        Ok(())
    })?;

    dict.define_native("/", |env| {
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a.checked_div(b).ok_or(crate::Error::IntegerRange)?));
        Ok(())
    })?;

    ///////////
    // Logic //
    ///////////

    dict.define_native("not", |env| {
        let value = env.pop()?.try_into_bool()?;
        env.push(Bool(!value));
        Ok(())
    })?;

    dict.define_native("=", |env| {
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(Bool(a == b));
        Ok(())
    })?;

    dict.define_native("<", |env| {
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(Bool(a < b));
        Ok(())
    })?;

    dict.define_native(">", |env| {
        let b = env.pop()?;
        let a = env.pop()?;
        env.push(Bool(a > b));
        Ok(())
    })?;

    //////////////
    // Complete //
    //////////////

    Ok(())
}

pub(crate) fn register_builtins(dict: &mut Dictionary) {
    do_register(dict).expect("registering builtins failed");
}
