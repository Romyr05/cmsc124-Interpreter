use std::fmt;

use crate::ast::{ Attribute, Element, Expr, Value };
use crate::scanner::Tokenizer;
use crate::keyword_list::{ is_element_keyword, is_attribute_keyword };
use crate::token_types::{ Token, TokenType };
use crate::tree_printer::{ print_expr, print_element };

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

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

#[derive(Debug)]
pub enum TopLevel<'a> {
    Expr(Expr<'a>),
    Element(Element<'a>),
}

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

    fn expect(
        &mut self,
        token_type: TokenType,
        message: &str
    ) -> Result<&Token<'a>, ParseError<'a>> {
        if self.is_type(token_type) {
            return Ok(self.advance());
        }
        Err(ParseError {
            token: *self.peek(),
            message: message.to_string(),
        })
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

        // IMPORTANT: For now, mark identifiers as strings as well.
        if self.consume_on_type(&[TokenType::String, TokenType::Identifier]) {
            let raw = self.previous().lexeme;
            let s = raw.trim_matches('"').to_string();
            return Ok(Expr::Literal {
                value: Value::Str(s),
            });
        }

        if self.consume_on_type(&[TokenType::LeftParen]) {
            let inner = self.expression()?;
            self.expect(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping {
                expression: Box::new(inner),
            });
        }

        Err(ParseError {
            token: *self.peek(),
            message: "Expected expression".to_string(),
        })
    }

    fn element(&mut self) -> Result<Element<'a>, ParseError<'a>> {
        let kind = *self.peek();
        if !is_element_keyword(kind.token_type) {
            return Err(ParseError {
                token: kind,
                message: "Expected an element keyword (button, box, text, image, div, par)".to_string(),
            });
        }
        self.advance();

        self.expect(TokenType::LeftParen, "Expected '(' after element keyword")?;
        let attributes = self.attribute_list()?;
        self.expect(TokenType::RightParen, "Expected ')' after attributes")?;

        self.expect(TokenType::LeftBrace, "Expected '{' to start element body")?;
        let mut children = Vec::new();
        while !self.is_type(TokenType::RightBrace) && !self.is_at_end() {
            if is_element_keyword(self.peek().token_type) {
                children.push(TopLevel::Element(self.element()?));
            } else {
                children.push(TopLevel::Expr(self.expression()?));
            }
        }
        self.expect(TokenType::RightBrace, "Expected '}' to close element body")?;

        Ok(Element {
            kind: kind.token_type,
            attributes,
            children,
        })
    }

    fn attribute_list(&mut self) -> Result<Vec<Attribute<'a>>, ParseError<'a>> {
        let mut attributes = Vec::new();

        // Empty parens, e.g. div() { ... }, is valid: no attributes.
        if self.is_type(TokenType::RightParen) {
            return Ok(attributes);
        }

        attributes.push(self.attribute()?);
        while self.consume_on_type(&[TokenType::Comma]) {
            attributes.push(self.attribute()?);
        }

        Ok(attributes)
    }

    fn attribute(&mut self) -> Result<Attribute<'a>, ParseError<'a>> {
        let kind = *self.peek();
        if !is_attribute_keyword(kind.token_type) {
            return Err(ParseError {
                token: kind,
                message: "Expected an attribute name (id, class, align, padding, margin, \
                          height, width, color, border)".to_string(),
            });
        }
        self.advance();

        self.expect(TokenType::Equal, "Expected '=' after attribute name")?;
        let value = self.primary()?;

        Ok(Attribute {
            kind: kind.token_type,
            value,
        })
    }

    fn is_at_element_start(&self) -> bool {
        !self.is_at_end() && is_element_keyword(self.peek().token_type)
    }

    pub fn parse_top_level(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        let result = if self.is_at_element_start() {
            TopLevel::Element(self.element()?)
        } else {
            TopLevel::Expr(self.expression()?)
        };

        // Same leftover-input check as before, now shared by both branches.
        if !self.is_at_end() {
            return Err(ParseError {
                token: *self.peek(),
                message: "Expected end of input".to_string(),
            });
        }
        Ok(result)
    }
}

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
    match parser.parse_top_level() {
        Ok(TopLevel::Expr(expr)) => println!("{}", print_expr(&expr)),
        Ok(TopLevel::Element(element)) => println!("{}", print_element(&element)),
        Err(err) => eprintln!("{}", err),
    }
}
