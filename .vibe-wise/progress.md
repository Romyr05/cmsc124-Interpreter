# Learning Progress

## Line continuation (multiline expressions)
- Introduced (AI explained): Python-style implicit line joining (scanner tracks paren depth, drops Newline while depth > 0) and operator-continuation rule.
- Needs reinforcement: learner initially read "Python-style" as changing block syntax (`:` vs `{}`); clarified it only concerns when a newline ends a statement.
- Needs reinforcement: learner assumed an unfinished line automatically continues; AI explained scanner emits Newline before parser sees context, and the `1 + 2\n-3` ambiguity.
- Learner's own intuition ("unfinished line should continue") matches the trailing-operator rule.

- Demonstrated understanding: chose parser over scanner because the parser knows grammar context ("scanner just scans tokens"); correctly predicted `1 + 2\n-3` stays two statements.
- Demonstrated understanding: placed skip_newlines after `let operator = *self.previous()` (before parsing the right side).
- Needs reinforcement: thought the helper should error using `depth`; AI explained depth is scanner-only and the existing rules report errors.
- Paren/brace depth rule was already implemented (src/scanner.rs:121, 265–267).
- Tests chosen by learner: split `+` continuation, and `1 + 2` / `-3` staying separate. Added tests/lab3/multiline/continuation.fmyr (AI wrote at learner's request).

## Logic operators (and/or)
- Introduced (AI explained): short-circuit evaluation; eval_binary evaluates both sides before matching; precedence vs associativity; precedence comes from call order.
- Demonstrated understanding: chose to place the and/or check before evaluating `right` for short-circuit.
- Needs reinforcement: confused precedence with associativity (asked if the and/or test was about associativity); initially planned to error when left/right types differ (not needed with is_truthy).
- Learner decisions: short-circuit ("less to check"), operands via is_truthy like `not`, return Bool, switch to `and` tighter than `or`.
- Implemented by AI (learner approved each step): precedence swap (and tighter than or), eval_logical short-circuit, tests/lab3/logic/and_or.fmyr, combined.expected updated. Uncommitted on feat/logic-operators.
- Explained: why each precedence rule calls the next rule twice (first operand + each operand after the operator) and how call depth creates precedence.

## Stack depth and recursion limits
- Introduced (AI explained): call stack/frames, why recursive descent + recursive evaluator overflow, Rust stack overflow aborts (not catchable); how Python/Node/Ruby/serde_json/Go handle it (depth limit, stack-remaining check, stack growth).
- Learner correctly identified assignment (#4) as a Lab 4 feature rather than a bug.
- Decision deferred by learner: logged as GitHub issue #53.

## Identifiers as Expr::Variable
- Introduced (AI explained): parser discards name-vs-text distinction (both become Literal Str), so fix must start in parser; exhaustive match makes compiler list remaining sites; token vs name+location node styles (jlox, Python, Go, rustc); borrowed &str vs owned String.
- Needs reinforcement: initially thought a new TokenType and a letter check were needed in the parser (scanner already produces Identifier).
- Learner decision: name + separate location fields ("better long term, easier to debug").
- Implemented by AI (learner approved): Expr::Variable { name: String, line: usize } on feat/expr_identifiers (branched from main), uncommitted.

## Stateful execution (#60, Lab 4 Week 2)
- Introduced (AI explained): environment = name→value table; ownership/lifetime across REPL lines; owned data vs borrowing from per-line source; parameter vs struct vs global for giving evaluate access.
- Learner's first idea: "a variable for name→value, a function creates it, evaluate accesses it" — location/ownership not yet specified.
- Needs reinforcement: read "sharing the path" as putting everything in one file; AI clarified it means one shared function with callers keeping their own error policy.
- Learner decisions: do #62 before #60; state in a struct ("modularized"); create it in repl `run` before the loop (and before the item loop in file mode); a failed declaration creates nothing, Python-style.
- Demonstrated understanding: evaluate right-hand side first, then store, so a runtime error leaves the table untouched ("we still need to check if it's correct").
- Learner decisions (#62): redeclaring with `let` in the same scope is an error; a later plain `x = 3` overwrites (assignment, #63); a `let` line prints nothing in REPL or --eval (follow #60/#61: only expression values print). Learner retracted "most languages overwrite" after comparison table.
- Needs reinforcement: thought the AST node stores the value; guessed the let node's fields were "expression and element" (Expr is right for the initializer; no element; name was missing).
- Explained (Discuss): two flows (declare = evaluate -> check -> insert; read = lookup -> value or undefined); `&` vs `&mut` references; "binding" = one table entry; REPL keeps one Environment across lines while each line's tokens/tree are discarded.
- Learner summary before Discuss: struct holds names+values, accessed through methods (correct); merged declare and read flows into one.
- Learner decision: name the struct `Environment` (not Interpreter).
- Implemented by AI (learner approved, uncommitted on feat/let-declarations): Environment { values: HashMap<String, Value> } with declare/evaluate methods; TopLevel::Let { name, initializer, line }; parser let_declaration; generate takes &Environment; --parse prints (let x ...); tests/lab4/bindings (6 tests); README redeclaration + undefined-name rules.
- PR #65 opened (feat/let-declarations, 3 commits, "Part of #62"); learner asked for no attribution.
- Explained: self/methods (&self vs &mut self, why every recursive call carries self); exhaustive match and why TopLevel::Let arms repeat (duplicated run path + type-allowed-but-unparsed element children).
- Learner decisions (rest of #62): a variable cannot change type (reason not yet given); uninitialized = option B: `let x` allowed with no value and no type, first assignment sets the type (learner: "label has no value and must have no type too"). Chose B after considering future `if` (Framyr has no `if` yet; README placeholder only).
- Learner confirmed visibility: usable only after its `let`.
- Implemented by AI (learner approved): optional initializer (Option<Expr>), Environment stores Option<Value>, "used before assignment" error, 3 tests, README binding rules. Pushed to PR #65, now "Closes #62".

## Shared run path (#60)
- Explained: what the shared function is (the duplicated scan/parse/run steps; only error handling differs per caller); option A (run a list of items) vs B (run source text).
- Learner decisions: stacked PR off feat/let-declarations; option B (source text) because CliError already has Compile/Runtime; shared function does the printing; do shared path now, finish acceptance with #63.
- Needs reinforcement: thought A's cost was parsing every item into a list (file already gets a list; REPL would just wrap one item; A's real cost is duplicated scan+parse).
- Learner chose new module src/runner.rs.
- Implemented by AI (learner approved): runner::run_source; eval_file and eval_line call it. PR #66 (base feat/let-declarations, "Part of #60"). Only behavior change: empty REPL line prints nothing.
- Next: #63 assignment (enforce fixed type; first assignment sets type for `let x`) to complete #60 acceptance.
- CI: added lab4 step on PR #65 (7356ccf).
- REPL line numbers: learner chose counting lines across the session, on PR #66 (asked AI to "just accomplish it"; mechanism chosen by AI: Tokenizer::starting_at + run_source first_line + REPL counter). Commit 0dbbbb9.
- Issue edits (learner approved): #60 acceptance "update" moved to #63; PR #66 now "Closes #60".
- Resolved conflict: #63's "permit a binding's value type to change" comes from the Lab 4 spec; learner switched to dynamic types (type can change). README on PR #65 updated (ea9ed76). `let x` rule kept (no value until first assignment).
- Explained: fixed vs changeable variable types (Python/JS/Lox vs Rust/TypeScript).
- PR #66 merged into feat/let-declarations 17s after #65 merged to main, so its commits missed main and #60 stayed open. Re-opened as PR #68 (feat/shared-execution -> main, Closes #60).

## Lab 4 gap review (2026-11-14)
- Compared Lab 4 spec PDF (modules/mods) to main: missing plain ./run, assignment, scopes, print, examples/, README decisions; defense Nov 16-17.
- Learner decisions: block syntax B (element bodies are scopes, `let` allowed inside); assignment is a statement (no `let y = (x = 5)`, no chaining); add a `print` statement.
- Open decision: `print` inside element bodies (AI explained: prints before the element's HTML because generate returns a String; learner wants it to show scoping). Recorded on #69.
- Explained: expression vs statement; declare (current scope) vs assign (search outward, never create); environment chain/shadowing; restore scope on error; scope vs lifetime vs binding time.
- Learner check: environment = HashMap name -> Option<Value> (correct; refined: only bindings stored, lifetime comes from where it's created). evaluate-as-method = function with self (mostly mechanical; core logic is declare, Variable lookup, let_declaration, env creation site). Self-check half right: missed that failed `let` leaves x undefined.
- Issues created: #69 print, #70 element-body scopes, #71 scope/print tests, #72 README decisions, #73 examples/. Edited #61 (output convention), #63/#64 (assignment is a statement). Labels: lab4 + one type.
- Suggested order: #61 plain ./run -> #63 assignment -> #69 print -> #70 scopes -> #71 tests -> #72/#73 docs.
- Decision: `print(x)` with required parentheses, terminal-only, allowed inside element bodies (prints before the element's HTML); page content stays via elements like `text()`. #69/#71 updated.
- Found: scanner drops newlines inside `{}` (blocks #70; added to #70); REPL can't take multi-line elements (#74); parse errors lack line numbers + missing file exits 74 (#75). Closed #55 (fixed by 12d73bc).

## print() statement (#69)
- Explained: a new statement touches every stage (keyword/token, parser rule reachable from parse_program/parse_top_level/element bodies, TopLevel::Print, execution in runner + code_generator, tree_printer); body print runs inside generate, so it appears before the element's HTML.
- Dependencies (AI checked): #63 not needed; #61 needed for the plain ./run acceptance lines; #70 needed only for the "inner binding" line (plus children loop / &Environment may change later).
- Learner decision: build print first (test via --eval/REPL), then #61. #69 closes after #61 and #70.
- Learner approach: print keyword checked in parse_top_level like let; errors via self.expect (needed prompting; first said CliError).
- Explained: ParseError (parser) vs CliError (runner wraps -> exit 65); `print()` fails in expression() with no new code; body print uses println! and never touches `body`, so it appears before the HTML.
- Needs reinforcement: error layers (named CliError as the parser helper); first thought print output meant removing the <div>.
- Demonstrated: print is terminal logging/checking, HTML is the page.
- Implemented by AI (learner approved, atomic commits, no attribution) on feat/print_statement: 77eb654 token, 8a60aac top-level print + --parse, 56b2cf9 print in element bodies, 5a871e0 README. Not pushed; no PR yet. Harness lab0-4 pass.
- Explained: TopLevel = statement (Expr/Element/Let/Print); Expr = value-producing; Print as TopLevel blocks `1 + print(2)` and fits Element.children: Vec<TopLevel>. Noted TopLevel name is misleading (means statement); rename not done.
- Demonstrated: summarized TopLevel as "a statement that is an expression, let, element, or print" (correct; refined: TopLevel::Expr wraps an expression).
