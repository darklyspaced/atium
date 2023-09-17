use serde::{Deserialize, Serialize};

use std::{fmt, fmt::Display};

pub use self::{r#type::Type, value::Value};
use crate::error::Span;

pub mod r#type;
pub mod value;

/// A token
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    /// The type of token
    pub kind: TokenKind,
    /// The value of the token
    pub literal: Option<Value>,
    /// The span of the token
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, literal: Option<Value>, span: Span) -> Self {
        Self {
            kind,
            literal,
            span,
        }
    }

    /// Returns the lexeme of the token
    pub fn lex(&self) -> String {
        self.span.lex.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.span.lex)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
