use std::process::exit;
use std::io::Write;

mod keywords;
pub use keywords::*;
mod token;
pub use token::*;
mod token_type;
pub use token_type::*;
mod errors;
pub use errors::*;
mod scanner;
mod expr;
mod parser;

type LoxResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> LoxResult {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(0);
    } else if args.len() == 2 {
        run_file(args[1].clone())?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(path: String) -> LoxResult {
    let source = std::fs::read_to_string(path)?;
    run(source)
}

fn run_prompt() -> LoxResult {
    loop {
        print!("> ");
        std::io::stdout().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(read_bytes) => { 
                if read_bytes == 0 { break; }
                if let Err(msg) = run(input) {
                    println!("{}", msg);
                }
            }
            _ => return Err("Could not read from stdin".into())
        }
    }

    Ok(())
}

fn run(source: String) -> LoxResult {
    let mut scanner = scanner::Scanner::new(&source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens.iter() {
        println!("token: {:?}", token);
    }

    Ok(())
}
