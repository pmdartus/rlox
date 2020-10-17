use std::{f32, fmt};

use crate::ast::{BinaryOp, Expr, LiteralValue, UnaryOp, ExprVisitor, Stmt, StmtVisitor};

#[derive(Debug)]
pub enum Object {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

impl Object {
    fn is_truthy(&self) -> bool {
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
            Object::Number(value)  => write!(f, "{}", value),
            Object::String(value) => write!(f, "{}", value),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
            Object::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeException {
    // token: Token,
    msg: String,
}

impl RuntimeException {
    fn new(/*token: Token,*/ msg: &str) -> Self {
        Self {
            // token,
            msg: String::from(msg),
        }
    }
}

type InterpreterResult<T> = Result<T, RuntimeException>;

pub struct Interpret {}

impl Interpret {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> InterpreterResult<()> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> InterpreterResult<()> {
        statement.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult<Object> {
        expr.accept(self)
    }
}

impl ExprVisitor<InterpreterResult<Object>> for Interpret {
    fn visit_binary(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
    ) -> InterpreterResult<Object> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match op {
            BinaryOp::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::String(a), Object::String(b)) => Ok(Object::String(format!("{}{}", a, b))),
                (_, _) => Err(RuntimeException::new(
                    "Operands must be numbers or strings.",
                )),
            },
            BinaryOp::Minus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
                (_, _) => Err(RuntimeException::new("Operands must be numbers.")),
            },
            BinaryOp::Star => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
                (_, _) => Err(RuntimeException::new("Operands must be numbers.")),
            },
            BinaryOp::Slash => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
                (_, _) => Err(RuntimeException::new("Operands must be numbers.")),
            },
            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => {
                    if a == b {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (_, _) => Err(RuntimeException::new("Operands must be numbers.")),
            },
        }
    }

    fn visit_unary(&mut self, op: &UnaryOp, right: &Expr) -> InterpreterResult<Object> {
        let right = self.evaluate(right)?;

        match op {
            UnaryOp::Neg => Ok(Object::from(!right.is_truthy())),
            UnaryOp::Not => match right {
                Object::Number(value) => Ok(Object::Number(value * -1.0)),
                _ => Err(RuntimeException::new("Operand must be a number.")),
            },
        }
    }
    fn visit_grouping(&mut self, expr: &Expr) -> InterpreterResult<Object> {
        self.evaluate(expr)
    }

    fn visit_literal(&mut self, value: &LiteralValue) -> InterpreterResult<Object> {
        Ok(Object::from(value))
    }
}

impl StmtVisitor<InterpreterResult<()>> for Interpret {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> InterpreterResult<()> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> InterpreterResult<()> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }
}