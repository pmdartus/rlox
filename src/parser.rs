use crate::scanner::{Token, TokenKind};
use std::{fmt, error};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literator(Value),
    Unary(UnaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token, &'static str),
    ExpectedExpression(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (token, msg) = match self {
            ParseError::UnexpectedToken(token, msg) => (token, msg),
            ParseError::ExpectedExpression(token) => (token, &"Expected expression"),
        };

        write!(f, "[line: {}] {}", token.line, msg)
    }
}

impl error::Error for ParseError {}

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Expr, Vec<ParseError>> {
        let mut parser = Parser {
            tokens,
            current: 0,
            errors: vec![],
        };

        match parser.expression() {
            Err(error) => Err(vec![error]),
            Ok(expr) => {
                if parser.errors.len() > 0 {
                    Err(parser.errors)
                } else {
                    Ok(expr)
                }
            }
        }
    }

    /// expression     → equality ;
    fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while !self.is_at_end() {
            let operator = match self.peek() {
                Token {
                    kind: TokenKind::BangEqual,
                    ..
                } => Some(BinaryOp::NotEqual),
                Token {
                    kind: TokenKind::EqualEqual,
                    ..
                } => Some(BinaryOp::Equal),
                _ => None,
            };
            if let Some(operator) = operator {
                self.advance();

                let right = self.comparison()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
        }

        Ok(expr)
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while !self.is_at_end() {
            let operator = match self.peek() {
                Token {
                    kind: TokenKind::Less,
                    ..
                } => Some(BinaryOp::Less),
                Token {
                    kind: TokenKind::LessEqual,
                    ..
                } => Some(BinaryOp::LessEqual),
                Token {
                    kind: TokenKind::Greater,
                    ..
                } => Some(BinaryOp::Greater),
                Token {
                    kind: TokenKind::GreaterEqual,
                    ..
                } => Some(BinaryOp::GreaterEqual),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.term()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
        }

        Ok(expr)
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while !self.is_at_end() {
            let operator = match self.peek() {
                Token {
                    kind: TokenKind::Plus,
                    ..
                } => Some(BinaryOp::Plus),
                Token {
                    kind: TokenKind::Minus,
                    ..
                } => Some(BinaryOp::Minus),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.factor()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
        }

        Ok(expr)
    }

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while !self.is_at_end() {
            let operator = match self.peek() {
                Token {
                    kind: TokenKind::Slash,
                    ..
                } => Some(BinaryOp::Slash),
                Token {
                    kind: TokenKind::Star,
                    ..
                } => Some(BinaryOp::Star),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.unary()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
        }

        Ok(expr)
    }

    /// unary          → ( "!" | "-" ) unary
    ///                  | primary ;
    fn unary(&mut self) -> ParseResult<Expr> {
        let operator = match self.peek() {
            Token {
                kind: TokenKind::Bang,
                ..
            } => Some(UnaryOp::Not),
            Token {
                kind: TokenKind::Minus,
                ..
            } => Some(UnaryOp::Minus),
            _ => None,
        };

        if let Some(operator) = operator {
            self.advance();

            let unary = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(unary)))
        } else {
            self.primary()
        }
    }

    /// primary        → NUMBER | STRING | "true" | "false" | "nil"
    ///                  | "(" expression ")" ;
    fn primary(&mut self) -> ParseResult<Expr> {
        match self.advance() {
            Token {
                kind: TokenKind::Number(value),
                ..
            } => Ok(Expr::Literator(Value::Number(*value))),
            Token {
                kind: TokenKind::String(value),
                ..
            } => Ok(Expr::Literator(Value::String(String::from(value)))),
            Token {
                kind: TokenKind::True,
                ..
            } => Ok(Expr::Literator(Value::True)),
            Token {
                kind: TokenKind::False,
                ..
            } => Ok(Expr::Literator(Value::False)),
            Token {
                kind: TokenKind::Nil,
                ..
            } => Ok(Expr::Literator(Value::Nil)),

            Token {
                kind: TokenKind::LeftParen,
                ..
            } => {
                let expr = self.expression()?;
                self.consume(&TokenKind::RightParen, "Expected ')' after expression.")?;

                Ok(Expr::Grouping(Box::new(expr)))
            }

            _ => Err(ParseError::ExpectedExpression(self.peek().clone())),
        }
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, kind: &TokenKind, msg: &'static str) -> ParseResult<&Token> {
        if &self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken(self.peek().clone(), msg))
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
}
