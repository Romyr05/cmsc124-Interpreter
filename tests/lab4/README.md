# Lab 4 tests and manual demonstrations

Run these commands from the repository root after `./build.sh`.

## Annotated file tests

```bash
python3 run_tests.py tests/lab4
```

The existing harness runs `.fmyr` files through plain `./run`, checking stdout,
stderr substrings, and exit codes. Its per-file timeout catches recovery loops.
`recovery/` intentionally contains multiple errors per file so synchronization
can be observed; other error tests keep one error per file. Expected stdout is
empty for these syntax-error cases, including the valid earlier prints.

For a manual recovery demonstration:

```bash
./run tests/lab4/recovery/nested_body_errors.fmyr
```

Expect four diagnostics, in source order: missing variable name, missing `(`
after `print`, missing expression, and missing `(` after the final `print`.
There must be no program output, no extra unclosed-body error, and exit code 65.
The harness checks the annotated diagnostic substrings; inspect diagnostic count
and order during this demonstration.

Also demonstrate `top_level_errors.fmyr`, `malformed_nested_header.fmyr`, and
`stray_closer.fmyr` from the same directory. Their annotations state the expected
errors. Every case must terminate, report all indicated errors, and exit 65.

## REPL demonstrations

The `.repl` files in `manual/` are demonstration inputs, not file-harness tests.
Start `./run` and type a file's lines into one session, or feed that file to the
same REPL input loop:

```bash
./run < tests/lab4/manual/nested_error.repl
```

Do not pass the filename as an argument: `./run file` selects file execution,
which exits on an error and suppresses bare expression values. Comments describe
expected values and diagnostics; they do not run assertions. Ignore the `>`
prompts when comparing output. Runtime diagnostics also include the session line
number. At EOF the REPL exits successfully after reporting individual input errors.

Run each demonstration in a fresh session:

| Input file | What to observe |
|---|---|
| `manual/nested_error.repl` | Nested failure reports `missing`; later values are blue, green, 7. Both scopes were restored, assignment still works, and later declarations survive. |
| `manual/normal_scope_exit.repl` | An empty div document prints. `temporary` is then undefined; declaring it globally gives 9. |
| `manual/failed_scope_exit.repl` | `missing` and then `temporary` are undefined; no partial HTML prints. A later global declaration gives 9. |
| `manual/outer_assignment_survives.repl` | An inner error occurs, but the completed outer assignment remains: count is 2. |
| `manual/failed_initializer.repl` | `missing` and `fresh` are undefined. A later declaration of fresh works; values are 9 and 8. |
| `manual/static_error.repl` | The malformed submission is rejected; fresh remains undefined, while saved is still 8. |

These checks demonstrate that restoring the active scope does not roll back outer
assignments, that body-local bindings are gone after both success and failure,
and that initializer/syntax errors create no half-finished binding.

## Scoped button example

```bash
./run examples/scoped-button.fmyr
```

Expect:

```html
<!DOCTYPE html><html lang="html"><head></head><body><div style="color:blue;">Choose an action: <button style="color:red;">Submit</button></div></body></html>
blue
```

The div header reads the global blue binding; its body creates a red binding for
the button header. The final blue proves the outer value survived. The related
annotated case is `rendering/scoped_colors.fmyr`.
