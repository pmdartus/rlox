use std::collections::HashMap;

use crate::object::Object;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&Object, String> {
        match self.values.get(name) {
            Some(obj) => Ok(obj),
            None => Err(format!("Undefined variable '{}'.", name)),
        }
    }
}
