use std::collections::HashMap;

use crate::ast::Token;
use crate::object::Object;
use crate::result::{Error, RloxResult};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, id: &Token, value: Object) {
        self.values.insert(id.lexeme.to_owned(), value);
    }

    pub fn get(&self, id: &Token) -> RloxResult<Object> {
        match self.values.get(&id.lexeme) {
            Some(val) => Ok((*val).clone()),
            None => Err(Error::Runtime(
                id.line,
                format!("Undefined variable '{}'.", id.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, id: &Token, value: Object) -> RloxResult<Object> {
        match self.values.insert(id.lexeme.to_owned(), value) {
            Some(_) => Ok(self.values.get(&id.lexeme).unwrap().clone()),
            None => Err(Error::Runtime(
                id.line,
                format!("Undefined variable '{}'.", id.lexeme),
            )),
        }
    }
}
