use crate::dump;
use crate::error::RuntimeError;
use color_eyre::Result;

use super::Parser;
use crate::{ast::Expr, error::SyntaxError, impetuous::Impetuous, token::TokenKind};

impl Parser {
    pub fn expression(&mut self) -> Result<Expr> {
        self.expr(0)
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr> {
        let mut left = match self.peer()?.kind {
            TokenKind::Number | TokenKind::String | TokenKind::True | TokenKind::False => {
                Expr::Literal(self.advance()?)
            }
            TokenKind::Identifier => Expr::Variable(self.advance()?), // NOTE variables are not
            // only one character
            TokenKind::LeftParen => {
                self.advance()?; // consume LeftParen
                let inner = self.expr(0)?;

                if self.peer()?.kind != TokenKind::RightParen {
                    return Err(SyntaxError::ExpectedCharacter {
                        expected: ')',
                        found: self.advance()?.span.lex,
                    }
                    .into());
                }
                self.advance()?; // consume RightParen

                Expr::Grouping(Box::new(inner))
            }
            TokenKind::Minus | TokenKind::Bang => {
                let op = self.advance()?;
                let (_, r_bp) = prefix_bp(&op.kind);
                let right = self.expr(r_bp)?;
                Expr::Unary(op, Box::new(right))
            }
            x => {
                dbg!(&self.iter);
                unimplemented!("{x:?}")
            }
        };

        while let Some(op) = self.iter.peek() {
            if let Some((l_bp, r_bp)) = infix_bp(&op.kind) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.advance()?; // consume operator
                let right = self.expr(r_bp)?;

                left = match op.kind {
                    TokenKind::Equal => {
                        if let Expr::Variable(name) = left {
                            Expr::Assignment(name, Box::new(right))
                        } else {
                            dump!(RuntimeError::InvalidAssignmentTarget::<String>)
                        }
                    }
                    _ => Expr::Binary(Box::new(left), op, Box::new(right)),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }
}

/// Returns the binding power for an infix operator
fn infix_bp(op: &TokenKind) -> Option<(u8, u8)> {
    let bp = match op {
        TokenKind::Equal => (2, 1),
        TokenKind::EqualEqual => (4, 3),
        TokenKind::Plus | TokenKind::Minus => (5, 6),
        TokenKind::Star | TokenKind::Slash => (7, 8),
        _ => return None,
    };

    Some(bp)
}

/// Returns the binding power of a prefix operator
fn prefix_bp(op: &TokenKind) -> ((), u8) {
    match op {
        TokenKind::Minus | TokenKind::Bang => ((), 7),
        _ => panic!("bad op: {:?}", op),
    }
}
