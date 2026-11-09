use crate::evaluator::Environment;
use crate::runner::run_source;
use std::io::{self, Write};

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut command = String::new();
    // created once, before the loop, so bindings survive between lines
    let mut env = Environment::default();
    // counts input lines across the session so errors say which line they came from
    let mut line = 0;

    loop {
        command.clear();

        print!("> ");
        io::stdout().flush()?;

        let bytes_read = stdin.read_line(&mut command)?;
        if bytes_read == 0 {
            break;
        }

        line += 1;
        eval_line(&command, &mut env, line);
    }
    Ok(())
}

// evaluate one line of input and print the value, or an error, then return to the prompt
// the same scan/parse/run path as file mode (runner.rs); errors are reported, not fatal
fn eval_line(src: &str, env: &mut Environment, line: usize) {
    if let Err(err) = run_source(src, env, line) {
        eprintln!("{}", err);
    }
}
