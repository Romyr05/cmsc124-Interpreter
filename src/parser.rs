use crate::token_types::Token;
use crate::ast::{ Expr, Value };

// Recursive Descent Parser

// test: when recieving 2 + 3 * 4, it should output the tree inorder (2 + (3 * 4))

let tree = Expr::Binary {
    left: num(2.0),
    operator: Token::new(TokenType::Plus, "+", 1),
    right: Box::new(Expr::Binary {
        left: num(3.0),
        operator: Token::new(TokenType::Star, "*", 1),
        right: num(4.0),
    }),
};
