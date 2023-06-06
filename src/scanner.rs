use std::collections::HashMap;

use super::token::{Token, TokenType};
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
            let token = Token::new(tt, lex, lit, 0);
            self.tokens.push(token);
        };

        loop {
            let next = source.next();
            if let Some(chr) = next {
                match chr {
                    '(' => add_tok(LeftParen, ctos(chr), String::new()),
                    ')' => add_tok(RightParen, ctos(chr), String::new()),
                    '{' => add_tok(LeftBrace, ctos(chr), String::new()),
                    '}' => add_tok(RightBrace, ctos(chr), String::new()),
                    ',' => add_tok(Comma, ctos(chr), String::new()),
                    '.' => add_tok(Dot, ctos(chr), String::new()),
                    '-' => add_tok(Minus, ctos(chr), String::new()),
                    '+' => add_tok(Plus, ctos(chr), String::new()),
                    ';' => add_tok(Semicolon, ctos(chr), String::new()),
                    '*' => add_tok(Star, ctos(chr), String::new()),
                    '!' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(BangEqual, ctos(chr), String::new());
                        } else {
                            add_tok(Bang, ctos(chr), String::new());
                        }
                    }
                    '=' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(EqualEqual, ctos(chr), String::new());
                        } else {
                            add_tok(Equal, ctos(chr), String::new());
                        }
                    }
                    '<' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(LessEqual, ctos(chr), String::new());
                        } else {
                            add_tok(Less, ctos(chr), String::new());
                        }
                    }
                    '>' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            add_tok(GreaterEqual, ctos(chr), String::new());
                        } else {
                            add_tok(Greater, ctos(chr), String::new());
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
                                add_tok(tt, String::new(), String::new()); // EOF has no lexeme
                            }
                            self.line += 1; // if its a comment then scanner consumes the \n
                        } else {
                            add_tok(Slash, ctos(chr), String::new());
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
                        add_tok(token, chars.into_iter().collect::<String>(), lit);
                    }
                    _ if chr.is_ascii_digit() => {
                        let mut num = vec![chr];
                        loop {
                            match source.peek() {
                                Some(char) if char.is_ascii_digit() || char.eq(&'.') => {
                                    num.push(source.next().unwrap());
                                }
                                Some(_) => break,
                                None => add_tok(EOF, String::new(), String::new()),
                            }
                        }
                        let lit = num.into_iter().collect::<String>();
                        add_tok(Number, lit.clone(), lit);
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
                                None => add_tok(EOF, String::new(), String::new()),
                            }
                        }
                        let ident = ident.into_iter().collect::<String>();
                        if self.reserved.contains_key::<str>(&ident) {
                            let tt = self.reserved.get::<str>(&ident).unwrap();
                            add_tok(tt.clone(), ident, String::new());
                        } else {
                            add_tok(Identifier, ident, String::new());
                        }
                    }
                    ' ' => (),
                    _ => panic!("unexpected character: {}", chr),
                };
            } else {
                self.tokens
                    .push(Token::new(EOF, String::new(), String::new(), self.line));
                break;
            }
        }
    }
}
