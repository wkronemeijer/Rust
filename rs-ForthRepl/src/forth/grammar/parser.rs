use super::error::DiagnosticList;
use super::error::SyntaxError;
use super::result::CompileResult;
use super::scanner::TokenList;
use super::token::Token;
use super::token::TokenKind;
use super::token::TokenKind::*;
use crate::forth::value::Value;
use crate::forth::value::ValueList;

/// Slice off the `"` on both ends
fn extract_string<'s>(mut lexeme: &'s str, delim: char) -> &'s str {
    if lexeme.starts_with(delim) {
        lexeme = &lexeme[1..];
    }
    if lexeme.ends_with(delim) {
        lexeme = &lexeme[..lexeme.len() - 1];
    }
    // TODO: interpret escape sequences like \n
    lexeme
}

fn extract_char(lexeme: &str, report: &mut DiagnosticList) -> Option<char> {
    let mut chars = extract_string(lexeme, '\'').chars();
    let Some(c) = chars.next() else {
        report.error(SyntaxError::CharEmpty);
        return None;
    };
    let None = chars.next() else {
        report.error(SyntaxError::CharTooLong);
        return None;
    };
    Some(c)
}

struct Parser<'s> {
    source: &'s str,
    tokens: Vec<Token>,
    current: usize,
    report: DiagnosticList,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str, tokens: Vec<Token>) -> Self {
        let report = DiagnosticList::new();
        Parser { source, tokens, current: 0, report }
    }

    fn is_at_end(&self) -> bool { self.current >= self.tokens.len() }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.current) }

    /// Returns the previous [Parser::current] before advancing.
    fn advance(&mut self) -> Option<Token> {
        let value = self.peek().copied();
        self.current += 1;
        value
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        matches!(self.peek(), Some(token) if token.kind() == kind)
    }

    /// if check() then advance
    fn matches(&mut self, kind: TokenKind) -> bool {
        let matches = self.check(kind);
        if matches {
            self.advance();
        }
        matches
    }

    fn consume(&mut self, expected_kind: TokenKind) -> Option<Token> {
        let previous = self.advance();
        let Some(actual) = previous else { return None };
        let actual_kind = actual.kind();
        if actual_kind != expected_kind {
            self.report.error(SyntaxError::ExpectedToken {
                expected: expected_kind,
                actual: actual_kind,
            });
            return None;
        }
        Some(actual)
    }
}

// domain-specific
impl<'s> Parser<'s> {
    fn list_body(&mut self) -> Option<ValueList> {
        let mut elements = Vec::new();
        loop {
            if self.check(RIGHT_BRACKET) || self.check(END_OF_FILE) {
                break;
            }
            if self.matches(COMMENT) {
                continue;
            }
            if self.is_at_end() {
                return None;
            }
            elements.push(self.expr()?);
        }
        Some(ValueList::from_vec(elements))
    }

    fn list(&mut self) -> Option<Value> {
        let body = self.list_body()?;
        self.consume(RIGHT_BRACKET)?;
        Some(Value::List(body))
    }

    fn program(&mut self) -> Option<Value> {
        self.consume(START_OF_FILE)?;
        let body = self.list_body()?;
        self.consume(END_OF_FILE)?;
        Some(Value::List(body))
    }

    fn expr(&mut self) -> Option<Value> {
        let token = self.advance().expect("unexpected eof");
        match token.kind() {
            LEFT_BRACKET => self.list(),
            NULL => Some(Value::Null),
            FALSE => Some(Value::Bool(false)),
            TRUE => Some(Value::Bool(true)),
            NUMBER => {
                let lexeme = token.lexeme(self.source);
                let number = lexeme.parse::<f64>().ok()?;
                Some(Value::Number(number))
            }
            IDENTIFIER => {
                let lexeme = token.lexeme(self.source);
                Some(Value::Symbol(lexeme.to_owned().into()))
            }
            CHARACTER => {
                let lexeme = token.lexeme(self.source);
                let c = extract_char(lexeme, &mut self.report)?;
                Some(Value::Char(c))
            }
            STRING => {
                let lexeme = token.lexeme(self.source);
                Some(Value::Text(extract_string(lexeme, '"').to_owned().into()))
            }
            _ => {
                self.report.error(SyntaxError::UnexpectedToken(token.kind()));
                None
            }
        }
    }

    pub fn parse(mut self) -> CompileResult<Value> {
        if let Some(program) = self.program() {
            CompileResult::new(program, self.report)
        } else {
            self.report.error(SyntaxError::FailedToSynchronize);
            CompileResult::fail(self.report)
        }
    }
}

pub fn parse(TokenList(source, tokens): TokenList) -> CompileResult<Value> {
    Parser::new(source, tokens).parse()
}
