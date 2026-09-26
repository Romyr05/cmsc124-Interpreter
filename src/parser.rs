// refactor after to use the same components as scanner

use std::fmt;

use crate::ast::{Expr, Value};
use crate::scanner::Tokenizer;
use crate::token_types::{Token, TokenType};
use crate::tree_printer::print_expr;

// Recursive Descent Parser
//
// Grammar:
//   expression -> term
//   term       -> factor ( ( "-" | "+" ) factor )*
//   factor     -> primary ( ( "*" | "/" ) primary )*
//   primary    -> NUMBER
//
// test: when receiving 2 + 3 * 4, it should output the tree as (2 + (3 * 4))

#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Token<'a>,
    pub message: String,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(f, "Error at end: {}", self.message)
        } else {
            write!(f, "Error at '{}': {}", self.token.lexeme, self.message)
        }
    }
}

impl<'a> std::error::Error for ParseError<'a> {}

#[expect(dead_code)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

#[expect(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, current: 0 }
    }

    // ---------- helper functions ----------

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    // Advance first, then return the token we just consumed.
    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_type(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn consume_on_type(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.is_type(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    // ---------- grammar rules ----------
    // Every rule returns Expr<'a> so the result borrows from the source
    // text ('a), NOT from the &mut self borrow.

    fn expression(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.factor()?;

        while self.consume_on_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = *self.previous();
            let right = self.factor()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.primary()?;

        while self.consume_on_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = *self.previous();
            let right = self.primary()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    fn primary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        // TODO: add proper error handling, strings, and parentheses
        if self.consume_on_type(&[TokenType::Number]) {
            let n: f64 = self.previous().lexeme.parse().unwrap();
            return Ok(Expr::Literal {
                value: Value::Number(n),
            });
        }

        Err(ParseError {
            token: *self.peek(),
            message: "Expected expression".to_string(),
        })
    }

}

#[expect(dead_code)]
pub fn parse(src: &str) {
    let (tokens, errors) = Tokenizer::new(src).scan();

    for token in &tokens {
        println!("{:?}", token);
    }

    for error in &errors {
        println!("{}", error);
    }

    // TODO: report `errors` before parsing

    let mut parser = Parser::new(tokens);
    match parser.expression() {
        Ok(expr) => println!("{}", print_expr(&expr)),
        Err(err) => eprintln!("{}", err),
    }
}
