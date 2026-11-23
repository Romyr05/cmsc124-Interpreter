// Tree-walking evaluator: walks an Expr and produces a runtime Value.
// Post-order traversal: each node evaluates its children before itself.
use crate::ast::{Expr, Value};
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::token_types::{Token, TokenType};

// Expression evaluation is an extension of the environment: it reads bindings
// through Environment::lookup but does not own scope or statement execution.
impl Environment {
    // the one entry point: ask what node this is and do the matching thing
    pub fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            // literal: value was built during parsing, just hand it back
            Expr::Literal { value } => Ok(value.clone()),
            // grouping: parentheses carry no runtime meaning, evaluate the inside
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(operator, left, right),
            // look the name up, innermost scope first; missing everywhere -> undefined
            Expr::Variable { name, line } => match self.lookup(name) {
                Some(Some(value)) => Ok(value.clone()),
                // declared with `let x` but never given a value
                Some(None) => Err(RuntimeError::new(
                    *line,
                    format!("Variable '{}' used before assignment.", name),
                )),
                None => Err(RuntimeError::new(
                    *line,
                    format!("Undefined variable '{}'.", name),
                )),
            },
        }
    }

    fn eval_unary(&self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let right = self.evaluate(right)?; // operand first
        match operator.token_type {
            // arithmetic negation: operand must be a number
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError::new(
                    operator.line,
                    "Operand must be a number.",
                )),
            },
            // logical not -> any value has a truthiness, so this never errors
            TokenType::Not => Ok(Value::Bool(!is_truthy(&right))),
            _ => Err(RuntimeError::new(operator.line, "Unknown unary operator.")),
        }
    }

    // this is short circuit same as module
    // and/or: evaluate left first; only evaluate right if left doesn't already decide the answer
    fn eval_logical(
        &self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left = is_truthy(&self.evaluate(left)?);
        match operator.token_type {
            // short-circuit: right never runs
            TokenType::LogicAnd if !left => Ok(Value::Bool(false)),
            TokenType::LogicOr if left => Ok(Value::Bool(true)),
            // left didn't decide it, so right's truthiness is the answer
            _ => Ok(Value::Bool(is_truthy(&self.evaluate(right)?))),
        }
    }

    fn eval_binary(
        &self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        // checked before the operands are evaluated so and/or can skip the right side
        if matches!(
            operator.token_type,
            TokenType::LogicAnd | TokenType::LogicOr
        ) {
            return self.eval_logical(operator, left, right);
        }

        let left = self.evaluate(left)?; // left, then right
        let right = self.evaluate(right)?;

        // each operator checks the types it needs
        match operator.token_type {
            // + adds two numbers or joins two strings, mixing two types -> error
            TokenType::Plus => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),

                // Foir the concatenation of both string
                (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                _ => Err(RuntimeError::new(
                    operator.line,
                    "Operands must be two numbers or two strings.",
                )),
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

            // Comparison operators
            TokenType::Less => {
                let (a, b) = number_operands(operator, &left, &right)?;
                Ok(Value::Bool(a < b))
            }

            TokenType::LessEqual => {
                let (a, b) = number_operands(operator, &left, &right)?;
                Ok(Value::Bool(a <= b))
            }
            TokenType::Greater => {
                let (a, b) = number_operands(operator, &left, &right)?;
                Ok(Value::Bool(a > b))
            }
            TokenType::GreaterEqual => {
                let (a, b) = number_operands(operator, &left, &right)?;
                Ok(Value::Bool(a >= b))
            }

            //Equality
            // any pair of types is allowed but different types are never equal
            TokenType::Equality => Ok(Value::Bool(left == right)),
            TokenType::NotEqual => Ok(Value::Bool(left != right)),

            _ => Err(RuntimeError::new(
                operator.line,
                "Operator not supported yet.",
            )),
        }
    }
}

// For bool values
// only false and null are falsey, everything else (0, "") is truthy
pub(crate) fn is_truthy(value: &Value) -> bool {
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
        _ => Err(RuntimeError::new(
            operator.line,
            "Operands must be numbers.",
        )),
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
