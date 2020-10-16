use rlox;

use std::env;
use std::fs;
use std::io::{self, Write};

use rlox::interpreter::Interpret;
use rlox::parser::Parser;
use rlox::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).expect("Can't read file");

    run(&content);
}

fn run_prompt() {
    let mut buffer = String::new();

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.trim().len() == 0 {
            break;
        } else {
            run(&buffer);
        }
    }
}

fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Scanner::tokenize(source)?;
    let expr = Parser::parse(tokens).unwrap();

    let mut interpreter = Interpret::new();
    let res = interpreter.evaluate(&expr);

    println!("{:?}", res);

    Ok(())
}
