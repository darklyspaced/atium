use std::collections::HashMap;

use super::token::{Token, TokenType, Type};
use std::char;
use TokenType::*;

#[derive(Default)]
pub struct Scanner<'a> {
    src: String,
    pub tokens: Vec<Token>,
    reserved: HashMap<&'a str, TokenType>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String) -> Self {
        Self {
            src: source,
            reserved: HashMap::from([
                ("and", And),
                ("class", Class),
                ("else", Else),
                ("false", False),
                ("fun", Fun),
                ("for", For),
                ("if", If),
                ("nil", Nil),
                ("or", Or),
                ("print", Print),
                ("return", Return),
                ("super", Super),
                ("this", This),
                ("true", True),
                ("var", Var),
                ("while", While),
            ]),
            tokens: vec![],
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        fn ctos(char: char) -> String {
            let vector = vec![char];
            vector.into_iter().collect()
        }

        let mut source = self.src.chars().peekable();

        let mut add_tok = |tt, lex, lit| {
            let token: Token = Token::new(tt, lex, lit, 0);
            self.tokens.push(token);
        };

        loop {
            let next = source.next();
            if let Some(chr) = next {
                match chr {
                    '(' => add_tok(LeftParen, ctos(chr), None),
                    ')' => add_tok(RightParen, ctos(chr), None),
                    '{' => add_tok(LeftBrace, ctos(chr), None),
                    '}' => add_tok(RightBrace, ctos(chr), None),
                    ',' => add_tok(Comma, ctos(chr), None),
                    '.' => add_tok(Dot, ctos(chr), None),
                    '-' => add_tok(Minus, ctos(chr), None),
                    '+' => add_tok(Plus, ctos(chr), None),
                    ';' => add_tok(Semicolon, ctos(chr), None),
                    '*' => add_tok(Star, ctos(chr), None),
                    '!' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(BangEqual, ctos(chr), None);
                        } else {
                            add_tok(Bang, ctos(chr), None);
                        }
                    }
                    '=' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(EqualEqual, ctos(chr), None);
                        } else {
                            add_tok(Equal, ctos(chr), None);
                        }
                    }
                    '<' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(LessEqual, ctos(chr), None);
                        } else {
                            add_tok(Less, ctos(chr), None);
                        }
                    }
                    '>' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(GreaterEqual, ctos(chr), None);
                        } else {
                            add_tok(Greater, ctos(chr), None);
                        }
                    }
                    '/' => {
                        if source.peek().unwrap() == &'/' {
                            source.next();
                            let token_type = loop {
                                match source.next() {
                                    Some('\n') => break None,
                                    None => break Some(EOF),
                                    _ => (),
                                };
                            };
                            if let Some(tt) = token_type {
                                add_tok(tt, String::new(), None);
                                // EOF has no lexeme
                            }
                            self.line += 1; // if its a comment then scanner consumes the \n
                        } else {
                            add_tok(Slash, ctos(chr), None);
                        }
                    }
                    '"' => {
                        let mut chars = vec!['"'];
                        let (token, lit) = loop {
                            match source.next() {
                                Some('"') => break (Str, chars[1..].iter().collect::<String>()),
                                Some(char) => chars.push(char),
                                None => panic!("unterminated string"),
                            }
                        };
                        chars.push('"');
                        add_tok(
                            token,
                            chars.into_iter().collect::<String>(),
                            Some(Type::String(lit)),
                        );
                    }
                    _ if chr.is_ascii_digit() => {
                        let mut num = vec![chr];
                        loop {
                            match source.peek() {
                                Some(char) if char.is_ascii_digit() || char.eq(&'.') => {
                                    num.push(source.next().unwrap());
                                }
                                Some(_) => break,
                                None => add_tok(EOF, String::new(), None),
                            }
                        }
                        let lit = num.into_iter().collect::<String>();
                        let int_lit = lit.clone().parse::<i64>().unwrap();

                        add_tok(Number, lit, Some(Type::Integer(int_lit)));
                    }
                    '\r' | '\t' => (),
                    '\n' => {
                        self.line += 1;
                    }
                    _ if chr.is_alphabetic() => {
                        let mut ident = vec![chr];
                        loop {
                            match source.peek() {
                                Some(char) if char.is_alphanumeric() || char.eq(&'_') => {
                                    ident.push(source.next().unwrap());
                                }
                                Some(_) => break,
                                None => add_tok(EOF, String::new(), None),
                            }
                        }
                        let ident = ident.into_iter().collect::<String>();
                        if self.reserved.contains_key::<str>(&ident) {
                            let tt = self.reserved.get::<str>(&ident).unwrap();
                            add_tok(tt.clone(), ident, None);
                        } else {
                            add_tok(Identifier, ident, None);
                        }
                    }
                    ' ' => (),
                    _ => panic!("unexpected character: {}", chr),
                };
            } else {
                self.tokens
                    .push(Token::new(EOF, String::new(), None, self.line));
                break;
            }
        }
    }
}
