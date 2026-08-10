//! Recognizes the advanced, beginner, and sentence levels of NME.
//!
//! A real Python parse always runs first. Valid Python therefore remains
//! byte-identical even when a Python name resembles an easier NME phrase.
//! Easier forms are matched only from lexer tokens; strings and comments are
//! never searched or rewritten as text.

use std::collections::HashSet;

use rustpython_parser::{parse as parse_python, Mode, Tok};

use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::{LogicalLine, Token};
use crate::syntax::{
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, Literal, LogicalOp,
    ModuleVersion, NmeLine, NmeStmt, Spelling, TextPart, TextTemplate, Value, RANDOM_MODULE,
    RANDOM_MODULE_KO, RANDOM_MODULE_VERSION, SAY_KEYWORD, SAY_KEYWORD_KO, SAY_WORDS_EN,
    TIMES_KEYWORD, TIMES_KEYWORD_KO,
};

const SAY_WORDS_KO: &[&str] = &[
    "말해",
    "말해줘",
    "말해주세요",
    "보여줘",
    "보여주세요",
    "출력해",
    "출력해줘",
    "출력해주세요",
];
const ASK_WORDS_EN: &[&str] = &["ask", "prompt", "question"];
const ASK_WORDS_KO: &[&str] = &[
    "물어봐",
    "물어봐줘",
    "물어보세요",
    "질문해",
    "질문해줘",
    "입력받아",
    "입력받아줘",
    "입력받아주세요",
    "물어봐요",
    "물어봐주세요",
    "질문해주세요",
];
const REPEAT_WORDS_EN: &[&str] = &["repeat", "again", "do"];
const REPEAT_WORDS_KO: &[&str] = &[
    "반복",
    "반복해",
    "반복해줘",
    "반복해주세요",
    "반복하세요",
    "반복해서",
    "반복하고",
    "반복한다음",
    "다시해",
    "다시해주세요",
];
const WHEN_WORDS_EN: &[&str] = &["when", "if"];
const WHEN_WORDS_KO: &[&str] = &["만약", "만약에", "만일", "혹시"];
const WHILE_WORDS_EN: &[&str] = &["while"];
const WHILE_WORDS_KO: &[&str] = &["동안", "하는동안", "할동안"];
const BREAK_WORDS_EN: &[&str] = &["break", "breakhere"];
const BREAK_WORDS_KO: &[&str] = &["멈춰", "멈춰라", "중단", "반복멈춰", "여기서멈춰"];
const ELSE_WORDS_EN: &[&str] = &["else", "otherwise"];
const ELSE_WORDS_KO: &[&str] = &["아니면", "그렇지않으면", "아니면만약", "그렇지않으면만약"];
const END_WORDS_EN: &[&str] = &["end"];
const END_WORDS_KO: &[&str] = &["끝"];
const USE_WORDS_EN: &[&str] = &["use", "load", "get", "import"];
const USE_WORDS_KO: &[&str] = &[
    "사용",
    "사용해",
    "사용해줘",
    "사용해주세요",
    "불러와",
    "불러와줘",
    "가져와",
    "가져와줘",
    "받아",
    "받아줘",
];
const LATEST_WORDS: &[&str] = &["latest", "newest", "최신", "최신판", "최신버전"];
const NUMBER_WORDS: &[&str] = &["number", "numeric", "숫자", "숫자로", "수로"];
const KOREAN_PARTICLES: &[&str] = &[
    "에게서는",
    "한테서는",
    "에게서",
    "한테서",
    "으로는",
    "로는",
    "에게",
    "한테",
    "에서",
    "으로",
    "까지",
    "부터",
    "처럼",
    "보다",
    "이라도",
    "라도",
    "에는",
    "에서",
    "은",
    "는",
    "이",
    "가",
    "을",
    "를",
    "와",
    "과",
    "도",
    "의",
    "에",
    "로",
    "아",
    "야",
    "랑",
    "이랑",
    "예요",
    "이에요",
];

const SET_WORDS_EN: &[&str] = &["set", "save", "remember"];
const SENTENCE_FILLERS: &[&str] = &["please", "좀", "혹시"];
const COMMAND_ENDINGS: &[&str] = &["?", "!"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum MatchMode {
    Exact,
    Recover,
}

/// The parser result also records virtual indentation for explicit sentence
/// blocks.  The transpiler uses that information to indent ordinary Python
/// lines mixed into a `while ... 끝` block without changing the source file.
#[derive(Debug, Clone)]
pub struct ParsedProgram {
    pub nme_lines: Vec<NmeLine>,
    pub virtual_indents: Vec<usize>,
}

/// Parse all logical lines, collecting independent beginner-facing errors.
pub fn parse(source: &str, lines: &[LogicalLine]) -> Result<Vec<NmeLine>, Vec<Diagnostic>> {
    parse_program(source, lines).map(|program| program.nme_lines)
}

/// Parse a complete program, including indentation-free blocks closed by
/// `end`/`끝`.  Existing indentation-based blocks remain supported, so users
/// can move one line at a time from sentence syntax to Python.
pub fn parse_program(
    source: &str,
    lines: &[LogicalLine],
) -> Result<ParsedProgram, Vec<Diagnostic>> {
    let mut found = Vec::new();
    let mut problems = Vec::new();
    let mut bindings = BindingEnv::new();
    let mut virtual_indents = vec![0; lines.len()];
    let mut blocks = Vec::<ExplicitBlock>::new();

    for (index, line) in lines.iter().enumerate() {
        let depth = blocks.len();
        let is_end = exact_end(line.tokens.as_slice());
        let is_break = exact_break(line.tokens.as_slice());
        let branch_shape = branch_shape(line.tokens.as_slice());

        // A logical line inside an explicit block receives a virtual level.
        // Physical indentation is still retained, so nested Python remains
        // possible and ordinary Python lines can be mixed freely.
        let branch_depth = branch_shape.is_some().then(|| depth.saturating_sub(1));
        let line_depth = if is_end.is_some() || branch_depth.is_some() {
            branch_depth.unwrap_or_else(|| depth.saturating_sub(1))
        } else {
            depth
        };
        virtual_indents[index] = line_depth;
        let mut parse_line = line.clone();
        parse_line.indent = line.indent + line_depth;

        // A sentence header without physical indentation is allowed to open
        // an explicit block when a matching end appears later.  Giving the
        // existing suite parser a synthetic next indent keeps all old
        // indentation diagnostics and inline handling intact.
        let next_indent = lines.get(index + 1).map(|next| next.indent + depth);
        let has_next_line = lines.get(index + 1).is_some();
        let unindented_next_line = lines
            .get(index + 1)
            .is_some_and(|next| next.indent <= line.indent);
        let force_suite = is_header_shape(&line.tokens)
            && line
                .tokens
                .iter()
                .all(|token| !matches!(token.tok, Tok::Colon))
            && (has_future_end(lines, index) || (has_next_line && unindented_next_line));
        let next_indent = force_suite.then_some(parse_line.indent + 1).or(next_indent);

        bindings.enter_line(parse_line.indent);
        let known_names = bindings.visible_names();
        let block = BlockCtx::TopLevel {
            line: &parse_line,
            next_indent,
        };

        // `end` and a bare `break` are valid Python-shaped words in a few
        // contexts, so explicit blocks claim them before Python-wins.  A
        // Korean `끝` is never valid Python and is handled the same way at
        // top level to provide a useful unmatched-end diagnostic.
        let direct_stmt = if is_end.is_some() && (depth > 0 || is_end == Some(Spelling::Korean)) {
            Some(Ok(Some(NmeStmt::End)))
        } else if is_break && (depth > 0 || is_korean_break_alias(&line.tokens)) {
            Some(Ok(Some(NmeStmt::Break)))
        } else if branch_shape.is_some()
            && (depth > 0 || is_korean_branch_alias(&line.tokens))
            && !line
                .tokens
                .iter()
                .any(|token| matches!(token.tok, Tok::Equal | Tok::Colon))
        {
            Some(match_branch(
                source,
                &line.tokens,
                &block,
                &known_names,
                MatchMode::Exact,
            ))
        } else {
            None
        };
        let classified =
            direct_stmt.unwrap_or_else(|| classify(source, &line.tokens, &block, &known_names));
        match classified {
            Ok(Some(stmt)) => {
                if matches!(stmt, NmeStmt::End) {
                    if blocks.is_empty() {
                        problems.push(unmatched_end_diagnostic(line.span));
                        continue;
                    }
                    blocks.pop();
                }
                if matches!(stmt, NmeStmt::Break)
                    && line.indent == 0
                    && !blocks
                        .iter()
                        .any(|block| matches!(block, ExplicitBlock::Loop))
                {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if let Some(branch) = &branch_shape {
                    if !validate_branch(branch, &mut blocks, line.span, &mut problems) {
                        continue;
                    }
                }
                let virtual_indent = if matches!(stmt, NmeStmt::End) || branch_shape.is_some() {
                    line_depth
                } else {
                    depth
                };
                bindings.remember_nme(&stmt);
                found.push(NmeLine {
                    line_index: index,
                    span: line.span,
                    stmt,
                    virtual_indent,
                });
                if let Some(
                    NmeStmt::Times { inline: None, .. }
                    | NmeStmt::When { inline: None, .. }
                    | NmeStmt::While { inline: None, .. },
                ) = found.last().map(|line| &line.stmt)
                {
                    if force_suite || has_future_end(lines, index) {
                        let is_loop = matches!(
                            found.last().map(|line| &line.stmt),
                            Some(NmeStmt::While { .. } | NmeStmt::Times { .. })
                        );
                        blocks.push(if is_loop {
                            ExplicitBlock::Loop
                        } else {
                            ExplicitBlock::Conditional { else_seen: false }
                        });
                    }
                }
            }
            Ok(None) => bindings.remember_python(&line.tokens, parse_line.indent),
            Err(problem) => problems.push(problem),
        }
    }

    if !blocks.is_empty() {
        problems.extend(blocks.iter().map(|block| {
            missing_end_diagnostic(block, lines.last().map_or(0, |line| line.span.end))
        }));
    }

    if problems.is_empty() {
        Ok(ParsedProgram {
            nme_lines: found,
            virtual_indents,
        })
    } else {
        Err(problems)
    }
}

#[derive(Debug, Clone, Copy)]
enum ExplicitBlock {
    Loop,
    Conditional { else_seen: bool },
}

enum BlockCtx<'a> {
    TopLevel {
        line: &'a LogicalLine,
        next_indent: Option<usize>,
    },
    Inline,
}

#[derive(Debug, Clone, Copy)]
enum BranchShape {
    Else,
    ElseIf,
}

fn exact_end(tokens: &[Token]) -> Option<Spelling> {
    if tokens.len() == 1 && token_matches_exact(&tokens[0], END_WORDS_EN) {
        Some(Spelling::English)
    } else if tokens.len() == 1 && token_matches_exact(&tokens[0], END_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

fn exact_break(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let consumed = action_phrase_at(tokens, 0, BREAK_WORDS_EN, MatchMode::Exact)
        .or_else(|| action_phrase_at(tokens, 0, BREAK_WORDS_KO, MatchMode::Exact));
    consumed.is_some_and(|consumed| {
        tokens[consumed..]
            .iter()
            .all(|token| is_command_ending(token))
    })
}

fn branch_shape(tokens: &[Token]) -> Option<BranchShape> {
    if tokens.is_empty() {
        return None;
    }
    if matches!(tokens[0].tok, Tok::Elif)
        || token_matches_exact(&tokens[0], &["elif"])
        || token_matches_exact(&tokens[0], &["아니면만약", "그렇지않으면만약"])
        || (token_matches_exact(&tokens[0], &["아니면", "그렇지않으면"])
            && when_action_at(tokens, 1, MatchMode::Exact).is_some())
        || (action_phrase_at(tokens, 0, ELSE_WORDS_EN, MatchMode::Exact)
            .is_some_and(|consumed| when_action_at(tokens, consumed, MatchMode::Exact).is_some()))
        || (action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact)
            .is_some_and(|consumed| when_action_at(tokens, consumed, MatchMode::Exact).is_some()))
    {
        return Some(BranchShape::ElseIf);
    }
    (action_phrase_at(tokens, 0, ELSE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact).is_some())
    .then_some(BranchShape::Else)
}

fn is_korean_branch_alias(tokens: &[Token]) -> bool {
    tokens.first().is_some()
        && action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_korean_break_alias(tokens: &[Token]) -> bool {
    action_phrase_at(tokens, 0, BREAK_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_header_shape(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    when_action_at(tokens, 0, MatchMode::Exact).is_some()
        || repeat_action_at(tokens, 0, MatchMode::Exact).is_some()
        || matches!(tokens[0].tok, Tok::While)
        || action_phrase_at(tokens, 0, WHILE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_KO, MatchMode::Exact).is_some()
        || find_count_marker(tokens, MatchMode::Exact).is_some()
        || tokens.iter().any(|token| {
            name_word(token).is_some_and(|word| {
                word.len() > TIMES_KEYWORD_KO.len() && word.ends_with(TIMES_KEYWORD_KO)
            })
        })
        || tokens
            .last()
            .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO) && tokens.len() > 1)
}

fn has_future_end(lines: &[LogicalLine], index: usize) -> bool {
    lines[index + 1..]
        .iter()
        .any(|line| exact_end(&line.tokens).is_some())
}

fn validate_branch(
    branch: &BranchShape,
    blocks: &mut [ExplicitBlock],
    span: Span,
    problems: &mut Vec<Diagnostic>,
) -> bool {
    let Some(top) = blocks.last_mut() else {
        problems.push(branch_without_condition_diagnostic(span));
        return false;
    };
    let ExplicitBlock::Conditional { else_seen } = top else {
        problems.push(branch_without_condition_diagnostic(span));
        return false;
    };
    match branch {
        BranchShape::ElseIf if *else_seen => {
            problems.push(duplicate_else_diagnostic(span));
            false
        }
        BranchShape::Else => {
            if *else_seen {
                problems.push(duplicate_else_diagnostic(span));
                false
            } else {
                *else_seen = true;
                true
            }
        }
        BranchShape::ElseIf => true,
    }
}

fn unmatched_end_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "there is no open NME block for this `end`",
        "이 `끝`을 닫을 열린 NME 블록이 없어요",
        span,
    )
    .with_bilingual_hint(
        "open a `while`, `if`, or `repeat` block first",
        "먼저 `동안`, `만약`, 또는 `반복` 블록을 열어 주세요",
    )
}

fn break_outside_loop_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "`break` can only be used inside a loop",
        "`멈춰`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside `while ... end` or `repeat ... end`",
        "`동안 ... 끝` 또는 `반복 ... 끝` 안에 넣어 주세요",
    )
}

