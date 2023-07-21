use super::ast::{Expr, Stmt};
use super::error::RuntimeError;
use super::token::{TokenType, Type, Value};

use color_eyre::Result;

pub fn interpret(stmts: Vec<Stmt>) -> Vec<color_eyre::Report> {
    let mut errors = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Expr(expr) => errors.push(expression(expr).err()),
            Stmt::Print(expr) => errors.push(print(expr).err()),
        };
    }
    errors.into_iter().flatten().collect()
}

fn expression(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Literal(lit) => Ok(lit.literal.unwrap()),
        Expr::Grouping(expr) => expression(*expr),
        Expr::Unary(op, expr) => {
            let expr = expression(*expr)?;

            match op.token_type {
                TokenType::Minus => match expr {
                    Value::Integer(a) => Ok(Value::Integer(-a)),
                    _ => Err(RuntimeError::InvalidType(expr.into(), vec![Type::Integer]).into()),
                },
                TokenType::Bang => match expr {
                    Value::Boolean(a) => Ok(Value::Boolean(!a)),
                    _ => Err(RuntimeError::InvalidType(expr.into(), vec![Type::Boolean]).into()),
                },
                _ => Err(RuntimeError::InvalidOperator(op.lexeme, vec!['-', '!']).into()),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = expression(*left)?;
            let right = expression(*right)?;

            match op.token_type {
                TokenType::Slash => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Minus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Star => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Plus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer), (Type::String, Type::String)],
                    )
                    .into()),
                },
                _ => Err(RuntimeError::InvalidOperator(op.lexeme, vec!['+', '/', '-', '*']).into()),
            }
        }
    }
}

fn print(expr: Expr) -> Result<()> {
    println!("{}", expression(expr)?);
    Ok(())
}
