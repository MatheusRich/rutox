mod token;
use super::rutox_error::RutoxError;
use token::{SrcLocation, Token, TokenKind};

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
            current_column: 1,
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
            _ => return Err(RutoxError::SyntaxError(format!("Unexpected character: {}", c))),
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.current_column += 1;

        self.source
            .chars()
            .nth(self.current - 1)
            .expect("Called advance, but scanner is at end of source")
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
}
