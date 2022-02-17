use crate::scanner::{
    token::{Token, TokenKind},
    SrcLocation,
};

#[derive(Debug, PartialEq)]
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
            LiteralData::String(s, _) => write!(f, "string \"{s}\""),
            LiteralData::Number(n, _) => write!(f, "number {n}"),
            LiteralData::Bool(bool, _) => write!(f, "boolean {bool}"),
            LiteralData::Nil(_) => write!(f, "nil"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryData {
    pub operator: UnaryOp,
    pub expr: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct BinaryData {
    pub operator: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    BangEqual(SrcLocation),
    EqualEqual(SrcLocation),
    Greater(SrcLocation),
    GreaterEqual(SrcLocation),
    Less(SrcLocation),
    LessEqual(SrcLocation),
}

impl From<Token> for BinaryOp {
    fn from(item: Token) -> Self {
        match item.kind {
            TokenKind::BangEqual => BinaryOp::BangEqual(item.location),
            TokenKind::EqualEqual => BinaryOp::EqualEqual(item.location),
            TokenKind::Greater => BinaryOp::Greater(item.location),
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual(item.location),
            TokenKind::Less => BinaryOp::Less(item.location),
            TokenKind::LessEqual => BinaryOp::LessEqual(item.location),
            _ => panic!("Cannot convert `{}` to BinaryOp", item.kind),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinaryOp::BangEqual(_) => write!(f, "!="),
            BinaryOp::EqualEqual(_) => write!(f, "=="),
            BinaryOp::Greater(_) => write!(f, ">"),
            BinaryOp::GreaterEqual(_) => write!(f, ">="),
            BinaryOp::Less(_) => write!(f, "<"),
            BinaryOp::LessEqual(_) => write!(f, "<="),
        }
    }
}
