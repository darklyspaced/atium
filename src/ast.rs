use super::token::{Token, TokenType, Type};
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

pub fn compute(expr: Expr) -> Type {
    match expr {
        Expr::Literal(lit) => lit.literal.unwrap(),
        Expr::Grouping(expr) => compute(*expr),
        Expr::Unary(op, expr) => {
            let expr = compute(*expr);

            match op.token_type {
                TokenType::Minus => {
                    if let Type::Integer(a) = expr {
                        return Type::Integer(-a);
                    }
                    unreachable!()
                }
                TokenType::Bang => {
                    if let Type::Boolean(a) = expr {
                        return Type::Boolean(!a);
                    }
                    unreachable!()
                }
                _ => panic!("can only use - or ! as unary operators"),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = compute(*left);
            let right = compute(*right);

            match op.token_type {
                TokenType::Slash => {
                    if let (Type::Integer(a), Type::Integer(b)) = (left, right) {
                        return Type::Integer(a / b);
                    }
                    unreachable!()
                }
                TokenType::Minus => {
                    if let (Type::Integer(a), Type::Integer(b)) = (left, right) {
                        return Type::Integer(a - b);
                    }
                    unreachable!()
                }
                TokenType::Star => {
                    if let (Type::Integer(a), Type::Integer(b)) = (left, right) {
                        return Type::Integer(a * b);
                    }
                    unreachable!()
                }
                TokenType::Plus => {
                    if let (Type::Integer(a), Type::Integer(b)) = (&left, &right) {
                        return Type::Integer(a + b);
                    } else {
                        if let (Type::String(a), Type::String(b)) = (&left, &right) {
                            return Type::String(a.clone() + b);
                        }
                        unreachable!()
                    }
                }
                _ => panic!("operator other than +, /, - or * found in binary expression"),
            }
        }
    }
}
