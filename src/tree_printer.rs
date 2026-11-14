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
        parts.push(print_statement(child));
    }

    format!("({})", parts.join(" "))
}

// one statement in --parse form; shared by top-level lines (main.rs) and bodies
pub fn print_statement(item: &TopLevel<'_>) -> String {
    match item {
        TopLevel::Element(e) => print_element(e),
        TopLevel::Expr(e) => print_expr(e),
        TopLevel::Let {
            name, initializer, ..
        } => print_let(name, initializer.as_ref()),
        TopLevel::Print(e) => format!("(print {})", print_expr(e)),
        // same (= x value) form as the Lab 2 tree output
        TopLevel::Assign { name, value, .. } => format!("(= {} {})", name, print_expr(value)),
        // (if cond (then s1 s2) (else s3)); the else part only when present
        TopLevel::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut parts = vec![
                "if".to_string(),
                print_expr(condition),
                print_block("then", then_branch),
            ];
            if let Some(branch) = else_branch {
                parts.push(print_block("else", branch));
            }
            format!("({})", parts.join(" "))
        }
        // (while cond (body s1 s2))
        TopLevel::While { condition, body } => {
            format!(
                "(while {} {})",
                print_expr(condition),
                print_block("body", body)
            )
        }
    }
}

// (label s1 s2 ...) for a body of statements
fn print_block(label: &str, stmts: &[TopLevel<'_>]) -> String {
    let mut parts = vec![label.to_string()];
    for stmt in stmts {
        parts.push(print_statement(stmt));
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
