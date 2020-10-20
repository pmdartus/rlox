use crate::ast::LiteralValue;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil | Object::False => false,
            Object::True | _ => true,
        }
    }
}

impl From<&LiteralValue> for Object {
    fn from(value: &LiteralValue) -> Self {
        match value {
            LiteralValue::Number(value) => Object::Number(*value),
            LiteralValue::String(value) => Object::String(String::from(value)),
            LiteralValue::True => Object::True,
            LiteralValue::False => Object::False,
            LiteralValue::Nil => Object::Nil,
        }
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        match value {
            true => Object::True,
            false => Object::False,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Number(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "{}", value),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
            Object::Nil => write!(f, "nil"),
        }
    }
}
