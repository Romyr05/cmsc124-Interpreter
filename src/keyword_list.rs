use crate::token_types::TokenType;

// maps a word to its keyword TokenType, or Identifier if it isn't a keyword
pub fn keywords_lookup(word: &str) -> TokenType {
    match word {
        "let" => TokenType::Initialize,
        "button" => TokenType::Button,
        "box" => TokenType::Box,
        "text" => TokenType::Text,
        "image" => TokenType::Image,
        "div" => TokenType::Div,
        "par" => TokenType::Paragraph,
        // attributes
        "id" => TokenType::IdName,
        "class" => TokenType::ClassName,
        //styling
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
