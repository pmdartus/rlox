use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Token;
use crate::object::Object;
use crate::result::{Error, RloxResult};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: RefCell<HashMap<String, Object>>,
}

impl Environment {
    pub fn new() -> Rc<Self> {
        Self::init(None)
    }

    pub fn from(parent: &Rc<Self>) -> Rc<Self> {
        let enclosing = Some(Rc::clone(parent));
        Self::init(enclosing)
    }

    fn init(parent: Option<Rc<Self>>) -> Rc<Self> {
        Rc::new(Self {
            enclosing: parent,
            values: RefCell::new(HashMap::new()),
        })
    }

    pub fn define(&self, id: &Token, value: Object) {
        self.values.borrow_mut().insert(id.lexeme.to_owned(), value);
    }

    pub fn get(&self, id: &Token) -> RloxResult<Object> {
        if let Some(val) = self.values.borrow().get(&id.lexeme) {
            return Ok((*val).clone());
        }

        match &self.enclosing {
            Some(enclosing) => enclosing.get(id),
            _ => Err(Error::Runtime(
                id.line,
                format!("Undefined variable '{}'.", id.lexeme),
            )),
        }
    }

    pub fn assign(&self, id: &Token, value: Object) -> RloxResult<Object> {
        let mut values = self.values.borrow_mut();

        if values.get(&id.lexeme).is_some() {
            values.insert(id.lexeme.to_owned(), value.to_owned());
            return Ok(value);
        }

        match &self.enclosing {
            Some(enclosing) => enclosing.assign(id, value),
            _ => Err(Error::Runtime(
                id.line,
                format!("Undefined variable '{}'.", id.lexeme),
            )),
        }
    }
}
