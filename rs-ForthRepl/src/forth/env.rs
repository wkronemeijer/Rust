use super::dictionary::Dictionary;
use super::stack::Stack;
use super::value::Value;
use super::word::{Token, Word, WordKind};

pub struct Env<'a> {
    dict: &'a Dictionary,
    stack: &'a mut Stack,
}

impl<'a> Env<'a> {
    pub fn new(dict: &'a Dictionary, stack: &'a mut Stack) -> Self {
        Self { dict, stack }
    }

    pub fn dict(&self) -> &Dictionary {
        self.dict
    }

    pub fn evaluate_token(&mut self, token: &Token) -> crate::Result {
        match token {
            Token::PushValue(value) => self.stack.push(*value),
            Token::CallWord(name) => self.evaluate_word(self.dict.get(name)?)?,
        };
        Ok(())
    }

    pub fn evaluate_word(&mut self, word: &Word) -> crate::Result {
        word.evaluate(self)
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    #[must_use]
    pub fn pop(&mut self) -> crate::Result<Value> {
        self.stack.pop()
    }
}

trait Evaluate {
    fn evaluate(&self, env: &mut Env) -> crate::Result;
}

impl Evaluate for WordKind {
    fn evaluate(&self, env: &mut Env) -> crate::Result {
        match self {
            WordKind::Native(body) => body.body()(env),
            WordKind::User(user) => user.iter().try_for_each(|token| env.evaluate_token(token)),
        }
    }
}

impl Evaluate for Word {
    fn evaluate(&self, env: &mut Env) -> crate::Result {
        self.kind().evaluate(env)
    }
}
