pub mod ast;
pub mod ast_printer;
pub mod visitor;
use crate::rutox_error::RutoxError;
use crate::scanner::{
    token::{Token, TokenKind},
    SrcLocation,
};
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
        let mut expr = self.term()?;

        while self.match_any(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.factor()?;

        while self.match_any(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.unary()?;

        while self.match_any(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RutoxError> {
        if self.match_any(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let expr = self.unary()?;

            return Ok(Expr::Unary(UnaryData {
                expr: Box::new(expr),
                operator,
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, RutoxError> {
        let token = self.try_advance()?;

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
            &TokenKind::LParen => {
                let expr = self.expression()?;
                self.expect(TokenKind::RParen, "Expect `)` after expression")?;

                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(RutoxError::SyntaxError(
                format!("Expect expression, got `{}`", token),
                token.location.clone(),
            )),
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

    fn try_advance(&mut self) -> Result<Token, RutoxError> {
        if self.is_at_end() {
            Err(RutoxError::SyntaxError(
                "Unexpected end of input".to_string(),
                self.previous().location,
            ))
        } else {
            self.advance();
            Ok(self.previous())
        }
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

    fn expect(&mut self, kind: TokenKind, message: &str) -> Result<Token, RutoxError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(RutoxError::SyntaxError(
                message.to_string(),
                self.current_location(),
            ))
        }
    }

    fn current_location(&self) -> SrcLocation {
        match self.peek() {
            Some(token) => token.location.clone(),
            None => self.previous().location,
        }
    }
}
