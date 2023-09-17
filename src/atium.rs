use crate::{interpreter::Interpreter, lexer::Cursor, parser::Parser};
use std::{marker::PhantomData, path::Path};

use color_eyre::{Report, Result};

pub struct Lexing;
pub struct Parsing;
pub struct Interpreting;

type AResult<'a, T> = Result<Atium<'a, T>, Vec<Report>>;

/// Internal logic for the language
pub struct Atium<'a, State = Lexing> {
    /// cursor that travrses source code and tokenises it
    cursor: Cursor<'a>,
    /// parser that converts tokens into AST
    parser: Parser,
    /// interpreter that walks AST, computing it
    interpeter: Interpreter,
    /// state of the program
    state: PhantomData<State>,
}

impl<'a> Atium<'a> {
    pub fn new(src: &'a str, file: Option<&str>) -> Self {
        Self {
            cursor: Cursor::new(src, file),
            parser: Parser::new(Vec::default()), // NOTE: should not be used until State = Parsing
            interpeter: Interpreter::new(Vec::default()), // NOTE: don't use if State != Interpret
            state: PhantomData::<Lexing>,
        }
    }
}

impl<State> Atium<'_, State> {
    pub fn report(&self, errs: &[color_eyre::Report]) {
        if !errs.is_empty() {
            for err in errs {
                println!("Error: {err}"); // TODO: pretty printing of errors
            }
        }
    }
}

impl<'a> Atium<'a, Lexing> {
    pub fn lex(self) -> AResult<'a, Parsing> {
        self.cursor.lex().map(|ok| Atium {
            state: PhantomData::<Parsing>,
            parser: Parser::new(ok),
            cursor: Cursor::new::<&str>("", None),
            interpeter: Interpreter::new(vec![]),
        })
    }
}

impl<'a> Atium<'a, Parsing> {
    pub fn parse(mut self) -> AResult<'a, Interpreting> {
        self.parser.parse().map(|ok| {
            println!("{}", serde_json::to_string_pretty(&ok).unwrap());
            Atium {
                state: PhantomData::<Interpreting>,
                interpeter: Interpreter::new(ok),
                parser: Parser::new(vec![]),
                cursor: Cursor::new::<&str>("", None),
            }
        })
    }
}

impl<'a> Atium<'a, Interpreting> {
    pub fn interpret(self) -> Result<(), Vec<Report>> {
        self.interpeter.interpret()
    }
}
