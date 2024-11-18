use super::dictionary::Dictionary;
use super::host::Host;
use super::stack::Stack;
use super::value::Value;
use super::word::Token;
use super::word::Word;
use super::word::WordKind;

pub struct Env<'a> {
    dict: &'a Dictionary,
    stack: &'a mut Stack,
    output: &'a mut dyn Host,
}

impl<'a> Env<'a> {
    pub fn new(
        dict: &'a Dictionary,
        stack: &'a mut Stack,
        output: &'a mut dyn Host,
    ) -> Self {
        Self { dict, stack, output }
    }

    pub fn dict(&self) -> &Dictionary { self.dict }

    pub fn stack(&self) -> &Stack { self.stack }

    fn evaluate_word(&mut self, word: &Word) -> crate::Result {
        match word.kind() {
            WordKind::Native(func) => func.body()(self),
            WordKind::User(func) => {
                func.iter().try_for_each(|token| self.evaluate_token(token))
            }
        }
    }

    pub fn evaluate_token(&mut self, token: &Token) -> crate::Result {
        use Token::*;
        match token {
            PushValue(value) => self.stack.push(value.clone()),
            CallWord(name) => self.evaluate_word(self.dict.get(name)?)?,
        };
        Ok(())
    }

    pub fn push(&mut self, value: Value) { self.stack.push(value) }

    #[must_use]
    pub fn pop(&mut self) -> crate::Result<Value> { self.stack.pop() }

    pub fn println(&mut self, line: &str) -> crate::Result {
        self.output.println(line)?;
        Ok(())
    }
}
