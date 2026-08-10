//! Recognizes NME statements among ordinary Python logical lines.
//!
//! ## The golden rule: Python wins
//!
//! A line that is valid Python is *always* treated as Python, even when it
//! looks like NME. NME only claims lines that Python itself would reject.
//! This is what lets Python and NME share one file safely:
//!
//! * `say("hi")` — valid Python (a function call) → left untouched
//! * `say "hi"` — not valid Python → NME `say`
//! * `if times:` — valid Python header → left untouched
//! * `5 times:` — not valid Python → NME `times` loop
//!
//! Because validity is decided by a real Python parser ([`rustpython_parser`],
//! the same engine that powers RustPython), this rule is exact — not a
//! heuristic, and not a regex.
//!
//! ## Adding a new NME construct
//!
//! 1. Add a variant to [`crate::syntax::NmeStmt`].
//! 2. Add a `match_*` function here and call it from [`classify`].
//! 3. Add lowering in [`crate::lower`].
//! 4. Add tests (including proof that look-alike valid Python is untouched).
//!
//! Matchers only ever inspect *tokens*, never raw text, so strings and
//! comments can never trigger a construct by accident.

use rustpython_parser::{parse as parse_python, Mode, Tok};

use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::{LogicalLine, Token};
use crate::syntax::{InlineStmt, NmeLine, NmeStmt, SAY_KEYWORD, TIMES_KEYWORD};

/// Parses every logical line, collecting the NME statements it finds.
///
/// Returns *all* problems found (not just the first) so beginners can fix
/// everything in one go.
pub fn parse(source: &str, lines: &[LogicalLine]) -> Result<Vec<NmeLine>, Vec<Diagnostic>> {
    let mut found = Vec::new();
    let mut problems = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let next_indent = lines.get(index + 1).map(|next| next.indent);
        match classify(
            source,
            &line.tokens,
            &BlockCtx::TopLevel { line, next_indent },
        ) {
            Ok(Some(stmt)) => found.push(NmeLine {
                span: line.span,
                stmt,
            }),
            Ok(None) => {} // ordinary Python line: no edit, byte-identical output
            Err(problem) => problems.push(problem),
        }
    }

    if problems.is_empty() {
        Ok(found)
    } else {
        Err(problems)
    }
}

/// Where a statement appears; decides whether a block-form `times:` may
/// follow. (An inline body after `:` cannot open an indented block.)
enum BlockCtx<'a> {
    TopLevel {
        line: &'a LogicalLine,
        next_indent: Option<usize>,
    },
    Inline,
}

/// Classifies the tokens of one logical line.
///
/// `text` is the exact source text covered by `tokens`. Returns `Ok(None)`
/// for ordinary Python (passthrough), `Ok(Some(..))` for NME, and
/// `Err(..)` for lines that are neither valid Python nor valid NME — those
/// get a friendly explanation instead of silently broken output.
fn classify(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    debug_assert!(!tokens.is_empty());

    // NME statements never start with a Python keyword. This single check
    // keeps every Python compound statement (`if times:`, `case times:`,
    // `while times:`, ...) out of NME's way by construction. (Literals,
    // names and brackets *can* start an NME statement: `5 times:`,
    // `(2 + 3) times:`, `say "hi"`.)
    if is_python_keyword(&tokens[0].tok) {
        return Ok(None);
    }

    let text = token_text(source, tokens);

    if let Some(stmt) = match_say(source, tokens, text)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_times(source, tokens, text, block)? {
        return Ok(Some(stmt));
    }
    Ok(None)
}

/// Matches `say <expr>`.
fn match_say(source: &str, tokens: &[Token], text: &str) -> Result<Option<NmeStmt>, Diagnostic> {
    let first = match &tokens[0].tok {
        Tok::Name { name } if name == SAY_KEYWORD => &tokens[0],
        _ => return Ok(None),
    };
    if tokens.len() == 1 {
        // A bare `say` is a valid Python name expression; Python wins.
        // (Beginners who write it get Python's own NameError at runtime.)
        return Ok(None);
    }
    if is_valid_python_statement(text) {
        return Ok(None); // e.g. `say(x)`, `say.x`, `say[0]`: Python wins.
    }

    let expr = Span::new(tokens[1].span.start, tokens[tokens.len() - 1].span.end);
    if !is_valid_python_expression(&source[expr.start..expr.end]) {
        return Err(
            Diagnostic::new("I couldn't understand what you want to `say`", expr)
                .with_hint("after `say`, write any value, like `say \"Hello\"` or `say 1 + 1`"),
        );
    }
    let _ = first;
    Ok(Some(NmeStmt::Say { expr }))
}

