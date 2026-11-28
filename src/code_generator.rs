use crate::ast::{Attribute, Element, Markup, Value, script_tag, style_tag};
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::evaluator::stringify;
use crate::keyword_list::{attribute_name, element_name, event_html_name, is_event_attribute};
use crate::parser::TopLevel;
use crate::token_types::TokenType;
use std::path::Path;

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
    out: &mut Markup,
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
                    // insert their parts raw so a bound button/div keeps its
                    // tags and its styles/scripts still reach head and end.
                    if let Some(markup) = value.as_markup() {
                        out.append(&markup);
                    } else {
                        let text = stringify(&value);
                        if escape_text {
                            out.body.push_str(&escape_html(&text));
                        } else {
                            out.body.push_str(&text);
                        }
                    }
                }
                TopLevel::Element(element) => append_element(element, out, env)?,
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

// Script and style bodies are raw text: expression values go straight into
// <script>/<style>, with a newline between consecutive chunks so separate DSL
// strings do not silently merge into one line.
fn append_raw_children(
    element: &Element<'_>,
    body: &mut String,
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    let name = element_name(element.kind);
    env.in_new_scope(|env| {
        let mut wrote_expression = false;
        for child in &element.children {
            match child {
                TopLevel::Expr(expr) => {
                    let value = env.evaluate(expr)?;
                    // A script/style body is source code, not an HTML parent:
                    // inserting a component here would emit tags into JS/CSS.
                    if value.is_markup() {
                        return Err(RuntimeError::new(
                            element.line,
                            format!("Components are not allowed inside a {} body.", name),
                        ));
                    }
                    if wrote_expression {
                        body.push('\n');
                    }
                    body.push_str(&stringify(&value));
                    wrote_expression = true;
                }
                // The parser rejects nested elements inside script/style bodies.
                // This arm only protects the generator if that invariant breaks.
                TopLevel::Element(_) => {
                    return Err(RuntimeError::new(
                        0,
                        format!("Elements are not allowed inside a {} body.", name),
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

// The JavaScript or CSS text of a script/style element: the linked file when
// it has src, otherwise its body.
fn raw_source(element: &Element<'_>, env: &mut Environment) -> Result<String, RuntimeError> {
    match element
        .attributes
        .iter()
        .find(|attribute| attribute.kind == TokenType::Src)
    {
        Some(src) => read_linked(element, src, env),
        None => {
            let mut text = String::new();
            append_raw_children(element, &mut text, env)?;
            Ok(text)
        }
    }
}

// Read a src file so its text can be inlined. The path is relative to the
// running .fmyr file (the folder render() writes into); the REPL has no source
// file, so there it is relative to the current folder.
fn read_linked(
    element: &Element<'_>,
    src: &Attribute<'_>,
    env: &mut Environment,
) -> Result<String, RuntimeError> {
    let Value::Str(path) = env.evaluate(&src.value)? else {
        return Err(RuntimeError::new(src.line, "src must be a string."));
    };
    let base = env
        .source_path()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_default();
    let text = std::fs::read_to_string(base.join(&path))
        .map_err(|_| RuntimeError::new(src.line, format!("Cannot read file '{}'.", path)))?;

    // Inlined text that contains the closing tag would end the block early and
    // spill the rest of the file into the page.
    let closing = format!("</{}", element_name(element.kind));
    if text.to_ascii_lowercase().contains(&closing) {
        return Err(RuntimeError::new(
            src.line,
            format!(
                "File '{}' contains '{}>', which cannot be inlined.",
                path, closing
            ),
        ));
    }
    Ok(text)
}

fn append_element(
    element: &Element<'_>,
    out: &mut Markup,
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    // Script and style content is source code, not HTML: no escaping and no
    // styling. JavaScript is collected at the end of <body> so it runs after
    // every element exists; CSS is collected into <head> so it applies before
    // the page is drawn.
    match element.kind {
        TokenType::Script => {
            let javascript = raw_source(element, env)?;
            out.end.push_str(&script_tag(&javascript));
            return Ok(());
        }
        TokenType::Style => {
            let css = raw_source(element, env)?;
            out.head.push_str(&style_tag(&css));
            return Ok(());
        }
        _ => {}
    }

    let tag = match element.kind {
        TokenType::Text => "h1",
        TokenType::Paragraph => "p",
        TokenType::Div => "div",
        TokenType::Button => "button",
        _ => return append_children(element, out, false, env),
    };

    let body = &mut out.body;
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
    append_children(element, out, true, env)?;
    out.body.push_str("</");
    out.body.push_str(tag);
    out.body.push('>');
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

// Render only the element's own markup, without the document wrapper.
// Component values use this so they can be inserted into another element or
// render().
pub fn generate_fragment(
    element: &Element<'_>,
    env: &mut Environment,
) -> Result<Markup, RuntimeError> {
    let mut out = Markup::default();
    append_element(element, &mut out, env)?;
    Ok(out)
}

// Render only a script element's JavaScript, without the <script> tags (its
// linked file when it has src). Script components keep this raw JavaScript so
// it can later become an event handler.
pub fn generate_script_body(
    element: &Element<'_>,
    env: &mut Environment,
) -> Result<String, RuntimeError> {
    raw_source(element, env)
}

// Wrap already-generated markup in the document boilerplate: collected CSS
// after the title in <head>, collected JavaScript after the body content.
pub fn wrap_document(markup: &Markup, options: &DocumentOptions) -> String {
    // Defaults produce exactly <head></head>; only an explicit title or
    // collected styles add content.
    let title = match &options.title {
        Some(title) => format!("<title>{}</title>", escape_html(title)),
        None => String::new(),
    };

    format!(
        "<!DOCTYPE html><html lang=\"{}\"><head>{}{}</head><body>{}{}</body></html>",
        escape_html(&options.lang),
        title,
        markup.head,
        markup.body,
        markup.end
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
    let markup = generate_fragment(element, env)?;
    Ok(wrap_document(&markup, options))
}
