use crate::ast::{BinaryOp, Expr, LiteralValue, Stmt, Token, TokenKind, UnaryOp};
use crate::result::{Error, RloxResult};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<Error>,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Vec<Error>> {
        let mut parser = Parser {
            tokens,
            current: 0,
            errors: vec![],
        };

        let statements = parser.program();

        if !parser.errors.is_empty() {
            Err(parser.errors)
        } else {
            Ok(statements)
        }
    }

    fn program(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    //TODO: Add synchronization when handling error
                    self.errors.push(err)
                }
            }
        }

        statements
    }

    fn declaration(&mut self) -> RloxResult<Stmt> {
        match self.peek().kind {
            TokenKind::Var => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> RloxResult<Stmt> {
        // TODO: Clean this up.
        let peeked = self.peek();
        let name = match &peeked.kind {
            TokenKind::Identifier(_) => Ok(peeked.clone()),
            _ => Err(self.err("Expected variable name.")),
        }?;

        self.advance();

        let initializer = match self.peek().kind {
            TokenKind::Equal => {
                self.advance();
                Some(Box::new(self.expression()?))
            }
            _ => None,
        };

        self.consume(
            &TokenKind::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> RloxResult<Stmt> {
        match self.peek().kind {
            TokenKind::Print => self.print_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> RloxResult<Stmt> {
        self.consume(&TokenKind::Print, "Expected print")?;
        let expr = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expected ';' after value")?;

        Ok(Stmt::Print(Box::new(expr)))
    }

    fn expression_statement(&mut self) -> RloxResult<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression(Box::new(expr)))
    }

    fn expression(&mut self) -> RloxResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> RloxResult<Expr> {
        let expr = self.equality()?;

        if TokenKind::Equal == self.peek().kind {
            self.advance();
            let value = self.assignment()?;

            if let Expr::Variable(id) = expr {
                return Ok(Expr::Assign(id, Box::new(value)));
            }

            return Err(self.err("Invalid assignment target."));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> RloxResult<Expr> {
        let mut expr = self.comparison()?;

        while !self.is_at_end() {
            let operator = match self.peek().kind {
                TokenKind::BangEqual => Some(BinaryOp::NotEqual),
                TokenKind::EqualEqual => Some(BinaryOp::Equal),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.comparison()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> RloxResult<Expr> {
        let mut expr = self.term()?;

        while !self.is_at_end() {
            let operator = match self.peek().kind {
                TokenKind::Less => Some(BinaryOp::Less),
                TokenKind::LessEqual => Some(BinaryOp::LessEqual),
                TokenKind::Greater => Some(BinaryOp::Greater),
                TokenKind::GreaterEqual => Some(BinaryOp::GreaterEqual),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.term()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> RloxResult<Expr> {
        let mut expr = self.factor()?;

        while !self.is_at_end() {
            let operator = match self.peek().kind {
                TokenKind::Plus => Some(BinaryOp::Plus),
                TokenKind::Minus => Some(BinaryOp::Minus),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.factor()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> RloxResult<Expr> {
        let mut expr = self.unary()?;

        while !self.is_at_end() {
            let operator = match self.peek().kind {
                TokenKind::Slash => Some(BinaryOp::Slash),
                TokenKind::Star => Some(BinaryOp::Star),
                _ => None,
            };

            if let Some(operator) = operator {
                self.advance();

                let right = self.unary()?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> RloxResult<Expr> {
        let operator = match self.peek().kind {
            TokenKind::Bang => Some(UnaryOp::Not),
            TokenKind::Minus => Some(UnaryOp::Neg),
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

    fn primary(&mut self) -> RloxResult<Expr> {
        let token = self.advance();
        match &token.kind {
            TokenKind::Number(value) => Ok(Expr::Literal(LiteralValue::Number(*value))),
            TokenKind::String(value) => {
                Ok(Expr::Literal(LiteralValue::String(String::from(value))))
            }
            TokenKind::True => Ok(Expr::Literal(LiteralValue::True)),
            TokenKind::False => Ok(Expr::Literal(LiteralValue::False)),
            TokenKind::Nil => Ok(Expr::Literal(LiteralValue::Nil)),
            TokenKind::Identifier(_) => Ok(Expr::Variable(token.clone())),
            TokenKind::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenKind::RightParen, "Expected ')' after expression.")?;

                Ok(Expr::Grouping(Box::new(expr)))
            }

            _ => Err(self.err("Expected expression")),
        }
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, kind: &TokenKind, msg: &'static str) -> RloxResult<&Token> {
        if &self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(self.err(msg))
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn err(&self, msg: &str) -> Error {
        Error::Parser(self.peek().line, String::from(msg))
    }
}
