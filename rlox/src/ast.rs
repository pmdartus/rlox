#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Slash,
    Star,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
}

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> T;
    fn visit_unary(&mut self, op: &UnaryOp, expr: &Expr) -> T;
    fn visit_grouping(&mut self, expr: &Expr) -> T;
    fn visit_literal(&mut self, value: &LiteralValue) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(left, op, right) => visitor.visit_binary(left, op, right),
            Expr::Unary(op, expr) => visitor.visit_unary(op, expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(value) => visitor.visit_literal(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
        }
    }
}