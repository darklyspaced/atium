use color_eyre::Result;

use super::{
    ast::{Expr, Stmt},
    error::SyntaxError,
    token::{Token, TokenType},
};
use std::{iter::Peekable, result, vec::IntoIter};

pub(super) struct Parser {
    /// peekable iterator over tokens
    pub iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self {
            iter: token_stream.into_iter().peekable(),
        }
    }

    /// Advance the iterator, erroring if EOF occurs early -- while an expression was expected
    fn advance(&mut self) -> Result<Token> {
        self.iter
            .next()
            .ok_or_else(|| SyntaxError::NoExpression.into())
    }

    /// Peek the iterator, erroring if EOF occurs early -- while an expression was expected
    fn peer(&mut self) -> Result<&Token> {
        self.iter
            .peek()
            .ok_or_else(|| SyntaxError::NoExpression.into())
    }
}

impl Parser {
    /// Prevents error cascading (one error causing a bunch of other ones later in the program)
    ///
    /// Discards tokens until the next statement is reached. Invoked when an error is thrown while
    /// parsing. Discarded tokens were part of a statement that caused an error and therefore were
    /// most likely erroneous themselves.
    fn sync(&mut self, prev: &TokenType) -> Option<()> {
        self.iter.next()?;

        if prev == &TokenType::Semicolon {
            return Some(());
        }

        for next in self.iter.by_ref() {
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
                _ => (),
            }
        }
        Some(())
    }

    /// Converts a stream of tokens into an abstract syntax tree
    ///
    /// # Errors
    /// This functions only produces [`SyntaxError`] that describe the errors that were exhibited.
    /// Any production of errors here, aborts the interpretation procedure as to not cascade.
    ///
    /// This function should never panic as None should never be returned from the iterator.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<color_eyre::Report>> {
        let mut statements: Vec<Result<Stmt>> = vec![];

        while let Some(peek_next) = self.iter.peek() {
            if peek_next.token_type == TokenType::Print {
                self.iter.next(); // consume print token
                let print = self.expression().map(Stmt::Print);
                let Some(semi) = self.iter.next() else {
                    statements.push(Err(SyntaxError::ExpectedCharacter(
                        String::from("EOF"),
                        ';',
                    )
                    .into()));
                    break;
                };

                if !matches!(semi.token_type, TokenType::Semicolon) {
                    statements.push(Err(SyntaxError::ExpectedCharacter(semi.lexeme, ';').into()));
                }

                statements.push(print);
            } else {
                let expression = self.expression().map(Stmt::Expr);
                statements.push(expression);
            }
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

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut left = self.comparison()?;
        let mut next = self.peer()?;

        while let TokenType::EqualEqual | TokenType::BangEqual = next.token_type {
            let operator = self.advance()?;
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.peer()?;
        }

        Ok(left)
    }

    /// Resolves into an [`Expr::Binary`] that represents a sequence of comparisons between two
    /// literals
    fn comparison(&mut self) -> Result<Expr> {
        let mut left = self.term()?;
        let mut next = self.peer()?;

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = next.token_type
        {
            let operator = self.advance()?;
            let right = self.term()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.peer()?;
        }

        Ok(left)
    }

    /// Resolves into an [`Expr::Binary`] that represents a sequence of additions and subtractions
    fn term(&mut self) -> Result<Expr> {
        let mut left = self.factor()?;
        let mut next = self.iter.peek().unwrap();

        while let TokenType::Plus | TokenType::Minus = next.token_type {
            let operator = self.iter.next().unwrap();
            let right = self.factor()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek().unwrap();
        }

        Ok(left)
    }

    /// Resolves into an [`Expr::Binary`] that represents a sequence of muliplications and
    /// divisions
    fn factor(&mut self) -> Result<Expr> {
        let mut left = self.unary()?;
        let mut next = self.peer()?;

        while let TokenType::Star | TokenType::Slash = next.token_type {
            let operator = self.iter.next().unwrap();
            let right = self.unary()?;

            left = Expr::Binary(Box::new(left), operator, Box::new(right));
            next = self.iter.peek().unwrap();
        }

        Ok(left)
    }

    /// Resolves into an [`Expr::Unary`] that represents a literal with an operator applied to it
    fn unary(&mut self) -> Result<Expr> {
        let next = self.peer()?;

        match next.token_type {
            TokenType::Bang | TokenType::Minus => {
                let next = self.advance()?;

                Ok(Expr::Unary(next, Box::new(self.unary()?)))
            }
            TokenType::Plus => {
                color_eyre::eyre::bail!("'+' cannot be used as a unary operator")
            }
            _ => self.primary(),
        }
    }

    /// Resolves into a [`Expr::Literal`] that represents, you guessed it, a literal.
    fn primary(&mut self) -> Result<Expr> {
        let next = self.peer()?;

        match next.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Str
            | TokenType::Number => {
                let next = self.advance()?;
                Ok(Expr::Literal(next))
            }
            TokenType::LeftParen => {
                self.advance()?; // left paren
                let expr = self.expression()?;

                let next = self.advance()?;
                if TokenType::RightParen == next.token_type {
                    return Ok(Expr::Grouping(Box::new(expr)));
                }

                Err(SyntaxError::ExpectedCharacter(next.lexeme, ')').into())
            }
            _ => Err(SyntaxError::NoExpression.into()),
        }
    }
}
