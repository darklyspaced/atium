use serde::{Deserialize, Serialize};

use std::{
    fmt,
    fmt::Display,
    hash::{Hash, Hasher},
};

pub use self::{r#type::Type, value::Value};
use crate::error::Span;

pub mod r#type;
pub mod value;

/// A token
#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
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

/// Only lexeme and kind must be used in hash function. Span is different for each token as they
/// all appear in different places in source.
impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.lex().hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.lex() == other.lex()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
