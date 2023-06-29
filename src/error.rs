use thiserror::Error;

use crate::token::Type;
use std::{fmt, fmt::Write};

/// Error that can be generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: {0}")]
    UnexpectedCharacter(char),
    #[error("expected {1}, found {0}")]
    ExpectedCharacter(String, char),
}
/// Error that is generated during interpretation.
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("an invalid operator was used. found: {0} expected: {}", display_vec(.1))]
    InvalidOperator(String, Vec<char>),
    #[error("an invalid type was found: {0}; expected: {}", display_vec(.1))]
    InvalidType(Type, Vec<Type>),
    #[error("cannot apply {0} to values ({}); expected: {}", display_vec(.1), display_tuple_vec(.2))]
    InvalidTypes(String, Vec<Type>, Vec<(Type, Type)>),
}

fn display_vec<'a, T>(vec: &Vec<T>) -> String
where
    T: fmt::Debug,
{
    let mut buffer = String::new();
    write!(&mut buffer, "{:?}", vec).unwrap();
    String::from(&buffer[1..buffer.len() - 1])
}

fn display_tuple_vec<'a, T>(vec: &Vec<(T, T)>) -> String
where
    T: fmt::Debug,
{
    let mut buffer = format!("{:?}", vec[0]);
    for tup in 1..vec.len() {
        buffer.push_str(&format!(" or {:?}", tup))
    }
    buffer
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Integer => write!(f, "Integer"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Null => write!(f, "Null"),
        }
    }
}
