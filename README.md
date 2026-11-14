# Framyr

## Creators

- Renz Fredrick Banas (d3nzus)
- John Romyr Lopez (romyr05)

## Overview

A domain-specific-language (DSL) designed for GUI-construction based around components, executing and compiling them into HTML.

## Host language and build

- Host language: Rust 1.98.1
- Version metadata: rust-toolchain.toml
- Build: `./build.sh`
- [Anything a fresh clone needs to know.]

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | Runs a program. Only `print()` and top-level elements produce output; bare expression values are evaluated and discarded. |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | Runs a program like `./run <file>`, but also prints each bare expression's value. |
| `./run` | Starts the REPL; bare expression values are printed. |

Every mode checks the whole source for scan/parse errors (65) before running anything.


Exit codes: 0 [when successful], 65 [when input invalid], 70 [when system error].

## File extension

`[.fmyr]` [Must match the `ext` field in every tests/lab*/manifest.json.]

## Lexical structure




### Keywords

#### Components

| Keyword | function |
|---|---|
| `div` | container component, wraps other components |
| `text` | text component, programmer can freely adjust font and color |
| `button` | button component, can run a pre-defined function |
| `box` | similar to div but predetermined height and width |
| `image` | image component, can contain a filepath to the image |
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
| Keyword | function |
|---|---|
| `id` | id attribute, uniquely identifies a component
| `class` | class attribute, identifies common components
| `color` | color style attribute, can run a pre-determined color scheme
| `align` | align style attribute, can be set to start, center, end
| `padding` | padding style attribute, can be set to integer values
| `margin` | margin style attribute, can be set to integer values
| `height` | height style attribute, can be set to integer values
| `width` | width style attribute, can be set to integer values
| `border` | border style attribute, can be set to integer values


## Events
 
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

| Operator | Category | Operands | Associativity | Precedence |
|---|---|---|---|---|
| [op] | [arithmetic, comparison, logical, assignment, other] | [unary or binary] | [left, right, none] | [1 = loosest] |
| = | assignment | binary | none | [1 = loosest] |
| + | arithmetic | binary | none | [1 = loosest] |

### Literals


| Kind | Syntax | Produces |
|---|---|---|
| number | e.g. `42`, `3.14` | Numeric value (int or float, `Token::Number`) |
| string | e.g. `"hello"`, escapes supported | Text value (`Token::String`) |
| boolean | `true`, `false` | Boolean value (`Token::Boolean`) |
| null | `null` | Absence-of-value / null reference (`Token::Null`) |


### Identifiers

- Start characters: [which]
- Continue characters: [which]
- Case-sensitive: [yes or no]
- [Reserved patterns, length limits, or other restrictions.]

### Comments