fn branch_without_condition_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "`else` or `elif` needs an open condition block",
        "`아니면`이나 `elif` 앞에 열린 조건 블록이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "start with `if condition` and close the whole block with `end`",
        "`만약 조건`으로 시작하고 전체 블록을 `끝`으로 닫아 주세요",
    )
}

fn duplicate_else_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "this condition already has an `else` branch",
        "이 조건에는 이미 `아니면` 가지가 있어요",
        span,
    )
    .with_bilingual_hint(
        "put another condition before `else`, or close the block",
        "`아니면` 전에 조건을 더 쓰거나 블록을 닫아 주세요",
    )
}

fn missing_end_diagnostic(block: &ExplicitBlock, offset: usize) -> Diagnostic {
    let (english, korean) = match block {
        ExplicitBlock::Loop => (
            "this loop is missing its closing `end`",
            "이 반복문에는 닫는 `끝`이 필요해요",
        ),
        ExplicitBlock::Conditional { .. } => (
            "this condition is missing its closing `end`",
            "이 조건문에는 닫는 `끝`이 필요해요",
        ),
    };
    Diagnostic::bilingual(english, korean, Span::new(offset, offset)).with_bilingual_hint(
        "add `end`/`끝` on a line by itself",
        "줄 하나에 `end` 또는 `끝`만 적어 주세요",
    )
}

