// enum token with different types, kulang pa ni
use crate::{keyword_list::keywords_lookup, token_app::Token};
use std::fmt;

// the two malformed-input cases the scanner can report
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
                write!(f, "[line {}] Error: Unterminated comment", line,)
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

// Helper functions
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

    // Peeking for the next character
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    // consume if, consumes the if expected char after call
    fn consume_if(&mut self, expected_char: char) -> bool {
        // use this to "peek" at the next
        match self.peek() {
            Some(c) if c == expected_char => {
                self.cursor += c.len_utf8(); //Advances past
                true
            }
            _ => false,
        }
    }

    //Fn trait -> closure, captures external variables from its enclosing
    fn consume_while(&mut self, condition: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if condition(c) {
                self.cursor += c.len_utf8();
            } else {
                break;
            }
        }
    }

    // Token, if not identifier
    fn word_token(text: &'a str) -> Token<'a> {
        keywords_lookup(text).unwrap_or(Token::Identifier(text))
    }

    fn skip_whitespace(&mut self) {
        // peeking until matches
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.cursor += c.len_utf8();
                }
                '\n' => {
                    self.line += 1;
                    self.cursor += c.len_utf8();
                }
                // else
                _ => break,
            }
        }
    }
}

// we impl iterator so that can just run .collect() on the tokenizer to get the tokens
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace(); // Check if cursor is whitespace
        let remaining = self.remaining();
        if remaining.is_empty() {
            return None;
        }

        let mut chars = remaining.chars();
        let curr_char = chars.next();

        if let Some(c) = curr_char {
            // when it detects a digit, iterate until no more
            // digits and then return the whole number as a token
            if c.is_ascii_digit() {
                let start_pos = self.cursor;
                self.consume_while(|c| c.is_ascii_digit());
                let text = &self.source[start_pos..self.cursor];
                return Some(Ok(Token::Number(text)));
            }
            //Comment
            else if c == '/' {
                self.cursor += c.len_utf8();
                //Inline comment
                if self.consume_if('/') {
                    self.consume_while(|c| c != '\n');
                    return self.next(); // skip comment, yield next real token
                }
                //Block comment
                else if self.consume_if('*') {
                    let starting_line = self.line;

                    // consume until the closing */
                    while !self.remaining().is_empty() && !self.remaining().starts_with("*/") {
                        let char = self.peek().unwrap();
                        if char == '\n' {
                            // \n is only one char
                            self.line += 1
                        }
                        self.cursor += char.len_utf8();
                    }

                    if self.remaining().starts_with("*/") {
                        self.cursor += 2; // consume the closing */
                        return self.next(); // skip comment, yield next real token
                    } else {
                        return Some(Err(ScanError::UnterminatedComment {
                            line: starting_line,
                        }));
                    }
                }
                //Lone '/' Error
                return Some(Err(ScanError::UnexpectedChar {
                    line: self.line,
                    ch: '/',
                }));
            }
            // string literal: "..." may span lines; error only if EOF hits first
            else if c == '"' {
                self.cursor += c.len_utf8();
                let start_line = self.line;
                let content_start = self.cursor;

                while let Some(c) = self.peek() {
                    if c == '"' {
                        break; // found closer; break to continue on the consume below
                    }
                    if c == '\n' {
                        self.line += 1;
                    }
                    self.cursor += c.len_utf8();
                }

                if self.consume_if('"') {
                    let text = &self.source[content_start..self.cursor - 1]; // exclude closing quote
                    return Some(Ok(Token::String(text)));
                } else {
                    return Some(Err(ScanError::UnterminatedString { line: start_line }));
                }
            } else if
            // here for all the other token types
            c == '(' {
                self.cursor += c.len_utf8();
                return Some(Ok(Token::LeftParen(
                    &self.source[self.cursor - 1..self.cursor],
                )));
            } else if c == ')' {
                self.cursor += c.len_utf8();
                return Some(Ok(Token::RightParen(
                    &self.source[self.cursor - 1..self.cursor],
                )));
            } else if c == '+' {
                self.cursor += c.len_utf8();
                return Some(Ok(Token::Plus(&self.source[self.cursor - 1..self.cursor])));
            } else if c == '=' {
                self.cursor += c.len_utf8();
                if self.consume_if('=') {
                    return Some(Ok(Token::Equality(
                        &self.source[self.cursor - 2..self.cursor],
                    )));
                }
                return Some(Ok(Token::Equal(&self.source[self.cursor - 1..self.cursor])));

                // words start with a letter or underscore and run until any other char
            } else if c.is_ascii_alphanumeric() || c == '_' {
                let start_pos = self.cursor;
                self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                let text = &self.source[start_pos..self.cursor];
                return Some(Ok(Self::word_token(text)));

                // any other char can't begin a lexeme: report it and move on
            } else {
                self.cursor += c.len_utf8();
                return Some(Err(ScanError::UnexpectedChar {
                    line: self.line,
                    ch: c,
                }));
            }
        }
        None
    }
}
