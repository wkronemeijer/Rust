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

/// Slice off the `"` on both ends
fn unescape(mut lexeme: &str) -> &str {
    if lexeme.starts_with('"') {
        lexeme = &lexeme[1..];
    }

    if lexeme.ends_with('"') {
        lexeme = &lexeme[..lexeme.len() - 1];
    }
    // TODO: interpret sequences like \n
    lexeme
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

    fn expr(&mut self) -> CanThrow<Ast> {
        if self.matches(LEFT_BRACKET) {
            self.list()
        } else if self.matches(NULL) {
            Ok(Ast::Null)
        } else if self.matches(FALSE) {
            Ok(Ast::False)
        } else if self.matches(TRUE) {
            Ok(Ast::True)
        } else if self.check(NUMBER) {
            let token = self.consume(NUMBER)?;
            let lexeme = token.lexeme(self.source);
            let number = lexeme.parse::<f64>().map_err(|_| Sync)?;
            Ok(Ast::Number(number))
        } else if self.check(IDENTIFIER) {
            let token = self.consume(IDENTIFIER)?;
            let lexeme = token.lexeme(self.source);
            Ok(Ast::Identifier(lexeme.to_owned()))
        } else if self.check(STRING) {
            let token = self.consume(STRING)?;
            let lexeme = token.lexeme(self.source);
            Ok(Ast::StringLiteral(unescape(lexeme).to_owned()))
        } else {
            let Some(token) = self.advance() else {
                panic!("unexpected eof");
            };
            self.report.error(SyntaxError::UnexpectedToken(token.kind()));
            Err(Sync)
        }
    }

    fn inner_parse(&mut self) -> CanThrow<Ast> {
        self.consume(START_OF_FILE)?;
        let nodes = self.list_body()?;
        self.consume(END_OF_FILE)?;
        Ok(Ast::List(nodes))
    }

    pub fn parse(mut self) -> CompileResult<Ast> {
        if let Ok(program) = self.inner_parse() {
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
