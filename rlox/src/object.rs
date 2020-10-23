use std::{cmp, fmt};

use crate::ast::LiteralValue;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Object::Nil | Object::False)
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

impl cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::True, Object::True)
            | (Object::False, Object::False)
            | (Object::Nil, Object::Nil) => true,
            (Object::Number(a), Object::Number(b)) if a.eq(b) => true,
            (Object::String(a), Object::String(b)) if a.eq(b) => true,
            _ => false,
        }
    }
}

impl cmp::PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Object::True, Object::True)
            | (Object::False, Object::False)
            | (Object::Nil, Object::Nil) => Some(cmp::Ordering::Equal),
            (Object::Number(a), Object::Number(b)) => a.partial_cmp(b),
            (Object::String(a), Object::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
