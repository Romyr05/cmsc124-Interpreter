// Render pipeline: turn a render(element) argument into a sibling .html file.
// HTML construction lives in code_generator; this module owns the file target,
// filesystem errors, and the success message.
use crate::ast::Element;
use crate::code_generator::generate;
use crate::environment::Environment;
use crate::errors::RuntimeError;
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

// Generate the element with the current environment and write the document next
// to the source file. The source path is checked before the element body runs,
// so a REPL render fails without evaluating anything.
pub fn render_element(
    element: &Element<'_>,
    env: &mut Environment,
    line: usize,
) -> Result<(), RuntimeError> {
    let source = env
        .source_path()
        .map(Path::to_path_buf)
        .ok_or_else(|| RuntimeError::new(line, "render() requires running a .fmyr file."))?;

    let html = generate(element, env)?;
    write_document(&html, &source, line)?;
    Ok(())
}
