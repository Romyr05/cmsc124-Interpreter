// Shared run path for file mode and the REPL:
// scan -> parse the whole source -> run each item, printing values and HTML.
// The caller owns the Environment (so the REPL keeps bindings between lines)
// and decides what an error means (file: exit 65/70, REPL: print and continue).
use crate::code_generator::generate;
use crate::errors::CliError;
use crate::evaluator::{Environment, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;

pub fn run_source(source: &str, env: &mut Environment) -> Result<(), CliError> {
    //Scanning whole source -> check if there are errors
    let (tokens, errors) = Tokenizer::new(source).scan();

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
    for item in &items {
        match item {
            TopLevel::Expr(expr) => match env.evaluate(expr) {
                Ok(value) => println!("{}", stringify(&value)),
                Err(err) => return Err(CliError::Runtime(err.to_string())),
            },
            TopLevel::Element(element) => match generate(element, env) {
                Ok(html) => println!("{}", html),
                Err(err) => return Err(CliError::Runtime(err.to_string())),
            },
            // a declaration prints nothing; on error the table is left as it was
            TopLevel::Let {
                name,
                initializer,
                line,
            } => {
                if let Err(err) = env.declare(name, initializer.as_ref(), *line) {
                    return Err(CliError::Runtime(err.to_string()));
                }
            }
        }
    }
    Ok(())
}
