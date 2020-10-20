mod ast;
mod environment;
mod object;

pub mod result;
pub mod interpreter;
pub mod parser;
pub mod scanner;

use std::io::{Write};
use scanner::Scanner;
use parser::Parser;
use interpreter::Interpret;
use result::RloxResult;

pub fn evaluate<W: Write>(input: &str, out: W) -> RloxResult<()> {
    let tokens = Scanner::scan(input)?;
    let statements = Parser::parse(tokens).map_err(|e| e[0].clone())?;
    
    let mut interpreter = Interpret::new(out);
    interpreter.interpret(&statements)?;
    
    Ok(())
}