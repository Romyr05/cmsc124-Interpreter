use crate::code_generator::generate;
use crate::evaluator::{evaluate, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut command = String::new();

    loop {
        command.clear();

        print!("> ");
        io::stdout().flush()?;

        let bytes_read = stdin.read_line(&mut command)?;
        if bytes_read == 0 {
            break;
        }

        eval_line(&command);
    }
    Ok(())
}

// evaluate one line of input and print the value, or an error, then return to the prompt
fn eval_line(src: &str) {
    let (tokens, errors) = Tokenizer::new(src).scan();
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        return;
    }

    let mut parser = Parser::new(tokens);
    match parser.parse_top_level() {
        Ok(TopLevel::Expr(expr)) => match evaluate(&expr) {
            Ok(value) => println!("{}", stringify(&value)),
            Err(err) => eprintln!("{}", err), // runtime error: report and keep the session alive
        },
        Ok(TopLevel::Element(element)) => match generate(&element) {
            Ok(html) => println!("{}", html),
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}
