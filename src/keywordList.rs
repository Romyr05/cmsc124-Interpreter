// (Token enum
use crate::token_app::Token;

pub fn keywords_Lookup(word: &str) -> Option<Token<'_>> {
    match word {
        "let" => Some(Token::Initialize(word)),
        "button" => Some(Token::Button(word)),
        "box" => Some(Token::Box(word)),
        "text" => Some(Token::Text(word)),
        "image" => Some(Token::Image(word)),
        "div" => Some(Token::Div(word)),
        "par" => Some(Token::Paragraph(word)),
        // attributes
        "id" => Some(Token::IdName(word)),
        "class" => Some(Token::ClassName(word)),
        //styling
        "align" => Some(Token::StyleAlign(word)),
        "padding" => Some(Token::StylePad(word)),
        "margin" => Some(Token::StyleMargin(word)),
        "height" => Some(Token::StyleHeight(word)),
        "width" => Some(Token::StyleWidth(word)),
        "color" => Some(Token::StyleColor(word)),
        "border" => Some(Token::StyleBorder(word)),
        _ => Some(Token::Identifier(word)),
    }
}
