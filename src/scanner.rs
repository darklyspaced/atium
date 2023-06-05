use super::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Default)]
struct Scanner {
    src: String,
    tokens: Vec<Token>,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            src: String::from(source),
            tokens: vec![],
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut source = self.src.chars().peekable();
        let mut full_toks = vec![];
        let mut sprse_toks = vec![];

        let mut full_tok = |tt, lex, lit| {
            let token = Token::new(tt, lex, lit, 0);
            full_toks.push(token);
        };
        let mut sprse_tok = |tt, lex: char| {
            let mut buf = [0; 1];
            let lex = lex.encode_utf8(&mut buf).to_string();
            let token = Token::new(tt, lex, String::from(""), 0);
            sprse_toks.push(token);
        };

        loop {
            let next = source.next();
            if let Some(chr) = next {
                match chr {
                    '(' => sprse_tok(TokenType::LEFTPAREN, chr),
                    ')' => sprse_tok(TokenType::RIGHTPAREN, chr),
                    '{' => sprse_tok(TokenType::LEFTBRACE, chr),
                    '}' => sprse_tok(TokenType::RIGHTBRACE, chr),
                    ',' => sprse_tok(TokenType::COMMA, chr),
                    '.' => sprse_tok(TokenType::DOT, chr),
                    '-' => sprse_tok(TokenType::MINUS, chr),
                    '+' => sprse_tok(TokenType::PLUS, chr),
                    ';' => sprse_tok(TokenType::SEMICOLON, chr),
                    '*' => sprse_tok(TokenType::STAR, chr),
                    '!' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::BANGEQUAL, chr)
                        } else {
                            sprse_tok(TokenType::BANG, chr)
                        }
                    }
                    '=' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::EQUALEQUAL, chr)
                        } else {
                            sprse_tok(TokenType::EQUAL, chr)
                        }
                    }
                    '<' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::LESSEQUAL, chr)
                        } else {
                            sprse_tok(TokenType::LESSEQUAL, chr)
                        }
                    }
                    '>' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::GREATEREQUAL, chr)
                        } else {
                            sprse_tok(TokenType::GREATER, chr)
                        }
                    }
                    '/' => {
                        if source.peek().unwrap() == &'/' {
                            source.next();
                            let token_type = loop {
                                match source.next() {
                                    Some('\n') => break None,
                                    None => break Some(TokenType::EOF),
                                    _ => (),
                                };
                            };
                            if let Some(tt) = token_type {
                                sprse_tok(tt, ' '); // EOF has no lexeme
                            }
                            self.line += 1; // if its a comment then scanner consumes the \n
                        } else {
                            sprse_tok(TokenType::SLASH, chr)
                        }
                    }
                    '"' => {
                        let mut chars = vec!['"'];
                        let (token, lit) = loop {
                            match source.next() {
                                Some('"') => {
                                    break (TokenType::STRING, String::from_iter(chars[1..].iter()))
                                }
                                Some(char) => chars.push(char),
                                None => panic!("unterminated string"),
                            }
                        };
                        chars.push('"');
                        full_tok(token, String::from_iter(chars.into_iter()), lit);
                    }
                    _ if chr.is_digit(10) => {
                        let mut num = vec![chr];
                        loop {
                            match source.peek() {
                                Some(char) if char.is_digit(10) || char.eq(&'.') => {
                                    num.push(source.next().unwrap())
                                }
                                Some(_) => break,
                                None => full_tok(TokenType::EOF, "".into(), "".into()),
                            }
                        }
                        let lit = String::from_iter(num.into_iter());
                        full_tok(TokenType::NUMBER, lit.clone(), lit)
                    }
                    '\r' => (),
                    '\t' => (),
                    '\n' => {
                        self.line += 1;
                    }
                    _ if chr.is_alphabetic() => {
                        let mut ident = vec![chr];
                        loop {
                            match source.peek() {
                                Some(char) if char.is_alphabetic() || char.eq(&'_') => {
                                    ident.push(source.next().unwrap())
                                }
                                Some(_) => break,
                                None => full_tok(TokenType::EOF, "".into(), "".into()),
                            }
                        }
                    }
                    ' ' => (),
                    _ => panic!("unexpected character"),
                };
            } else {
                self.tokens
                    .push(Token::new(TokenType::EOF, "".into(), "".into(), self.line));
                break;
            }
        }

        full_toks.extend(sprse_toks);
        self.tokens.extend(full_toks);
        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), "".into(), self.line));
        self.tokens.clone()
    }
}
