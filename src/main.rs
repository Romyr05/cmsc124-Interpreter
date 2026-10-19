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
use crate::evaluator::{evaluate, stringify};
use crate::parser::{Parser, TopLevel};
use crate::scanner::Tokenizer;
use crate::token_types::{Token, TokenType};
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

// --eval: evaluate each expression in the file and print its value, one per line.
// One expression per line, so we group the scanned tokens by line number.
fn eval_file(path: &str) {
    let source = fs::read_to_string(path).expect("could not read input file");
    let (tokens, errors) = Tokenizer::new(&source).scan();

    for e in &errors {
        eprintln!("{}", e);
    }
    if !errors.is_empty() {
        exit(65);
    }

    // split the flat token stream into one group per source line
    let mut groups: Vec<Vec<Token>> = Vec::new();
    let mut current_line: Option<usize> = None;
    for token in tokens {
        if token.token_type == TokenType::Eof {
            continue;
        }
        if current_line != Some(token.line) {
            groups.push(Vec::new());
            current_line = Some(token.line);
        }
        groups.last_mut().unwrap().push(token);
    }

    for mut group in groups {
        let line = group[0].line;
        group.push(Token::new(TokenType::Eof, "", line)); // parser expects a trailing Eof

        let mut parser = Parser::new(group);
        match parser.parse_top_level() {
            Ok(TopLevel::Expr(expr)) => match evaluate(&expr) {
                Ok(value) => println!("{}", stringify(&value)),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(70); // started running, then hit a runtime error
                }
            },
            Ok(TopLevel::Element(element)) => match generate(&element) {
                Ok(html) => println!("{}", html),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(70);
                }
            },
            Err(err) => {
                eprintln!("{}", err);
                exit(65); // rejected before running
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
