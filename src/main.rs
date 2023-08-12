use clap::Parser;
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
            run_prompt();
        }
        Some(filename) => {
            run_file(filename);
        }
    }
}

fn run_prompt() {
    let mut buf = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        buf.clear();

        match std::io::stdin().lock().read_line(&mut buf) {
            Ok(n) if n > 0 => run(&buf),
            Err(e) => {
                panic!("{}", e);
            }
            _ => break, // otherwise break.
        }
    }
}

fn run_file(filepath: PathBuf) {
    println!("The filename is {:?}", filepath);
    let contents = fs::read_to_string(filepath).expect("reading file failed.");
    run(&contents); // eval contents.
}

fn run(source: &str) {
    println!("{}", source);
}
