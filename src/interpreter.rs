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
        let mut errors = Vec::new();
        for stmt in &self.stmts {
            match stmt {
                // TODO stop throwing standalone `Expr` into the void and actually _do_ something
                // with them
                Stmt::Expr(expr) => errors.push(self.expression(expr).err()),
                Stmt::Print(expr) => errors.push(self.print(expr).err()),
                Stmt::Var { name, value } => {
                    if let Some(expr) = value {
                        match self.expression(expr) {
                            // NOTE notice how this allows variable shadowing as we don't check if
                            // variables already exist before 'defining' them.
                            Ok(val) => self.def_var(name.to_owned(), Some(val)),
                            Err(e) => errors.push(Some(e)),
                        }
                    } else {
                        self.def_var(name.to_owned(), None)
                    }
                }
            };
        }
        Err(errors.into_iter().flatten().collect())
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

    fn def_var(&self, ident: Token, value: Option<Value>) {
        self.env.borrow_mut().define(ident, value);
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
