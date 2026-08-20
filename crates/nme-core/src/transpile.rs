//! The public entry point: NME source in, Python source out.
//!
//! [`transpile`] simply wires the pipeline together:
//! [`lexer`] → [`parser`] → [`lower`]. Each stage has exactly one job and
//! one reason to change; keep it that way.

use crate::diagnostics::{Diagnostic, DiagnosticCode, Span};
use crate::syntax::{Code, NmeStmt};
use crate::{lexer, lower, parser};

/// A `.nme` module import discovered while transpiling: the other file's
/// path and the explicit names that form its interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleImport {
    /// Path as written, quotes stripped (for example `helper.nme`).
    pub file: String,
    /// The names this file imports from the module.
    pub names: Vec<String>,
}

/// Transpiles NME source into ordinary Python source code.
///
/// * Pure Python input comes out **byte-identical**.
/// * NME statements are replaced by Python on the same line, so line
///   numbers never shift and Python tracebacks stay meaningful.
/// * On failure, returns *all* problems found, ready to render with
///   [`crate::diagnostics::render_all`].
pub fn transpile(source: &str) -> Result<String, Vec<Diagnostic>> {
    transpile_with_modules(source).map(|(python, _)| python)
}

/// Like [`transpile`], but also reports every `from "module.nme" import ...`
/// so the CLI can transpile those modules and make them importable at
/// runtime.
pub fn transpile_with_modules(
    source: &str,
) -> Result<(String, Vec<ModuleImport>), Vec<Diagnostic>> {
    let lines = lexer::logical_lines(source).map_err(|problem| vec![problem])?;
    let program = parser::parse_program(source, &lines)?;
    let nme_lines = &program.nme_lines;
    let imports = nme_lines
        .iter()
        .filter_map(|line| match &line.stmt {
            NmeStmt::ModuleImport { path, names } => {
                let Code::Source(span) = path else {
                    // A module path is always copied from the source; the
                    // compiler never writes one itself.
                    return None;
                };
                let span = *span;
                let raw = &source[span.start..span.end];
                Some(ModuleImport {
                    file: raw.trim_matches(['\'', '"']).to_string(),
                    names: names.clone(),
                })
            }
            _ => None,
        })
        .collect();
    let mut edits = lower::lower_lines(nme_lines, source);
    // A blank line inside a story block prints an empty line. It has no
    // tokens, so the parser reports its exact place instead of a statement.
    edits.extend(
        program
            .story_blank_lines
            .iter()
            .map(|(span, replacement)| lower::Edit {
                span: *span,
                replacement: replacement.clone(),
            }),
    );
    let nme_indexes = nme_lines
        .iter()
        .map(|line| line.line_index)
        .collect::<std::collections::HashSet<_>>();
    for (index, line) in lines.iter().enumerate() {
        let level = program.virtual_indents[index];
        if level == 0 || nme_indexes.contains(&index) {
            continue;
        }
        edits.push(lower::Edit {
            span: Span::new(line.span.start, line.span.start),
            replacement: "    ".repeat(level),
        });
    }
    // One NME statement is one physical Python line — the invariant the whole
    // pairing of editor lines and Python lines rests on. No lowering breaks it
    // today, so this is a guard rather than a message a reader is meant to
    // meet: it turns a silent line-numbering drift into a named refusal.
    let line_break_problems = edits
        .iter()
        .filter(|edit| {
            count_line_breaks(&source[edit.span.start..edit.span.end])
                != count_line_breaks(&edit.replacement)
        })
        .map(|edit| {
            Diagnostic::bilingual(
                DiagnosticCode::MultilineSentence,
                "sentence-style NME must stay on one physical line",
                "문장형 NME 한 문장은 실제 한 줄 안에 써야 합니다",
                Span::new(edit.span.start, edit.span.end),
            )
            .with_bilingual_hint(
                "keep this easy statement on one line; multiline Python expressions remain supported",
                "이 쉬운 문장은 한 줄에 쓰세요. 여러 줄 Python 표현식은 그대로 지원합니다",
            )
        })
        .collect::<Vec<_>>();
    if !line_break_problems.is_empty() {
        return Err(line_break_problems);
    }
    Ok((
        without_the_left_margin(&lower::apply_edits(source, &edits)),
        imports,
    ))
}

/// The program with the space every line of it starts with taken off.
///
/// A block of text copied out of a page, and a program typed into an editor
/// that was already one step in, both arrive with every line indented. The
/// parser reads such a file — the first line has nothing above it to be
/// indented under — but the Python it lowers to keeps the same left margin,
/// and CPython refuses an indented first statement. Only whitespace shared by
/// every line that has anything on it is taken, so nothing inside the program
/// moves relative to anything else.
///
/// A triple-quoted string may carry its own indentation as data, so a program
/// holding one is left exactly as it is.
fn without_the_left_margin(python: &str) -> String {
    if !python.starts_with([' ', '\t']) || python.contains(r#"""""#) || python.contains("'''") {
        return python.to_string();
    }
    let Some(margin) = python
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| &line[..line.len() - line.trim_start().len()])
        .reduce(|left, right| {
            let shared = left
                .char_indices()
                .zip(right.chars())
                .take_while(|((_, mine), theirs)| mine == theirs)
                .last()
                .map_or(0, |((at, mine), _)| at + mine.len_utf8());
            &left[..shared]
        })
        .filter(|margin| !margin.is_empty())
    else {
        return python.to_string();
    };
    let body = python
        .lines()
        .map(|line| line.strip_prefix(margin).unwrap_or(line.trim_start()))
        .collect::<Vec<_>>()
        .join("\n");
    if python.ends_with('\n') {
        body + "\n"
    } else {
        body
    }
}

fn count_line_breaks(text: &str) -> usize {
    text.bytes().filter(|byte| *byte == b'\n').count()
}
