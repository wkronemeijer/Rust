use TokenKind::*;

use super::token::Token;
use super::token::TokenKind;
use crate::core::CharIndex as _;

/////////////
// Scanner //
/////////////

pub struct Scanner<'source> {
    source: &'source str,
    start: usize,
    current: usize,
}

// Domain-unspecific scanning methods
impl<'source> Scanner<'source> {
    pub fn new(source: &str) -> Scanner {
        let start = 0;
        let current = 0;
        Scanner { source, start, current }
    }

    fn lexeme(&self) -> &str { &self.source[self.start..self.current] }

    fn peek(&self) -> Option<char> { self.source.try_char_at(self.current) }

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

    fn advance_until(&mut self, pred: fn(char) -> bool) {
        loop {
            let Some(c) = self.peek() else { break };
            let false = pred(c) else { break };
            self.advance();
        }
    }

    fn sync(&mut self) { self.start = self.current }

    fn token(&mut self, kind: TokenKind) -> Token {
        Token::new(kind, self.start, self.current)
    }
}

fn is_whitespace(c: char) -> bool { matches!(c, ' ' | '\t' | '\r' | '\n') }
fn is_digit(c: char) -> bool { matches!(c, '0'..='9') }
// Forth is pretty tolerant when it comes to identifiers
// Safe because it is always matched last
fn is_alphanum(c: char) -> bool { !is_whitespace(c) }

// Domain-specific scanning methods
impl<'source> Scanner<'source> {
    fn finish_number(&mut self) -> Token {
        self.advance_while(is_digit);
        if let Some('.') = self.peek() {
            self.advance(); // consume the .
            self.advance_while(is_digit);
        }
        self.token(NUMBER)
    }

    fn finish_comment(&mut self) -> Token {
        self.advance_until(|c| c == ')');
        self.advance(); // consume the )
        self.token(COMMENT)
    }

    fn finish_string(&mut self) -> Token {
        // TODO: Choose to restrict strings to a single line
        self.advance_until(|c| c == '"');
        self.token(STRING)
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

    fn scan(&mut self) -> Option<Token> {
        Some(loop {
            self.sync();
            let Some(c) = self.advance() else { return None };
            // sync + advance means we are always progressing
            break match c {
                _ if is_whitespace(c) => continue,
                '[' => self.token(LEFT_BRACKET),
                ']' => self.token(RIGHT_BRACKET),
                '(' => self.finish_comment(),
                '"' => self.finish_string(),
                _ if is_digit(c) => self.finish_number(),
                _ if is_alphanum(c) => self.finish_identifier(),
                _ => {
                    // TODO: report unexpected token (replacing print)
                    println!("unexpected char '{c}'");
                    continue;
                }
            };
        })
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> { self.scan() }
}

/// Synonym for [Scanner::new]
pub fn scan(source: &str) -> Scanner { Scanner::new(source) }

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
            let actual = actual_token.map(|token| token.kind);
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
