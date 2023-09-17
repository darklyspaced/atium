use colored::Colorize;
use thiserror::Error;

use std::{
    error::Error,
    fmt,
    fmt::{Debug, Write},
};

pub use self::diagnostics::{Column, Line, Span};
use crate::token::Type;

pub mod diagnostics;

#[macro_export]
macro_rules! dump {
    ($kind:expr) => {{
        return Err($crate::error::Diagnostic {
            kind: $kind,
            #[cfg(debug_assertions)]
            dbg_span: $crate::error::diagnostics::DbgSpan::new(
                ::std::file!(),
                ::std::line!(),
                ::std::column!(),
            ),
            span: $crate::error::Span {
                line: $crate::error::Line(0),
                column: $crate::error::Column(0),
                file: None,
                lex: String::new(),
            }, // TODO: replace placeholder once Span is impl
        }
        .into());
    }};
}

/// Prints out the diagnostic in the format specified below:
///
/// error: no method `frobnicate` exists for `foo`
///     --> bar.as:26:4
///      |
///   26 | foo.frobnicate();
///      |     ^^^^^^^^^^ methods doesn't exist
///      |
///
/// Or if compiled with debug assertions:
///
/// [src/interpreter:63:12]:
/// error: no method `frobnicate` exists for `foo`
///     --> bar.as:26:4
///      |
///   26 | foo.frobnicate();
///      |     ^^^^^^^^^^ methods doesn't exist
///      |
///
impl<E> fmt::Display for Diagnostic<E>
where
    E: Error,
{
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.dbg_span,
            "error".red().bold(),
            self.kind.to_string().green()
        )
    }

    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Error, Debug)]
pub struct Diagnostic<E>
where
    E: Error,
{
    #[source]
    pub kind: E,
    /// The line and column where the error was generated in the _compiler_
    #[cfg(debug_assertions)]
    pub dbg_span: self::diagnostics::DbgSpan,
    /// Information about where the error originates in _source code_
    pub span: Span,
}

/// Error that is generated during the lexing phase of the interpreter.
#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("an unexpected character was found while lexing: '{0}'")]
    UnexpectedCharacter(char),

    #[error("expected '{expected}' but found '{found}'")]
    ExpectedCharacter { found: String, expected: char },

    #[error("expected: identifier but found: {0}")]
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

    #[error("cannot apply '{0}' to values ({}), expected: {}", display_vec(.1), display_tuple_vec(.2))]
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
