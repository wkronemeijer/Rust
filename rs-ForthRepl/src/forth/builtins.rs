use super::{dictionary::Dictionary, error::Result, value::Value};

pub(crate) fn register_builtins(dict: &mut Dictionary) -> Result<()> {
    ///////////
    // Stack //
    ///////////

    dict.define_native("dup", |inter| {
        let a = inter.pop()?;
        inter.push(a);
        inter.push(a);
        Ok(())
    })?;

    //////////////////
    // Input/output //
    //////////////////

    dict.define_native(".", |inter| {
        println!("{}", inter.pop()?);
        Ok(())
    })?;

    ///////////
    // Logic //
    ///////////

    dict.define_native("not", |inter| {
        let value = inter.pop()?.try_into_bool()?;
        inter.push(Value::Bool(!value));
        Ok(())
    })?;

    //////////
    // Math //
    //////////

    dict.define_native("+", |inter| {
        let a = inter.pop()?.try_into_int()?;
        let b = inter.pop()?.try_into_int()?;
        inter.push(Value::Int(a + b));
        Ok(())
    })?;

    dict.define_native("-", |inter| {
        let a = inter.pop()?.try_into_int()?;
        let b = inter.pop()?.try_into_int()?;
        inter.push(Value::Int(a - b));
        Ok(())
    })?;

    Ok(())
}
