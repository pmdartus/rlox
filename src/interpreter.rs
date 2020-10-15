use super::ast::{Expr, LiteralValue, BinaryOp, UnaryOp, Visitor};

#[derive(Debug)]
pub enum Object {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Object {
    fn negate(&self) -> Object {
        unimplemented!();
    }

    fn minus(&self) -> Object {
        unimplemented!();
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
                (Object::Number(a), Object::Number(b)) => if a == b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
            BinaryOp::NotEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => if a != b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
            BinaryOp::Greater => match (left, right) {
                (Object::Number(a), Object::Number(b)) => if a > b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
            BinaryOp::GreaterEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => if a >= b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
            BinaryOp::Less => match (left, right) {
                (Object::Number(a), Object::Number(b)) => if a < b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
            BinaryOp::LessEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => if a <= b { Object::True } else { Object::False },
                (_, _) => unimplemented!(),
            },
        }
    }

    fn visit_unary(&mut self, op: &UnaryOp, right: &Expr) -> Object {
        let right = self.evaluate(right);

        match op {
            UnaryOp::Not => right.negate(),
            UnaryOp::Minus => right.minus(),
        }
    }
    
    fn visit_grouping(&mut self, expr: &Expr) -> Object {
        Object::True
    }

    fn visit_literal(&mut self, value: &LiteralValue) -> Object {
        Object::from(value)
    }
}