use super::ast::*;
use crate::{
    rutox_error::RutoxError,
    scanner::{token::Token, SrcLocation},
};

pub trait ExprVisitor<T> {
    fn visit_expr(&self, expr: &Expr) -> Result<T, RutoxError> {
        match expr {
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(args) => self.visit_unary_expr(args),
            Expr::Binary(args) => self.visit_binary_expr(args),
            Expr::Grouping(grouped_expr, _) => self.visit_grouping_expr(grouped_expr),
            Expr::Variable(name, location) => self.visit_variable_expr(name, location),
        }
    }

    fn visit_literal_expr(&self, literal: &LiteralData) -> Result<T, RutoxError>;
    fn visit_unary_expr(&self, unary: &UnaryData) -> Result<T, RutoxError>;
    fn visit_binary_expr(&self, binary: &BinaryData) -> Result<T, RutoxError>;
    fn visit_grouping_expr(&self, expr: &Expr) -> Result<T, RutoxError>;
    fn visit_variable_expr(&self, name: &Token, location: &SrcLocation) -> Result<T, RutoxError>;
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<T, RutoxError> {
        match stmt {
            Stmt::Print(expr, location) => self.visit_print_stmt(expr, location),
            Stmt::Expr(expr, location) => self.visit_expr_stmt(expr, location),
            Stmt::Var(name, initializer, location) => {
                self.visit_var_stmt(name, initializer, location)
            }
        }
    }

    fn visit_print_stmt(&self, expr: &Expr, location: &SrcLocation) -> Result<T, RutoxError>;
    fn visit_expr_stmt(&self, expr: &Expr, location: &SrcLocation) -> Result<T, RutoxError>;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>, location: &SrcLocation) -> Result<T, RutoxError>;
}
