use super::interpreter::InterpreterCommand;
use super::value::Value;
use super::word::Token;

fn parse_word(word: &str) -> crate::Result<Token> {
    use Token::*;
    use Value::*;

    if let Ok(number) = word.parse::<i32>() {
        Ok(PushValue(Int(number)))
    } else if word == "true" {
        Ok(PushValue(Bool(true)))
    } else if word == "false" {
        Ok(PushValue(Bool(false)))
    } else {
        // TODO: Maybe check with word regex?
        // Then again, "1+" is a valid word
        Ok(CallWord(word.to_string()))
    }
}

fn parse_token(token: &str) -> crate::Result<InterpreterCommand> {
    if token == ":" {
        Ok(InterpreterCommand::StartCompile)
    } else if token == ";" {
        Ok(InterpreterCommand::EndCompile)
    } else {
        Ok(InterpreterCommand::ExecuteToken(parse_word(token)?))
    }
}

pub fn parse(input: &str) -> crate::Result<Vec<InterpreterCommand>> {
    let commands: crate::Result<Vec<_>> =
        input.split_ascii_whitespace().map(parse_token).collect();
    let mut commands = commands?;
    commands.push(InterpreterCommand::EndOfInput);
    Ok(commands)
}
