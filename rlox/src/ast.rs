#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Coma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub lexeme: String,
}

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
    Variable(Token),
    Assign(Token, Box<Expr>),
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, op: &BinaryOp, right: &Expr) -> T;
    fn visit_unary_expr(&mut self, op: &UnaryOp, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &LiteralValue) -> T;
    fn visit_variable_expr(&mut self, id: &Token) -> T;
    fn visit_assignment_expr(&mut self, id: &Token, expr: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(left, op, right) => visitor.visit_binary_expr(left, op, right),
            Expr::Unary(op, expr) => visitor.visit_unary_expr(op, expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(value) => visitor.visit_literal_expr(value),
            Expr::Variable(id) => visitor.visit_variable_expr(id),
            Expr::Assign(id, expr) => visitor.visit_assignment_expr(id, expr),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Var(Token, Option<Box<Expr>>),
    Print(Box<Expr>),
    Block(Vec<Stmt>),
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, expr: &Expr) -> T;
    fn visit_var_stmt(&mut self, id: &Token, initalizer: &Option<Box<Expr>>) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_block_stmt(&mut self, body: &[Stmt]) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression_stmt(expr),
            Stmt::Var(id, initalizer) => visitor.visit_var_stmt(id, initalizer),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Block(body) => visitor.visit_block_stmt(body),
        }
    }
}
