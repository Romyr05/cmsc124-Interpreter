use crate::scanner::Tokenizer;
use crate::parser::{ parse };
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

        // print tokens and AST if ever
        println!("Tokens: ");
        let tokenizer = Tokenizer::new(&command);
        for result in tokenizer {
            match result {
                Ok(token) => println!("{:?}", token),
                Err(e) => eprintln!("{}", e),
            }
        }

        println!("AST: ");
        parse(&command);
    }
}
