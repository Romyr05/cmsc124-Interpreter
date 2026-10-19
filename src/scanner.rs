// enum token with different types, kulang pa ni
use crate::errors::ScanError;
use crate::keyword_list::keywords_lookup;
use crate::token_types::TokenType::Eof;
use crate::token_types::{Token, TokenType};

pub struct Tokenizer<'a> {
    source: &'a str,
    cursor: usize,
    // 1-based line number, advanced as newlines are consumed, for error reports
    line: usize,
    // how many ( or { are open; a newline inside brackets is plain whitespace
    depth: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            line: 1,
            depth: 0,
        }
    }

    // helper function to get remaining chars from string
    fn remaining(&self) -> &'a str {
        &self.source[self.cursor..]
    }

    // peek at the next character without consuming it
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    // consume the next character only if it matches; report whether it did
    fn consume_if(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.cursor += c.len_utf8();
                true
            }
            _ => false,
        }
    }

    // consume characters while the condition holds (Fn closure captures nothing here)
    fn consume_while(&mut self, condition: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if condition(c) {
                self.cursor += c.len_utf8();
            } else {
                break;
            }
        }
    }

    // skip whitespace (no token emitted); stops at '\n' so scan() can give out a Newline
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_whitespace() && c != '\n' => self.cursor += c.len_utf8(), //added guard clause
                _ => break,
            }
        }
    }

    // a word is a keyword if it matches the list, otherwise an identifier
    fn word_token(text: &'a str, line: usize) -> Token<'a> {
        Token::new(keywords_lookup(text), text, line)
    }

    // single character tokens; '=' and '#' are handled separately because they
    // can start the '==' operator and comments respectively
    fn single_char_type(c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Star),
            '/' => Some(TokenType::Slash),
            '.' => Some(TokenType::Dot),
            '^' => Some(TokenType::Caret),
            _ => None,
        }
    }
    // scans each token and then logic based
    pub fn scan(mut self) -> (Vec<Token<'a>>, Vec<ScanError>) {
        //Tokens and Error Vectors
        let mut tokens = Vec::new();
        let mut error = Vec::new();

        // loop so comments (and whitespace) can be skipped and scanning continues
        loop {
            self.skip_whitespace();

            let c = match self.peek() {
                Some(c) => c,
                // EOF if no tokens
                None => {
                    tokens.push(Token::new(Eof, "", self.line));
                    break;
                }
            };

            let start = self.cursor;

            match c {
                // newline ends an expression, but only outside brackets;
                // blank lines and a leading newline produce no token
                '\n' => {
                    self.cursor += c.len_utf8();
                    let last_is_newline =
                        matches!(tokens.last(), Some(t) if t.token_type == TokenType::Newline);

                    // For the second newline guard clause (for if the next is )
                    if self.depth == 0 && !tokens.is_empty() && !last_is_newline {
                        tokens.push(Token::new(TokenType::Newline, "", self.line));
                    }
                    self.line += 1; // after pushing, so the token reports the line it ends
                }

                // Numbers
                '0'..='9' => {
                    self.consume_while(|c| c.is_ascii_digit());
                    let mut num_type = TokenType::Number;

                    // Float
                    // only when a digit follows the '.'
                    let mut after = self.remaining().chars();
                    if after.next() == Some('.') && after.next().is_some_and(|c| c.is_ascii_digit())
                    {
                        self.cursor += '.'.len_utf8();
                        self.consume_while(|c| c.is_ascii_digit());
                        num_type = TokenType::Float;
                    }
                    let text = &self.source[start..self.cursor];
                    tokens.push(Token::new(num_type, text, self.line));
                }

                // alphabetic to not get the numbers
                // start with a letter or '_', then letters, digits or '_'
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                    let text = &self.source[start..self.cursor];
                    tokens.push(Self::word_token(text, self.line));
                }

                // string literal: "..." may span lines; error only if EOF hits first
                // Same yung " "
                '"' => {
                    self.cursor += c.len_utf8(); // consume opening quote
                    let content_start = self.cursor;
                    let mut closed = false;
                    while let Some(ch) = self.peek() {
                        match ch {
                            '"' => {
                                closed = true;
                                break;
                            }
                            '\n' => self.line += 1,
                            _ => {}
                        }
                        self.cursor += ch.len_utf8();
                    }
                    if closed {
                        let text = &self.source[content_start..self.cursor];
                        self.cursor += c.len_utf8(); // Consumes the last quote
                        tokens.push(Token::new(TokenType::String, text, self.line));
                    } else {
                        error.push(ScanError::UnterminatedString { line: self.line });
                    }
                }

                // comments start with '#'; a comment produces no token
                '#' => {
                    self.cursor += c.len_utf8(); // consume '#'
                    if self.consume_if('*') {
                        // block comment '#* ... *#': may span lines, error if never closed
                        loop {
                            match self.peek() {
                                None => {
                                    error.push(ScanError::UnterminatedComment { line: self.line });
                                    break;
                                }
                                Some(_) if self.remaining().starts_with("*#") => {
                                    self.cursor += 2; // consume closing '*#'
                                    break;
                                }
                                Some('\n') => {
                                    self.line += 1;
                                    self.cursor += '\n'.len_utf8();
                                }
                                Some(ch) => self.cursor += ch.len_utf8(),
                            }
                        }
                    } else {
                        // inline comment: run to end of line, the newline is left for the '\n' arm
                        self.consume_while(|c| c != '\n');
                    }
                }

                // '==' equality operator, otherwise a single '=' assignment
                '=' => {
                    self.cursor += c.len_utf8();
                    let tt = if self.consume_if('=') {
                        TokenType::Equality
                    } else {
                        TokenType::Equal
                    };
                    let text = &self.source[start..self.cursor];
                    tokens.push(Token::new(tt, text, self.line));
                }

                // '<=' otherwise a single '<'
                '<' => {
                    self.cursor += c.len_utf8();
                    let tt = if self.consume_if('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    let text = &self.source[start..self.cursor];
                    tokens.push(Token::new(tt, text, self.line));
                }

                // '>=' otherwise a single '>'
                '>' => {
                    self.cursor += c.len_utf8();
                    let tt = if self.consume_if('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    let text = &self.source[start..self.cursor];
                    tokens.push(Token::new(tt, text, self.line));
                }

                // '!=' not-equal; a lone '!' is not a valid token
                '!' => {
                    self.cursor += c.len_utf8();
                    if self.consume_if('=') {
                        let text = &self.source[start..self.cursor];
                        tokens.push(Token::new(TokenType::NotEqual, text, self.line));
                    } else {
                        error.push(ScanError::UnexpectedChar {
                            line: self.line,
                            ch: c,
                        });
                    }
                }

                _ => match Self::single_char_type(c) {
                    // other single-character tokens: ( ) + - * /
                    Some(tt) => {
                        self.cursor += c.len_utf8();
                        let text = &self.source[start..self.cursor];
                        tokens.push(Token::new(tt, text, self.line));
                        // track open brackets; saturating_sub so a stray ')' can't underflow
                        match tt {
                            TokenType::LeftParen | TokenType::LeftBrace => self.depth += 1,
                            TokenType::RightParen | TokenType::RightBrace => {
                                self.depth = self.depth.saturating_sub(1) // if overflowed then stops it on the max boundary
                            }
                            _ => {}
                        }
                    }
                    // any other, report it and continue
                    None => {
                        self.cursor += c.len_utf8();
                        error.push(ScanError::UnexpectedChar {
                            line: self.line,
                            ch: c,
                        });
                    }
                },
            }
        }
        (tokens, error)
    }
}
