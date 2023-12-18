use super::token::Token;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The base building blocks of the language
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
    Var { name: Token, value: Option<Expr> },
}

/// An expression: something that can be evaluated to produce a side effect
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
    Assignment(Token, Box<Expr>),
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
            Self::Assignment(tok, expr) => write!(f, "{expr} -> {tok}"),
        }
    }
}
