pub mod ast;
pub mod visitors;
use crate::rutox_error::RutoxError;
use crate::scanner::{
    token::{Token, TokenKind},
    SrcLocation,
};
use ast::{BinaryData, Expr, LiteralData, Stmt, UnaryData, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, RutoxError> {
        let mut stmts = vec![];

        while !self.is_at_end() {
            stmts.push(self.statement()?)
        }

        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, RutoxError> {
        if self.match_any(&[TokenKind::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, RutoxError> {
        let value = self.expression()?;
        self.expect(TokenKind::Semicolon, "Expect `;` after value")?;

        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RutoxError> {
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon, "Expect `;` after expression")?;

        Ok(Stmt::Expr(expr))
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
                location: operator.location.clone(),
                operator: operator.into(),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.term()?;

        while self.match_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                location: operator.location.clone(),
                operator: operator.into(),
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
                location: operator.location.clone(),
                operator: operator.into(),
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
                location: operator.location.clone(),
                operator: operator.into(),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RutoxError> {
        if self.match_any(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let expr = self.unary()?;
            let op = if operator.kind == TokenKind::Bang {
                UnaryOp::Bang(operator.location.clone())
            } else {
                UnaryOp::Minus(operator.location.clone())
            };

            return Ok(Expr::Unary(UnaryData {
                expr: Box::new(expr),
                location: operator.location,
                operator: op,
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

                Ok(Expr::Grouping(Box::new(expr), token.location.clone()))
            }
            _ => Err(RutoxError::Syntax(
                format!("Expect expression, got `{}`", token),
                token.location.clone(),
            )),
        }
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().unwrap().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => self.advance(),
            };
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
            Err(RutoxError::Syntax(
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
        self.peek().is_none() || self.peek().unwrap().kind == TokenKind::Eof
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
            Err(RutoxError::Syntax(
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

#[cfg(test)]
mod tests {
    use super::ast::BinaryOp;
    use super::*;

    #[test]
    fn it_parses_unary_exprs() {
        let tokens = vec![
            token(TokenKind::Bang, 1, 1),
            token(TokenKind::Number(1.0), 1, 2),
            token(TokenKind::Semicolon, 1, 3),
        ];

        let result = Parser::new(tokens).parse().ok().unwrap();

        assert_eq!(
            result,
            vec![stmt_expr(unary_expr(
                number(1.0, 1, 2),
                UnaryOp::Bang(SrcLocation::new(1, 1)),
                SrcLocation::new(1, 1)
            ))]
        );

        let tokens = vec![
            token(TokenKind::Minus, 1, 1),
            token(TokenKind::Number(1.0), 1, 2),
            token(TokenKind::Semicolon, 1, 3),
        ];

        let result = Parser::new(tokens).parse().ok().unwrap();

        assert_eq!(
            result,
            vec![stmt_expr(unary_expr(
                number(1.0, 1, 2),
                UnaryOp::Minus(SrcLocation::new(1, 1)),
                SrcLocation::new(1, 1)
            ))]
        );
    }

    #[test]
    fn it_parses_equality() {
        let tokens = vec![
            token(TokenKind::Number(1.0), 1, 1),
            token(TokenKind::EqualEqual, 1, 2),
            token(TokenKind::Number(2.0), 1, 4),
            token(TokenKind::Semicolon, 1, 5),
        ];

        let result = Parser::new(tokens).parse().ok().unwrap();

        assert_eq!(
            result,
            vec![stmt_expr(binary_expr(
                number(1.0, 1, 1),
                BinaryOp::EqualEqual(SrcLocation::new(1, 2)),
                number(2.0, 1, 4),
                SrcLocation::new(1, 2)
            ))]
        );
    }

    #[test]
    fn it_parses_inequality() {
        let tokens = vec![
            token(TokenKind::Number(1.0), 1, 1),
            token(TokenKind::BangEqual, 1, 2),
            token(TokenKind::Number(2.0), 1, 4),
            token(TokenKind::Semicolon, 1, 5),
        ];

        let result = Parser::new(tokens).parse().ok().unwrap();

        assert_eq!(
            result,
            vec![stmt_expr(binary_expr(
                number(1.0, 1, 1),
                BinaryOp::BangEqual(SrcLocation::new(1, 2)),
                number(2.0, 1, 4),
                SrcLocation::new(1, 2)
            ))]
        );
    }

    #[test]
    fn it_parses_comparison() {
        let comparison_token_kinds = [
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ];

        for token_kind in comparison_token_kinds {
            let tokens = vec![
                token(TokenKind::Number(1.0), 1, 1),
                token(token_kind, 1, 2),
                token(TokenKind::Number(2.0), 1, 4),
                token(TokenKind::Semicolon, 1, 5),
            ];

            let result = Parser::new(tokens.clone()).parse().ok().unwrap();

            assert_eq!(
                result,
                vec![stmt_expr(binary_expr(
                    number(1.0, 1, 1),
                    tokens[1].clone().into(),
                    number(2.0, 1, 4),
                    SrcLocation::new(1, 2)
                ))]
            );
        }
    }

    #[test]
    fn it_parses_mathematical_operations() {
        let math_token_kinds = [
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
        ];

        for token_kind in math_token_kinds {
            let tokens = vec![
                token(TokenKind::Number(1.0), 1, 1),
                token(token_kind, 1, 2),
                token(TokenKind::Number(2.0), 1, 3),
                token(TokenKind::Semicolon, 1, 4),
            ];

            let result = Parser::new(tokens.clone()).parse().ok().unwrap();

            assert_eq!(
                result,
                vec![stmt_expr(binary_expr(
                    number(1.0, 1, 1),
                    tokens[1].clone().into(),
                    number(2.0, 1, 3),
                    SrcLocation::new(1, 2)
                ))]
            )
        }
    }

    fn stmt_expr(expr: Expr) -> Stmt {
        Stmt::Expr(expr)
    }

    fn binary_expr(left: Expr, operator: BinaryOp, right: Expr, location: SrcLocation) -> Expr {
        Expr::Binary(BinaryData {
            left: Box::new(left),
            right: Box::new(right),
            operator,
            location,
        })
    }

    fn unary_expr(expr: Expr, operator: UnaryOp, location: SrcLocation) -> Expr {
        Expr::Unary(UnaryData {
            expr: Box::new(expr),
            operator,
            location,
        })
    }

    fn number(value: f64, line: usize, column: usize) -> Expr {
        Expr::Literal(LiteralData::Number(value, SrcLocation::new(line, column)))
    }

    fn token(kind: TokenKind, line: usize, col: usize) -> Token {
        Token {
            kind,
            lexeme: "".to_string(),
            location: SrcLocation::new(line, col),
        }
    }
}
