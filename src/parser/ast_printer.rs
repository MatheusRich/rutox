use super::ast::*;
use super::visitor::ExprVisitor;
use crate::rutox_error::RutoxError;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(expr: &Expr) {
        println!("{}", Self {}.visit_expr(expr).ok().unwrap());
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_literal_expr(&self, literal: &LiteralData) -> Result<String, RutoxError> {
        match literal {
            LiteralData::String(s, _) => Ok(format!("\"{s}\"")),
            LiteralData::Number(n, _) => Ok(format!("{n}")),
            LiteralData::Bool(b, _) => Ok(format!("{}", b)),
            LiteralData::Nil(_) => Ok("nil".to_string()),
        }
    }

    fn visit_unary_expr(&self, unary: &UnaryData) -> Result<String, RutoxError> {
        Ok(format!(
            "({} {})",
            unary.operator,
            self.visit_expr(&unary.expr)?
        ))
    }

    fn visit_binary_expr(&self, binary: &BinaryData) -> Result<String, RutoxError> {
        Ok(format!(
            "({} {} {})",
            binary.operator,
            self.visit_expr(&binary.left)?,
            self.visit_expr(&binary.right)?,
        ))
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<String, RutoxError> {
        Ok(format!("(group {} )", self.visit_expr(expr)?))
    }
}
