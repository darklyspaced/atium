use super::token::Token;
use std::fmt;

/// The base building blocks of the language
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

/// An expression: something that can be evalutated and always produces a result
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary(left, op, right) => {
                write!(f, "({left} {op} {right})")
            }
            Self::Unary(op, expr) => write!(f, "({op}{expr})"),
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::Grouping(expr) => write!(f, "([{expr}])"),
        }
    }
}
