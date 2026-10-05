use crate::token_types::{Token, TokenType};
use crate::parser::TopLevel;

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub enum Value {
    Number(f64),
    Str(String), // owned, so Value needs no lifetime
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub struct Element<'a> {
    pub kind: TokenType,
    pub attributes: Vec<Attribute<'a>>,
    pub children: Vec<TopLevel<'a>>,
}

#[derive(Debug)]
pub struct Attribute<'a> {
    pub kind: TokenType,
    pub value: Expr<'a>,
}

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub enum Expr<'a> {
    Literal {
        value: Value,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
}
