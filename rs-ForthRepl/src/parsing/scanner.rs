use super::error::DiagnosticList;
use super::error::SyntaxError;
use super::result::CompileResult;
use super::token::Token;
use super::token::TokenKind;
use super::token::TokenKind::*;

/// Re-implementation of the old `char_at`
///
/// See [this GitHub file](https://github.com/rust-lang/regex/blob/1a069b9232c607b34c4937122361aa075ef573fa/regex-syntax/src/ast/parse.rs#L483) for more info
fn try_char_at(this: &str, i: usize) -> Option<char> {
    if this.is_char_boundary(i) { this[i..].chars().next() } else { None }
}

/////////////
// Scanner //
/////////////

struct Scanner<'s> {
    source: &'s str,
    start: usize,
    current: usize,
    report: DiagnosticList,
}

// Domain-unspecific scanning methods
impl<'s> Scanner<'s> {
    pub fn new(source: &str) -> Scanner {
        let start = 0;
        let current = 0;
        let report = DiagnosticList::new();
        Scanner { source, start, current, report }
    }

    fn lexeme(&self) -> &str { &self.source[self.start..self.current] }

    fn peek(&self) -> Option<char> { try_char_at(self.source, self.current) }

    fn advance(&mut self) -> Option<char> {
        let value = self.peek()?;
        self.current += value.len_utf8();
        debug_assert!(
            self.source.is_char_boundary(self.current),
            "current must always lie on a char boundary"
        );
        Some(value)
    }

    fn advance_while(&mut self, pred: fn(char) -> bool) {
        loop {
            let Some(c) = self.peek() else { break };
            let true = pred(c) else { break };
            self.advance();
        }
    }

    fn advance_until(&mut self, pred: fn(char) -> bool) -> Option<()> {
        loop {
            let Some(c) = self.peek() else {
                self.report.error(SyntaxError::UnterminatedLiteral);
                break None;
            };
            if pred(c) {
                break Some(());
            };
            self.advance();
        }
    }

    fn sync(&mut self) { self.start = self.current }

    fn token(&mut self, kind: TokenKind) -> Token {
        Token::new(kind, self.start, self.current)
    }
}

// TODO: Use is_ascii_??? directly
fn is_whitespace(c: char) -> bool { matches!(c, ' ' | '\t' | '\r' | '\n') }
fn is_digit(c: char) -> bool { matches!(c, '0'..='9') }
fn is_delimiter(c: char) -> bool {
    matches!(c, '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'' | '`')
}
// TODO: Support emoji names
fn is_alphanum(c: char) -> bool { !(is_whitespace(c) || is_delimiter(c)) }

// Domain-specific scanning methods
impl<'s> Scanner<'s> {
    fn finish_number(&mut self) -> Token {
        self.advance_while(is_digit);
        if let Some('.') = self.peek() {
            self.advance(); // consume the .
            self.advance_while(is_digit);
        }
        self.token(NUMBER)
    }

    fn finish_comment(&mut self) -> Option<Token> {
        self.advance_until(|c| c == ')')?;
        self.advance(); // consume the )
        Some(self.token(COMMENT))
    }

    fn finish_string(&mut self) -> Option<Token> {
        // TODO: Choose to restrict strings to a single line
        self.advance_until(|c| c == '"')?;
        self.advance(); // consume the "
        Some(self.token(STRING))
    }

    fn finish_character(&mut self) -> Option<Token> {
        // TODO: Choose to restrict strings to a single line
        self.advance_until(|c| c == '\'')?;
        self.advance(); // consume the "
        Some(self.token(CHARACTER))
    }

    fn finish_identifier(&mut self) -> Token {
        self.advance_while(is_alphanum);
        self.token(match self.lexeme() {
            "null" => NULL,
            "false" => FALSE,
            "true" => TRUE,
            _ => IDENTIFIER,
        })
    }

    fn scan_one(&mut self) -> Option<Token> {
        Some(loop {
            self.sync();
            let Some(c) = self.advance() else { return None };
            // sync + advance means we are always progressing
            break match c {
                _ if is_whitespace(c) => continue,
                '[' => self.token(LEFT_BRACKET),
                ']' => self.token(RIGHT_BRACKET),
                '(' => self.finish_comment()?,
                '"' => self.finish_string()?,
                '\'' => self.finish_character()?,
                '-' if self.peek().is_some_and(is_digit) => {
                    self.finish_number()
                },
                _ if is_digit(c) => self.finish_number(),
                _ if is_alphanum(c) => self.finish_identifier(),
                _ => {
                    self.report.error(SyntaxError::UnexpectedCharacter(c));
                    continue;
                },
            };
        })
    }

    fn scan(mut self) -> CompileResult<Vec<Token>> {
        let mut tokens = Vec::new();
        tokens.push(self.token(START_OF_FILE));
        while let Some(token) = self.scan_one() {
            tokens.push(token);
        }
        tokens.push(self.token(END_OF_FILE));
        CompileResult::new(tokens, self.report)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> { self.scan_one() }
}

#[derive(Debug)]
pub struct TokenList<'s>(pub &'s str, pub Vec<Token>);

/// Synonym for [Scanner::new]
pub fn scan(source: &str) -> CompileResult<TokenList> {
    Scanner::new(source).scan().map(|data| TokenList(source, data))
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans() {
        let source = "3 dup 1 + [(a b -- a)] 5 * 35. 1.42 \n / .load 😀😂🤣";
        let ref mut scanner = Scanner::new(source);

        fn check(actual_token: Option<Token>, expected: Option<TokenKind>) {
            let actual = actual_token.map(|t| t.kind());
            if actual != expected {
                panic!("expected {expected:?}, encountered {actual:?}")
            }
        }

        check(scanner.next(), Some(NUMBER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), Some(NUMBER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), Some(LEFT_BRACKET));
        check(scanner.next(), Some(COMMENT));
        check(scanner.next(), Some(RIGHT_BRACKET));
        check(scanner.next(), Some(NUMBER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), Some(NUMBER));
        check(scanner.next(), Some(NUMBER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), Some(IDENTIFIER));
        check(scanner.next(), None);
    }
}
