mod environment;
mod errors;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
#[cfg(test)]
mod tests;
mod tokens;
use anyhow::Result;
use clap::Parser;
use interpreter::Interpreter;
use log::{debug, error};
use parser::Parser as MyParser;
use scanner::Scanner;
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    filename: Option<PathBuf>,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.filename {
        None => {
            let _ = run_prompt(); // ignore this result
        }
        Some(filename) => {
            if let Err(e) = run_file(filename) {
                error!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_prompt() -> Result<()> {
    let mut buf = String::new();
    let interpreter = Interpreter::new(true);
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        buf.clear();

        match std::io::stdin().lock().read_line(&mut buf) {
            Ok(n) if n > 0 => {
                if let Err(e) = run(&interpreter, &buf) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
            _ => break, // otherwise break.
        }
    }

    Ok(())
}

fn run_file(filepath: PathBuf) -> Result<()> {
    let interpreter = Interpreter::new(false);
    let contents = fs::read_to_string(filepath)?;
    run(&interpreter, &contents)?; // eval contents.
                                   //
    Ok(())
}

fn run(interpreter: &Interpreter, source: &str) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    //println!("{:#?}", &tokens);

    let parser = MyParser::new(&tokens);
    let stmts = parser.parse()?;
    //println!("{:#?}", stmts);

    interpreter.interpret(&stmts)?;

    Ok(())
}
