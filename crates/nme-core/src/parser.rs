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
use crate::syntax::{
    InlineStmt, NmeLine, NmeStmt, Spelling, ASK_KEYWORD, ASK_KEYWORD_KO, RANDOM_MODULE,
    RANDOM_MODULE_KO, SAY_KEYWORD, SAY_KEYWORD_KO, TIMES_KEYWORD, TIMES_KEYWORD_KO, USE_KEYWORD,
    USE_KEYWORD_KO, WHEN_KEYWORD, WHEN_KEYWORD_KO,
};

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

/// Where a statement appears; decides whether a block-form NME header may
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
    if is_valid_python_statement(text) || is_valid_python_header(text) {
        return Ok(None);
    }

    if let Some(stmt) = match_say(source, tokens)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_ask(source, tokens)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_when(source, tokens, block)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_times(source, tokens, block)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_use_random(tokens)? {
        return Ok(Some(stmt));
    }
    Ok(None)
}

/// Matches `say <expr>` and `말해 <expr>`.
fn match_say(source: &str, tokens: &[Token]) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(spelling) = name_spelling(&tokens[0].tok, SAY_KEYWORD, SAY_KEYWORD_KO) else {
        return Ok(None);
    };

    let expr = Span::new(tokens[1].span.start, tokens[tokens.len() - 1].span.end);
    if !is_valid_python_expression(&source[expr.start..expr.end]) {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("I couldn't understand what you want to `say`", expr)
                    .with_hint("after `say`, write any value, like `say \"Hello\"` or `say 1 + 1`")
            }
            Spelling::Korean => Diagnostic::new("`말해` 뒤의 값을 이해하지 못했어요", expr)
                .with_hint("`말해 \"안녕\"` 또는 `말해 1 + 1`처럼 값을 적어 보세요"),
        });
    }
    Ok(Some(NmeStmt::Say { expr }))
}

/// Matches `ask name[, prompt]` and `물어봐 이름[, 안내문]`.
fn match_ask(source: &str, tokens: &[Token]) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(spelling) = name_spelling(&tokens[0].tok, ASK_KEYWORD, ASK_KEYWORD_KO) else {
        return Ok(None);
    };

    let Some(target_token) = tokens.get(1) else {
        // A bare keyword is valid Python and returned before this matcher.
        return Ok(None);
    };
    if !matches!(target_token.tok, Tok::Name { .. }) {
        return Err(match spelling {
            Spelling::English => Diagnostic::new(
                "after `ask`, write the name that should hold the answer",
                target_token.span,
            )
            .with_hint("use a simple name, like `ask name, \"What is your name? \"`"),
            Spelling::Korean => Diagnostic::new(
                "`물어봐` 뒤에 대답을 담을 이름을 적어 주세요",
                target_token.span,
            )
            .with_hint("`물어봐 이름, \"이름이 뭐예요? \"`처럼 간단한 이름을 쓰세요"),
        });
    }
    let target = target_token.span;

    if tokens.len() == 2 {
        return Ok(Some(NmeStmt::Ask {
            target,
            prompt: None,
        }));
    }

    if !matches!(tokens[2].tok, Tok::Comma) {
        let span = Span::new(tokens[2].span.start, tokens[tokens.len() - 1].span.end);
        return Err(match spelling {
            Spelling::English => Diagnostic::new("put a comma before the question", span)
                .with_hint("write `ask name, \"What is your name? \"`"),
            Spelling::Korean => Diagnostic::new("질문 앞에 쉼표를 넣어 주세요", span)
                .with_hint("`물어봐 이름, \"이름이 뭐예요? \"`처럼 쓰세요"),
        });
    }

    let Some(first_prompt) = tokens.get(3) else {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("the question after the comma is missing", tokens[2].span)
                    .with_hint("add a value, like `ask name, \"What is your name? \"`")
            }
            Spelling::Korean => Diagnostic::new("쉼표 뒤의 질문이 비어 있어요", tokens[2].span)
                .with_hint("`물어봐 이름, \"이름이 뭐예요? \"`처럼 질문을 추가하세요"),
        });
    };
    let prompt = Span::new(first_prompt.span.start, tokens[tokens.len() - 1].span.end);
    if !is_valid_python_expression(&source[prompt.start..prompt.end]) {
        return Err(match spelling {
            Spelling::English => Diagnostic::new("I couldn't understand the question", prompt)
                .with_hint("write one value after the comma, usually a quoted sentence"),
            Spelling::Korean => Diagnostic::new("질문 내용을 이해하지 못했어요", prompt)
                .with_hint("쉼표 뒤에 따옴표로 감싼 문장 같은 값 하나를 적어 주세요"),
        });
    }

    Ok(Some(NmeStmt::Ask {
        target,
        prompt: Some(prompt),
    }))
}

