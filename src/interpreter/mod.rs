use crate::parser::{
    ast::{BinaryData, Expr, LiteralData, UnaryData, UnaryOp},
    visitor::ExprVisitor,
};
use crate::rutox_error::RutoxError;
use crate::scanner::token::TokenKind;

pub type LoxObj = LiteralData; // consider making this a separate type

pub struct Interpreter {}

impl ExprVisitor<LoxObj> for Interpreter {
    fn visit_literal_expr(&self, literal: &LoxObj) -> Result<LoxObj, RutoxError> {
        Ok(literal.clone())
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<LoxObj, RutoxError> {
        self.visit_expr(expr)
    }

    fn visit_unary_expr(&self, unary: &UnaryData) -> Result<LoxObj, RutoxError> {
        match &unary.operator {
            UnaryOp::Bang(location) => Ok(LoxObj::Bool(
                !self.is_truthy(self.visit_expr(&unary.expr)?),
                location.clone(),
            )),
            UnaryOp::Minus(location) => {
                let value = self.visit_expr(&unary.expr)?;

                match value {
                    LoxObj::Number(number, _) => Ok(LoxObj::Number(-number, location.clone())),
                    other => Err(RutoxError::Runtime(
                        format!(
                            "Unary operator `-` can only be applied to numbers, but got {other}"
                        ),
                        location.clone(),
                    )),
                }
            }
        }
    }

    fn visit_binary_expr(&self, binary: &BinaryData) -> Result<LoxObj, RutoxError> {
        match binary.operator.kind {
            TokenKind::EqualEqual => Ok(LoxObj::Bool(
                self.is_equal(
                    self.visit_expr(&binary.left)?,
                    self.visit_expr(&binary.right)?,
                ),
                binary.operator.location.clone(),
            )),
            TokenKind::BangEqual => Ok(LoxObj::Bool(
                !self.is_equal(
                    self.visit_expr(&binary.left)?,
                    self.visit_expr(&binary.right)?,
                ),
                binary.operator.location.clone(),
            )),
            _ => Err(RutoxError::Programmer(
                format!("Unknown binary operator `{}`", binary.operator.kind),
                binary.operator.location.clone(),
            )),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expr: &Expr) -> Result<LoxObj, RutoxError> {
        self.visit_expr(expr)
    }

    fn is_truthy(&self, obj: LoxObj) -> bool {
        !matches!(obj, LoxObj::Bool(false, _) | LoxObj::Nil(_))
    }

    fn is_equal(&self, a: LoxObj, b: LoxObj) -> bool {
        match (a, b) {
            (LoxObj::Nil(_), LoxObj::Nil(_)) => true,
            (LoxObj::Bool(b1, _), LoxObj::Bool(b2, _)) => b1 == b2,
            (LoxObj::Number(n1, _), LoxObj::Number(n2, _)) => n1 == n2,
            (LoxObj::String(s1, _), LoxObj::String(s2, _)) => s1 == s2,
            _ => false,
        }
    }
}
