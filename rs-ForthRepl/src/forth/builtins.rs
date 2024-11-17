use super::dictionary::Dictionary;
use super::value::Value;

fn do_register(dict: &mut Dictionary) -> crate::Result {
    use Value::*;

    ///////////
    // Stack //
    ///////////

    dict.define_native("dup", |env| {
        let a = env.pop()?;
        env.push(a.clone());
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
        println!("{}", env.pop()?);
        Ok(())
    })?;

    dict.define_native("words", |env| {
        println!("{}", env.dict());
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
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a + b));
        Ok(())
    })?;

    dict.define_native("-", |env| {
        let b = env.pop()?.try_into_int()?;
        let a = env.pop()?.try_into_int()?;
        env.push(Int(a - b));
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
