use color_eyre::{Report, Result};

use crate::{
    ast::{Expr, Stmt},
    dump,
    error::RuntimeError,
    token::{TokenKind, Type, Value},
};

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
                _ => unimplemented!(),
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

            match op.kind {
                TokenKind::Minus => match expr {
                    Value::Integer(a) => Ok(Value::Integer(-a)),
                    _ => dump!(RuntimeError::InvalidType::<&str>(
                        expr.into(),
                        vec![Type::Integer]
                    )),
                },
                TokenKind::Bang => match expr {
                    Value::Boolean(a) => Ok(Value::Boolean(!a)),
                    _ => dump!(RuntimeError::InvalidType::<&str>(
                        expr.into(),
                        vec![Type::Boolean]
                    )),
                },
                _ => dump!(RuntimeError::InvalidOperator(op.lex(), vec!['-', '!'])),
            }
        }
        Expr::Binary(left, op, right) => {
            let left = expression(*left)?;
            let right = expression(*right)?;

            match op.kind {
                TokenKind::Slash => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a / b).into()),
                    _ => dump!(RuntimeError::InvalidTypes(
                        op.lex(),
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )),
                },
                TokenKind::Minus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a - b).into()),
                    _ => dump!(RuntimeError::InvalidTypes(
                        op.lex(),
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )),
                },
                TokenKind::Star => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a * b).into()),
                    _ => dump!(RuntimeError::InvalidTypes(
                        op.lex(),
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer)],
                    )),
                },
                TokenKind::Plus => match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => Ok((a + b).into()),
                    (Value::String(a), Value::String(b)) => Ok(format!("{a}{b}").into()),
                    _ => dump!(RuntimeError::InvalidTypes(
                        op.lex(),
                        vec![left.into(), right.into()],
                        vec![(Type::Integer, Type::Integer), (Type::String, Type::String)],
                    )),
                },
                _ => dump!(RuntimeError::InvalidOperator(
                    op.lex(),
                    vec!['+', '/', '-', '*']
                )),
            }
        }
        _ => unimplemented!(),
    }
}

fn print(expr: Expr) -> Result<()> {
    println!("{}", expression(expr)?);
    Ok(())
}
