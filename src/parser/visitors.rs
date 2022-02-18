use super::ast::*;
use crate::rutox_error::RutoxError;

pub trait ExprVisitor<T> {
    fn visit_expr(&self, expr: &Expr) -> Result<T, RutoxError> {
        match expr {
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(args) => self.visit_unary_expr(args),
            Expr::Binary(args) => self.visit_binary_expr(args),
            Expr::Grouping(grouped_expr, _) => self.visit_grouping_expr(grouped_expr),
        }
    }

    fn visit_literal_expr(&self, literal: &LiteralData) -> Result<T, RutoxError>;
    fn visit_unary_expr(&self, unary: &UnaryData) -> Result<T, RutoxError>;
    fn visit_binary_expr(&self, binary: &BinaryData) -> Result<T, RutoxError>;
    fn visit_grouping_expr(&self, expr: &Expr) -> Result<T, RutoxError>;
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&self, stmt: &Stmt) -> Result<T, RutoxError> {
        match stmt {
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Expr(expr) => self.visit_expr_stmt(expr),
        }
    }

    fn visit_print_stmt(&self, expr: &Expr) -> Result<T, RutoxError>;
    fn visit_expr_stmt(&self, expr: &Expr) -> Result<T, RutoxError>;
}
