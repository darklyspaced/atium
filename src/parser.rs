use std::{iter::Peekable, result, vec::IntoIter};

use color_eyre::Result;

use crate::{dump, error::SyntaxError};

use super::{
    ast::Stmt,
    impetuous::Impetuous,
    token::{Token, TokenKind},
};

mod expr;

pub(super) struct Parser {
    iter: Peekable<IntoIter<Token>>,
    prev: Option<Token>,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            iter: token_stream.into_iter().peekable(),
            prev: None,
        }
    }

    /// Converts a stream of tokens into an abstract syntax tree
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<color_eyre::Report>> {
        let mut statements: Vec<Result<Stmt>> = vec![];

        while self.iter.peek().is_some() {
            statements.push(self.declaration());
        }

        if statements.iter().any(result::Result::is_err) {
            Err(statements
                .into_iter()
                .filter_map(result::Result::err)
                .collect())
        } else {
            Ok(statements.into_iter().flatten().collect())
        }
    }

    fn declaration(&mut self) -> Result<Stmt> {
        match self.peer()?.kind {
            TokenKind::Var => {
                self.advance()?; // consume Var tok
                match self.var_decl() {
                    Ok(stmt) => Ok(stmt),
                    Err(e) => {
                        let prev = &self.prev().unwrap().kind.clone();
                        self.recover(prev);
                        Err(e)
                    }
                }
            }
            _ => self.statement().map_err(|e| {
                if let Some(prev) = &self.prev() {
                    self.recover(&prev.kind.clone());
                }
                e
            }),
        }
    }

    fn var_decl(&mut self) -> Result<Stmt> {
        let Some(ident) = self.eat(TokenKind::Identifier) else {
            match self.next() {
                Some(tok) => dump!(SyntaxError::ExpectedIdent(String::from(&tok.lex()))),
                None => dump!(SyntaxError::ExpectedIdent(String::from("EOF"))),
            }
        };

        let initial_value = if self.taste(TokenKind::Equal)? {
            self.advance()?; // consume the Equal
            Some(self.expression().unwrap())
        } else {
            None
        };

        if self.eat(TokenKind::Semicolon).is_none() {
            dump!(SyntaxError::ExpectedCharacter {
                found: self
                    .prev()
                    .map_or_else(|| String::from("EOF"), |tok| String::from(&tok.lex())),
                expected: ';',
            })
        }

        Ok(Stmt::Var {
            name: ident,
            value: initial_value,
        })
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.peer()?.kind {
            TokenKind::Print => {
                self.eat(TokenKind::Print).unwrap();
                let expr = self.expression()?;

                match self.step() {
                    Some(tok) => {
                        if tok.kind != TokenKind::Semicolon {
                            dump!(SyntaxError::ExpectedCharacter {
                                expected: ';',
                                found: self.advance()?.lex(),
                            })
                        }
                        Ok(Stmt::Print(expr))
                    }
                    None => dump!(SyntaxError::ExpectedCharacter {
                        expected: ';',
                        found: String::from("EOF"),
                    }),
                }
            }
            TokenKind::LeftBrace => {
                let mut stmts = vec![];
                self.eat(TokenKind::LeftBrace).unwrap();

                while let Ok(false) = self.taste(TokenKind::RightBrace) {
                    stmts.push(self.declaration()?)
                }

                match self.step() {
                    Some(tok) => match tok.kind {
                        TokenKind::RightBrace => Ok(Stmt::Block(stmts)),
                        _ => dump!(SyntaxError::ExpectedCharacter {
                            expected: '}',
                            found: tok.lex()
                        }),
                    },
                    None => dump!(SyntaxError::ExpectedCharacter {
                        expected: '}',
                        found: String::from("EOF")
                    }),
                }
            }
            _ => Ok(Stmt::Expr(self.expression()?)),
        }
    }

    /// Prevents error cascading.
    ///
    /// Discards tokens until the next statement is reached. Invoked when an error is thrown while
    /// parsing. Discarded tokens were part of a statement that caused an error and therefore were
    /// most likely erroneous themselves.
    fn recover(&mut self, prev: &TokenKind) -> Option<()> {
        self.step()?;

        if *prev == TokenKind::Semicolon {
            return Some(());
        }

        for next in self.iter.by_ref() {
            match next.kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => {
                    return Some(());
                }
                _ => (),
            }
        }
        Some(())
    }
}

// make this all generic behaviour that can be overriden if need be.
impl Impetuous for Parser {
    type Scrutinee = TokenKind;
    /// Replaces the `next` function on Iterators, advancing one step forwards while keeping track
    /// of the previous elem
    fn step(&mut self) -> Option<Token> {
        self.prev = self.iter.clone().next();
        self.iter.next()
    }

    /// Access the element returned last
    fn prev(&self) -> Option<&Token> {
        self.prev.as_ref()
    }

    /// Advance the iterator, erroring if EOF occurs prematurely
    fn advance(&mut self) -> Result<Token> {
        self.step().ok_or(SyntaxError::UnexpectedEOF.into())
    }

    /// Peek the iterator, erroring if EOF occurs early
    fn peer(&mut self) -> Result<Token> {
        let mut iter = self.iter.clone();
        iter.next().ok_or(SyntaxError::UnexpectedEOF.into())
    }

    /// Consumes the next item, verifing that it is the right value
    ///
    /// Does not advance the underlying iterator if predicate not fulfilled
    fn eat(&mut self, expected: TokenKind) -> Option<Token> {
        if self.iter.peek()?.kind == expected {
            return Some(self.advance().unwrap());
        }
        None
    }

    /// Peeks the next item, verifing that it is the right value
    fn taste(&mut self, expected: TokenKind) -> Result<bool> {
        Ok(self.peer()?.kind == expected)
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
