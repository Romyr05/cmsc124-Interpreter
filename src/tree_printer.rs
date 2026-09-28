use crate::ast::{Expr, Value};

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
