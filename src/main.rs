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
    // a flag starts with "--"; the path is the first argument that doesn't
    let flag = args.iter().skip(1).find(|a| a.starts_with("--")).cloned();
    let path = args.iter().skip(1).find(|a| !a.starts_with("--")).cloned();

    // no file argument: drop into the interactive REPL
    let path = match path {
        Some(p) => p,
        None => {
            repl::run();
            return;
        }
    };

    match flag.as_deref() {
        Some("--parse") => parse_file(&path),
        Some("--tokenize") | None => tokenize_file(&path),
        Some(other) => {
            eprintln!("Unknown flag: {}", other);
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
