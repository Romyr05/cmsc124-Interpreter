mod ast;
mod code_generator;
mod errors;
mod evaluator;
pub mod keyword_list;
mod parser;
mod repl;
mod runner;
mod scanner;
mod token_types;
mod tree_printer;

use crate::errors::CliError;
use crate::evaluator::Environment;
use crate::parser::{Parser, TopLevel};
use crate::runner::run_source;
use crate::scanner::Tokenizer;
use crate::token_types::TokenType;
use crate::tree_printer::{print_element, print_expr, print_let};
use std::{fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    let result = match args.get(1).map(|s| s.as_str()) {
        // no command: drop into the interactive REPL
        None => repl::run().map_err(CliError::Io),
        Some("--parse") => match args.get(2) {
            Some(path) => parse_file(path),
            None => Err(CliError::Usage("--parse needs a file".to_string())),
        },
        Some("--eval") => match args.get(2) {
            Some(path) => eval_file(path),
            None => Err(CliError::Usage("--eval needs a file".to_string())),
        },
        Some("--tokenize") => match args.get(2) {
            Some(path) => tokenize_file(path),
            None => Err(CliError::Usage("--tokenize needs a file".to_string())),
        },
        Some(other) => Err(CliError::Usage(format!("Unknown command: {}", other))),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::from(error.exit_code())
        }
    }
}

// --tokenize: print the scanned tokens
fn tokenize_file(path: &str) -> Result<(), CliError> {
    let source = fs::read_to_string(path)?;
    let (tokens, errors) = Tokenizer::new(&source).scan();

    // For newline token to hide them
    for token in &tokens {
        if token.token_type != TokenType::Newline {
            println!("{:?}", token);
        }
    }
    if !errors.is_empty() {
        let message = errors
            .iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        return Err(CliError::Compile(message));
    }
    Ok(())
}

// --eval: evaluate each expression in the file and print its value, one per line.
// scan, parse, and run are shared with the REPL (runner.rs)
fn eval_file(path: &str) -> Result<(), CliError> {
    let source = fs::read_to_string(path)?;
    // one environment for the whole file
    let mut env = Environment::default();
    run_source(&source, &mut env, 1)
}

// --parse: print the AST tree
fn parse_file(path: &str) -> Result<(), CliError> {
    let source = fs::read_to_string(path)?;
    let (tokens, errors) = Tokenizer::new(&source).scan();

    // a scan error means the tokens are unreliable, so stop before parsing
    if !errors.is_empty() {
        let message = errors
            .iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        return Err(CliError::Compile(message));
    }

    let items = Parser::new(tokens).parse_program().map_err(|errors| {
        CliError::Compile(
            errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    })?;

    for item in &items {
        match item {
            TopLevel::Expr(expr) => println!("{}", print_expr(expr)),
            TopLevel::Element(element) => println!("{}", print_element(element)),
            TopLevel::Let {
                name, initializer, ..
            } => println!("{}", print_let(name, initializer.as_ref())),
            TopLevel::Print(expr) => println!("(print {})", print_expr(expr)),
        }
    }
    Ok(())
}
