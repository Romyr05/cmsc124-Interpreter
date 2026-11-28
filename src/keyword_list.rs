use crate::token_types::TokenType;

// maps a word to its keyword TokenType, or Identifier if it isn't a keyword
pub fn keywords_lookup(word: &str) -> TokenType {
    match word {
        "let" => TokenType::Initialize,
        "print" => TokenType::Print,
        "render" => TokenType::Render,
        "script" => TokenType::Script,
        "style" => TokenType::Style,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "while" => TokenType::While,
        "button" => TokenType::Button,
        "box" => TokenType::Box,
        "text" => TokenType::Text,
        "image" => TokenType::Image,
        "div" => TokenType::Div,
        "par" => TokenType::Paragraph,
        // literals
        "true" | "false" => TokenType::Boolean,
        "null" => TokenType::Null,
        // logic operators
        "and" => TokenType::LogicAnd,
        "or" => TokenType::LogicOr,
        "not" => TokenType::Not,
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
        // events
        "on_click" => TokenType::OnClick,
        // linked files
        "src" => TokenType::Src,
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
            | TokenType::Script
            | TokenType::Style
    )
}

pub fn is_event_attribute(token_type: TokenType) -> bool {
    matches!(token_type, TokenType::OnClick)
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
            | TokenType::Src
    ) || is_event_attribute(token_type)
}

pub fn element_name(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::Button => "button",
        TokenType::Box => "box",
        TokenType::Text => "text",
        TokenType::Image => "image",
        TokenType::Div => "div",
        TokenType::Paragraph => "par",
        TokenType::Script => "script",
        TokenType::Style => "style",
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
        TokenType::OnClick => "on_click",
        TokenType::Src => "src",
        _ => "?attr",
    }
}

// Maps a DSL event attribute to the HTML event-handler attribute it emits.
pub fn event_html_name(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::OnClick => "onclick",
        _ => "?event",
    }
}
