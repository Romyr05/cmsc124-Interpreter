use crate::ast::Element;
use crate::errors::RuntimeError;
use crate::evaluator::{evaluate, stringify};
use crate::parser::TopLevel;
use crate::token_types::TokenType;

fn append_children(
    element: &Element<'_>,
    body: &mut String,
    escape_text: bool,
) -> Result<(), RuntimeError> {
    for child in &element.children {
        match child {
            TopLevel::Expr(expr) => {
                let value = stringify(&evaluate(expr)?);
                if escape_text {
                    body.push_str(&escape_html(&value));
                } else {
                    body.push_str(&value);
                }
            }
            TopLevel::Element(element) => append_element(element, body)?,
        }
    }
    Ok(())
}

fn append_element(element: &Element<'_>, body: &mut String) -> Result<(), RuntimeError> {
    match element.kind {
        TokenType::Text => {
            body.push_str("<h1>");
            append_children(element, body, true)?;
            body.push_str("</h1>");
        }
        TokenType::Paragraph => {
            body.push_str("<p>");
            append_children(element, body, true)?;
            body.push_str("</p>");
        }
        TokenType::Div => {
            body.push_str("<div>");
            append_children(element, body, true)?;
            body.push_str("</div>");
        }
        _ => append_children(element, body, false)?,
    }
    Ok(())
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// Generate the HTML document for an element AST.
pub fn generate(element: &Element<'_>) -> Result<String, RuntimeError> {
    let html_start = "<!DOCTYPE html><html lang=\"html\">";
    let html_end = "</html>";
    let head_start = "<head>";
    let head_end = "</head>";
    let body_start = "<body>";
    let body_end = "</body>";
    let mut body = String::new();

    append_element(element, &mut body)?;

    Ok(format!(
        "{}{}{}{}{}{}{}",
        html_start, head_start, head_end, body_start, body, body_end, html_end
    ))
}
