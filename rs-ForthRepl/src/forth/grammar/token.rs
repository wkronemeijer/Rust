use std::fmt;

///////////////
// TokenKind //
///////////////

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    START_OF_FILE,
    END_OF_FILE,

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

#[derive(Debug, Clone, Copy)]
/// A single, Copy token from _some_ source string.
pub struct Token {
    kind: TokenKind,
    start: u32,
    len: u16,
}

impl Token {
    /// Creates a new Token. Fails if the given range can not be compressed.
    pub fn try_new(kind: TokenKind, start: usize, end: usize) -> Option<Self> {
        let u32_start = u32::try_from(start).ok()?;
        let u16_len = u16::try_from(end.saturating_sub(start)).ok()?;
        Some(Token { kind, start: u32_start, len: u16_len })
    }

    /// Creates a new token. Panics if the given range can not be compressed.
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self::try_new(kind, start, end).unwrap_or_else(|| {
            panic!("could not compress range {start}..{end}");
        })
    }

    /// This Token's kind.
    pub fn kind(&self) -> TokenKind { self.kind }

    // safe because conversion TO usize was successful, so conversion FROM usize should be safe too
    /// Uncompresses this token's start.
    pub fn start(&self) -> usize { usize::try_from(self.start).unwrap() }

    /// Uncompresses this token's end.
    pub fn end(&self) -> usize { self.start() + usize::from(self.len) }

    /// Slices the given string using this token's range. The token does not track from which string it came, so it might panic if slicing the wrong source string.
    pub fn lexeme<'s>(&self, source: &'s str) -> &'s str {
        &source[self.start()..self.end()]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind;
        let start = self.start();
        let end = self.end();
        write!(f, "{kind} [{start}..<{end}]")
    }
}
