mod ast;
mod evaluator;
pub mod keyword_list;
mod parser;
mod repl;
mod scanner;
mod token_types;
mod tree_printer;

use crate::evaluator::{evaluate, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
use crate::token_types::TokenType;
use crate::tree_printer::{print_element, print_expr};
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        // no command: drop into the interactive REPL
        None => repl::run(),
        Some("--parse") => match args.get(2) {
            Some(path) => parse_file(path),
            None => {
                eprintln!("--parse needs a file");
                exit(64);
            }
        },
        Some("--eval") => match args.get(2) {
            Some(path) => eval_file(path),
            None => {
                eprintln!("--eval needs a file");
                exit(64);
            }
        },
        Some("--tokenize") => match args.get(2) {
            Some(path) => tokenize_file(path),
            None => {
                eprintln!("--tokenize needs a file");
                exit(64);
            }
        },
        Some(other) => {
            eprintln!("Unknown command: {}", other);
            exit(64);
        }
    }
}

// --tokenize: print the scanned tokens
fn tokenize_file(path: &str) {
    let source = fs::read_to_string(path).expect("could not read input file");
    let (tokens, errors) = Tokenizer::new(&source).scan();

    // For newline token to hide them
    for token in &tokens {
        if token.token_type != TokenType::Newline {
            println!("{:?}", token);
        }
    }
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        exit(65);
    }
}

// --eval: evaluate each expression in the file and print its value, one per line.
//Parse then -> Run
fn eval_file(path: &str) {
    //Scanning whole file -> check if there are errors
    let source = fs::read_to_string(path).expect("could not read input file");
    let (tokens, errors) = Tokenizer::new(&source).scan();

    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        exit(65);
    }

    // check phase: report every syntax error; no running
    // Basically parsing whole file; stop -> error
    let (items, errors) = Parser::new(tokens).parse_program();
    for err in &errors {
        eprintln!("{}", err);
    }
    if !errors.is_empty() {
        exit(65); // rejected before running
    }

    // run phase: the whole file is valid, stop at the first runtime error
    // We evaluate here
    for item in &items {
        match item {
            TopLevel::Expr(expr) => match evaluate(expr) {
                Ok(value) => println!("{}", stringify(&value)),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(70); // started running, then hit a runtime error
                }
            },
            // elements parse, but evaluating them isn't supported yet
            TopLevel::Element(_) => {
                eprintln!("Runtime error: Elements can't be evaluated yet.");
                exit(70);
            }
        }
    }
}

// --parse: print the AST tree
fn parse_file(path: &str) {
    let source = fs::read_to_string(path).expect("could not read input file");
    let (tokens, errors) = Tokenizer::new(&source).scan();

    // a scan error means the tokens are unreliable, so stop before parsing
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        exit(65);
    }

    let (items, errors) = Parser::new(tokens).parse_program();
    for err in &errors {
        eprintln!("{}", err);
    }
    if !errors.is_empty() {
        exit(65);
    }

    for item in &items {
        match item {
            TopLevel::Expr(expr) => println!("{}", print_expr(expr)),
            TopLevel::Element(element) => println!("{}", print_element(element)),
        }
    }
}
