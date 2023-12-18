use color_eyre::{Report, Result};
use std::cell::RefCell;

use crate::{
    ast::{Expr, Stmt},
    dump,
    environment::Env,
    error::RuntimeError,
    token::{Token, TokenKind, Type, Value},
};

pub(super) struct Interpreter {
    stmts: Vec<Stmt>,
    env: RefCell<Env>,
}

impl Interpreter {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self {
            stmts,
            env: RefCell::new(Env::new()),
        }
    }

    pub fn interpret(self) -> Result<(), Vec<Report>> {
        let errors = self
            .stmts
            .iter()
            .map(|stmt| self.execute(stmt).err())
            .flatten() // only statements that produces errors
            .flatten() // flatten all errors into one stream
            .flatten() // gets only errors, ignoring successes
            .collect::<Vec<Report>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), Vec<Option<Report>>> {
        let errors = match stmt {
            Stmt::Expr(expr) => vec![self.expression(expr).err()],
            Stmt::Block(stmts) => self
                .execute_block(stmts, Env::new())
                .err()
                .map_or(vec![], |v| v),
            Stmt::Print(expr) => vec![self.print(expr).err()],
            Stmt::Var { name, value } => vec![self.def_var(name.clone(), value.clone()).err()],
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn execute_block(&self, stmts: &[Stmt], new_env: Env) -> Result<(), Vec<Option<Report>>> {
        let prev_env = self.env.replace(new_env);
        self.env.borrow_mut().set_parent(prev_env.clone());

        let errors = stmts
            .iter()
            .map(|stmt| self.execute(stmt).err())
            .flatten() // only statements that produce errors
            .flatten() // Item: Vec<Option<Report>> -> Option<Report>
            .collect::<Vec<Option<Report>>>();

        self.env.replace(prev_env);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn get_var(&self, ident: &Token) -> Result<Value> {
        match self.env.borrow_mut().get(ident) {
            Some(val) => match val {
                Some(val) => Ok(val.clone()),
                None => dump!(RuntimeError::UninitialisedVar(ident.lex())),
            },
            None => dump!(RuntimeError::InvalidIdent(ident.lex())),
        }
    }

    fn def_var(&self, ident: Token, value: Option<Expr>) -> Result<()> {
        if let Some(expr) = value {
            match self.expression(&expr) {
                Ok(val) => {
                    self.env.borrow_mut().define(ident, Some(val));
                    return Ok(());
                }
                Err(err) => Err(err),
            }
        } else {
            self.env.borrow_mut().define(ident, None);
            Ok(())
        }
    }

    /// Interpret and expression, either producing a value or an error than occurred during the
    /// interpretation of the expression.
    fn expression(&self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(lit) => Ok(lit.literal.clone().unwrap()),
            Expr::Grouping(expr) => self.expression(expr),
            Expr::Variable(ident) => self.get_var(ident),
            Expr::Assignment(ident, val) => self
                .env
                .borrow_mut()
                .assign(ident.clone(), self.expression(val)?),
            Expr::Unary(op, expr) => {
                let expr = self.expression(expr)?;

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
                let left = self.expression(left)?;
                let right = self.expression(right)?;

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
        }
    }

    fn print(&self, expr: &Expr) -> Result<()> {
        println!("{}", self.expression(expr)?);
        Ok(())
    }
}
