# Project Map

## Purpose
Framyr: a DSL for building GUIs from components (div, text, button, ...), interpreted in Rust and compiled to HTML. CMSC 124 course project, built lab by lab (lab0–lab3 so far).

## Requirements
- CLI: `./run --tokenize|--parse|--eval <file>`, bare `./run` starts REPL.
- Exit codes: 64 usage, 65 compile (scan/parse), 70 runtime, 74 I/O (current code).
- Tests run by the course harness (`run_tests.py`) against `tests/lab*/manifest.json`; file extension `.fmyr`.
- Statements are newline-terminated.

## Components
- `src/scanner.rs` — Tokenizer; emits `Newline` tokens (collapses repeats), suppressed while `depth > 0` (inside `(` or `{`) — implemented.
- `src/token_types.rs` — Token / TokenType.
- `src/keyword_list.rs` — keyword table (components, attributes).
- `src/parser.rs` — recursive descent; `parse_program` (whole file, newline sync) and `parse_top_level` (REPL). Produces `TopLevel::Expr | TopLevel::Element`. `skip_newlines()` after each binary operator lets a line ending in an operator continue (learner-chosen design; committed on feat/multiline as 646736c + 954854b, pushed; test tests/lab3/multiline/continuation.fmyr).
- `src/ast.rs` — `Value` (Number(f64)/Str/Bool/Nil), `Expr`, `Element`, `Attribute`.
- `src/evaluator.rs` — tree-walking evaluator; `stringify` for printing.
- `src/code_generator.rs` — Element → HTML.
- `src/tree_printer.rs` — `--parse` output.
- `src/errors.rs` — ScanError, ParseError, RuntimeError, CliError (exit codes).
- `src/main.rs` — CLI dispatch; `src/repl.rs` — line REPL; `src/runner.rs` — shared scan/parse/run (`run_source`), on PR #66.

## Main Flow
source --scan--> tokens (+Newline) --parse_program--> Vec<TopLevel>
  TopLevel::Expr    --evaluate--> Value --stringify--> stdout
  TopLevel::Element --generate--> HTML  --> stdout
(implemented, verified in code)

## Data and Trust Boundaries
No storage or external services. Input = source file / stdin.

## Build and Deployment
- `./build.sh` → `cargo build --release`; `./run` execs `target/release/rust`.
- CI `.github/workflows/test.yml`: fmt check, clippy -D warnings, build, harness lab1–lab3.
- No dependencies in Cargo.toml.

## Unknowns
- and/or, multiline, Expr::Variable: all merged to main (PRs #51, #52, #54; verified 2026-11-06 at ccf0960).
- Lab 4 Week 2 issues #60, #61, #62, #63, #64 open. #62 implemented in PR #65 (feat/let-declarations, open, closes #62): `evaluator::Environment` (HashMap<String, Option<Value>>, one per file run / REPL session), `TopLevel::Let` (optional initializer). #60 shared path in PR #68 (to main; #66 merged into wrong base): `src/runner.rs::run_source`; #60 acceptance waits on #63. REPL errors numbered by session line (PR #66). tests/lab4 in CI (PR #65).
- #69 print: implemented on feat/print_statement (4 commits, unpushed); TopLevel::Print(Expr); runs in runner and code_generator::append_children (println!, never into HTML). Acceptance waits on #61, #70.
- Known gaps found 2026-10-25 (not yet addressed): assignment left-associative; parse errors lack line numbers; missing file exits 74 vs 66.
- Deep-nesting stack overflow (parser + evaluator) and identifiers-as-strings: deferred, tracked in GitHub issue #53. Options discussed (depth limit / bigger stack / Expr::Variable); no decision made.
- Lab 3 audit (2026-10-27): lab0 manifest ext `.testFile` + `./run <file>` unsupported, lab0 commented out in CI; missing tests (negation error, div-by-zero value, string-vs-number comparison, expect error:, `2 < 2`); README Semantics placeholders and nil prints `null` not `nil`.
