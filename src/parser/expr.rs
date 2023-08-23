use color_eyre::Result;

use super::Parser;
use crate::{ast::Expr, error::SyntaxError, impetuous::Impetuous, token::TokenType};

impl Parser {
    pub fn expression(&mut self) -> Result<Expr> {
        self.expr(0)
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr> {
        let mut left = match self.peer()?.token_type {
            TokenType::Number | TokenType::String | TokenType::True | TokenType::False => {
                Expr::Literal(self.advance()?)
            }
            TokenType::Identifier => Expr::Variable(self.peer()?),
            TokenType::LeftParen => {
                self.advance()?; // consume LeftParen
                let inner = self.expr(0)?;

                if self.peer()?.token_type != TokenType::RightParen {
                    return Err(SyntaxError::ExpectedCharacter {
                        expected: ')',
                        found: self.advance()?.lexeme,
                    }
                    .into());
                }
                self.advance()?; // consume RightParen

                Expr::Grouping(Box::new(inner))
            }
            TokenType::Minus | TokenType::Bang => {
                let op = self.advance()?;
                let (_, r_bp) = prefix_bp(&op.token_type);
                let right = self.expr(r_bp)?;
                Expr::Unary(op, Box::new(right))
            }
            x => unimplemented!("{x:?}"),
        };

        while let Some(op) = self.iter.peek() {
            if let Some((l_bp, r_bp)) = infix_bp(&op.token_type) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.advance()?; // consume operator
                let right = self.expr(r_bp)?;

                left = Expr::Binary(Box::new(left), op, Box::new(right));
                continue;
            }

            break;
        }

        Ok(left)
    }
}

/// Returns the binding power for an infix operator
fn infix_bp(op: &TokenType) -> Option<(u8, u8)> {
    let bp = match op {
        TokenType::EqualEqual => (2, 1),
        TokenType::Plus | TokenType::Minus => (3, 4),
        TokenType::Star | TokenType::Slash => (5, 6),
        _ => return None,
    };

    Some(bp)
}

/// Returns the binding power of a prefix operator
fn prefix_bp(op: &TokenType) -> ((), u8) {
    match op {
        TokenType::Minus | TokenType::Bang => ((), 7),
        _ => panic!("bad op: {:?}", op),
    }
}
