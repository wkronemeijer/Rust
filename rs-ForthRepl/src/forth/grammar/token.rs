use std::fmt;

///////////////
// TokenKind //
///////////////

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum TokenKind {
    LEFT_BRACKET,
    RIGHT_BRACKET,

    NULL,
    FALSE,
    TRUE,
    NUMBER,
    STRING,

    IDENTIFIER,
    COMMENT,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

///////////
// Token //
///////////

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Token { kind, start, end }
    }

    pub fn start(&self) -> usize { self.start }

    pub fn end(&self) -> usize { self.end }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start()..self.end()]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Token { kind, .. } = self;
        let start = self.start();
        let end = self.end();
        write!(f, "{kind} [{start}..<{end}]")
    }
}