fn classify(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    debug_assert!(!tokens.is_empty());

    let text = token_text(source, tokens);
    if is_valid_python_statement(text) || is_valid_python_header(text) {
        return Ok(None);
    }

    // Future Python grammar may be newer than rustpython-parser. A
    // call/attribute/subscript shape is never NME's whitespace-led beginner
    // form, so preserve it for the selected CPython instead of hijacking it.
    if looks_like_python_invocation(tokens) {
        return Ok(None);
    }

    if let Some(stmt) = match_break(source, tokens, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_while(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_branch(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }

    if is_python_keyword(&tokens[0].tok) && !matches!(tokens[0].tok, Tok::If) {
        return Ok(None);
    }

    macro_rules! exact_match {
        ($matcher:expr) => {
            if let Some(stmt) = $matcher? {
                return Ok(Some(stmt));
            }
        };
    }
    if when_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_when(source, tokens, block, known_names, MatchMode::Exact);
    }
    if repeat_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_times(source, tokens, block, known_names, MatchMode::Exact);
    }
    if ask_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_ask(source, tokens, known_names, MatchMode::Exact);
    }
    if output_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_say(source, tokens, known_names, MatchMode::Exact);
    }
    if action_phrase_at(tokens, 0, SET_WORDS_EN, MatchMode::Exact).is_some() {
        return match_set(source, tokens, known_names, MatchMode::Exact);
    }
    if action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Exact).is_some()
    {
        return match_use_random(source, tokens, MatchMode::Exact);
    }
    exact_match!(match_when(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_times(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));
    // A count marker followed by a one-edit repeat action is stronger
    // sentence structure than an exact output action at the end of the line.
    // For example, `2번 반목해서 다시 말해줘` should recover `반복해서`
    // instead of printing the entire prefix as plain text.
    if has_recoverable_repeat_shape(tokens) {
        exact_match!(match_times(
            source,
            tokens,
            block,
            known_names,
            MatchMode::Recover
        ));
    }
    exact_match!(match_ask(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_say(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_set(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_use_random(source, tokens, MatchMode::Exact));

    let recovered = [
        match_when(source, tokens, block, known_names, MatchMode::Recover),
        match_times(source, tokens, block, known_names, MatchMode::Recover),
        match_ask(source, tokens, known_names, MatchMode::Recover),
        match_say(source, tokens, known_names, MatchMode::Recover),
        match_set(source, tokens, known_names, MatchMode::Recover),
        match_use_random(source, tokens, MatchMode::Recover),
    ];
    let mut candidates = Vec::new();
    let mut recovery_problems = Vec::new();
    for result in recovered {
        match result {
            Ok(Some(stmt)) => candidates.push(stmt),
            Ok(None) => {}
            Err(problem) => recovery_problems.push(problem),
        }
    }
    if candidates.len() == 1 && recovery_problems.is_empty() {
        return Ok(candidates.pop());
    }
    if candidates.len() > 1 || (!candidates.is_empty() && !recovery_problems.is_empty()) {
        return Err(ambiguous_action_diagnostic(tokens));
    }
    if recovery_problems.len() == 1 {
        return Err(recovery_problems.pop().expect("one recovery problem"));
    }
    if recovery_problems.len() > 1 {
        return Err(ambiguous_action_diagnostic(tokens));
    }

    if tokens.iter().any(is_sentence_punctuation) {
        return Err(Diagnostic::bilingual(
            "`?` and `!` can be used in sentence-style NME, but this line was ambiguous",
            "문장형 NME에서 `?`와 `!`를 쓸 수 있지만, 이 줄의 뜻은 모호해요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "add `show` or `ask` so the sentence has one clear meaning",
            "문장의 뜻이 하나가 되도록 `말해줘` 또는 `물어봐`를 붙이세요",
        ));
    }

    // Invalid Python led by another Python keyword belongs to Python. This
    // preserves its own context-sensitive diagnostics (`elif`, `except`, ...)
    // while still allowing the deliberately supported mixed `if 조건` form.
    if is_python_keyword(&tokens[0].tok) {
        return Ok(None);
    }
    if looks_like_plain_prose(tokens) {
        return Err(missing_action_diagnostic(tokens));
    }
    Ok(None)
}

// ---------------------------------------------------------------- output

fn match_say(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some((spelling, consumed)) = output_action_at(tokens, 0, mode) {
        let mut body_start = consumed;
        if tokens.get(body_start).is_some_and(is_command_ending) && body_start + 1 < tokens.len() {
            body_start += 1;
        }
        if body_start >= tokens.len() {
            return Err(say_missing(spelling, tokens[0].span));
        }
        let body = &tokens[body_start..];
        let prefer_text = consumed != 1
            || mode == MatchMode::Recover
            || (!token_is_exact_name(&tokens[0], SAY_KEYWORD)
                && !token_is_exact_name(&tokens[0], SAY_KEYWORD_KO));
        if !prefer_text {
            let span = span_of(body);
            let text = &source[span.start..span.end];
            if looks_like_broken_expression(body) && !is_valid_python_expression(text) {
                return Err(Diagnostic::bilingual(
                    "I couldn't understand what you want to `say`",
                    "`말해` 뒤의 값을 이해하지 못했어요",
                    span,
                )
                .with_bilingual_hint(
                    "finish the value, or use plain words such as `show Hello world`",
                    "값을 완성하거나 `안녕하세요 말해줘`처럼 평범한 문장으로 쓰세요",
                ));
            }
        }
        let value = parse_value(source, body, known_names, prefer_text).map_err(|()| {
            Diagnostic::bilingual(
                "I couldn't understand what to show",
                "무엇을 말할지 이해하지 못했어요",
                span_of(body),
            )
            .with_bilingual_hint(
                "write a value, or a sentence such as `show Hello world`",
                "`안녕하세요 말해줘`처럼 평범한 문장으로 적어도 돼요",
            )
        })?;
        return Ok(Some(NmeStmt::Say { value }));
    }

    let Some((action_start, spelling, action_end)) = output_action_ending(tokens, mode) else {
        return Ok(None);
    };
    if action_start == 0 {
        return Err(say_missing(spelling, tokens[action_start].span));
    }
    debug_assert!(action_end <= tokens.len());
    let value_tokens = trim_suffix_say_value(&tokens[..action_start]);
    if value_tokens.is_empty() {
        return Err(say_missing(spelling, tokens[action_start].span));
    }
    let value = parse_value(source, &value_tokens, known_names, true).map_err(|()| {
        Diagnostic::bilingual(
            "I couldn't understand the sentence to show",
            "말할 문장을 이해하지 못했어요",
            span_of(&value_tokens),
        )
        .with_bilingual_hint(
            "write it like `Hello world show`",
            "`안녕하세요 말해줘`처럼 쓰세요",
        )
    })?;
    Ok(Some(NmeStmt::Say { value }))
}

fn say_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual("there is nothing to show", "말할 내용이 비어 있어요", span)
        .with_bilingual_hint(
            "write `show Hello world`",
            "`안녕하세요 말해줘`처럼 내용을 함께 적어 주세요",
        )
}

fn output_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, SAY_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, SAY_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

// ---------------------------------------------------------------- input

fn match_ask(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(shape) = find_ask_shape(tokens, mode) else {
        return Ok(None);
    };
    let Some(target_token) = tokens.get(shape.target_at) else {
        return Err(ask_target_diagnostic(
            shape.spelling,
            tokens[shape.action_start].span,
        ));
    };
    let Some(target_word) = name_word(target_token) else {
        return Err(ask_target_diagnostic(shape.spelling, target_token.span));
    };
    let target = strip_target_particle(target_word).to_string();
    if target.is_empty() {
        return Err(ask_target_diagnostic(shape.spelling, target_token.span));
    }

    let mut prompt_end = tokens.len();
    if shape.prompt_start + 1 == prompt_end && tokens.last().is_some_and(is_command_ending) {
        prompt_end -= 1;
    }
    let prompt = if shape.prompt_start >= prompt_end {
        None
    } else if matches!(tokens[shape.prompt_start].tok, Tok::Comma) {
        let expression_tokens = &tokens[shape.prompt_start + 1..prompt_end];
        if expression_tokens.is_empty() {
            return Err(Diagnostic::bilingual(
                "the question after the comma is missing",
                "쉼표 뒤의 질문이 비어 있어요",
                tokens[shape.prompt_start].span,
            )
            .with_bilingual_hint(
                "add a question after the comma",
                "쉼표 뒤에 질문을 적어 주세요",
            ));
        }
        let span = span_of(expression_tokens);
        if !is_valid_python_expression(&source[span.start..span.end]) {
            return Err(Diagnostic::bilingual(
                "I couldn't understand the question",
                "질문 내용을 이해하지 못했어요",
                span,
            )
            .with_bilingual_hint(
                "remove the comma to write a plain sentence without quotes",
                "쉼표를 빼면 따옴표 없는 평범한 문장으로 쓸 수 있어요",
            ));
        }
        Some(Value::Python(Code::Source(span)))
    } else {
        // A comma means precise beginner syntax. Without one, the remainder is
        // deliberately sentence text and therefore needs no quotes.
        let prompt_tokens = &tokens[shape.prompt_start..prompt_end];
        let prompt_span = span_of(prompt_tokens);
        if is_valid_python_expression(&source[prompt_span.start..prompt_span.end])
            && !matches!(prompt_tokens[0].tok, Tok::Name { .. })
        {
            Some(Value::Python(Code::Source(prompt_span)))
        } else {
            let mut prompt_names = known_names.clone();
            prompt_names.remove(&target);
            Some(Value::Text(make_text_template(
                source,
                prompt_tokens,
                &prompt_names,
            )))
        }
    };
    Ok(Some(NmeStmt::Ask {
        target,
        prompt,
        kind: shape.kind,
    }))
}

struct AskShape {
    action_start: usize,
    target_at: usize,
    prompt_start: usize,
    spelling: Spelling,
    kind: InputKind,
}

fn find_ask_shape(tokens: &[Token], mode: MatchMode) -> Option<AskShape> {
    if let Some((spelling, consumed)) = ask_action_at(tokens, 0, mode) {
        let mut target_at = consumed;
        let kind = if tokens
            .get(target_at)
            .is_some_and(|token| token_matches_exact(token, NUMBER_WORDS))
        {
            target_at += 1;
            InputKind::Number
        } else {
            InputKind::Text
        };
        return Some(AskShape {
            action_start: 0,
            target_at,
            prompt_start: target_at + 1,
            spelling,
            kind,
        });
    }

    let mut target_at = 0;
    while tokens
        .get(target_at)
        .is_some_and(|token| token_matches_exact(token, SENTENCE_FILLERS))
    {
        target_at += 1;
    }
    name_word(tokens.get(target_at)?).filter(|name| !name.is_empty())?;
    for action_start in target_at + 1..tokens.len() {
        let Some((spelling, consumed)) = ask_action_at(tokens, action_start, mode) else {
            continue;
        };
        let modifiers = &tokens[target_at + 1..action_start];
        if !modifiers.iter().all(is_ask_modifier) {
            continue;
        }
        let kind = if modifiers.iter().any(|token| {
            token_matches_exact(token, NUMBER_WORDS) || name_word(token) == Some("숫자")
        }) {
            InputKind::Number
        } else {
            InputKind::Text
        };
        return Some(AskShape {
            action_start,
            target_at,
            prompt_start: action_start + consumed,
            spelling,
            kind,
        });
    }
    None
}

fn ask_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, ASK_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, ASK_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn is_ask_modifier(token: &Token) -> bool {
    token_matches_exact(
        token,
        &[
            "을",
            "를",
            "에게",
            "한테",
            "number",
            "numeric",
            "숫자",
            "숫자로",
            "수로",
            "로",
            "으로",
            "좀",
        ],
    )
}

fn ask_target_diagnostic(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "write the name that should hold the answer",
        "대답을 담을 이름이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "for example: `ask name What is your name`",
        "`이름을 물어봐 이름이 뭐예요`처럼 쓰세요",
    )
}

// ----------------------------------------------------------- control flow

fn match_break(
    _source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(consumed) = action_phrase_at(tokens, 0, BREAK_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, 0, BREAK_WORDS_KO, mode))
    else {
        return Ok(None);
    };
    if tokens[consumed..]
        .iter()
        .any(|token| !is_command_ending(token))
    {
        return Err(Diagnostic::bilingual(
            "I couldn't understand this break command",
            "이 반복 중단 명령을 이해하지 못했어요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write only `break` or `여기서 멈춰`",
            "`break` 또는 `여기서 멈춰`만 적어 주세요",
        ));
    }
    Ok(Some(NmeStmt::Break))
}

fn match_while(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let (spelling, consumed, suffix_form) =
        if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While))
            || action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).is_some()
        {
            let consumed = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While)) {
                1
            } else {
                action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).expect("checked above")
            };
            (Spelling::English, consumed, false)
        } else if action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).is_some() {
            (
                Spelling::Korean,
                action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).expect("checked above"),
                false,
            )
        } else if tokens.len() > 1
            && tokens
                .last()
                .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
        {
            (Spelling::Korean, tokens.len() - 1, true)
        } else {
            return Ok(None);
        };

    let condition_slice = if suffix_form {
        &tokens[..consumed]
    } else {
        &tokens[consumed..]
    };
    if condition_slice.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if !suffix_form {
        if let Some(colon_at) = find_condition_colon(source, tokens, consumed) {
            if colon_at == consumed {
                return Err(condition_missing(spelling, tokens[colon_at].span));
            }
            let condition_span =
                Span::new(tokens[consumed].span.start, tokens[colon_at - 1].span.end);
            if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
                return Err(condition_invalid(spelling, condition_span));
            }
            let inline = parse_suite_body(
                source,
                &tokens[colon_at + 1..],
                block,
                SuiteKind::Condition,
                Span::new(tokens[0].span.start, tokens[colon_at].span.end),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While {
                condition: Condition::Python(Code::Source(condition_span)),
                inline,
            }));
        }
    }

    let (condition_tokens, connector, body) = if suffix_form {
        if let Some((connector_at, connector)) = find_condition_connector(condition_slice) {
            (
                &condition_slice[..connector_at],
                Some(connector),
                &tokens[tokens.len()..],
            )
        } else {
            (condition_slice, None, &tokens[tokens.len()..])
        }
    } else if let Some((relative_at, connector)) = find_condition_connector(condition_slice) {
        let at = relative_at + consumed;
        (&tokens[consumed..at], Some(connector), &tokens[at + 1..])
    } else {
        (&tokens[consumed..], None, &tokens[tokens.len()..])
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::While { condition, inline }))
}

fn match_branch(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A colon-bearing `else:`/`elif ...:` is ordinary Python.  The easy
    // branch spelling deliberately omits the colon and closes with `end`.
    if tokens.iter().any(|token| matches!(token.tok, Tok::Colon)) {
        return Ok(None);
    }
    let Some(shape) = branch_shape(tokens) else {
        return Ok(None);
    };
    let (consumed, spelling) = if matches!(shape, BranchShape::ElseIf) {
        if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Elif))
            || token_matches_exact(&tokens[0], &["elif"])
        {
            (1, Spelling::English)
        } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_EN, mode) {
            (
                consumed + when_action_at(tokens, consumed, mode).map_or(0, |(_, used)| used),
                Spelling::English,
            )
        } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_KO, mode) {
            (
                consumed + when_action_at(tokens, consumed, mode).map_or(0, |(_, used)| used),
                Spelling::Korean,
            )
        } else {
            return Ok(None);
        }
    } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_EN, mode) {
        (consumed, Spelling::English)
    } else if let Some(consumed) = action_phrase_at(tokens, 0, ELSE_WORDS_KO, mode) {
        (consumed, Spelling::Korean)
    } else {
        return Ok(None);
    };

    if matches!(shape, BranchShape::Else) {
        let body = &tokens[consumed..];
        let inline = parse_suite_body(
            source,
            body,
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Else { inline }));
    }

    if consumed >= tokens.len() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition_start = consumed;
    if let Some(colon_at) = find_condition_colon(source, tokens, condition_start) {
        if colon_at == condition_start {
            return Err(condition_missing(spelling, tokens[colon_at].span));
        }
        let condition_span = Span::new(
            tokens[condition_start].span.start,
            tokens[colon_at - 1].span.end,
        );
        if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
            return Err(condition_invalid(spelling, condition_span));
        }
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Condition,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::ElseIf {
            condition: Condition::Python(Code::Source(condition_span)),
            inline,
        }));
    }
    let remainder = &tokens[condition_start..];
    let (condition_tokens, connector, body) = match find_condition_connector(remainder) {
        Some((relative_at, connector)) => {
            let at = relative_at + condition_start;
            (
                &tokens[condition_start..at],
                Some(connector),
                &tokens[at + 1..],
            )
        }
        None => (remainder, None, &tokens[tokens.len()..]),
    };
    let condition =
        parse_natural_condition(source, condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::ElseIf { condition, inline }))
}

