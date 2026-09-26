use crate::parser::parse;
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

        parse(&command);
    }
}
