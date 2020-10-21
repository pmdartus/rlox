use std::collections::HashMap;

use crate::object::Object;
use crate::scanner::{Token};
use crate::result::{RloxResult, Error};

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
            Some(_) => unimplemented!(),
            None => Err(Error::Runtime(id.line, format!("Undefined variable '{}'.", id.lexeme))),
        }
    }
}
