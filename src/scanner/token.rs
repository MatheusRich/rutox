use super::src_location::SrcLocation;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: SrcLocation,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TokenKind::Eof => write!(f, "EOF"),
            _ => write!(f, "{}", self.lexeme),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} at {}>", self.kind, self.location)
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::*;

        match self {
            TokenKind::And
            | TokenKind::Class
            | TokenKind::Else
            | TokenKind::False
            | TokenKind::For
            | TokenKind::Fun
            | TokenKind::If
            | TokenKind::Nil
            | TokenKind::Or
            | TokenKind::Print
            | TokenKind::Return
            | TokenKind::Super
            | TokenKind::This
            | TokenKind::True
            | TokenKind::Var
            | TokenKind::While => {
                let str = format!("{:?}", self).to_lowercase().purple();
                write!(f, "{str}")
            }

            TokenKind::Number(n) => {
                let str = "Number".blue();
                write!(f, "{str}({n})")
            }
            TokenKind::String(s) => {
                let str = "String".green();
                write!(f, "{str}({s})")
            }

            _ => write!(f, "{:?}", self),
        }
    }
}
