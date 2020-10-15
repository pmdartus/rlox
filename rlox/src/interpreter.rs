use std::f32;
use std::ops::{Neg, Not};

use crate::ast::{BinaryOp, Expr, LiteralValue, UnaryOp, Visitor};

#[derive(Debug)]
pub enum Object {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Not for Object {
    type Output = Object;

    fn not(self) -> Self::Output {
        match self {
            Object::True => Object::False,
            Object::False => Object::True,
            Object::Nil => Object::True,
            Object::Number(value) => {
                if value == 0.0 {
                    Object::True
                } else {
                    Object::False
                }
            }
            Object::String(value) => {
                if value.len() == 0 {
                    Object::True
                } else {
                    Object::False
                }
            }
        }
    }
}

impl Neg for Object {
    type Output = Object;

    fn neg(self) -> Self::Output {
        match self {
            Object::Number(value) => Object::Number(value.neg()),
            Object::True => Object::Number(-1.0),
            Object::False => Object::Number(-0.0),
            Object::Nil | Object::String(_) => Object::Number(f32::NAN),
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

pub struct Interpret {}

impl Interpret {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Object {
        expr.accept(self)
    }
}

impl Visitor<Object> for Interpret {
    fn visit_binary(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> Object {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match op {
            BinaryOp::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Object::Number(a + b),
                (Object::String(a), Object::String(b)) => Object::String(format!("{}{}", a, b)),
                (_, _) => unimplemented!(),
            },
            BinaryOp::Minus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Object::Number(a - b),
                (_, _) => unimplemented!(),
            },
            BinaryOp::Star => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Object::Number(a * b),
                (_, _) => unimplemented!(),
            },
            BinaryOp::Slash => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Object::Number(a / b),
                (_, _) => unimplemented!(),
            },
            BinaryOp::Equal => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a == b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
            BinaryOp::NotEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a != b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
            BinaryOp::Greater => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a > b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a >= b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
            BinaryOp::Less => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a < b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a <= b {
                        Object::True
                    } else {
                        Object::False
                    }
                }
                (_, _) => unimplemented!(),
            },
        }
    }

    fn visit_unary(&mut self, op: &UnaryOp, right: &Expr) -> Object {
        let right = self.evaluate(right);

        match op {
            UnaryOp::Not => right.not(),
            UnaryOp::Neg => right.neg(),
        }
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Object {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &LiteralValue) -> Object {
        Object::from(value)
    }
}
