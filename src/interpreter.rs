use super::ast::{Expr, Stmt};
use super::error::RuntimeError;
use super::token::{TokenType, Type, Value};

use color_eyre::{Report, Result};

pub(super) struct Interpreter {
    stmts: Vec<Stmt>,
}

impl Interpreter {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    pub fn interpret(self) -> Result<(), Vec<Report>> {
        let mut errors = Vec::new();
        for stmt in self.stmts {
            match stmt {
                Stmt::Expr(expr) => errors.push(expression(expr).err()),
                Stmt::Print(expr) => errors.push(print(expr).err()),
            };
        }
        Err(errors.into_iter().flatten().collect())
    }
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
                    _ => Err(
                        RuntimeError::InvalidType::<&str>(expr.into(), vec![Type::Integer]).into(),
                    ),
                },
                TokenType::Bang => match expr {
                    Value::Boolean(a) => Ok(Value::Boolean(!a)),
                    _ => Err(
                        RuntimeError::InvalidType::<&str>(expr.into(), vec![Type::Boolean]).into(),
                    ),
                },
                _ => Err(RuntimeError::InvalidOperator(op.lexeme, vec!['-', '!']).into()),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = expression(*left)?;
            let right = expression(*right)?;

            match op.token_type {
                TokenType::Slash => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a / b).into()),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Minus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a - b).into()),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Star => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a * b).into()),
                    _ => Err(RuntimeError::InvalidTypes(
                        op.lexeme,
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )
                    .into()),
                },
                TokenType::Plus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a + b).into()),
                    (Value::String(a), Value::String(b)) => Ok(format!("{a}{b}").into()),
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
