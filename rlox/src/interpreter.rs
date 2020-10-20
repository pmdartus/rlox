use std::io;
use crate::environment::{Environment};
use crate::ast::{BinaryOp, Expr, LiteralValue, UnaryOp, ExprVisitor, Stmt, StmtVisitor};
use crate::object::Object;
use crate::result::{RloxResult,Error};

pub struct Interpret<W: io::Write> {
    out: W,
    environment: Environment,
}

impl<W: io::Write> Interpret<W> {
    pub fn new(out: W) -> Self {
        Self {
            environment: Environment::new(),
            out,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> RloxResult<()> {
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

impl<W: io::Write> ExprVisitor<RloxResult<Object>> for Interpret<W> {
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
                (_, _) => Err(self.err(
                    "Operands must be numbers or strings.",
                )),
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
                (_, _) => Err(self.err("Operands must be numbers.")),
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

    fn visit_variable_expr(&mut self, name: &str) -> RloxResult<Object> {
        // self.environment.get(name)
        unimplemented!();
    }
}

impl<W: io::Write> StmtVisitor<RloxResult<()>> for Interpret<W> {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> RloxResult<()> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &str, intializer: &Option<Box<Expr>>) -> RloxResult<()> {
        let value = match intializer {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Nil,
        };

        // TODO: Avoid string copy
        self.environment.define(String::from(name), value);
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> RloxResult<()> {
        let value = self.evaluate(expr)?;
        writeln!(self.out, "{}", value);
        Ok(())
    }
}