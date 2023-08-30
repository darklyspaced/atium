use std::{iter::Peekable, result, vec::IntoIter};

use color_eyre::Result;

use crate::{dump, error::SyntaxError};

use super::{
    ast::Stmt,
    impetuous::Impetuous,
    token::{Token, TokenType},
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
        match self.peer()?.token_type {
            TokenType::Var => {
                self.advance()?; // consume Var tok
                match self.var_decl() {
                    Ok(stmt) => Ok(stmt),
                    Err(e) => {
                        let prev = &self.prev().unwrap().token_type.clone();
                        self.recover(prev);
                        Err(e)
                    }
                }
            }
            _ => self.statement().map_err(|e| {
                if let Some(prev) = &self.prev() {
                    self.recover(&prev.token_type.clone());
                }
                e
            }),
        }
    }

    fn var_decl(&mut self) -> Result<Stmt> {
        let Some(ident) = self.eat(TokenType::Identifier) else {
            match self.prev() {
                Some(tok) => {
                    return Err(SyntaxError::ExpectedIdent(String::from(&tok.lexeme)).into())
                }
                None => return dump!(SyntaxError::ExpectedIdent(String::from("EOF"))),
                // None => return Err(SyntaxError::ExpectedIdent(String::from("EOF")).into()),
            }
        };

        let initial_value = if self.taste(TokenType::Equal)? {
            self.advance()?; // consume the Equal
            Some(self.expression().unwrap())
        } else {
            None
        };

        if self.eat(TokenType::Semicolon).is_none() {
            return Err(SyntaxError::ExpectedCharacter {
                found: self
                    .prev()
                    .map_or_else(|| String::from("EOF"), |tok| String::from(&tok.lexeme)),
                expected: ';',
            }
            .into());
        }

        Ok(Stmt::Var {
            name: ident,
            value: initial_value,
        })
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.peer()?.token_type {
            TokenType::Print => {
                self.advance()?; // consume the print token
                let expr = self.expression()?;

                match self.step() {
                    Some(tok) => {
                        if tok.token_type != TokenType::Semicolon {
                            return Err(SyntaxError::ExpectedCharacter {
                                expected: ';',
                                found: self.advance()?.lexeme,
                            }
                            .into());
                        }
                        Ok(Stmt::Print(expr))
                    }
                    None => Err(SyntaxError::ExpectedCharacter {
                        expected: ';',
                        found: String::from("EOF"),
                    }
                    .into()),
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
    fn recover(&mut self, prev: &TokenType) -> Option<()> {
        self.step()?;

        if *prev == TokenType::Semicolon {
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

// make this all generic behaviour that can be overriden if need be.
impl Impetuous for Parser {
    type Scrutinee = TokenType;
    /// Override the `next` function on Iterators, advancing one step forwards
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
    fn eat(&mut self, expected: TokenType) -> Option<Token> {
        if self.iter.peek()?.token_type == expected {
            return Some(self.advance().unwrap());
        }
        None
    }

    /// Peeks the next item, verifing that it is the right value
    fn taste(&mut self, expected: TokenType) -> Result<bool> {
        Ok(self.peer()?.token_type == expected)
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Cursor, parser::Parser};
    #[test]
    fn add_sub() {
        let src = "print 10+5--4+1;";
        let cursor = Cursor::new(src);
        let tokens = cursor.lex().unwrap();

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
        let tokens = cursor.lex().unwrap();

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
