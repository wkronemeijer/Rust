use super::{
    dictionary::Dictionary,
    stack::Stack,
    value::Value,
    word::{Token, Word},
};

pub struct Env<'a> {
    pub dict: &'a Dictionary,
    stack: &'a mut Stack,
}

impl<'a> Env<'a> {
    pub fn new(dict: &'a Dictionary, stack: &'a mut Stack) -> Self {
        Self { dict, stack }
    }

    pub fn evaluate_token(&mut self, token: &Token) -> crate::Result<()> {
        match token {
            Token::PushValue(value) => self.stack.push(*value),
            Token::CallWord(name) => {
                let word = self.dict.get(name)?;
                self.evaluate_word(word)?;
            }
        };
        Ok(())
    }
    pub fn evaluate_word(&mut self, word: &Word) -> crate::Result<()> {
        match word {
            Word::Native(_, func) => func(self)?,
            Word::User(_, tokens) => tokens
                .iter()
                .try_for_each(|token| self.evaluate_token(token))?,
        }
        Ok(())
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> crate::Result<Value> {
        self.stack.pop()
    }
}
