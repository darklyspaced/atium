use super::ast::Expr;
use super::error::RuntimeError;
use super::token::{TokenType, Type};

use color_eyre::Result;

pub fn interpret(expr: Expr) -> Result<Type> {
    match expr {
        Expr::Literal(lit) => Ok(lit.literal.unwrap()),
        Expr::Grouping(expr) => compute(*expr),
        Expr::Unary(op, expr) => {
            let expr = compute(*expr)?;

            match op.token_type {
                TokenType::Minus => match expr {
                    Type::Integer(a) => Ok(Type::Integer(-a)),
                    _ => Err(RuntimeError::InvalidType(expr, vec![Type::Integer(0.0)]).into()),
                },
                TokenType::Bang => match expr {
                    Type::Boolean(a) => Ok(Type::Boolean(!a)),
                    _ => Err(RuntimeError::InvalidType(expr, vec![Type::Boolean(false)]).into()),
                },
                _ => Err(RuntimeError::InvalidOperator(op.lexeme, vec!['-', '!']).into()),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = compute(*left)?;
            let right = compute(*right)?;

            match op.token_type {
                TokenType::Slash => match (&left, &right) {
                    (Type::Integer(a), Type::Integer(b)) => Ok(Type::Integer(a / b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left, right],
                        vec![Type::Integer(0.0)],
                    )
                    .into()),
                },
                TokenType::Minus => match (&left, &right) {
                    (Type::Integer(a), Type::Integer(b)) => Ok(Type::Integer(a - b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left, right],
                        vec![Type::Integer(0.0)],
                    )
                    .into()),
                },
                TokenType::Star => match (&left, &right) {
                    (Type::Integer(a), Type::Integer(b)) => Ok(Type::Integer(a * b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left, right],
                        vec![Type::Integer(0.0)],
                    )
                    .into()),
                },
                TokenType::Plus => match (&left, &right) {
                    (Type::Integer(a), Type::Integer(b)) => Ok(Type::Integer(a + b)),
                    (Type::String(a), Type::String(b)) => Ok(Type::String(a.to_owned() + b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left, right],
                        vec![Type::Integer(0.0)],
                    )
                    .into()),
                },
                _ => Err(RuntimeError::InvalidOperator(op.lexeme, vec!['+', '/', '-', '*']).into()),
            }
        }
    }
}
