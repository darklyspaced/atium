use super::token::Token;
use std::fmt;

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

pub enum Stmt {}
