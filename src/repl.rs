// repl.rs
use std::io::{self, Write};
use crate::scanner::Tokenizer;
use crate::token_app::Token;

pub fn run() {
    let stdin = io::stdin();
    let mut command = String::new();

    loop {
        command.clear();

        print!("> ");
        io::stdout().flush().expect("Error flushing stdout.");

        let bytes_read = stdin.read_line(&mut command).expect("Error reading input.");
        if bytes_read == 0 {
            break;
        }

        let tokenizer = Tokenizer::new(&command);
        let tokens: Vec<Token> = tokenizer.collect();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}