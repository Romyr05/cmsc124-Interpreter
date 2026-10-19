use std::fmt;

use crate::token_types::{Token, TokenType};

// Errors reported while scanning source text.
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

#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Token<'a>,
    pub message: String,
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token.token_type {
            TokenType::Eof => write!(f, "Error at end: {}", self.message),
            _ => write!(f, "Error at '{}': {}", self.token.lexeme, self.message),
        }
    }
}

impl std::error::Error for ParseError<'_> {}

// Runtime errors carry the source line and a diagnostic message.
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Runtime error: {}", self.line, self.message)
    }
}
