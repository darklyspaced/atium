use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    dump,
    error::RuntimeError,
    token::{Token, Value},
};

#[derive(Debug, Clone)]
pub struct Env {
    env: HashMap<Token, Option<Rc<RefCell<Value>>>>,
    parent: Option<RefCell<Box<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            parent: None,
        }
    }

    /// Defines a new variable using an identifier as its name
    pub fn define(&mut self, ident: Token, value: Option<Value>) {
        self.env
            .insert(ident, value.map(|inner| Rc::new(RefCell::new(inner))));
    }

    /// Get a variable based on an identifier
    ///
    /// Keeps recursively checking outer scopes until it finds a variable or errors. Outer scopes
    /// can be accessed, inner scopes cannot.
    ///
    /// The outer `Option` refers to whether the variable exists in the first place or not. The
    /// inner `Option` denotes whether the variable has an associated value or not.
    pub fn get(&self, ident: &Token) -> Option<Option<Value>> {
        match self.env.get(ident) {
            None => self
                .parent
                .as_ref()
                .map_or(None, |inner| inner.borrow().get(ident)),
            Some(val) => Some(val.as_ref().map(|x| x.borrow().clone())),
        }
    }

    /// Assigns a value to a variable, overwriting the previous value.
    ///
    /// Keeps recursively checking outer scopes until it finds a variable or errors. Outer scopes
    /// can be accessed, inner scopes cannot.
    ///
    /// Errors if the assignment target is undefined
    pub fn assign(&mut self, ident: Token, value: Value) -> color_eyre::Result<Value> {
        if self.env.contains_key(&ident) {
            self.env
                .insert(ident, Some(Rc::new(RefCell::new(value.clone()))));
            Ok(value)
        } else {
            self.parent.as_ref().map_or_else(
                || dump!(RuntimeError::InvalidAssignmentTarget::<String>),
                |outer| outer.borrow_mut().assign(ident, value),
            )
        }
    }

    pub fn set_parent(&mut self, parent: Env) {
        self.parent = Some(RefCell::new(Box::new(parent)));
    }
}
