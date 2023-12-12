use std::collections::HashMap;

use crate::{
    dump,
    error::RuntimeError,
    token::{Token, Value},
};

#[derive(Debug)]
pub struct Env {
    env: HashMap<Token, Option<Value>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    /// Defines a new variable using an identifier as its name
    pub fn define(&mut self, ident: Token, value: Option<Value>) {
        self.env.insert(ident, value);
    }

    /// Get a variable based on an identifier
    ///
    /// The outer `Option` refers to whether the variable exists in the first place or not. The
    /// inner `Option` denotes whether the variable has an associated value or not.
    pub fn get(&self, ident: &Token) -> Option<&Option<Value>> {
        self.env.get(ident)
    }

    /// Assigns a value to a variable, overwriting the previous value.
    ///
    /// Errors if the assignment target is uninitialised / doesn't exist
    pub fn assign(&mut self, ident: Token, value: Value) -> color_eyre::Result<Value> {
        if self.env.contains_key(&ident) {
            self.env.insert(ident, Some(value.clone()));
            Ok(value)
        } else {
            dump!(RuntimeError::UninitialisedVar(ident.lex()))
        }
    }
}
