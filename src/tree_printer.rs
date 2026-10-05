use crate::ast::{Attribute, Element, Expr, Value};
use crate::keyword_list::{attribute_name, element_name};
use crate::parser::TopLevel;

pub fn print_expr(expr: &Expr<'_>) -> String {
    match expr {
        Expr::Literal { value } => match value {
            Value::Number(n) => format!("{:?}", n), // "2.0", not "2"
            Value::Str(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
        },
        Expr::Unary { operator, right } => parenthesize(operator.lexeme, &[right.as_ref()]),
        Expr::Binary {
            left,
            operator,
            right,
        } => parenthesize(operator.lexeme, &[left.as_ref(), right.as_ref()]),
        Expr::Grouping { expression } => parenthesize("group", &[expression.as_ref()]),
    }
}

pub fn print_attribute(attr: &Attribute) -> String {
    format!(
        "({} {})",
        attribute_name(attr.kind),
        print_expr(&attr.value)
    )
}

pub fn print_element(element: &Element) -> String {
    let mut parts = vec![element_name(element.kind).to_string()]; // Easy initialization of rust

    for attr in &element.attributes {
        parts.push(print_attribute(attr));
    }

    for child in &element.children {
        parts.push(match child {
            TopLevel::Element(e) => print_element(e),
            TopLevel::Expr(e) => print_expr(e),
        });
    }

    format!("({})", parts.join(" "))
}

fn parenthesize(name: &str, children: &[&Expr<'_>]) -> String {
    let mut out = String::new();
    out.push('(');
    out.push_str(name);
    for child in children {
        out.push(' ');
        out.push_str(&print_expr(child));
    }
    out.push(')');
    out
}
