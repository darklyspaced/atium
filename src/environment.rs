use std::collections::HashMap;

use crate::token::{Token, Value};

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
}
