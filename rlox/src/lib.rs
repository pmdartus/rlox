mod ast;
mod environment;
mod object;

pub mod interpreter;
pub mod parser;
pub mod result;
pub mod scanner;

use interpreter::Interpreter;
use parser::Parser;
use result::RloxResult;
use scanner::Scanner;
use std::io::Write;

pub fn evaluate<W: Write>(input: &str, out: W) -> RloxResult<()> {
    let tokens = Scanner::scan(input)?;
    let statements = Parser::parse(tokens).map_err(|e| e[0].clone())?;

    let mut interpreter = Interpreter::new(out);
    interpreter.interpret(&statements)?;

    Ok(())
}