- Line comments: [// inline Comments ]
- Block comments: [/* supported */]
- Nesting: [supported or not] 
- [Harness note: comment_prefix in tests/lab*/manifest.json is set to the
  token above.]

## Whitespace and termination

- Whitespace significant: only newlines. Spaces and indentation are ignored.
- Statement terminator: newline, at top level and inside element bodies. Inside a body, children may also share one line (`div() { text() { "a" } text() { "b" } }`).
- Block delimiters: braces `{ }` for element bodies. A newline inside braces still ends a line.
- Grouping delimiters: parentheses `( )`. A newline inside parentheses is ignored, so an expression can continue across lines.

## Token output format

```
[one line of real --tokenize output]
```

[What each field means. Frozen as of Lab 1; changes are recorded in the
changelog.]

## Grammar

```
program     -> ( statement ( NEWLINE | EOF ) )* EOF
statement   -> element
             | letDecl
             | printStmt
             | assignment
             | exprStmt

letDecl     -> "let" IDENTIFIER ( "=" expression )?
printStmt   -> "print" "(" expression ")"
assignment  -> IDENTIFIER "=" expression
exprStmt    -> expression

element     -> ELEMENT "(" attributes? ")" "{" body "}"
ELEMENT     -> "button" | "box" | "text" | "image" | "div" | "par"
attributes  -> attribute ( "," attribute )*
attribute   -> ATTRIBUTE "=" expression
ATTRIBUTE   -> "id" | "class" | "align" | "padding" | "margin"
             | "height" | "width" | "color" | "border"
body        -> ( child | NEWLINE )*
child       -> element | letDecl | printStmt | assignment | exprStmt

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
- Inside an element body `{ }`, newlines separate children, but children may also share a line.

Statement notes:
- `assignment` is chosen when a line starts with `IDENTIFIER "="`. `=` is not an expression operator, so assignment cannot appear inside an expression or be chained.
- `print` requires its parentheses; `print x` and `print()` are parse errors.

## Parse output format

```
[one line of real --parse output, e.g. (+ 1.0 (* 2.0 3.0))]
```

- Groupings print as: [form]
- Numbers print as: [form]

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

Numbers are all `f64` (one numeric type, dynamically typed).

### Value printing

- Numbers: trimmed, so `5.0` prints as `5` (Rust's `f64` Display does this).
- Nil: `nil`.
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
65 and produces no evaluated output. A runtime error exits with code 70. Output
produced by earlier expressions remains visible, while the failing expression
and later expressions produce no output.

### Scope and bindings

- Declaration: `let name = expression`, or `let name` with no value yet. `let` creates the binding in the current scope.
- Scopes: the file (or REPL session) is the global scope. Every element body `{ ... }` is a new scope nested inside the one around it, to any depth. Bindings declared in a body disappear when the body ends.
- Lookup: a name is searched from the innermost scope outward; the first match wins.
- Errors in a body: the body's scope is still discarded, so the REPL continues at the global scope.
- Visibility: a binding can be used on any line after its `let` (including element bodies and later REPL lines), never before.
- Types: dynamic. A variable has no fixed type; assignment may give it a value of any type (`let x = 5` then `x = "hi"` is allowed).
- Assignment: `name = expression` on its own line, a statement, not an expression. It evaluates the value, then updates the nearest scope (innermost outward) that already has `name`; an assignment inside an element body can update an outer binding. It never creates a binding, and it prints nothing.
- Assignment errors: an unknown name is a runtime error (exit 70): `Undefined variable 'name'.` A target that isn't a plain name (`(1 + 2) = 3`) is a parse error (exit 65): `Invalid assignment target.` Chained assignment (`a = b = 3`) and assignment used as a value (`let y = (x = 5)`) are parse errors (exit 65).
- Redeclaration in the same scope: an error. `let x = 1` then `let x = 2` is a runtime error (exit 70): `Variable 'x' already declared.`
- Uninitialized variable holds: nothing. `let x` has no value until its first assignment; reading it first is a runtime error (exit 70): `Variable 'x' used before assignment.`
- Shadowing: allowed across scopes. An inner `let x` hides an outer `x` inside that body without changing it; after the body ends, the outer `x` is visible again with its old value.
- Undefined name: runtime error with exit 70: `[line N] Runtime error: Undefined variable 'name'.`

### Control flow and functions

- `if (condition) { ... } else { ... }`: parentheses and braces are required. The condition uses the truthiness rule above, and only the chosen branch runs; the other branch is never evaluated. `else` is optional: with no `else`, a falsey condition does nothing. `else if (...) { ... }` chains another condition.
- `else` goes on the same line as the closing `}` (`} else {`), because the newline after `}` ends the `if` statement.
- Each branch body is its own scope, like an element body: a `let` inside it disappears when the branch ends, and assignment can update outer bindings.
- Branch bodies hold statements only (`let`, assignment, `print`, `if`, bare expressions); an element inside one is a parse error (exit 65). An `if` may appear inside an element body.
- Logical operators return: [booleans, or the operand]
- Dangling else binds to: the nearest `if`. With required braces the question only arises through `else if`, which always belongs to the `if` just before it.
- Closure capture of a loop variable: [per iteration, or shared]
- Function with no return statement produces: [value]
- Arity mismatch: [message and exit code]

## Native functions


| Name | Arguments | Returns | Notes |
|---|---|---|---|
| [name] | [count and types] | [type] | [caveats] |


## Errors and diagnostics

Message format:

```
[one real static error]
[one real runtime error]
```


| Failure | Exit code |
|---|---|
| [lexical error] | 65 |
| [syntax error] | 65 |
| [runtime error] | 70 |


## Testing conventions


| Folder | Activity | Mode | Flag |
|---|---|---|---|
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |


```
[specific tests]...
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
```

## Sample code

```
[a short program]
```

Output:

```
[its output]
```

## Design rationale

[Why the language is the way it is. Cover the choices that surprised you, the
features you cut, and the decisions you reversed. Specific reasons, not
approval of your own work.]

## Known limitations

- [What doesn't work, what is unimplemented, where behavior is worse than you
  would like.]

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | [entry] |
| 1PR1 | defined valid tokens and operators (ie. Grammars) |
| 1PR2 | expanded on keywords and allowed tokenizer to detect them |
