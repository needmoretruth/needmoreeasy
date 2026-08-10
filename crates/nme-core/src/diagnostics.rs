//! Beginner-friendly diagnostics.
//!
//! NME's audience is people who find Python intimidating, so an error
//! message must answer three questions in plain language:
//!
//! 1. *What* is wrong?  (`message`)
//! 2. *Where* exactly?  (`span`, rendered as a caret under the source line)
//! 3. *What should I try instead?*  (`hint`)
//!
//! Diagnostics are plain data. Rendering is separated from reporting so the
//! CLI (and future tools, e.g. an LSP server) can present them differently
//! without touching the compiler.

use std::fmt::Write as _;

/// A byte range in the original source text (`start..end`, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn len(self) -> usize {
        self.end - self.start
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// One problem found in NME source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// What is wrong, in plain language. No compiler jargon.
    pub message: String,
    /// Where it is wrong.
    pub span: Span,
    /// What to try instead, if we know.
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
            hint: None,
        }
    }

    #[must_use]
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// 1-based `(line, column)` of the start of the span in `source`.
    pub fn line_col(&self, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (offset, ch) in source.char_indices() {
            if offset >= self.span.start {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    /// Renders the diagnostic in a friendly, rustc-inspired format:
    ///
    /// ```text
    /// error: `say` needs something to print
    ///  --> hello.nme:2:1
    ///   |
    /// 2 | say
    ///   | ^^^
    ///   = hint: try `say "Hello"`
    /// ```
    pub fn render(&self, source: &str, path: &str) -> String {
        let (line_no, col) = self.line_col(source);
        let line_text = source_line(source, line_no);
        // The underline covers the span, but never spills past the line and
        // is always at least one caret wide so zero-width spans are visible.
        let line_start = self.span.start - (col - 1);
        let underline_len = (self.span.end.min(line_start + line_text.len()))
            .saturating_sub(self.span.start)
            .max(1);

        let mut out = String::new();
        let gutter = line_no.to_string().len();
        let _ = writeln!(out, "error: {}", self.message);
        let _ = writeln!(out, "{:>gutter$} --> {path}:{line_no}:{col}", "");
        let _ = writeln!(out, "{:>gutter$} |", "");
        let _ = writeln!(out, "{line_no:>gutter$} | {line_text}");
        let _ = writeln!(
            out,
            "{:>gutter$} | {:width$}{}",
            "",
            "",
            "^".repeat(underline_len),
            width = col - 1
        );
        if let Some(hint) = &self.hint {
            let _ = writeln!(out, "{:>gutter$} = hint: {hint}", "");
        }
        out
    }
}

/// Renders several diagnostics, separated by blank lines.
pub fn render_all(diagnostics: &[Diagnostic], source: &str, path: &str) -> String {
    diagnostics
        .iter()
        .map(|d| d.render(source, path))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns physical line `line_no` (1-based) without the trailing newline,
/// or an empty string when the line does not exist.
fn source_line(source: &str, line_no: usize) -> &str {
    source
        .lines()
        .nth(line_no - 1)
        .unwrap_or("")
        .trim_end_matches('\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_line_column_and_caret() {
        let source = "say \"hi\"\nsay\n";
        let diag = Diagnostic::new("`say` needs something to print", Span::new(9, 12))
            .with_hint("try `say \"Hello\"`");
        let rendered = diag.render(source, "hello.nme");
        assert!(rendered.contains("error: `say` needs something to print"));
        assert!(rendered.contains("hello.nme:2:1"));
        assert!(rendered.contains("2 | say"));
        assert!(rendered.contains("^^^"));
        assert!(rendered.contains("hint: try `say \"Hello\"`"));
    }

    #[test]
    fn line_col_counts_from_one() {
        let source = "ab\ncd\nef";
        let diag = Diagnostic::new("x", Span::new(6, 8));
        assert_eq!(diag.line_col(source), (3, 1));
    }
}
