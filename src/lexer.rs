use super::error::SyntaxError;
use color_eyre::{Report, Result};

use super::token::{Token, TokenType, Value};
use std::{collections::HashMap, iter::Peekable, str::Chars};
use TokenType::*;

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
                (String::from("and"), And),
                (String::from("class"), Class),
                (String::from("else"), Else),
                (String::from("false"), False),
                (String::from("fun"), Fun),
                (String::from("for"), For),
                (String::from("if"), If),
                (String::from("nil"), Nil),
                (String::from("or"), Or),
                (String::from("print"), Print),
                (String::from("return"), Return),
                (String::from("super"), Super),
                (String::from("this"), This),
                (String::from("true"), True),
                (String::from("var"), Var),
                (String::from("while"), While),
            ]),
            line: 0,
        }
    }

    pub fn add_token(&mut self, tt: TokenType, lex: String, lit: Option<Value>) {
        let token: Token = Token::new(tt, lex, lit, self.line);
        self.tokens.push(token);
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<Report>> {
        loop {
            let next = self.iter.next();
            if let Some(curr) = next {
                match curr {
                    '(' => self.add_token(LeftParen, curr.to_string(), None),
                    ')' => self.add_token(RightParen, curr.to_string(), None),
                    '{' => self.add_token(LeftBrace, curr.to_string(), None),
                    '}' => self.add_token(RightBrace, curr.to_string(), None),
                    ',' => self.add_token(Comma, curr.to_string(), None),
                    '.' => self.add_token(Dot, curr.to_string(), None),
                    '-' => self.add_token(Minus, curr.to_string(), None),
                    '+' => self.add_token(Plus, curr.to_string(), None),
                    ';' => self.add_token(Semicolon, curr.to_string(), None),
                    '*' => self.add_token(Star, curr.to_string(), None),
                    '!' => self.handle_two_char_op(curr, '=', BangEqual, Bang),
                    '=' => self.handle_two_char_op(curr, '=', EqualEqual, EqualEqual),
                    '<' => self.handle_two_char_op(curr, '=', LessEqual, Less),
                    '>' => self.handle_two_char_op(curr, '=', GreaterEqual, Greater),
                    '/' => self.handle_comment(curr),
                    '"' => self.handle_string(),
                    '0'..='9' => self.handle_number(curr),
                    'a'..='z' | 'A'..='Z' => self.handle_ident(curr),
                    '\n' => self.line += 1,
                    '\r' | '\t' | ' ' => (),
                    _ => self
                        .errors
                        .push(SyntaxError::UnexpectedCharacter(curr).into()),
                };
            } else {
                break;
            }
        }

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    pub fn handle_two_char_op(
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
            self.add_token(Identifier, ident, None);
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
                Number,
                lexeme,
                Some(Value::Float(pre_literal.parse::<f64>().unwrap())),
            );
        } else {
            self.add_token(
                Number,
                lexeme,
                Some(Value::Integer(pre_literal.parse::<i128>().unwrap())),
            );
        }
    }

    pub fn handle_string(&mut self) {
        let mut chars = vec!['"'];
        let (token, lit) = loop {
            match self.iter.next() {
                Some('"') => break (Str, chars[1..].iter().collect::<String>()),
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
            self.add_token(Slash, curr.to_string(), None);
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
    fn print_stmt() {
        let input = "print 10;";
        let cursor = Cursor::new(input);

        let tokens = cursor.scan_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
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
                }
            ]
        );
    }

    #[test]
    fn successive_print_stmts() {
        let input = r#"print 10; print "string";"#;
        let cursor = Cursor::new(input);

        let tokens = cursor.scan_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
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
                    token_type: TokenType::Str,
                    lexeme: String::from("\"string\""),
                    literal: Some(Value::String(String::from("string"))),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Semicolon,
                    lexeme: String::from(";"),
                    literal: None,
                    line: 0,
                }
            ]
        );
    }

    #[test]
    fn comment() {
        let input = "// there should be no tokens!";
        let cursor = Cursor::new(input);

        let tokens = cursor.scan_tokens().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn keyword_ident() {
        let input = "var stormlight";
        let cursor = Cursor::new(input);

        let tokens = cursor.scan_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
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
            ]
        );
    }

    #[test]
    fn types() {
        let input = r#"true 1 "foo" false 69.420"#;
        let cursor = Cursor::new(input);

        let tokens = cursor.scan_tokens().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::True,
                    lexeme: String::from("true"),
                    literal: Some(true.into(),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: String::from("1"),
                    literal: Some(1.into(),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Str,
                    lexeme: String::from("\"foo\""),
                    literal: Some(String::from("foo").into()),
                    line: 0,
                },
                Token {
                    token_type: TokenType::False,
                    lexeme: String::from("false"),
                    literal: Some(false.into(),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: String::from("69.420"),
                    literal: Some(69.420.into(),),
                    line: 0,
                }
            ]
        );
    }
}
