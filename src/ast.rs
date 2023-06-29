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
            Self::Binary(left, op, right) => write!(
                f,
                "({} {} {})",
                (*left).to_string(),
                op.to_string(),
                (*right).to_string()
            ),
            Self::Unary(op, expr) => write!(f, "({}{})", op.to_string(), ((*expr).to_string())),
            Self::Literal(lit) => write!(f, "{}", lit.to_string()),
            Self::Grouping(expr) => write!(f, "([{}])", (*expr).to_string()),
        }
    }
}
