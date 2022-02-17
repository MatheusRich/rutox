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

impl std::fmt::Display for LiteralData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LiteralData::String(s, _) =>  write!(f, "string \"{s}\""),
            LiteralData::Number(n, _) =>  write!(f, "number {n}"),
            LiteralData::Bool(bool, _) =>  write!(f, "boolean {bool}"),
            LiteralData::Nil(_) =>  write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct UnaryData {
    pub operator: UnaryOp,
    pub expr: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug)]
pub enum UnaryOp {
    Bang(SrcLocation),
    Minus(SrcLocation),
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOp::Bang(_) => write!(f, "!"),
            UnaryOp::Minus(_) => write!(f, "-"),
        }
    }
}

#[derive(Debug)]
pub struct BinaryData {
    pub operator: Token, // restrict further
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub location: SrcLocation,
}
