use thiserror::Error;

use crate::token::Type;
use std::{
    error::Error,
    fmt,
    fmt::{Debug, Write},
};

#[macro_export]
macro_rules! dump {
    ($kind:expr) => {{
        Err($crate::error::AError {
            kind: $kind,
            #[cfg(debug_assertions)]
            dbg_span: $crate::error::DbgSpan::new(::std::file!(), ::std::line!(), ::std::column!()),
            span: $crate::error::Span {
                lo: 0,
                hi: 0,
            } // placeholder until Span is in lexer and is propagated
        }.into())
    }}
}

impl<E> fmt::Display for AError<E>
where
    E: Error,
{
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}:{}] {}",
            self.dbg_span.file, self.dbg_span.line, self.dbg_span.column, self.kind
        )
    }
    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Error, Debug)]
pub struct AError<E>
where
    E: Error,
{
    #[source]
    pub kind: E,
    /// The line and column where the error was generated in the _compiler_
    #[cfg(debug_assertions)]
    pub dbg_span: DbgSpan,
    /// Information about where the error originates in _source code_
    pub span: Span,
}

#[derive(Debug)]
pub struct Span {
    // Refers to actual byte positions so its easier to work with UTF-8 and non UTF-8
    pub lo: u16,
    pub hi: u16,
}

#[derive(Debug)]
pub struct DbgSpan {
    file: String,
    line: u32,
    column: u32,
}

impl DbgSpan {
    pub fn new(file: &str, line: u32, column: u32) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
    }
}

/// Error that is generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: '{0}'")]
    UnexpectedCharacter(char),

    #[error("expected: '{expected}' | found: '{found}'")]
    ExpectedCharacter { found: String, expected: char },

    #[error("expected: identifier | found: {0}")]
    ExpectedIdent(String),

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
