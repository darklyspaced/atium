use color_eyre::{Report, Result};

use std::{
    collections::HashMap,
    iter::Peekable,
    path::{Path, PathBuf},
    str::Chars,
};

use crate::{
    error::{Column, Line, Span, SyntaxError},
    token::{Token, TokenKind, Value},
};

/// Contains a peekable iterator over a stream of characters (the source code).
///
/// The source code is converted into a stream of tokens.
pub(super) struct Cursor<'a> {
    /// peekable iterator over stream of chars
    iter: Peekable<Chars<'a>>,
    /// tokens present in source code
    tokens: Vec<Token>,
    /// the file that the cursor is iterating over (None == repl)
    file: Option<PathBuf>,
    /// reserved keywords for the language
    reserved: HashMap<String, TokenKind>,
    /// errors present in the source code
    errors: Vec<Report>,
    /// offset from start of file
    offset: u32,
    /// offset of beggining of current line
    line_start: u32,
    /// current line number
    line: u32,
}

impl<'a> Cursor<'a> {
    pub fn new<P>(src: &'a str, file: Option<P>) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            iter: src.chars().peekable(),
            file: file.map(|inner| PathBuf::from(inner.as_ref())),
            tokens: Vec::default(),
            errors: Vec::default(),
            offset: 0,
            line_start: 0,
            line: 0,
            reserved: HashMap::from([
                (String::from("and"), TokenKind::And),
                (String::from("class"), TokenKind::Class),
                (String::from("else"), TokenKind::Else),
                (String::from("false"), TokenKind::False),
                (String::from("fun"), TokenKind::Fun),
                (String::from("for"), TokenKind::For),
                (String::from("if"), TokenKind::If),
                (String::from("nil"), TokenKind::Nil),
                (String::from("or"), TokenKind::Or),
                (String::from("print"), TokenKind::Print),
                (String::from("return"), TokenKind::Return),
                (String::from("super"), TokenKind::Super),
                (String::from("this"), TokenKind::This),
                (String::from("true"), TokenKind::True),
                (String::from("var"), TokenKind::Var),
                (String::from("while"), TokenKind::While),
            ]),
        }
    }

    pub fn add_token(&mut self, kind: TokenKind, lex: String, lit: Option<Value>) {
        let span = Span {
            line: Line(self.line + 1),
            column: Column(self.offset - self.line_start),
            file: self.file.clone(),
            lex,
        };
        let token: Token = Token::new(kind, lit, span);
        self.tokens.push(token);
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<Report>> {
        while let Some(c) = self.iter.next() {
            self.offset += 1;
            match c {
                '(' => self.add_token(TokenKind::LeftParen, c.to_string(), None),
                ')' => self.add_token(TokenKind::RightParen, c.to_string(), None),
                '{' => self.add_token(TokenKind::LeftBrace, c.to_string(), None),
                '}' => self.add_token(TokenKind::RightBrace, c.to_string(), None),
                ',' => self.add_token(TokenKind::Comma, c.to_string(), None),
                '.' => self.add_token(TokenKind::Dot, c.to_string(), None),
                '-' => self.add_token(TokenKind::Minus, c.to_string(), None),
                '+' => self.add_token(TokenKind::Plus, c.to_string(), None),
                ';' => self.add_token(TokenKind::Semicolon, c.to_string(), None),
                '*' => self.add_token(TokenKind::Star, c.to_string(), None),
                '!' => self.branching_char(c, '=', TokenKind::BangEqual, TokenKind::Bang),
                '=' => self.branching_char(c, '=', TokenKind::EqualEqual, TokenKind::Equal),
                '<' => self.branching_char(c, '=', TokenKind::LessEqual, TokenKind::Less),
                '>' => self.branching_char(c, '=', TokenKind::GreaterEqual, TokenKind::Greater),
                '/' => self.handle_comment(c),
                '"' => self.handle_string(),
                '0'..='9' => self.handle_number(c),
                'a'..='z' | 'A'..='Z' => self.handle_ident(c),
                '\n' => {
                    self.line_start = self.offset;
                    self.line += 1;
                }
                '\r' | '\t' | ' ' => (),
                _ => self.errors.push(SyntaxError::UnexpectedCharacter(c).into()),
            }
        }

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    pub fn branching_char(
        &mut self,
        curr: char,
        next: char,
        success: TokenKind,
        failure: TokenKind,
    ) {
        match self.iter.peek() {
            Some(x) if *x == next => {
                self.iter.next().unwrap();
                self.add_token(success, format!("{curr}{next}"), None);
            }
            Some(_) => self.add_token(failure, next.to_string(), None),
            None => (),
        }
    }

    pub fn handle_ident(&mut self, curr: char) {
        let mut ident = vec![curr];
        while let Some('a'..='z' | 'A'..='Z' | '1'..='9') = self.iter.peek() {
            ident.push(self.iter.next().unwrap());
        }

        let ident = ident.into_iter().collect::<String>();
        if self.reserved.contains_key(&ident) {
            let tt = self.reserved.get(&ident).unwrap().clone();
            match tt {
                TokenKind::True => {
                    self.add_token(tt, ident, Some(true.into()));
                }
                TokenKind::False => {
                    self.add_token(tt, ident, Some(false.into()));
                }
                _ => self.add_token(tt, ident, None),
            }
        } else {
            self.add_token(TokenKind::Identifier, ident, None);
        }
    }

    pub fn handle_number(&mut self, curr: char) {
        let mut num = vec![curr];
        let mut float = false;

        loop {
            match self.iter.peek() {
                Some('0'..='9') => num.push(self.iter.next().unwrap()),
                Some('.') => {
                    float = true;
                    num.push(self.iter.next().unwrap());
                }
                _ => break,
            }
        }

        let pre_literal = num.into_iter().collect::<String>();
        let lexeme = pre_literal.clone();

        if float {
            self.add_token(
                TokenKind::Number,
                lexeme,
                Some(Value::Float(ordered_float::OrderedFloat(
                    pre_literal.parse::<f64>().unwrap(),
                ))),
            );
        } else {
            self.add_token(
                TokenKind::Number,
                lexeme,
                Some(Value::Integer(pre_literal.parse::<i128>().unwrap())),
            );
        }
    }

    pub fn handle_string(&mut self) {
        let mut chars = vec!['"'];
        let (token, lit) = loop {
            match self.iter.next() {
                Some('"') => break (TokenKind::String, chars[1..].iter().collect::<String>()),
                Some(char) => chars.push(char),
                None => self.errors.push(
                    SyntaxError::ExpectedCharacter {
                        expected: '"',
                        found: String::from("EOF"),
                    }
                    .into(),
                ),
            }
        };

        chars.push('"');
        self.add_token(
            token,
            chars.into_iter().collect::<String>(),
            Some(Value::String(lit)),
        );
    }

    pub fn handle_comment(&mut self, curr: char) {
        if self.iter.peek().unwrap() == &'/' {
            loop {
                match self.iter.next() {
                    Some('\n') | None => break,
                    Some(_) => (),
                }
            }
            self.line += 1;
        } else {
            self.add_token(TokenKind::Slash, curr.to_string(), None);
        }
    }
}
