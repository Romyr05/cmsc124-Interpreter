use crate::ast::{Attribute, Element, Expr, Value};
use crate::keyword_list::{attribute_name, element_name};
use crate::parser::TopLevel;

pub fn print_expr(expr: &Expr<'_>) -> String {
    match expr {
        Expr::Literal { value } => match value {
            Value::Number(n) => format!("{:?}", n), // "2.0", not "2"
            Value::Str(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Nil => "null".to_string(),
        },
        Expr::Unary { operator, right } => parenthesize(operator.lexeme, &[right.as_ref()]),
        Expr::Binary {
            left,
            operator,
            right,
        } => parenthesize(operator.lexeme, &[left.as_ref(), right.as_ref()]),
        Expr::Grouping { expression } => parenthesize("group", &[expression.as_ref()]),
        Expr::Variable { name, .. } => name.clone(),
    }
}

pub fn print_attribute(attr: &Attribute) -> String {
    format!(
        "({} {})",
        attribute_name(attr.kind),
        print_expr(&attr.value)
    )
}

// let x = 1 + 2  ->  (let x (+ 1.0 2.0)),  let x  ->  (let x)
pub fn print_let(name: &str, initializer: Option<&Expr<'_>>) -> String {
    match initializer {
        Some(expr) => format!("(let {} {})", name, print_expr(expr)),
        None => format!("(let {})", name),
    }
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
            TopLevel::Let {
                name, initializer, ..
            } => print_let(name, initializer.as_ref()),
            TopLevel::Print(e) => format!("(print {})", print_expr(e)),
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
