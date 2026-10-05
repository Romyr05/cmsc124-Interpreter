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

pub fn is_element_keyword(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Button
            | TokenType::Box
            | TokenType::Text
            | TokenType::Image
            | TokenType::Div
            | TokenType::Paragraph
    )
}

pub fn is_attribute_keyword(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::IdName
            | TokenType::ClassName
            | TokenType::StyleAlign
            | TokenType::StylePad
            | TokenType::StyleMargin
            | TokenType::StyleHeight
            | TokenType::StyleWidth
            | TokenType::StyleColor
            | TokenType::StyleBorder
    )
}

pub fn element_name(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::Button => "button",
        TokenType::Box => "box",
        TokenType::Text => "text",
        TokenType::Image => "image",
        TokenType::Div => "div",
        TokenType::Paragraph => "par",
        _ => "?element",
    }
}

pub fn attribute_name(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::IdName => "id",
        TokenType::ClassName => "class",
        TokenType::StyleAlign => "align",
        TokenType::StylePad => "padding",
        TokenType::StyleMargin => "margin",
        TokenType::StyleHeight => "height",
        TokenType::StyleWidth => "width",
        TokenType::StyleColor => "color",
        TokenType::StyleBorder => "border",
        _ => "?attr",
    }
}
