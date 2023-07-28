use std::{fmt, fmt::Display};

/// A token
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The type of token represented by the struct whether that be a STRING or a CLASS
    pub token_type: TokenType,
    /// The textual representation of the `Token`: "cat"
    pub lexeme: String,
    /// The value represented by a literal: cat
    pub literal: Option<Value>,
    /// The line that the token is on
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

// TODO: make a macro that does all the generation so that you don't have the wrap the values
// anymore
impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Integer(i64::from(value))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String,
    Integer,
    Float,
    Boolean,
    Null,
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => Self::String,
            Value::Integer(_) => Self::Integer,
            Value::Float(_) => Self::Float,
            Value::Boolean(_) => Self::Boolean,
            Value::Null => Self::Null,
        }
    }
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

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(a) => write!(f, "{a}"),
            Self::Integer(a) => write!(f, "{a}"),
            Self::Float(a) => write!(f, "{a}"),
            Self::Boolean(a) => write!(f, "{a}"),
            Self::Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
}
