use crate::ast::{Attribute, Element, Expr, Value};
use crate::errors::ParseError;
use crate::keyword_list::{is_attribute_keyword, is_element_keyword};
use crate::token_types::{Token, TokenType};

// Recursive Descent Parser
//
// Grammar:
//   expression -> term
//   term       -> factor ( ( "-" | "+" ) factor )*
//   factor     -> primary ( ( "*" | "/" ) primary )*
//   primary    -> NUMBER
//
// test: when receiving 2 + 3 * 4, it should output the tree as (2 + (3 * 4))

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
    // every syntax error found so far, in source order (top level and element bodies)
    errors: Vec<ParseError<'a>>,
}

#[derive(Debug)]
//This just basically means a statemetn
pub enum TopLevel<'a> {
    Expr(Expr<'a>),
    Element(Element<'a>),
    // let name = initializer; line points at the name for error messages
    // initializer is None for `let x` (declared, no value or type yet)
    Let {
        name: String,
        initializer: Option<Expr<'a>>,
        line: usize,
    },
    // print(expression): writes the value to the terminal, never into HTML
    Print(Expr<'a>),
    // name = value: updates the nearest existing binding; line points at the name
    Assign {
        name: String,
        value: Expr<'a>,
        line: usize,
    },
    // if (condition) { then_branch } else { else_branch }; else is optional.
    // `else if` is stored as an else_branch holding one If
    If {
        condition: Expr<'a>,
        then_branch: Vec<TopLevel<'a>>,
        else_branch: Option<Vec<TopLevel<'a>>>,
    },
    // while (condition) { body }
    While {
        condition: Expr<'a>,
        body: Vec<TopLevel<'a>>,
    },
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    // ---------- helper functions ----------

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current]
    }

    // one token past peek()
    fn peek_next(&self) -> &Token<'a> {
        &self.tokens[(self.current + 1).min(self.tokens.len() - 1)] // .min for boundary checking to not pass its len no.
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
        message: &str,
    ) -> Result<&Token<'a>, ParseError<'a>> {
        match self.peek().token_type {
            t if t == token_type && !self.is_at_end() => Ok(self.advance()),
            _ => Err(ParseError {
                token: *self.peek(),
                message: message.to_string(),
            }),
        }
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

    // after a binary operator the line can't be finished, so a newline there is a continuation
    fn skip_newlines(&mut self) {
        while self.is_type(TokenType::Newline) {
            self.advance();
        }
    }

    // ---------- grammar rules ----------
    // Every rule returns Expr<'a> so the result borrows from the source
    // text ('a), NOT from the &mut self borrow.

    // assignment is a statement; which is our decisin  , so `=` is not part of
    // the expression grammar: `let y = (x = 5)` is a parse error
    fn expression(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        self.logic_or()
    }

    fn logic_and(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.logic_not()?;

        while self.consume_on_type(&[TokenType::LogicAnd]) {
            let operator = *self.previous();
            self.skip_newlines();
            let right = self.logic_not()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    fn logic_or(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.logic_and()?;

        while self.consume_on_type(&[TokenType::LogicOr]) {
            let operator = *self.previous();
            self.skip_newlines();
            let right = self.logic_and()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    // not (unary prefix): not not a works
    fn logic_not(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        match self.peek().token_type {
            TokenType::Not => {
                let operator = *self.advance();
                let right = self.logic_not()?;
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => self.comparisons(),
        }
    }

    fn comparisons(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.term()?;
        while self.consume_on_type(&[
            TokenType::Equality,
            TokenType::NotEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = *self.previous();
            self.skip_newlines();
            let right = self.term()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    // - and +
    fn term(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.factor()?;

        while self.consume_on_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = *self.previous();
            self.skip_newlines();
            let right = self.factor()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    // * and /
    fn factor(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let mut node = self.unary()?;

        while self.consume_on_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = *self.previous();
            self.skip_newlines();
            let right = self.unary()?;
            node = Expr::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    //for those --x works
    fn unary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        match self.peek().token_type {
            TokenType::Minus => {
                let operator = *self.advance();
                let right = self.unary()?;
                Ok(Expr::Unary {
                    operator,
                    right: (Box::new(right)),
                })
            }
            _ => self.exponent(),
        }
    }

    // ^ (right-associative: 2^3^2 = 2^(3^2))
    fn exponent(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        let node = self.primary()?;

        match self.peek().token_type {
            TokenType::Caret => {
                let operator = *self.advance();
                self.skip_newlines();
                let right = self.unary()?;
                Ok(Expr::Binary {
                    left: Box::new(node),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(node),
        }
    }

    // numbers
    fn primary(&mut self) -> Result<Expr<'a>, ParseError<'a>> {
        // TODO: add proper error handling, strings, and parentheses
        match self.peek().token_type {
            TokenType::Number | TokenType::Float => {
                let token = *self.advance();
                let n: f64 = match token.lexeme.parse() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(ParseError {
                            token,
                            message: "Invalid number literal".to_string(),
                        });
                    }
                };
                Ok(Expr::Literal {
                    value: Value::Number(n),
                })
            }

            // the keyword lookup only makes Boolean for "true"/"false"
            TokenType::Boolean => {
                let b = self.advance().lexeme == "true";
                Ok(Expr::Literal {
                    value: Value::Bool(b),
                })
            }

            TokenType::Null => {
                self.advance();
                Ok(Expr::Literal { value: Value::Nil })
            }

            TokenType::String => {
                let raw = self.advance().lexeme;
                let s = raw.trim_matches('"').to_string();
                Ok(Expr::Literal {
                    value: Value::Str(s),
                })
            }

            // separated it from the string already so its good na
            // just the same but with name and line for error handling purposes
            TokenType::Identifier => {
                let token = *self.advance();
                Ok(Expr::Variable {
                    name: token.lexeme.to_string(),
                    line: token.line,
                })
            }

            TokenType::LeftParen => {
                self.advance();
                let inner = self.expression()?;
                self.expect(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(Expr::Grouping {
                    expression: Box::new(inner),
                })
            }

            _ => Err(ParseError {
                token: *self.peek(),
                message: "Expected expression".to_string(),
            }),
        }
    }

    fn element(&mut self) -> Result<Element<'a>, ParseError<'a>> {
        let kind = *self.peek();
        if !is_element_keyword(kind.token_type) {
            return Err(ParseError {
                token: kind,
                message: "Expected an element keyword (button, box, text, image, div, par)"
                    .to_string(),
            });
        }
        self.advance();

        // Gets the element
        self.expect(TokenType::LeftParen, "Expected '(' after element keyword")?;
        let attributes = self.attribute_list()?;
        self.expect(TokenType::RightParen, "Expected ')' after attributes")?;

        // Gets the children
        let children = self.body(true, "element")?;

        Ok(Element {
            kind: kind.token_type,
            attributes,
            children,
        })
    }

    // { children }: shared by element bodies and if/else bodies.
    // Newlines separate children; blank lines are skipped, and children may also
    // share a line. A bad child is recorded and skipped; the rest keeps parsing.
    // `what` names the body in error messages ("element", "if", "else", "while").
    fn body(
        &mut self,
        allow_elements: bool,
        what: &str,
    ) -> Result<Vec<TopLevel<'a>>, ParseError<'a>> {
        self.expect(
            TokenType::LeftBrace,
            &format!("Expected '{{' to start {} body", what),
        )?;
        let mut children = Vec::new();
        while !self.is_type(TokenType::RightBrace) && !self.is_at_end() {
            if self.consume_on_type(&[TokenType::Newline]) {
                continue;
            }
            match self.body_child(allow_elements) {
                Ok(child) => children.push(child),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize_body();
                }
            }
        }
        self.expect(
            TokenType::RightBrace,
            &format!("Expected '}}' to close {} body", what),
        )?;
        Ok(children)
    }

    // if ( expression ) { statements } ( else if ... | else { statements } )?
    // (the `if` was already consumed). `else` must be on the same line as the `}`,
    // since the newline after `}` ends the statement. A dangling else binds to the
    // nearest if: the innermost if_statement call is the one that sees it.
    fn if_statement(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        self.expect(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.expect(TokenType::RightParen, "Expected ')' after if condition")?;
        let then_branch = self.body(false, "if")?;
        let else_branch = if self.consume_on_type(&[TokenType::Else]) {
            if self.consume_on_type(&[TokenType::If]) {
                // else if: an else whose body is another if
                Some(vec![self.if_statement()?])
            } else {
                Some(self.body(false, "else")?)
            }
        } else {
            None
        };
        Ok(TopLevel::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // while ( expression ) { statements }   (the `while` was already consumed)
    fn while_statement(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        self.expect(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.expect(TokenType::RightParen, "Expected ')' after while condition")?;
        let body = self.body(false, "while")?;
        Ok(TopLevel::While { condition, body })
    }

    // one child of an element body: a nested element, a let, a print, or an expression
    // (also if/else bodies, which pass allow_elements = false: statements only)
    fn body_child(&mut self, allow_elements: bool) -> Result<TopLevel<'a>, ParseError<'a>> {
        if is_element_keyword(self.peek().token_type) {
            if !allow_elements {
                return Err(ParseError {
                    token: *self.peek(),
                    message: "Elements are not allowed inside an if or while body".to_string(),
                });
            }
            Ok(TopLevel::Element(self.element()?))
        } else if self.consume_on_type(&[TokenType::If]) {
            self.if_statement()
        } else if self.consume_on_type(&[TokenType::While]) {
            self.while_statement()
        } else if self.consume_on_type(&[TokenType::Initialize]) {
            self.let_declaration()
        } else if self.consume_on_type(&[TokenType::Print]) {
            self.print_statement()
        } else if self.is_at_assignment() {
            self.assignment_statement() // for those identifier followed by =
        } else {
            self.expression_statement() // next to identifier is an not = 
        }
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
                          height, width, color, border)"
                    .to_string(),
            });
        }
        self.advance();

        self.expect(TokenType::Equal, "Expected '=' after attribute name")?;
        let value = self.expression()?; // Chaned to allow calculations for the GUI

        Ok(Attribute {
            kind: kind.token_type,
            value,
        })
    }

    fn is_at_element_start(&self) -> bool {
        !self.is_at_end() && is_element_keyword(self.peek().token_type)
    }

    // parse every item in the file; on an error, record it and resume on the next line
    // so all syntax errors are reported at once
    pub fn parse_program(&mut self) -> Result<Vec<TopLevel<'a>>, Vec<ParseError<'a>>> {
        let mut items = Vec::new();
        while !self.is_at_end() {
            match self.parse_top_level() {
                Ok(item) => items.push(item),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }
        // errors from inside element bodies were already pushed while parsing them
        if self.errors.is_empty() {
            Ok(items)
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    // one element or expression, ended by a newline or the end of the file
    pub fn parse_top_level(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        let result = if self.is_at_element_start() {
            TopLevel::Element(self.element()?)
        } else if self.consume_on_type(&[TokenType::Initialize]) {
            self.let_declaration()?
            // When printing is found
        } else if self.consume_on_type(&[TokenType::Print]) {
            self.print_statement()?
        } else if self.consume_on_type(&[TokenType::If]) {
            self.if_statement()?
        } else if self.consume_on_type(&[TokenType::While]) {
            self.while_statement()?
        } else if self.is_at_assignment() {
            self.assignment_statement()?
        } else {
            self.expression_statement()?
        };

        // the terminator check, shared by both branches; consumes the newline
        if !self.is_at_end() && !self.consume_on_type(&[TokenType::Newline]) {
            return Err(ParseError {
                token: *self.peek(),
                message: "Expected newline after expression".to_string(),
            });
        }
        Ok(result)
    }

    // let NAME ( = expression )?   (the `let` was already consumed)
    fn let_declaration(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        let name = *self.expect(TokenType::Identifier, "Expected variable name after 'let'")?;
        // `= expression` is optional: `let x` declares x with no value yet
        let initializer = if self.consume_on_type(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(TopLevel::Let {
            name: name.lexeme.to_string(),
            initializer,
            line: name.line,
        })
    }

    // an assignment starts with a name followed by '='
    fn is_at_assignment(&self) -> bool {
        self.peek().token_type == TokenType::Identifier
            && self.peek_next().token_type == TokenType::Equal
    }

    // NAME = expression   (nothing consumed yet; is_at_assignment checked the first two tokens)
    fn assignment_statement(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        let name = *self.advance();
        self.advance(); // the '='
        self.skip_newlines(); // a line may end right after '=', like after an operator
        let value = self.expression()?;
        // `a = b = 3`: the value was `b`, and another '=' follows
        if self.is_type(TokenType::Equal) {
            return Err(ParseError {
                token: *self.peek(),
                message: "Assignment is a statement and can't be chained.".to_string(),
            });
        }
        Ok(TopLevel::Assign {
            name: name.lexeme.to_string(),
            value,
            line: name.line,
        })
    }

    // a bare expression on its own; a '=' right after it means the left side
    // wasn't a plain name, e.g. `(1 + 2) = 3`
    fn expression_statement(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        let expr = self.expression()?;
        if self.is_type(TokenType::Equal) {
            return Err(ParseError {
                token: *self.peek(),
                message: "Invalid assignment target.".to_string(),
            });
        }
        Ok(TopLevel::Expr(expr))
    }

    // print ( expression )   (the `print` was already consumed)
    // `print x` fails at '(', `print()` fails in expression(), a missing ')' fails at ')'
    fn print_statement(&mut self) -> Result<TopLevel<'a>, ParseError<'a>> {
        self.expect(TokenType::LeftParen, "Expected '(' after 'print'")?;
        let expr = self.expression()?;
        self.expect(TokenType::RightParen, "Expected ')' after print value")?;
        Ok(TopLevel::Print(expr))
    }

    // error recovery: discard tokens up to and including the next newline,
    // so parsing resumes at the start of the next line. A { ... } opened on the
    // bad line is skipped whole (its inner newlines don't count), so a broken
    // `if` or element header doesn't leave a stray '}' to be reported again.
    fn synchronize(&mut self) {
        let mut depth: usize = 0;
        while !self.is_at_end() {
            match self.advance().token_type {
                TokenType::LeftBrace => depth += 1,
                TokenType::RightBrace => depth = depth.saturating_sub(1),
                TokenType::Newline if depth == 0 => return,
                _ => {}
            }
        }
    }

    // error recovery inside an element body: discard the rest of the bad line.
    // Stops after a newline, or before the '}' that closes this body (left for the
    // children loop to close the element). A nested { ... } on the bad line is
    // skipped whole, so its '}' doesn't close this body early.
    fn synchronize_body(&mut self) {
        let mut depth = 0;
        while !self.is_at_end() {
            match self.peek().token_type {
                TokenType::RightBrace if depth == 0 => return,
                TokenType::RightBrace => depth -= 1,
                TokenType::LeftBrace => depth += 1,
                TokenType::Newline if depth == 0 => {
                    self.advance();
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}
