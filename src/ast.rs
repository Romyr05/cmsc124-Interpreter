use crate::token_types::Token;

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub enum Value {
    Number(f64),
    Str(String), // owned, so Value needs no lifetime
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone)]
#[expect(dead_code)]
pub enum Expr<'a> {
    Literal { value: Value },
    Unary { operator: Token<'a>, right: Box<Expr<'a>> },
    Binary { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },
    Grouping { expression: Box<Expr<'a>> },
}