use crate::parser::ast::LiteralData;
use crate::scanner::src_location::SrcLocation;

// Why should lox objs have a location?
#[derive(Clone, PartialEq)]
pub enum LoxObj {
    String(String, SrcLocation),
    Number(f64, SrcLocation),
    Bool(bool, SrcLocation),
    Nil(SrcLocation),
}

impl From<LiteralData> for LoxObj {
    fn from(literal: LiteralData) -> Self {
        match literal {
            LiteralData::String(string, location) => LoxObj::String(string, location),
            LiteralData::Number(n, location) => LoxObj::Number(n, location),
            LiteralData::Bool(b, location) => LoxObj::Bool(b, location),
            LiteralData::Nil(location) => LoxObj::Nil(location),
        }
    }
}

impl std::fmt::Display for LoxObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxObj::String(s, _) => write!(f, "{s}"),
            LoxObj::Number(n, _) => write!(f, "{n}"),
            LoxObj::Bool(bool, _) => write!(f, "{bool}"),
            LoxObj::Nil(_) => write!(f, "nil"),
        }
    }
}

impl std::fmt::Debug for LoxObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxObj::String(s, _) => write!(f, "string \"{s}\""),
            LoxObj::Number(n, _) => write!(f, "number {n}"),
            LoxObj::Bool(bool, _) => write!(f, "boolean {bool}"),
            LoxObj::Nil(_) => write!(f, "nil"),
        }
    }
}

use colored::*;
impl LoxObj {
    pub fn as_colored_string(&self) -> ColoredString {
        match self {
            LoxObj::String(_, _) => format!("{self}").green(),
            LoxObj::Number(_, _) => format!("{self}").blue().bold(),
            LoxObj::Bool(_, _) => format!("{self}").cyan().bold(),
            LoxObj::Nil(_) => format!("{self}").cyan().bold(),
        }
    }
}
