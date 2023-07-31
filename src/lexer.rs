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
    /// line that the cursor is currently lexing
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: src.chars().peekable(),
            tokens: Vec::default(),
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
        let mut errors: Vec<Report> = vec![];

        loop {
            let next = self.iter.next();
            if let Some(chr) = next {
                match chr {
                    '(' => self.add_token(LeftParen, chr.to_string(), None),
                    ')' => self.add_token(RightParen, chr.to_string(), None),
                    '{' => self.add_token(LeftBrace, chr.to_string(), None),
                    '}' => self.add_token(RightBrace, chr.to_string(), None),
                    ',' => self.add_token(Comma, chr.to_string(), None),
                    '.' => self.add_token(Dot, chr.to_string(), None),
                    '-' => self.add_token(Minus, chr.to_string(), None),
                    '+' => self.add_token(Plus, chr.to_string(), None),
                    ';' => self.add_token(Semicolon, chr.to_string(), None),
                    '*' => self.add_token(Star, chr.to_string(), None),
                    '!' => self.handle_two_char_op(chr, '=', BangEqual, Bang),
                    '=' => self.handle_two_char_op(chr, '=', EqualEqual, EqualEqual),
                    '<' => self.handle_two_char_op(chr, '=', LessEqual, Less),
                    '>' => self.handle_two_char_op(chr, '=', GreaterEqual, Greater),
                    '/' => {
                        dbg!(&self.iter.peek());
                        if self.iter.peek().unwrap() == &'/' {
                            loop {
                                match self.iter.next() {
                                    Some('\n') | None => break,
                                    Some(_) => (),
                                }
                            }
                            self.line += 1;
                        } else {
                            self.add_token(Slash, chr.to_string(), None);
                        }
                    }
                    '"' => {
                        let mut chars = vec!['"'];
                        let (token, lit) = loop {
                            match self.iter.next() {
                                Some('"') => break (Str, chars[1..].iter().collect::<String>()),
                                Some(char) => chars.push(char),
                                None => errors.push(
                                    SyntaxError::ExpectedCharacter(String::from("EOF"), '"').into(),
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
                    _ if chr.is_ascii_digit() => {
                        let mut num = vec![chr];
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
                    'a'..='z' | 'A'..='Z' => {
                        let mut ident = vec![chr];
                        while let Some('a'..='z' | 'A'..='Z' | '1'..='9') = self.iter.peek() {
                            ident.push(self.iter.next().unwrap());
                        }
                        // loop {
                        //     match self.iter.peek() {
                        //         Some(char) if char.is_alphanumeric() || char.eq(&'_') => {
                        //             ident.push(self.iter.next().unwrap());
                        //         }
                        //         _ => break,
                        //     }
                        // }
                        let ident = ident.into_iter().collect::<String>();
                        if self.reserved.contains_key(&ident) {
                            let tt = self.reserved.get(&ident).unwrap().clone();
                            match tt {
                                TokenType::True => {
                                    self.add_token(tt, ident, Some(Value::Boolean(true)));
                                }
                                TokenType::False => {
                                    self.add_token(tt, ident, Some(Value::Boolean(false)));
                                }
                                _ => self.add_token(tt, ident, None),
                            }
                        } else {
                            self.add_token(Identifier, ident, None);
                        }
                    }
                    '\n' => self.line += 1,
                    '\r' | '\t' | ' ' => (),
                    _ => errors.push(SyntaxError::UnexpectedCharacter(chr).into()),
                };
            } else {
                break;
            }
        }

        dbg!(&self.tokens);
        if errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(errors)
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
                    literal: Some(Value::Boolean(true,),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: String::from("1"),
                    literal: Some(Value::Integer(1,),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Str,
                    lexeme: String::from("\"foo\""),
                    literal: Some(Value::String(String::from("foo"),),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::False,
                    lexeme: String::from("false"),
                    literal: Some(Value::Boolean(false,),),
                    line: 0,
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: String::from("69.420"),
                    literal: Some(Value::Float(69.420),),
                    line: 0,
                }
            ]
        );
    }
}
