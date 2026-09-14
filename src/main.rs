mod scanner;
mod token_app;
mod repl;
use crate::repl::run;
use crate::scanner::Tokenizer;
use crate::token_app::Token;
use std::fs;

fn main() {
    // iterates and gets the file name
    let path = std::env
        ::args()
        .skip(1)
        .find(|arg| !arg.starts_with("--"));

    let source = match path {
        Some(path) => fs::read_to_string(&path).expect("could not read input file"),

        // For manual purposes (cargo run)
        None => String::from("let x = 42 + 5 button () == aa"),
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

    // a rejected file puts nothing on stdout and exits 65; a clean scan prints tokens and exits 0
    if errors.is_empty() {
        for token in tokens {
            println!("{:?}", token);
        }
    } else {
        for e in errors {
            eprintln!("{}", e);
        }
        std::process::exit(65);
    }
}
