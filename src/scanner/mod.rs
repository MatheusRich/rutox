pub mod src_location;
mod token;
use super::rutox_error::RutoxError;
use src_location::SrcLocation;
use token::{Token, TokenKind};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    current_line: usize,
    current_column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            current_line: 1,
            current_column: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, RutoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?
        }

        self.tokens.push(Token {
            kind: TokenKind::EOF,
            lexeme: "".to_string(),
            literal: None,
            location: SrcLocation {
                line: self.current_line,
                col: self.current_column,
            },
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), RutoxError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenKind::LParen, None),
            ')' => self.add_token(TokenKind::RParen, None),
            '{' => self.add_token(TokenKind::LBrace, None),
            '}' => self.add_token(TokenKind::RBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '+' => self.add_token(TokenKind::Plus, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            '*' => self.add_token(TokenKind::Star, None),
            '!' => {
                let kind = self.either('=', TokenKind::BangEqual, TokenKind::Bang);

                self.add_token(kind, None)
            }
            '=' => {
                let kind = self.either('=', TokenKind::EqualEqual, TokenKind::Equal);

                self.add_token(kind, None)
            }
            '<' => {
                let kind = self.either('=', TokenKind::LessEqual, TokenKind::Less);

                self.add_token(kind, None)
            }
            '>' => {
                let kind = self.either('=', TokenKind::GreaterEqual, TokenKind::Greater);

                self.add_token(kind, None)
            }
            '/' => {
                if self.matches('/') {
                    self.skip_comment();
                } else {
                    self.add_token(TokenKind::Slash, None);
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => {
                self.current_line += 1;
                self.current_column = 0;
            },
            _ => {
                return Err(RutoxError::SyntaxError(
                    format!("Unexpected character: `{c}`"),
                    self.current_location(),
                ))
            }
        }

        Ok(())
    }

    fn skip_comment(&mut self) {
        self.consume_while(|c| c != '\n');
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.current_column += 1;

        self.source
            .chars()
            .nth(self.current - 1)
            .expect("Called advance, but scanner is at end of source")
    }

    fn matches(&mut self, expected: char) -> bool {
        self.consume_if(|c| c == expected)
    }

    fn either(&mut self, expected: char, matched: TokenKind, unmatched: TokenKind) -> TokenKind {
        if self.matches(expected) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();

        while let Some(ch) = self.peek() {
            if f(ch) {
                self.advance();
                chars.push(ch)
            } else {
                break;
            }
        }

        chars
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(c) = self.peek() {
            if f(c) {
                self.advance();
                return true;
            } else {
                false
            }
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();

        self.tokens.push(Token {
            kind,
            literal,
            lexeme: text,
            location: SrcLocation {
                line: self.current_line,
                col: self.current_column,
            },
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn current_location(&self) -> SrcLocation {
        SrcLocation {
            line: self.current_line,
            col: self.current_column,
        }
    }
}
