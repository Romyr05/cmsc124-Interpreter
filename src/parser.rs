use crate::token_types::Token;
use crate::scanner::Tokenizer;
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

pub struct Parser {
    tokens: Vec<Token<'a>>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, current: 0 }
    }

    //helper functions
    fn peek(&self) -> Token<'a>{
        self.tokens[self.current]
    }

    fn previous(&self) -> Token<'a> {
        self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> Token<'a> {
        let token = self.peek();
        if !self.is_at_end() {
            self.current += 1;
        }
        token
    }

    fn isType(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn consumeOnType(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token<'a>, ParseError<'a>> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.peek(),
                message: message.to_string(),
            })
        }
    }

    //expression
    fn expression(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.consumeOnType(&[TokenType::Number]){
            let token = self.previous();
            let n: f64 = token.lexeme
            return Expr::Literal { value: Value::Number(n) }
        }

        if self.consumeOnType(&[TokenType::String]) {
            let token = self.previous();
            let text = token.lexeme.to_string();
            return Expr::Literal { value: Value::Str(text) };
        }

        if self.match_any(&[TokenType::LeftParen]) {
            let inner = self.expression(); 
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(inner) });
        }
    }

}