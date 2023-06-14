use super::{
    ast::Expr,
    token::{Token, TokenType},
};
use std::{iter::Peekable, vec::IntoIter};

pub struct Parser {
    iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            iter: tokens.into_iter().peekable(),
        }
    }
    pub fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut left = self.comparison()?;
        let mut next = self.iter.peek().unwrap();

        loop {
            match next.token_type {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    let operator = self.iter.next()?;
                    let right = self.comparison()?;
                    left = Expr::Binary(Box::new(left), operator, Box::new(right));
                    next = self.iter.peek()?;
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut left = self.term()?;
        let mut next = self.iter.peek()?;

        loop {
            match next.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    println!("comparison");
                    let operator = self.iter.next()?;
                    let right = self.term()?;
                    left = Expr::Binary(Box::new(left), operator, Box::new(right));
                    next = self.iter.peek()?;
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut left = self.factor()?;
        let mut next = self.iter.peek()?;

        loop {
            match next.token_type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = self.iter.next()?;
                    let right = self.factor()?;
                    left = Expr::Binary(Box::new(left), operator, Box::new(right));
                    next = self.iter.peek()?;
                }
                _ => break,
            }
        }

        Some(left)
    }

    /// Resolves into an [`Expr::Binary`] that represents a sequence of muliplications and
    /// divisions
    fn factor(&mut self) -> Option<Expr> {
        let mut left = self.unary()?;
        let mut next = self.iter.peek()?;

        loop {
            match next.token_type {
                TokenType::Star | TokenType::Slash => {
                    let operator = self.iter.next()?;
                    let right = self.unary()?;
                    left = Expr::Binary(Box::new(left), operator, Box::new(right));
                    next = self.iter.peek()?;
                }
                _ => break,
            }
        }

        Some(left)
    }

    /// Resolves into an [`Expr::Unary`] that represents a negated literal
    fn unary(&mut self) -> Option<Expr> {
        let next = self.iter.peek()?;
        match next.token_type {
            TokenType::Bang | TokenType::Minus => {
                let next = self.iter.next()?;

                Some(Expr::Unary(next, Box::new(self.unary()?)))
            }
            _ => self.primary(),
        }
    }

    /// Resolves into a [`Expr::Literal`] that represents, you guessed it, a literal
    fn primary(&mut self) -> Option<Expr> {
        let next = self.iter.peek().unwrap();
        match next.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Str
            | TokenType::Number => {
                let next = self.iter.next()?;
                Some(Expr::Literal(next))
            }
            TokenType::LeftParen => {
                self.iter.next()?; // left paren
                let expr = self.expression()?;

                if self.iter.peek()?.token_type != TokenType::RightParen {
                    panic!("expected ')' after expression")
                }
                self.iter.next()?; // right paren

                return Some(Expr::Grouping(Box::new(expr)));
            }
            _ => panic!("this should not have happened..."),
        }
    }
}
