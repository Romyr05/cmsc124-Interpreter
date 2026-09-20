use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Equality,
    Dot,
    Number,
    Float,
    String,
    Word,
    Identifier,
    Initialize,
    Button,
    Box,
    Text,
    Image,
    Div,
    Paragraph,
    IdName,
    ClassName,
    StyleAlign,
    StylePad,
    StyleMargin,
    StyleHeight,
    StyleWidth,
    StyleColor,
    StyleBorder,
    Eof,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({:?})", self.token_type, self.lexeme)
    }
}
