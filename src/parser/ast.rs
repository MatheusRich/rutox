use super::visitor::ExprVisitor;
use crate::scanner::token::Token;

pub enum Expr {
    Binary(BinaryData),
    Grouping(Box<Expr>),
    Unary(UnaryData),
    Literal(LiteralData),
}

pub enum LiteralData {
    String(String),
    Number(f64),
}

pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

pub struct BinaryData {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
