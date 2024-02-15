use super::{dictionary::Dictionary, value::Value};

pub(crate) fn register_builtins(dict: &mut Dictionary) -> crate::Result<()> {
    ///////////
    // Stack //
    ///////////

    dict.define_native("dup", |env| {
        let a = env.pop()?;
        env.push(a);
        env.push(a);
        Ok(())
    })?;

    //////////////////
    // Input/output //
    //////////////////

    dict.define_native(".", |env| {
        println!("{}", env.pop()?);
        Ok(())
    })?;

    dict.define_native("words", |env| {
        println!("{}", env.dict);
        Ok(())
    })?;

    ///////////
    // Logic //
    ///////////

    dict.define_native("not", |env| {
        let value = env.pop()?.try_into_bool()?;
        env.push(Value::Bool(!value));
        Ok(())
    })?;

    //////////
    // Math //
    //////////

    dict.define_native("+", |env| {
        let a = env.pop()?.try_into_int()?;
        let b = env.pop()?.try_into_int()?;
        env.push(Value::Int(a + b));
        Ok(())
    })?;

    dict.define_native("-", |env| {
        let a = env.pop()?.try_into_int()?;
        let b = env.pop()?.try_into_int()?;
        env.push(Value::Int(a - b));
        Ok(())
    })?;

    Ok(())
}
