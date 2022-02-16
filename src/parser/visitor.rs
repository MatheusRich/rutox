use super::ast::*;

pub trait ExprVisitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T {
        match expr {
            Expr::Literal(literal) => self.visit_literal_expr(literal),
            Expr::Unary(args) => self.visit_unary_expr(args),
            Expr::Binary(args) => self.visit_binary_expr(args),
            Expr::Grouping(grouped_expr) => self.visit_grouping_expr(grouped_expr),
        }
    }

    fn visit_literal_expr(&self, literal: &LiteralData) -> T;
    fn visit_unary_expr(&self, unary: &UnaryData) -> T;
    fn visit_binary_expr(&self, binary: &BinaryData) -> T;
    fn visit_grouping_expr(&self, expr: &Expr) -> T;
}
