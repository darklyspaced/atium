use std::{iter::Peekable, result, vec::IntoIter};

use color_eyre::Result;

use super::{
    ast::{Expr, Stmt},
    error::SyntaxError,
    token::{Token, TokenType},
};

mod pratt;

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

    /// Converts a stream of tokens into an abstract syntax tree
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<color_eyre::Report>> {
        let mut statements: Vec<Result<Stmt>> = vec![];

        while let Some(_) = self.iter.peek() {
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

    /// Advance the iterator, erroring if EOF occurs early -- while an expression was expected
    fn advance(&mut self) -> Result<Token> {
        self.iter
            .next()
            .ok_or_else(|| SyntaxError::UnexpectedEOF.into())
    }

    /// Peek the iterator, erroring if EOF occurs early -- while an expression was expected
    fn peer(&mut self) -> Result<&Token> {
        self.iter
            .peek()
            .ok_or_else(|| SyntaxError::UnexpectedEOF.into())
    }

    fn repeat_op(
        &mut self,
        func: impl Fn(&mut Parser) -> Result<Expr>,
        tt: &[TokenType],
    ) -> Result<Expr> {
        let mut left = func(self)?;

        loop {
            match self.iter.peek() {
                Some(token) => {
                    if tt.contains(&token.token_type) {
                        let operator = self.advance()?;
                        let right = func(self)?;
                        left = Expr::Binary(Box::new(left), operator, Box::new(right));
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        Ok(left)
    }

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
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Cursor, parser::Parser};
    #[test]
    fn add_sub() {
        let src = "print 10+5--4+1;";
        let cursor = Cursor::new(src);
        let tokens = cursor.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        let found = serde_json::to_string_pretty(&result).unwrap();
        let expected = r#"[
  {
    "Print": {
      "Binary": [
        {
          "Binary": [
            {
              "Binary": [
                {
                  "Literal": {
                    "token_type": "Number",
                    "lexeme": "10",
                    "literal": {
                      "Integer": 10
                    },
                    "line": 0
                  }
                },
                {
                  "token_type": "Plus",
                  "lexeme": "+",
                  "literal": null,
                  "line": 0
                },
                {
                  "Literal": {
                    "token_type": "Number",
                    "lexeme": "5",
                    "literal": {
                      "Integer": 5
                    },
                    "line": 0
                  }
                }
              ]
            },
            {
              "token_type": "Minus",
              "lexeme": "-",
              "literal": null,
              "line": 0
            },
            {
              "Unary": [
                {
                  "token_type": "Minus",
                  "lexeme": "-",
                  "literal": null,
                  "line": 0
                },
                {
                  "Literal": {
                    "token_type": "Number",
                    "lexeme": "4",
                    "literal": {
                      "Integer": 4
                    },
                    "line": 0
                  }
                }
              ]
            }
          ]
        },
        {
          "token_type": "Plus",
          "lexeme": "+",
          "literal": null,
          "line": 0
        },
        {
          "Literal": {
            "token_type": "Number",
            "lexeme": "1",
            "literal": {
              "Integer": 1
            },
            "line": 0
          }
        }
      ]
    }
  }
]"#;
        assert_eq!(expected, found);
    }

    #[test]
    pub fn grouping() {
        let src = "print (10+5)*3/(2--7);";
        let cursor = Cursor::new(src);
        let tokens = cursor.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        let found = serde_json::to_string_pretty(&result).unwrap();
        let expected = r#"[
  {
    "Print": {
      "Binary": [
        {
          "Binary": [
            {
              "Grouping": {
                "Binary": [
                  {
                    "Literal": {
                      "token_type": "Number",
                      "lexeme": "10",
                      "literal": {
                        "Integer": 10
                      },
                      "line": 0
                    }
                  },
                  {
                    "token_type": "Plus",
                    "lexeme": "+",
                    "literal": null,
                    "line": 0
                  },
                  {
                    "Literal": {
                      "token_type": "Number",
                      "lexeme": "5",
                      "literal": {
                        "Integer": 5
                      },
                      "line": 0
                    }
                  }
                ]
              }
            },
            {
              "token_type": "Star",
              "lexeme": "*",
              "literal": null,
              "line": 0
            },
            {
              "Literal": {
                "token_type": "Number",
                "lexeme": "3",
                "literal": {
                  "Integer": 3
                },
                "line": 0
              }
            }
          ]
        },
        {
          "token_type": "Slash",
          "lexeme": "/",
          "literal": null,
          "line": 0
        },
        {
          "Grouping": {
            "Binary": [
              {
                "Literal": {
                  "token_type": "Number",
                  "lexeme": "2",
                  "literal": {
                    "Integer": 2
                  },
                  "line": 0
                }
              },
              {
                "token_type": "Minus",
                "lexeme": "-",
                "literal": null,
                "line": 0
              },
              {
                "Unary": [
                  {
                    "token_type": "Minus",
                    "lexeme": "-",
                    "literal": null,
                    "line": 0
                  },
                  {
                    "Literal": {
                      "token_type": "Number",
                      "lexeme": "7",
                      "literal": {
                        "Integer": 7
                      },
                      "line": 0
                    }
                  }
                ]
              }
            ]
          }
        }
      ]
    }
  }
]"#;
        assert_eq!(expected, found);
    }
}
