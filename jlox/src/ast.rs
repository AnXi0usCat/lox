use std::fmt;

use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Nequals,
    Gthen,
    Lthen,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Equals => write!(f, "=="),
            Self::Nequals => write!(f, "!="),
            Self::Gthen => write!(f, ">"),
            Self::Lthen => write!(f, "<"),
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
    Binary(Box<Expression<'a>>, Token<'a>, Box<Expression<'a>>),
    Unary(Token<'a>, Box<Expression<'a>>),
    Grouping(Box<Expression<'a>>),
    NumberLiteral(f64),
    StringLiteral(String),
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Self::Binary(exp1, op, exp2) => write!(f, "({} {} {})", String::from_utf8_lossy(op.lexeme), exp1, exp2),
            Self::Unary(op, exp) => write!(f, "({} {})", String::from_utf8_lossy(op.lexeme), exp),
            Self::Grouping(exp) => write!(f, "(group {})", exp),
            Self::NumberLiteral(num) => write!(f, "{}", num),
            Self::StringLiteral(string) => write!(f, "\"{}\"", string),
        };
        Ok(())
    }
}
