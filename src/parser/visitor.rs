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
