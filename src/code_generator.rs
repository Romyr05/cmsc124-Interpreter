use crate::ast::{Element, Value};
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::evaluator::stringify;
use crate::keyword_list::{attribute_name, event_html_name, is_event_attribute};
use crate::parser::TopLevel;
use crate::token_types::TokenType;

// Values for the document wrapper around a rendered element. `Default` keeps
// the original boilerplate, so top-level elements and old tests are unchanged.
#[derive(Debug, Clone)]
pub struct DocumentOptions {
    pub lang: String,
    pub title: Option<String>,
}

impl Default for DocumentOptions {
    fn default() -> Self {
        DocumentOptions {
            lang: "html".to_string(),
            title: None,
        }
    }
}

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
                    let value = env.evaluate(expr)?;
                    // Component values are generated markup, not user text:
                    // insert them raw so a bound button/div keeps its tags.
                    if value.is_markup() {
                        body.push_str(&stringify(&value));
                    } else {
                        let text = stringify(&value);
                        if escape_text {
                            body.push_str(&escape_html(&text));
                        } else {
                            body.push_str(&text);
                        }
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

// Script bodies are raw text: expression values go straight into <script>,
// with a newline between consecutive chunks so separate DSL strings do not
// silently merge into one JavaScript line.
fn append_script_children(
    element: &Element<'_>,
    body: &mut String,
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    env.in_new_scope(|env| {
        let mut wrote_expression = false;
        for child in &element.children {
            match child {
                TopLevel::Expr(expr) => {
                    let value = env.evaluate(expr)?;
                    // A script body is JavaScript source, not an HTML parent:
                    // inserting a component here would emit tags into JS.
                    if value.is_markup() {
                        return Err(RuntimeError::new(
                            element.line,
                            "Components are not allowed inside a script body.",
                        ));
                    }
                    if wrote_expression {
                        body.push('\n');
                    }
                    body.push_str(&stringify(&value));
                    wrote_expression = true;
                }
                // The parser rejects nested elements inside script bodies.
                // This arm only protects the generator if that invariant breaks.
                TopLevel::Element(_) => {
                    return Err(RuntimeError::new(
                        0,
                        "Elements are not allowed inside a script body.",
                    ));
                }
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
    // <script> content is JavaScript, not HTML: no escaping and no styling.
    if element.kind == TokenType::Script {
        body.push_str("<script>");
        append_script_children(element, body, env)?;
        body.push_str("</script>");
        return Ok(());
    }

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
    let mut events: Vec<(&'static str, String)> = Vec::new();
    // Headers read the enclosing scope. A declaration in this element's body
    // can affect a child's header, but cannot change this header retroactively.
    for attribute in &element.attributes {
        match attribute.kind {
            TokenType::StyleColor => {
                let value = stringify(&env.evaluate(&attribute.value)?);
                style.push_str("color:");
                style.push_str(&value);
                style.push(';');
            }
            kind if is_event_attribute(kind) => {
                let javascript = match env.evaluate(&attribute.value)? {
                    // A plain string is already JavaScript source.
                    Value::Str(javascript) => javascript,
                    // A script component contributes only its raw body; its
                    // <script> wrapper would be wrong inside an attribute.
                    Value::Script(javascript) => javascript,
                    _ => {
                        return Err(RuntimeError::new(
                            attribute.line,
                            format!(
                                "{} handler must be a string or a script component.",
                                attribute_name(kind)
                            ),
                        ));
                    }
                };
                events.push((event_html_name(kind), javascript));
            }
            _ => {}
        }
    }
    if !style.is_empty() {
        body.push_str(" style=\"");
        body.push_str(&escape_html(&style));
        body.push('"');
    }
    for (name, javascript) in events {
        body.push(' ');
        body.push_str(name);
        body.push_str("=\"");
        body.push_str(&escape_attribute(&javascript));
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

// Escape a value for a double-quoted HTML attribute. Single quotes stay as-is:
// they are safe inside double quotes and keeping them makes JavaScript handlers
// readable. The browser decodes this once before compiling the handler.
fn escape_attribute(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// Render only the element's own HTML, without the document wrapper. Component
// values use this so they can be inserted into another element or render().
pub fn generate_fragment(
    element: &Element<'_>,
    env: &mut Environment,
) -> Result<String, RuntimeError> {
    let mut body = String::new();
    append_element(element, &mut body, env)?;
    Ok(body)
}

// Render only a script element's body, without the <script> tags. Script
// components keep this raw JavaScript so it can later become an event handler.
pub fn generate_script_body(
    element: &Element<'_>,
    env: &mut Environment,
) -> Result<String, RuntimeError> {
    let mut body = String::new();
    append_script_children(element, &mut body, env)?;
    Ok(body)
}

// Wrap an already-generated body in the document boilerplate.
pub fn wrap_document(body: &str, options: &DocumentOptions) -> String {
    // Defaults produce exactly <head></head>; only an explicit title adds a tag.
    let head = match &options.title {
        Some(title) => format!("<title>{}</title>", escape_html(title)),
        None => String::new(),
    };

    format!(
        "<!DOCTYPE html><html lang=\"{}\"><head>{}</head><body>{}</body></html>",
        escape_html(&options.lang),
        head,
        body
    )
}

// Generate the HTML document for an element AST with the default boilerplate.
// Top-level elements use this path; render() evaluates its root component and
// reuses wrap_document instead.
pub fn generate(element: &Element<'_>, env: &mut Environment) -> Result<String, RuntimeError> {
    generate_document(element, env, &DocumentOptions::default())
}

// Generate a full document with caller-selected lang/title values.
// env is mutable: element bodies are scopes and may declare their own bindings.
pub fn generate_document(
    element: &Element<'_>,
    env: &mut Environment,
    options: &DocumentOptions,
) -> Result<String, RuntimeError> {
    let body = generate_fragment(element, env)?;
    Ok(wrap_document(&body, options))
}
