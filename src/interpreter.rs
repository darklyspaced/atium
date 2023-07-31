use super::ast::{Expr, Stmt};
use super::error::RuntimeError;
use super::token::{TokenType, Type, Value};

use color_eyre::{Report, Result};

/// Macro that checks that the arguments to an operator are correct
///
/// `$op_token`: the [`Token`] of the operator
/// `$prov_value`: the [`Value`] that the operation is being performed on
/// `$valid_type`: the list of valid [`Value`] that the operation _could_ be performed on
macro_rules! type_check {
    ($op_token:expr, $prov_value:expr, $($valid_op:path),+; $($valid_value:path),+; $($valid_type:expr),+; $($op:tt),+) => {{
        match $op_token.token_type {
            $($valid_op => match $prov_value {
                $valid_value(a) => Ok($valid_value(a)),
                _ => Err(RuntimeError::InvalidType::<Type>($prov_value.into(), vec![$valid_type]).into())
            }),+
            _ => Err(RuntimeError::InvalidOperator($op_token.lexeme, vec![stringify!($($op),+)]).into()),
        }
    }};
}

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

            type_check!(
               op, expr,
               TokenType::Minus, TokenType::Bang;
               Value::Integer, Value::Boolean;
               Type::Integer, Type::Boolean;
               -, !
            )
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
