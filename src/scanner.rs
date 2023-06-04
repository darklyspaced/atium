use super::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Default)]
struct Scanner {
    src: String,
    tokens: Vec<Token>,
    line: usize,
}

fn handle_string<I>(iter: &mut Peekable<I>) -> (TokenType, String)
where
    I: Iterator<Item = char>,
{
    let mut chars = vec!['"'];
    loop {
        match iter.next() {
            Some('"') => break (TokenType::STRING, String::from_iter(chars.into_iter())),
            Some(char) => chars.push(char),
            None => panic!("unterminated string"),
        }
    }
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
            if let Some(char) = next {
                match char {
                    '(' => sprse_tok(TokenType::LEFTPAREN, char),
                    ')' => sprse_tok(TokenType::RIGHTPAREN, char),
                    '{' => sprse_tok(TokenType::LEFTBRACE, char),
                    '}' => sprse_tok(TokenType::RIGHTBRACE, char),
                    ',' => sprse_tok(TokenType::COMMA, char),
                    '.' => sprse_tok(TokenType::DOT, char),
                    '-' => sprse_tok(TokenType::MINUS, char),
                    '+' => sprse_tok(TokenType::PLUS, char),
                    ';' => sprse_tok(TokenType::SEMICOLON, char),
                    '*' => sprse_tok(TokenType::STAR, char),
                    '!' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::BANGEQUAL, char)
                        } else {
                            sprse_tok(TokenType::BANG, char)
                        }
                    }
                    '=' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::EQUALEQUAL, char)
                        } else {
                            sprse_tok(TokenType::EQUAL, char)
                        }
                    }
                    '<' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::LESSEQUAL, char)
                        } else {
                            sprse_tok(TokenType::LESSEQUAL, char)
                        }
                    }
                    '>' => {
                        if source.peek().unwrap() == &'=' {
                            source.next();
                            sprse_tok(TokenType::GREATEREQUAL, char)
                        } else {
                            sprse_tok(TokenType::GREATER, char)
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
                            sprse_tok(TokenType::SLASH, char)
                        }
                    }
                    '"' => {
                        let (token, lit) = handle_string(&mut source);
                        let mut buf = [0; 1];
                        full_tok(token, char.encode_utf8(&mut buf).into(), lit);
                    }
                    ' ' => (),
                    '\r' => (),
                    '\t' => (),
                    '\n' => {
                        self.line += 1;
                    }
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