/// Matches `<expr> times:` (block form) and `<expr> times: <stmt>` (inline).
fn match_times(
    source: &str,
    tokens: &[Token],
    text: &str,
    block: &BlockCtx<'_>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // Find the first `times :` pair at bracket depth 0 with an expression
    // before it. Colons inside brackets, slices, dicts or lambdas never sit
    // at depth 0 *after* a name `times` in valid Python — and if one ever
    // does, the Python-wins check below still protects it.
    let Some(times_at) = find_times_colon(tokens) else {
        return Ok(None);
    };
    if is_valid_python_statement(text) || is_valid_python_header(text) {
        return Ok(None); // Python wins.
    }

    let count = Span::new(tokens[0].span.start, tokens[times_at - 1].span.end);
    if !is_valid_python_expression(&source[count.start..count.end]) {
        return Err(
            Diagnostic::new("I couldn't understand how many times to repeat", count)
                .with_hint("before `times:`, write a number or any value, like `5 times:`"),
        );
    }

    let colon_at = times_at + 1;
    if colon_at + 1 == tokens.len() {
        // Block form: `5 times:` — the body is the following indented lines.
        return match block {
            BlockCtx::TopLevel { line, next_indent } => {
                if next_indent.is_some_and(|next| next > line.indent) {
                    Ok(Some(NmeStmt::Times {
                        count,
                        inline: None,
                    }))
                } else {
                    Err(Diagnostic::new(
                        "after `times:` the lines you want to repeat must be indented",
                        line.span,
                    )
                    .with_hint("indent them, like this:\n\n    5 times:\n        say \"Hello\""))
                }
            }
            BlockCtx::Inline => Err(Diagnostic::new(
                "a repeated block can't start at the end of a line like this",
                Span::new(tokens[times_at].span.start, tokens[colon_at].span.end),
            )
            .with_hint("put the repeated lines on the next line, indented")),
        };
    }

    // Inline form: `5 times: <one statement>`.
    let body = &tokens[colon_at + 1..];
    if has_top_level_semicolon(body) {
        return Err(Diagnostic::new(
            "only one statement can follow `:`",
            Span::new(body[0].span.start, body[body.len() - 1].span.end),
        )
        .with_hint("put each statement on its own indented line"));
    }
    let body_span = Span::new(body[0].span.start, body[body.len() - 1].span.end);
    let inline = if let Some(inner) = classify(source, body, &BlockCtx::Inline)? {
        InlineStmt::Nme(Box::new(inner))
    } else {
        let body_text = &source[body_span.start..body_span.end];
        if !is_valid_python_statement(body_text) {
            return Err(Diagnostic::new(
                "I couldn't understand the statement after `:`",
                body_span,
            )
            .with_hint("after `:`, write one statement, like `5 times: say \"Hello\""));
        }
        InlineStmt::Python(body_span)
    };
    Ok(Some(NmeStmt::Times {
        count,
        inline: Some(inline),
    }))
}

/// Is this token one of Python's keywords? NME statements never start with
/// one, so keyword-led lines are always treated as Python.
fn is_python_keyword(tok: &Tok) -> bool {
    matches!(
        tok,
        Tok::False
            | Tok::None
            | Tok::True
            | Tok::And
            | Tok::As
            | Tok::Assert
            | Tok::Async
            | Tok::Await
            | Tok::Break
            | Tok::Case
            | Tok::Class
            | Tok::Continue
            | Tok::Def
            | Tok::Del
            | Tok::Elif
            | Tok::Else
            | Tok::Except
            | Tok::Finally
            | Tok::For
            | Tok::From
            | Tok::Global
            | Tok::If
            | Tok::Import
            | Tok::In
            | Tok::Is
            | Tok::Lambda
            | Tok::Match
            | Tok::Nonlocal
            | Tok::Not
            | Tok::Or
            | Tok::Pass
            | Tok::Raise
            | Tok::Return
            | Tok::Try
            | Tok::Type
            | Tok::While
            | Tok::With
            | Tok::Yield
    )
}

/// Finds the first `Name("times")` directly followed by `:` at bracket
/// depth 0, with at least one token before it. Returns its token index.
fn find_times_colon(tokens: &[Token]) -> Option<usize> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Name { name } if name == TIMES_KEYWORD && depth == 0 && index > 0 => {
                if matches!(
                    tokens.get(index + 1),
                    Some(Token {
                        tok: Tok::Colon,
                        ..
                    })
                ) {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// Is there a `;` at bracket depth 0 in these tokens?
fn has_top_level_semicolon(tokens: &[Token]) -> bool {
    let mut depth = 0usize;
    for token in tokens {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Semi if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

/// The exact source text covered by a token slice.
fn token_text<'a>(source: &'a str, tokens: &[Token]) -> &'a str {
    &source[tokens[0].span.start..tokens[tokens.len() - 1].span.end]
}

/// Python-wins check for simple statements and complete lines.
fn is_valid_python_statement(text: &str) -> bool {
    parse_python(text, Mode::Module, "<nme>").is_ok()
}

/// Python-wins check for compound-statement headers (`if times:` and
/// friends), which only parse when followed by a body.
fn is_valid_python_header(text: &str) -> bool {
    parse_python(&format!("{text}\n    pass"), Mode::Module, "<nme>").is_ok()
}

/// NME never parses expressions itself; it only asks Python whether the
/// text is a valid expression and then copies it verbatim.
fn is_valid_python_expression(text: &str) -> bool {
    parse_python(text, Mode::Expression, "<nme>").is_ok()
}
