use super::ast::Ast;
use super::error::DiagnosticList;
use super::error::SyntaxError;
use super::result::CompileResult;
use super::scanner::TokenList;
use super::token::Token;
use super::token::TokenKind;
use super::token::TokenKind::*;

struct Sync;

type CanThrow<T = ()> = std::result::Result<T, Sync>;
// ...this is just Option<T>...

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

fn extract_char(lexeme: &str, report: &mut DiagnosticList) -> CanThrow<char> {
    let mut chars = extract_string(lexeme, '\'').chars();
    let Some(c) = chars.next() else {
        report.error(SyntaxError::CharTooShort);
        return Err(Sync);
    };
    let None = chars.next() else {
        report.error(SyntaxError::CharTooLong);
        return Err(Sync);
    };
    Ok(c)
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

    fn consume(&mut self, expected_kind: TokenKind) -> CanThrow<Token> {
        let previous = self.advance();
        let Some(actual) = previous else { return Err(Sync) };
        let actual_kind = actual.kind();
        if actual_kind != expected_kind {
            self.report.error(SyntaxError::ExpectedToken {
                expected: expected_kind,
                actual: actual_kind,
            });
            return Err(Sync);
        }
        Ok(actual)
    }
}

// domain-specific
impl<'s> Parser<'s> {
    fn list_body(&mut self) -> CanThrow<Vec<Ast>> {
        let mut elements = Vec::new();
        loop {
            if self.check(RIGHT_BRACKET) || self.check(END_OF_FILE) {
                break;
            }
            if self.matches(COMMENT) {
                continue;
            }
            if self.is_at_end() {
                return Err(Sync);
            }
            elements.push(self.expr()?);
        }
        Ok(elements)
    }

    fn list(&mut self) -> CanThrow<Ast> {
        let nodes = self.list_body()?;
        self.consume(RIGHT_BRACKET)?;
        Ok(Ast::List(nodes))
    }

    fn program(&mut self) -> CanThrow<Ast> {
        self.consume(START_OF_FILE)?;
        let nodes = self.list_body()?;
        self.consume(END_OF_FILE)?;
        Ok(Ast::List(nodes))
    }

    fn expr(&mut self) -> CanThrow<Ast> {
        let Some(token) = self.advance() else {
            panic!("unexpected eof");
        };
        match token.kind() {
            LEFT_BRACKET => self.list(),
            NULL => Ok(Ast::Null),
            FALSE => Ok(Ast::False),
            TRUE => Ok(Ast::True),
            NUMBER => {
                let lexeme = token.lexeme(self.source);
                let number = lexeme.parse::<f64>().map_err(|_| Sync)?;
                Ok(Ast::Number(number))
            }
            IDENTIFIER => {
                let lexeme = token.lexeme(self.source);
                Ok(Ast::Identifier(lexeme.to_owned()))
            }
            CHARACTER => {
                let lexeme = token.lexeme(self.source);
                let c = extract_char(lexeme, &mut self.report)?;
                Ok(Ast::Char(c))
            }
            STRING => {
                let lexeme = token.lexeme(self.source);
                Ok(Ast::String(extract_string(lexeme, '"').to_owned()))
            }
            _ => {
                self.report.error(SyntaxError::UnexpectedToken(token.kind()));
                Err(Sync)
            }
        }
    }

    pub fn parse(mut self) -> CompileResult<Ast> {
        if let Ok(program) = self.program() {
            CompileResult::new(program, self.report)
        } else {
            self.report.error(SyntaxError::FailedToSynchronize);
            CompileResult::fail(self.report)
        }
    }
}

pub fn parse(TokenList(source, tokens): TokenList) -> CompileResult<Ast> {
    Parser::new(source, tokens).parse()
}
