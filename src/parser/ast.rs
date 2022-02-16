use crate::scanner::{token::Token, SrcLocation};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryData),
    Grouping(Box<Expr>, SrcLocation),
    Unary(UnaryData),
    Literal(LiteralData),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralData {
    String(String, SrcLocation),
    Number(f64, SrcLocation),
    Bool(bool, SrcLocation),
    Nil(SrcLocation),
}

#[derive(Debug)]
pub struct UnaryData {
    pub operator: Token, // restrict further
    pub expr: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug)]
pub struct BinaryData {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub location: SrcLocation,
}