/// Matches `when <expr>:` / `만약 <expr>:` in block and inline forms.
fn match_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(spelling) = name_spelling(&tokens[0].tok, WHEN_KEYWORD, WHEN_KEYWORD_KO) else {
        return Ok(None);
    };

    let Some(colon_at) = find_condition_colon(source, tokens) else {
        let span = Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end);
        return Err(match spelling {
            Spelling::English => Diagnostic::new("a `when` condition needs `:` at the end", span)
                .with_hint("write it like `when score > 10:`"),
            Spelling::Korean => Diagnostic::new("`만약` 조건 끝에는 `:`이 필요해요", span)
                .with_hint("`만약 점수 > 10:`처럼 쓰세요"),
        });
    };
    if colon_at == 1 {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("the `when` condition is missing", tokens[colon_at].span)
                    .with_hint("put a condition before `:`, like `when ready:`")
            }
            Spelling::Korean => {
                Diagnostic::new("`만약` 뒤의 조건이 비어 있어요", tokens[colon_at].span)
                    .with_hint("`만약 준비됨:`처럼 `:` 앞에 조건을 적으세요")
            }
        });
    }

    let condition = Span::new(tokens[1].span.start, tokens[colon_at - 1].span.end);
    if !is_valid_python_expression(&source[condition.start..condition.end]) {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("I couldn't understand the `when` condition", condition)
                    .with_hint("write any Python condition, like `when score >= 10:`")
            }
            Spelling::Korean => Diagnostic::new("`만약` 뒤의 조건을 이해하지 못했어요", condition)
                .with_hint("`만약 점수 >= 10:`처럼 조건을 적어 보세요"),
        });
    }

    let header_span = Span::new(tokens[0].span.start, tokens[colon_at].span.end);
    let inline = parse_suite(
        source,
        tokens,
        colon_at,
        block,
        SuiteKind::Condition(spelling),
        header_span,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

/// Matches `<expr> times:` / `<expr> 번:` in block and inline forms.
fn match_times(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // Find the first `times :` pair at bracket depth 0 with an expression
    // before it. Colons inside brackets, slices, dicts or lambdas never sit
    // at depth 0 *after* a name `times` in valid Python — and if one ever
    // does, the Python-wins check below still protects it.
    let Some((times_at, spelling)) = find_times_colon(tokens) else {
        return Ok(None);
    };

    let count = Span::new(tokens[0].span.start, tokens[times_at - 1].span.end);
    if !is_valid_python_expression(&source[count.start..count.end]) {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("I couldn't understand how many times to repeat", count)
                    .with_hint("before `times:`, write a number or any value, like `5 times:`")
            }
            Spelling::Korean => Diagnostic::new("몇 번 반복할지 이해하지 못했어요", count)
                .with_hint("`번:` 앞에 `5번:`처럼 횟수나 값을 적어 주세요"),
        });
    }

    let colon_at = times_at + 1;
    if colon_at + 1 == tokens.len() {
        return match block {
            BlockCtx::TopLevel { line, next_indent } => {
                if next_indent.is_some_and(|next| next > line.indent) {
                    Ok(Some(NmeStmt::Times {
                        count,
                        inline: None,
                    }))
                } else {
                    Err(indentation_diagnostic(
                        SuiteKind::Repeat(spelling),
                        line.span,
                    ))
                }
            }
            BlockCtx::Inline => Err(inline_block_diagnostic(
                SuiteKind::Repeat(spelling),
                Span::new(tokens[times_at].span.start, tokens[colon_at].span.end),
            )),
        };
    }

    // Inline form: `5 times: <one statement>`.
    let body = &tokens[colon_at + 1..];
    let body_span = Span::new(body[0].span.start, body[body.len() - 1].span.end);
    if has_top_level_semicolon(body) {
        return Err(one_statement_diagnostic(
            SuiteKind::Repeat(spelling),
            body_span,
        ));
    }
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

