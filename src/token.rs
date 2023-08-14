use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Display};

pub mod r#type;
pub mod value;

pub use self::r#type::Type;
pub use self::value::Value;

/// A token
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Token {
    /// The type of token represented by the struct whether that be a [`TokenType::Str`] or a
    /// [`TokenType::Class`]
    pub token_type: TokenType,
    /// The textual representation of the `Token`: "cat"
    pub lexeme: String,
    /// The value represented by a literal: cat
    pub literal: Option<Value>,
    /// The line that the token is on
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Value>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
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
