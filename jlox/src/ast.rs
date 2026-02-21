use std::fmt;


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
pub enum Expression {
    Binary(Box<Expression>, Infix, Box<Expression>),
    Unary(Infix, Box<Expression>),
    Grouping(Box<Expression>),
    NumberLiteral(f64),
    StringLiteral(String)
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Self::Binary(exp1, op, exp2) => write!(f, "({} {} {})", exp1, op, exp2),
            Self::Unary(op, exp) => write!(f, "({} {})", op, exp),
            Self::Grouping(exp) => write!(f, "(group {})", exp),
            Self::NumberLiteral(num) => write!(f, "{}", num),
            Self::StringLiteral(string) => write!(f, "\"{}\"", string),
        };
        Ok(())
    }
}
