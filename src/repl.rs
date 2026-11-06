use crate::code_generator::generate;
use crate::evaluator::{Environment, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut command = String::new();
    // created once, before the loop, so bindings survive between lines
    let mut env = Environment::default();

    loop {
        command.clear();

        print!("> ");
        io::stdout().flush()?;

        let bytes_read = stdin.read_line(&mut command)?;
        if bytes_read == 0 {
            break;
        }

        eval_line(&command, &mut env);
    }
    Ok(())
}

// evaluate one line of input and print the value, or an error, then return to the prompt
fn eval_line(src: &str, env: &mut Environment) {
    let (tokens, errors) = Tokenizer::new(src).scan();
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        return;
    }

    let mut parser = Parser::new(tokens);
    match parser.parse_top_level() {
        Ok(TopLevel::Expr(expr)) => match env.evaluate(&expr) {
            Ok(value) => println!("{}", stringify(&value)),
            Err(err) => eprintln!("{}", err), // runtime error: report and keep the session alive
        },
        Ok(TopLevel::Element(element)) => match generate(&element, env) {
            Ok(html) => println!("{}", html),
            Err(err) => eprintln!("{}", err),
        },
        // a declaration prints nothing; on error the table is left as it was
        Ok(TopLevel::Let {
            name,
            initializer,
            line,
        }) => {
            if let Err(err) = env.declare(&name, initializer.as_ref(), line) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
