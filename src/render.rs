// Render pipeline: turn render(element, title = ..., lang = ...) into a sibling
// .html file. HTML construction lives in code_generator; this module owns the
// file target, filesystem errors, and the success message.
use crate::ast::{Element, RenderOption, RenderOptionKind};
use crate::code_generator::{DocumentOptions, generate_document};
use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::evaluator::stringify;
use std::fs;
use std::path::{Path, PathBuf};

fn output_path(source: &Path) -> PathBuf {
    source.with_extension("html")
}

fn write_document(html: &str, source: &Path, line: usize) -> Result<PathBuf, RuntimeError> {
    let path = output_path(source);
    fs::write(&path, html).map_err(|error| {
        RuntimeError::new(
            line,
            format!("Failed to write '{}': {}", path.display(), error),
        )
    })?;
    eprintln!("Rendered {}", path.display());
    Ok(path)
}

// Evaluate title/lang in the enclosing scope before the element body runs.
// Options are parsed at most once each, so plain assignment is unambiguous.
fn evaluate_options(
    options: &[RenderOption<'_>],
    env: &Environment,
) -> Result<DocumentOptions, RuntimeError> {
    let mut document = DocumentOptions::default();
    for option in options {
        let value = stringify(&env.evaluate(&option.value)?);
        match option.kind {
            RenderOptionKind::Title => document.title = Some(value),
            RenderOptionKind::Lang => document.lang = value,
        }
    }
    Ok(document)
}

// Generate the element with the current environment and write the document next
// to the source file. The source path is checked before options or the element
// body run, so a REPL render fails without evaluating anything.
pub fn render_element(
    element: &Element<'_>,
    options: &[RenderOption<'_>],
    env: &mut Environment,
    line: usize,
) -> Result<(), RuntimeError> {
    let source = env
        .source_path()
        .map(Path::to_path_buf)
        .ok_or_else(|| RuntimeError::new(line, "render() requires running a .fmyr file."))?;

    let document = evaluate_options(options, env)?;
    let html = generate_document(element, env, &document)?;
    write_document(&html, &source, line)?;
    Ok(())
}
