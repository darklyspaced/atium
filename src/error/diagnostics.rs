use colored::Colorize;
use serde::{Deserialize, Serialize};

use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Line(pub u32);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Column(pub u32);

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Span {
    pub line: Line,
    pub column: Column,
    pub file: Option<PathBuf>,
    pub lex: String,
}

impl Span {
    pub fn to_snippet() {}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbgSpan {
    pub file: String,
    pub line: Line,
    pub column: Column,
}

impl DbgSpan {
    pub fn new(file: &str, line: u32, column: u32) -> Self {
        Self {
            file: file.to_string(),
            line: Line(line),
            column: Column(column),
        }
    }
}

impl Display for DbgSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("[{}:{}:{}]", self.file, self.line, self.column)
                .blue()
                .bold()
        )
    }
}