// -------------------------------------------------------------- condition

fn match_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((spelling, consumed)) = when_action_at(tokens, 0, mode) else {
        return Ok(None);
    };
    if token_word(&tokens[0]) == Some("혹시")
        && tokens
            .iter()
            .enumerate()
            .any(|(index, _)| ask_action_at(tokens, index, MatchMode::Exact).is_some())
    {
        return Ok(None);
    }
    let starter_exact = mode == MatchMode::Exact;
    if tokens.len() == consumed {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if let Some(colon_at) = find_condition_colon(source, tokens, consumed) {
        if colon_at == consumed {
            return Err(condition_missing(spelling, tokens[colon_at].span));
        }
        let condition_span = Span::new(tokens[consumed].span.start, tokens[colon_at - 1].span.end);
        if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
            return Err(condition_invalid(spelling, condition_span));
        }
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Condition,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When {
            condition: Condition::Python(Code::Source(condition_span)),
            inline,
        }));
    }

    let natural = find_condition_connector(&tokens[consumed..]);
    if !starter_exact && natural.is_none() && matches!(block, BlockCtx::Inline) {
        // A short sentence word may be one edit away from a condition alias.
        // Without a connector, colon, or following block there is not enough
        // evidence to recover it as a typo, so let another construct decide.
        return Ok(None);
    }
    let (condition_tokens, connector, body) = match natural {
        Some((relative_at, connector)) => {
            let at = relative_at + consumed;
            (&tokens[consumed..at], Some(connector), &tokens[at + 1..])
        }
        None => (&tokens[consumed..], None, &tokens[tokens.len()..]),
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

fn when_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    if tokens
        .get(start)
        .is_some_and(|token| matches!(token.tok, Tok::If))
    {
        return Some((Spelling::English, 1));
    }
    action_phrase_at(tokens, start, WHEN_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, WHEN_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConditionConnector {
    Then,
    Exists,
    Missing,
    Equals,
    Greater,
    Less,
}

fn find_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    let exact = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            condition_connector_exact(token, index + 1 == tokens.len())
                .map(|connector| (index, connector))
        })
        .collect::<Vec<_>>();
    if let Some(connector) = exact
        .iter()
        .copied()
        .find(|(_, connector)| *connector == ConditionConnector::Then)
        .or_else(|| exact.first().copied())
    {
        return Some(connector);
    }

    // Only recover a connector typo when the whole condition has no exact
    // connector. Otherwise `than ... then` could split at `than`, because it
    // is one edit away from `then`.
    let recovered = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            condition_connector_recovered(token, index + 1 == tokens.len())
                .map(|connector| (index, connector))
        })
        .collect::<Vec<_>>();
    (recovered.len() == 1).then(|| recovered[0])
}

fn parse_natural_condition(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    // `or` has lower precedence than `and`, just like Python.  Splitting on
    // tokens (rather than source text) keeps strings and nested expressions
    // out of the easy-language grammar.
    if let Some(index) = logical_operator_at(tokens, LogicalOp::Or) {
        let left = parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
        let right = parse_natural_condition(
            source,
            &tokens[index + 1..],
            connector,
            known_names,
            spelling,
        )?;
        return Ok(Condition::Logical {
            left: Box::new(left),
            operator: LogicalOp::Or,
            right: Box::new(right),
        });
    }
    if let Some(index) = logical_operator_at(tokens, LogicalOp::And) {
        let left = parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
        let right = parse_natural_condition(
            source,
            &tokens[index + 1..],
            connector,
            known_names,
            spelling,
        )?;
        return Ok(Condition::Logical {
            left: Box::new(left),
            operator: LogicalOp::And,
            right: Box::new(right),
        });
    }
    parse_natural_condition_atom(source, tokens, connector, known_names, spelling)
}

fn logical_operator_at(tokens: &[Token], operator: LogicalOp) -> Option<usize> {
    let expected = match operator {
        LogicalOp::And => &["and", "그리고"][..],
        LogicalOp::Or => &["or", "또는"][..],
    };
    let mut depth = 0usize;
    tokens.iter().enumerate().find_map(|(index, token)| {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        (depth == 0 && token_matches_exact(token, expected)).then_some(index)
    })
}

fn parse_natural_condition_atom(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    let mut cleaned: Vec<&Token> = tokens.iter().collect();
    while cleaned.first().is_some_and(|token| {
        token_matches_exact(token, &["정말", "혹시", "please", "really", "the"])
    }) {
        cleaned.remove(0);
    }
    if cleaned.is_empty() {
        return Err(condition_missing(spelling, span_of(tokens)));
    }

    if let Some(condition) = parse_english_condition(source, &cleaned, known_names) {
        return Ok(condition);
    }

    match connector {
        Some(ConditionConnector::Missing) => {
            let (value, explicit_not) = parse_truth_subject(&cleaned, known_names, spelling)?;
            return Ok(Condition::Truthy {
                value,
                negated: !explicit_not,
            });
        }
        Some(ConditionConnector::Exists) => {
            let (value, explicit_not) = parse_truth_subject(&cleaned, known_names, spelling)?;
            return Ok(Condition::Truthy {
                value,
                negated: explicit_not,
            });
        }
        Some(ConditionConnector::Greater | ConditionConnector::Less) => {
            let operator = if matches!(connector, Some(ConditionConnector::Greater)) {
                CompareOp::Greater
            } else {
                CompareOp::Less
            };
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                operator,
                &["보다", "더", "작을", "클"],
                spelling,
            );
        }
        Some(ConditionConnector::Equals) => {
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                CompareOp::Equal,
                &["과", "와", "랑", "이랑", "하고", "to"],
                spelling,
            );
        }
        _ => {}
    }

    if cleaned.len() == 1 {
        return Ok(Condition::Truthy {
            value: condition_left(cleaned[0], known_names),
            negated: false,
        });
    }

    let condition_span = Span::new(cleaned[0].span.start, cleaned[cleaned.len() - 1].span.end);
    let condition_text = &source[condition_span.start..condition_span.end];
    if is_valid_python_expression(condition_text) {
        return Ok(Condition::Python(Code::Source(condition_span)));
    }

    Err(condition_invalid(spelling, condition_span))
}

fn parse_truth_subject(
    tokens: &[&Token],
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<(ConditionValue, bool), Diagnostic> {
    let mut cursor = 1;
    if tokens
        .get(cursor)
        .is_some_and(|token| token_word(token) == Some("is"))
    {
        cursor += 1;
    }
    let explicit_not = tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"));
    if explicit_not {
        cursor += 1;
    }
    if cursor != tokens.len() {
        return Err(condition_invalid(
            spelling,
            Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end),
        ));
    }
    Ok((condition_left(tokens[0], known_names), explicit_not))
}

fn parse_korean_comparison(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
    operator: CompareOp,
    trailing_markers: &[&str],
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    if tokens.len() < 2 {
        return Err(condition_invalid(spelling, span_of_refs(tokens)));
    }
    let left = condition_left(tokens[0], known_names);
    let mut right = tokens[1..]
        .iter()
        .map(|token| (*token).clone())
        .collect::<Vec<_>>();
    while right
        .first()
        .is_some_and(|token| token_matches_exact(token, &["은", "는", "이", "가"]))
    {
        right.remove(0);
    }
    trim_condition_markers(&mut right, trailing_markers);
    if right.is_empty() {
        return Err(condition_invalid(spelling, span_of_refs(tokens)));
    }
    let right = condition_rhs(source, &right, known_names)
        .ok_or_else(|| condition_invalid(spelling, span_of_refs(tokens)))?;
    Ok(Condition::Compare {
        left,
        operator,
        right,
        negated: false,
    })
}

fn parse_english_condition(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
) -> Option<Condition> {
    if tokens.len() < 2 {
        return None;
    }
    let left = condition_left(tokens[0], known_names);
    let mut cursor = 1;
    if token_word(tokens[cursor]) == Some("is") {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["really"]))
    {
        cursor += 1;
    }
    let negated = tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"));
    if negated {
        cursor += 1;
    }
    let predicate = tokens.get(cursor).and_then(|token| token_word(token))?;
    if condition_word_matches(predicate, &["exists", "present", "missing", "absent"]) {
        if cursor + 1 != tokens.len() {
            return None;
        }
        let missing = condition_word_matches(predicate, &["missing", "absent"]);
        return Some(Condition::Truthy {
            value: left,
            negated: missing ^ negated,
        });
    }
    let operator = if condition_word_matches(predicate, &["greater", "above"]) {
        CompareOp::Greater
    } else if condition_word_matches(predicate, &["less", "below"]) {
        CompareOp::Less
    } else if condition_word_matches(predicate, &["equals", "equal"]) {
        CompareOp::Equal
    } else {
        return None;
    };
    cursor += 1;
    while tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["to", "than"]))
    {
        cursor += 1;
    }
    let right_tokens = tokens.get(cursor..)?;
    if right_tokens.is_empty() {
        return None;
    }
    let owned = right_tokens
        .iter()
        .map(|token| (*token).clone())
        .collect::<Vec<_>>();
    let right = condition_rhs(source, &owned, known_names)?;
    Some(Condition::Compare {
        left,
        operator,
        right,
        negated,
    })
}

fn condition_left(token: &Token, known_names: &HashSet<String>) -> ConditionValue {
    if let Some(literal) = literal_token(token) {
        return ConditionValue::Literal(literal);
    }
    let Some(word) = name_word(token) else {
        return ConditionValue::Python(Code::Source(token.span));
    };
    let name = resolve_known_particle(word, known_names)
        .or_else(|| strip_any_suffix(word, &["은", "는", "이", "가"]))
        .unwrap_or(word);
    ConditionValue::Name(name.to_string())
}

