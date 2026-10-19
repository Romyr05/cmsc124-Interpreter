use crate::ast::{Element, Expr};
use crate::errors::RuntimeError;
use crate::evaluator::{evaluate, stringify};
use crate::parser::TopLevel;

fn append_expressions(element: &Element<'_>, body: &mut String) -> Result<(), RuntimeError> {
    for child in &element.children {
        match child {
            TopLevel::Expr(expr) => append_expression(expr, body)?,
            TopLevel::Element(element) => append_expressions(element, body)?,
        }
    }
    Ok(())
}

fn append_expression(expr: &Expr<'_>, body: &mut String) -> Result<(), RuntimeError> {
    body.push_str(&stringify(&evaluate(expr)?));
    Ok(())
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

    append_expressions(element, &mut body)?;

    Ok(format!(
        "{}{}{}{}{}{}{}",
        html_start, head_start, head_end, body_start, body, body_end, html_end
    ))
}
