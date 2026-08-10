//! Splits NME source into *logical lines* using Python's own token rules.
//!
//! This module deliberately does **not** scan text itself. It delegates to
//! the battle-tested [`rustpython_parser`] lexer, so strings, comments,
//! triple-quoted strings, f-strings, escapes and line continuations are
//! recognized exactly the way CPython recognizes them. That is what
//! guarantees NME can never mistake text inside a string or comment for
//! code — the tokenizer hands us whole, unambiguous tokens with byte spans.
//!
//! A *logical line* is one Python statement line: the tokens between two
//! `Newline` tokens. A logical line may span several physical lines (inside
//! brackets or after a backslash), which keeps multi-line Python expressions
//! intact and invisible to NME's line-oriented grammar.

use rustpython_parser::lexer::lex;
use rustpython_parser::{Mode, Tok};

use crate::diagnostics::{Diagnostic, Span};

/// One Python token together with its byte span in the source.
#[derive(Debug, Clone)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
}

/// A logical line of source: one statement's worth of tokens.
#[derive(Debug)]
pub struct LogicalLine {
    /// 1-based number of the physical line the statement starts on.
    pub number: usize,
    /// Indentation depth: how many `Indent` levels deep this line is.
    pub indent: usize,
    /// Significant tokens of the statement (no `Newline`/`Indent`/`Dedent`).
    pub tokens: Vec<Token>,
    /// Byte span from the first token's start to the last token's end.
    /// Leading indentation and trailing comments are *outside* this span,
    /// so replacing it never disturbs comments or whitespace.
    pub span: Span,
}

impl LogicalLine {
    /// Convenience for pattern matching: is the first significant token a
    /// name with the given value (e.g. `say`)?
    pub fn starts_with_name(&self, name: &str) -> bool {
        matches!(
            self.tokens.first(),
            Some(Token {
                tok: Tok::Name { name: n },
                ..
            }) if n == name
        )
    }

    /// The source text covered by this line's token span.
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }
}

/// Tokenizes `source` and groups the tokens into logical lines.
///
/// Returns a single [`Diagnostic`] when Python's lexer itself rejects the
/// source (for example an unterminated string). Such input cannot be
/// Python and cannot be NME, so one clear error is all we can offer.
pub fn logical_lines(source: &str) -> Result<Vec<LogicalLine>, Diagnostic> {
    let line_starts = line_start_offsets(source);
    let mut lines = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    let mut indent: usize = 0;

    for result in lex(source, Mode::Module) {
        let (tok, range) = result.map_err(|err| lexical_diagnostic(source, &err))?;
        let span = Span::new(usize::from(range.start()), usize::from(range.end()));
        match tok {
            // Framing tokens carry no meaning for us.
            Tok::StartModule | Tok::EndOfFile => {}
            Tok::Indent => indent += 1,
            Tok::Dedent => indent = indent.saturating_sub(1),
            Tok::Newline => {
                if !current.is_empty() {
                    lines.push(finish_line(source, &line_starts, indent, &mut current));
                }
            }
            _ => current.push(Token { tok, span }),
        }
    }
    // Defensive: a well-formed token stream always ends with Newline, but if
    // the lexer ever changes we still keep the trailing tokens.
    if !current.is_empty() {
        lines.push(finish_line(source, &line_starts, indent, &mut current));
    }
    Ok(lines)
}

fn finish_line(
    source: &str,
    line_starts: &[usize],
    indent: usize,
    tokens: &mut Vec<Token>,
) -> LogicalLine {
    debug_assert!(!tokens.is_empty());
    let first = tokens.first().map_or(0, |t| t.span.start);
    let last = tokens.last().map_or(0, |t| t.span.end);
    let number = line_number(line_starts, first);
    debug_assert!(last <= source.len());
    LogicalLine {
        number,
        indent,
        tokens: std::mem::take(tokens),
        span: Span::new(first, last),
    }
}

/// Byte offsets at which each physical line starts (offset 0 included).
fn line_start_offsets(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (offset, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(offset + 1);
        }
    }
    starts
}

/// 1-based physical line number containing byte `offset`.
fn line_number(line_starts: &[usize], offset: usize) -> usize {
    match line_starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index,
    }
}

fn lexical_diagnostic(source: &str, err: &rustpython_parser::lexer::LexicalError) -> Diagnostic {
    let offset = usize::from(err.location);
    // The lexer points at the offending character; underline it (or the
    // final byte when the error is at end of input).
    let start = offset.min(source.len().saturating_sub(1));
    Diagnostic::new(
        format!(
            "this is not something Python or NME can read: {}",
            err.error
        ),
        Span::new(start, start + 1),
    )
    .with_hint("check for an unterminated string or a stray character")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(source: &str) -> Vec<LogicalLine> {
        logical_lines(source).expect("test sources must lex")
    }

    #[test]
    fn groups_tokens_per_logical_line() {
        let src = "x = 1\ny = 2\n";
        let ls = lines(src);
        assert_eq!(ls.len(), 2);
        assert_eq!(ls[0].number, 1);
        assert_eq!(ls[0].tokens.len(), 3); // x, =, 1
        assert_eq!(ls[1].number, 2);
        assert_eq!(ls[0].text(src), "x = 1");
    }

    #[test]
    fn tracks_indent_depth() {
        let src = "for i in range(3):\n    print(i)\nprint('done')\n";
        let ls = lines(src);
        assert_eq!(ls.len(), 3);
        assert_eq!(ls[0].indent, 0);
        assert_eq!(ls[1].indent, 1);
        assert_eq!(ls[2].indent, 0);
    }

    #[test]
    fn comment_and_blank_lines_produce_no_logical_lines() {
        let src = "# 5 times: say \"hi\"\n\nx = 1  # trailing comment\n";
        let ls = lines(src);
        assert_eq!(ls.len(), 1);
        assert_eq!(ls[0].number, 3);
        // The span excludes the trailing comment, so edits keep it intact.
        assert_eq!(ls[0].text(src), "x = 1");
    }

    #[test]
    fn triple_quoted_strings_stay_inside_one_logical_line() {
        let src = "text = \"\"\"\n5 times: say \"not code\"\n\"\"\"\nprint(text)\n";
        let ls = lines(src);
        assert_eq!(ls.len(), 2);
        assert_eq!(ls[0].number, 1);
        assert_eq!(ls[1].number, 4);
    }

    #[test]
    fn bracket_continuations_stay_in_one_logical_line() {
        let src = "total = (1 +\n         2)\n";
        let ls = lines(src);
        assert_eq!(ls.len(), 1);
        assert_eq!(ls[0].text(src), "total = (1 +\n         2)");
    }

    #[test]
    fn f_strings_are_single_tokens() {
        let src = "name = \"n\"\nsay_text = f\"hi {name}\"\n";
        let ls = lines(src);
        assert!(matches!(ls[1].tokens[2].tok, Tok::String { .. }));
    }

    #[test]
    fn unterminated_string_is_a_friendly_error() {
        let err = logical_lines("say \"oops\n").unwrap_err();
        assert!(err.message.contains("not something Python or NME can read"));
    }
}
