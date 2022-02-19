pub mod ast;
pub mod visitors;
use crate::rutox_error::RutoxError;
use crate::scanner::{
    token::{Token, TokenKind},
    SrcLocation,
};
use ast::{BinaryData, Expr, LiteralData, LogicalOp, Stmt, UnaryData, UnaryOp};

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
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    errors.push(err);
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(stmts)
        } else {
            Err(RutoxError::Multiple(errors))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, RutoxError> {
        if self.match_any(&[TokenKind::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, RutoxError> {
        let var_keyword_location = self.previous_location();

        if !self.is_at_end() {
            let token = self.advance();

            match &token.kind {
                TokenKind::Identifier(_name) => {
                    let mut location = var_keyword_location;
                    let mut initializer = None;

                    if self.match_any(&[TokenKind::Equal]) {
                        location = self.previous_location();
                        initializer = Some(self.expression()?);
                    }

                    self.expect(TokenKind::Semicolon, "Expect semicolon after declaration")?;

                    Ok(Stmt::Var(token.clone(), initializer, location))
                }
                _ => Err(RutoxError::Syntax(
                    format!("Expect variable name, got {token}"),
                    self.previous_location(),
                )),
            }
        } else {
            Err(RutoxError::Syntax(
                "Expect variable name, got EOF".to_string(),
                self.previous_location(),
            ))
        }
    }

    fn statement(&mut self) -> Result<Stmt, RutoxError> {
        if self.match_any(&[TokenKind::If]) {
            return self.if_statement();
        }
        if self.match_any(&[TokenKind::Print]) {
            return self.print_statement();
        }
        if self.match_any(&[TokenKind::LBrace]) {
            return Ok(Stmt::Block(self.block()?, self.previous_location()));
        }

        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, RutoxError> {
        let if_keyword_location = self.previous_location();
        self.expect(TokenKind::LParen, "Expect `(` after `if`")?;
        let condition = self.expression()?;
        self.expect(TokenKind::RParen, "Expect `)` after if condition")?;

        let then = self.statement()?;
        let else_branch = if self.match_any(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(
            condition,
            Box::new(then),
            else_branch,
            if_keyword_location,
        ))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, RutoxError> {
        let mut stmts = vec![];

        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        self.expect(TokenKind::RBrace, "Expect `}` after block")?;

        Ok(stmts)
    }

    fn print_statement(&mut self) -> Result<Stmt, RutoxError> {
        let value = self.expression()?;
        self.expect(TokenKind::Semicolon, "Expect `;` after print value")?;

        Ok(Stmt::Print(value, self.previous_location()))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RutoxError> {
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon, "Expect `;` after expression")?;

        Ok(Stmt::Expr(expr.clone(), expr.location()))
    }

    fn expression(&mut self) -> Result<Expr, RutoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RutoxError> {
        let expr = self.or()?;

        if self.match_any(&[TokenKind::Equal]) {
            let operator = self.previous();
            let value = self.assignment()?;

            match &expr {
                Expr::Variable(name, _location) => {
                    let location = operator.location;

                    return Ok(Expr::Assign(name.clone(), Box::new(value), location));
                }
                _ => {
                    return Err(RutoxError::Syntax(
                        format!("Expect assignment target to be a variable, got {:?}", expr),
                        expr.location(),
                    ))
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.and()?;

        while self.match_any(&[TokenKind::Or]) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical(
                Box::new(expr),
                LogicalOp::Or(operator.location.clone()),
                Box::new(right),
                operator.location,
            );
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, RutoxError> {
        let mut expr = self.equality()?;

        while self.match_any(&[TokenKind::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical(
                Box::new(expr),
                LogicalOp::And(operator.location.clone()),
                Box::new(right),
                operator.location,
            );
        }

        Ok(expr)
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
            TokenKind::Identifier(_) => Ok(Expr::Variable(token.clone(), token.location.clone())),
            TokenKind::LParen => {
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
                self.previous_location(),
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
            None => self.previous_location(),
        }
    }

    fn previous_location(&self) -> SrcLocation {
        self.previous().location
    }
}