fn condition_rhs(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Option<ConditionValue> {
    if tokens.len() == 1 {
        if let Some(literal) = literal_token(&tokens[0]) {
            return Some(ConditionValue::Literal(literal));
        }
        if let Some(word) = name_word(&tokens[0]) {
            if let Some(name) = resolve_known_particle(word, known_names) {
                return Some(ConditionValue::Name(name.to_string()));
            }
            return Some(ConditionValue::Text(word.to_string()));
        }
    }
    let span = span_of(tokens);
    let text = &source[span.start..span.end];
    if is_valid_python_expression(text) && tokens.iter().any(is_code_token) {
        return Some(ConditionValue::Python(Code::Source(span)));
    }
    tokens
        .iter()
        .all(is_text_token)
        .then(|| ConditionValue::Text(text.to_string()))
}

fn trim_condition_markers(tokens: &mut Vec<Token>, markers: &[&str]) {
    while tokens
        .last()
        .is_some_and(|token| token_matches_exact(token, markers))
    {
        tokens.pop();
    }
    if let Some(last) = tokens.last_mut() {
        trim_name_token_suffix(last, markers);
    }
}

fn condition_connector_exact(token: &Token, is_last: bool) -> Option<ConditionConnector> {
    let word = token_word(token)?;
    let candidates = [
        (
            ConditionConnector::Then,
            &["then", "그러면", "그럼", "하면", "경우", "때", "일때"][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (ConditionConnector::Missing, &["없으면", "없다면"][..]),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "이면", "라면"][..],
        ),
        (ConditionConnector::Greater, &["크면", "크다면", "클"][..]),
        (ConditionConnector::Less, &["작으면", "작다면", "작을"][..]),
    ];
    for (kind, words) in candidates {
        if words.contains(&word) {
            return Some(kind);
        }
    }
    if is_last {
        if matches!(word, "exists" | "present") {
            return Some(ConditionConnector::Exists);
        }
        if matches!(word, "missing" | "absent") {
            return Some(ConditionConnector::Missing);
        }
    }
    None
}

fn condition_connector_recovered(token: &Token, is_last: bool) -> Option<ConditionConnector> {
    let word = token_word(token)?;
    let candidates = [
        (
            ConditionConnector::Then,
            &["then", "그러면", "그럼", "하면", "경우", "때", "일때"][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (ConditionConnector::Missing, &["없으면", "없다면"][..]),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "이면", "라면"][..],
        ),
        (ConditionConnector::Greater, &["크면", "크다면", "클"][..]),
        (ConditionConnector::Less, &["작으면", "작다면", "작을"][..]),
    ];
    let mut recovered = candidates
        .iter()
        .filter_map(|(kind, words)| {
            words
                .iter()
                .any(|candidate| {
                    !word.eq_ignore_ascii_case(candidate)
                        && word != "the"
                        && word.chars().count() >= 2
                        && one_typo_away(word, candidate)
                })
                .then_some(*kind)
        })
        .collect::<Vec<_>>();
    if is_last {
        if ["exists", "present"].iter().any(|candidate| {
            !word.eq_ignore_ascii_case(candidate) && one_typo_away(word, candidate)
        }) {
            recovered.push(ConditionConnector::Exists);
        }
        if ["missing", "absent"].iter().any(|candidate| {
            !word.eq_ignore_ascii_case(candidate) && one_typo_away(word, candidate)
        }) {
            recovered.push(ConditionConnector::Missing);
        }
    }
    recovered.sort_by_key(|kind| *kind as u8);
    recovered.dedup();
    if recovered.len() == 1 {
        recovered.first().copied()
    } else {
        None
    }
}

fn condition_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual("the condition is missing", "조건이 비어 있어요", span)
        .with_bilingual_hint(
            "write `if ready` or `if score > 10` and indent the next line",
            "`만약에 준비됐으면`처럼 적고 다음 줄을 들여쓰세요",
        )
}

fn condition_invalid(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "I couldn't understand this condition",
        "이 조건을 확실하게 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "try `if ready`, `if score > 10`, or `if name exists`",
        "`만약에 이름이 있으면` 또는 `만약 점수 > 10`처럼 적어 보세요",
    )
}

// --------------------------------------------------------------- repeat

fn match_times(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(count) = attached_korean_times_sentence(source, tokens) {
        let inline = parse_suite_body(
            source,
            &tokens[1..],
            block,
            SuiteKind::Repeat,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }
    if let Some((count, colon_at)) = attached_korean_times_header(source, tokens) {
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Repeat,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }
    if let Some((times_at, spelling)) = find_times_colon(tokens, mode) {
        let count = parse_count(source, &tokens[..times_at], spelling)?;
        let colon_at = times_at + 1;
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Repeat,
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    // A bare count header (`3 times` / `3번`) opens a block in the same way as
    // the colon form, with `end`/`끝` providing the closing line.
    if let Some((marker_at, spelling)) = find_count_marker(tokens, mode) {
        if marker_at + 1 == tokens.len()
            && marker_at > 0
            && repeat_action_at(tokens, 0, mode).is_none()
        {
            let count = parse_count(source, &tokens[..marker_at], spelling)?;
            let inline = parse_suite_body(
                source,
                &tokens[tokens.len()..],
                block,
                SuiteKind::Repeat,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::Times { count, inline }));
        }
    }

    // Sentence order: `3번 반복해 ...` / `3 times repeat ...`.
    if let Some((marker_at, spelling)) = find_count_marker(tokens, mode) {
        if let Some((_, consumed)) = repeat_action_at(tokens, marker_at + 1, mode) {
            if marker_at == 0 {
                return Err(repeat_count_missing(spelling, tokens[marker_at + 1].span));
            }
            let count = parse_count(source, &tokens[..marker_at], spelling)?;
            let mut body_start = marker_at + 1 + consumed;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
            let inline = parse_suite_body(
                source,
                &tokens[body_start..],
                block,
                SuiteKind::Repeat,
                span_of(&tokens[..body_start]),
                known_names,
            )?;
            return Ok(Some(NmeStmt::Times { count, inline }));
        }
    }

    // English-first and freely mixed order: `repeat 3 times` / `반복해 3 times`.
    if let Some((spelling, consumed)) = repeat_action_at(tokens, 0, mode) {
        let Some((relative_marker, marker_spelling)) = find_count_marker(&tokens[consumed..], mode)
        else {
            return Err(repeat_count_missing(spelling, tokens[0].span));
        };
        let marker_at = relative_marker + consumed;
        if marker_at == consumed {
            return Err(repeat_count_missing(spelling, tokens[0].span));
        }
        let count = parse_count(source, &tokens[consumed..marker_at], marker_spelling)?;
        let mut body_start = marker_at + 1;
        if tokens.get(body_start).is_some_and(is_connector_word) {
            body_start += 1;
        }
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Repeat,
            span_of(&tokens[..body_start]),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    Ok(None)
}

fn has_recoverable_repeat_shape(tokens: &[Token]) -> bool {
    if let Some((marker_at, _)) = find_count_marker(tokens, MatchMode::Exact) {
        if repeat_action_at(tokens, marker_at + 1, MatchMode::Exact).is_none()
            && repeat_action_at(tokens, marker_at + 1, MatchMode::Recover).is_some()
        {
            return true;
        }
    }

    repeat_action_at(tokens, 0, MatchMode::Exact).is_none()
        && repeat_action_at(tokens, 0, MatchMode::Recover).is_some()
        && find_count_marker(tokens, MatchMode::Exact).is_some()
}

fn repeat_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, REPEAT_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, REPEAT_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn parse_count(source: &str, tokens: &[Token], spelling: Spelling) -> Result<Code, Diagnostic> {
    if tokens.is_empty() {
        return Err(repeat_count_missing(spelling, Span::new(0, 0)));
    }
    let span = span_of(tokens);
    if !is_valid_python_expression(&source[span.start..span.end]) {
        return Err(Diagnostic::bilingual(
            "I couldn't understand how many times to repeat",
            "몇 번 반복할지 이해하지 못했어요",
            span,
        )
        .with_bilingual_hint(
            "write a number, like `repeat 3 times`",
            "`3번 반복해`처럼 횟수를 적어 주세요",
        ));
    }
    Ok(Code::Source(span))
}

fn repeat_count_missing(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "the repeat count is missing",
        "반복 횟수가 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `repeat 3 times`",
        "`3번 반복해`처럼 숫자를 함께 적어 주세요",
    )
}

fn find_count_marker(tokens: &[Token], mode: MatchMode) -> Option<(usize, Spelling)> {
    tokens.iter().enumerate().find_map(|(index, token)| {
        if token_word_matches(token, TIMES_KEYWORD, mode) {
            Some((index, Spelling::English))
        } else if token_is_exact_name(token, TIMES_KEYWORD_KO) {
            Some((index, Spelling::Korean))
        } else {
            None
        }
    })
}

// --------------------------------------------------------------- modules

fn match_use_random(
    source: &str,
    tokens: &[Token],
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((action_start, action_end, spelling)) = find_use_action(tokens, mode) else {
        return Ok(None);
    };

    let random_positions = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| random_word_matches(token, mode).then_some(index))
        .collect::<Vec<_>>();
    if random_positions.len() != 1 {
        return Err(Diagnostic::bilingual(
            "NME only bundles `use random` for now",
            "NME에는 아직 쉬운 `랜덤` 모듈만 들어 있어요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write one module line such as `use random latest`",
            "`랜덤 사용 최신`처럼 모듈 하나를 적어 주세요",
        ));
    }
    let random_at = random_positions[0];

    let latest_positions = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| word_matches_any(token, LATEST_WORDS, mode).then_some(index))
        .collect::<Vec<_>>();
    let version_positions = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            word_matches_any(token, &["version", "버전"], mode).then_some(index)
        })
        .collect::<Vec<_>>();
    if !latest_positions.is_empty() && !version_positions.is_empty() {
        return Err(Diagnostic::bilingual(
            "choose either latest or one exact module version",
            "최신 버전과 특정 버전 중 하나만 골라 주세요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write `use random latest` or `use random version 0.0.1`",
            "`랜덤 사용 최신` 또는 `랜덤 사용 버전 0.0.1`처럼 쓰세요",
        ));
    }
    if latest_positions.len() > 1 || version_positions.len() > 1 {
        return Err(module_shape_diagnostic(spelling, span_of(tokens)));
    }

    let mut used = vec![false; tokens.len()];
    for slot in &mut used[action_start..action_end] {
        *slot = true;
    }
    used[random_at] = true;
    for &index in &latest_positions {
        used[index] = true;
    }

    let requested = if !latest_positions.is_empty() {
        ModuleVersion::Latest
    } else if let Some(&version_at) = version_positions.first() {
        if version_at < action_end.max(random_at + 1) {
            return Err(module_shape_diagnostic(spelling, tokens[version_at].span));
        }
        used[version_at] = true;
        let mut value_end = tokens.len();
        if tokens.last().is_some_and(is_command_ending) {
            value_end -= 1;
            used[value_end] = true;
        }
        let value_tokens = tokens.get(version_at + 1..value_end).ok_or_else(|| {
            Diagnostic::bilingual(
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {RANDOM_MODULE_VERSION}"),
                format!("`최신` 또는 버전 {RANDOM_MODULE_VERSION}을 사용하세요"),
            )
        })?;
        if value_tokens.is_empty() {
            return Err(Diagnostic::bilingual(
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {RANDOM_MODULE_VERSION}"),
                format!("`최신` 또는 버전 {RANDOM_MODULE_VERSION}을 사용하세요"),
            ));
        }
        for slot in &mut used[version_at + 1..value_end] {
            *slot = true;
        }
        let value_span = span_of(value_tokens);
        let raw = &source[value_span.start..value_span.end];
        let version = raw.trim_matches(['\'', '"']).to_string();
        if version != RANDOM_MODULE_VERSION {
            return Err(Diagnostic::bilingual(
                format!("random version {version} is not bundled"),
                format!("랜덤 버전 {version}은 내장되어 있지 않아요"),
                value_span,
            )
            .with_bilingual_hint(
                format!("use `latest`; this compiler bundles {RANDOM_MODULE_VERSION}"),
                format!(
                    "`최신`을 사용하세요. 이 컴파일러에는 {RANDOM_MODULE_VERSION}이 들어 있어요"
                ),
            ));
        }
        ModuleVersion::Exact(version)
    } else {
        ModuleVersion::Bundled
    };

    for (index, token) in tokens.iter().enumerate() {
        if used[index]
            || token_matches_exact(token, &["please", "the", "module", "모듈", "모듈을", "좀"])
            || is_command_ending(token)
        {
            continue;
        }
        return Err(module_shape_diagnostic(spelling, token.span));
    }

    Ok(Some(NmeStmt::UseRandom { requested }))
}

