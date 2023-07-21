use super::{
    ast::{Expr, Stmt},
    error::SyntaxError,
    token::{Token, TokenType},
};
use std::{iter::Peekable, vec::IntoIter};

/// Holds the iterator for the stream that the parser is processing
pub struct Parser {
    iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            iter: tokens.into_iter().peekable(),
        }
    }

    /// Prevents error cascading (one error causing a bunch of other ones later in the program)
    ///
    /// Discards tokens until the next statement is reached. Invoked when an error is thrown while
    /// parsing. Discarded tokens were part of a statement that caused an error and therefore were
    /// most likely erroneous themselves.
    fn sync(&mut self, prev: &TokenType) -> Option<()> {
        let mut next = self.iter.next()?;

        if prev == &TokenType::Semicolon {
            return Some(());
        }

        while next.token_type != TokenType::EOF {
            match next.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return Some(());
                }
                _ => next = self.iter.next()?,
            }
        }
        Some(())
    }

    /// Converts a stream of tokens into an abstract syntax tree that can be interpreted
    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        let mut next = self.iter.peek()?.clone();

        loop {
            match next.token_type {
                TokenType::Print => statements.push({
                    self.iter.next(); // consume print token
                    let print = Stmt::Print(self.expression()?); // consume argument to print
                    let semi = self.iter.next()?; // expecting semicolon

                    match semi.token_type {
                        TokenType::EOF => {
                            println!(
                                "{}",
                                SyntaxError::ExpectedCharacter(String::from("EOF"), ';')
                            );
                            return None;
                        }
                        TokenType::Semicolon => (),
                        _ => {
                            println!("{}", SyntaxError::ExpectedCharacter(semi.lexeme, ';'));
                            return None;
                        }
                    }
                    next = self.iter.peek()?.clone();
                    print
                }),
                TokenType::EOF => break,
                _ => {
                    let expression = self.expression()?;
                    statements.push(Stmt::Expr(expression));
                    next = self.iter.peek()?.clone();
                }
            }
        }

        // dbg!(&statements);
        Some(statements)
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut left = self.comparison()?;
        let mut next = self.iter.peek()?;

        while let TokenType::EqualEqual | TokenType::BangEqual = next.token_type {
            let operator = self.iter.next()?;
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek()?;
        }

        Some(left)
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut left = self.term()?;
        let mut next = self.iter.peek()?;

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = next.token_type
        {
            let operator = self.iter.next()?;
            let right = self.term()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek()?;
        }

        Some(left)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut left = self.factor()?;
        let mut next = self.iter.peek()?;

        while let TokenType::Plus | TokenType::Minus = next.token_type {
            let operator = self.iter.next()?;
            let right = self.factor()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek()?;
        }

        Some(left)
    }

    /// Resolves into an [`Expr::Binary`] that represents a sequence of muliplications and
    /// divisions
    fn factor(&mut self) -> Option<Expr> {
        let mut left = self.unary()?;
        let mut next = self.iter.peek()?;

        while let TokenType::Star | TokenType::Slash = next.token_type {
            let operator = self.iter.next()?;
            let right = self.unary()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek()?;
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
            TokenType::Plus => {
                panic!("cannot use '+' as an operator in a unary expression")
            }
            _ => self.primary(),
        }
    }

    /// Resolves into a [`Expr::Literal`] that represents, you guessed it, a literal.
    fn primary(&mut self) -> Option<Expr> {
        let next = self.iter.peek()?;

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

                assert!(
                    !(self.iter.peek()?.token_type != TokenType::RightParen),
                    "expected ')' after expression. error occured on line {}",
                    self.iter.peek()?.line
                );
                self.iter.next()?; // right paren

                Some(Expr::Grouping(Box::new(expr)))
            }
            _ => panic!("expected an expression"),
        }
    }
}
