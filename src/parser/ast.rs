use crate::scanner::{
    token::{Token, TokenKind},
    SrcLocation,
};

// TODO: add location
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(Expr, SrcLocation),
    Expr(Expr, SrcLocation),
    Var(Token, Option<Expr>, SrcLocation),
    Block(Vec<Stmt>, SrcLocation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(BinaryData),
    Grouping(Box<Expr>, SrcLocation),
    Unary(UnaryData),
    Literal(LiteralData),
    Variable(Token, SrcLocation),
    Assign(Token, Box<Expr>, SrcLocation),
}

impl Expr {
    pub fn location(&self) -> SrcLocation {
        match self {
            Expr::Binary(binary) => binary.location.clone(),
            Expr::Grouping(_, location) => location.clone(),
            Expr::Literal(literal) => literal.location(),
            Expr::Unary(unary) => unary.location.clone(),
            Expr::Variable(_token, location) => location.clone(),
            Expr::Assign(_token, _expr, location) => location.clone(),
        }
    }
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

impl LiteralData {
    pub fn location(&self) -> SrcLocation {
        match self {
            LiteralData::String(_, location) => location.clone(),
            LiteralData::Bool(_, location) => location.clone(),
            LiteralData::Nil(location) => location.clone(),
            LiteralData::Number(_, location) => location.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryData {
    pub operator: UnaryOp,
    pub expr: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryData {
    pub operator: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub location: SrcLocation,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    BangEqual(SrcLocation),
    EqualEqual(SrcLocation),
    Greater(SrcLocation),
    GreaterEqual(SrcLocation),
    Less(SrcLocation),
    LessEqual(SrcLocation),
    Plus(SrcLocation),
    Minus(SrcLocation),
    Div(SrcLocation),
    Mul(SrcLocation),
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
            TokenKind::Plus => BinaryOp::Plus(item.location),
            TokenKind::Minus => BinaryOp::Minus(item.location),
            TokenKind::Slash => BinaryOp::Div(item.location),
            TokenKind::Star => BinaryOp::Mul(item.location),
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
            BinaryOp::Plus(_) => write!(f, "+"),
            BinaryOp::Minus(_) => write!(f, "-"),
            BinaryOp::Div(_) => write!(f, "/"),
            BinaryOp::Mul(_) => write!(f, "*"),
        }
    }
}
