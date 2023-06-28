use std::{fmt, fmt::Display};
#[derive(Clone, Debug, PartialEq)]

/// A token
pub struct Token {
    /// The type of token represented by the struct whether that be a STRING or a CLASS
    pub token_type: TokenType,
    /// The textual representation of the `Token`: "cat"
    pub lexeme: String,
    /// The value represented by a literal: cat
    pub literal: Option<Type>,
    /// The line that the token is on
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    String(String),
    Integer(f64),
    Boolean(bool),
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Type>, line: usize) -> Self {
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

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    Str,
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

    EOF,
}
