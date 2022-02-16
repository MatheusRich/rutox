use super::ast::*;
use super::visitor::ExprVisitor;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(expr: &Expr) {
        println!("{}", Self {}.visit_expr(expr));
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&self, literal: &LiteralData) -> String {
        match literal {
            LiteralData::String(s, _) => format!("\"{s}\""),
            LiteralData::Number(n, _) => format!("{n}"),
            LiteralData::Bool(b, _) => format!("{}", b),
            LiteralData::Nil(_) => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&self, unary: &UnaryData) -> String {
        format!("({} {})", unary.operator, self.visit_expr(&unary.expr))
    }

    fn visit_binary_expr(&self, binary: &BinaryData) -> String {
        format!(
            "({} {} {})",
            binary.operator,
            self.visit_expr(&binary.left),
            self.visit_expr(&binary.right)
        )
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        format!(
            "(group {} )",
            self.visit_expr(expr)
        )
    }
}
