// Tree-walking evaluator: walks an Expr and produces a runtime Value.
// Post-order traversal: each node evaluates its children before itself.
use std::fmt;

use crate::ast::{Expr, Value};
use crate::token_types::{Token, TokenType};

// a runtime error carries the line (from the operator token) and a message.
// kept minimal: the lab forbids the host language panicking on bad input.
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Runtime error: {}", self.line, self.message)
    }
}

// the one entry point: ask what node this is and do the matching thing
pub fn evaluate(expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
        // literal: value was built during parsing, just hand it back
        Expr::Literal { value } => Ok(value.clone()),
        // grouping: parentheses carry no runtime meaning, evaluate the inside
        Expr::Grouping { expression } => evaluate(expression),
        Expr::Unary { operator, right } => eval_unary(operator, right),
        Expr::Binary {
            left,
            operator,
            right,
        } => eval_binary(operator, left, right),
    }
}

fn eval_unary(operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
    let right = evaluate(right)?; // operand first
    match operator.token_type {
        // arithmetic negation: operand must be a number
        TokenType::Minus => match right {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(RuntimeError {
                line: operator.line,
                message: "Operand must be a number.".to_string(),
            }),
        },
        // logical not: any value has a truthiness, so this never errors
        TokenType::Not => Ok(Value::Bool(!is_truthy(&right))),
        _ => Err(RuntimeError {
            line: operator.line,
            message: "Unknown unary operator.".to_string(),
        }),
    }
}

fn eval_binary(operator: &Token, left: &Expr, right: &Expr) -> Result<Value, RuntimeError> {
    let left = evaluate(left)?; // left, then right
    let right = evaluate(right)?;

    // each operator checks the types it needs
    match operator.token_type {
        // + adds two numbers or joins two strings, mixing two types -> error
        TokenType::Plus => match (&left, &right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),

            // Foir the concatenation of both string
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
            _ => Err(RuntimeError {
                line: operator.line,
                message: "Operands must be two numbers or two strings.".to_string(),
            }),
        },
        
        TokenType::Minus => {
            let (a, b) = number_operands(operator, &left, &right)?;
            Ok(Value::Number(a - b))
        }
        TokenType::Star => {
            let (a, b) = number_operands(operator, &left, &right)?;
            Ok(Value::Number(a * b))
        }
        TokenType::Slash => {
            let (a, b) = number_operands(operator, &left, &right)?;
            Ok(Value::Number(a / b)) // a / 0 yields inf (IEEE 754)
        }
        TokenType::Caret => {
            let (a, b) = number_operands(operator, &left, &right)?;
            Ok(Value::Number(a.powf(b)))
        }

        // Operands
        TokenType::Less => {
            let(a,b) = number_operands(operator, &left, &right)?;
            Ok(Value::Bool(a<b))
        }

        TokenType::LessEqual => {
            let(a,b) = number_operands(operator, &left, &right)?;
            Ok(Value::Bool(a<=b))
        }
        TokenType::Greater => {
            let(a,b) = number_operands(operator, &left, &right)?;
            Ok(Value::Bool(a>b))
        }
        TokenType::GreaterEqual => {
            let(a,b) = number_operands(operator, &left, &right)?;
            Ok(Value::Bool(a>=b))
        }




        _ => Err(RuntimeError {
            line: operator.line,
            message: "Operator not supported yet.".to_string(),
        }),


    }
}

// For bool values
// only false and null are falsey, everything else (0, "") is truthy
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Nil => false,
        _ => true,
    }
}

// shared type check so the "must be numbers" message is written once
fn number_operands(
    operator: &Token,
    left: &Value,
    right: &Value,
) -> Result<(f64, f64), RuntimeError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok((*a, *b)),
        _ => Err(RuntimeError {
            line: operator.line,
            message: "Operands must be numbers.".to_string(),
        }),
    }
}

// how a value prints: numbers trimmed, strings unquoted, nil as "null"
pub fn stringify(value: &Value) -> String {
    match value {
        Value::Number(n) => format!("{}", n), // f64 Display prints 5, not 5.0
        Value::Str(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Nil => "null".to_string(),
    }
}
