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
            kind: TokenKind::Eof,
            lexeme: "".to_string(),
            location: SrcLocation {
                line: self.current_line,
                col: self.current_column,
            },
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), RutoxError> {
        match self.advance() {
            '(' => self.add_token(TokenKind::LParen),
            ')' => self.add_token(TokenKind::RParen),
            '{' => self.add_token(TokenKind::LBrace),
            '}' => self.add_token(TokenKind::RBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => {
                let kind = self.either('=', TokenKind::BangEqual, TokenKind::Bang);

                self.add_token(kind)
            }
            '=' => {
                let kind = self.either('=', TokenKind::EqualEqual, TokenKind::Equal);

                self.add_token(kind)
            }
            '<' => {
                let kind = self.either('=', TokenKind::LessEqual, TokenKind::Less);

                self.add_token(kind)
            }
            '>' => {
                let kind = self.either('=', TokenKind::GreaterEqual, TokenKind::Greater);

                self.add_token(kind)
            }
            '/' => {
                if self.matches('/') {
                    self.skip_comment();
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            '"' => self.consume_string()?,
            ' ' | '\t' | '\r' => (),
            '\n' => {
                self.current_line += 1;
                self.current_column = 0;
            }
            c if c.is_digit(10) => self.consume_number()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.consume_identifier(),
            c => {
                return Err(RutoxError::SyntaxError(
                    format!("Unexpected character: `{c}`"),
                    self.current_location(),
                ))
            }
        }

        Ok(())
    }

    fn consume_number(&mut self) -> Result<(), RutoxError> {
        self.consume_while(|c| c.is_digit(10));

        if let (Some('.'), Some(next)) = (self.peek(), self.peek_next()) {
            if next.is_digit(10) {
                self.advance();
                self.consume_while(|c| c.is_digit(10));
            }
        }

        let n_string = &self.source[self.start..self.current];
        let n = n_string.parse::<f64>().map_err(|_| {
            RutoxError::ProgrammerError(
                format!("Could not parse number `{n_string}`"),
                self.current_location(),
            )
        })?;
        self.add_token(TokenKind::Number(n));

        Ok(())
    }

    fn consume_string(&mut self) -> Result<(), RutoxError> {
        while let Some(ch) = self.peek() {
            if ch == '"' {
                break;
            }
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 0;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(RutoxError::SyntaxError(
                "Unterminated string".into(),
                self.current_location(),
            ));
        }

        self.expect('"')?;

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenKind::String(value));

        Ok(())
    }

    fn consume_identifier(&mut self) {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let text = self.current_token_string();
        let kind = self.keyword_to_token_kind(&text).unwrap_or(TokenKind::Identifier(text));
        self.add_token(kind);
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
                true
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

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn expect(&mut self, expected: char) -> Result<char, RutoxError> {
        let current = self.advance();

        if current == expected {
            Ok(current)
        } else {
            Err(RutoxError::ProgrammerError(
                format!("Expected `{expected}`, found `{current}`"),
                self.current_location(),
            ))
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        let text = self.current_token_string();

        self.tokens.push(Token {
            kind,
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

    fn current_token_string(&self) -> String {
        self.source[self.start..self.current].to_string()
    }

    fn current_location(&self) -> SrcLocation {
        SrcLocation {
            line: self.current_line,
            col: self.current_column,
        }
    }

    fn keyword_to_token_kind(&self, kw: &str) -> Option<TokenKind> {
        match kw {
            "and" => Some(TokenKind::And),
            "class" => Some(TokenKind::Class),
            "else" => Some(TokenKind::Else),
            "false" => Some(TokenKind::False),
            "for" => Some(TokenKind::For),
            "fun" => Some(TokenKind::Fun),
            "if" => Some(TokenKind::If),
            "nil" => Some(TokenKind::Nil),
            "or" => Some(TokenKind::Or),
            "print" => Some(TokenKind::Print),
            "return" => Some(TokenKind::Return),
            "super" => Some(TokenKind::Super),
            "this" => Some(TokenKind::This),
            "true" => Some(TokenKind::True),
            "var" => Some(TokenKind::Var),
            "while" => Some(TokenKind::While),
            _ => None,
        }
    }
}
