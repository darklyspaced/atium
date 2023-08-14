use color_eyre::Result;

use super::{
    super::{
        ast::{Expr, Stmt},
        error::SyntaxError,
        token::TokenType,
    },
    Parser,
};

impl Parser {
    pub(super) fn declaration(&mut self) -> Result<Stmt> {
        match self.peer()?.token_type {
            TokenType::Var => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        color_eyre::eyre::bail!("");
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.peer()?.token_type {
            TokenType::Print => {
                self.iter.next(); // consume print token
                let print = self.expression().map(Stmt::Print)?;

                let Some(semi) = self.iter.next() else {
                 return Err(SyntaxError::ExpectedCharacter(
                        String::from("EOF"),
                        ';',
                    )
                    .into());
                };

                if semi.token_type != TokenType::Semicolon {
                    return Err(SyntaxError::ExpectedCharacter(semi.lexeme, ';').into());
                }

                Ok(print)
            }
            _ => {
                self.iter.next();
                self.expression().map(Stmt::Expr)
            }
        }
    }

    /// Consume the expression, leaving the iterator after the expression
    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        self.repeat_op(
            Parser::comparison,
            &[TokenType::EqualEqual, TokenType::BangEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr> {
        self.repeat_op(
            Parser::term,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn term(&mut self) -> Result<Expr> {
        self.repeat_op(Parser::factor, &[TokenType::Plus, TokenType::Minus])
    }

    fn factor(&mut self) -> Result<Expr> {
        self.repeat_op(Parser::unary, &[TokenType::Star, TokenType::Slash])
    }

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

    fn primary(&mut self) -> Result<Expr> {
        match self.peer()?.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::String
            | TokenType::Number => Ok(Expr::Literal(self.advance()?)),
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
