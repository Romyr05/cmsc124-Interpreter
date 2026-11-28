# Framyr

## Creators

- Renz Fredrick Banas (d3nzus)
- John Romyr Lopez (romyr05)

## Overview

Framyr is an imperative DSL for constructing HTML from GUI components. Programs
execute statements in order, create mutable named bindings with `let`, and update
them with assignment. Expressions compute values, including reusable component
values; statements control effects such as printing, changing a binding, or
emitting an element.

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

`render(element, title = ..., lang = ...)` writes a sibling `.html` file named
after the source file and stops interpretation after a successful write. See
[render](#render) below.

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

The parser recognizes `div`, `text`, `par`, `button`, `box`, `image`, and
`script`. `div`, `text`, `par`, `button`, and `script` render tags; `box` and
`image` currently emit only their contents. The broader component catalog below
includes planned features; entries beyond those seven are not recognized as
element keywords yet.

| Keyword | function |
|---|---|
| `div` | container component, wraps other components |
| `text` | heading rendered as `<h1>`, with optional text color |
| `button` | static HTML button; event handlers are planned |
| `box` | similar to div but predetermined height and width |
| `image` | parsed element; image rendering is planned |
| `par` | paragraph rendered as `<p>` |
| `script` | inline JavaScript rendered raw inside `<script>` |
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

These names are reserved attribute keywords. Only `color` and `on_click` currently
change the rendered tags; `color` sets CSS text color and `on_click` attaches an
inline JavaScript click handler. Other attributes are parsed but ignored by the
HTML generator.
| Keyword | function |
|---|---|
| `id` | id attribute, uniquely identifies a component
| `class` | class attribute, identifies common components
| `color` | CSS text color, computed from an expression in the enclosing scope
| `on_click` | inline JavaScript click handler, from a string or script component
| `align` | align style attribute, can be set to start, center, end
| `padding` | padding style attribute, can be set to integer values
| `margin` | margin style attribute, can be set to integer values
| `height` | height style attribute, can be set to integer values
| `width` | width style attribute, can be set to integer values
| `border` | border style attribute, can be set to integer values


## Events

`on_click` is implemented as an attribute that emits an inline `onclick` handler.
The remaining event handlers, functions, and `return` are still planned features.

| Keyword | Function |
|---|---|
| `on_click` | runs JavaScript when the element is clicked |
| `on_hover` | runs a function on mouse hover (planned) |
| `on_change` | runs a function when value changes (planned) |
| `on_submit` | runs a function on form submission (planned) |
| `on_focus` | runs a function when component gains focus (planned) |
| `on_blur` | runs a function when component loses focus (planned) |
| `function` | defines a function (planned) |
| `return` | returns a value from a function (planned) |

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
renderStmt  -> "render" "(" expression ( "," renderOption )* ")"
renderOption -> ("title" | "lang") "=" expression
assignment  -> IDENTIFIER "=" expression
exprStmt    -> expression

element     -> ELEMENT "(" attributes? ")" "{" body "}"
ELEMENT     -> "button" | "box" | "text" | "image" | "div" | "par" | "script"
attributes  -> attribute ( "," attribute )*
attribute   -> ATTRIBUTE "=" expression
ATTRIBUTE   -> "id" | "class" | "align" | "padding" | "margin"
             | "height" | "width" | "color" | "border" | "on_click"
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
             | IDENTIFIER | element | "(" expression ")"
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
`render` prints `(render <element> (title <expr>) (lang <expr>))`, including only
the options present in the source.

## Semantics

### Values and types

Runtime values are held in a Rust `enum` (a tagged union), `Value` in
`src/ast.rs`:

```rust
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Component(String), // generated HTML fragment
    Script(String),    // raw JavaScript body
}
```

We chose a tagged union over a `Box<dyn Any>` + downcast approach because it is
the idiom Rust encourages: matching on the enum gives us runtime interrogation
for free, and the compiler forces us to handle every variant. `Box<dyn Any>`
would rebuild Java's answer inside a language that handed us a better one.

Numbers are all `f64` (one numeric type, dynamically typed), so arithmetic uses
floating-point precision even when a source literal looks like an integer.
Operands are evaluated left to right, except when `and` or `or` short-circuits.

`Component` and `Script` are the two component value kinds. A `Component` holds
an already-generated HTML fragment. A `Script` holds a script element's raw
JavaScript body; it renders as `<script>...</script>` when used as a child. Both
are markup values and are inserted raw when they appear in an element body,
unlike strings, which are HTML-escaped.

### Value printing

- Numbers: trimmed, so `5.0` prints as `5` (Rust's `f64` Display does this).
- Null: `null` (represented internally by `Value::Nil`).
- Booleans: `true` or `false`.
- Strings: without quotes, so `"hi"` prints `hi`.
- Components: the stored HTML fragment, without an added document wrapper.
- Script components: `<script>` followed by the raw JavaScript and `</script>`.

### Components as values

Element literals are expressions. Any component can be bound to a variable and
reused:

```
let card = div() {
  text() { "Hello" }
  button() { "Open" }
}

div() {
  card
  card
}
```

- `let card = div() { ... }` evaluates the element immediately and stores its
  HTML fragment as a component value.
- Using `card` as a child of another element inserts that HTML raw, so the tags
  are preserved. Reusing it inserts the same fragment each time.
- The component body runs once, when the binding is evaluated. Its declarations
  belong to the element body's own scope, and any `print` runs at that time.
- Component values are snapshots. Changing a variable used inside the component
  after the binding does not update the stored HTML.
- `let` and assignment share the same rule, so a component can be replaced
  later with `card = text() { "New" }`.
- Element expressions can be used inside `if`/`while` statement bodies even
  though direct element statements there are still rejected. This allows
  conditional component construction:

  ```
  let label
  if (score > 50) {
    label = text() { "Pass" }
  } else {
    label = text() { "Fail" }
  }
  div() { label }
  ```

- A script component keeps its raw JavaScript. Used as a child it emits a
  `<script>` element:

  ```
  let helper = script() { "console.log('hi')" }
  div() { helper }
  ```

  The raw body is also what event handlers will use once `on_click` is
  implemented.

A bare component expression at top level follows the normal expression rule: in
plain file execution its value is computed and discarded, while `--eval` and the
REPL print it. To put a component on a page, use it inside an element body or
pass it to `render`.

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

- Syntax: `render(component, option = expression, ...)` on its own line, for example
  `render(box() { text() { "Hello" } }, title = "Greeting", lang = "en")`.
- The first argument is an expression that must evaluate to a component value. It can
  be an element literal or a variable holding a component:
  - `render(div() { text() { "Hello" } })`
  - `let card = div() { text() { "Hello" } }` then `render(card, title = "Card")`
  - A script component renders as `<script>...</script>` in the document body.
  - `render()` is a parse error. A non-component value such as `render(5)` is a
    runtime error (exit 70): `render() expects an element or a component value.`
- Optional named parameters customize the document boilerplate:
  - `title`: evaluated in the enclosing scope, converted with the normal value-printing
    rules, HTML-escaped, and written as `<title>...</title>` inside `<head>`.
  - `lang`: evaluated in the enclosing scope, converted with the normal value-printing
    rules, HTML-escaped, and used for `<html lang="...">`. The default remains `"html"`.
  - Parameters may appear in any order after the component. Unknown, duplicate, or
    malformed parameters are parse errors (exit 65). With no parameters, the document
    wrapper is unchanged: `<head></head>` and `<html lang="html">`.
  - Parameter expressions run before the component expression, so they cannot read
    bindings declared inside an inline element body. A component variable was
    already evaluated when its binding ran, so its snapshot is unaffected.
- `render` is allowed only at top level. It is a parse error inside an element or control-flow body.
- A source file may contain at most one render; a second one is a parse error (exit 65).
- Output path: the source file with its extension replaced by `.html`. `examples/demo.fmyr` writes `examples/demo.html`; an existing file is overwritten.
- After a successful write, interpretation stops. Statements after render do not run.
- Top-level elements before render still print their own documents to stdout.
- `render` prints `Rendered <path>` to stderr on success. It produces no stdout output.
- In the REPL there is no source file, so render reports `render() requires running a .fmyr file.` and the session continues.
- `--parse` shows it as `(render <component> (title <expr>) (lang <expr>))`, including
  only the options present in the source; `--tokenize` and `--parse` never write files.

### script

- Syntax: `script() { ... }`, usable anywhere an element is allowed: at top level,
  nested inside another element, or as the render root.
- The element emits `<script>` followed by its body's expression values and then
  `</script>`. Expression values use the normal value-printing rules and are inserted
  **raw**, without HTML escaping. Consecutive expression values are separated by a
  newline so separate DSL strings do not merge into one JavaScript statement.
- A top-level script prints a complete default document to stdout, like any other
  element. To attach JavaScript to a rendered page, nest the script inside the
  element passed to `render`.
- A script is also a component value: `let helper = script() { "..." }` stores its
  raw JavaScript body. Using `helper` as a child emits `<script>...</script>`;
  `render(helper)` renders it as the document body. The raw body is retained so
  event handlers can use it later.
- Script bodies may contain `let`, assignment, `print`, `if`, and `while`, but
  nested elements are a parse error (exit 65). Using a component value as a
  direct script expression is a runtime error because it would insert HTML into
  JavaScript. `print()` still writes to the terminal, never into the generated
  JavaScript.
- As in other element bodies, only direct expression children contribute output;
  bare expressions inside `if`/`while` bodies are evaluated and discarded. To build
  conditional or repeated JavaScript, accumulate it in a string variable and expose
  that variable as the final expression.
- Attributes are parsed but ignored, and no external `src` support exists yet.
- Strings keep the normal literal rules: no escape decoding, and literal newlines
  are allowed. Use single quotes for JavaScript string literals. A literal
  `</script>` in the JavaScript closes the HTML tag early; it is not escaped.

### on_click

- Syntax: `button(on_click = expression) { ... }`. It is also allowed on `div`,
  `text`, and `par`. `box`, `image`, and `script` currently reject it because
  they do not emit an element tag for the handler to attach to.
- The expression is evaluated in the enclosing scope before the element body
  runs, like `color`. It must produce either:
  - a string, used directly as JavaScript source, or
  - a script component, whose raw JavaScript body is used as the handler.
- A script component used as `on_click` is not also emitted as `<script>`.
  To include the script tag, use the component as an element child as well.
- The handler is emitted as a double-quoted `onclick` attribute. HTML-special
  characters are escaped in the generated source; the browser decodes the
  attribute before compiling it. Single quotes are left alone inside the
  double-quoted attribute, so handlers remain readable.
- A duplicate `on_click` on one element is a parse error.
- Direct handler:

  ```
  button(on_click = "alert('Hello!')") { "Say hello" }
  ```

- Script component handler:

  ```
  let hello = script() { "alert('Hello!')" }
  button(on_click = hello) { "Say hello" }
  ```

- Shared function defined in a script, called by the handler:

  ```
  let helpers = script() { "function greet() { alert('Hello!') }" }

  render(div() {
    button(on_click = "greet()") { "Say hello" }
    helpers
  })
  ```

  Here `helpers` is emitted as a `<script>` and the button calls `greet()`.
  Passing `helpers` directly as `on_click` would use the function declaration
  itself as the click handler, which does nothing useful.
- Handlers run later in the browser's global scope; `this` is the element and
  `event` is available. Framyr variables are evaluated at build time, so their
  values are baked into the generated JavaScript. Use JavaScript variables for
  state that changes while the page is open.
- Functions and closures are still not implemented. Inline handlers may be
  blocked by a strict Content Security Policy (see Known limitations).

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
- Component bindings: `let card = div() { ... }` evaluates the element immediately. Its body is its own scope; local declarations disappear when the body ends, while the generated HTML is stored in the binding. Reusing the binding does not re-run the body, so it captures the values that existed when it was bound.
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
| `let` executes successfully | A named storage slot is inserted into the current environment after evaluating the initializer. With an initializer, its value and runtime type are established now. A component initializer is rendered now; the binding stores the finished HTML fragment. |
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
| Bind a component once, reuse its HTML | `tests/lab5/components/bind_button.fmyr`, `tests/lab5/components/reuse.fmyr` |
| Component snapshot ignores later assignments | `tests/lab5/components/snapshot.fmyr` |
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
| tests/lab5 | Control flow, render options, script elements, components, and on_click | inline | none |


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
the body ends, `print(shade)` still reads blue. The button is a static HTML control
unless it has an `on_click` handler; JavaScript behavior can also be added with a
nested `script` element.

`div`, `text`, `par`, and `button` generate HTML tags (`div`, `h1`, `p`, and
`button`). Their `color` expressions are evaluated in the enclosing scope before
the body runs, converted using the normal value-printing rules, and escaped for an
HTML style attribute. Text content is also escaped. CSS interprets the resulting
color value. `on_click` expressions are also evaluated before the body and emit
an inline handler. Other attributes currently parse but do not affect generated
HTML; `box` and `image` currently emit only their contents.

Run [examples/interactive.fmyr](examples/interactive.fmyr) to see document
options, component values, and `on_click` together:

```text
let heading = "Interactive Demo"

let hello = script() {
  "alert('Hello!')"
}

render(div() {
  text() { heading }
  button(on_click = hello) { "Say hello" }
}, title = heading, lang = "en")
```

This writes `examples/interactive.html` with `<html lang="en">`, a `<title>`,
and a button carrying `onclick="alert('Hello!')"`.

Run [examples/components.fmyr](examples/components.fmyr) to see components bound
to variables and reused:

```text
let heading = text() { "Components as values" }

let card = div() {
  heading
  par() { "The card is bound once and passed to render." }
}

render(card, title = "Component values")
```

This writes `examples/components.html`:

```html
<!DOCTYPE html><html lang="html"><head><title>Component values</title></head><body><div><h1>Components as values</h1><p>The card is bound once and passed to render.</p></div></body></html>
```

## Design rationale

Element bodies provide the DSL's nested binding forms. `let` creates a local
binding, while assignment searches for an existing one. Assignment is a statement,
so it cannot silently become a value inside a larger expression. Optional
initializers use a separate uninitialized state to catch reads before assignment.
The same environment and executor serve files, element bodies, control flow, and
the REPL; the entry point only selects whether bare values are displayed.

Element literals are expressions that render eagerly to component values. This
keeps `Value` owned (no borrowed ASTs or source lifetimes), makes reuse a simple
HTML fragment insertion, and matches the imperative rule that a `let` initializer
is evaluated when the `let` runs. Lazy templates, component props, and closures
would each need a larger evaluation model, so they remain future work.

## Known limitations

- REPL submissions must be complete on one line (#74).
- Parser diagnostics currently lack line numbers (#75).
- String escapes are not decoded; literal newlines in strings are supported (#59).
- Only `color` affects element styling; `on_click` emits a behavior attribute;
  the other parsed attributes are ignored.
- `box` and `image` currently emit only their contents. Additional components,
  functions, closures, and the remaining events remain planned features.
- `on_click` only accepts a JavaScript string or a script component. There are no
  Framyr function handlers yet, and `box`, `image`, and `script` reject the
  attribute. Inline handlers may be blocked by a strict Content Security Policy.
- Component values are eager HTML snapshots, not lazy templates. Reusing one does
  not re-run its body or update it after outer bindings change. There are no
  component props or closures yet.
- A bare component expression at top level is still a bare expression: plain file
  execution computes and discards it. Use the component inside an element body or
  pass it to `render`.
- `render` is top-level only, limited to one call per file, and unavailable in
  the REPL. Its first argument must evaluate to a component value. It overwrites
  an existing sibling `.html` file. Its document options are limited to `title`
  and `lang`.
- Direct element statements are still rejected inside `if`/`while` and script
  bodies; element expressions may be used there as values. Component values used
  as script expressions are a runtime error.
- `script` content is emitted raw; external scripts (`src`) and `</script>`
  escaping are not implemented. Its non-event attributes are parsed but ignored;
  event attributes on `script` are rejected.
- Extremely deep expressions can overflow the host stack (#53).

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | Token stream, keywords, literals, operators, and comments. |
| 1PR1 | defined valid tokens and operators (ie. Grammars) |
| 1PR2 | expanded on keywords and allowed tokenizer to detect them |
| Lab 4 | Mutable bindings, lexical body scopes, explicit printing, file execution, and scoped button colors. |
| Lab 5 | `render(element, title = ..., lang = ...)` customizes the document wrapper and ends interpretation; `script() {}` embeds raw JavaScript. |
| Components | Element literals are expressions; `let`/assignment can bind any component; component values are inserted raw as children; `render` accepts an element or component variable; script components retain raw JavaScript. |
| Events | `on_click = <string or script component>` emits an inline `onclick` handler on `button`, `div`, `text`, and `par`. |
