// Shared run path for file mode and the REPL:
// scan -> parse the whole source -> run each item, printing values and HTML.
// The caller owns the Environment (so the REPL keeps bindings between lines)
// and decides what an error means (file: exit 65/70, REPL: print and continue).
// first_line is the line number of the source's first line (file: 1, REPL: session line).
// print_values: print bare expression values (--eval, REPL) or discard them (plain ./run).
use crate::errors::CliError;
use crate::evaluator::Environment;
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
        env.execute(item, print_values)
            .map_err(|err| CliError::Runtime(err.to_string()))?;
    }
    Ok(())
}
