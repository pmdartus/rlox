use std::{cmp, io};

use crate::ast::{BinaryOp, Expr, ExprVisitor, LiteralValue, Stmt, StmtVisitor, UnaryOp};
use crate::environment::Environment;
use crate::object::Object;
use crate::result::{Error, RloxResult};
use crate::scanner::Token;

pub struct Interpreter<W: io::Write> {
    out: W,
    environment: Environment,
}

impl<W: io::Write> Interpreter<W> {
    pub fn new(out: W) -> Self {
        Self {
            environment: Environment::new(),
            out,
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> RloxResult<()> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> RloxResult<()> {
        statement.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> RloxResult<Object> {
        expr.accept(self)
    }

    fn err(&self, msg: &str) -> Error {
        Error::Runtime(0, String::from(msg))
    }
}

impl<W: io::Write> ExprVisitor<RloxResult<Object>> for Interpreter<W> {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        op: &BinaryOp,
        right: &Expr,
    ) -> RloxResult<Object> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match op {
            BinaryOp::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::String(a), Object::String(b)) => Ok(Object::String(format!("{}{}", a, b))),
                (_, _) => Err(self.err("Operands must be numbers or strings.")),
            },
            BinaryOp::Minus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
                (_, _) => Err(self.err("Operands must be numbers.")),
            },
            BinaryOp::Star => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
                (_, _) => Err(self.err("Operands must be numbers.")),
            },
            BinaryOp::Slash => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
                (_, _) => Err(self.err("Operands must be numbers.")),
            },
            BinaryOp::Equal => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Equal) => Ok(Object::True),
                _ => Ok(Object::False),
            },
            BinaryOp::NotEqual => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Equal) => Ok(Object::False),
                _ => Ok(Object::True),
            },
            BinaryOp::Greater => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Greater) => Ok(Object::True),
                Some(_) => Ok(Object::False),
                _ => Err(self.err(&format!(
                    "Invalid comparison between types (left: {:?}, right: {:?}).",
                    left, right
                ))),
            },
            BinaryOp::GreaterEqual => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Greater) | Some(cmp::Ordering::Equal) => Ok(Object::True),
                Some(_) => Ok(Object::False),
                _ => Err(self.err(&format!(
                    "Invalid comparison between types (left: {:?}, right: {:?}).",
                    left, right
                ))),
            },
            BinaryOp::Less => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Less) => Ok(Object::True),
                Some(_) => Ok(Object::False),
                _ => Err(self.err(&format!(
                    "Invalid comparison between types (left: {:?}, right: {:?}).",
                    left, right
                ))),
            },
            BinaryOp::LessEqual => match left.partial_cmp(&right) {
                Some(cmp::Ordering::Less) | Some(cmp::Ordering::Equal) => Ok(Object::True),
                Some(_) => Ok(Object::False),
                _ => Err(self.err(&format!(
                    "Invalid comparison between types (left: {:?}, right: {:?}).",
                    left, right
                ))),
            },
        }
    }

    fn visit_unary_expr(&mut self, op: &UnaryOp, right: &Expr) -> RloxResult<Object> {
        let right = self.evaluate(right)?;

        match op {
            UnaryOp::Neg => Ok(Object::from(!right.is_truthy())),
            UnaryOp::Not => match right {
                Object::Number(value) => Ok(Object::Number(value * -1.0)),
                _ => Err(self.err("Operand must be a number.")),
            },
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> RloxResult<Object> {
        self.evaluate(expr)
    }

    fn visit_literal_expr(&mut self, value: &LiteralValue) -> RloxResult<Object> {
        Ok(Object::from(value))
    }

    fn visit_variable_expr(&mut self, name: &Token) -> RloxResult<Object> {
        self.environment.get(name)
    }

    fn visit_assignment_expr(&mut self, id: &Token, expr: &Expr) -> RloxResult<Object> {
        let value = self.evaluate(expr)?;
        self.environment.assign(id, value)
    }
}

impl<W: io::Write> StmtVisitor<RloxResult<()>> for Interpreter<W> {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> RloxResult<()> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, id: &Token, intializer: &Option<Box<Expr>>) -> RloxResult<()> {
        let value = match intializer {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Nil,
        };

        self.environment.define(id, value);
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> RloxResult<()> {
        let value = self.evaluate(expr)?;
        writeln!(self.out, "{}", value).unwrap();
        Ok(())
    }
}
