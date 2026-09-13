// hi rom, gincopy ko ang solution ko sa hard exercism problem
// where i also made a tokenizer, ill try to explain it w comments
// as best as i can

// enum token with different types, kulang pa ni
#[derive(Debug)]
pub enum Token<'a> {
    LeftParen(&'a str),
    RightParen(&'a str),
    Plus(&'a str),
    Equal(&'a str),
    Number(&'a str),
    Word(&'a str),
    WhiteSpace(&'a str),
    Identifier(&'a str),
    // keyword tokens below
    Button(&'a str),
    Box(&'a str),
    Text(&'a str),
    Image(&'a str),
    Div(&'a str),
    Paragraph(&'a str),
    // attributes
    ID_name(&'a str),
    Class_name(&'a str),
    //styling
    Style_Align(&'a str),
    Style_Pad(&'a str),
    Style_Margin(&'a str),
    Style_Height(&'a str),
    Style_Width(&'a str),
    Style_Color(&'a str),
    Style_Border(&'a str),
}

pub struct Tokenizer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    // helper function to get remaining chars from string
    fn remaining(&self) -> &'a str {
        &self.source[self.cursor..]
    }

    // words encapsulate either identifiers or actual keywords
    // handle_words matches the token with existing keywords in a list
    fn handle_words(word: &'a str) -> Token<'a> {
        match word {
            "button" => {
                return Token::Button(word);
            }
            "box" => {
                return Token::Box(word);
            }
            "text" => {
                return Token::Text(word);
            }
            "image" => {
                return Token::Image(word);
            }
            "div" => {
                return Token::Div(word);
            }
            "par" => {
                return Token::Paragraph(word);
            }
            // attributes
            "id" => {
                return Token::ID_name(word);
            }
            "class" => {
                return Token::Class_name(word);
            }
            //styling
            "align" => {
                return Token::Style_Align(word);
            }
            "padding" => {
                return Token::Style_Pad(word);
            }
            "margin" => {
                return Token::Style_Margin(word);
            }
            "height" => {
                return Token::Style_Height(word);
            }
            "width" => {
                return Token::Style_Width(word);
            }
            "color" => {
                return Token::Style_Color(word);
            }
            "border" => {
                return Token::Style_Border(word);
            }
            _ => {
                return Token::Identifier(word);
            }
        }
    }
}

// we impl iterator so that can just run .collect() on the tokenizer to get the tokens
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

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
                return Some(Token::Number(text));

                // same for whitespace
            } else if c.is_whitespace() {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_whitespace() {
                        len += next_c.len_utf8();
                    } else {
                        break;
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Token::WhiteSpace(text));
            } else if
                // here for all the other token types

                c == '('
            {
                self.cursor += c.len_utf8();
                return Some(Token::LeftParen(&self.source[self.cursor - 1..self.cursor]));
            } else if c == ')' {
                self.cursor += c.len_utf8();
                return Some(Token::RightParen(&self.source[self.cursor - 1..self.cursor]));
            } else if c == '+' {
                self.cursor += c.len_utf8();
                return Some(Token::Plus(&self.source[self.cursor - 1..self.cursor]));
            } else if c == '=' {
                self.cursor += c.len_utf8();
                return Some(Token::Equal(&self.source[self.cursor - 1..self.cursor]));

                // for words, we break if we see whitespace or a digit
            } else {
                let mut len = c.len_utf8();
                let start_pos = self.cursor;
                for next_c in chars {
                    if next_c.is_whitespace() {
                        break;
                    } else if next_c.is_ascii_digit() {
                        break;
                    } else {
                        len += next_c.len_utf8();
                    }
                }
                self.cursor += len;
                let text = &self.source[start_pos..self.cursor];
                return Some(Self::handle_words(text));
            }
        } // errors not implemented yet
        None
    }
}
