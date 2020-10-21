use std::{char, str};

use crate::result::{Error, RloxResult};

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

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner<'_> {
    pub fn scan(source: &str) -> RloxResult<Vec<Token>> {
        let mut scanner = Scanner {
            source: source.as_bytes(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        };

        scanner.scan_tokens()?;
        Ok(scanner.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) -> RloxResult<()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.add_token(TokenKind::EOF);
        Ok(())
    }

    fn scan_token(&mut self) -> RloxResult<()> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Coma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),

            '!' => {
                if self.matches('=') {
                    self.add_token(TokenKind::BangEqual)
                } else {
                    self.add_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenKind::EqualEqual)
                } else {
                    self.add_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenKind::LessEqual)
                } else {
                    self.add_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenKind::GreaterEqual)
                } else {
                    self.add_token(TokenKind::Greater)
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash)
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' => self.process_string()?,
            c if Self::is_digit(c) => self.process_number()?,
            c if Self::is_alpha(c) => self.process_identifier()?,

            _ => {
                return Err(self.err("Unexpected character."));
            }
        }

        Ok(())
    }

    fn process_string(&mut self) -> RloxResult<()> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(self.err("Unterminated string"));
        }

        self.advance();
        self.add_token(TokenKind::String(
            str::from_utf8(&self.source[self.start + 1..self.current - 1])
                .map_err(|_| self.err("Invalid string"))?
                .to_string(),
        ));

        Ok(())
    }

    fn process_number(&mut self) -> RloxResult<()> {
        while let Some(c) = self.peek() {
            if Self::is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            if let Some(c) = self.peek_next() {
                if Self::is_digit(c) {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if Self::is_digit(c) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        self.add_token(TokenKind::Number(
            str::from_utf8(&self.source[self.start..self.current])
                .map_err(|_| self.err("Invalid number."))?
                .parse()
                .map_err(|_| self.err("Invalid number."))?,
        ));

        Ok(())
    }

    fn process_identifier(&mut self) -> RloxResult<()> {
        while let Some(c) = self.peek() {
            if Self::is_alpha_numeric(c) {
                self.advance();
            } else {
                break;
            }
        }

        let value = str::from_utf8(&self.source[self.start..self.current])
            .map_err(|_| self.err("Invalid string."))?;

        self.add_token(match value {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Function,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier(value.to_string()),
        });

        Ok(())
    }

    fn add_token(&mut self, token: TokenKind) {
        self.tokens.push(Token {
            kind: token,
            line: self.line,
            lexeme: str::from_utf8(&self.source[self.start..self.current]).unwrap().to_owned()
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1] as char
    }

    fn peek(&self) -> Option<char> {
        if self.current >= self.source.len() {
            None
        } else {
            Some(self.source[self.current] as char)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1] as char)
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn err(&self, msg: &str) -> Error {
        Error::Scanner(self.line, String::from(msg))
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }
}
