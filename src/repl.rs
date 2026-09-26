use crate::scanner::Tokenizer;
use std::io::{self, Write};

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

        let (tokens, errors) = Tokenizer::new(&command).scan();
        for token in &tokens {
            println!("{:?}", token);
        }
        for e in &errors {
            println!("{}", e);
        }
    }
}
