use super::error::SyntaxError;
use color_eyre::{Report, Result};

use super::token::{Token, TokenType, Value};
use std::{collections::HashMap, iter::Peekable, str::Chars};

/// Contains a peekable iterator over a stream of characters (the source code).
///
/// The source code is converted into a stream of tokens.
pub(super) struct Cursor<'a> {
    /// peekable iterator over stream of chars
    iter: Peekable<Chars<'a>>,
    /// tokens present in source code
    tokens: Vec<Token>,
    /// reserved keywords for the language
    reserved: HashMap<String, TokenType>,
    /// errors present in the source code
    errors: Vec<Report>,
    /// line that the cursor is currently lexing
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars().peekable(),
            tokens: Vec::default(),
            errors: Vec::default(),
            reserved: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("fun"), TokenType::Fun),
                (String::from("for"), TokenType::For),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
            line: 0,
        }
    }

    pub fn add_token(&mut self, tt: TokenType, lex: String, lit: Option<Value>) {
        let token: Token = Token::new(tt, lex, lit, self.line);
        self.tokens.push(token);
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<Report>> {
        while let Some(c) = self.iter.next() {
            match c {
                '(' => self.add_token(TokenType::LeftParen, c.to_string(), None),
                ')' => self.add_token(TokenType::RightParen, c.to_string(), None),
                '{' => self.add_token(TokenType::LeftBrace, c.to_string(), None),
                '}' => self.add_token(TokenType::RightBrace, c.to_string(), None),
                ',' => self.add_token(TokenType::Comma, c.to_string(), None),
                '.' => self.add_token(TokenType::Dot, c.to_string(), None),
                '-' => self.add_token(TokenType::Minus, c.to_string(), None),
                '+' => self.add_token(TokenType::Plus, c.to_string(), None),
                ';' => self.add_token(TokenType::Semicolon, c.to_string(), None),
                '*' => self.add_token(TokenType::Star, c.to_string(), None),
                '!' => self.branching_char(c, '=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.branching_char(c, '=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.branching_char(c, '=', TokenType::LessEqual, TokenType::Less),
                '>' => self.branching_char(c, '=', TokenType::GreaterEqual, TokenType::Greater),
                '/' => self.handle_comment(c),
                '"' => self.handle_string(),
                '0'..='9' => self.handle_number(c),
                'a'..='z' | 'A'..='Z' => self.handle_ident(c),
                '\n' => self.line += 1,
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
        predicate: char,
        success: TokenType,
        failure: TokenType,
    ) {
        match self.iter.peek() {
            Some(x) if x == &predicate => {
                self.iter.next().unwrap(); // NOTE: cannot be None as alr peeked
                self.add_token(success, format!("{curr}{predicate}"), None);
            }
            Some(_) => self.add_token(failure, predicate.to_string(), None),
            None => (), // NOTE: we now can add smthing to None case in the future
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
                TokenType::True => {
                    self.add_token(tt, ident, Some(true.into()));
                }
                TokenType::False => {
                    self.add_token(tt, ident, Some(false.into()));
                }
                _ => self.add_token(tt, ident, None),
            }
        } else {
            self.add_token(TokenType::Identifier, ident, None);
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
                TokenType::Number,
                lexeme,
                Some(Value::Float(pre_literal.parse::<f64>().unwrap())),
            );
        } else {
            self.add_token(
                TokenType::Number,
                lexeme,
                Some(Value::Integer(pre_literal.parse::<i128>().unwrap())),
            );
        }
    }

    pub fn handle_string(&mut self) {
        let mut chars = vec!['"'];
        let (token, lit) = loop {
            match self.iter.next() {
                Some('"') => break (TokenType::String, chars[1..].iter().collect::<String>()),
                Some(char) => chars.push(char),
                None => self
                    .errors
                    .push(SyntaxError::ExpectedCharacter(String::from("EOF"), '"').into()),
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
            self.add_token(TokenType::Slash, curr.to_string(), None);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Cursor,
        token::{Token, TokenType, Value},
    };

    #[test]
    fn print_stmt() -> color_eyre::Result<()> {
        let input = "print 10;";
        let cursor = Cursor::new(input);

        let tokens = serde_json::to_string_pretty(&cursor.lex().unwrap())?;
        let expected = r#"[
  {
    "token_type": "Print",
    "lexeme": "print",
    "literal": null,
    "line": 0
  },
  {
    "token_type": "Number",
    "lexeme": "10",
    "literal": {
      "Integer": 10
    },
    "line": 0
  },
  {
    "token_type": "Semicolon",
    "lexeme": ";",
    "literal": null,
    "line": 0
  }
]"#;

        assert_eq!(tokens, expected);
        Ok(())
    }

    #[test]
    fn successive_print_stmts() {
        let input = r#"print 10; print "string";"#;
        let cursor = Cursor::new(input);

        let tokens = cursor.lex().unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::Print,
                lexeme: String::from("print"),
                literal: None,
                line: 0,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("10"),
                literal: Some(Value::Integer(10)),
                line: 0,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: String::from(";"),
                literal: None,
                line: 0,
            },
            Token {
                token_type: TokenType::Print,
                lexeme: String::from("print"),
                literal: None,
                line: 0,
            },
            Token {
                token_type: TokenType::String,
                lexeme: String::from("\"string\""),
                literal: Some(Value::String(String::from("string"))),
                line: 0,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: String::from(";"),
                literal: None,
                line: 0,
            },
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn comment() {
        let input = "// there should be no tokens!";
        let cursor = Cursor::new(input);

        let tokens = cursor.lex().unwrap();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn keyword_ident() {
        let input = "var stormlight";
        let cursor = Cursor::new(input);

        let tokens = cursor.lex().unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::Var,
                lexeme: String::from("var"),
                literal: None,
                line: 0,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("stormlight"),
                literal: None,
                line: 0,
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn types() {
        let input = r#"true 1 "foo" false 69.420"#;
        let cursor = Cursor::new(input);

        let tokens = cursor.lex().unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::True,
                lexeme: String::from("true"),
                literal: Some(true.into()),
                line: 0,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("1"),
                literal: Some(1.into()),
                line: 0,
            },
            Token {
                token_type: TokenType::String,
                lexeme: String::from("\"foo\""),
                literal: Some(String::from("foo").into()),
                line: 0,
            },
            Token {
                token_type: TokenType::False,
                lexeme: String::from("false"),
                literal: Some(false.into()),
                line: 0,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: String::from("69.420"),
                literal: Some(69.420.into()),
                line: 0,
            },
        ];

        assert_eq!(tokens, expected);
    }
}
