mod env;
mod lox_obj;
use crate::parser::{
    ast::{BinaryData, BinaryOp, Expr, LiteralData, Stmt, UnaryData, UnaryOp},
    visitors::{ExprVisitor, StmtVisitor},
};
use crate::rutox_error::RutoxError;
use crate::scanner::{token::Token, SrcLocation};
pub use env::Env;
pub use lox_obj::LoxObj;
use std::cmp::Ordering;

pub struct Interpreter {
    env: Env,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_print_stmt(&self, expr: &Expr, _location: &SrcLocation) -> Result<(), RutoxError> {
        let value = self.visit_expr(expr)?;
        println!("{value}");

        Ok(())
    }

    fn visit_expr_stmt(&self, expr: &Expr, _location: &SrcLocation) -> Result<(), RutoxError> {
        self.visit_expr(expr)?;

        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
        location: &SrcLocation,
    ) -> Result<(), RutoxError> {
        let mut value = LoxObj::Nil(location.clone());

        if let Some(initial_val) = initializer {
            value = self.visit_expr(initial_val)?;
        }

        self.env.define(&name.lexeme, value);

        Ok(())
    }
}

impl ExprVisitor<LoxObj> for Interpreter {
    fn visit_literal_expr(&self, literal: &LiteralData) -> Result<LoxObj, RutoxError> {
        Ok(literal.clone().into())
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
        match &binary.operator {
            BinaryOp::EqualEqual(location) => Ok(LoxObj::Bool(
                self.is_equal(
                    self.visit_expr(&binary.left)?,
                    self.visit_expr(&binary.right)?,
                ),
                location.clone(),
            )),
            BinaryOp::BangEqual(location) => Ok(LoxObj::Bool(
                !self.is_equal(
                    self.visit_expr(&binary.left)?,
                    self.visit_expr(&binary.right)?,
                ),
                location.clone(),
            )),
            BinaryOp::Greater(location)
            | BinaryOp::Less(location)
            | BinaryOp::GreaterEqual(location)
            | BinaryOp::LessEqual(location) => {
                let a = &self.visit_expr(&binary.left)?;
                let b = &self.visit_expr(&binary.right)?;
                let ordering = self.compare(a, b).ok_or_else(|| {
                    RutoxError::Runtime(
                        format!("Cannot compare {:?} and {:?}", a, b),
                        location.clone(),
                    )
                })?;

                match &binary.operator {
                    BinaryOp::Greater(_) => Ok(LoxObj::Bool(
                        ordering == Ordering::Greater,
                        location.clone(),
                    )),
                    BinaryOp::GreaterEqual(_) => {
                        Ok(LoxObj::Bool(ordering != Ordering::Less, location.clone()))
                    }
                    BinaryOp::Less(_) => {
                        Ok(LoxObj::Bool(ordering == Ordering::Less, location.clone()))
                    }
                    BinaryOp::LessEqual(_) => Ok(LoxObj::Bool(
                        ordering != Ordering::Greater,
                        location.clone(),
                    )),
                    _ => panic!("Unreachable"),
                }
            }
            BinaryOp::Plus(location) => {
                let a = &self.visit_expr(&binary.left)?;
                let b = &self.visit_expr(&binary.right)?;

                match (a, b) {
                    (LoxObj::Number(a, _), LoxObj::Number(b, _)) => {
                        Ok(LoxObj::Number(a + b, location.clone()))
                    }
                    (LoxObj::String(s1, _), LoxObj::String(s2, _)) => {
                        Ok(LoxObj::String(format!("{}{}", s1, s2), location.clone()))
                    }
                    _ => Err(RutoxError::Runtime(
                        format!("Cannot compare {:?} and {:?}", a, b),
                        location.clone(),
                    )),
                }
            }
            BinaryOp::Minus(location) => {
                let a = &self.visit_expr(&binary.left)?;
                let b = &self.visit_expr(&binary.right)?;

                match (a, b) {
                    (LoxObj::Number(a, _), LoxObj::Number(b, _)) => {
                        Ok(LoxObj::Number(a - b, location.clone()))
                    }
                    _ => Err(RutoxError::Runtime(
                        format!("Cannot subtract {:?} and {:?}", a, b),
                        location.clone(),
                    )),
                }
            }
            BinaryOp::Div(location) => {
                let a = &self.visit_expr(&binary.left)?;
                let b = &self.visit_expr(&binary.right)?;

                match (a, b) {
                    (LoxObj::Number(a, _), LoxObj::Number(b, _)) => {
                        Ok(LoxObj::Number(a / b, location.clone()))
                    }
                    _ => Err(RutoxError::Runtime(
                        format!("Cannot divide {:?} and {:?}", a, b),
                        location.clone(),
                    )),
                }
            }
            BinaryOp::Mul(location) => {
                let a = &self.visit_expr(&binary.left)?;
                let b = &self.visit_expr(&binary.right)?;

                match (a, b) {
                    (LoxObj::Number(a, _), LoxObj::Number(b, _)) => {
                        Ok(LoxObj::Number(a * b, location.clone()))
                    }
                    (LoxObj::String(s, _), LoxObj::Number(times, _)) => Ok(LoxObj::String(
                        s.repeat((*times) as usize),
                        location.clone(),
                    )),
                    _ => Err(RutoxError::Runtime(
                        format!("Cannot multiply {:?} and {:?}", a, b),
                        location.clone(),
                    )),
                }
            }
        }
    }

    fn visit_variable_expr(
        &self,
        name: &Token,
        location: &SrcLocation,
    ) -> Result<LoxObj, RutoxError> {
        match self.env.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RutoxError::Runtime(
                format!("Undefined variable `{}`", name.lexeme),
                location.clone(),
            )),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Env::new() }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RutoxError> {
        for stmt in stmts {
            self.visit_stmt(&stmt)?;
        }

        Ok(())
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

    fn compare(&self, a: &LoxObj, b: &LoxObj) -> Option<Ordering> {
        match (a, b) {
            (LoxObj::Number(a, _), LoxObj::Number(b, _)) => a.partial_cmp(b),
            (LoxObj::String(a, _), LoxObj::String(b, _)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
