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
            TokenKind::Identifier => Expr::Variable(self.peer()?),
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
            x => unimplemented!("{x:?}"),
        };

        while let Some(op) = self.iter.peek() {
            if let Some((l_bp, r_bp)) = infix_bp(&op.kind) {
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
fn infix_bp(op: &TokenKind) -> Option<(u8, u8)> {
    let bp = match op {
        TokenKind::EqualEqual => (2, 1),
        TokenKind::Plus | TokenKind::Minus => (3, 4),
        TokenKind::Star | TokenKind::Slash => (5, 6),
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
