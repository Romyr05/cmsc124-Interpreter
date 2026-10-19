use crate::ast::Element;

// Generate the HTML document for an element AST.
pub fn generate(_element: &Element<'_>) -> String {
    let html_start = "<!DOCTYPE html><html lang\"html\">";
    let html_end = "</html>";
    let head_start = "<head>";
    let head_end = "</head>";
    let body_start = "<body>";
    let body_end = "</body>";

    format!(
        "{}{}{}{}{}{}",
        html_start, head_start, head_end, body_start, body_end, html_end
    )
}