/// Matches the one deliberately bundled module spelling.
fn match_use_random(tokens: &[Token]) -> Result<Option<NmeStmt>, Diagnostic> {
    if name_is(&tokens[0].tok, USE_KEYWORD) {
        if tokens.len() == 2 && name_is(&tokens[1].tok, RANDOM_MODULE) {
            return Ok(Some(NmeStmt::UseRandom {
                spelling: Spelling::English,
            }));
        }
        let span = Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end);
        return Err(
            Diagnostic::new("NME only bundles `use random` for now", span).with_hint(
                "write `use random`, or use a normal Python import such as `import math`",
            ),
        );
    }

    if tokens.len() >= 2
        && name_is(&tokens[0].tok, RANDOM_MODULE_KO)
        && name_is(&tokens[1].tok, USE_KEYWORD_KO)
    {
        if tokens.len() == 2 {
            return Ok(Some(NmeStmt::UseRandom {
                spelling: Spelling::Korean,
            }));
        }
        let span = Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end);
        return Err(
            Diagnostic::new("`랜덤 사용` 뒤에는 다른 내용을 쓰지 않아요", span)
                .with_hint("한 줄에 `랜덤 사용`만 적어 주세요"),
        );
    }

    Ok(None)
}

#[derive(Clone, Copy)]
enum SuiteKind {
    Repeat(Spelling),
    Condition(Spelling),
}

/// Parses the block or one-statement body of a `when` header.
fn parse_suite(
    source: &str,
    tokens: &[Token],
    colon_at: usize,
    block: &BlockCtx<'_>,
    kind: SuiteKind,
    header_span: Span,
) -> Result<Option<InlineStmt>, Diagnostic> {
    if colon_at + 1 == tokens.len() {
        return match block {
            BlockCtx::TopLevel { line, next_indent } => {
                if next_indent.is_some_and(|next| next > line.indent) {
                    Ok(None)
                } else {
                    Err(indentation_diagnostic(kind, line.span))
                }
            }
            BlockCtx::Inline => Err(inline_block_diagnostic(kind, header_span)),
        };
    }

    let body = &tokens[colon_at + 1..];
    let body_span = Span::new(body[0].span.start, body[body.len() - 1].span.end);
    if has_top_level_semicolon(body) {
        return Err(one_statement_diagnostic(kind, body_span));
    }
    if let Some(inner) = classify(source, body, &BlockCtx::Inline)? {
        return Ok(Some(InlineStmt::Nme(Box::new(inner))));
    }
    if !is_valid_python_statement(&source[body_span.start..body_span.end]) {
        return Err(body_diagnostic(kind, body_span));
    }
    Ok(Some(InlineStmt::Python(body_span)))
}

fn indentation_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match kind {
        SuiteKind::Repeat(Spelling::English) => Diagnostic::new(
            "after `times:` the lines you want to repeat must be indented",
            span,
        )
        .with_hint("indent them, like this:\n\n    5 times:\n        say \"Hello\""),
        SuiteKind::Repeat(Spelling::Korean) => {
            Diagnostic::new("`번:` 다음에서 반복할 줄은 들여써야 해요", span)
                .with_hint("다음처럼 들여쓰세요:\n\n    5번:\n        말해 \"안녕\"")
        }
        SuiteKind::Condition(Spelling::English) => {
            Diagnostic::new("after `when ...:` the lines to run must be indented", span)
                .with_hint("indent them, like this:\n\n    when ready:\n        say \"Go!\"")
        }
        SuiteKind::Condition(Spelling::Korean) => {
            Diagnostic::new("`만약 ...:` 다음에서 실행할 줄은 들여써야 해요", span)
                .with_hint("다음처럼 들여쓰세요:\n\n    만약 준비됨:\n        말해 \"시작!\"")
        }
    }
}

