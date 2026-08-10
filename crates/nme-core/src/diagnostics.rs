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

use unicode_width::UnicodeWidthChar;

const TAB_WIDTH: usize = 4;

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
        let rendered_line = expand_tabs(line_text);
        // The underline covers the span, but never spills past the line and
        // is always at least one caret wide so zero-width spans are visible.
        let line_start = source_line_start(source, line_no);
        let line_end = line_start + line_text.len();
        let span_start = self.span.start.clamp(line_start, line_end);
        let span_end = self.span.end.clamp(span_start, line_end);
        let local_start = floor_char_boundary(line_text, span_start - line_start);
        let raw_end = span_end - line_start;
        let local_end = if raw_end == span_start - line_start {
            local_start
        } else {
            ceil_char_boundary(line_text, raw_end)
        };
        let prefix = &line_text[..local_start];
        let highlighted = &line_text[local_start..local_end];
        let underline_start = display_width(prefix, 0);
        let underline_len = display_width(highlighted, underline_start).max(1);

        let mut out = String::new();
        let gutter = line_no.to_string().len();
        let _ = writeln!(out, "error: {}", self.message);
        let _ = writeln!(out, "{:>gutter$} --> {path}:{line_no}:{col}", "");
        let _ = writeln!(out, "{:>gutter$} |", "");
        let _ = writeln!(out, "{line_no:>gutter$} | {rendered_line}");
        let _ = writeln!(
            out,
            "{:>gutter$} | {:width$}{}",
            "",
            "",
            "^".repeat(underline_len),
            width = underline_start
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

fn source_line_start(source: &str, line_no: usize) -> usize {
    if line_no <= 1 {
        return 0;
    }
    source
        .char_indices()
        .filter_map(|(offset, character)| (character == '\n').then_some(offset + 1))
        .nth(line_no - 2)
        .unwrap_or(source.len())
}

fn floor_char_boundary(text: &str, mut offset: usize) -> usize {
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn ceil_char_boundary(text: &str, mut offset: usize) -> usize {
    while offset < text.len() && !text.is_char_boundary(offset) {
        offset += 1;
    }
    offset
}

fn display_width(text: &str, starting_column: usize) -> usize {
    let mut column = starting_column;
    for character in text.chars() {
        column += if character == '\t' {
            TAB_WIDTH - (column % TAB_WIDTH)
        } else {
            character.width().unwrap_or(0)
        };
    }
    column - starting_column
}

fn expand_tabs(text: &str) -> String {
    let mut expanded = String::with_capacity(text.len());
    let mut column = 0;
    for character in text.chars() {
        if character == '\t' {
            let spaces = TAB_WIDTH - (column % TAB_WIDTH);
            expanded.push_str(&" ".repeat(spaces));
            column += spaces;
        } else {
            expanded.push(character);
            column += character.width().unwrap_or(0);
        }
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn underline(rendered: &str) -> &str {
        rendered
            .lines()
            .find(|line| line.contains('^'))
            .and_then(|line| line.split_once("| "))
            .map(|(_, underline)| underline)
            .expect("rendered diagnostic should contain an underline")
    }

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

    #[test]
    fn ascii_caret_uses_character_width() {
        let source = "show value\n";
        let start = source.find("value").unwrap();
        let rendered = Diagnostic::new("x", Span::new(start, start + "value".len()))
            .render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^^^^^");
    }

    #[test]
    fn cjk_caret_uses_display_cell_width() {
        let source = "말해 잘못된값\n";
        let start = source.find("잘못된값").unwrap();
        let rendered = Diagnostic::new("x", Span::new(start, start + "잘못된값".len()))
            .render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^^^^^^^^");
    }

    #[test]
    fn tabs_are_expanded_to_four_column_stops() {
        let source = "a\tbroken\n";
        let start = source.find("broken").unwrap();
        let rendered = Diagnostic::new("x", Span::new(start, start + "broken".len()))
            .render(source, "hello.nme");

        assert!(rendered.contains("1 | a   broken"));
        assert_eq!(underline(&rendered), "    ^^^^^^");
    }

    #[test]
    fn tab_inside_span_uses_its_expanded_width() {
        let source = "a\tb\n";
        let rendered = Diagnostic::new("x", Span::new(1, 3)).render(source, "hello.nme");

        assert_eq!(underline(&rendered), " ^^^^");
    }

    #[test]
    fn zero_width_span_has_one_caret() {
        let source = "say\n";
        let rendered = Diagnostic::new("x", Span::new(3, 3)).render(source, "hello.nme");

        assert_eq!(underline(&rendered), "   ^");
    }

    #[test]
    fn partial_unicode_byte_span_covers_the_whole_character() {
        let source = "show …\n";
        let start = source.find('…').unwrap();
        let rendered =
            Diagnostic::new("x", Span::new(start, start + 1)).render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^");
    }
}
