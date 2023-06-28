use std::fmt::Debug;
use std::fmt::Write;
use thiserror::Error;

use crate::token::Type;

/// Error that can be generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: {0}")]
    UnexpectedCharacter(char),
}
/// Error that is generated during interpretation.
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("an invalid operator was used. found: {0} expected: {}", display_vec(.1))]
    InvalidOperator(String, Vec<char>),
    #[error("an invalid type was found: {0}; expected: {}", display_vec(.1))]
    InvalidType(Type, Vec<Type>),
    #[error("cannot apply {0} to values ({}); expected: {}", display_vec(.1), display_vec(.2))]
    InvalidTypes(String, Vec<Type>, Vec<Type>),
}

fn display_vec<'a, T>(vec: &Vec<T>) -> String
where
    T: Debug,
{
    let mut buffer = String::new();
    write!(buffer, "{:?}", vec).unwrap();
    String::from(&buffer[1..buffer.len() - 2])
}