fn random_word_matches(token: &Token, mode: MatchMode) -> bool {
    name_word(token).is_some_and(|word| {
        word_matches(word, RANDOM_MODULE, mode)
            || word == RANDOM_MODULE_KO
            || strip_target_particle(word) == RANDOM_MODULE_KO
    })
}

fn find_use_action(tokens: &[Token], mode: MatchMode) -> Option<(usize, usize, Spelling)> {
    for start in 0..tokens.len() {
        if let Some(consumed) = action_phrase_at(tokens, start, USE_WORDS_EN, mode) {
            return Some((start, start + consumed, Spelling::English));
        }
        if let Some(consumed) = action_phrase_at(tokens, start, USE_WORDS_KO, mode) {
            return Some((start, start + consumed, Spelling::Korean));
        }
    }
    None
}

fn module_shape_diagnostic(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "I couldn't understand this module line",
        "이 모듈 문장을 확실하게 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `use random latest` or `use random version 0.0.1`",
        "`랜덤 사용 최신` 또는 `랜덤 사용 버전 0.0.1`처럼 쓰세요",
    )
}

// ------------------------------------------------------------ assignment

fn match_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(first) = name_word(&tokens[0]) {
        if let Some(target) = strip_assignment_particle(first) {
            if tokens.len() == 1 {
                return Err(Diagnostic::bilingual(
                    "the value to save is missing",
                    "저장할 값이 비어 있어요",
                    tokens[0].span,
                )
                .with_bilingual_hint(
                    "write a value after the name",
                    "`인사는 안녕하세요`처럼 값을 뒤에 적어 주세요",
                ));
            }
            let value = parse_value(source, &tokens[1..], known_names, true).map_err(|()| {
                Diagnostic::bilingual(
                    "I couldn't understand the value to save",
                    "저장할 값을 이해하지 못했어요",
                    span_of(&tokens[1..]),
                )
                .with_bilingual_hint(
                    "write a number, name, or plain sentence",
                    "숫자, 이름, 또는 평범한 문장을 적어 주세요",
                )
            })?;
            return Ok(Some(NmeStmt::Set {
                target: target.to_string(),
                value,
            }));
        }
    }

    if tokens.len() >= 3
        && name_word(&tokens[0]).is_some()
        && token_matches_exact(&tokens[1], &["은", "는"])
    {
        let target = name_word(&tokens[0]).expect("checked name token");
        let value = parse_value(source, &tokens[2..], known_names, true).map_err(|()| {
            Diagnostic::bilingual(
                "I couldn't understand the value to save",
                "저장할 값을 이해하지 못했어요",
                span_of(&tokens[2..]),
            )
            .with_bilingual_hint(
                "write a value after the name",
                "`인사 는 안녕하세요`처럼 값을 뒤에 적어 주세요",
            )
        })?;
        return Ok(Some(NmeStmt::Set {
            target: target.to_string(),
            value,
        }));
    }

    if let Some(consumed) = action_phrase_at(tokens, 0, SET_WORDS_EN, mode) {
        let Some(target_token) = tokens.get(consumed) else {
            return Err(Diagnostic::bilingual(
                "the name to save is missing",
                "값을 저장할 이름이 비어 있어요",
                tokens[0].span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello`",
                "`인사는 안녕하세요`처럼 쓰세요",
            ));
        };
        let Some(target) = name_word(target_token) else {
            return Err(Diagnostic::bilingual(
                "use a simple name here",
                "여기에는 간단한 이름을 써 주세요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello`",
                "`인사는 안녕하세요`처럼 쓰세요",
            ));
        };
        let mut value_start = consumed + 1;
        if tokens
            .get(value_start)
            .is_some_and(|token| token_matches_exact(token, &["to", "as", "is"]))
        {
            value_start += 1;
        }
        if value_start >= tokens.len() {
            return Err(Diagnostic::bilingual(
                "the value to save is missing",
                "저장할 값이 비어 있어요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello`",
                "`인사는 안녕하세요`처럼 쓰세요",
            ));
        }
        let value =
            parse_value(source, &tokens[value_start..], known_names, true).map_err(|()| {
                Diagnostic::bilingual(
                    "I couldn't understand the value to save",
                    "저장할 값을 이해하지 못했어요",
                    span_of(&tokens[value_start..]),
                )
                .with_bilingual_hint(
                    "write a number, name, or plain sentence",
                    "숫자, 이름, 또는 평범한 문장을 적어 주세요",
                )
            })?;
        return Ok(Some(NmeStmt::Set {
            target: target.to_string(),
            value,
        }));
    }
    Ok(None)
}

// ---------------------------------------------------------- value parsing

fn parse_value(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    prefer_text: bool,
) -> Result<Value, ()> {
    if tokens.is_empty() {
        return Err(());
    }
    if tokens.len() == 1 {
        if let Some(literal) = literal_token(&tokens[0]) {
            return Ok(Value::Literal(literal));
        }
    }
    if let Some(value) = parse_random_integer(source, tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_random_choice(source, tokens) {
        return Ok(value);
    }

    let span = span_of(tokens);
    let text = &source[span.start..span.end];
    let single_known_name =
        tokens.len() == 1 && name_word(&tokens[0]).is_some_and(|name| known_names.contains(name));
    let single_unknown_name =
        tokens.len() == 1 && name_word(&tokens[0]).is_some() && !single_known_name;
    let clearly_code = tokens.len() == 1 && !matches!(tokens[0].tok, Tok::Name { .. })
        || tokens.iter().any(|token| {
            matches!(
                token.tok,
                Tok::Plus
                    | Tok::Minus
                    | Tok::Star
                    | Tok::DoubleStar
                    | Tok::Slash
                    | Tok::DoubleSlash
                    | Tok::Percent
                    | Tok::Lpar
                    | Tok::Lsqb
                    | Tok::Lbrace
                    | Tok::EqEqual
                    | Tok::NotEqual
                    | Tok::Less
                    | Tok::Greater
                    | Tok::LessEqual
                    | Tok::GreaterEqual
            )
        });
    if is_valid_python_expression(text)
        && ((!prefer_text && !single_unknown_name) || single_known_name || clearly_code)
    {
        return Ok(Value::Python(Code::Source(span)));
    }
    Ok(Value::Text(make_text_template(source, tokens, known_names)))
}

fn parse_random_integer(source: &str, tokens: &[Token]) -> Option<Value> {
    let random_at = tokens.iter().position(|token| {
        word_matches_any(
            token,
            &[
                "랜덤",
                "랜덤정수",
                "무작위",
                "무작위숫자",
                "random",
                "randomnumber",
            ],
            MatchMode::Recover,
        )
    })?;

    // Korean/mixed order: `1부터 6까지 랜덤정수`.
    if random_at > 0 {
        let from_at = tokens[..random_at]
            .iter()
            .position(|token| token_matches_exact(token, &["부터", "에서", "from"]))?;
        let to_at = tokens[from_at + 1..random_at]
            .iter()
            .position(|token| token_matches_exact(token, &["까지", "to"]))?
            + from_at
            + 1;
        if from_at > 0 && to_at > from_at + 1 {
            let low = span_of(&tokens[..from_at]);
            let high = span_of(&tokens[from_at + 1..to_at]);
            if is_valid_python_expression(&source[low.start..low.end])
                && is_valid_python_expression(&source[high.start..high.end])
            {
                return Some(Value::RandomInteger {
                    low: Code::Source(low),
                    high: Code::Source(high),
                });
            }
        }
    }

    // English-first order: `random number from 1 to 6`.
    let from_at = tokens
        .iter()
        .position(|token| token_matches_exact(token, &["from", "부터", "에서"]))?;
    let to_at = tokens[from_at + 1..]
        .iter()
        .position(|token| token_matches_exact(token, &["to", "까지"]))?
        + from_at
        + 1;
    let low = span_of(&tokens[from_at + 1..to_at]);
    let high = span_of(&tokens[to_at + 1..]);
    if is_valid_python_expression(&source[low.start..low.end])
        && is_valid_python_expression(&source[high.start..high.end])
    {
        Some(Value::RandomInteger {
            low: Code::Source(low),
            high: Code::Source(high),
        })
    } else {
        None
    }
}

fn parse_random_choice(source: &str, tokens: &[Token]) -> Option<Value> {
    let pick_at = tokens.iter().position(|token| {
        word_matches_any(
            token,
            &["랜덤선택", "하나골라", "골라", "randomchoice", "pick"],
            MatchMode::Recover,
        )
    })?;
    let choices_tokens = if pick_at == 0 {
        let start = tokens
            .iter()
            .position(|token| token_matches_exact(token, &["from", "중에서"]))?
            + 1;
        &tokens[start..]
    } else {
        &tokens[..pick_at]
    };
    let choices: Vec<String> = choices_tokens
        .iter()
        .filter(|token| {
            !token_matches_exact(token, &["or", "and", "또는", "이나", "중", "중에서"])
                && !matches!(token.tok, Tok::Comma)
        })
        .map(|token| {
            let raw = &source[token.span.start..token.span.end];
            raw.trim_matches(['\'', '"']).to_string()
        })
        .filter(|choice| !choice.is_empty())
        .collect();
    (choices.len() >= 2).then_some(Value::RandomChoice { choices })
}

fn make_text_template(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> TextTemplate {
    let mut parts = Vec::new();
    let mut cursor = tokens[0].span.start;
    let end = tokens[tokens.len() - 1].span.end;

    for token in tokens {
        let Some(word) = name_word(token) else {
            continue;
        };
        let Some((variable, particle)) = split_template_variable(word, known_names) else {
            continue;
        };
        if cursor < token.span.start {
            push_literal(&mut parts, &source[cursor..token.span.start]);
        }
        parts.push(TextPart::Variable(variable.to_string()));
        if !particle.is_empty() {
            push_literal(&mut parts, particle);
        }
        cursor = token.span.end;
    }
    if cursor < end {
        push_literal(&mut parts, &source[cursor..end]);
    }
    if parts.is_empty() {
        parts.push(TextPart::Literal(
            source[tokens[0].span.start..end].to_string(),
        ));
    }
    TextTemplate { parts }
}

fn push_literal(parts: &mut Vec<TextPart>, text: &str) {
    if text.is_empty() {
        return;
    }
    match parts.last_mut() {
        Some(TextPart::Literal(existing)) => existing.push_str(text),
        _ => parts.push(TextPart::Literal(text.to_string())),
    }
}

fn split_template_variable<'a>(
    word: &'a str,
    known_names: &'a HashSet<String>,
) -> Option<(&'a str, &'a str)> {
    if known_names.contains(word) {
        return Some((word, ""));
    }
    let mut candidates: Vec<&String> = known_names
        .iter()
        .filter(|name| word.starts_with(name.as_str()))
        .collect();
    candidates.sort_by_key(|name| std::cmp::Reverse(name.chars().count()));
    for name in candidates {
        let particle = &word[name.len()..];
        if KOREAN_PARTICLES.contains(&particle) {
            return Some((&word[..name.len()], particle));
        }
    }
    None
}

// --------------------------------------------------------------- suites

#[derive(Clone, Copy)]
enum SuiteKind {
    Repeat,
    Condition,
}

fn parse_suite_body(
    source: &str,
    body: &[Token],
    block: &BlockCtx<'_>,
    kind: SuiteKind,
    header_span: Span,
    known_names: &HashSet<String>,
) -> Result<Option<InlineStmt>, Diagnostic> {
    if body.is_empty() {
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

    let body_span = span_of(body);
    if has_top_level_semicolon(body) {
        return Err(one_statement_diagnostic(kind, body_span));
    }
    if let Some(inner) = classify(source, body, &BlockCtx::Inline, known_names)? {
        return Ok(Some(InlineStmt::Nme(Box::new(inner))));
    }
    if !is_valid_python_statement(&source[body_span.start..body_span.end]) {
        return Err(body_diagnostic(kind, body_span));
    }
    Ok(Some(InlineStmt::Python(body_span)))
}

fn indentation_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match kind {
        SuiteKind::Repeat => Diagnostic::bilingual(
            "the lines that should repeat must be indented",
            "반복할 다음 줄은 들여써야 해요",
            span,
        )
        .with_bilingual_hint(
            "or keep it on one line: `repeat 3 times and show Hello`",
            "한 줄로 `3번 반복해서 안녕 말해줘`라고 써도 돼요",
        ),
        SuiteKind::Condition => Diagnostic::bilingual(
            "this condition needs `:` or an indented next line",
            "조건 다음에는 실행할 줄이나 `:`이 필요해요",
            span,
        )
        .with_bilingual_hint(
            "or put one statement after `then`",
            "한 문장은 `있으면` 뒤에 바로 적어도 돼요",
        ),
    }
}

fn inline_block_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "a block can't start here without a statement",
        "이 한 줄 블록에 실행할 문장이 없어요",
        span,
    )
    .with_bilingual_hint(
        "put one statement here, or use an indented block on the next line",
        "실행할 문장을 이어 쓰거나 다음 줄에 들여쓰세요",
    )
}

