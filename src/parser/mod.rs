pub mod ast;
pub mod ast_printer;
pub mod visitor;
use crate::rutox_error::RutoxError;
use crate::scanner::token::{Token, TokenKind};
use ast::{BinaryData, Expr, LiteralData, UnaryData};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, RutoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, RutoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.comparison()?;

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.primary()?;

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.primary()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, RutoxError> {
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::True | TokenKind::False => Ok(Expr::Literal(LiteralData::Bool(
                    token.kind == TokenKind::True,
                    token.location.clone(),
                ))),
                TokenKind::Number(n) => Ok(Expr::Literal(LiteralData::Number(
                    *n,
                    token.location.clone(),
                ))),
                TokenKind::String(s) => Ok(Expr::Literal(LiteralData::String(
                    s.clone(),
                    token.location.clone(),
                ))),
                TokenKind::Nil => Ok(Expr::Literal(LiteralData::Nil(token.location.clone()))),
                _ => Err(RutoxError::SyntaxError(
                    format!("Unexpected token kind `{}`", token.kind),
                    token.location.clone(),
                )),
            }
        } else {
            Err(RutoxError::SyntaxError(
                "Unexpected end of input".to_string(),
                self.previous().location,
            ))
        }
    }

    // helpers

    fn match_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        match self.peek() {
            Some(token) => token.kind == *kind,
            None => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }
}
