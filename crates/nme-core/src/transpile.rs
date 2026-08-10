//! The public entry point: NME source in, Python source out.
//!
//! [`transpile`] simply wires the pipeline together:
//! [`lexer`] → [`parser`] → [`lower`]. Each stage has exactly one job and
//! one reason to change; keep it that way.

use crate::diagnostics::{Diagnostic, Span};
use crate::{lexer, lower, parser};

/// Transpiles NME source code into ordinary Python source code.
///
/// * Pure Python input comes out **byte-identical**.
/// * NME statements are replaced by Python on the same line, so line
///   numbers never shift and Python tracebacks stay meaningful.
/// * On failure, returns *all* problems found, ready to render with
///   [`crate::diagnostics::render_all`].
pub fn transpile(source: &str) -> Result<String, Vec<Diagnostic>> {
    let lines = lexer::logical_lines(source).map_err(|problem| vec![problem])?;
    let nme_lines = parser::parse(source, &lines)?;
    let edits = lower::lower_lines(&nme_lines, source);
    let line_break_problems = edits
        .iter()
        .filter(|edit| {
            count_line_breaks(&source[edit.span.start..edit.span.end])
                != count_line_breaks(&edit.replacement)
        })
        .map(|edit| {
            Diagnostic::new(
                "sentence-style NME must stay on one physical line",
                Span::new(edit.span.start, edit.span.end),
            )
            .with_hint(
                "keep this easy statement on one line; multiline Python expressions remain supported",
            )
        })
        .collect::<Vec<_>>();
    if !line_break_problems.is_empty() {
        return Err(line_break_problems);
    }
    Ok(lower::apply_edits(source, &edits))
}

fn count_line_breaks(text: &str) -> usize {
    text.bytes().filter(|byte| *byte == b'\n').count()
}