fn one_statement_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "only one statement fits on this line",
        "한 줄에는 문장 하나만 넣을 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put multiple statements on separate indented lines",
        "여러 문장은 다음 줄부터 하나씩 들여쓰세요",
    )
}

fn body_diagnostic(_kind: SuiteKind, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        "I couldn't understand the statement here",
        "여기 있는 문장을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write one Python, beginner, or sentence-style statement",
        "Python, 초급, 문장형 문법 중 한 문장을 적어 주세요",
    )
}

// --------------------------------------------------------------- helpers

struct BindingScope {
    body_indent: usize,
    names: HashSet<String>,
}

struct PendingScope {
    header_indent: usize,
    names: HashSet<String>,
}

struct BindingEnv {
    scopes: Vec<BindingScope>,
    pending: Option<PendingScope>,
}

impl BindingEnv {
    fn new() -> Self {
        Self {
            scopes: vec![BindingScope {
                body_indent: 0,
                names: HashSet::new(),
            }],
            pending: None,
        }
    }

    fn enter_line(&mut self, indent: usize) {
        if let Some(pending) = self.pending.take() {
            if indent > pending.header_indent {
                self.scopes.push(BindingScope {
                    body_indent: indent,
                    names: pending.names,
                });
            }
        }
        while self.scopes.len() > 1 && indent < self.scopes.last().expect("root scope").body_indent
        {
            self.scopes.pop();
        }
    }

    fn visible_names(&self) -> HashSet<String> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.names.iter().cloned())
            .collect()
    }

    fn remember_nme(&mut self, stmt: &NmeStmt) {
        remember_bindings(stmt, &mut self.scopes.last_mut().expect("root scope").names);
    }

    fn remember_python(&mut self, tokens: &[Token], indent: usize) {
        remember_python_binding(
            tokens,
            &mut self.scopes.last_mut().expect("root scope").names,
        );
        if let Some((name, parameters)) = python_scope_header(tokens) {
            self.scopes
                .last_mut()
                .expect("root scope")
                .names
                .insert(name);
            self.pending = Some(PendingScope {
                header_indent: indent,
                names: parameters,
            });
        }
    }
}

fn python_scope_header(tokens: &[Token]) -> Option<(String, HashSet<String>)> {
    let keyword_at = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def))
    {
        1
    } else if matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::Def | Tok::Class)
    ) {
        0
    } else {
        return None;
    };
    let name = name_word(tokens.get(keyword_at + 1)?)?.to_string();
    let mut parameters = HashSet::new();
    if matches!(tokens[keyword_at].tok, Tok::Def) {
        let mut inside_parameters = false;
        for token in &tokens[keyword_at + 2..] {
            match &token.tok {
                Tok::Lpar => inside_parameters = true,
                Tok::Rpar => break,
                Tok::Name { name } if inside_parameters => {
                    parameters.insert(name.clone());
                }
                _ => {}
            }
        }
    }
    Some((name, parameters))
}

fn remember_python_binding(tokens: &[Token], names: &mut HashSet<String>) {
    if let [Token {
        tok: Tok::Name { name },
        ..
    }, Token {
        tok: Tok::Equal, ..
    }, ..] = tokens
    {
        names.insert(name.clone());
    }

    // A simple Python loop target is available to sentence syntax in its
    // indented body. Destructuring names are safe to remember too; attribute
    // and subscript targets contain no standalone binding token we claim.
    if matches!(tokens.first().map(|token| &token.tok), Some(Tok::For)) {
        for token in tokens.iter().skip(1) {
            if matches!(token.tok, Tok::In) {
                break;
            }
            if let Tok::Name { name } = &token.tok {
                names.insert(name.clone());
            }
        }
    }
}

fn remember_bindings(stmt: &NmeStmt, names: &mut HashSet<String>) {
    match stmt {
        NmeStmt::Ask { target, .. } | NmeStmt::Set { target, .. } => {
            names.insert(target.clone());
        }
        NmeStmt::Times {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::When {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::While {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::ElseIf {
            inline: Some(InlineStmt::Nme(inner)),
            ..
        }
        | NmeStmt::Else {
            inline: Some(InlineStmt::Nme(inner)),
        } => remember_bindings(inner, names),
        _ => {}
    }
}

fn strip_target_particle(word: &str) -> &str {
    for particle in ["에게", "한테", "으로", "로", "을", "를"] {
        if let Some(base) = word.strip_suffix(particle) {
            if !base.is_empty() {
                return base;
            }
        }
    }
    word
}

fn strip_assignment_particle(word: &str) -> Option<&str> {
    for particle in ["은", "는"] {
        if let Some(base) = word.strip_suffix(particle) {
            if !base.is_empty() {
                return Some(base);
            }
        }
    }
    None
}

fn resolve_known_particle<'a>(word: &'a str, known_names: &'a HashSet<String>) -> Option<&'a str> {
    if known_names.contains(word) {
        return Some(word);
    }
    for particle in KOREAN_PARTICLES {
        if let Some(base) = word.strip_suffix(particle) {
            if known_names.contains(base) {
                return Some(&word[..base.len()]);
            }
        }
    }
    None
}

fn is_connector_word(token: &Token) -> bool {
    matches!(token.tok, Tok::And)
        || token_matches_exact(token, &["and", "then", "해서", "그리고", "그러면"])
}

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

fn find_times_colon(tokens: &[Token], mode: MatchMode) -> Option<(usize, Spelling)> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Name { name }
                if (word_matches(name, TIMES_KEYWORD, mode) || name == TIMES_KEYWORD_KO)
                    && depth == 0
                    && index > 0
                    && matches!(
                        tokens.get(index + 1).map(|next| &next.tok),
                        Some(Tok::Colon)
                    ) =>
            {
                return Some((
                    index,
                    if word_matches(name, TIMES_KEYWORD, mode) {
                        Spelling::English
                    } else {
                        Spelling::Korean
                    },
                ));
            }
            _ => {}
        }
    }
    None
}

