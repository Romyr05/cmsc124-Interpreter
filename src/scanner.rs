// enum token with different types, kulang pa ni
use crate::{keyword_list::keywords_lookup, token_app::Token};
use std::fmt;

// the two malformed-input cases the scanner can report
pub enum ScanError {
    UnterminatedString { line: usize },
    UnexpectedChar { line: usize, ch: char },
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

    // consume if matched
    fn consume_if(&mut self, expected_char: char) -> bool {
        // use this to "peek" at the next
        match self.peek() {
            Some(c) if c == expected_char => {
                self.cursor += c.len_utf8();
                true
            }
            _ => false,
        }
    }

    // Token, if not identifier
    fn word_token(text: &'a str) -> Token<'a> {
        keywords_lookup(text).unwrap_or(Token::Identifier(text))
    }
}

// we impl iterator so that can just run .collect() on the tokenizer to get the tokens
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
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
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_ascii_digit() {
                        len += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Ok(Token::Number(text)));

                // same for whitespace; count newlines so line numbers stay accurate
            } else if c.is_whitespace() {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                if c == '\n' {
                    self.line += 1;
                }
                for next_c in chars {
                    if next_c.is_whitespace() {
                        len += next_c.len_utf8();
                        if next_c == '\n' {
                            self.line += 1;
                        }
                    } else {
                        break;
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Ok(Token::WhiteSpace(text)));

                // string literal: "..." may span lines; error only if EOF hits first
            } else if c == '"' {
                let start_line = self.line;
                let content_start = self.cursor + c.len_utf8();
                let mut len = c.len_utf8();
                let mut closed = false;
                for next_c in chars {
                    len += next_c.len_utf8();
                    if next_c == '"' {
                        closed = true;
                        break;
                    }
                    if next_c == '\n' {
                        self.line += 1;
                    }
                }
                if closed {
                    let content_end = self.cursor + len - '"'.len_utf8();
                    self.cursor += len;
                    let text = &self.source[content_start..content_end];
                    return Some(Ok(Token::String(text)));
                } else {
                    // consume the rest so scanning ends cleanly
                    self.cursor += len;
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
                return Some(Ok(Token::Equal(&self.source[self.cursor - 1..self.cursor])));

                // words start with a letter or underscore and run until any other char
            } else if c.is_ascii_alphabetic() || c == '_' {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_ascii_alphabetic() || next_c == '_' {
                        len += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.cursor += len;
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
