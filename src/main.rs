mod ast;
pub mod keyword_list;
mod parser;
mod repl;
mod scanner;
mod token_types;
mod tree_printer;

use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
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

    for token in &tokens {
        println!("{:?}", token);
    }
    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        exit(65);
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

    let mut parser = Parser::new(tokens);
    match parser.parse_top_level() {
        Ok(TopLevel::Expr(expr)) => println!("{}", print_expr(&expr)),
        Ok(TopLevel::Element(element)) => println!("{}", print_element(&element)),
        Err(err) => {
            eprintln!("{}", err);
            exit(65);
        }
    }
}
