use thiserror::Error;

use crate::token::Type;
use std::{
    fmt,
    fmt::{Debug, Write},
};

/// Error that is generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: '{0}'")]
    UnexpectedCharacter(char),

    #[error("expected: '{1}' | found: '{0}'")]
    ExpectedCharacter(String, char),

    #[error("expected an expression, or a statement. idfk, you figure it out")]
    NoExpression,

    /// EOF was found in an unexpected place. don't know what was expected instead of it
    #[error("unexpected EOF found")]
    UnexpectedEOF,
}
/// Error that is generated during interpretation.
#[derive(Error, Debug)]
pub enum RuntimeError<D: Debug> {
    #[error("an invalid operator was used. found: {0} | expected: {}", display_vec(.1))]
    InvalidOperator(String, Vec<D>),

    #[error("an invalid type was found: {0} | expected: {}", display_vec(.1))]
    InvalidType(Type, Vec<Type>),

    #[error("cannot apply '{0}' to values ({}) | expected: {}", display_vec(.1), display_tuple_vec(.2))]
    InvalidTypes(D, Vec<Type>, Vec<(Type, Type)>),
}

fn display_vec<T: fmt::Debug>(vec: &[T]) -> String {
    let mut buffer = String::new();
    write!(&mut buffer, "{vec:?}").unwrap();
    String::from(&buffer[1..buffer.len() - 1])
}

fn display_tuple_vec<T: fmt::Debug>(vec: &[(T, T)]) -> String {
    let mut buffer = format!("{:?}", vec[0]);
    for tup in vec.iter().skip(1) {
        buffer.push_str(&format!(" or {tup:?}"));
    }
    buffer
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Float => write!(f, "Float"),
            Self::Null => write!(f, "Null"),
        }
    }
}
