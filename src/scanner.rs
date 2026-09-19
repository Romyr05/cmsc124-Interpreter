// enum token with different types, kulang pa ni
use crate::token_app::{Token, TokenType};
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

    // seperate single character tokens and keyword tokens
    fn keyword_type(word: &str) -> TokenType {
        match word {
            "let" => TokenType::Initialize,
            "button" => TokenType::Button,
            "box" => TokenType::Box,
            "text" => TokenType::Text,
            "image" => TokenType::Image,
            "div" => TokenType::Div,
            "par" => TokenType::Paragraph,
            "id" => TokenType::IdName,
            "class" => TokenType::ClassName,
            "align" => TokenType::StyleAlign,
            "padding" => TokenType::StylePad,
            "margin" => TokenType::StyleMargin,
            "height" => TokenType::StyleHeight,
            "width" => TokenType::StyleWidth,
            "color" => TokenType::StyleColor,
            "border" => TokenType::StyleBorder,
            _ => TokenType::Identifier,
        }
    }

    fn single_char_type(c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Star),
            '/' => Some(TokenType::Slash),
            '=' => Some(TokenType::Equal),
            _ => None,
        }
    }
}

// we impl iterator so that can just run .collect() on the tokenizer to get the tokens
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.remaining().chars();
        let c = chars.next()?; // None at end of input, replaces your is_empty check
        let line = self.line; // the line this token STARTS on
        let start = self.cursor;

        if c.is_ascii_digit() {
            let mut len = c.len_utf8();
            for next_c in chars {
                if next_c.is_ascii_digit() {
                    len += next_c.len_utf8();
                } else {
                    break;
                }
            }
            self.cursor += len;
            let text = &self.source[start..self.cursor];
            Some(Ok(Token::new(TokenType::Number, text, line)))
        } else if c.is_whitespace() {
            let mut len = c.len_utf8();
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
            let text = &self.source[start..self.cursor];
            Some(Ok(Token::new(TokenType::WhiteSpace, text, line)))
        } else if c == '"' {
            let content_start = start + c.len_utf8();
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
            self.cursor += len;
            if closed {
                let content_end = self.cursor - '"'.len_utf8();
                let text = &self.source[content_start..content_end];
                Some(Ok(Token::new(TokenType::String, text, line)))
            } else {
                Some(Err(ScanError::UnterminatedString { line }))
            }
        } else if let Some(tt) = Self::single_char_type(c) {
            self.cursor += c.len_utf8();
            let text = &self.source[start..self.cursor];
            Some(Ok(Token::new(tt, text, line)))
        } else if c.is_ascii_alphabetic() || c == '_' {
            let mut len = c.len_utf8();
            for next_c in chars {
                if next_c.is_ascii_alphabetic() || next_c == '_' {
                    len += next_c.len_utf8();
                } else {
                    break;
                }
            }
            self.cursor += len;
            let text = &self.source[start..self.cursor];
            Some(Ok(Token::new(Self::keyword_type(text), text, line)))
        } else {
            self.cursor += c.len_utf8();
            Some(Err(ScanError::UnexpectedChar { line, ch: c }))
        }
    }
}
