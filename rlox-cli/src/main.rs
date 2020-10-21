use rlox;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use rlox::{interpreter::Interpreter, parser::Parser, result::Error, scanner::Scanner};

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

    let out = io::stdout();
    let mut interpreter = Interpreter::new(out);

    if let Err(err) = run(&mut interpreter, &content) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run_prompt() {
    let mut buffer = String::new();

    let out = io::stdout();
    let mut interpreter = Interpreter::new(out);

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.trim().len() == 0 {
            break;
        } else {
            if let Err(err) = run(&mut interpreter, &buffer) {
                eprintln!("{}", err);
            }
        }
    }
}

fn run<W: io::Write>(interpreter: &mut Interpreter<W>, source: &str) -> Result<(), Error> {
    let tokens = Scanner::scan(source)?;
    let statements = Parser::parse(tokens).map_err(|e| e[0].clone())?;

    interpreter.interpret(&statements)?;

    Ok(())
}
