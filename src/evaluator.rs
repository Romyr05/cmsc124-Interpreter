// Tree-walking evaluator: walks an Expr and produces a runtime Value.
// Post-order traversal: each node evaluates its children before itself.
use crate::ast::{Expr, Value};
use crate::errors::RuntimeError;
use crate::token_types::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// one scope: its own bindings (name -> value) plus a link to the scope around it.
// None = declared with `let x` but not given a value yet.
// parent is None for the global scope.
// Rc<RefCell<..>> = shared ownership + mutation checked at runtime (closures need this, Lab 5)
#[derive(Default)]
struct Scope {
    values: HashMap<String, Option<Value>>,
    parent: Option<Rc<RefCell<Scope>>>,
}

// the interpreter's state: the scope code is currently running in.
// created once per file run / REPL session, so global bindings survive between lines
#[derive(Default)]
pub struct Environment {
    current: Rc<RefCell<Scope>>,
}

impl Environment {
    // run `work` in a new scope whose parent is the current one, then put the outer
    // scope back. The restore always runs, even when work fails (no ? in between),
    // so after an error the REPL is back at the scope it started in.
    pub fn in_new_scope(
        &mut self,
        work: impl FnOnce(&mut Environment) -> Result<(), RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let outer = Rc::clone(&self.current);
        self.current = Rc::new(RefCell::new(Scope {
            values: HashMap::new(),
            parent: Some(Rc::clone(&outer)),
        }));
        let result = work(self);
        self.current = outer;
        result
    }

    // search from the current scope outward; the innermost scope that has the name wins
    // outer None = no scope has it; inner None = declared but no value yet
    fn lookup(&self, name: &str) -> Option<Option<Value>> {
        let mut scope = Rc::clone(&self.current);
        loop {
            let parent = {
                let s = scope.borrow();
                if let Some(value) = s.values.get(name) {
                    return Some(value.clone());
                }
                s.parent.clone()
            };
            // no parent left = searched every scope without finding it
            scope = parent?;
        }
    }

    // let name = initializer   (or just `let name`, which stores no value)
    // evaluate first: if that fails we return early and the table is untouched
    // then check for redeclaration, then store
    pub fn declare(
        &mut self,
        name: &str,
        initializer: Option<&Expr>,
        line: usize,
    ) -> Result<(), RuntimeError> {
        let value = match initializer {
            Some(expr) => Some(self.evaluate(expr)?),
            None => None,
        };
        // only the current scope counts: an inner `let x` may shadow an outer x
        let mut scope = self.current.borrow_mut();
        if scope.values.contains_key(name) {
            return Err(RuntimeError {
                line,
                message: format!("Variable '{}' already declared.", name),
            });
        }
        scope.values.insert(name.to_string(), value);
        Ok(())
    }

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
                Some(Some(value)) => Ok(value),
                // declared with `let x` but never given a value
                Some(None) => Err(RuntimeError {
                    line: *line,
                    message: format!("Variable '{}' used before assignment.", name),
                }),
                None => Err(RuntimeError {
                    line: *line,
                    message: format!("Undefined variable '{}'.", name),
                }),
            },
        }
    }

    fn eval_unary(&self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let right = self.evaluate(right)?; // operand first
        match operator.token_type {
            // arithmetic negation: operand must be a number
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(RuntimeError {
                    line: operator.line,
                    message: "Operand must be a number.".to_string(),
                }),
            },
            // logical not -> any value has a truthiness, so this never errors
            TokenType::Not => Ok(Value::Bool(!is_truthy(&right))),
            _ => Err(RuntimeError {
                line: operator.line,
                message: "Unknown unary operator.".to_string(),
            }),
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

            _ => Err(RuntimeError {
                line: operator.line,
                message: "Operator not supported yet.".to_string(),
            }),
        }
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
