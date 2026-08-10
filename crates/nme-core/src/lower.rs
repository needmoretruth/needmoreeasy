//! Lowers NME statements to ordinary Python source text.
//!
//! Lowering works by **edits**: each [`NmeLine`] knows the exact source span
//! it occupies, and lowering replaces just that span with Python. Everything
//! else — comments, blank lines, docstrings, pure-Python code — stays
//! byte-for-byte identical to what the user wrote.
//!
//! Every NME statement lowers to a single line of Python, so the output has
//! exactly as many lines as the input and Python tracebacks point at the
//! line numbers the user actually sees in their `.nme` file.

use crate::syntax::{InlineStmt, NmeLine, NmeStmt, Spelling};

const ENGLISH_RANDOM_TOOLS: &str = concat!(
    "import random; ",
    "random_number = random.randint; ",
    "random_pick = random.choice; ",
    "shuffle = random.shuffle",
);
const KOREAN_RANDOM_TOOLS: &str = concat!(
    "import random as 랜덤; ",
    "랜덤정수 = 랜덤.randint; ",
    "랜덤선택 = 랜덤.choice; ",
    "섞기 = 랜덤.shuffle",
);

/// A single source replacement: overwrite `span` with `replacement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    /// Byte span in the original source to replace.
    pub span: crate::diagnostics::Span,
    /// The Python text to put in its place.
    pub replacement: String,
}

/// Lowers each parsed NME statement to one edit, in source order.
pub fn lower_lines(lines: &[NmeLine], source: &str) -> Vec<Edit> {
    lines
        .iter()
        .map(|line| Edit {
            span: line.span,
            replacement: lower_stmt(&line.stmt, source),
        })
        .collect()
}

/// Lowers one NME statement to Python text (without its original indent —
/// the span being replaced never included it, so indentation is preserved
/// automatically).
pub fn lower_stmt(stmt: &NmeStmt, source: &str) -> String {
    match stmt {
        NmeStmt::Say { expr } => format!("print({})", slice(source, *expr)),
        NmeStmt::Ask { target, prompt } => match prompt {
            Some(prompt) => format!(
                "{} = input({})",
                slice(source, *target),
                slice(source, *prompt)
            ),
            None => format!("{} = input()", slice(source, *target)),
        },
        NmeStmt::Times { count, inline } => {
            let header = format!("for _ in range({}):", slice(source, *count));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::When { condition, inline } => {
            let header = format!("if ({}):", slice(source, *condition));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::UseRandom {
            spelling: Spelling::English,
        } => ENGLISH_RANDOM_TOOLS.to_string(),
        NmeStmt::UseRandom {
            spelling: Spelling::Korean,
        } => KOREAN_RANDOM_TOOLS.to_string(),
    }
}

fn lower_suite(header: String, inline: Option<&InlineStmt>, source: &str) -> String {
    match inline {
        None => header,
        Some(InlineStmt::Nme(inner)) => format!("{header} {}", lower_stmt(inner, source)),
        Some(InlineStmt::Python(span)) => format!("{header} {}", slice(source, *span)),
    }
}

/// Applies edits to `source`, returning the final Python program.
///
/// Edits must not overlap; the parser guarantees this because it produces
/// at most one edit per logical line.
pub fn apply_edits(source: &str, edits: &[Edit]) -> String {
    let mut sorted: Vec<&Edit> = edits.iter().collect();
    sorted.sort_by_key(|edit| edit.span.start);

    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;
    for edit in sorted {
        debug_assert!(edit.span.start >= cursor, "overlapping edits");
        debug_assert!(edit.span.end <= source.len());
        out.push_str(&source[cursor..edit.span.start]);
        out.push_str(&edit.replacement);
        cursor = edit.span.end;
    }
    out.push_str(&source[cursor..]);
    out
}

fn slice(source: &str, span: crate::diagnostics::Span) -> &str {
    &source[span.start..span.end]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::Span;

    #[test]
    fn applies_edits_without_touching_the_rest() {
        let source = "ab\ncd\nef\n";
        let edits = [Edit {
            span: Span::new(3, 5),
            replacement: "XY".to_string(),
        }];
        assert_eq!(apply_edits(source, &edits), "ab\nXY\nef\n");
    }

    #[test]
    fn lowers_say_and_times() {
        let source = "5 times: say \"hi\"";
        let stmt = NmeStmt::Times {
            count: Span::new(0, 1),
            inline: Some(InlineStmt::Nme(Box::new(NmeStmt::Say {
                expr: Span::new(13, 17),
            }))),
        };
        assert_eq!(
            lower_stmt(&stmt, source),
            "for _ in range(5): print(\"hi\")"
        );
    }
}
