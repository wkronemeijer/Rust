use std::error;
use std::fmt;

use thiserror::Error;

use super::token::TokenKind;

///////////
// Error //
///////////
// We don't have any warning right now
// So all this code is a little simpler (for now)
// TODO: Do we split `ScannerError`s from `ParserError`s?
// It /is/ neater...
// unfortunately you can't easily take the union of different Error kinds

#[derive(Debug, Clone, Error)]
pub enum SyntaxError {
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char),

    #[error("unterminated literal")]
    UnterminatedLiteral,

    #[error("expected token '{expected}', received '{actual}'")]
    ExpectedToken { expected: TokenKind, actual: TokenKind },

    #[error("unexpected token '{0}'")]
    UnexpectedToken(TokenKind),

    #[error("failed to synchronize")]
    FailedToSynchronize,
}

////////////////
// Diagnostic //
////////////////

#[derive(Debug, Clone)]
pub enum Diagnostic {
    Error(SyntaxError),
}

impl Diagnostic {
    pub fn is_fatal(&self) -> bool { matches!(self, Self::Error(..)) }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // TODO: Maybe the warning prefix adds to much clutter
            Self::Error(e) => write!(f, "error: {e}"),
        }
    }
}

impl error::Error for Diagnostic {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Diagnostic::Error(e) => e.source(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> { self.source() }
}

/////////////////////
// Diagnostic List //
/////////////////////

#[derive(Debug, Clone)]
pub struct DiagnosticList {
    data: Vec<Diagnostic>,
}

impl DiagnosticList {
    pub fn new() -> Self { DiagnosticList { data: Vec::new() } }

    pub fn is_fatal(&self) -> bool {
        self.data.iter().any(Diagnostic::is_fatal)
    }

    pub fn error(&mut self, error: SyntaxError) {
        self.data.push(Diagnostic::Error(error));
    }

    pub fn join(self, other: DiagnosticList) -> DiagnosticList {
        DiagnosticList {
            data: self.into_iter().chain(other.into_iter()).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> { self.data.iter() }

    pub fn into_iter(self) -> impl Iterator<Item = Diagnostic> {
        self.data.into_iter()
    }
}

impl fmt::Display for DiagnosticList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self.iter() {
            item.fmt(f)?;
        }
        Ok(())
    }
}
