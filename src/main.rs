mod ast;
mod code_generator;
mod errors;
mod evaluator;
pub mod keyword_list;
mod parser;
mod repl;
mod scanner;
mod token_types;
mod tree_printer;

use crate::code_generator::generate;
use crate::errors::CliError;
use crate::evaluator::{evaluate, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
use crate::token_types::{Token, TokenType};
use crate::tree_printer::{print_element, print_expr};
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

    for token in &tokens {
        println!("{:?}", token);
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
// One expression per line, so we group the scanned tokens by line number.
fn eval_file(path: &str) -> Result<(), CliError> {
    let source = fs::read_to_string(path)?;
    let (tokens, errors) = Tokenizer::new(&source).scan();

    if !errors.is_empty() {
        let message = errors
            .iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        return Err(CliError::Compile(message));
    }

    // split the flat token stream into one group per source line
    let mut groups: Vec<Vec<Token>> = Vec::new();
    for token in tokens {
        if token.token_type == TokenType::Eof {
            continue;
        }
        match groups.last_mut() {
            Some(group) if group.first().is_some_and(|first| first.line == token.line) => {
                group.push(token);
            }
            _ => groups.push(vec![token]),
        }
    }

    for mut group in groups {
        let line = group[0].line;
        group.push(Token::new(TokenType::Eof, "", line)); // parser expects a trailing Eof

        let mut parser = Parser::new(group);
        match parser.parse_top_level() {
            Ok(TopLevel::Expr(expr)) => match evaluate(&expr) {
                Ok(value) => println!("{}", stringify(&value)),
                Err(err) => return Err(CliError::Runtime(err.to_string())),
            },
            Ok(TopLevel::Element(element)) => match generate(&element) {
                Ok(html) => println!("{}", html),
                Err(err) => return Err(CliError::Runtime(err.to_string())),
            },
            Err(err) => return Err(CliError::Compile(err.to_string())),
        }
    }
    Ok(())
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

    let mut parser = Parser::new(tokens);
    match parser.parse_top_level() {
        Ok(TopLevel::Expr(expr)) => println!("{}", print_expr(&expr)),
        Ok(TopLevel::Element(element)) => println!("{}", print_element(&element)),
        Err(err) => return Err(CliError::Compile(err.to_string())),
    }
    Ok(())
}
