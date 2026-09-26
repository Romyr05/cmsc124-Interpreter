mod ast;
pub mod keyword_list;
mod parser;
mod repl;
mod scanner;
mod token_types;
mod tree_printer;
use crate::scanner::Tokenizer;
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

    let (tokens, errors) = Tokenizer::new(&source).scan();

    // For expected, error and exit
    for token in &tokens {
        println!("{:?}", token);
    }
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        std::process::exit(65);
    }
}
