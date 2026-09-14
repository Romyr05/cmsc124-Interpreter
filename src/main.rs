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
        Some(path) => {
            fs::read_to_string(&path).expect("could not read input file");
            let tokenizer = Tokenizer::new(&path);
            let tokens: Vec<Token> = tokenizer.collect();
            for token in tokens {
                println!("{:?}", token);
            }
        }

        // For manual purposes (cargo run)
        None => run(),
    };
}
