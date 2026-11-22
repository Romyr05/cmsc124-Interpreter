// Shared run path for file mode and the REPL since they do the same things again and again
// This scans (Parse -> run each item -> print values and HTML)
// Environment owned by caller (so repl keeps bindings between lines)
// Print values and first line controlled by caller
// Error handlings are the same for both modes.

use crate::environment::{Environment, Flow};
use crate::errors::CliError;
use crate::parser::Parser;
use crate::scanner::Tokenizer;

pub fn run_source(
    source: &str,
    env: &mut Environment,
    first_line: usize,
    print_values: bool,
) -> Result<(), CliError> {
    //Scanning whole source -> check if there are errors
    let (tokens, errors) = Tokenizer::starting_at(source, first_line).scan();

    if !errors.is_empty() {
        let message = errors
            .iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        return Err(CliError::Compile(message));
    }

    // check phase: report every syntax error; no running
    // Basically parsing whole source; stop -> error
    let items = Parser::new(tokens).parse_program().map_err(|errors| {
        CliError::Compile(
            errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    })?;

    // run phase: the whole source is valid, stop at the first runtime error
    // each statement's rule lives in Environment::execute
    for item in &items {
        let flow = env
            .execute(item, print_values)
            .map_err(|err| CliError::Runtime(err.to_string()))?;
        // render() asks the whole program to stop after it writes its file
        if flow == Flow::Stop {
            break;
        }
    }
    Ok(())
}
