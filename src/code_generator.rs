use crate::ast::Element;
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::evaluator::stringify;
use crate::parser::TopLevel;
use crate::token_types::TokenType;

fn append_children(
    element: &Element<'_>,
    body: &mut String,
    escape_text: bool,
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    // the body is its own scope: created before the children run, and the outer
    // scope is restored afterward, even if a child fails
    // closure
    env.in_new_scope(|env| {
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
                // every other child is a statement (print, let, assignment), run by the
                // shared executor in this body's scope. print goes straight to the
                // terminal, never into body (the HTML), so its line appears before the
                // element's HTML is printed
                // For the terminal side; makes it a
                // render() is top-level only, so a body statement never stops execution
                statement => {
                    env.execute(statement, false)?;
                }
            }
        }
        Ok(())
    })
}

fn append_element(
    element: &Element<'_>,
    body: &mut String,
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    let tag = match element.kind {
        TokenType::Text => "h1",
        TokenType::Paragraph => "p",
        TokenType::Div => "div",
        TokenType::Button => "button",
        _ => return append_children(element, body, false, env),
    };

    body.push('<');
    body.push_str(tag);
    let mut style = String::new();
    // Headers read the enclosing scope. A declaration in this element's body
    // can affect a child's header, but cannot change this header retroactively.
    for attribute in &element.attributes {
        if attribute.kind == TokenType::StyleColor {
            let value = stringify(&env.evaluate(&attribute.value)?);
            style.push_str("color:");
            style.push_str(&value);
            style.push(';');
        }
    }
    if !style.is_empty() {
        body.push_str(" style=\"");
        body.push_str(&escape_html(&style));
        body.push('"');
    }
    body.push('>');
    append_children(element, body, true, env)?;
    body.push_str("</");
    body.push_str(tag);
    body.push('>');
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
// env is mutable: element bodies are scopes and may declare their own bindings
pub fn generate(element: &Element<'_>, env: &mut Environment) -> Result<String, RuntimeError> {
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
