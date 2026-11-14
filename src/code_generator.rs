use crate::ast::Element;
use crate::errors::RuntimeError;
use crate::evaluator::{Environment, stringify};
use crate::parser::TopLevel;
use crate::token_types::TokenType;

fn append_children(
    element: &Element<'_>,
    body: &mut String,
    escape_text: bool,
    env: &Environment,
) -> Result<(), RuntimeError> {
    for child in &element.children {
        match child {
            TopLevel::Expr(expr) => {
                let value = stringify(&env.evaluate(expr)?);
                if escape_text {
                    body.push_str(&escape_html(&value));
                } else {
                    body.push_str(&value);
                }
            }
            TopLevel::Element(element) => append_element(element, body, env)?,
            // print goes straight to the terminal, never into body (the HTML),
            // so its line appears before the element's HTML is printed
            TopLevel::Print(expr) => println!("{}", stringify(&env.evaluate(expr)?)),
            // the parser never puts a let inside an element body
            TopLevel::Let { line, .. } => {
                return Err(RuntimeError {
                    line: *line,
                    message: "'let' is not allowed inside an element.".to_string(),
                });
            }
        }
    }
    Ok(())
}

fn append_element(
    element: &Element<'_>,
    body: &mut String,
    env: &Environment,
) -> Result<(), RuntimeError> {
    match element.kind {
        TokenType::Text => {
            body.push_str("<h1>");
            append_children(element, body, true, env)?;
            body.push_str("</h1>");
        }
        TokenType::Paragraph => {
            body.push_str("<p>");
            append_children(element, body, true, env)?;
            body.push_str("</p>");
        }
        TokenType::Div => {
            body.push_str("<div>");
            append_children(element, body, true, env)?;
            body.push_str("</div>");
        }
        _ => append_children(element, body, false, env)?,
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
// env is read-only: element bodies can read bindings but not declare them
pub fn generate(element: &Element<'_>, env: &Environment) -> Result<String, RuntimeError> {
    let html_start = "<!DOCTYPE html><html lang=\"html\">";
    let html_end = "</html>";
    let head_start = "<head>";
    let head_end = "</head>";
    let body_start = "<body>";
    let body_end = "</body>";
    let mut body = String::new();

    append_element(element, &mut body, env)?;

    Ok(format!(
        "{}{}{}{}{}{}{}",
        html_start, head_start, head_end, body_start, body, body_end, html_end
    ))
}
