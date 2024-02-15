use crate::prelude::*;

use super::{dictionary::Dictionary, value::Value};

pub(crate) fn register_builtins(dict: &mut Dictionary) -> Result<()> {
    use Value::*;

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
        env.push(Bool(!value));
        Ok(())
    })?;

    //////////
    // Math //
    //////////

    dict.define_native("+", |env| {
        let a = env.pop()?.try_into_int()?;
        let b = env.pop()?.try_into_int()?;
        env.push(Int(a + b));
        Ok(())
    })?;

    dict.define_native("-", |env| {
        let a = env.pop()?.try_into_int()?;
        let b = env.pop()?.try_into_int()?;
        env.push(Int(a - b));
        Ok(())
    })?;

    Ok(())
}
