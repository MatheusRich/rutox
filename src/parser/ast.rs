use crate::scanner::{token::Token, SrcLocation};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryData),
    Grouping(Box<Expr>),
    Unary(UnaryData),
    Literal(LiteralData),
}

#[derive(Debug)]
pub enum LiteralData {
    String(String, SrcLocation),
    Number(f64, SrcLocation),
    Bool(bool, SrcLocation),
    Nil(SrcLocation),
}

#[derive(Debug)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryData {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