fn find_condition_colon(source: &str, tokens: &[Token], condition_start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut first = None;
    for (index, token) in tokens.iter().enumerate().skip(condition_start) {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Colon if depth == 0 => {
                first.get_or_insert(index);
                if index > condition_start {
                    let condition = Span::new(
                        tokens[condition_start].span.start,
                        tokens[index - 1].span.end,
                    );
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

fn action_phrase_at(
    tokens: &[Token],
    start: usize,
    expected: &[&str],
    mode: MatchMode,
) -> Option<usize> {
    let available = tokens.len().saturating_sub(start).min(3);
    for consumed in (1..=available).rev() {
        let mut actual = String::new();
        let mut all_words = true;
        for token in &tokens[start..start + consumed] {
            if let Some(word) = token_word(token) {
                actual.push_str(word);
            } else {
                all_words = false;
                break;
            }
        }
        if !all_words {
            continue;
        }
        if expected
            .iter()
            .any(|candidate| word_matches(&actual, candidate, mode))
        {
            return Some(consumed);
        }
    }
    None
}

fn token_matches_exact(token: &Token, expected: &[&str]) -> bool {
    token_word(token).is_some_and(|actual| {
        expected
            .iter()
            .any(|candidate| actual.eq_ignore_ascii_case(candidate))
    })
}

fn word_matches_any(token: &Token, expected: &[&str], mode: MatchMode) -> bool {
    token_word(token).is_some_and(|actual| {
        expected
            .iter()
            .any(|candidate| word_matches(actual, candidate, mode))
    })
}

fn token_word_matches(token: &Token, expected: &str, mode: MatchMode) -> bool {
    token_word(token).is_some_and(|actual| word_matches(actual, expected, mode))
}

fn word_matches(actual: &str, expected: &str, mode: MatchMode) -> bool {
    if actual.eq_ignore_ascii_case(expected) {
        return true;
    }
    if mode == MatchMode::Exact || actual.chars().count() < 2 {
        return false;
    }
    one_typo_away(
        &actual
            .chars()
            .flat_map(char::to_lowercase)
            .collect::<String>(),
        &expected
            .chars()
            .flat_map(char::to_lowercase)
            .collect::<String>(),
    )
}

fn condition_word_matches(actual: &str, expected: &[&str]) -> bool {
    expected.iter().any(|candidate| {
        actual.eq_ignore_ascii_case(candidate)
            || (actual != "the" && actual.chars().count() >= 2 && one_typo_away(actual, candidate))
    })
}

fn output_action_ending(tokens: &[Token], mode: MatchMode) -> Option<(usize, Spelling, usize)> {
    let mut end = tokens.len();
    if tokens.last().is_some_and(is_command_ending) {
        end -= 1;
    }
    let start_at = end.saturating_sub(3);
    for start in start_at..end {
        if let Some((spelling, consumed)) = output_action_at(tokens, start, mode) {
            if start + consumed == end {
                return Some((start, spelling, end));
            }
        }
    }
    None
}

fn trim_suffix_say_value(tokens: &[Token]) -> Vec<Token> {
    let mut value = tokens.to_vec();
    while value
        .last()
        .is_some_and(|token| token_matches_exact(token, &["라고", "이라고", "하고", "을", "를"]))
    {
        value.pop();
    }
    if let Some(last) = value.last_mut() {
        trim_name_token_suffix(last, &["이라고", "라고", "하고", "을", "를"]);
    }
    value
}

fn trim_name_token_suffix(token: &mut Token, suffixes: &[&str]) -> bool {
    let Some(word) = name_word(token) else {
        return false;
    };
    let Some(base) = strip_any_suffix(word, suffixes) else {
        return false;
    };
    let removed = word.len() - base.len();
    token.tok = Tok::Name {
        name: base.to_string(),
    };
    token.span.end = token.span.end.saturating_sub(removed);
    true
}

fn strip_any_suffix<'a>(word: &'a str, suffixes: &[&str]) -> Option<&'a str> {
    let mut ordered = suffixes.to_vec();
    ordered.sort_by_key(|suffix| std::cmp::Reverse(suffix.len()));
    ordered
        .into_iter()
        .find_map(|suffix| word.strip_suffix(suffix).filter(|base| !base.is_empty()))
}

fn literal_token(token: &Token) -> Option<Literal> {
    match &token.tok {
        Tok::True => Some(Literal::True),
        Tok::False => Some(Literal::False),
        Tok::None => Some(Literal::None),
        Tok::Name { name } if name.eq_ignore_ascii_case("true") || name == "참" => {
            Some(Literal::True)
        }
        Tok::Name { name } if name.eq_ignore_ascii_case("false") || name == "거짓" => {
            Some(Literal::False)
        }
        Tok::Name { name }
            if name.eq_ignore_ascii_case("none")
                || name.eq_ignore_ascii_case("null")
                || name == "없음" =>
        {
            Some(Literal::None)
        }
        _ => None,
    }
}

fn is_code_token(token: &Token) -> bool {
    !matches!(token.tok, Tok::Name { .. })
}

fn is_text_token(token: &Token) -> bool {
    matches!(token.tok, Tok::Name { .. } | Tok::String { .. })
}

fn is_command_ending(token: &Token) -> bool {
    matches!(token.tok, Tok::Dot) || token_matches_exact(token, COMMAND_ENDINGS)
}

fn looks_like_python_invocation(tokens: &[Token]) -> bool {
    tokens.len() > 1
        && name_word(&tokens[0]).is_some()
        && matches!(tokens[1].tok, Tok::Lpar | Tok::Dot | Tok::Lsqb)
}

fn looks_like_plain_prose(tokens: &[Token]) -> bool {
    tokens.iter().all(|token| {
        token_word(token).is_some() || is_command_ending(token) || matches!(token.tok, Tok::Comma)
    })
}

fn ambiguous_action_diagnostic(tokens: &[Token]) -> Diagnostic {
    Diagnostic::bilingual(
        "this sentence could mean more than one action",
        "이 문장은 두 가지 동작으로 읽힐 수 있어요",
        span_of(tokens),
    )
    .with_bilingual_hint(
        "spell the action word exactly so there is one clear meaning",
        "동작 단어를 정확히 적어 뜻을 하나로 정해 주세요",
    )
}

fn missing_action_diagnostic(tokens: &[Token]) -> Diagnostic {
    Diagnostic::bilingual(
        "I couldn't find one clear action on this line",
        "이 줄에서 무엇을 할지 찾지 못했어요",
        span_of(tokens),
    )
    .with_bilingual_hint(
        "add an action such as `show`, `ask`, or `repeat`",
        "끝에 `말해줘`를 붙이거나 `물어봐`, `반복해` 같은 동작을 적어 주세요",
    )
}

fn span_of_refs(tokens: &[&Token]) -> Span {
    debug_assert!(!tokens.is_empty());
    Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end)
}

fn attached_korean_times_header(source: &str, tokens: &[Token]) -> Option<(Code, usize)> {
    let [Token {
        tok: Tok::Name { name },
        span,
    }, Token {
        tok: Tok::Colon, ..
    }, ..] = tokens
    else {
        return None;
    };
    let count = name.strip_suffix(TIMES_KEYWORD_KO)?;
    if count.is_empty() {
        return None;
    }
    let count_span = Span::new(span.start, span.end - TIMES_KEYWORD_KO.len());
    is_valid_python_expression(&source[count_span.start..count_span.end])
        .then_some((Code::Source(count_span), 1))
}

fn attached_korean_times_sentence(source: &str, tokens: &[Token]) -> Option<Code> {
    let [Token {
        tok: Tok::Name { name },
        span,
    }] = tokens
    else {
        return None;
    };
    let count = name.strip_suffix(TIMES_KEYWORD_KO)?;
    if count.is_empty() {
        return None;
    }
    let count_span = Span::new(span.start, span.end - TIMES_KEYWORD_KO.len());
    is_valid_python_expression(&source[count_span.start..count_span.end])
        .then_some(Code::Source(count_span))
}

fn one_typo_away(actual: &str, expected: &str) -> bool {
    if actual == expected {
        return true;
    }
    let left: Vec<char> = actual.chars().collect();
    let right: Vec<char> = expected.chars().collect();
    if left.len().abs_diff(right.len()) > 1 {
        return false;
    }
    if left.len() == right.len() {
        let differences: Vec<usize> = left
            .iter()
            .zip(&right)
            .enumerate()
            .filter_map(|(index, (a, b))| (a != b).then_some(index))
            .collect();
        return differences.len() == 1
            || (differences.len() == 2
                && differences[1] == differences[0] + 1
                && left[differences[0]] == right[differences[1]]
                && left[differences[1]] == right[differences[0]]);
    }
    let (shorter, longer) = if left.len() < right.len() {
        (&left, &right)
    } else {
        (&right, &left)
    };
    let (mut short_at, mut long_at, mut skipped) = (0, 0, false);
    while short_at < shorter.len() && long_at < longer.len() {
        if shorter[short_at] == longer[long_at] {
            short_at += 1;
            long_at += 1;
        } else if skipped {
            return false;
        } else {
            skipped = true;
            long_at += 1;
        }
    }
    true
}

fn token_word(token: &Token) -> Option<&str> {
    match &token.tok {
        Tok::Name { name } => Some(name),
        Tok::If => Some("if"),
        Tok::While => Some("while"),
        Tok::Break => Some("break"),
        Tok::Elif => Some("elif"),
        Tok::Else => Some("else"),
        Tok::Is => Some("is"),
        Tok::And => Some("and"),
        Tok::Or => Some("or"),
        Tok::As => Some("as"),
        Tok::From => Some("from"),
        Tok::Import => Some("import"),
        _ => None,
    }
}

fn name_word(token: &Token) -> Option<&str> {
    match &token.tok {
        Tok::Name { name } => Some(name),
        _ => None,
    }
}

fn token_is_exact_name(token: &Token, expected: &str) -> bool {
    matches!(&token.tok, Tok::Name { name } if name == expected)
}

fn is_sentence_punctuation(token: &Token) -> bool {
    matches!(name_word(token), Some("?" | "!"))
}

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

fn looks_like_broken_expression(tokens: &[Token]) -> bool {
    tokens.iter().any(|token| {
        matches!(
            token.tok,
            Tok::Plus
                | Tok::Minus
                | Tok::Star
                | Tok::DoubleStar
                | Tok::Slash
                | Tok::DoubleSlash
                | Tok::Percent
                | Tok::Lpar
                | Tok::Rpar
                | Tok::Lsqb
                | Tok::Rsqb
                | Tok::Lbrace
                | Tok::Rbrace
                | Tok::EqEqual
                | Tok::NotEqual
                | Tok::Less
                | Tok::Greater
                | Tok::LessEqual
                | Tok::GreaterEqual
        )
    })
}

fn span_of(tokens: &[Token]) -> Span {
    debug_assert!(!tokens.is_empty());
    Span::new(tokens[0].span.start, tokens[tokens.len() - 1].span.end)
}

fn token_text<'a>(source: &'a str, tokens: &[Token]) -> &'a str {
    let span = span_of(tokens);
    &source[span.start..span.end]
}

fn is_valid_python_statement(text: &str) -> bool {
    parse_python(text, Mode::Module, "<nme>").is_ok()
}

fn is_valid_python_header(text: &str) -> bool {
    parse_python(&format!("{text}\n    pass"), Mode::Module, "<nme>").is_ok()
}

fn is_valid_python_expression(text: &str) -> bool {
    parse_python(text, Mode::Expression, "<nme>").is_ok()
}

#[cfg(test)]
mod tests {
    use super::one_typo_away;

    #[test]
    fn accepts_one_edit_or_adjacent_transposition() {
        assert!(one_typo_away("말헤", "말해"));
        assert!(one_typo_away("물어바", "물어봐"));
        assert!(one_typo_away("repaet", "repeat"));
        assert!(!one_typo_away("completely", "repeat"));
    }
}
