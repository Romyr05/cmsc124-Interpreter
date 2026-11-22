# Framyr

## Creators

- Renz Fredrick Banas (d3nzus)
- John Romyr Lopez (romyr05)

## Overview

Framyr is an imperative DSL for constructing HTML from GUI components. Programs
execute statements in order, create mutable named bindings with `let`, and update
them with assignment. Expressions compute values; statements control effects such
as printing, changing a binding, or generating an element.

## Host language and build

- Host language: Rust, edition 2024, using the stable toolchain in `rust-toolchain.toml`.
- Prerequisites: Rust/Cargo and Bash; Python 3 for the course test harness.
- Build from the repository root: `./build.sh` (creates `target/release/rust`).
- Run `./build.sh` after source changes; `./run` executes the built binary.

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | Runs a program. Only `print()` and top-level elements produce output; bare expression values are evaluated and discarded. |
| `./run --tokenize <file>` | Prints the token stream. |
| `./run --parse <file>` | Prints the parsed tree. |
| `./run --eval <file>` | Runs a program like `./run <file>`, but also prints each bare expression's value. |
| `./run` | Starts the REPL; bare expression values are printed. |

`render(element)` writes a sibling `.html` file named after the source file and
stops interpretation after a successful write. See [render](#render) below.

File execution and `--eval` check the whole source for scan/parse errors before
running anything. `--parse` also requires valid syntax; `--tokenize` prints tokens
and reports scanner errors without executing the program. The REPL validates each
submitted line before running it and keeps earlier session bindings after errors.


Exit codes: 0 for success, 64 for invalid CLI usage, 65 for scan/parse errors,
70 for runtime errors, and 74 for I/O failures such as a missing input file.
Program output goes to stdout; diagnostics go to stderr.

## File extension

`.fmyr`, matching the `ext` field in each `tests/lab*/manifest.json`.

## Lexical structure




### Keywords

#### Components

The parser recognizes `div`, `text`, `par`, `button`, `box`, and `image`.
`div`, `text`, `par`, and `button` render tags; `box` and `image` currently emit
only their contents. The broader component catalog below includes planned features;
entries beyond those six are not recognized as element keywords yet.

| Keyword | function |
|---|---|
| `div` | container component, wraps other components |
| `text` | heading rendered as `<h1>`, with optional text color |
| `button` | static HTML button; event handlers are planned |
| `box` | similar to div but predetermined height and width |
| `image` | parsed element; image rendering is planned |
| `par` | paragraph rendered as `<p>` |
| `input` | text entry field |
| `checkbox` | boolean toggle component |
| `radio` | single-select option among a group |
| `slider` | draggable value selector within a range |
| `dropdown` | collapsible selection list |
| `icon` | small symbolic graphic |
| `list` | scrollable collection of items |
| `grid` | arranges children in rows/columns |
| `row` | horizontal layout container |
| `column` | vertical layout container |


#### Attributes

These names are reserved attribute keywords. Only `color` currently changes the
rendered tags; it sets CSS text color. Other attributes are parsed but ignored by
the HTML generator.
| Keyword | function |
|---|---|
| `id` | id attribute, uniquely identifies a component
| `class` | class attribute, identifies common components
| `color` | CSS text color, computed from an expression in the enclosing scope
| `align` | align style attribute, can be set to start, center, end
| `padding` | padding style attribute, can be set to integer values
| `margin` | margin style attribute, can be set to integer values
| `height` | height style attribute, can be set to integer values
| `width` | width style attribute, can be set to integer values
| `border` | border style attribute, can be set to integer values


## Events (planned)

Event handlers, functions, and `return` are not implemented or reserved keywords.
The following catalog describes intended features.
 
| Keyword | Function |
|---|---|
| `on_click` | runs a function when clicked |
| `on_hover` | runs a function on mouse hover |
| `on_change` | runs a function when value changes |
| `on_submit` | runs a function on form submission |
| `on_focus` | runs a function when component gains focus |
| `on_blur` | runs a function when component loses focus |
| `function` | defines a function, followed by {} and runs the code inside |
| `return` | returns a value of the returned type from the function |

### Operators

| Operators | Category | Operands | Associativity | Precedence (lowest first) |
|---|---|---|---|---|
| `or` | logical | binary | left | 1 |
| `and` | logical | binary | left | 2 |
| `not` | logical | unary | right | 3 |
| `== != < <= > >=` | comparison | binary | left | 4 |
| `+ -` | arithmetic / string concatenation (`+`) | binary | left | 5 |
| `* /` | arithmetic | binary | left | 6 |
| `-` | numeric negation | unary | right | 7 |
| `^` | exponentiation | binary | right | 8 |

`=` belongs to declarations, attributes, and assignment statements. It is not an
expression operator.

### Literals


| Kind | Syntax | Produces |
|---|---|---|
| number | e.g. `42`, `3.14` | Numeric value (int or float, `Token::Number`) |
| string | double-quoted, e.g. `"hello"`; literal newlines allowed, escapes not decoded | String value |
| boolean | `true`, `false` | Boolean value (`Token::Boolean`) |
| null | `null` | Absence-of-value / null reference (`Token::Null`) |


### Identifiers

Identifiers begin with an ASCII letter or `_`, followed by ASCII letters, digits,
or `_`. Names are case-sensitive. Reserved keywords, including attribute names
such as `color`, cannot be binding names. There is no explicit identifier length
limit.

### Comments

- Line comments: `#` through the end of the line.
- Block comments: `#*` through the first `*#`; nesting is not supported.
- Harness annotations use the manifest's `comment_prefix`, `#`.

## Whitespace and termination

- Whitespace significant: only newlines. Spaces and indentation are ignored.
- Statement terminator: newline, at top level and inside element bodies. Inside a body, children may also share one line (`div() { text() { "a" } text() { "b" } }`).
- Block delimiters: braces `{ }` for element bodies. A newline inside braces still ends a line.
- Grouping delimiters: parentheses `( )`. A newline inside parentheses is ignored, so an expression can continue across lines.

## Token output format

For `print("Hello, world!")`, `--tokenize` includes:

```text
Token { token_type: Print, lexeme: "print", line: 1 }
```

Each token reports its enum kind, lexeme, and one-based source line. A string's
lexeme is its contents without the surrounding quotes. `Newline` tokens are hidden
from this output; an `Eof` token is printed last.

## Grammar

```
program     -> ( statement ( NEWLINE | EOF ) )* EOF
statement   -> element
             | ifStmt
             | whileStmt
             | letDecl
             | printStmt
             | renderStmt
             | assignment
             | exprStmt

ifStmt      -> "if" "(" expression ")" block
               ( "else" ( ifStmt | block ) )?
whileStmt   -> "while" "(" expression ")" block
block       -> "{" ( blockStmt | NEWLINE )* "}"
blockStmt   -> ifStmt | whileStmt | letDecl | printStmt | assignment | exprStmt

letDecl     -> "let" IDENTIFIER ( "=" expression )?
printStmt   -> "print" "(" expression ")"
renderStmt  -> "render" "(" element ")"
assignment  -> IDENTIFIER "=" expression
exprStmt    -> expression

element     -> ELEMENT "(" attributes? ")" "{" body "}"
ELEMENT     -> "button" | "box" | "text" | "image" | "div" | "par"
attributes  -> attribute ( "," attribute )*
attribute   -> ATTRIBUTE "=" expression
ATTRIBUTE   -> "id" | "class" | "align" | "padding" | "margin"
             | "height" | "width" | "color" | "border"
body        -> ( child | NEWLINE )*
child       -> element | ifStmt | whileStmt | letDecl | printStmt
             | assignment | exprStmt

expression  -> logic_or
logic_or    -> logic_and ( "or" logic_and )*
logic_and   -> logic_not ( "and" logic_not )*
logic_not   -> "not" logic_not | comparison
comparison  -> term ( ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) term )*
term        -> factor ( ( "+" | "-" ) factor )*
factor      -> unary ( ( "*" | "/" ) unary )*
unary       -> "-" unary | exponent
exponent    -> primary ( "^" unary )?
primary     -> NUMBER | FLOAT | STRING | "true" | "false" | "null"
             | IDENTIFIER | "(" expression ")"
```

Precedence, lowest to highest: `or`, `and`, `not`, comparisons, `+ -`, `* /`, unary `-`, `^`. All binary operators are left-associative except `^`, which is right-associative (`2^3^2` is `2^(3^2)`). Unary `-` binds looser than `^`, so `-2^2` is `-(2^2)`.

Newline rules:
- A top-level statement ends at a NEWLINE (or end of file). Blank lines produce no NEWLINE token.
- Inside `( )` newlines are ignored, so an expression or attribute list can span lines.
- A line ending in a binary operator (or `=` of an assignment) continues on the next line.
- Inside an element body or an `if`/`while` block `{ }`, newlines separate statements, but statements may also share a line.
- `else` must follow the closing `}` on the same line (`} else {`).

Statement notes:
- `assignment` is chosen when a line starts with `IDENTIFIER "="`. `=` is not an expression operator, so assignment cannot appear inside an expression or be chained.
- `print` requires its parentheses; `print x` and `print()` are parse errors.

## Parse output format

For `(2 + 3)`, `--parse` prints:

```text
(group (+ 2.0 3.0))
```

Operators use prefix notation, parentheses become `(group <expr>)`, and numbers
include a fractional part (`2.0`). Statements include `(let name <expr>)`,
`(= name <expr>)`, and `(print <expr>)`; uninitialized declarations print `(let name)`.

## Semantics

### Values and types

Runtime values are held in a Rust `enum` (a tagged union), `Value` in
`src/ast.rs`:

```rust
enum Value { Number(f64), Str(String), Bool(bool), Nil }
```

We chose a tagged union over a `Box<dyn Any>` + downcast approach because it is
the idiom Rust encourages: matching on the enum gives us runtime interrogation
for free, and the compiler forces us to handle every variant. `Box<dyn Any>`
would rebuild Java's answer inside a language that handed us a better one.

Numbers are all `f64` (one numeric type, dynamically typed), so arithmetic uses
floating-point precision even when a source literal looks like an integer.
Operands are evaluated left to right, except when `and` or `or` short-circuits.

### Value printing

- Numbers: trimmed, so `5.0` prints as `5` (Rust's `f64` Display does this).
- Null: `null` (represented internally by `Value::Nil`).
- Booleans: `true` or `false`.
- Strings: without quotes, so `"hi"` prints `hi`.

### print

- Syntax: `print(expression)` on its own line. Parentheses are required: `print x`, `print()`, and a missing `)` are parse errors (exit 65).
- Output: the value is written to the terminal (stdout) using the rules above. It never becomes part of the generated HTML; page content comes from elements such as `text()`.
- Allowed at top level and inside element bodies.
- Ordering: inside an element, `print` runs immediately while the element is being built, but the element's HTML is printed only when the top-level element finishes. So the `print` line appears before that element's HTML:

  ```
  let x = 7
  div() {
    text() { "page" }
    print(x)
  }
  ```
  prints `7`, then the `<div>` HTML.
- `--parse` shows it as `(print <expr>)`.

### render

- Syntax: `render(element)` on its own line, for example `render(box() { text() { "Hello" } })`.
- The argument must be an element literal. `render()` and `render(expression)` are parse errors.
- `render` is allowed only at top level. It is a parse error inside an element or control-flow body.
- A source file may contain at most one render; a second one is a parse error (exit 65).
- The element is generated with the current environment and the normal default document wrapper.
- Output path: the source file with its extension replaced by `.html`. `examples/demo.fmyr` writes `examples/demo.html`; an existing file is overwritten.
- After a successful write, interpretation stops. Statements after render do not run.
- Top-level elements before render still print their own documents to stdout.
- `render` prints `Rendered <path>` to stderr on success. It produces no stdout output.
- In the REPL there is no source file, so render reports `render() requires running a .fmyr file.` and the session continues.
- `--parse` shows it as `(render <element>)`; `--tokenize` and `--parse` never write files.

### Truthiness

Only `false` and `null` are falsey. Numbers (including `0`) and strings
(including `""`) are truthy. `not` returns the inverse truthiness as a boolean.
`and` and `or` also return booleans: `and` skips its right operand when the
left operand is falsey, and `or` skips it when the left operand is truthy.

### Operator semantics

- `-`, `*`, `/`, and `^` require two numeric operands. Comparisons (`<`, `<=`,
  `>`, `>=`) also require two numbers. A wrong operand type raises a runtime
  error with exit code 70.
- `+` accepts either two numbers or two strings. Two strings concatenate;
  mixing a string and a number raises a runtime error with exit code 70.
- Division follows IEEE 754 behavior: dividing a positive or negative number by
  zero produces `inf` or `-inf`, and `0 / 0` produces `NaN`.
- Equality (`==`, `!=`) accepts values of any types; values of different types
  are unequal.
- Comparisons are strict at `<` and `>` and inclusive at `<=` and `>=`; for
  example, `2 < 2` is `false` and `2 <= 2` is `true`.

### Errors during evaluation

Invalid syntax, an unclosed grouping, or an unterminated string exits with code
65 and produces no evaluated output. The parser collects errors and synchronizes
at a newline or enclosing closing brace so later independent errors can also be
reported. A runtime error exits with code 70 and stops subsequent statements.
Earlier output and assignments remain in effect. A failing top-level element does
not emit its partially built HTML, though any `print` already executed inside it
remains visible. In the REPL an error returns to the prompt; it does not end the
session or erase earlier global bindings.

### Scope and bindings

- Declaration: `let name = expression`, or `let name` with no value yet. `let` creates the binding in the current scope. The initializer runs before inserting the new binding; a failed initializer inserts nothing. An inner `let x = x` reads the enclosing `x` before shadowing it.
- Scopes: the file (or REPL session) is the global scope. Every element body `{ ... }` is a new scope nested inside the one around it, to any depth. Bindings declared in a body disappear when the body ends.
- Lookup is lexical (static scope): environment links follow enclosing source bodies, rather than active callers. Search starts in the innermost scope and the first matching binding wins. Function calls and closures are not implemented yet; blocks alone cannot demonstrate lexical versus dynamic function scope.
- Errors in a body: each body restores its enclosing environment while the error propagates, so the REPL returns to its global scope. Earlier assignments to surviving outer bindings remain in effect.
- Visibility: a binding becomes visible after its `let` executes, for the remaining statements of that scope and nested bodies. There is no hoisting. Element attributes run before entering the element body and therefore cannot use declarations from that body.
- Types: dynamic. A variable has no fixed type; assignment may give it a value of any type (`let x = 5` then `x = "hi"` is allowed).
- Assignment: `name = expression` on its own line, a statement, not an expression. It evaluates the value, then updates the nearest scope (innermost outward) that already has `name`; an assignment inside an element body can update an outer binding. It never creates a binding, and it prints nothing.
- Assignment errors: an unknown name is a runtime error (exit 70): `Undefined variable 'name'.` A target that isn't a plain name (`(1 + 2) = 3`) is a parse error (exit 65): `Invalid assignment target.` Chained assignment (`a = b = 3`) and assignment used as a value (`let y = (x = 5)`) are parse errors (exit 65).
- Redeclaration in the same scope: an error. `let x = 1` then `let x = 2` is a runtime error (exit 70): `Variable 'x' already declared.`
- Uninitialized variable holds: nothing. `let x` has no value until its first assignment; reading it first is a runtime error (exit 70): `Variable 'x' used before assignment.`
- Shadowing: allowed across scopes. An inner `let x` hides an outer `x` inside that body without changing it; after the body ends, the outer `x` is visible again with its old value.
- Undefined name: runtime error with exit 70: `[line N] Runtime error: Undefined variable 'name'.` Name resolution runs when a read or assignment executes, using the current environment; there is no separate static name-resolution pass. This matches the interpreter's execution model, including branches that are never evaluated.

### Binding time and storage lifetime

Scope describes where a binding is visible; lifetime describes how long its
storage exists. These are separate from the time a name acquires a value or type.

| Event | Binding and storage behavior |
|---|---|
| File/REPL starts | A global environment is created once for that run or session. |
| `let` executes successfully | A named storage slot is inserted into the current environment after evaluating the initializer. With an initializer, its value and runtime type are established now. |
| `let name` executes | Storage exists, but no value or type is assigned yet. This differs from explicitly storing `null`. |
| Assignment executes | The nearest existing slot gets the new value and its runtime type; storage is reused and the type may change. |
| Element or control-flow body starts | A new scope is pushed onto the environment stack; declarations create slots as they execute. Each loop iteration has a fresh scope. |
| Body ends, normally or with an error | Its environment is removed from lookup and its local binding storage is released. Outer bindings survive. |
| File/REPL ends | The global environment and its bindings are released. |

Global bindings have run/session duration once declared. Body-local storage is
stack-dynamic in the interpreted language: it lasts for that body activation.
The Rust implementation keeps a stack of `Scope` maps, with the global scope at
the bottom. This does not promise implicit heap-dynamic guest bindings. There are
no guest allocation or deallocation operations, escaping closures, or retained
local bindings yet.

The scope tests provide concrete traces:

| Rule | Evidence |
|---|---|
| Create/read initialized bindings | `tests/lab4/bindings/declare_and_read.fmyr` |
| No read before declaration | `tests/lab4/bindings/read_before_declaration.fmyr` |
| Explicit null is initialized | `tests/lab4/bindings/null_is_initialized.fmyr` |
| Uninitialized read versus assignment | `tests/lab4/bindings/uninitialized_read.fmyr`, `tests/lab4/assignment/assign_uninitialized.fmyr` |
| Dynamic type changes; illegal redeclaration | `tests/lab4/assignment/change_type.fmyr`, `tests/lab4/bindings/redeclare.fmyr` |
| Inner shadowing preserves outer value | `tests/lab4/scopes/shadowing.fmyr` |
| Initializer reads before installing inner binding | `tests/lab4/scopes/initializer_reads_outer.fmyr` |
| Lookup and assignment across two bodies | `tests/lab4/scopes/two_levels.fmyr`, `tests/lab4/scopes/assign_two_levels.fmyr` |
| Assignment chooses nearest binding | `tests/lab4/scopes/assign_nearest_shadow.fmyr` |
| Inner binding disappears after body ends | `tests/lab4/scopes/inner_gone_after_body.fmyr` |
| Header reads enclosing scope | `tests/lab4/rendering/scoped_colors.fmyr`, `tests/lab4/rendering/header_before_body.fmyr` |
| Session persistence and cleanup after success/error | Manual REPL inputs and expected results in `tests/lab4/manual/` and `tests/lab4/README.md` |


### Control flow and functions

- `if (condition) { ... } else { ... }`: parentheses and braces are required. The condition uses the truthiness rule above, and only the chosen branch runs; the other branch is never evaluated. `else` is optional: with no `else`, a falsey condition does nothing. `else if (...) { ... }` chains another condition.
- `else` goes on the same line as the closing `}` (`} else {`), because the newline after `}` ends the `if` statement.
- Each branch body is its own scope, like an element body: a `let` inside it disappears when the branch ends, and assignment can update outer bindings.
- Branch bodies hold statements only (`let`, assignment, `print`, `if`, bare expressions); an element inside one is a parse error (exit 65). An `if` may appear inside an element body.
- `while (condition) { ... }`: checks the condition, runs the body if truthy, and checks again. A falsey first check runs the body zero times. Assignment in the body updates the outer binding, which is how a loop ends. Each iteration gets a fresh body scope, so a `let` inside the body is new every time (no redeclaration error) and is gone after the loop. There is no `for` loop yet.
- Logical operators return booleans, using the short-circuit rules above.
- Dangling else binds to: the nearest `if`. With required braces the question only arises through `else if`, which always belongs to the `if` just before it.
- Functions, closures, return values, and arity checks are not implemented yet.

## Native functions

There are no callable native functions yet. `print(expression)` is a statement
recognized by the parser.

## Errors and diagnostics

Examples of current diagnostics:

```text
Error at '=': Expected variable name after 'let'
[line 1] Runtime error: Undefined variable 'missing'.
```

Scanner and runtime diagnostics include a one-based source line. Parser errors
identify the offending token, end of line, or end of input; parser line numbers
are tracked separately in issue #75.

| Failure | Exit code |
|---|---|
| Invalid CLI usage | 64 |
| Lexical or syntax error, before execution | 65 |
| Runtime error, including undefined/uninitialized names and redeclaration | 70 |
| I/O failure | 74 |
| Successful file execution or a REPL session ending at EOF | 0 |

In the REPL, scan/parse/runtime errors print to stderr and return to the next
prompt. Its global bindings survive. Input currently accepts one complete line at
a time; a nested element can be entered on one line. Multiline REPL input is
tracked in issue #74.

## Testing conventions


| Folder | Activity | Mode | Flag |
|---|---|---|---|
| tests/lab0 | Execution smoke test | sidecar | none |
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Control flow (implemented subset) | inline | none |


Parser recovery cases run through the same annotated file harness in
`tests/lab4/recovery/`. They deliberately contain multiple syntax errors to check
synchronization and verify that earlier valid statements do not execute. The
harness applies its normal 15-second timeout.

REPL persistence and recovery are demonstrated manually using the `.repl` inputs
in `tests/lab4/manual/`. Run `./run` and enter their lines, or feed a demonstration
into the REPL:

```bash
./run < tests/lab4/manual/nested_error.repl
```

[Lab 4 demonstrations](tests/lab4/README.md) lists the expected values, diagnostics,
and example output. The `.repl` inputs are not automatically asserted by CI.

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
for lab in tests/lab{0..5}; do
  python3 run_tests.py "$lab" || exit 1
done
```

## Sample code

Run [examples/scoped-button.fmyr](examples/scoped-button.fmyr) with:

```bash
./run examples/scoped-button.fmyr
```

```text
let shade = "blue"
div(color = shade) {
  "Choose an action: "
  let shade = "red"
  button(color = shade) { "Submit" }
}
print(shade)
```

Output:

```html
<!DOCTYPE html><html lang="html"><head></head><body><div style="color:blue;">Choose an action: <button style="color:red;">Submit</button></div></body></html>
blue
```

The div's text is blue and the button's label is red. `color` sets the CSS text
color, rather than the background. The div header reads the global `shade` before
entering its body. The inner declaration supplies red to the button header; after
the body ends, `print(shade)` still reads blue. The button is a static HTML control;
click handlers are not implemented.

`div`, `text`, `par`, and `button` generate HTML tags (`div`, `h1`, `p`, and
`button`). Their `color` expressions are evaluated in the enclosing scope before
the body runs, converted using the normal value-printing rules, and escaped for an
HTML style attribute. Text content is also escaped. CSS interprets the resulting
color value. Other attributes currently parse but do not affect generated HTML;
`box` and `image` currently emit only their contents.

## Design rationale

Element bodies provide the DSL's nested binding forms. `let` creates a local
binding, while assignment searches for an existing one. Assignment is a statement,
so it cannot silently become a value inside a larger expression. Optional
initializers use a separate uninitialized state to catch reads before assignment.
The same environment and executor serve files, element bodies, control flow, and
the REPL; the entry point only selects whether bare values are displayed.

## Known limitations

- REPL submissions must be complete on one line (#74).
- Parser diagnostics currently lack line numbers (#75).
- String escapes are not decoded; literal newlines in strings are supported (#59).
- Only `color` affects element styling; the other parsed attributes are ignored.
- `box` and `image` currently emit only their contents. Additional components,
  functions, closures, and events remain planned features.
- `render` is top-level only, limited to one call per file, and unavailable in
  the REPL. It overwrites an existing sibling `.html` file.
- Extremely deep expressions can overflow the host stack (#53).

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | Token stream, keywords, literals, operators, and comments. |
| 1PR1 | defined valid tokens and operators (ie. Grammars) |
| 1PR2 | expanded on keywords and allowed tokenizer to detect them |
| Lab 4 | Mutable bindings, lexical body scopes, explicit printing, file execution, and scoped button colors. |
| Lab 5 | `render(element)` writes a sibling `.html` document and ends interpretation. |
