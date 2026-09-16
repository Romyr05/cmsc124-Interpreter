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
| `./run <file>` | [Executes a program. Available from Lab 4.] |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | [Evaluates each expression and prints its value.] |
| `./run` | [Starts the REPL.] |


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
| nil | `null` | Absence-of-value / null reference (`Token::Null`) |


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

- Whitespace significant: [yes or no, and where]
- Statement terminator: [e.g. semicolon, newline, none]
- Block delimiters: [e.g. braces, indentation]
- Grouping delimiters: [e.g. parentheses]

## Token output format

```
[one line of real --tokenize output]
```

[What each field means. Frozen as of Lab 1; changes are recorded in the
changelog.]

## Grammar

```
[Your complete context-free grammar, current as of the latest activity.
Unambiguous, with precedence and associativity encoded in rule structure.]
```

## Parse output format

```
[one line of real --parse output, e.g. (+ 1.0 (* 2.0 3.0))]
```

- Groupings print as: [form]
- Numbers print as: [form]

## Semantics

### Values and types

[What runtime values exist, and how they are represented in the host
language.]

### Value printing

- Numbers: [e.g. 5 rather than 5.0]
- Nil: [spelling]
- Strings: [with or without quotes]

### Truthiness

[The complete rule. Which values are false in a condition; everything else is
true.]

### Operator semantics

- Arithmetic: [accepted operand types]
- `+` on strings: [concatenation, error, or coercion]
- Mixed types: [what happens]
- Comparison: [accepted operand types]
- Equality across types: [false, or an error]
- Division by zero: [value produced, or runtime error]

### Scope and bindings

- Redeclaration in the same scope: [allowed or an error]
- Uninitialized variable holds: [value]
- Shadowing: [behavior]
- Undefined name: [static error with exit 65, or runtime error with exit 70]

### Control flow and functions

- Logical operators return: [booleans, or the operand]
- Dangling else binds to: [which if]
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
