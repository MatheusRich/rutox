use super::src_location::SrcLocation;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: SrcLocation,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} at {}>", self.kind, self.location)
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Single-character tokens.
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
