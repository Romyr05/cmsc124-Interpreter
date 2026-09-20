// enum token with different types, kulang pa ni
use crate::keyword_list::keywords_lookup;
use crate::token_types::{Token, TokenType};
use std::fmt;

// the malformed-input cases the scanner can report
pub enum ScanError {
    UnterminatedString { line: usize },
    UnexpectedChar { line: usize, ch: char },
    UnterminatedComment { line: usize },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::UnterminatedString { line } => {
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
            ScanError::UnexpectedChar { line, ch } => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, ch)
            }
            ScanError::UnterminatedComment { line } => {
                write!(f, "[line {}] Error: Unterminated comment", line)
            }
        }
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    cursor: usize,
    // 1-based line number, advanced as newlines are consumed, for error reports
    line: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            line: 1,
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

    // skip whitespace entirely (no token emitted); count newlines so lines stay accurate
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.line += 1;
                self.cursor += c.len_utf8();
            } else if c.is_whitespace() {
                self.cursor += c.len_utf8();
            } else {
                break;
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
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Star),
            '/' => Some(TokenType::Slash),
            _ => None,
        }
    }
}

// we impl iterator so that can just run .collect() on the tokenizer to get the tokens
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        // loop so comments (and whitespace) can be skipped and scanning continues
        loop {
            self.skip_whitespace();

            let c = self.peek()?; // None at end of input
            let line = self.line; // the line this token STARTS on
            let start = self.cursor;

            // Checkers
            if c.is_ascii_digit() {
                self.consume_while(|c| c.is_ascii_digit());
                let text = &self.source[start..self.cursor];
                return Some(Ok(Token::new(TokenType::Number, text, line)));
            }

            // alphabetic to not get the numbers
            // start with a letter or '_', then letters, digits or '_'
            if c.is_ascii_alphabetic() || c == '_' {
                self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                let text = &self.source[start..self.cursor];
                return Some(Ok(Self::word_token(text, line)));
            }

            // string literal: "..." may span lines; error only if EOF hits first
            if c == '"' {
                self.cursor += c.len_utf8(); // consume opening quote
                let content_start = self.cursor;
                let mut closed = false;
                while let Some(ch) = self.peek() {
                    if ch == '"' {
                        closed = true;
                        break;
                    }
                    if ch == '\n' {
                        self.line += 1;
                    }
                    self.cursor += ch.len_utf8();
                }
                if closed {
                    let text = &self.source[content_start..self.cursor];
                    self.cursor += c.len_utf8(); // Consumes the last quote
                    return Some(Ok(Token::new(TokenType::String, text, line)));
                } else {
                    return Some(Err(ScanError::UnterminatedString { line }));
                }
            }

            // comments start with '#'
            if c == '#' {
                self.cursor += c.len_utf8(); // consume '#'
                if self.consume_if('*') {
                    // block comment '#* ... *#': may span lines, error if never closed
                    loop {
                        if self.remaining().is_empty() {
                            return Some(Err(ScanError::UnterminatedComment { line }));
                        }
                        if self.remaining().starts_with("*#") {
                            self.cursor += 2; // consume closing '*#'
                            break;
                        }
                        let ch = self.peek().unwrap();
                        if ch == '\n' {
                            self.line += 1;
                        }
                        self.cursor += ch.len_utf8();
                    }
                } else {
                    // inline comment: run to end of line, the newline is left for skip_whitespace
                    self.consume_while(|c| c != '\n');
                }
                continue; // comment produces no token; scan the next one
            }

            // '==' equality operator, otherwise a single '=' assignment
            if c == '=' {
                self.cursor += c.len_utf8();
                let tt = if self.consume_if('=') {
                    TokenType::Equality
                } else {
                    TokenType::Equal
                };
                let text = &self.source[start..self.cursor];
                return Some(Ok(Token::new(tt, text, line)));
            }

            // other single-character tokens: ( ) + - * /
            if let Some(tt) = Self::single_char_type(c) {
                self.cursor += c.len_utf8();
                let text = &self.source[start..self.cursor];
                return Some(Ok(Token::new(tt, text, line)));
            }

            // any other, report it and continue
            self.cursor += c.len_utf8();
            return Some(Err(ScanError::UnexpectedChar { line, ch: c }));
        }
    }
}
