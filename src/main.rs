use anyhow::Result;
use clap::Parser;
use myjlox::parser::Parser as MyParser;
use myjlox::scanner::Scanner;
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    filename: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    match cli.filename {
        None => {
            let _ = run_prompt(); // ignore this result
        }
        Some(filename) => {
            if let Err(e) = run_file(filename) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_prompt() -> Result<()> {
    let mut buf = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        buf.clear();

        match std::io::stdin().lock().read_line(&mut buf) {
            Ok(n) if n > 0 => {
                if let Err(e) = run(&buf) {
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
    println!("The filename is {:?}", filepath);
    let contents = fs::read_to_string(filepath).expect("reading file failed.");
    run(&contents) // eval contents.
}

fn run(source: &str) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let parser = MyParser::new(tokens);
    let expr = parser.parse();

    println!("{:#?}", expr);

    Ok(())
}
