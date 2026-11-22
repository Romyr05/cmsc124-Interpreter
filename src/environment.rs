use crate::ast::{Expr, Value};
use crate::code_generator::generate;
use crate::errors::RuntimeError;
use crate::evaluator::{is_truthy, stringify};
use crate::parser::TopLevel;
use std::collections::HashMap;
use std::path::PathBuf;

// One scope: the bindings declared in this block. None means the name was
// declared with `let x` but has not been assigned yet.
#[derive(Default)]
struct Scope {
    values: HashMap<String, Option<Value>>,
}

// Whether execution should keep going or stop. render() will return Stop after
// writing its file; every other statement currently returns Continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    #[allow(dead_code)] // returned by render(), added in the next workstream
    Stop,
}

// The interpreter's state. Scopes form a stack: the global scope is index 0 and
// the innermost block is the last element. Lookups walk the stack from the back,
// assignments update the nearest matching scope, and leaving a block just pops
// its scope. This replaces the old Rc<RefCell<Scope>> parent chain, so there is
// no runtime borrow checking and no manual parent restoration.
pub struct Environment {
    scopes: Vec<Scope>,
    // Source file being run. render() will use it to choose the sibling .html
    // output path; the REPL has no source file, so this stays None there.
    #[allow(dead_code)] // read by render(), added in the next workstream
    source_path: Option<PathBuf>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            scopes: vec![Scope::default()],
            source_path: None,
        }
    }
}

impl Environment {
    // File mode keeps the source path so generated output can be written next to
    // the program. Errors still do not depend on it.
    pub fn for_source(path: impl Into<PathBuf>) -> Self {
        Environment {
            scopes: vec![Scope::default()],
            source_path: Some(path.into()),
        }
    }

    // Run `work` in a new scope, then pop it. Popping happens whether `work`
    // succeeds or fails, so an error leaves the environment at the outer scope.
    pub fn in_new_scope<T>(
        &mut self,
        work: impl FnOnce(&mut Environment) -> Result<T, RuntimeError>,
    ) -> Result<T, RuntimeError> {
        self.scopes.push(Scope::default());
        let result = work(self);
        self.scopes.pop();
        result
    }

    // Search from the innermost scope outward. The innermost match wins:
    // outer None = no scope has the name; inner None = declared but no value yet.
    pub(crate) fn lookup(&self, name: &str) -> Option<&Option<Value>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.values.get(name))
    }

    // run one statement. Shared by the runner (top level), element bodies, and
    // control-flow bodies, so each statement's rule is written once.
    // print_values: print a bare expression's value (--eval, REPL) or discard it (plain ./run)
    //
    // Flow::Stop is how render() ends interpretation once it has written its
    // file: bodies propagate it and the runner stops iterating.
    pub fn execute(&mut self, stmt: &TopLevel, print_values: bool) -> Result<Flow, RuntimeError> {
        match stmt {
            // always evaluated (so errors still surface); the value is printed only if asked
            TopLevel::Expr(expr) => {
                let value = self.evaluate(expr)?;

                // If false needs to have a print otherwise no need na
                if print_values {
                    println!("{}", stringify(&value));
                }
            }
            // a top-level element: its HTML is printed once the whole element is built
            TopLevel::Element(element) => println!("{}", generate(element, self)?),
            // print writes the value to the terminal
            // If it catches this it just prints tha value to the terminal and no replacing it as html
            // Makes it the appropriate value to be shown
            TopLevel::Print(expr) => println!("{}", stringify(&self.evaluate(expr)?)),
            // a declaration prints nothing; on error the table is left as it was
            TopLevel::Let {
                name,
                initializer,
                line,
            } => self.declare(name, initializer.as_ref(), *line)?,
            // an assignment prints nothing, like a declaration
            TopLevel::Assign { name, value, line } => self.assign(name, value, *line)?,
            // only the chosen branch runs; with no else, a falsey condition does nothing
            TopLevel::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition)?) {
                    return self.execute_block(then_branch, print_values);
                }
                if let Some(branch) = else_branch {
                    return self.execute_block(branch, print_values);
                }
            }
            // check, run the body, check again; a falsey first check runs it zero times.
            // execute_block makes a fresh scope on every iteration
            TopLevel::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)?) {
                    if let Flow::Stop = self.execute_block(body, print_values)? {
                        return Ok(Flow::Stop);
                    }
                }
            }
        }
        Ok(Flow::Continue)
    }

    // run a control-flow body in its own scope (restored even if a statement fails)
    fn execute_block(
        &mut self,
        stmts: &[TopLevel],
        print_values: bool,
    ) -> Result<Flow, RuntimeError> {
        self.in_new_scope(|env| {
            for stmt in stmts {
                if let Flow::Stop = env.execute(stmt, print_values)? {
                    return Ok(Flow::Stop);
                }
            }
            Ok(Flow::Continue)
        })
    }

    // checks whether the name already exists in the current scope only. It doesn’t check the outer scopes
    // Evaluates first and then if fails return early
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
        // only the innermost scope counts: an inner `let x` may shadow an outer x
        let scope = self
            .scopes
            .last_mut()
            .expect("environment always has a global scope");
        if scope.values.contains_key(name) {
            return Err(RuntimeError::new(
                line,
                format!("Variable '{}' already declared.", name),
            ));
        }
        scope.values.insert(name.to_string(), value);
        Ok(())
    }

    // computes the new value, then searches for an existing binding to update.
    // starts in the current scope and moves outward if necessary
    // name = value: evaluate first (an error leaves every scope untouched), then
    // update the nearest scope, innermost outward, that already has the name.
    // Never creates a binding; the new value may be a different type.
    pub fn assign(&mut self, name: &str, value: &Expr, line: usize) -> Result<(), RuntimeError> {
        let value = self.evaluate(value)?; // Evaluates right hand side first
        // update the nearest scope that already has the name, innermost first
        for scope in self.scopes.iter_mut().rev() {
            if let Some(slot) = scope.values.get_mut(name) {
                *slot = Some(value);
                return Ok(());
            }
        }
        // searched every scope: assignment must not create the name
        Err(RuntimeError::new(
            line,
            format!("Undefined variable '{}'.", name),
        ))
    }
}