fn inline_block_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match kind {
        SuiteKind::Repeat(Spelling::English) => Diagnostic::new(
            "a repeated block can't start at the end of a line like this",
            span,
        )
        .with_hint("put the repeated lines on the next line, indented"),
        SuiteKind::Repeat(Spelling::Korean) => {
            Diagnostic::new("반복 블록을 이 위치에서 새로 시작할 수 없어요", span)
                .with_hint("반복할 줄을 다음 줄에 들여써 주세요")
        }
        SuiteKind::Condition(Spelling::English) => Diagnostic::new(
            "a conditional block can't start at the end of a line like this",
            span,
        )
        .with_hint("put the conditional lines on the next line, indented"),
        SuiteKind::Condition(Spelling::Korean) => {
            Diagnostic::new("조건 블록을 이 위치에서 새로 시작할 수 없어요", span)
                .with_hint("조건에 따라 실행할 줄을 다음 줄에 들여써 주세요")
        }
    }
}

fn one_statement_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    let spelling = match kind {
        SuiteKind::Repeat(spelling) | SuiteKind::Condition(spelling) => spelling,
    };
    match spelling {
        Spelling::English => Diagnostic::new("only one statement can follow `:`", span)
            .with_hint("put each statement on its own indented line"),
        Spelling::Korean => Diagnostic::new("`:` 뒤에는 문장 하나만 쓸 수 있어요", span)
            .with_hint("각 문장을 다음 줄에 하나씩 들여써 주세요"),
    }
}

fn body_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    let spelling = match kind {
        SuiteKind::Repeat(spelling) | SuiteKind::Condition(spelling) => spelling,
    };
    match spelling {
        Spelling::English => Diagnostic::new("I couldn't understand the statement after `:`", span)
            .with_hint("after `:`, write one Python or NME statement"),
        Spelling::Korean => Diagnostic::new("`:` 뒤의 문장을 이해하지 못했어요", span)
            .with_hint("`:` 뒤에 Python 또는 NME 문장 하나를 적어 주세요"),
    }
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

/// Finds the first English or Korean repetition marker followed by `:`.
fn find_times_colon(tokens: &[Token]) -> Option<(usize, Spelling)> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Name { name }
                if (name == TIMES_KEYWORD || name == TIMES_KEYWORD_KO)
                    && depth == 0
                    && index > 0 =>
            {
                if matches!(
                    tokens.get(index + 1),
                    Some(Token {
                        tok: Tok::Colon,
                        ..
                    })
                ) {
                    let spelling = if name == TIMES_KEYWORD {
                        Spelling::English
                    } else {
                        Spelling::Korean
                    };
                    return Some((index, spelling));
                }
            }
            _ => {}
        }
    }
    None
}

/// Finds the header colon whose preceding tokens form a Python expression.
/// This handles lambda conditions without confusing the lambda's own colon
/// with the NME suite colon.
fn find_condition_colon(source: &str, tokens: &[Token]) -> Option<usize> {
    let mut depth = 0usize;
    let mut first = None;
    for (index, token) in tokens.iter().enumerate().skip(1) {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Colon if depth == 0 => {
                first.get_or_insert(index);
                if index > 1 {
                    let condition = Span::new(tokens[1].span.start, tokens[index - 1].span.end);
                    if is_valid_python_expression(&source[condition.start..condition.end]) {
                        return Some(index);
                    }
                }
            }
            _ => {}
        }
    }
    first
}

fn name_spelling(tok: &Tok, english: &str, korean: &str) -> Option<Spelling> {
    match tok {
        Tok::Name { name } if name == english => Some(Spelling::English),
        Tok::Name { name } if name == korean => Some(Spelling::Korean),
        _ => None,
    }
}

fn name_is(tok: &Tok, expected: &str) -> bool {
    matches!(tok, Tok::Name { name } if name == expected)
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
