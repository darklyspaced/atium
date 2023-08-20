use super::token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The base building blocks of the language
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        name: Token,
        initialiser: Option<Expr>,
    },
}

/// An expression: something that can be evalutated and always produces a result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary(left, op, right) => {
                write!(f, "({left} {op} {right})")
            }
            Self::Unary(op, expr) => write!(f, "({op}{expr})"),
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::Grouping(expr) => write!(f, "[{expr}]"),
            Self::Variable(tok) => write!(f, "{tok}"),
        }
    }
}
