use std::fmt;

/// Result type for all the rlox interfaces.
pub type RloxResult<T> = Result<T, Error>;

/// rlox specific error.
#[derive(Debug, Clone)]
pub enum Error {
    /// Error returned if the scanner encounters an error.
    Scanner(usize, String),
    /// Error returned if the parser encounters an error.
    Parser(usize, String),
    /// Error returned if the interpreter encounters an error.
    Runtime(usize, String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Scanner(line, msg) => write!(f, "Scanner error [line: {}]: {}", line, msg),
            Error::Parser(line, msg) => write!(f, "Parser error [line: {}]: {}", line, msg),
            Error::Runtime(line, msg) => write!(f, "Runtime error [line: {}]: {}", line, msg),
        }
    }
}