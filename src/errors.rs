use std::fmt;
use std::io;

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

pub(crate) enum CliError {
    Io(io::Error),
    Usage(String),
    Compile(String),
    Runtime(String),
}

impl CliError {
    pub(crate) fn exit_code(&self) -> u8 {
        match self {
            CliError::Io(_) => 74,
            CliError::Usage(_) => 64,
            CliError::Compile(_) => 65,
            CliError::Runtime(_) => 70,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Io(error) => write!(f, "I/O error: {}", error),
            CliError::Usage(message) | CliError::Compile(message) | CliError::Runtime(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::Io(error)
    }
}
