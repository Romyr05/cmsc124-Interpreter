pub mod keyword_list;
mod repl;
mod scanner;
mod token_types;
use crate::scanner::Tokenizer;
use crate::token_types::Token;
use std::fs;

fn main() {
    // iterates and gets the file name
    let path = std::env::args().skip(1).find(|arg| !arg.starts_with("--"));

    let source = match path {
        Some(path) => fs::read_to_string(&path).expect("could not read input file"),

        // no file argument: drop into the interactive REPL instead of scanning a file
        None => {
            repl::run();
            return;
        }
    };

    let tokenizer = Tokenizer::new(&source);
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors = Vec::new();
    for result in tokenizer {
        match result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e),
        }
    }

    // For expected, error and exit
    for token in tokens {
        println!("{:?}", token);
    }
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        std::process::exit(65);
    }
}
