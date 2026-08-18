//! Recognizes the advanced, beginner, and sentence levels of NME.
//!
//! A real Python parse always runs first. Valid Python therefore remains
//! byte-identical even when a Python name resembles an easier NME phrase.
//! Easier forms are matched only from lexer tokens; strings and comments are
//! never searched or rewritten as text.

use std::collections::{HashMap, HashSet};

use rustpython_parser::{parse as parse_python, Mode, Tok};

use crate::diagnostics::{Diagnostic, DiagnosticCode, Span};
use crate::lexer::{LogicalLine, Token};
use crate::syntax::{
    BundledModuleId, Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, Literal,
    LogicalOp, ModuleVersion, NmeLine, NmeStmt, Spelling, TextPart, TextTemplate, UpdateOp, Value,
    COOLDOWN_PREFIX, ELAPSED_PYTHON, FILE_MODULE, FILE_MODULE_KO, FILE_READ_WORDS_EN,
    FILE_READ_WORDS_KO, FILE_WRITE_WORDS_EN, FILE_WRITE_WORDS_KO, RANDOM_MODULE, RANDOM_MODULE_KO,
    SAY_KEYWORD, SAY_KEYWORD_KO, SAY_WORDS_EN, TIMER_NAME, TIMES_KEYWORD, TIMES_KEYWORD_KO,
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
    "해줘",
    "해주세요",
    "읽어줘",
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
const ELSE_WORDS_KO: &[&str] = &[
    "아니면",
    "그렇지않으면",
    "아니면만약",
    "아니면만약에",
    "그렇지않으면만약",
    "그렇지않으면만약에",
];
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
const SET_WORDS_KO: &[&str] = &["저장", "저장해", "기억해", "기억해줘", "설정", "설정해"];
const UPDATE_ADD_WORDS_EN: &[&str] = &["add", "increase", "increment", "plus"];
const UPDATE_ADD_WORDS_KO: &[&str] = &["더해", "더해줘", "올려", "올려줘", "늘려", "늘려줘"];
const UPDATE_SUBTRACT_WORDS_EN: &[&str] = &["subtract", "decrease", "decrement", "minus", "remove"];
const UPDATE_SUBTRACT_WORDS_KO: &[&str] = &["빼", "빼줘", "내려", "내려줘", "줄여", "줄여줘"];
// `times` is deliberately absent from the English multiply words: it is the
// repeat marker, and `score times 2` must keep meaning "repeat".
const UPDATE_MULTIPLY_WORDS_EN: &[&str] = &["multiply", "multiplied"];
const UPDATE_MULTIPLY_WORDS_KO: &[&str] = &["곱해", "곱해줘", "곱하기해"];
const UPDATE_DIVIDE_WORDS_EN: &[&str] = &["divide", "divided"];
const UPDATE_DIVIDE_WORDS_KO: &[&str] = &["나눠", "나눠줘", "나누어줘"];
/// Particles that may be attached to the number in a value change.
const UPDATE_AMOUNT_PARTICLES_KO: &[&str] = &["으로", "로", "만큼", "씩", "을", "를"];
const WAIT_WORDS_EN: &[&str] = &["wait", "pause", "sleep"];
const WAIT_WORDS_KO: &[&str] = &[
    "기다려",
    "기다려줘",
    "기다리세요",
    "기다려주세요",
    "쉬어",
    "쉬어줘",
    "쉬세요",
];
/// Time units dropped before the wait amount is read as an expression. The
/// Korean ones are also stripped when written attached, as in `3초`.
const SECOND_WORDS_EN: &[&str] = &["second", "seconds", "sec", "secs"];
const SECOND_WORDS_KO: &[&str] = &["초동안", "초간", "초만", "초"];
const WAIT_FILLER_WORDS: &[&str] = &["for", "about", "동안", "간"];
const CONTINUE_WORDS_EN: &[&str] = &["skip", "skipthis", "skipit", "nextone"];
const CONTINUE_WORDS_KO: &[&str] = &["건너뛰어", "건너뛰어줘", "건너뛰기", "건너뛰자", "넘어가", "넘어가줘"];
const APPEND_WORDS_EN: &[&str] = &["append", "push"];
const APPEND_WORDS_KO: &[&str] = &["넣어", "넣어줘", "추가해", "추가해줘", "붙여", "붙여줘"];
const APPEND_CONNECTORS_EN: &[&str] = &["to", "into", "onto"];
/// Particles marking the list a value is being put into (`친구들에 민수 넣어`).
const APPEND_TARGET_PARTICLES_KO: &[&str] = &["에다가", "에다", "에", "한테", "에게"];
const LIST_WORDS_EN: &[&str] = &["list"];
const LIST_WORDS_KO: &[&str] = &["목록", "리스트"];
/// `say slowly Hello` / `천천히 말해줘 안녕` — text told one character at a time.
const SLOW_WORDS_EN: &[&str] = &["slowly"];
const SLOW_WORDS_KO: &[&str] = &["천천히"];
/// The intensity word in `say very slowly` / `아주 천천히 말해줘`.
const VERY_WORDS_EN: &[&str] = &["very"];
const VERY_WORDS_KO: &[&str] = &["아주"];
/// The marker before an explicit pause: `slowly every 3 seconds` / `3초씩`.
const SLOW_EVERY_WORDS_EN: &[&str] = &["every"];
const SLOW_EVERY_WORDS_KO: &[&str] = &["초씩"];
/// Seconds between characters for the plain and the very slow spelling.
const SLOW_SECONDS: &str = "0.04";
const VERY_SLOW_SECONDS: &str = "0.12";
/// `clear the screen` / `화면 지워`.
const CLEAR_SCREEN_WORDS_EN: &[&str] = &["clear"];
const CLEAR_SCREEN_WORDS_KO: &[&str] = &["화면"];
const CLEAR_SCREEN_ACTIONS_EN: &[&str] = &["screen"];
const CLEAR_SCREEN_ACTIONS_KO: &[&str] = &["지워", "지워줘", "비워", "비워줘"];
/// `draw a line` / `줄 그어`.
const DRAW_LINE_WORDS_EN: &[&str] = &["draw"];
const DRAW_LINE_WORDS_KO: &[&str] = &["줄", "가로줄"];
const DRAW_LINE_ACTIONS_EN: &[&str] = &["line"];
const DRAW_LINE_ACTIONS_KO: &[&str] = &["그어", "그어줘"];
/// `say in a box Hello` / `상자로 말해줘 안녕`.
const BOX_WORDS_EN: &[&str] = &["box"];
const BOX_WORDS_KO: &[&str] = &["상자로"];
/// `say in the middle Hello` / `가운데 말해줘 안녕`.
const MIDDLE_WORDS_EN: &[&str] = &["middle"];
const MIDDLE_WORDS_KO: &[&str] = &["가운데"];
/// `start the timer` / `시간 재기 시작해`. The Korean spellings are written
/// joined, exactly like the other multi-word Korean actions, because the
/// matcher glues neighbouring words back together before comparing.
const START_TIMER_WORDS_EN: &[&str] = &["start"];
const START_TIMER_WORDS_KO: &[&str] = &["시간재기시작해", "시간재기시작"];
const TIMER_WORDS_EN: &[&str] = &["timer"];
/// `put door on cooldown for 3 seconds` / `문 쿨타임 3초 걸어`.
const COOLDOWN_WORDS_EN: &[&str] = &["cooldown"];
const COOLDOWN_WORDS_KO: &[&str] = &["쿨타임", "쿨타임을", "쿨타임은", "쿨타임이"];
const COOLDOWN_SET_WORDS_EN: &[&str] = &["put"];
const COOLDOWN_SET_WORDS_KO: &[&str] = &["걸어", "걸어줘"];
/// `when door is ready` / `문 쿨타임이 끝났으면`.
const COOLDOWN_READY_WORDS_EN: &[&str] = &["ready"];
const COOLDOWN_READY_WORDS_KO: &[&str] = &["끝났으면"];
/// `when door is on cooldown` / `문 쿨타임이 남았으면`.
const COOLDOWN_BUSY_WORDS_KO: &[&str] = &["남았으면"];
/// `문 쿨타임 끝날때까지 기다려` — the Korean wait spelling, written joined.
const COOLDOWN_UNTIL_WORDS_KO: &[&str] = &["끝날때까지"];
/// `elapsed` / `잰시간` — the stopwatch reading, usable wherever a value is.
const ELAPSED_WORDS_EN: &[&str] = &["elapsed"];
const ELAPSED_WORDS_KO: &[&str] = &["잰시간", "걸린시간"];
const EACH_WORDS_EN: &[&str] = &["each", "every"];
/// Korean loop-variable ending in `이름들의 이름마다 반복해`.
const EACH_SUFFIX_KO: &str = "마다";
/// Particles that may sit between the collection and the loop variable.
const EACH_CONTAINER_PARTICLES_KO: &[&str] = &["가운데", "안의", "속의", "에서", "중", "의"];
const SENTENCE_FILLERS: &[&str] = &["please", "좀", "혹시", "제발"];
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
#[allow(clippy::too_many_lines)]
pub fn parse_program(
    source: &str,
    lines: &[LogicalLine],
) -> Result<ParsedProgram, Vec<Diagnostic>> {
    let mut found = Vec::new();
    let mut problems = Vec::new();
    let mut bindings = BindingEnv::new();
    let mut virtual_indents = vec![0; lines.len()];
    let mut blocks = Vec::<ExplicitBlock>::new();
    // Tracks whether any NME statement has been seen. A lone `end`/`끝`
    // after that is almost always a leftover block terminator, so it gets a
    // friendly diagnostic instead of silently staying Python (where it would
    // fail at runtime). Pure-Python files still keep `end` byte-identical.
    let mut saw_nme = false;
    // Python compound headers normally get their body indentation from the
    // source.  Inside an indentation-free NME block, however, a learner may
    // write a normal Python header (`if x:`) and then continue with the body
    // at the same logical level.  Keep those headers separately so ordinary
    // Python receives the same virtual indentation that NME statements do.
    // The source indent is retained as the unambiguous signal for a real
    // Python dedent; explicit NME `break`/`end`/branch lines also close a
    // flat Python suite when they are at the header's level.
    let mut python_header_indents = Vec::<(usize, bool)>::new();
    // Top-level Python headers are intentionally not virtualized, but their
    // loop kind still matters when an NME inline body appears in their
    // physically indented suite. This keeps `for ...:`/`while ...:` bodies
    // valid while allowing inline `break` diagnostics in ordinary Python
    // conditional suites.
    let mut top_level_python_loop_indents = Vec::<usize>::new();
    // `except*` suites have one Python-specific control-flow restriction:
    // `break`, `continue`, and `return` are not allowed in their bodies.
    // Track their physical header indentation separately from ordinary
    // Python headers so nested functions and classes still use BindingEnv.
    let mut python_except_star_indents = Vec::<(usize, usize)>::new();
    let mut python_try_indents = Vec::<usize>::new();
    let mut async_function_contexts = Vec::<AsyncFunctionContext>::new();
    let mut completed_async_functions = Vec::<AsyncFunctionContext>::new();
    let mut python_declaration_contexts = vec![PythonDeclarationContext {
        body_scope_depth: 0,
        seen_names: HashSet::new(),
        annotation_targets: HashSet::new(),
        declarations: HashMap::new(),
    }];

    for (index, line) in lines.iter().enumerate() {
        let is_end = exact_end(line.tokens.as_slice());
        let is_break = exact_break(line.tokens.as_slice());
        let is_continue = exact_continue(line.tokens.as_slice());
        let branch_shape = branch_shape(line.tokens.as_slice());

        // An indented-suite sentence block (whose first body line was
        // physically indented) may end at the physical dedent, like ordinary
        // Python, but only when the remaining `end`/`끝` lines cannot close
        // the nested reading anyway. That keeps every previously valid
        // program unchanged: a nested header with enough closing `end`s stays
        // nested, while an ambiguous indented block followed by a flat block
        // with too few `end`s becomes a sibling instead of a missing-end
        // error. A flat statement at the block's own level keeps the suite
        // flat from there on (only `end` closes it), so mixed indented+flat
        // bodies keep working. Explicit closers (`end`, `break`, branches)
        // are handled by their own paths below.
        if !(is_end.is_some() || is_break || is_continue || branch_shape.is_some()) {
            let line_is_header = is_header_shape(&line.tokens);
            let remaining_ends = count_remaining_ends(lines, index);
            loop {
                let open = blocks.len();
                let Some(close_on_dedent) = blocks.last().and_then(|block| block.close_on_dedent())
                else {
                    break;
                };
                if line.indent == close_on_dedent && line_is_header && open >= remaining_ends {
                    blocks.pop();
                } else if line.indent == close_on_dedent {
                    if let Some(top) = blocks.last_mut() {
                        top.clear_close_on_dedent();
                    }
                    break;
                } else {
                    break;
                }
            }
        }

        let depth = blocks.len();
        if depth == 0 {
            python_header_indents.clear();
        }
        while top_level_python_loop_indents
            .last()
            .is_some_and(|header_indent| line.indent <= *header_indent)
        {
            top_level_python_loop_indents.pop();
        }
        let closes_flat_python_suite =
            is_end.is_some() || is_break || is_continue || branch_shape.is_some();
        while python_header_indents.last().is_some_and(|header_indent| {
            line.indent < header_indent.0
                || (closes_flat_python_suite && line.indent <= header_indent.0)
        }) {
            python_header_indents.pop();
        }
        while python_try_indents
            .last()
            .is_some_and(|header_indent| line.indent < *header_indent)
        {
            python_try_indents.pop();
        }
        if python_try_indents.last().is_some_and(|header_indent| {
            line.indent == *header_indent
                && !is_python_try_header(&line.tokens)
                && !is_python_try_clause_header(&line.tokens)
        }) {
            python_try_indents.pop();
        }
        while python_except_star_indents
            .last()
            .is_some_and(|(header_indent, _)| line.indent <= *header_indent)
        {
            python_except_star_indents.pop();
        }
        // Keep Python's compatibility rule strict at the top level: an
        // indented Python body is still the user's responsibility unless an
        // explicit NME block is already open. This prevents a malformed
        // ordinary `if` from being repaired silently by NME.
        let python_depth = if depth > 0 {
            python_header_indents.len()
        } else {
            0
        };

        // A logical line inside an explicit block receives a virtual level.
        // Physical indentation is still retained, so nested Python remains
        // possible and ordinary Python lines can be mixed freely.
        let branch_depth = branch_shape.is_some().then(|| depth.saturating_sub(1));
        let base_line_depth = if is_end.is_some() || branch_depth.is_some() {
            branch_depth.unwrap_or_else(|| depth.saturating_sub(1))
        } else {
            depth
        };
        let line_depth = base_line_depth + python_depth;
        virtual_indents[index] = line_depth.saturating_sub(line.indent);
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
        let has_colon = line
            .tokens
            .iter()
            .any(|token| matches!(token.tok, Tok::Colon));
        // A colon normally means advanced Python, so keep its indentation
        // rules.  The compact NME repeat header (`3 times:` / `3번:`) is not
        // valid Python, though, and may use the same explicit `end`/`끝`
        // terminator as sentence blocks without forcing the learner to indent.
        let nme_colon_header =
            has_colon && !is_valid_python_header(token_text(source, &line.tokens));
        let flat_body_follows = has_next_line && unindented_next_line;
        let force_suite = is_header_shape(&line.tokens)
            && ((!has_colon && (has_future_end(lines, index) || flat_body_follows))
                // A colon-bearing beginner header only needs virtual
                // indentation when its body is actually flat and an
                // explicit terminator exists. If the next line is
                // physically indented, keep the ordinary suite semantics
                // and do not claim a later `end` for this header.
                || (nme_colon_header && has_future_end(lines, index) && flat_body_follows));
        let next_indent = force_suite.then_some(parse_line.indent + 1).or(next_indent);

        bindings.enter_line(parse_line.indent);
        let python_scope_depth = bindings.python_scope_depth();
        while async_function_contexts
            .last()
            .is_some_and(|context| context.body_scope_depth > python_scope_depth)
        {
            if let Some(context) = async_function_contexts.pop() {
                completed_async_functions.push(context);
            }
        }
        while python_declaration_contexts
            .last()
            .is_some_and(|context| context.body_scope_depth > python_scope_depth)
        {
            python_declaration_contexts.pop();
        }
        let inside_python_except_star = python_except_star_indents
            .last()
            .is_some_and(|(_, scope_depth)| *scope_depth == bindings.python_scope_depth());
        let known_names = bindings.visible_names();
        let block = BlockCtx::TopLevel {
            line: &parse_line,
            next_indent,
        };

        // `end` and a bare `break` are valid Python-shaped words in a few
        // contexts, so an already-open explicit block claims them before
        // Python-wins. Outside a block, a stray `end` after any NME
        // statement is reported (it would only fail at runtime as Python);
        // in a pure-Python file `end`/`끝` remain untouched identifiers.
        let direct_stmt = if is_end.is_some() && depth > 0 {
            Some(Ok(Some(NmeStmt::End)))
        } else if is_end.is_some() && saw_nme {
            Some(Err(unmatched_end_diagnostic(line.span)))
        } else if is_break
            && (depth > 0
                || (line.indent == 0
                    && action_phrase_at(&line.tokens, 0, BREAK_WORDS_EN, MatchMode::Exact)
                        .is_some())
                || (is_korean_break_alias(&line.tokens)
                    && !is_valid_python_statement(token_text(source, &line.tokens))))
        {
            Some(Ok(Some(NmeStmt::Break)))
        } else if is_continue && depth > 0 {
            // `skip` and `건너뛰어` are ordinary Python names on their own, so
            // like `break` they are only read as NME inside an NME block.
            Some(Ok(Some(NmeStmt::Continue)))
        } else if branch_shape.is_some()
            && depth == 0
            && !line
                .tokens
                .iter()
                .any(|token| matches!(token.tok, Tok::Equal | Tok::Colon))
            && !is_valid_python_statement(token_text(source, &line.tokens))
        {
            Some(Err(branch_without_condition_diagnostic(line.span)))
        } else if branch_shape.is_some()
            && (depth > 0 || is_korean_branch_alias(&line.tokens))
            && !line
                .tokens
                .iter()
                .any(|token| matches!(token.tok, Tok::Equal | Tok::Colon))
            && (!is_valid_python_statement(token_text(source, &line.tokens))
                || (depth > 0 && line.tokens.len() == 1))
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
                saw_nme = true;
                let inside_loop = blocks
                    .iter()
                    .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                    || python_header_indents.iter().any(|(_, is_loop)| *is_loop)
                    || !top_level_python_loop_indents.is_empty();
                if inline_break_is_outside_loop(&stmt, source, inside_loop) {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_continue_is_outside_loop(&stmt, &line.tokens, inside_loop) {
                    problems.push(continue_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_return_is_outside_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_function(),
                ) {
                    problems.push(return_outside_function_diagnostic(line.span));
                    continue;
                }
                if inline_yield_inside_comprehension(&stmt, &line.tokens) {
                    problems.push(yield_inside_comprehension_diagnostic(line.span));
                    continue;
                }
                if inline_async_comprehension_outside_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(async_comprehension_outside_async_function_diagnostic(
                        line.span,
                    ));
                    continue;
                }
                remember_async_generator_context(
                    &mut async_function_contexts,
                    &line.tokens,
                    python_scope_depth,
                    line.span,
                );
                if inline_yield_is_outside_function(&stmt, &line.tokens, bindings.inside_function())
                {
                    problems.push(yield_outside_function_diagnostic(line.span));
                    continue;
                }
                if inline_await_is_outside_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(await_outside_async_function_diagnostic(line.span));
                    continue;
                }
                if inline_yield_from_is_in_async_function(
                    &stmt,
                    &line.tokens,
                    bindings.inside_async_function(),
                ) {
                    problems.push(yield_from_async_function_diagnostic(line.span));
                    continue;
                }
                if inside_python_except_star
                    && (matches!(stmt, NmeStmt::Break)
                        || inline_except_star_control_flow(&stmt, &line.tokens))
                {
                    problems.push(except_star_control_flow_diagnostic(line.span));
                    continue;
                }
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
                        .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if let Some(branch) = &branch_shape {
                    if !validate_branch(branch, &mut blocks, line.span, &mut problems) {
                        continue;
                    }
                }
                let base_target_indent = if matches!(stmt, NmeStmt::End) || branch_shape.is_some() {
                    base_line_depth
                } else {
                    depth
                };
                let virtual_indent =
                    (base_target_indent + python_depth).saturating_sub(line.indent);
                if reads_elapsed(&stmt) && !known_names.contains(TIMER_NAME) {
                    problems.push(timer_not_started_diagnostic(line.span));
                    continue;
                }
                bindings.remember_nme(&stmt);
                found.push(NmeLine {
                    line_index: index,
                    span: line.span,
                    stmt,
                    virtual_indent,
                });
                if let Some(
                    NmeStmt::Times { inline: None, .. }
                    | NmeStmt::ForEach { inline: None, .. }
                    | NmeStmt::When { inline: None, .. }
                    | NmeStmt::While { inline: None, .. },
                ) = found.last().map(|line| &line.stmt)
                {
                    if force_suite {
                        let is_loop = matches!(
                            found.last().map(|line| &line.stmt),
                            Some(
                                NmeStmt::While { .. }
                                    | NmeStmt::Times { .. }
                                    | NmeStmt::ForEach { .. }
                            )
                        );
                        bindings.push_explicit_scope(parse_line.indent + 1);
                        let close_on_dedent = (!flat_body_follows).then_some(line.indent);
                        blocks.push(if is_loop {
                            ExplicitBlock::Loop { close_on_dedent }
                        } else {
                            ExplicitBlock::Conditional {
                                else_seen: false,
                                close_on_dedent,
                            }
                        });
                    }
                }
            }
            Ok(None) => {
                let valid_python_header = is_valid_python_header(token_text(source, &line.tokens));
                let python_loop_header = is_python_loop_header(&line.tokens);
                let inline_python_scope_body = python_inline_suite_body(&line.tokens);
                let inline_python_function_body = python_inline_function_body(&line.tokens);
                let inline_python_class_body =
                    inline_python_scope_body.filter(|_| is_python_class_header(&line.tokens));
                let (context_tokens, inside_function, inside_async_function) =
                    if let Some(body) = inline_python_function_body {
                        (body, true, is_python_async_function_header(&line.tokens))
                    } else if let Some(body) = inline_python_scope_body {
                        (body, false, false)
                    } else {
                        (
                            line.tokens.as_slice(),
                            bindings.inside_function(),
                            bindings.inside_async_function(),
                        )
                    };
                let inline_python_scope = inline_python_scope_body.is_some();
                let inside_inline_python_class = inline_python_class_body.is_some();
                let contextual_function = inside_function && !inside_inline_python_class;
                let has_enclosing_function = if inline_python_scope {
                    bindings.has_function_scope()
                } else {
                    bindings.has_enclosing_function()
                };
                if let Some(kind) = remember_python_declaration_context(
                    &mut python_declaration_contexts,
                    &line.tokens,
                    python_scope_depth,
                ) {
                    problems.push(python_declaration_conflict_diagnostic(kind, line.span));
                    continue;
                }
                bindings.remember_python(&line.tokens, parse_line.indent);
                if depth > 0 && valid_python_header {
                    python_header_indents.push((line.indent, python_loop_header));
                } else if depth == 0 && valid_python_header && python_loop_header {
                    top_level_python_loop_indents.push(line.indent);
                }
                if is_python_async_for_header(&line.tokens) && !bindings.inside_async_function() {
                    problems.push(async_for_outside_async_function_diagnostic(line.span));
                }
                if is_python_async_with_header(&line.tokens) && !bindings.inside_async_function() {
                    problems.push(async_with_outside_async_function_diagnostic(line.span));
                }
                if contains_python_nonlocal(context_tokens) && !has_enclosing_function {
                    problems.push(nonlocal_outside_function_diagnostic(line.span));
                }
                if is_python_import_star_line(context_tokens)
                    && is_valid_python_statement(token_text(source, context_tokens))
                    && (inline_python_scope || bindings.inside_non_module_scope())
                {
                    problems.push(import_star_outside_module_diagnostic(line.span));
                }
                if is_python_return_line(context_tokens) && !contextual_function {
                    problems.push(return_outside_function_diagnostic(line.span));
                    continue;
                }
                let inside_loop = blocks
                    .iter()
                    .any(|block| matches!(block, ExplicitBlock::Loop { .. }))
                    || python_header_indents.iter().any(|(_, is_loop)| *is_loop)
                    || !top_level_python_loop_indents.is_empty();
                if is_python_continue_line(context_tokens) && (!inside_loop || inline_python_scope)
                {
                    problems.push(continue_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inline_python_scope && is_python_break_line(context_tokens) {
                    problems.push(break_outside_loop_diagnostic(line.span));
                    continue;
                }
                if inside_python_except_star && is_python_except_star_control_line(&line.tokens) {
                    problems.push(except_star_control_flow_diagnostic(line.span));
                    continue;
                }
                if contains_yield_inside_comprehension(context_tokens) {
                    problems.push(yield_inside_comprehension_diagnostic(line.span));
                    continue;
                }
                if contains_async_comprehension_outside_async_function(
                    context_tokens,
                    inside_async_function,
                ) {
                    problems.push(async_comprehension_outside_async_function_diagnostic(
                        line.span,
                    ));
                    continue;
                }
                if let Some(body) = inline_python_function_body {
                    let has_direct_yield = contains_yield_outside_lambda(body)
                        && !contains_yield_inside_comprehension(body);
                    if is_python_async_function_header(&line.tokens)
                        && has_direct_yield
                        && contains_return_with_value(body)
                    {
                        problems.push(return_value_in_async_generator_diagnostic(line.span));
                        continue;
                    }
                } else if !inline_python_scope {
                    remember_async_generator_context(
                        &mut async_function_contexts,
                        &line.tokens,
                        python_scope_depth,
                        line.span,
                    );
                }
                if contains_yield_outside_lambda(context_tokens) && !inside_function {
                    problems.push(yield_outside_function_diagnostic(line.span));
                    continue;
                }
                if contains_invalid_await(context_tokens, inside_async_function) {
                    problems.push(await_outside_async_function_diagnostic(line.span));
                    continue;
                }
                if contains_yield_from_outside_lambda(context_tokens) && inside_async_function {
                    problems.push(yield_from_async_function_diagnostic(line.span));
                }
                if is_python_try_header(&line.tokens) {
                    python_try_indents.push(line.indent);
                }
                if is_python_except_star_header(&line.tokens)
                    && python_try_indents
                        .last()
                        .is_some_and(|header_indent| *header_indent == line.indent)
                {
                    python_except_star_indents.push((line.indent, bindings.python_scope_depth()));
                }
            }
            Err(problem) => problems.push(problem),
        }
    }

    completed_async_functions.extend(async_function_contexts);
    for context in completed_async_functions {
        if context.has_yield {
            for span in context.return_value_spans {
                problems.push(return_value_in_async_generator_diagnostic(span));
            }
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
    Loop {
        /// When the block's body started physically indented, the suite
        /// follows ordinary Python dedent rules: a later line at or above
        /// the header level closes it. Flat bodies only close on `end`.
        close_on_dedent: Option<usize>,
    },
    Conditional {
        else_seen: bool,
        close_on_dedent: Option<usize>,
    },
}

impl ExplicitBlock {
    fn close_on_dedent(self) -> Option<usize> {
        match self {
            ExplicitBlock::Loop { close_on_dedent }
            | ExplicitBlock::Conditional {
                close_on_dedent, ..
            } => close_on_dedent,
        }
    }

    /// A flat statement at the block's own level means the suite is flat from
    /// there on, so only an explicit `end` can close it again.
    fn clear_close_on_dedent(&mut self) {
        match self {
            ExplicitBlock::Loop { close_on_dedent }
            | ExplicitBlock::Conditional {
                close_on_dedent, ..
            } => {
                *close_on_dedent = None;
            }
        }
    }
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
    consumed.is_some_and(|consumed| tokens[consumed..].iter().all(is_command_ending))
}

fn exact_continue(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    let consumed = action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, MatchMode::Exact)
        .or_else(|| action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, MatchMode::Exact));
    consumed.is_some_and(|consumed| tokens[consumed..].iter().all(is_command_ending))
}

fn branch_shape(tokens: &[Token]) -> Option<BranchShape> {
    if tokens.is_empty() {
        return None;
    }
    if matches!(tokens[0].tok, Tok::Elif)
        || token_matches_exact(&tokens[0], &["elif"])
        || token_matches_exact(
            &tokens[0],
            &[
                "아니면만약",
                "아니면만약에",
                "그렇지않으면만약",
                "그렇지않으면만약에",
            ],
        )
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
    !tokens.is_empty() && action_phrase_at(tokens, 0, ELSE_WORDS_KO, MatchMode::Exact).is_some()
}

/// `skip` / `건너뛰어` on its own, as the one-line body of an NME block.
fn is_skip_alias(tokens: &[Token]) -> bool {
    action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_korean_break_alias(tokens: &[Token]) -> bool {
    action_phrase_at(tokens, 0, BREAK_WORDS_KO, MatchMode::Exact).is_some()
}

fn is_header_shape(tokens: &[Token]) -> bool {
    if tokens.is_empty() {
        return false;
    }
    when_action_at(tokens, 0, MatchMode::Exact).is_some()
        || english_for_each_start(tokens, MatchMode::Exact).is_some()
        || korean_for_each_shape(tokens)
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
        || subject_condition_shape(tokens)
}

fn has_future_end(lines: &[LogicalLine], index: usize) -> bool {
    lines[index + 1..]
        .iter()
        .any(|line| exact_end(&line.tokens).is_some())
}

/// Number of whole-statement `end`/`끝` lines after `index`. Used to decide
/// whether a dedented header can only be a sibling block (when the remaining
/// `end`s are not enough to close the nested reading anyway).
fn count_remaining_ends(lines: &[LogicalLine], index: usize) -> usize {
    lines[index + 1..]
        .iter()
        .filter(|line| exact_end(&line.tokens).is_some())
        .count()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
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
    let ExplicitBlock::Conditional { else_seen, .. } = top else {
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
        DiagnosticCode::StrayEnd,
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
        DiagnosticCode::BreakOutsideLoop,
        "`break` can only be used inside a loop",
        "`멈춰`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside `while ... end`, `repeat ... end`, or a Python `for`/`while` loop",
        "`동안 ... 끝`, `반복 ... 끝`, 또는 Python `for`/`while` 반복문 안에 넣어 주세요",
    )
}

fn continue_outside_loop_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ContinueOutsideLoop,
        "`continue` can only be used inside a loop",
        "`continue`는 반복문 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside `while`, `repeat`, or a Python `for`/`while` loop, or remove it",
        "`while`, `repeat`, 또는 Python `for`/`while` 반복문 안에 넣거나 지워 주세요",
    )
}

fn return_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ReturnOutsideFunction,
        "`return` can only be used inside a function",
        "`return`은 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside a `def` function, or remove it",
        "`def` 함수 안에 넣거나 지워 주세요",
    )
}

fn yield_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldOutsideFunction,
        "`yield` can only be used inside a function",
        "`yield`는 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside a `def` or `async def` function, or remove it",
        "`def` 또는 `async def` 함수 안에 넣거나 지워 주세요",
    )
}

fn await_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AwaitOutsideAsyncFunction,
        "`await` can only be used inside an async function",
        "`await`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or remove it",
        "`async def` 함수 안에 넣거나 지워 주세요",
    )
}

fn yield_from_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldFromAsyncFunction,
        "`yield from` cannot be used inside an async function",
        "비동기 함수 안에서는 `yield from`을 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "use `async for` to yield values from an async source, or use a normal `def` generator",
        "비동기 원천의 값을 내보내려면 `async for`를 쓰거나 일반 `def` 제너레이터를 사용해 주세요",
    )
}

fn async_for_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncForOutsideAsyncFunction,
        "`async for` can only be used inside an async function",
        "`async for`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or use an ordinary `for` loop",
        "`async def` 함수 안에 넣거나 일반 `for` 반복문을 사용해 주세요",
    )
}

fn async_with_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncWithOutsideAsyncFunction,
        "`async with` can only be used inside an async function",
        "`async with`는 비동기 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it inside an `async def` function, or use an ordinary `with` block",
        "`async def` 함수 안에 넣거나 일반 `with` 블록을 사용해 주세요",
    )
}

fn nonlocal_outside_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::NonlocalOutsideFunction,
        "`nonlocal` can only be used inside a nested function",
        "`nonlocal`은 중첩 함수 안에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "put it in a nested function or class under another function, or remove it",
        "다른 함수 아래의 중첩 함수나 클래스에 넣거나 지워 주세요",
    )
}

fn import_star_outside_module_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ImportStarOutsideModule,
        "`from ... import *` can only be used at module scope",
        "`from ... import *`은 모듈 범위에서만 쓸 수 있어요",
        span,
    )
    .with_bilingual_hint(
        "import the names explicitly here, or move the star import to the module level",
        "여기서는 이름을 명시적으로 import하거나 별표 import를 모듈 수준으로 옮겨 주세요",
    )
}

fn except_star_control_flow_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ControlFlowInExceptStar,
        "`break`, `continue`, and `return` cannot be used inside an `except*` block",
        "`except*` 블록 안에서는 `break`, `continue`, `return`을 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "move the control-flow statement outside the `except*` block, or use a normal `except` block",
        "제어 흐름 문장을 `except*` 블록 밖으로 옮기거나 일반 `except` 블록을 사용해 주세요",
    )
}

fn yield_inside_comprehension_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::YieldInsideComprehension,
        "`yield` cannot be used inside a comprehension",
        "컴프리헨션 안에서는 `yield`를 쓸 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "replace the comprehension with an explicit loop, or move `yield` outside it",
        "컴프리헨션을 명시적인 반복문으로 바꾸거나 `yield`를 밖으로 옮겨 주세요",
    )
}

fn async_comprehension_outside_async_function_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AsyncComprehensionOutsideAsyncFunction,
        "an async comprehension must be inside an async function",
        "비동기 컴프리헨션은 비동기 함수 안에 있어야 해요",
        span,
    )
    .with_bilingual_hint(
        "move the comprehension into an `async def` function, or use an ordinary `for` comprehension",
        "컴프리헨션을 `async def` 함수 안으로 옮기거나 일반 `for` 컴프리헨션을 사용해 주세요",
    )
}

fn return_value_in_async_generator_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ReturnValueInAsyncGenerator,
        "an async generator cannot return a value",
        "비동기 제너레이터에서는 값을 반환할 수 없어요",
        span,
    )
    .with_bilingual_hint(
        "use a bare `return`, or move the value-returning statement into a separate async function",
        "값이 없는 `return`을 사용하거나 값을 반환하는 문장을 별도의 비동기 함수로 옮겨 주세요",
    )
}

fn python_declaration_conflict_diagnostic(kind: PythonDeclarationKind, span: Span) -> Diagnostic {
    match kind {
        PythonDeclarationKind::Global => Diagnostic::bilingual(
            DiagnosticCode::GlobalDeclarationConflict,
            "`global` conflicts with an earlier name use or assignment",
            "`global` 선언이 앞선 이름 사용이나 대입과 충돌해요",
            span,
        )
        .with_bilingual_hint(
            "move `global` before the first use or assignment, and do not declare a parameter global",
            "첫 사용이나 대입보다 `global`을 먼저 적고, 매개변수를 global로 선언하지 말아 주세요",
        ),
        PythonDeclarationKind::Nonlocal => Diagnostic::bilingual(
            DiagnosticCode::NonlocalDeclarationConflict,
            "`nonlocal` conflicts with an earlier name use or assignment",
            "`nonlocal` 선언이 앞선 이름 사용이나 대입과 충돌해요",
            span,
        )
        .with_bilingual_hint(
            "move `nonlocal` before the first use or assignment, and do not declare a parameter nonlocal",
            "첫 사용이나 대입보다 `nonlocal`을 먼저 적고, 매개변수를 nonlocal로 선언하지 말아 주세요",
        ),
    }
}

fn branch_without_condition_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::BranchWithoutCondition,
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
        DiagnosticCode::DuplicateElse,
        "this condition already has an `else` branch",
        "이 조건에는 이미 `아니면` 가지가 있어요",
        span,
    )
    .with_bilingual_hint(
        "put another condition before `else`, or close the block",
        "`아니면` 전에 조건을 더 쓰거나 블록을 닫아 주세요",
    )
}

fn inline_break_is_outside_loop(stmt: &NmeStmt, source: &str, inside_loop: bool) -> bool {
    match stmt {
        NmeStmt::Break => !inside_loop,
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. } => inline
            .as_ref()
            .is_some_and(|body| inline_break_is_outside_loop_in_body(body, source, true)),
        NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_break_is_outside_loop_in_body(body, source, inside_loop)),
        _ => false,
    }
}

fn inline_break_is_outside_loop_in_body(
    body: &InlineStmt,
    source: &str,
    inside_loop: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_break_is_outside_loop(inner, source, inside_loop),
        InlineStmt::Python(span) => source[span.start..span.end].trim() == "break" && !inside_loop,
    }
}

fn inline_continue_is_outside_loop(stmt: &NmeStmt, tokens: &[Token], inside_loop: bool) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. } => inline
            .as_ref()
            .is_some_and(|body| inline_continue_is_outside_loop_in_body(body, tokens, true)),
        NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_continue_is_outside_loop_in_body(body, tokens, inside_loop)),
        _ => false,
    }
}

fn inline_continue_is_outside_loop_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_loop: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_continue_is_outside_loop(inner, tokens, inside_loop),
        InlineStmt::Python(span) => {
            first_token_in_span(tokens, *span)
                .is_some_and(|token| matches!(token.tok, Tok::Continue))
                && !inside_loop
        }
    }
}

fn inline_except_star_control_flow(stmt: &NmeStmt, tokens: &[Token]) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_except_star_control_flow_in_body(body, tokens)),
        _ => false,
    }
}

fn inline_except_star_control_flow_in_body(body: &InlineStmt, tokens: &[Token]) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            matches!(inner.as_ref(), NmeStmt::Break)
                || inline_except_star_control_flow(inner, tokens)
        }
        InlineStmt::Python(span) => first_token_in_span(tokens, *span)
            .is_some_and(|token| matches!(token.tok, Tok::Break | Tok::Continue | Tok::Return)),
    }
}

fn inline_return_is_outside_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_return_is_outside_function_in_body(body, tokens, inside_function)
        }),
        _ => false,
    }
}

fn inline_return_is_outside_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_return_is_outside_function(inner, tokens, inside_function),
        InlineStmt::Python(span) => {
            first_token_in_span(tokens, *span).is_some_and(|token| matches!(token.tok, Tok::Return))
                && !inside_function
        }
    }
}

fn inline_yield_inside_comprehension(stmt: &NmeStmt, tokens: &[Token]) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline
            .as_ref()
            .is_some_and(|body| inline_yield_inside_comprehension_in_body(body, tokens)),
        _ => false,
    }
}

fn inline_yield_inside_comprehension_in_body(body: &InlineStmt, tokens: &[Token]) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_yield_inside_comprehension(inner, tokens),
        InlineStmt::Python(span) => contains_yield_inside_comprehension_in_span(tokens, *span),
    }
}

fn inline_async_comprehension_outside_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_async_comprehension_outside_async_function_in_body(
                body,
                tokens,
                inside_async_function,
            )
        }),
        _ => false,
    }
}

fn inline_async_comprehension_outside_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_async_comprehension_outside_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => contains_async_comprehension_outside_async_function_in_span(
            tokens,
            *span,
            inside_async_function,
        ),
    }
}

fn inline_yield_is_outside_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_yield_is_outside_function_in_body(body, tokens, inside_function)
        }),
        _ => false,
    }
}

fn inline_yield_is_outside_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => inline_yield_is_outside_function(inner, tokens, inside_function),
        InlineStmt::Python(span) => {
            contains_yield_outside_lambda_in_span(tokens, *span) && !inside_function
        }
    }
}

fn inline_await_is_outside_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_await_is_outside_async_function_in_body(body, tokens, inside_async_function)
        }),
        _ => false,
    }
}

fn inline_await_is_outside_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_await_is_outside_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => {
            contains_invalid_await_in_span(tokens, *span, inside_async_function)
        }
    }
}

fn inline_yield_from_is_in_async_function(
    stmt: &NmeStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref().is_some_and(|body| {
            inline_yield_from_is_in_async_function_in_body(body, tokens, inside_async_function)
        }),
        _ => false,
    }
}

fn inline_yield_from_is_in_async_function_in_body(
    body: &InlineStmt,
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    match body {
        InlineStmt::Nme(inner) => {
            inline_yield_from_is_in_async_function(inner, tokens, inside_async_function)
        }
        InlineStmt::Python(span) => {
            contains_yield_from_outside_lambda_in_span(tokens, *span) && inside_async_function
        }
    }
}

fn first_token_in_span(tokens: &[Token], span: Span) -> Option<&Token> {
    tokens
        .iter()
        .find(|token| token.span.start >= span.start && token.span.end <= span.end)
}

fn contains_yield_inside_comprehension(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Yield) && yield_is_inside_comprehension(tokens, index)
    })
}

fn contains_yield_inside_comprehension_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Yield)
            && yield_is_inside_comprehension(tokens, index)
    })
}

fn contains_async_comprehension_outside_async_function(
    tokens: &[Token],
    inside_async_function: bool,
) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        matches!(pair[0].tok, Tok::Async)
            && matches!(pair[1].tok, Tok::For)
            && async_for_is_inside_comprehension(tokens, index)
            && (!inside_async_function || enclosing_lambda_body_start(tokens, index).is_some())
    })
}

fn contains_async_comprehension_outside_async_function_in_span(
    tokens: &[Token],
    span: Span,
    inside_async_function: bool,
) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        pair[0].span.start >= span.start
            && pair[1].span.end <= span.end
            && matches!(pair[0].tok, Tok::Async)
            && matches!(pair[1].tok, Tok::For)
            && async_for_is_inside_comprehension(tokens, index)
            && (!inside_async_function || enclosing_lambda_body_start(tokens, index).is_some())
    })
}

fn async_for_is_inside_comprehension(tokens: &[Token], async_index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    (0..async_index).any(|open_index| {
        let is_open = matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace);
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        is_open && async_index < close_index && depths[async_index] == depths[open_index] + 1
    })
}

fn remember_async_generator_context(
    contexts: &mut Vec<AsyncFunctionContext>,
    tokens: &[Token],
    python_scope_depth: usize,
    span: Span,
) {
    let has_direct_yield =
        contains_yield_outside_lambda(tokens) && !contains_yield_inside_comprehension(tokens);
    let has_return_value = contains_return_with_value(tokens);
    if let Some(context) = contexts
        .last_mut()
        .filter(|context| context.body_scope_depth == python_scope_depth)
    {
        context.has_yield |= has_direct_yield;
        if has_return_value {
            context.return_value_spans.push(span);
        }
    }
    if is_python_async_function_header(tokens) {
        contexts.push(AsyncFunctionContext {
            body_scope_depth: python_scope_depth + 1,
            has_yield: false,
            return_value_spans: Vec::new(),
        });
    }
}

fn contains_return_with_value(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Return)
            && tokens
                .get(index + 1)
                .is_some_and(|next| !matches!(next.tok, Tok::Semi))
    })
}

fn remember_python_declaration_context(
    contexts: &mut Vec<PythonDeclarationContext>,
    tokens: &[Token],
    python_scope_depth: usize,
) -> Option<PythonDeclarationKind> {
    if let Some(body) = python_inline_suite_body(tokens) {
        if let Some(context) = contexts
            .last_mut()
            .filter(|context| context.body_scope_depth == python_scope_depth)
        {
            for name in python_names_seen_in_scope(tokens, &[]) {
                context.seen_names.insert(name);
            }
        }
        let (_, parameters) = python_scope_header(tokens).expect("inline Python scope header");
        let mut inline_context = PythonDeclarationContext {
            body_scope_depth: python_scope_depth + 1,
            seen_names: parameters,
            annotation_targets: HashSet::new(),
            declarations: HashMap::new(),
        };
        let declarations = python_declarations(body);
        return remember_python_declarations_in_scope(&mut inline_context, body, &declarations);
    }

    let declarations = if python_scope_header(tokens).is_some() {
        Vec::new()
    } else {
        python_declarations(tokens)
    };
    let conflict = contexts
        .last_mut()
        .filter(|context| context.body_scope_depth == python_scope_depth)
        .and_then(|context| remember_python_declarations_in_scope(context, tokens, &declarations));

    if let Some((_, parameters)) = python_scope_header(tokens) {
        contexts.push(PythonDeclarationContext {
            body_scope_depth: python_scope_depth + 1,
            seen_names: parameters,
            annotation_targets: HashSet::new(),
            declarations: HashMap::new(),
        });
    }
    conflict
}

fn remember_python_declarations_in_scope(
    context: &mut PythonDeclarationContext,
    tokens: &[Token],
    declarations: &[PythonDeclaration],
) -> Option<PythonDeclarationKind> {
    let mut conflict = None;
    for (declaration_index, declaration) in declarations.iter().enumerate() {
        let declaration_start = declaration
            .names
            .first()
            .map_or(0, |(_, name_index)| name_index.saturating_sub(1));
        for name in python_names_seen_in_scope(
            &tokens[..declaration_start],
            &declarations[..declaration_index],
        ) {
            context.seen_names.insert(name);
        }
        for name in python_annotation_target_names(&tokens[..declaration_start]) {
            context.annotation_targets.insert(name);
        }
        for (name, _) in &declaration.names {
            let has_other_declaration = context
                .declarations
                .get(name)
                .is_some_and(|previous| *previous != declaration.kind);
            let has_annotation_target = context.annotation_targets.contains(name);
            if conflict.is_none()
                && (has_other_declaration
                    || context.seen_names.contains(name)
                    || has_annotation_target)
            {
                conflict = Some(declaration.kind);
            }
            context
                .declarations
                .entry(name.clone())
                .or_insert(declaration.kind);
        }
    }
    for name in python_annotation_target_names(tokens) {
        if context.body_scope_depth != 0 && conflict.is_none() {
            if let Some(kind) = context.declarations.get(&name) {
                conflict = Some(*kind);
            }
        }
        context.annotation_targets.insert(name);
    }
    for name in python_names_seen_in_scope(tokens, declarations) {
        context.seen_names.insert(name);
    }
    conflict
}

fn python_declarations(tokens: &[Token]) -> Vec<PythonDeclaration> {
    let mut declarations = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let kind = match tokens[index].tok {
            Tok::Global => PythonDeclarationKind::Global,
            Tok::Nonlocal => PythonDeclarationKind::Nonlocal,
            _ => {
                index += 1;
                continue;
            }
        };
        if index > 0 && !matches!(tokens[index - 1].tok, Tok::Semi | Tok::Colon | Tok::Newline) {
            index += 1;
            continue;
        }
        let mut names = Vec::new();
        let mut cursor = index + 1;
        while cursor < tokens.len() && !matches!(tokens[cursor].tok, Tok::Semi) {
            if let Tok::Name { name } = &tokens[cursor].tok {
                let previous_is_separator =
                    cursor == index + 1 || matches!(tokens[cursor - 1].tok, Tok::Comma);
                if previous_is_separator {
                    names.push((name.clone(), cursor));
                }
            }
            cursor += 1;
        }
        if !names.is_empty() {
            declarations.push(PythonDeclaration { kind, names });
        }
        index = cursor;
    }
    declarations
}

fn python_names_seen_in_scope(tokens: &[Token], declarations: &[PythonDeclaration]) -> Vec<String> {
    let declared_indices: HashSet<usize> = declarations
        .iter()
        .flat_map(|declaration| declaration.names.iter().map(|(_, index)| *index))
        .collect();
    if let Some((name, _)) = python_scope_header(tokens) {
        return vec![name];
    }
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            let Tok::Name { name } = &token.tok else {
                return None;
            };
            if declared_indices.contains(&index)
                || index > 0 && matches!(tokens[index - 1].tok, Tok::Dot)
                || is_python_keyword_argument_name(tokens, index)
                || token_is_inside_lambda(tokens, index)
                || is_lambda_parameter_name(tokens, index)
                || is_python_annotation_target_name(tokens, index)
                || is_comprehension_local_name(tokens, index)
            {
                None
            } else {
                Some(name.clone())
            }
        })
        .collect()
}

fn python_annotation_target_names(tokens: &[Token]) -> Vec<String> {
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if !is_python_annotation_target_name(tokens, index) {
                return None;
            }
            let Tok::Name { name } = &token.tok else {
                return None;
            };
            Some(name.clone())
        })
        .collect()
}

fn is_python_keyword_argument_name(tokens: &[Token], index: usize) -> bool {
    tokens
        .get(index + 1)
        .is_some_and(|token| matches!(token.tok, Tok::Equal))
        && tokens
            .get(index.wrapping_sub(1))
            .is_some_and(|token| matches!(token.tok, Tok::Lpar | Tok::Comma))
}

fn is_lambda_parameter_name(tokens: &[Token], index: usize) -> bool {
    let depths = token_depths(tokens);
    let Some(lambda_index) = (0..index).rev().find(|&candidate| {
        if !matches!(tokens[candidate].tok, Tok::Lambda) {
            return false;
        }
        (candidate + 1..tokens.len()).any(|colon| {
            depths[colon] == depths[candidate]
                && matches!(tokens[colon].tok, Tok::Colon)
                && index < colon
        })
    }) else {
        return false;
    };
    let Some(colon_index) = (lambda_index + 1..tokens.len()).find(|&candidate| {
        depths[candidate] == depths[lambda_index] && matches!(tokens[candidate].tok, Tok::Colon)
    }) else {
        return false;
    };
    if index >= colon_index || !matches!(tokens[index].tok, Tok::Name { .. }) {
        return false;
    }
    let mut in_default = false;
    for candidate in lambda_index + 1..index {
        if depths[candidate] != depths[lambda_index] {
            continue;
        }
        match tokens[candidate].tok {
            Tok::Equal => in_default = true,
            Tok::Comma => in_default = false,
            _ => {}
        }
    }
    !in_default
}

fn is_python_annotation_target_name(tokens: &[Token], index: usize) -> bool {
    if !matches!(
        tokens.get(index).map(|token| &token.tok),
        Some(Tok::Name { .. })
    ) {
        return false;
    }
    let depths = token_depths(tokens);
    let Some(next) = tokens.get(index + 1) else {
        return false;
    };
    if !matches!(next.tok, Tok::Colon) || depths[index] != depths[index + 1] {
        return false;
    }
    index == 0
        || tokens
            .get(index.wrapping_sub(1))
            .is_some_and(|previous| matches!(previous.tok, Tok::Comma | Tok::Semi))
}

fn is_comprehension_local_name(tokens: &[Token], index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    (0..index).any(|open_index| {
        if !matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace) {
            return false;
        }
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        if index >= close_index {
            return false;
        }
        let body_depth = depths[open_index] + 1;
        let for_indices: Vec<usize> = (open_index + 1..close_index)
            .filter(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::For)
            })
            .collect();
        let Some(first_for) = for_indices.first().copied() else {
            return false;
        };
        if index > open_index && index < first_for && depths[index] >= body_depth {
            return true;
        }
        for for_index in for_indices {
            let Some(in_index) = (for_index + 1..close_index).find(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::In)
            }) else {
                continue;
            };
            if (for_index + 1..in_index).contains(&index) && depths[index] >= body_depth {
                return true;
            }
            let next_for = (in_index + 1..close_index).find(|&candidate| {
                depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::For)
            });
            let segment_end = next_for.unwrap_or(close_index);
            if index > in_index
                && index < segment_end
                && (in_index + 1..index).any(|candidate| {
                    depths[candidate] == body_depth && matches!(tokens[candidate].tok, Tok::If)
                })
            {
                return true;
            }
        }
        false
    })
}

fn yield_is_inside_comprehension(tokens: &[Token], target_index: usize) -> bool {
    let depths = token_depths(tokens);
    let closes = matching_bracket_closes(tokens);
    let lambda_body_start = enclosing_lambda_body_start(tokens, target_index);
    (0..target_index).any(|open_index| {
        let is_open = matches!(tokens[open_index].tok, Tok::Lpar | Tok::Lsqb | Tok::Lbrace);
        let Some(close_index) = closes[open_index] else {
            return false;
        };
        if !is_open || target_index >= close_index {
            return false;
        }
        let body_depth = depths[open_index] + 1;
        let has_comprehension_for = (open_index + 1..close_index)
            .any(|index| depths[index] == body_depth && matches!(tokens[index].tok, Tok::For));
        has_comprehension_for
            && lambda_body_start.is_none_or(|lambda_start| open_index >= lambda_start)
    })
}

fn matching_bracket_closes(tokens: &[Token]) -> Vec<Option<usize>> {
    let mut stack = Vec::new();
    let mut closes = vec![None; tokens.len()];
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => stack.push(index),
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => {
                if let Some(open_index) = stack.pop() {
                    closes[open_index] = Some(index);
                }
            }
            _ => {}
        }
    }
    closes
}

fn contains_yield_outside_lambda(tokens: &[Token]) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Yield) && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_yield_outside_lambda_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Yield)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_invalid_await(tokens: &[Token], inside_async_function: bool) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        matches!(token.tok, Tok::Await)
            && (token_is_inside_lambda(tokens, index) || !inside_async_function)
    })
}

fn contains_invalid_await_in_span(
    tokens: &[Token],
    span: Span,
    inside_async_function: bool,
) -> bool {
    tokens.iter().enumerate().any(|(index, token)| {
        token.span.start >= span.start
            && token.span.end <= span.end
            && matches!(token.tok, Tok::Await)
            && (token_is_inside_lambda(tokens, index) || !inside_async_function)
    })
}

fn contains_yield_from_outside_lambda(tokens: &[Token]) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        matches!(pair[0].tok, Tok::Yield)
            && matches!(pair[1].tok, Tok::From)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn contains_yield_from_outside_lambda_in_span(tokens: &[Token], span: Span) -> bool {
    tokens.windows(2).enumerate().any(|(index, pair)| {
        pair[0].span.start >= span.start
            && pair[1].span.end <= span.end
            && matches!(pair[0].tok, Tok::Yield)
            && matches!(pair[1].tok, Tok::From)
            && !token_is_inside_lambda(tokens, index)
    })
}

fn token_is_inside_lambda(tokens: &[Token], target_index: usize) -> bool {
    enclosing_lambda_body_start(tokens, target_index).is_some()
}

fn enclosing_lambda_body_start(tokens: &[Token], target_index: usize) -> Option<usize> {
    let depths = token_depths(tokens);
    (0..target_index).rev().find_map(|lambda_index| {
        if !matches!(tokens[lambda_index].tok, Tok::Lambda) {
            return None;
        }
        let lambda_depth = depths[lambda_index];
        let colon_index = (lambda_index + 1..target_index).find(|&index| {
            depths[index] == lambda_depth && matches!(tokens[index].tok, Tok::Colon)
        })?;
        let body_ends_before_target = (colon_index + 1..target_index).any(|index| {
            depths[index] == lambda_depth
                && matches!(
                    tokens[index].tok,
                    Tok::Comma | Tok::Semi | Tok::Rpar | Tok::Rsqb | Tok::Rbrace
                )
        });
        (!body_ends_before_target).then_some(colon_index + 1)
    })
}

fn token_depths(tokens: &[Token]) -> Vec<usize> {
    let mut depth = 0usize;
    tokens
        .iter()
        .map(|token| {
            let before = depth;
            match token.tok {
                Tok::Rpar | Tok::Rsqb | Tok::Rbrace => {
                    depth = depth.saturating_sub(1);
                }
                Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
                _ => {}
            }
            before
        })
        .collect()
}

fn is_python_return_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Return))
}

fn is_python_continue_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Continue))
}

fn is_python_break_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| matches!(tok, Tok::Break))
}

fn has_direct_python_statement<F>(tokens: &[Token], predicate: F) -> bool
where
    F: Fn(&Tok) -> bool,
{
    let depths = token_depths(tokens);
    tokens.iter().enumerate().any(|(index, token)| {
        depths[index] == 0
            && predicate(&token.tok)
            && (index == 0
                || (depths[index - 1] == 0 && matches!(tokens[index - 1].tok, Tok::Semi)))
    })
}

fn missing_end_diagnostic(block: &ExplicitBlock, offset: usize) -> Diagnostic {
    let (english, korean) = match block {
        ExplicitBlock::Loop { .. } => (
            "this loop is missing its closing `end`",
            "이 반복문에는 닫는 `끝`이 필요해요",
        ),
        ExplicitBlock::Conditional { .. } => (
            "this condition is missing its closing `end`",
            "이 조건문에는 닫는 `끝`이 필요해요",
        ),
    };
    Diagnostic::bilingual(
        DiagnosticCode::MissingEnd,
        english,
        korean,
        Span::new(offset, offset),
    )
    .with_bilingual_hint(
        "add `end`/`끝` on a line by itself",
        "줄 하나에 `end` 또는 `끝`만 적어 주세요",
    )
}

#[allow(clippy::too_many_lines)]
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
    if looks_like_python_invocation(tokens) && !is_header_shape(tokens) {
        return Ok(None);
    }
    // rustpython-parser can lag behind the CPython selected by the CLI (for
    // example, Python 3.14 t-strings). An adjacent name+string prefix is a
    // strong signal for a newer string-prefix grammar rather than
    // conversational NME. Preserve it byte-for-byte; `nme check` and
    // `nme build` will ask the real CPython whether they are valid.
    if looks_like_future_python(tokens) {
        return Ok(None);
    }

    // A natural condition may start with its subject (`색이 빨강과 같으면
    // ...`) instead of an explicit `if`/`만약`. Check this before value-change
    // recovery so a misspelled action such as `말해` is not mistaken for
    // `더해`.
    if let Some(stmt) = match_subject_when(source, tokens, block, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
    }
    // Let a structured subject-first condition with a one-edit connector
    // typo win over an exact output word at the end (`이름이 있으먄
    // 안녕 말해줘`). Ordinary prose is guarded by the connector-shape checks
    // inside `match_subject_when`.
    if let Some(stmt) = match_subject_when(source, tokens, block, known_names, MatchMode::Recover)?
    {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_update(source, tokens, known_names, MatchMode::Exact)? {
        return Ok(Some(stmt));
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

    // `from "module.nme" import names` is not valid Python, so the keyword
    // gate below must not swallow it.
    if matches!(tokens.first().map(|token| &token.tok), Some(Tok::From))
        && tokens
            .get(1)
            .is_some_and(|token| matches!(token.tok, Tok::String { .. }))
    {
        if let Some(stmt) = match_module_import(source, tokens, known_names, MatchMode::Exact)? {
            return Ok(Some(stmt));
        }
    }

    // `for each name in names` is not valid Python either, and `for` is a
    // Python keyword, so it has to be recognized before the gate below.
    if english_for_each_start(tokens, MatchMode::Exact).is_some() {
        return match_for_each(source, tokens, block, known_names, MatchMode::Exact);
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
    // These four run before the older actions because each ends with a word
    // the output vocabulary would otherwise claim: `기다려`, `건너뛰어`, `넣어`,
    // and the `마다` loop shape.
    // The screen and timing sentences come first for the same reason: each
    // of them ends in, or contains, a word one of the older actions would
    // otherwise claim (`기다려`, `걸어`, a number of seconds, an output word).
    exact_match!(match_say_slowly(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_say_in_box(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    exact_match!(match_say_in_middle(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    if let Some(stmt) = match_clear_screen(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_draw_line(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_start_timer(tokens, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    exact_match!(match_cooldown(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));
    if let Some(stmt) = match_cooldown_wait(tokens, known_names, MatchMode::Exact) {
        return Ok(Some(stmt));
    }
    exact_match!(match_wait(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_continue(tokens, MatchMode::Exact));
    exact_match!(match_append(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_for_each(
        source,
        tokens,
        block,
        known_names,
        MatchMode::Exact
    ));

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
    if set_action_at(tokens, 0, MatchMode::Exact).is_some() {
        return match_set(source, tokens, known_names, MatchMode::Exact);
    }
    if action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Exact).is_some()
    {
        if recoverable_module_shape(tokens) {
            return match_use_module(source, tokens, known_names, MatchMode::Recover);
        }
        return match_use_module(source, tokens, known_names, MatchMode::Exact);
    }
    if action_phrase_at(tokens, 0, FILE_READ_WORDS_EN, MatchMode::Exact).is_some()
        || action_phrase_at(tokens, 0, FILE_WRITE_WORDS_EN, MatchMode::Exact).is_some()
        || tokens.iter().any(|token| {
            action_phrase_at(
                std::slice::from_ref(token),
                0,
                FILE_READ_WORDS_KO,
                MatchMode::Exact,
            )
            .is_some()
        })
        || tokens.iter().any(|token| {
            action_phrase_at(
                std::slice::from_ref(token),
                0,
                FILE_WRITE_WORDS_KO,
                MatchMode::Exact,
            )
            .is_some()
        })
    {
        exact_match!(match_file_io(source, tokens, known_names, MatchMode::Exact));
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
    // A misspelled condition starter followed by a real connector is a
    // stronger sentence shape than an exact output word at the end. Without
    // this early recovery, `만악에 이름이 있으면 안녕 말해줘` would be read as
    // plain output because `말해줘` is exact.
    let recoverable_condition_starter = when_action_at(tokens, 0, MatchMode::Exact).is_none()
        && when_action_at(tokens, 0, MatchMode::Recover)
            .is_some_and(|(_, consumed)| find_condition_connector(&tokens[consumed..]).is_some());
    if recoverable_condition_starter {
        exact_match!(match_when(
            source,
            tokens,
            block,
            known_names,
            MatchMode::Recover
        ));
    }
    exact_match!(match_ask(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_say(source, tokens, known_names, MatchMode::Exact));

    // Check this before Korean assignment particles such as `은`/`는` can
    // make `이름은 뭐예요?` look like a sentence assignment. Explicit output
    // and ask actions above still win when the learner writes them.
    if let Some(stmt) = match_natural_question(source, tokens, known_names) {
        return Ok(Some(stmt));
    }

    exact_match!(match_set(source, tokens, known_names, MatchMode::Exact));
    exact_match!(match_use_module(
        source,
        tokens,
        known_names,
        MatchMode::Exact
    ));

    // A bare contraction such as `Don't stop!` can put `Don` one edit away
    // from the repeat alias `do`, and `It's easy` can put `It` near `if`.
    // When no complete NME shape is present, ordinary word-like input should
    // win over those weak typo candidates.
    if looks_like_plain_prose(tokens) && !has_recoverable_sentence_shape(tokens) {
        let value = parse_value(source, tokens, known_names, true)
            .map_err(|()| missing_action_diagnostic(tokens))?;
        return Ok(Some(NmeStmt::Say { value }));
    }

    let recovered = [
        match_subject_when(source, tokens, block, known_names, MatchMode::Recover),
        match_update(source, tokens, known_names, MatchMode::Recover),
        match_when(source, tokens, block, known_names, MatchMode::Recover),
        match_times(source, tokens, block, known_names, MatchMode::Recover),
        match_ask(source, tokens, known_names, MatchMode::Recover),
        match_say(source, tokens, known_names, MatchMode::Recover),
        match_set(source, tokens, known_names, MatchMode::Recover),
        match_use_module(source, tokens, known_names, MatchMode::Recover),
        match_file_io(source, tokens, known_names, MatchMode::Recover),
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

    // Invalid Python led by another Python keyword belongs to Python. This
    // preserves its own context-sensitive diagnostics (`elif`, `except`, ...)
    // while still allowing the deliberately supported mixed `if 조건` form.
    if is_python_keyword(&tokens[0].tok) {
        return Ok(None);
    }
    if looks_like_plain_prose(tokens) {
        let value = parse_value(source, tokens, known_names, true)
            .map_err(|()| missing_action_diagnostic(tokens))?;
        return Ok(Some(NmeStmt::Say { value }));
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
    let action_start = leading_sentence_fillers(tokens);
    if let Some((spelling, consumed)) = output_action_at(tokens, action_start, mode) {
        let mut body_start = action_start + consumed;
        if tokens.get(body_start).is_some_and(is_command_ending) && body_start + 1 < tokens.len() {
            body_start += 1;
        }
        if body_start + 1 < tokens.len()
            && tokens.get(body_start).is_some_and(is_show_request_pronoun)
        {
            body_start += 1;
        }
        if body_start >= tokens.len() {
            return Err(say_missing(spelling, tokens[action_start].span));
        }
        let body = &tokens[body_start..];
        let prefer_text = action_start != 0
            || consumed != 1
            || mode == MatchMode::Recover
            || (!token_is_exact_name(&tokens[action_start], SAY_KEYWORD)
                && !token_is_exact_name(&tokens[action_start], SAY_KEYWORD_KO));
        if !prefer_text {
            let span = span_of(body);
            let text = &source[span.start..span.end];
            if looks_like_broken_expression(body) && !is_valid_python_expression(text) {
                return Err(Diagnostic::bilingual(
                    DiagnosticCode::SayValueBroken,
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
                DiagnosticCode::SayValueUnparseable,
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
    let mut value_start = leading_sentence_fillers(&tokens[..action_start]);
    if value_start + 1 < action_start
        && tokens.get(value_start).is_some_and(is_show_request_pronoun)
    {
        value_start += 1;
    }
    let value_tokens = trim_suffix_say_value(&tokens[value_start..action_start]);
    if value_tokens.is_empty() {
        return Err(say_missing(spelling, tokens[action_start].span));
    }
    let value = parse_value(source, &value_tokens, known_names, true).map_err(|()| {
        Diagnostic::bilingual(
            DiagnosticCode::SaySentenceUnparseable,
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
    Diagnostic::bilingual(
        DiagnosticCode::SayMissing,
        "there is nothing to show",
        "말할 내용이 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `show Hello world`",
        "`안녕하세요 말해줘`처럼 내용을 함께 적어 주세요",
    )
}

fn is_show_request_pronoun(token: &Token) -> bool {
    token_matches_exact(token, &["me", "나", "나를", "나에게"])
}

fn output_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, SAY_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, SAY_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn leading_sentence_fillers(tokens: &[Token]) -> usize {
    let mut index = 0;
    while tokens
        .get(index)
        .is_some_and(|token| token_matches_exact(token, SENTENCE_FILLERS))
    {
        index += 1;
    }
    index
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
                DiagnosticCode::AskQuestionMissing,
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
        if is_valid_python_expression(&source[span.start..span.end]) {
            Some(Value::Python(Code::Source(span)))
        } else if expression_tokens
            .iter()
            .all(|token| token_word(token).is_some() || is_command_ending(token))
        {
            Some(Value::Text(make_text_template(
                source,
                expression_tokens,
                known_names,
            )))
        } else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::AskQuestionUnparseable,
                "I couldn't understand the question",
                "질문 내용을 이해하지 못했어요",
                span,
            )
            .with_bilingual_hint(
                "remove the comma to write a plain sentence without quotes",
                "쉼표를 빼면 따옴표 없는 평범한 문장으로 쓸 수 있어요",
            ));
        }
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

/// Match the smallest conversational input forms:
/// `이름이 뭐예요` and `What is your name`.
///
/// The Korean/English question predicate is deliberate proof of intent. A
/// normal sentence such as `안녕하세요!` therefore remains output, while a
/// beginner can start asking without learning `ask`, a comma, quotes, or even
/// a question mark. More complex questions still use the explicit
/// `물어봐`/`ask` form.
fn match_natural_question(
    source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
) -> Option<NmeStmt> {
    let has_question_mark = tokens
        .last()
        .is_some_and(|token| token_matches_exact(token, &["?"]));
    let question_end = if has_question_mark {
        tokens.len().checked_sub(1)?
    } else {
        tokens.len()
    };
    if question_end < 2 {
        return None;
    }

    // A question about how many, or about an age, is answered with a number,
    // and the next line a learner writes is almost always a comparison — so
    // read it as one instead of leaving a string behind.
    let asks_for_a_number = natural_age_question_target(tokens, question_end).is_some()
        || tokens[..question_end]
            .iter()
            .any(|token| token_word(token) == Some("몇"));
    let target = if let Some(target) = natural_age_question_target(tokens, question_end) {
        Some(target)
    } else if let Some(first) = tokens.first().and_then(name_word) {
        // `내 이름은 뭐예요?` is the same beginner question as
        // `이름이 뭐예요?`; the possessive is natural speech, not part of
        // the variable name.
        let target_at = usize::from(matches!(first, "내" | "제" | "우리"));
        let target_word = tokens.get(target_at).and_then(name_word)?;
        let particle_at = target_at + 1;
        let predicate_at = if tokens
            .get(particle_at)
            .is_some_and(|token| token_matches_exact(token, &["은", "는", "이", "가", "을", "를"]))
        {
            particle_at + 1
        } else {
            particle_at
        };
        // Both attached (`이름이`) and spoken, separated particles (`이름 이`)
        // are common. A bare target is also safe once the distinctive
        // question predicate has been proven below.
        let korean_target = strip_natural_question_particle(target_word).or(Some(target_word));
        let predicate = tokens.get(predicate_at).and_then(token_word)?;
        let is_korean_question = [
            "뭐예요",
            "뭐에요",
            "뭐야",
            "뭐죠",
            "무엇인가요",
            "무엇이에요",
            "무엇입니까",
            "뭔가요",
        ]
        .contains(&predicate)
            || (predicate == "몇" && tokens.get(predicate_at + 1).and_then(token_word).is_some());
        korean_target.filter(|_| is_korean_question)
    } else {
        None
    }
    .or_else(|| {
        let first = tokens.first().and_then(token_word)?;
        let (subject_at, matches_shape) = if first.eq_ignore_ascii_case("what") {
            if tokens
                .get(1)
                .and_then(token_word)
                .is_some_and(|word| word.eq_ignore_ascii_case("is"))
            {
                (2, question_end >= 4)
            } else if tokens
                .get(1)
                .and_then(token_word)
                .is_some_and(|word| word.eq_ignore_ascii_case("s"))
            {
                // The sentence lexer separates the apostrophe in `What's`
                // so it can safely preserve ordinary contractions.
                (2, question_end >= 4)
            } else {
                (0, false)
            }
        } else if first.eq_ignore_ascii_case("what's") || first.eq_ignore_ascii_case("whats") {
            (1, question_end >= 3)
        } else {
            (0, false)
        };
        if !matches_shape
            || !tokens
                .get(subject_at)
                .and_then(token_word)
                .is_some_and(|word| {
                    word.eq_ignore_ascii_case("your")
                        || word.eq_ignore_ascii_case("the")
                        || word.eq_ignore_ascii_case("my")
                        || word.eq_ignore_ascii_case("our")
                })
        {
            return None;
        }
        name_word(tokens.get(question_end - 1)?)
    })?;

    let prompt_tokens = if has_question_mark {
        &tokens[..=question_end]
    } else {
        &tokens[..question_end]
    };
    let prompt = Value::Text(make_text_template(source, prompt_tokens, &HashSet::new()));
    Some(NmeStmt::Ask {
        target: target.to_string(),
        prompt: Some(prompt),
        kind: if asks_for_a_number || target == "age" {
            InputKind::Number
        } else {
            InputKind::Text
        },
    })
}

fn natural_age_question_target(tokens: &[Token], question_end: usize) -> Option<&'static str> {
    let word = |index: usize| tokens.get(index).and_then(token_word);
    let korean_age = [
        "살이에요",
        "살이예요",
        "살이야",
        "살인가요",
        "살입니까",
        "살이죠",
    ];
    if word(0) == Some("몇") && word(1).is_some_and(|value| korean_age.contains(&value)) {
        return Some("나이");
    }
    if word(0) == Some("나")
        && word(1) == Some("몇")
        && word(2).is_some_and(|value| korean_age.contains(&value))
    {
        return Some("나이");
    }
    if word(0).is_some_and(|value| value.eq_ignore_ascii_case("how"))
        && word(1).is_some_and(|value| value.eq_ignore_ascii_case("old"))
        && ((word(2).is_some_and(|value| value.eq_ignore_ascii_case("are"))
            && word(3).is_some_and(|value| value.eq_ignore_ascii_case("you")))
            || (word(2).is_some_and(|value| value.eq_ignore_ascii_case("am"))
                && word(3).is_some_and(|value| value.eq_ignore_ascii_case("i"))))
        && question_end >= 4
    {
        return Some("age");
    }
    None
}

fn strip_natural_question_particle(word: &str) -> Option<&str> {
    // A final `이` can be either the subject particle (`이름이`) or part of a
    // normal Korean noun (`나이`, `아이`, `종이`). Keep the common noun forms
    // intact; attached `은`/`는` and less ambiguous particles still strip as
    // expected.
    if [
        "나이",
        "아이",
        "고양이",
        "강아지",
        "종이",
        "사이",
        "회의",
        "이야기",
        "의미",
    ]
    .contains(&word)
    {
        return None;
    }
    strip_any_suffix(word, &["은", "는", "이", "가", "을", "를"])
}

struct AskShape {
    action_start: usize,
    target_at: usize,
    prompt_start: usize,
    spelling: Spelling,
    kind: InputKind,
}

fn find_ask_shape(tokens: &[Token], mode: MatchMode) -> Option<AskShape> {
    let action_start = leading_sentence_fillers(tokens);
    if let Some((spelling, consumed)) = ask_action_at(tokens, action_start, mode) {
        let mut target_at = action_start + consumed;
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
            action_start,
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
        DiagnosticCode::AskTargetInvalid,
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

fn match_update(
    source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A line that opens with an output or question word is that statement,
    // whatever its free text happens to say. Without this, the arithmetic
    // words inside a message quietly rewrite the whole line: `show I will
    // multiply by 2` used to become `show = show * 2`.
    if tokens
        .first()
        .is_some_and(|token| starts_a_different_statement(token))
    {
        return Ok(None);
    }
    if let Some((action_start, operation, _)) = update_action_ending(tokens, mode) {
        let target_token = tokens
            .first()
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        // Only a name can have its value changed. A line that starts with a
        // number or a piece of text is some other sentence that merely
        // happens to contain `더해`, so let the other matchers read it.
        if name_word(target_token).is_none() {
            return Ok(None);
        }
        let target = name_word(target_token)
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(target_token.span))?;
        let mut amount_tokens = tokens[1..action_start].to_vec();
        while amount_tokens
            .first()
            .is_some_and(|token| is_update_connector(token, &["에", "에서", "에게", "한테"]))
        {
            amount_tokens.remove(0);
        }
        while amount_tokens
            .last()
            .is_some_and(|token| is_update_connector(token, &["을", "를", "만큼"]))
        {
            amount_tokens.pop();
        }
        let amount = parse_update_amount(source, &amount_tokens)
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Update {
            target,
            amount,
            operation,
        }));
    }

    for action_start in 1..tokens.len() {
        let Some((operation, consumed)) = update_action_at(tokens, action_start, mode) else {
            continue;
        };
        if name_word(&tokens[0]).is_none() {
            return Ok(None);
        }
        let target = name_word(&tokens[0])
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(tokens[0].span))?;
        let mut amount_end = tokens.len();
        if tokens
            .get(amount_end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            amount_end -= 1;
        }
        let mut amount_tokens = tokens[action_start + consumed..amount_end].to_vec();
        while amount_tokens
            .first()
            .is_some_and(|token| is_update_connector(token, &["by", "to", "of"]))
        {
            amount_tokens.remove(0);
        }
        let amount = parse_update_amount(source, &amount_tokens)
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Update {
            target,
            amount,
            operation,
        }));
    }

    // English also reads naturally as `add 1 to score` or
    // `increase score by 1`. Keep this form deliberately exact so a normal
    // Python expression cannot be claimed by the sentence matcher.
    if let Some((operation, consumed)) = update_action_at(tokens, 0, mode) {
        let mut remainder_end = tokens.len();
        if tokens
            .get(remainder_end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            remainder_end -= 1;
        }
        let remainder = &tokens[consumed..remainder_end];
        let separator = remainder
            .iter()
            .position(|token| is_update_connector(token, &["to", "by", "from"]));
        let Some(separator) = separator else {
            return Err(update_diagnostic(span_of(tokens)));
        };
        let (left, right) = remainder.split_at(separator);
        let right = &right[1..];
        let (target_tokens, amount_tokens) = if (operation == UpdateOp::Add
            && token_matches_exact(&remainder[separator], &["to"]))
            || (operation == UpdateOp::Subtract
                && token_matches_exact(&remainder[separator], &["from"]))
        {
            (right, left)
        } else if !left.is_empty() && !right.is_empty() {
            (left, right)
        } else {
            return Err(update_diagnostic(span_of(tokens)));
        };
        if target_tokens.len() != 1 {
            return Err(update_diagnostic(span_of(tokens)));
        }
        let target = name_word(&target_tokens[0])
            .and_then(update_target_name)
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        let amount = parse_update_amount(source, amount_tokens)
            .ok_or_else(|| update_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Update {
            target,
            amount,
            operation,
        }));
    }

    Ok(None)
}

fn update_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(UpdateOp, usize)> {
    // A typo such as `말헤` is equally close to the output action `말해` and
    // the update action `더해`. Prefer the explicit output vocabulary rather
    // than silently turning a spoken sentence into arithmetic. Exact update
    // words (`더해`, `add`, ...) remain unaffected.
    if mode == MatchMode::Recover
        && (action_phrase_at(tokens, start, SAY_WORDS_EN, MatchMode::Recover).is_some()
            || action_phrase_at(tokens, start, SAY_WORDS_KO, MatchMode::Recover).is_some())
    {
        return None;
    }
    action_phrase_at(tokens, start, UPDATE_ADD_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, start, UPDATE_ADD_WORDS_KO, mode))
        .map(|consumed| (UpdateOp::Add, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_SUBTRACT_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_SUBTRACT_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Subtract, consumed))
        })
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_MULTIPLY_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_MULTIPLY_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Multiply, consumed))
        })
        .or_else(|| {
            action_phrase_at(tokens, start, UPDATE_DIVIDE_WORDS_EN, mode)
                .or_else(|| action_phrase_at(tokens, start, UPDATE_DIVIDE_WORDS_KO, mode))
                .map(|consumed| (UpdateOp::Divide, consumed))
        })
}

fn update_action_ending(tokens: &[Token], mode: MatchMode) -> Option<(usize, UpdateOp, usize)> {
    let mut end = tokens.len();
    if tokens.last().is_some_and(is_command_ending) {
        end -= 1;
    }
    let start_at = end.saturating_sub(3);
    for start in start_at..end {
        if let Some((operation, consumed)) = update_action_at(tokens, start, mode) {
            if start + consumed == end {
                return Some((start, operation, end));
            }
        }
    }
    None
}

/// Words that own the line they open, so a value change may not start there.
fn starts_a_different_statement(token: &Token) -> bool {
    [
        SAY_WORDS_EN,
        ASK_WORDS_EN,
        SAY_WORDS_KO,
        ASK_WORDS_KO,
        WHEN_WORDS_EN,
        WHEN_WORDS_KO,
        ELSE_WORDS_EN,
        ELSE_WORDS_KO,
        WHILE_WORDS_EN,
        REPEAT_WORDS_EN,
        SLOW_WORDS_KO,
        VERY_WORDS_KO,
        BOX_WORDS_KO,
        MIDDLE_WORDS_KO,
        CLEAR_SCREEN_WORDS_EN,
        CLEAR_SCREEN_WORDS_KO,
        DRAW_LINE_WORDS_EN,
        DRAW_LINE_WORDS_KO,
    ]
    .iter()
    .any(|words| token_matches_exact(token, words))
}

fn update_target_name(word: &str) -> Option<String> {
    strip_any_suffix(
        word,
        &[
            "에서", "에게", "한테", "에", "으로", "로", "을", "를", "은", "는",
        ],
    )
    .map(str::to_string)
    .or_else(|| (!word.is_empty()).then(|| word.to_string()))
}

fn parse_update_amount(source: &str, tokens: &[Token]) -> Option<Code> {
    if tokens.is_empty() {
        return None;
    }
    let mut tokens = tokens;
    // The lexer separates `2로` into a number and a particle, so drop a
    // trailing particle token before reading the expression.
    while tokens.len() > 1
        && tokens
            .last()
            .is_some_and(|token| token_matches_exact(token, UPDATE_AMOUNT_PARTICLES_KO))
    {
        tokens = &tokens[..tokens.len() - 1];
    }
    let span = span_of(tokens);
    if is_valid_python_expression(&source[span.start..span.end]) {
        return Some(Code::Source(span));
    }
    // Spoken Korean can also attach the particle: `점수를 2로 나눠`.
    let trimmed = strip_attached_particle_span(source, tokens, UPDATE_AMOUNT_PARTICLES_KO)?;
    is_valid_python_expression(&source[trimmed.start..trimmed.end]).then_some(Code::Source(trimmed))
}

/// Shortens `tokens`' span by one attached Korean particle on the last token.
/// Returns `None` when the last token carries none of them.
fn strip_attached_particle_span(source: &str, tokens: &[Token], particles: &[&str]) -> Option<Span> {
    let last = tokens.last()?;
    let Tok::Name { name } = &last.tok else {
        return None;
    };
    let mut ordered = particles.to_vec();
    ordered.sort_by_key(|particle| std::cmp::Reverse(particle.len()));
    let particle = ordered.into_iter().find(|particle| {
        name.strip_suffix(particle)
            .is_some_and(|base| !base.is_empty())
    })?;
    let start = span_of(tokens).start;
    let end = last.span.end - particle.len();
    (end > start && source.is_char_boundary(end)).then(|| Span::new(start, end))
}

fn is_update_connector(token: &Token, words: &[&str]) -> bool {
    token_matches_exact(token, words)
}

fn update_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UpdateUnparseable,
        "I couldn't understand this value change",
        "값을 어떻게 바꿀지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `score add 1` or `점수에 1 더해`",
        "`점수에 1 더해` 또는 `score add 1`처럼 적어 주세요",
    )
}

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
            DiagnosticCode::BreakCommandUnparseable,
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

// ------------------------------------------------------------------ waiting

/// `wait 3 seconds` / `3초 기다려`. The unit word is optional in English and
/// may be written attached to the number in Korean, which is how people
/// actually type it.
fn match_wait(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `… then show Time to sleep` ends in a waiting word but is a condition
    // with a message, not a wait. The opening word decides the line.
    if tokens
        .first()
        .is_some_and(|token| starts_a_different_statement(token))
    {
        return Ok(None);
    }
    if let Some(consumed) = action_phrase_at(tokens, 0, WAIT_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, 0, WAIT_WORDS_KO, mode))
    {
        return wait_from(source, tokens, &tokens[consumed..], known_names);
    }
    let Some(action_start) = wait_action_ending(tokens, mode) else {
        return Ok(None);
    };
    wait_from(source, tokens, &tokens[..action_start], known_names)
}

/// Builds the wait from its amount region. A region with no number at all is
/// ordinary speech (`잠깐 기다려`), so it falls through to the sentence output
/// rules instead of becoming an error.
fn wait_from(
    source: &str,
    tokens: &[Token],
    amount: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `잠깐 기다려` is ordinary speech, and `잠깐` happens to be a valid Python
    // name, so a wait needs a number or a name the program already knows.
    let mentions_a_number = amount.iter().any(|token| {
        matches!(token.tok, Tok::Int { .. } | Tok::Float { .. })
            || name_word(token).is_some_and(|word| word.starts_with(|c: char| c.is_ascii_digit()))
    });
    let names_a_known_value = amount.len() == 1
        && name_word(&amount[0]).is_some_and(|word| known_names.contains(word));
    if !mentions_a_number && !names_a_known_value {
        return Ok(None);
    }
    if let Some(seconds) = parse_wait_amount(source, amount) {
        return Ok(Some(NmeStmt::Wait { seconds }));
    }
    Err(wait_amount_diagnostic(span_of(tokens)))
}

fn wait_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<usize> {
    action_phrase_at(tokens, start, WAIT_WORDS_EN, mode)
        .or_else(|| action_phrase_at(tokens, start, WAIT_WORDS_KO, mode))
}

/// Index of a wait action that finishes the line, as Korean word order puts it.
fn wait_action_ending(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let mut end = tokens.len();
    while end > 0 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    let start_at = end.saturating_sub(2);
    (start_at..end)
        .find(|&start| wait_action_at(tokens, start, mode).is_some_and(|used| start + used == end))
}

fn parse_wait_amount(source: &str, tokens: &[Token]) -> Option<Code> {
    let mut tokens = tokens;
    // `for` lexes as a Python keyword rather than a word, so it is matched by
    // its token as well as by its spelling.
    while tokens.first().is_some_and(|token| {
        matches!(token.tok, Tok::For) || token_matches_exact(token, WAIT_FILLER_WORDS)
    }) {
        tokens = &tokens[1..];
    }
    while tokens.last().is_some_and(|token| {
        is_command_ending(token)
            || token_matches_exact(token, SECOND_WORDS_EN)
            || token_matches_exact(token, SECOND_WORDS_KO)
            || token_matches_exact(token, WAIT_FILLER_WORDS)
    }) {
        tokens = &tokens[..tokens.len() - 1];
    }
    if tokens.is_empty() {
        return None;
    }
    let span = span_of(tokens);
    if is_valid_python_expression(&source[span.start..span.end]) {
        return Some(Code::Source(span));
    }
    let trimmed = strip_attached_particle_span(source, tokens, SECOND_WORDS_KO)?;
    is_valid_python_expression(&source[trimmed.start..trimmed.end]).then_some(Code::Source(trimmed))
}

fn wait_amount_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::WaitAmountUnparseable,
        "I couldn't understand how long to wait",
        "얼마나 기다릴지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `wait 3 seconds` or `3초 기다려`",
        "`3초 기다려` 또는 `wait 3 seconds`처럼 적어 주세요",
    )
}

// ------------------------------------------------------------- skip a round

/// `skip` / `건너뛰어` — the sentence spelling of Python's `continue`.
fn match_continue(tokens: &[Token], mode: MatchMode) -> Result<Option<NmeStmt>, Diagnostic> {
    let english = action_phrase_at(tokens, 0, CONTINUE_WORDS_EN, mode);
    let Some(consumed) = english.or_else(|| action_phrase_at(tokens, 0, CONTINUE_WORDS_KO, mode))
    else {
        return Ok(None);
    };
    if tokens[consumed..]
        .iter()
        .any(|token| !is_command_ending(token))
    {
        // Korean skip words are ordinary verbs too, so a longer Korean line is
        // left to the sentence rules rather than claimed as a broken command.
        if english.is_none() {
            return Ok(None);
        }
        return Err(Diagnostic::bilingual(
            DiagnosticCode::ContinueCommandUnparseable,
            "I couldn't understand this skip command",
            "이 건너뛰기 명령을 이해하지 못했어요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            "write only `skip` or `건너뛰어`",
            "`건너뛰어` 또는 `skip`만 적어 주세요",
        ));
    }
    Ok(Some(NmeStmt::Continue))
}

// -------------------------------------------------------- adding to a list

/// `append Mina to friends` / `친구들에 민수 넣어`.
///
/// `add` is deliberately not an append word: `add 1 to score` already means a
/// value change, and one spelling may not mean two things.
fn match_append(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // `… then show Time to sleep` ends in a waiting word but is a condition
    // with a message, not a wait. The opening word decides the line.
    if tokens
        .first()
        .is_some_and(|token| starts_a_different_statement(token))
    {
        return Ok(None);
    }
    if let Some(consumed) = action_phrase_at(tokens, 0, APPEND_WORDS_EN, mode) {
        let mut end = tokens.len();
        while end > consumed && is_command_ending(&tokens[end - 1]) {
            end -= 1;
        }
        let rest = &tokens[consumed..end];
        let Some(separator) = rest
            .iter()
            .position(|token| token_matches_exact(token, APPEND_CONNECTORS_EN))
        else {
            return Err(append_diagnostic(span_of(tokens)));
        };
        let (value_tokens, target_tokens) = (&rest[..separator], &rest[separator + 1..]);
        if value_tokens.is_empty() || target_tokens.len() != 1 {
            return Err(append_diagnostic(span_of(tokens)));
        }
        let target = name_word(&target_tokens[0])
            .map(str::to_string)
            .ok_or_else(|| append_diagnostic(target_tokens[0].span))?;
        let value = parse_value(source, value_tokens, known_names, true)
            .map_err(|()| append_diagnostic(span_of(tokens)))?;
        return Ok(Some(NmeStmt::Append { target, value }));
    }

    // Korean puts the action last: `<목록>에 <값> 넣어`.
    let mut end = tokens.len();
    while end > 0 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    let start_at = end.saturating_sub(2);
    let Some(action_start) = (start_at..end).find(|&start| {
        action_phrase_at(tokens, start, APPEND_WORDS_KO, mode)
            .is_some_and(|used| start + used == end)
    }) else {
        return Ok(None);
    };
    if action_start < 2 {
        return Ok(None);
    }
    // The target particle is what separates `친구들에 민수 넣어` from ordinary
    // speech such as `설탕을 넣어`; without it this is not a list line.
    let Some(target) = name_word(&tokens[0])
        .and_then(|word| strip_any_suffix(word, APPEND_TARGET_PARTICLES_KO))
        .map(str::to_string)
    else {
        return Ok(None);
    };
    let value = parse_value(source, &tokens[1..action_start], known_names, true)
        .map_err(|()| append_diagnostic(span_of(tokens)))?;
    Ok(Some(NmeStmt::Append { target, value }))
}

fn append_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AppendUnparseable,
        "I couldn't understand what to add to the list",
        "목록에 무엇을 넣을지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `append Mina to friends` or `친구들에 민수 넣어`",
        "`친구들에 민수 넣어` 또는 `append Mina to friends`처럼 적어 주세요",
    )
}

// ------------------------------------------- slow text, screen, and timing

/// `say slowly Hello` / `천천히 말해줘 안녕`.
///
/// The message is read exactly the way the ordinary output statement reads
/// it, so a name written inside the sentence is still substituted.
fn match_say_slowly(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let start = leading_sentence_fillers(tokens);
    let Some((seconds, value_start)) = slow_speed_at(source, tokens, start, mode) else {
        return Ok(None);
    };
    let body = &tokens[value_start..];
    if body.is_empty() {
        return Err(say_missing(Spelling::English, span_of(tokens)));
    }
    let value = parse_value(source, body, known_names, true)
        .map_err(|()| say_value_unparseable(span_of(body)))?;
    Ok(Some(NmeStmt::SaySlowly { value, seconds }))
}

/// How long to pause between two characters, and where the message starts.
///
/// English puts the speed after the output word (`say very slowly …`);
/// Korean puts it before it (`아주 천천히 말해줘 …`, `3초씩 천천히 말해줘 …`).
fn slow_speed_at(
    source: &str,
    tokens: &[Token],
    start: usize,
    mode: MatchMode,
) -> Option<(Code, usize)> {
    if let Some((_, consumed)) = output_action_at(tokens, start, mode) {
        let mut cursor = start + consumed;
        let very = tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, VERY_WORDS_EN));
        if very {
            cursor += 1;
        }
        if !tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, SLOW_WORDS_EN))
        {
            return None;
        }
        cursor += 1;
        if tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, SLOW_EVERY_WORDS_EN))
        {
            let amount_start = cursor + 1;
            let unit = (amount_start..tokens.len())
                .find(|&index| token_matches_exact(&tokens[index], SECOND_WORDS_EN))?;
            let seconds = parse_wait_amount(source, &tokens[amount_start..=unit])?;
            return Some((seconds, unit + 1));
        }
        let fixed = if very {
            VERY_SLOW_SECONDS
        } else {
            SLOW_SECONDS
        };
        return Some((Code::Generated(fixed.to_string()), cursor));
    }

    let mut cursor = start;
    let mut seconds = None;
    // `3초씩 천천히` — the amount is everything before the `초씩` marker.
    let interval_unit = (start + 1..tokens.len()).find(|&index| {
        token_matches_exact(&tokens[index], SLOW_EVERY_WORDS_KO)
            && tokens
                .get(index + 1)
                .is_some_and(|next| token_matches_exact(next, SLOW_WORDS_KO))
    });
    if let Some(unit) = interval_unit {
        seconds = Some(expression_code(source, &tokens[start..unit])?);
        cursor = unit + 1;
    } else if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, VERY_WORDS_KO))
    {
        seconds = Some(Code::Generated(VERY_SLOW_SECONDS.to_string()));
        cursor += 1;
    }
    if !tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, SLOW_WORDS_KO))
    {
        return None;
    }
    cursor += 1;
    let consumed = action_phrase_at(tokens, cursor, SAY_WORDS_KO, mode)?;
    Some((
        seconds.unwrap_or_else(|| Code::Generated(SLOW_SECONDS.to_string())),
        cursor + consumed,
    ))
}

/// `clear the screen` / `화면 지워`.
fn match_clear_screen(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    fixed_screen_sentence(
        tokens,
        mode,
        CLEAR_SCREEN_WORDS_EN,
        CLEAR_SCREEN_ACTIONS_EN,
        CLEAR_SCREEN_WORDS_KO,
        CLEAR_SCREEN_ACTIONS_KO,
    )
    .then_some(NmeStmt::ClearScreen)
}

/// `draw a line` / `줄 그어`.
fn match_draw_line(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    fixed_screen_sentence(
        tokens,
        mode,
        DRAW_LINE_WORDS_EN,
        DRAW_LINE_ACTIONS_EN,
        DRAW_LINE_WORDS_KO,
        DRAW_LINE_ACTIONS_KO,
    )
    .then_some(NmeStmt::DrawLine)
}

/// A whole-line sentence with no value in it: an English verb and its object
/// (`clear the screen`), or a Korean subject and its verb (`화면 지워`).
///
/// Nothing else may be on the line, so a message that merely mentions the
/// same words (`화면 지워도 되는지 말해줘`) stays a message.
fn fixed_screen_sentence(
    tokens: &[Token],
    mode: MatchMode,
    english_verb: &[&str],
    english_object: &[&str],
    korean_subject: &[&str],
    korean_verb: &[&str],
) -> bool {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, english_verb, mode) {
        let cursor = consumed + usize::from(is_english_article(words.get(consumed)));
        return words.len() == cursor + 1 && token_matches_exact(&words[cursor], english_object);
    }
    words.len() == 2
        && token_matches_exact(&words[0], korean_subject)
        && token_matches_exact(&words[1], korean_verb)
}

/// `say in a box Hello` / `상자로 말해줘 안녕`, and the centred twin. Returns
/// the message; the caller decides which frame to draw around it.
fn framed_say_value(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
    english_frame: &[&str],
    korean_frame: &[&str],
) -> Result<Option<Value>, Diagnostic> {
    let start = leading_sentence_fillers(tokens);
    let value_start = if let Some((_, consumed)) = output_action_at(tokens, start, mode) {
        let mut cursor = start + consumed;
        if !tokens
            .get(cursor)
            .is_some_and(|token| matches!(token.tok, Tok::In))
        {
            return Ok(None);
        }
        cursor += 1;
        cursor += usize::from(is_english_article(tokens.get(cursor)));
        if !tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, english_frame))
        {
            return Ok(None);
        }
        cursor + 1
    } else if tokens
        .get(start)
        .is_some_and(|token| token_matches_exact(token, korean_frame))
    {
        let Some(consumed) = action_phrase_at(tokens, start + 1, SAY_WORDS_KO, mode) else {
            return Ok(None);
        };
        start + 1 + consumed
    } else {
        return Ok(None);
    };
    let body = &tokens[value_start..];
    if body.is_empty() {
        return Err(say_missing(Spelling::English, span_of(tokens)));
    }
    parse_value(source, body, known_names, true)
        .map(Some)
        .map_err(|()| say_value_unparseable(span_of(body)))
}

/// `say in a box Hello` / `상자로 말해줘 안녕`.
fn match_say_in_box(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    Ok(framed_say_value(
        source,
        tokens,
        known_names,
        mode,
        BOX_WORDS_EN,
        BOX_WORDS_KO,
    )?
    .map(|value| NmeStmt::SayInBox { value }))
}

/// `say in the middle Hello` / `가운데 말해줘 안녕`.
fn match_say_in_middle(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    Ok(framed_say_value(
        source,
        tokens,
        known_names,
        mode,
        MIDDLE_WORDS_EN,
        MIDDLE_WORDS_KO,
    )?
    .map(|value| NmeStmt::SayInMiddle { value }))
}

/// `start the timer` / `시간 재기 시작해`.
fn match_start_timer(tokens: &[Token], mode: MatchMode) -> Option<NmeStmt> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, START_TIMER_WORDS_KO, mode) {
        return (consumed == words.len()).then_some(NmeStmt::StartTimer);
    }
    let consumed = action_phrase_at(words, 0, START_TIMER_WORDS_EN, mode)?;
    let cursor = consumed + usize::from(is_english_article(words.get(consumed)));
    (words.len() == cursor + 1 && token_matches_exact(&words[cursor], TIMER_WORDS_EN))
        .then_some(NmeStmt::StartTimer)
}

/// `put door on cooldown for 3 seconds` / `문 쿨타임 3초 걸어`.
fn match_cooldown(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, COOLDOWN_SET_WORDS_EN, mode) {
        let Some(target) = words.get(consumed).and_then(name_word) else {
            return Ok(None);
        };
        let mut cursor = consumed + 1;
        if !words
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, &["on"]))
        {
            return Ok(None);
        }
        cursor += 1;
        if !words
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, COOLDOWN_WORDS_EN))
        {
            return Ok(None);
        }
        cursor += 1;
        let Some(seconds) = parse_wait_amount(source, &words[cursor..]) else {
            return Err(wait_amount_diagnostic(span_of(tokens)));
        };
        return Ok(Some(NmeStmt::Cooldown {
            target: cooldown_target_name(target, known_names),
            seconds,
        }));
    }

    // Korean puts the action last: `<이름> 쿨타임 <n>초 걸어`.
    let start_at = words.len().saturating_sub(2);
    let Some(action_start) = (start_at..words.len()).find(|&start| {
        action_phrase_at(words, start, COOLDOWN_SET_WORDS_KO, mode)
            .is_some_and(|used| start + used == words.len())
    }) else {
        return Ok(None);
    };
    // `<이름>`, `쿨타임`, and at least one word of amount have to come first;
    // without them this is an ordinary sentence that happens to end in `걸어`.
    if action_start < 3 || !token_matches_exact(&words[1], COOLDOWN_WORDS_KO) {
        return Ok(None);
    }
    let Some(target) = name_word(&words[0]) else {
        return Ok(None);
    };
    let Some(seconds) = parse_wait_amount(source, &words[2..action_start]) else {
        return Err(wait_amount_diagnostic(span_of(tokens)));
    };
    Ok(Some(NmeStmt::Cooldown {
        target: cooldown_target_name(target, known_names),
        seconds,
    }))
}

/// `wait for door` / `문 쿨타임 끝날때까지 기다려`.
fn match_cooldown_wait(
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Option<NmeStmt> {
    let words = trim_command_endings(tokens);
    if let Some(consumed) = action_phrase_at(words, 0, WAIT_WORDS_EN, mode) {
        if words.len() != consumed + 2 || !matches!(words[consumed].tok, Tok::For) {
            return None;
        }
        let target = cooldown_target_name(name_word(&words[consumed + 1])?, known_names);
        // `wait for pause_length` reads as a length of time when the program
        // already has a `pause_length`, so a name that is known but is not a
        // cooldown is left to the ordinary wait rules.
        let is_cooldown = known_names.contains(&format!("{COOLDOWN_PREFIX}{target}"));
        return (is_cooldown || !known_names.contains(&target))
            .then_some(NmeStmt::WaitForCooldown { target });
    }
    if words.len() < 4 {
        return None;
    }
    let action_start = words.len() - 1;
    if !token_matches_exact(&words[action_start], WAIT_WORDS_KO)
        || !token_matches_exact(&words[1], COOLDOWN_WORDS_KO)
    {
        return None;
    }
    let target = name_word(&words[0])?;
    let until = action_phrase_at(words, 2, COOLDOWN_UNTIL_WORDS_KO, mode)?;
    (2 + until == action_start).then(|| NmeStmt::WaitForCooldown {
        target: cooldown_target_name(target, known_names),
    })
}

/// `<name> is ready` / `<이름> 쿨타임이 끝났으면`, plus the index the inline
/// body starts at. Both spellings are ordinary conditions, so they work in
/// `when`, `while`, `else if`, and the one-line forms of all three.
fn cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    english_cooldown_condition_at(tokens, start)
        .or_else(|| korean_cooldown_condition_at(tokens, start))
}

fn english_cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    let target = name_word(tokens.get(start)?)?;
    if token_word(tokens.get(start + 1)?) != Some("is") {
        return None;
    }
    let (ready, mut body_start) = if tokens
        .get(start + 2)
        .is_some_and(|token| token_matches_exact(token, COOLDOWN_READY_WORDS_EN))
    {
        (true, start + 3)
    } else if tokens
        .get(start + 2)
        .is_some_and(|token| token_matches_exact(token, &["on"]))
        && tokens
            .get(start + 3)
            .is_some_and(|token| token_matches_exact(token, COOLDOWN_WORDS_EN))
    {
        (false, start + 4)
    } else {
        return None;
    };
    // `then` separates the condition from a one-line body, exactly as it
    // does after every other English condition.
    if tokens
        .get(body_start)
        .is_some_and(|token| token_word(token) == Some("then"))
    {
        body_start += 1;
    }
    Some((cooldown_condition(target, ready), body_start))
}

fn korean_cooldown_condition_at(tokens: &[Token], start: usize) -> Option<(Condition, usize)> {
    let target = name_word(tokens.get(start)?)?;
    if !token_matches_exact(tokens.get(start + 1)?, COOLDOWN_WORDS_KO) {
        return None;
    }
    let marker = tokens.get(start + 2)?;
    let ready = if token_matches_exact(marker, COOLDOWN_READY_WORDS_KO) {
        true
    } else if token_matches_exact(marker, COOLDOWN_BUSY_WORDS_KO) {
        false
    } else {
        return None;
    };
    Some((cooldown_condition(target, ready), start + 3))
}

/// The Python behind `is ready` and `is on cooldown`. It is written here
/// rather than taken from the source, because the source never spells it.
fn cooldown_condition(target: &str, ready: bool) -> Condition {
    let operator = if ready { ">=" } else { "<" };
    Condition::Truthy {
        value: ConditionValue::Python(Code::Generated(format!(
            "__import__(\"time\").time() {operator} {COOLDOWN_PREFIX}{target}"
        ))),
        negated: false,
    }
}

/// The NME name a cooldown belongs to. A Korean particle is only removed
/// when the program already knows the shorter name, exactly as everywhere
/// else, so a name that merely ends in a particle survives whole.
fn cooldown_target_name(word: &str, known_names: &HashSet<String>) -> String {
    resolve_known_particle(word, known_names)
        .unwrap_or(word)
        .to_string()
}

/// `elapsed` / `잰시간` standing alone as a value.
///
/// A name the program made itself always wins, so a program with its own
/// `elapsed` keeps it.
fn parse_elapsed_value(tokens: &[Token], known_names: &HashSet<String>) -> Option<Value> {
    (tokens.len() == 1 && is_elapsed_word(&tokens[0], known_names)).then_some(Value::Elapsed)
}

fn is_elapsed_word(token: &Token, known_names: &HashSet<String>) -> bool {
    name_word(token).is_some_and(|word| {
        !known_names.contains(word)
            && (ELAPSED_WORDS_EN.contains(&word) || ELAPSED_WORDS_KO.contains(&word))
    })
}

/// True when this statement reads the stopwatch, so the parser can say that
/// the timer was never started instead of leaving a `NameError` for later.
fn reads_elapsed(stmt: &NmeStmt) -> bool {
    match stmt {
        NmeStmt::Say { value }
        | NmeStmt::Set { value, .. }
        | NmeStmt::Append { value, .. }
        | NmeStmt::FileWrite { value, .. }
        | NmeStmt::SayInBox { value }
        | NmeStmt::SayInMiddle { value }
        | NmeStmt::SaySlowly { value, .. } => value_reads_elapsed(value),
        NmeStmt::Ask { prompt, .. } => prompt.as_ref().is_some_and(value_reads_elapsed),
        NmeStmt::When { condition, inline }
        | NmeStmt::While { condition, inline }
        | NmeStmt::ElseIf { condition, inline } => {
            condition_reads_elapsed(condition) || inline_reads_elapsed(inline.as_ref())
        }
        NmeStmt::Else { inline } => inline_reads_elapsed(inline.as_ref()),
        NmeStmt::Times { inline, .. } | NmeStmt::ForEach { inline, .. } => {
            inline_reads_elapsed(inline.as_ref())
        }
        _ => false,
    }
}

fn value_reads_elapsed(value: &Value) -> bool {
    match value {
        Value::Elapsed => true,
        Value::List(items) => items.iter().any(value_reads_elapsed),
        _ => false,
    }
}

fn condition_reads_elapsed(condition: &Condition) -> bool {
    match condition {
        Condition::Truthy { value, .. } => condition_value_reads_elapsed(value),
        Condition::Compare { left, right, .. } => {
            condition_value_reads_elapsed(left) || condition_value_reads_elapsed(right)
        }
        Condition::Logical { left, right, .. } => {
            condition_reads_elapsed(left) || condition_reads_elapsed(right)
        }
        Condition::Python(_) => false,
    }
}

fn condition_value_reads_elapsed(value: &ConditionValue) -> bool {
    matches!(value, ConditionValue::Python(Code::Generated(text)) if text == ELAPSED_PYTHON)
}

fn inline_reads_elapsed(inline: Option<&InlineStmt>) -> bool {
    matches!(inline, Some(InlineStmt::Nme(inner)) if reads_elapsed(inner))
}

fn timer_not_started_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::TimerNotStarted,
        "the timer has not been started yet",
        "시간 재기를 아직 시작하지 않았어요",
        span,
    )
    .with_bilingual_hint(
        "write `start the timer` on an earlier line",
        "앞 줄에 `시간 재기 시작해`라고 적어 주세요",
    )
}

/// The words of a line, with a trailing `?`, `!`, or `.` dropped.
fn trim_command_endings(tokens: &[Token]) -> &[Token] {
    let mut end = tokens.len();
    while end > 0 && is_command_ending(&tokens[end - 1]) {
        end -= 1;
    }
    &tokens[..end]
}

fn is_english_article(token: Option<&Token>) -> bool {
    token.is_some_and(|token| token_matches_exact(token, &["a", "an", "the"]))
}

fn say_value_unparseable(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::SayValueUnparseable,
        "I couldn't understand what to show",
        "무엇을 말할지 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write a value, or a sentence such as `show Hello world`",
        "`안녕하세요 말해줘`처럼 평범한 문장으로 적어도 돼요",
    )
}

// -------------------------------------------------------- repeat over a list

/// `for each name in names` / `이름들의 이름마다 반복해`.
fn match_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(stmt) = match_english_for_each(source, tokens, block, known_names, mode)? {
        return Ok(Some(stmt));
    }
    match_korean_for_each(source, tokens, block, known_names, mode)
}

/// Index of the loop variable in `for each <name> in <items>`, allowing an
/// optional leading repeat word.
fn english_for_each_start(tokens: &[Token], mode: MatchMode) -> Option<usize> {
    let start = repeat_action_at(tokens, 0, mode).map_or(0, |(_, consumed)| consumed);
    if !matches!(tokens.get(start)?.tok, Tok::For) {
        return None;
    }
    tokens
        .get(start + 1)
        .is_some_and(|token| token_matches_exact(token, EACH_WORDS_EN))
        .then_some(start + 2)
}

fn match_english_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(name_at) = english_for_each_start(tokens, mode) else {
        return Ok(None);
    };
    let Some(name) = tokens.get(name_at).and_then(name_word).map(str::to_string) else {
        return Err(for_each_diagnostic(span_of(tokens)));
    };
    if !tokens
        .get(name_at + 1)
        .is_some_and(|token| matches!(token.tok, Tok::In))
    {
        return Err(for_each_diagnostic(span_of(tokens)));
    }
    let tail = &tokens[name_at + 2..];
    let colon_at = tail.iter().position(|token| {
        matches!(token.tok, Tok::Colon | Tok::And) || token_matches_exact(token, &["then"])
    });
    let items_tokens = colon_at.map_or(tail, |at| &tail[..at]);
    let Some(items) = expression_code(source, items_tokens) else {
        return Err(for_each_diagnostic(span_of(tokens)));
    };
    let header_end = colon_at.map_or(span_of(tokens).end, |at| tail[at].span.end);
    let body = colon_at.map_or(&tail[tail.len()..], |at| &tail[at + 1..]);
    // The loop name is bound by the header, so the body may already use it.
    let mut body_names = known_names.clone();
    body_names.insert(name.clone());
    let inline = parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Repeat,
        Span::new(tokens[0].span.start, header_end),
        &body_names,
    )?;
    Ok(Some(NmeStmt::ForEach {
        name,
        items,
        inline,
    }))
}

/// True when the line looks like `<목록>의 <이름>마다 ...`.
fn korean_for_each_shape(tokens: &[Token]) -> bool {
    let Some(name_at) = tokens.iter().position(|token| {
        name_word(token).is_some_and(|word| {
            word.strip_suffix(EACH_SUFFIX_KO)
                .is_some_and(|base| !base.is_empty())
        })
    }) else {
        return false;
    };
    name_at > 0
        && (repeat_action_at(&tokens[name_at + 1..], 0, MatchMode::Exact).is_some()
            || tokens[name_at + 1..]
                .iter()
                .any(|token| matches!(token.tok, Tok::Colon)))
}

fn match_korean_for_each(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // The loop variable is the word ending in `마다`; everything before it is
    // the collection.
    let Some(name_at) = tokens.iter().position(|token| {
        name_word(token).is_some_and(|word| {
            word.strip_suffix(EACH_SUFFIX_KO)
                .is_some_and(|base| !base.is_empty())
        })
    }) else {
        return Ok(None);
    };
    if name_at == 0 {
        return Ok(None);
    }
    let rest = &tokens[name_at + 1..];
    let colon_at = rest.iter().position(|token| matches!(token.tok, Tok::Colon));
    let repeat_consumed = repeat_action_at(rest, 0, mode).map(|(_, consumed)| consumed);
    // Without a closing repeat word or a colon this is ordinary speech.
    let Some(body_at) = repeat_consumed.or_else(|| colon_at.map(|at| at + 1)) else {
        return Ok(None);
    };
    let name = name_word(&tokens[name_at])
        .and_then(|word| word.strip_suffix(EACH_SUFFIX_KO))
        .map(str::to_string)
        .ok_or_else(|| for_each_diagnostic(span_of(tokens)))?;
    let items_tokens = &tokens[..name_at];
    // `친구들의` is a valid Python name on its own, so the particle has to be
    // taken off first or the loop would read over a name nobody defined.
    let items = strip_attached_particle_span(source, items_tokens, EACH_CONTAINER_PARTICLES_KO)
        .filter(|span| is_valid_python_expression(&source[span.start..span.end]))
        .map(Code::Source)
        .or_else(|| expression_code(source, items_tokens))
        .ok_or_else(|| for_each_diagnostic(span_of(tokens)))?;
    let header_end = colon_at.map_or(tokens[name_at].span.end, |at| rest[at].span.end);
    let mut body_names = known_names.clone();
    body_names.insert(name.clone());
    let inline = parse_suite_body(
        source,
        &rest[body_at.min(rest.len())..],
        block,
        SuiteKind::Repeat,
        Span::new(tokens[0].span.start, header_end),
        &body_names,
    )?;
    Ok(Some(NmeStmt::ForEach {
        name,
        items,
        inline,
    }))
}

fn expression_code(source: &str, tokens: &[Token]) -> Option<Code> {
    if tokens.is_empty() {
        return None;
    }
    let span = span_of(tokens);
    is_valid_python_expression(&source[span.start..span.end]).then_some(Code::Source(span))
}

fn for_each_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ForEachUnparseable,
        "I couldn't understand this repeat-over-a-list line",
        "이 목록 반복 줄을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `for each name in names` or `이름들의 이름마다 반복해`",
        "`이름들의 이름마다 반복해` 또는 `for each name in names`처럼 적어 주세요",
    )
}


#[allow(clippy::too_many_lines)]
fn match_while(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // Spoken Korean often puts the loop ending on the subject: `준비하는
    // 동안` may be tokenized as the single name `준비하는동안`. Split only
    // these documented endings; Python-valid names still win before this
    // matcher is reached.
    if let Some(subject) = tokens.first().and_then(split_attached_while_token) {
        let condition = parse_natural_condition(
            source,
            std::slice::from_ref(&subject),
            None,
            known_names,
            Spelling::Korean,
        )?;
        let inline = parse_suite_body(
            source,
            &tokens[1..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::While { condition, inline }));
    }
    if let Some((condition_tokens, body_start)) = korean_while_connector(tokens) {
        if let Ok(condition) = parse_natural_condition(
            source,
            &condition_tokens,
            None,
            known_names,
            Spelling::Korean,
        ) {
            let inline = parse_suite_body(
                source,
                &tokens[body_start..],
                block,
                SuiteKind::Condition,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While { condition, inline }));
        }
    }
    let (spelling, condition_start, condition_end, trailing_while) =
        if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While))
            || action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).is_some()
        {
            let consumed = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::While)) {
                1
            } else {
                action_phrase_at(tokens, 0, WHILE_WORDS_EN, mode).expect("checked above")
            };
            // `while 준비 동안 성공 말해줘` and `while 점수가 3보다 작을 동안`
            // mix the English keyword with a Korean while ending. Split the
            // ending exactly like the Korean spellings so it cannot be
            // lowered as the loop's inline body.
            if let Some((condition_tokens, body_rel)) = korean_while_connector(&tokens[consumed..])
            {
                if let Ok(condition) = parse_natural_condition(
                    source,
                    &condition_tokens,
                    None,
                    known_names,
                    Spelling::Korean,
                ) {
                    let inline = parse_suite_body(
                        source,
                        &tokens[consumed + body_rel..],
                        block,
                        SuiteKind::Condition,
                        span_of(tokens),
                        known_names,
                    )?;
                    return Ok(Some(NmeStmt::While { condition, inline }));
                }
            }
            // A Korean while ending may also close a comparison condition
            // after the English keyword: `while 점수가 3보다 작을 동안`.
            let trailing = tokens
                .last()
                .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO));
            if trailing && tokens.len() > consumed + 1 {
                (Spelling::English, consumed, tokens.len() - 1, true)
            } else {
                (Spelling::English, consumed, tokens.len(), false)
            }
        } else if action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).is_some() {
            let consumed =
                action_phrase_at(tokens, 0, WHILE_WORDS_KO, mode).expect("checked above");
            (Spelling::Korean, consumed, tokens.len(), false)
        } else if tokens.len() > 1
            && tokens
                .last()
                .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
        {
            (Spelling::Korean, 0, tokens.len() - 1, true)
        } else {
            return Ok(None);
        };

    let condition_slice = &tokens[condition_start..condition_end];
    if condition_slice.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if !trailing_while {
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
            return Ok(Some(NmeStmt::While {
                condition: Condition::Python(Code::Source(condition_span)),
                inline,
            }));
        }
    }

    if !trailing_while {
        if let Some((condition, body_start)) = cooldown_condition_at(tokens, condition_start) {
            let inline = parse_suite_body(
                source,
                &tokens[body_start..],
                block,
                SuiteKind::Condition,
                span_of(tokens),
                known_names,
            )?;
            return Ok(Some(NmeStmt::While { condition, inline }));
        }
    }
    let (condition_tokens, body_start, connector) = if trailing_while {
        if let Some((relative_at, connector)) = find_condition_connector(condition_slice) {
            let (condition, _, connector) =
                condition_tokens_before(tokens, condition_start, relative_at, connector);
            (condition, tokens.len(), Some(connector))
        } else {
            (condition_slice.to_vec(), tokens.len(), None)
        }
    } else if let Some((relative_at, connector)) = find_condition_connector(condition_slice) {
        let (condition, body_start, connector) =
            condition_tokens_before(tokens, condition_start, relative_at, connector);
        (condition, body_start, Some(connector))
    } else {
        (tokens[condition_start..].to_vec(), tokens.len(), None)
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
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
    if let Some((condition, body_start)) = cooldown_condition_at(tokens, condition_start) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::ElseIf { condition, inline }));
    }
    let remainder = &tokens[condition_start..];
    let (condition_tokens, body_start, connector) = match find_condition_connector(remainder) {
        Some((relative_at, connector)) => {
            let (condition, body_start, connector) =
                condition_tokens_before(tokens, condition_start, relative_at, connector);
            (condition, body_start, Some(connector))
        }
        None => (remainder.to_vec(), tokens.len(), None),
    };
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::ElseIf { condition, inline }))
}

// -------------------------------------------------------------- condition

/// Match a conversational condition whose subject comes first, for example
/// `name exists then show hello` or `색이 빨강과 같으면 말해 yes`.
///
/// The explicit `if`/`만약` forms remain the clearest spelling, but accepting
/// the subject-first form is important for learners who are writing a spoken
/// sentence rather than translating Python word-for-word.  A bare `then`
/// sentence is only claimed when its body has an unmistakable action; this
/// keeps ordinary prose such as `Hello then world` as prose.
fn match_subject_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // Explicit starters and other high-confidence sentence actions own the
    // line. Without this guard, a normal `if ... then ...` or `3 times ...`
    // line could be re-read as a subject-first condition because it contains
    // a comparison word somewhere in its body.
    if when_action_at(tokens, 0, MatchMode::Recover).is_some()
        || repeat_action_at(tokens, 0, MatchMode::Recover).is_some()
        || attached_korean_times_sentence(source, tokens).is_some()
        || find_count_marker(tokens, MatchMode::Exact).is_some()
        || ask_action_at(tokens, 0, MatchMode::Recover).is_some()
        || output_action_at(tokens, 0, MatchMode::Recover).is_some()
        || set_action_at(tokens, 0, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_EN, MatchMode::Recover).is_some()
        || action_phrase_at(tokens, 0, WHILE_WORDS_KO, MatchMode::Recover).is_some()
        || matches!(tokens.first().map(|token| &token.tok), Some(Tok::While))
        || tokens
            .last()
            .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO))
    {
        return Ok(None);
    }
    // `문 쿨타임이 끝났으면 발사 말해줘` — the Korean cooldown condition also
    // works without an explicit `만약`. Only the Korean spelling is claimed
    // here: bare `door is ready` is a valid Python line and stays Python.
    if let Some((condition, body_start)) = korean_cooldown_condition_at(tokens, 0) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When { condition, inline }));
    }
    let Some((relative_at, connector)) = find_condition_connector(tokens) else {
        return Ok(None);
    };
    if mode == MatchMode::Exact && find_exact_condition_connector(tokens).is_none() {
        return Ok(None);
    }
    let attached_subject = relative_at == 0
        && split_attached_condition_token(&tokens[0])
            .is_some_and(|(_, attached)| attached == connector);
    if relative_at == 0 && !attached_subject {
        return Ok(None);
    }
    if mode == MatchMode::Recover
        && find_exact_condition_connector(tokens).is_none()
        && (output_action_at(tokens, relative_at, MatchMode::Recover).is_some()
            || !recovered_condition_connector_is_plausible(&tokens[relative_at]))
    {
        return Ok(None);
    }
    let (condition_tokens, body_start, connector) =
        condition_tokens_before(tokens, 0, relative_at, connector);
    if condition_tokens.is_empty() {
        return Ok(None);
    }
    if matches!(connector, ConditionConnector::Then)
        && !subject_condition_body_is_action(&tokens[body_start..], mode)
    {
        return Ok(None);
    }
    let condition = parse_natural_condition(
        source,
        &condition_tokens,
        Some(connector),
        known_names,
        Spelling::Korean,
    )?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
        block,
        SuiteKind::Condition,
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

fn recovered_condition_connector_is_plausible(token: &Token) -> bool {
    let Some(word) = token_word(token) else {
        return false;
    };
    word.is_ascii()
        || word
            .chars()
            .last()
            .is_some_and(|character| matches!(character, '면' | '먄'))
}

fn subject_condition_shape(tokens: &[Token]) -> bool {
    let Some((relative_at, connector)) = find_condition_connector(tokens) else {
        return false;
    };
    let attached_subject = relative_at == 0
        && split_attached_condition_token(&tokens[0])
            .is_some_and(|(_, attached)| attached == connector);
    if relative_at == 0 && !attached_subject {
        return false;
    }
    let (_, body_start, _) = condition_tokens_before(tokens, 0, relative_at, connector);
    !matches!(connector, ConditionConnector::Then)
        || subject_condition_body_is_action(&tokens[body_start..], MatchMode::Exact)
}

fn subject_condition_body_is_action(tokens: &[Token], mode: MatchMode) -> bool {
    if tokens.is_empty() {
        return false;
    }
    output_action_at(tokens, 0, mode).is_some()
        || output_action_ending(tokens, mode).is_some()
        || ask_action_at(tokens, 0, mode).is_some()
        || set_action_at(tokens, 0, mode).is_some()
        || update_action_at(tokens, 0, mode).is_some()
        || action_phrase_at(tokens, 0, BREAK_WORDS_EN, mode).is_some()
        || action_phrase_at(tokens, 0, BREAK_WORDS_KO, mode).is_some()
}

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

    // A Korean cooldown condition ends in its own connector (`끝났으면`), so
    // it has to be read before the generic connector search cuts that word
    // off and leaves a meaningless comparison behind.
    if let Some((condition, body_start)) = cooldown_condition_at(tokens, consumed) {
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Condition,
            span_of(tokens),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When { condition, inline }));
    }

    let natural = find_condition_connector(&tokens[consumed..]);
    if !starter_exact && natural.is_none() && matches!(block, BlockCtx::Inline) {
        // A short sentence word may be one edit away from a condition alias.
        // Without a connector, colon, or following block there is not enough
        // evidence to recover it as a typo, so let another construct decide.
        return Ok(None);
    }
    let (condition_tokens, body_start, connector) = match natural {
        Some((relative_at, connector)) => {
            let (condition, body_start, connector) =
                condition_tokens_before(tokens, consumed, relative_at, connector);
            (condition, body_start, Some(connector))
        }
        None => (tokens[consumed..].to_vec(), tokens.len(), None),
    };
    if condition_tokens.is_empty() {
        return Err(condition_missing(spelling, tokens[0].span));
    }
    let condition =
        parse_natural_condition(source, &condition_tokens, connector, known_names, spelling)?;
    let inline = parse_suite_body(
        source,
        &tokens[body_start..],
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
    NotEquals,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

fn find_exact_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return find_exact_condition_connector(inner)
            .map(|(index, connector)| (index + 1, connector));
    }
    // Spoken Korean splits `<=`/`>=` into two tokens (`10보다 작거나
    // 같으면`); the lone `같으면` would otherwise match equality.
    for (index, pair) in tokens.windows(2).enumerate() {
        if token_word(&pair[0]) == Some("작거나") && token_word(&pair[1]) == Some("같으면") {
            return Some((index, ConditionConnector::LessOrEqual));
        }
        if token_word(&pair[0]) == Some("크거나") && token_word(&pair[1]) == Some("같으면") {
            return Some((index, ConditionConnector::GreaterOrEqual));
        }
    }
    let last_operand = last_logical_operand_start(tokens);
    let exact = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if is_or_equal_phrase_at(tokens, index)
                || tokens.get(index.saturating_sub(1)).is_some_and(|previous| {
                    token_word(previous) == Some("or") && is_or_equal_phrase_at(tokens, index - 1)
                })
            {
                return None;
            }
            let connector = condition_connector_exact(token, index + 1 == tokens.len())?;
            // A `then` body marker may sit before the final `and`/`or`
            // operand because it separates the inline body. Korean
            // comparison endings may only close the final operand, so an
            // earlier comparison (`점수가 0보다 크면 그리고 ...`) stays
            // intact instead of being cut at its ending.
            (index >= last_operand || token_word(token) == Some("then"))
                .then_some((index, connector))
        })
        .collect::<Vec<_>>();
    if exact.is_empty() {
        // Spoken Korean splits the negation into two tokens: `같지 않으면`,
        // `같지 않다면`, or `같지 않을` at the end of a condition.
        for (index, pair) in tokens.windows(2).enumerate() {
            if index >= last_operand
                && token_word(&pair[0]) == Some("같지")
                && matches!(token_word(&pair[1]), Some("않으면" | "않다면" | "않을"))
            {
                return Some((index, ConditionConnector::NotEquals));
            }
        }
    }
    exact
        .iter()
        .copied()
        .find(|(_, connector)| *connector == ConditionConnector::Then)
        .or_else(|| exact.first().copied())
}

/// First token index of the last `and`/`or` operand at bracket depth zero.
/// English `then` ends the condition scan: logical words after it belong to
/// the inline body (`if a then show x or y`).
fn last_logical_operand_start(tokens: &[Token]) -> usize {
    let mut last = 0usize;
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 0 {
            if token_word(token) == Some("then") {
                break;
            }
            // `or equal` belongs to a `less/greater than or equal to`
            // comparison, not to logical `or`.
            if token_word(token) == Some("or") && is_or_equal_phrase_at(tokens, index) {
                continue;
            }
            if token_matches_exact(token, &["and", "or", "그리고", "또는"]) {
                last = index + 1;
            }
        }
    }
    last
}

/// True when `tokens[index]` is `or` followed by `equal`/`equals`, the
/// natural-language `<=`/`>=` phrase.
fn is_or_equal_phrase_at(tokens: &[Token], index: usize) -> bool {
    token_word(&tokens[index]) == Some("or")
        && tokens
            .get(index + 1)
            .is_some_and(|token| matches!(token_word(token), Some("equal" | "equals")))
}

fn find_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return find_condition_connector(inner).map(|(index, connector)| (index + 1, connector));
    }
    if let Some(connector) = find_exact_condition_connector(tokens) {
        return Some(connector);
    }

    // Only recover a connector typo when the whole condition has no exact
    // connector. Otherwise `than ... then` could split at `than`, because it
    // is one edit away from `then`.
    let last_operand = last_logical_operand_start(tokens);
    let recovered = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if is_or_equal_phrase_at(tokens, index)
                || tokens.get(index.saturating_sub(1)).is_some_and(|previous| {
                    token_word(previous) == Some("or") && is_or_equal_phrase_at(tokens, index - 1)
                })
            {
                return None;
            }
            let connector = condition_connector_recovered(token, index + 1 == tokens.len())?;
            (index >= last_operand || token_word(token) == Some("then"))
                .then_some((index, connector))
        })
        .collect::<Vec<_>>();
    (recovered.len() == 1).then(|| recovered[0])
}

/// Returns a condition token with a Korean connector suffix removed. Korean
/// writers commonly attach endings (`name이면`, `준비있으면`) to the preceding
/// name, while the Python tokenizer quite correctly keeps the whole word as
/// one identifier. The parser can split that one token without touching the
/// source bytes used for diagnostics or lowering.
fn split_attached_condition_token(token: &Token) -> Option<(Token, ConditionConnector)> {
    let word = name_word(token)?;
    // Leave a one-edit misspelling of a complete connector for the bounded
    // recovery path below. Otherwise the generic `면` suffix would turn
    // `잇으면` into the literal value `잇으` before recovery gets a chance.
    let full_connectors = [
        "그러면",
        "그럼",
        "하면",
        "이면",
        "이라면",
        "있으면",
        "있다면",
        "없으면",
        "없다면",
        "같으면",
        "같다면",
        "같지않으면",
        "같지않다면",
        "같지않을",
        "크면",
        "크다면",
        "작으면",
        "작다면",
    ];
    if full_connectors.iter().any(|candidate| {
        word != *candidate && word.chars().count() >= 2 && one_typo_away(word, candidate)
    }) {
        return None;
    }
    // Do not reinterpret a connector token itself as a value plus the short
    // `면` ending. For example, `같으면` must remain the equality connector;
    // otherwise its generic suffix would produce the bogus right-hand value
    // `같으`.
    if [
        "그러면",
        "그렇다면",
        "있으면",
        "없으면",
        "같으면",
        "같지않으면",
        "같지않다면",
        "같지않을",
        "크면",
        "작으면",
        "하면",
        "이면",
        "이라면",
        "라면",
        "면",
        "같먄",
        "있먄",
        "없먄",
        "크먄",
        "작먄",
        "라먄",
        "있으먄",
        "없으먄",
        "같으먄",
        "크으먄",
        "작으먄",
        "먄",
    ]
    .contains(&word)
    {
        return None;
    }
    let (suffix, connector) = [
        ("그러면", ConditionConnector::Then),
        ("그렇다면", ConditionConnector::Then),
        ("있으면", ConditionConnector::Exists),
        ("없으면", ConditionConnector::Missing),
        ("같으면", ConditionConnector::Equals),
        ("크면", ConditionConnector::Greater),
        ("작으면", ConditionConnector::Less),
        ("있으먄", ConditionConnector::Exists),
        ("없으먄", ConditionConnector::Missing),
        ("같으먄", ConditionConnector::Equals),
        ("크으먄", ConditionConnector::Greater),
        ("작으먄", ConditionConnector::Less),
        // Korean speakers often attach the short comparison ending to the
        // right-hand value: `이름이 철수면` / `준비가 거짓이면` /
        // `이름이 철수라면`. Treat those forms as equality, while keeping
        // the bare words `면` and `라면` ordinary text.
        ("이라면", ConditionConnector::Then),
        ("라면", ConditionConnector::Equals),
        ("이면", ConditionConnector::Then),
        ("하면", ConditionConnector::Then),
        ("먄", ConditionConnector::Then),
        ("면", ConditionConnector::Equals),
    ]
    .into_iter()
    .find(|(suffix, _)| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    let base = word.strip_suffix(suffix)?;
    Some((
        Token {
            tok: Tok::Name {
                name: base.to_string(),
            },
            span: Span::new(token.span.start, base_end),
        },
        connector,
    ))
}

fn split_attached_while_token(token: &Token) -> Option<Token> {
    let word = name_word(token)?;
    let suffix = ["하는동안", "할동안", "동안"]
        .into_iter()
        .find(|suffix| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base = word.strip_suffix(suffix)?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    Some(Token {
        tok: Tok::Name {
            name: base.to_string(),
        },
        span: Span::new(token.span.start, base_end),
    })
}

fn korean_while_connector(tokens: &[Token]) -> Option<(Vec<Token>, usize)> {
    // Prefer the last `동안` so a logical condition may carry an ending on
    // every operand: `점수가 5와 같지 않을 동안 그리고 점수가 0보다 클 동안`.
    // Earlier `동안` markers are loop endings too and only describe how the
    // operands are spoken, so they are dropped from the condition tokens. A
    // leading Korean while word is also dropped here; keeping it would make
    // an outer parenthesized condition start with the loop keyword instead
    // of its actual subject.
    let condition_start = usize::from(
        tokens
            .first()
            .is_some_and(|token| token_matches_exact(token, WHILE_WORDS_KO)),
    );
    for (index, token) in tokens.iter().enumerate().skip(condition_start + 1).rev() {
        if !token_matches_exact(token, &["동안"]) {
            continue;
        }
        let mut condition = tokens[condition_start..index]
            .iter()
            .filter(|token| !token_matches_exact(token, &["동안"]))
            .cloned()
            .collect::<Vec<_>>();
        if condition
            .last()
            .is_some_and(|last| token_matches_exact(last, &["하는", "할"]))
        {
            condition.pop();
        } else if let Some(last) = condition.last_mut() {
            if let Some(base) = split_while_participle(last) {
                *last = base;
            }
        }
        if !condition.is_empty() {
            let mut body_start = index + 1;
            while tokens
                .get(body_start)
                .is_some_and(|token| matches!(token.tok, Tok::Rpar | Tok::Rsqb | Tok::Rbrace))
            {
                condition.push(tokens[body_start].clone());
                body_start += 1;
            }
            // A Korean comparison ending may appear before a logical
            // connector inside a whole wrapper. It is part of the condition,
            // not the loop boundary, so keep the remaining wrapped tokens
            // while dropping only the spoken `동안` markers.
            if tokens
                .get(body_start)
                .is_some_and(|token| token_matches_exact(token, &["and", "or", "그리고", "또는"]))
            {
                if let Some(wrapper_end) = condition_wrapper_end(tokens, condition_start) {
                    condition.extend(
                        tokens[body_start..=wrapper_end]
                            .iter()
                            .filter(|token| !token_matches_exact(token, &["동안"]))
                            .cloned(),
                    );
                    body_start = wrapper_end + 1;
                }
            }
            return Some((condition, body_start));
        }
    }
    None
}

fn split_while_participle(token: &Token) -> Option<Token> {
    let word = name_word(token)?;
    let suffix = ["하는", "할"]
        .into_iter()
        .find(|suffix| word.ends_with(suffix) && word.len() > suffix.len())?;
    let base = word.strip_suffix(suffix)?;
    let base_end = token.span.end.saturating_sub(suffix.len());
    Some(Token {
        tok: Tok::Name {
            name: base.to_string(),
        },
        span: Span::new(token.span.start, base_end),
    })
}

fn condition_tokens_before(
    tokens: &[Token],
    start: usize,
    relative_connector_at: usize,
    connector: ConditionConnector,
) -> (Vec<Token>, usize, ConditionConnector) {
    let at = start + relative_connector_at;
    let mut condition = tokens[start..at].to_vec();
    let mut body_start = at + 1;
    let mut connector = connector;
    // A split Korean negation spans two tokens (`같지 않으면`), so the body
    // starts after the second one rather than after the connector word.
    if connector == ConditionConnector::NotEquals
        && tokens
            .get(at)
            .is_some_and(|token| token_word(token) == Some("같지"))
        && tokens
            .get(at + 1)
            .is_some_and(|next| matches!(token_word(next), Some("않으면" | "않다면" | "않을")))
    {
        body_start = at + 2;
    }
    // `작거나 같으면` / `크거나 같으면` also spans two tokens.
    if matches!(
        connector,
        ConditionConnector::LessOrEqual | ConditionConnector::GreaterOrEqual
    ) && tokens
        .get(at + 1)
        .is_some_and(|next| token_word(next) == Some("같으면"))
    {
        body_start = at + 2;
    }
    if let Some(token) = tokens.get(at) {
        if let Some((base, attached_connector)) = split_attached_condition_token(token) {
            // `name이면` is a truthy condition when it is the whole subject,
            // but `ready가 거짓이면` is naturally an equality comparison.
            // The preceding condition tokens provide the disambiguating
            // context without making the lexer guess from raw source text.
            let context_equality = attached_connector == ConditionConnector::Then
                && !condition.is_empty()
                && name_word(token).is_some_and(|word| {
                    word.ends_with("이면") || word.ends_with("이라면") || word.ends_with("먄")
                });
            // A short ending attached directly to the only subject —
            // `준비면` / `준비라면` — is a truthy condition, not an equality
            // with a missing right-hand value. Equality needs a preceding
            // subject and a separate right-hand word, as in `이름이 철수면`.
            let subject_only_then =
                attached_connector == ConditionConnector::Equals && condition.is_empty();
            if context_equality {
                connector = ConditionConnector::Equals;
            }
            if subject_only_then {
                connector = ConditionConnector::Then;
            }
            if attached_connector == connector || context_equality || subject_only_then {
                condition.push(base);
                body_start = at + 1;
            }
        }
        // Spoken Korean may separate both the subject particle and the short
        // ending: `준비 가 거짓 이면` or `이름 이 철수 면`. A multi-token
        // condition is an equality comparison; a single subject keeps the
        // truthy/then meaning.
        if token_matches_exact(token, &["이면", "이라면", "면", "먄"]) {
            connector = if condition.len() > 1 {
                ConditionConnector::Equals
            } else {
                ConditionConnector::Then
            };
        }
    }
    while tokens
        .get(body_start)
        .is_some_and(|token| matches!(token.tok, Tok::Rpar | Tok::Rsqb | Tok::Rbrace))
    {
        condition.push(tokens[body_start].clone());
        body_start += 1;
    }
    (condition, body_start, connector)
}

fn parse_natural_condition(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Condition, Diagnostic> {
    if tokens.is_empty() {
        return Err(condition_missing(spelling, Span::new(0, 0)));
    }
    // `<name> is ready` / `<이름> 쿨타임이 남았으면` is a whole condition on
    // its own; nothing inside it is a comparison to be taken apart.
    if let Some((condition, end)) = cooldown_condition_at(tokens, 0) {
        if end == tokens.len() {
            return Ok(condition);
        }
    }
    // Parentheses around a whole NME condition should not turn its logical
    // connectors into an opaque Python expression. Keep the token-based
    // logical grammar active while still allowing parentheses inside an
    // operand, such as `if (ready) and score > 2`.
    if let Some(inner) = strip_outer_condition_parentheses(tokens) {
        return parse_natural_condition(source, inner, connector, known_names, spelling);
    }
    // `or` has lower precedence than `and`, just like Python.  Splitting on
    // tokens (rather than source text) keeps strings and nested expressions
    // out of the easy-language grammar. A split that would produce an empty
    // operand (leading or trailing logical word) falls through to the atom
    // parser, which reports an exact diagnostic instead of panicking.
    if let Some(index) = logical_operator_at(tokens, LogicalOp::Or) {
        if index > 0 && index + 1 < tokens.len() {
            let left =
                parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
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
    }
    if let Some(index) = logical_operator_at(tokens, LogicalOp::And) {
        if index > 0 && index + 1 < tokens.len() {
            let left =
                parse_natural_condition(source, &tokens[..index], None, known_names, spelling)?;
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
    }
    parse_natural_condition_atom(source, tokens, connector, known_names, spelling)
}

fn condition_wrapper_end(tokens: &[Token], start: usize) -> Option<usize> {
    if !tokens
        .get(start)
        .is_some_and(|token| matches!(&token.tok, Tok::Lpar))
    {
        return None;
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate().skip(start) {
        match token.tok {
            Tok::Lpar => depth += 1,
            Tok::Rpar => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn strip_outer_condition_parentheses(tokens: &[Token]) -> Option<&[Token]> {
    if tokens.len() < 2
        || !tokens
            .first()
            .is_some_and(|token| matches!(&token.tok, Tok::Lpar))
    {
        return None;
    }
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match token.tok {
            Tok::Lpar => depth += 1,
            Tok::Rpar => {
                depth = depth.checked_sub(1)?;
                if depth == 0 && index + 1 != tokens.len() {
                    return None;
                }
            }
            _ => {}
        }
    }
    (depth == 0).then(|| &tokens[1..tokens.len() - 1])
}

fn logical_operator_at(tokens: &[Token], operator: LogicalOp) -> Option<usize> {
    let expected = match operator {
        LogicalOp::And => &["and", "그리고"][..],
        LogicalOp::Or => &["or", "또는"][..],
    };
    let mut depth = 0usize;
    let exact = tokens.iter().enumerate().find_map(|(index, token)| {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        // `or equal` is part of a `less/greater than or equal to`
        // comparison, not a logical `or`.
        (depth == 0
            && token_matches_exact(token, expected)
            && !(expected.contains(&"or") && is_or_equal_phrase_at(tokens, index)))
        .then_some(index)
    });
    if exact.is_some() {
        return exact;
    }

    // A single misspelled logical connector is easy to recover without
    // guessing across arbitrary expressions. Keep the same precedence and
    // bracket-depth rules as the exact path.
    depth = 0;
    tokens.iter().enumerate().find_map(|(index, token)| {
        match token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            _ => {}
        }
        (depth == 0
            && !(expected.contains(&"or") && is_or_equal_phrase_at(tokens, index))
            && word_matches_any(token, expected, MatchMode::Recover))
        .then_some(index)
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

    if looks_like_incomplete_english_condition(&cleaned) {
        let condition_span = Span::new(cleaned[0].span.start, cleaned[cleaned.len() - 1].span.end);
        return Err(condition_invalid(spelling, condition_span));
    }

    if connector.is_none()
        && cleaned.len() > 1
        && !cleaned
            .iter()
            .any(|token| token_matches_exact(token, &["and", "or", "그리고", "또는"]))
    {
        // A Korean comparison ending may live in a logical operand that has
        // no connector of its own: `점수가 0보다 크면 그리고 점수가 3보다
        // 작으면`. Discover the ending inside this operand and reparse it.
        let owned: Vec<Token> = cleaned.iter().map(|token| (*token).clone()).collect();
        if let Some((relative_at, found)) = find_condition_connector(&owned) {
            let (condition, body_start, found) =
                condition_tokens_before(&owned, 0, relative_at, found);
            if body_start == owned.len() && !condition.is_empty() {
                return parse_natural_condition(
                    source,
                    &condition,
                    Some(found),
                    known_names,
                    spelling,
                );
            }
        }
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
        Some(
            ConditionConnector::Greater
            | ConditionConnector::Less
            | ConditionConnector::GreaterOrEqual
            | ConditionConnector::LessOrEqual,
        ) => {
            let operator = match connector {
                Some(ConditionConnector::Greater) => CompareOp::Greater,
                Some(ConditionConnector::Less) => CompareOp::Less,
                Some(ConditionConnector::GreaterOrEqual) => CompareOp::GreaterOrEqual,
                _ => CompareOp::LessOrEqual,
            };
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                operator,
                &["보다", "더", "작을", "클", "작거나", "크거나"],
                spelling,
                false,
            );
        }
        Some(ConditionConnector::Equals | ConditionConnector::NotEquals) => {
            return parse_korean_comparison(
                source,
                &cleaned,
                known_names,
                CompareOp::Equal,
                &["과", "와", "랑", "이랑", "하고", "to"],
                spelling,
                matches!(connector, Some(ConditionConnector::NotEquals)),
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
    let tokens = if tokens.len() == 2 && token_matches_exact(tokens[1], &["은", "는", "이", "가"])
    {
        &tokens[..1]
    } else {
        tokens
    };
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
    negated: bool,
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
        negated,
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
    let operator = if condition_word_matches(
        predicate,
        &["greater", "above", "great", "larger", "bigger", "higher"],
    ) {
        CompareOp::Greater
    } else if condition_word_matches(predicate, &["less", "below", "small", "smaller", "lower"]) {
        CompareOp::Less
    } else if condition_word_matches(predicate, &["equals", "equal", "same"]) {
        CompareOp::Equal
    } else {
        return None;
    };
    cursor += 1;
    while tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
    {
        cursor += 1;
    }
    // `less than or equal to` / `greater than or equal to` narrow the
    // comparison to `<=` / `>=`.
    if tokens
        .get(cursor)
        .is_some_and(|token| token_word(token) == Some("or"))
        && tokens.get(cursor + 1).is_some_and(|token| {
            condition_word_matches(token_word(token).unwrap_or(""), &["equal", "equals"])
        })
    {
        let operator = match operator {
            CompareOp::Greater => CompareOp::GreaterOrEqual,
            CompareOp::Less => CompareOp::LessOrEqual,
            other => other,
        };
        cursor += 2;
        while tokens
            .get(cursor)
            .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
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
        return Some(Condition::Compare {
            left,
            operator,
            right,
            negated,
        });
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

/// A colon-free condition can make an incomplete comparison look like valid
/// Python (`score is greater` is a valid identity expression). Once the user
/// has clearly started one of NME's comparison words, keep the missing value
/// as a friendly NME diagnostic instead of silently emitting that expression.
fn looks_like_incomplete_english_condition(tokens: &[&Token]) -> bool {
    if tokens.len() < 2 {
        return false;
    }
    let mut cursor = 1;
    if tokens.get(cursor).and_then(|token| token_word(token)) == Some("is") {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["really"]))
    {
        cursor += 1;
    }
    if tokens
        .get(cursor)
        .is_some_and(|token| matches!(token.tok, Tok::Not) || token_word(token) == Some("not"))
    {
        cursor += 1;
    }
    let Some(predicate) = tokens.get(cursor).and_then(|token| token_word(token)) else {
        return false;
    };
    if !condition_word_matches(
        predicate,
        &[
            "greater", "above", "great", "larger", "bigger", "higher", "less", "below", "small",
            "smaller", "lower", "equals", "equal", "same",
        ],
    ) {
        return false;
    }
    cursor += 1;
    while tokens
        .get(cursor)
        .is_some_and(|token| token_matches_exact(token, &["to", "than", "as"]))
    {
        cursor += 1;
    }
    cursor >= tokens.len()
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
    if is_elapsed_name(name, known_names) {
        return elapsed_condition_value();
    }
    ConditionValue::Name(name.to_string())
}

/// `잰시간` and `elapsed` read the stopwatch wherever a condition may name a
/// value, so `만약 잰시간이 3보다 크면` compares seconds and not a name.
fn is_elapsed_name(name: &str, known_names: &HashSet<String>) -> bool {
    !known_names.contains(name)
        && (ELAPSED_WORDS_EN.contains(&name) || ELAPSED_WORDS_KO.contains(&name))
}

fn elapsed_condition_value() -> ConditionValue {
    ConditionValue::Python(Code::Generated(ELAPSED_PYTHON.to_string()))
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
            if is_elapsed_name(word, known_names) {
                return Some(elapsed_condition_value());
            }
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
    if let Some((_, connector)) = split_attached_condition_token(token) {
        return Some(connector);
    }
    let candidates = [
        (
            ConditionConnector::Then,
            &[
                "then",
                "그러면",
                "그럼",
                "하면",
                "이면",
                "이라면",
                "경우",
                "때",
                "일때",
            ][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (ConditionConnector::Missing, &["없으면", "없다면"][..]),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "라면", "면"][..],
        ),
        (
            ConditionConnector::NotEquals,
            &["같지않으면", "같지않다면", "같지않을"][..],
        ),
        (ConditionConnector::Greater, &["크면", "크다면", "클"][..]),
        (ConditionConnector::Less, &["작으면", "작다면", "작을"][..]),
        (
            ConditionConnector::GreaterOrEqual,
            &["크거나같으면", "크거나같다면"][..],
        ),
        (
            ConditionConnector::LessOrEqual,
            &["작거나같으면", "작거나같다면"][..],
        ),
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
    // Some Korean consonant substitutions are equally close to two
    // connectors under plain edit distance (`잇으면` is close to both
    // `있으면` and `없으면`). These common spoken spellings have a clear
    // intended meaning, so resolve them before collecting ambiguous fuzzy
    // candidates.
    match word {
        "잇으면" | "잇다면" => return Some(ConditionConnector::Exists),
        "업으면" | "업다면" => return Some(ConditionConnector::Missing),
        _ => {}
    }
    let candidates = [
        (
            ConditionConnector::Then,
            &[
                "then",
                "그러면",
                "그럼",
                "하면",
                "이면",
                "이라면",
                "경우",
                "때",
                "일때",
            ][..],
        ),
        (ConditionConnector::Exists, &["있으면", "있다면"][..]),
        (ConditionConnector::Missing, &["없으면", "없다면"][..]),
        (
            ConditionConnector::Equals,
            &["같으면", "같다면", "라면", "면"][..],
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
                        // `than` is a normal part of `greater than`/`less
                        // than`, not a misspelled `then` connector.  Keeping
                        // it out avoids making an otherwise clear condition
                        // ambiguous when the real connector is also mistyped.
                        && word != "than"
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
    // Korean learners often shorten `같으면`/`작으면`/`크면` to the spoken
    // `...먄` ending. It is a bounded connector-only repair, not a general
    // fuzzy match, and still goes through the unique-candidate check below.
    match word {
        "있으먄" | "있먄" => recovered.push(ConditionConnector::Exists),
        "없으먄" | "없먄" => recovered.push(ConditionConnector::Missing),
        "같먄" | "같으먄" | "라먄" | "먄" => recovered.push(ConditionConnector::Equals),
        "크먄" | "크으먄" => recovered.push(ConditionConnector::Greater),
        "작먄" | "작으먄" => recovered.push(ConditionConnector::Less),
        _ => {}
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
    Diagnostic::bilingual(
        DiagnosticCode::ConditionMissing,
        "the condition is missing",
        "조건이 비어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write `if ready` or `if score > 10` and indent the next line",
        "`만약에 준비됐으면`처럼 적고 다음 줄을 들여쓰세요",
    )
}

fn condition_invalid(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ConditionInvalid,
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

#[allow(clippy::too_many_lines)]
fn match_times(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some((count, body_start)) = attached_korean_times_sentence(source, tokens) {
        let mut body_start = body_start;
        if let Some((_, consumed)) = repeat_action_at(tokens, body_start, mode) {
            body_start += consumed;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
        }
        let inline = parse_sentence_repeat_body(
            source,
            &tokens[body_start..],
            block,
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
        let count_start = repeat_action_at(tokens, 0, mode).map_or(0, |(_, consumed)| consumed);
        let count = parse_count(source, &tokens[count_start..times_at], spelling)?;
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
        if marker_at > 0
            && marker_at + 1 < tokens.len()
            && repeat_action_at(tokens, 0, mode).is_none()
            && repeat_action_at(tokens, 0, MatchMode::Recover).is_none()
            && repeat_action_at(tokens, marker_at + 1, mode).is_none()
            && repeat_action_at(tokens, marker_at + 1, MatchMode::Recover).is_none()
        {
            let count = parse_count(source, &tokens[..marker_at], spelling)?;
            let mut body_start = marker_at + 1;
            if tokens.get(body_start).is_some_and(is_connector_word) {
                body_start += 1;
            }
            let inline = parse_sentence_repeat_body(
                source,
                &tokens[body_start..],
                block,
                span_of(&tokens[..body_start]),
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
            let inline = parse_sentence_repeat_body(
                source,
                &tokens[body_start..],
                block,
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
        let inline = parse_sentence_repeat_body(
            source,
            &tokens[body_start..],
            block,
            span_of(&tokens[..body_start]),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    Ok(None)
}

/// Parse a body that was introduced by the sentence repeat spelling. A plain
/// run of words is naturally a thing to say (`3번 안녕하세요`), while a
/// beginner/Python-shaped body still goes through the normal classifier.
fn parse_sentence_repeat_body(
    source: &str,
    body: &[Token],
    block: &BlockCtx<'_>,
    header_span: Span,
    known_names: &HashSet<String>,
) -> Result<Option<InlineStmt>, Diagnostic> {
    if branch_shape(body).is_some() {
        return Err(branch_without_condition_diagnostic(span_of(body)));
    }
    if let Some(inner) = match_break(source, body, known_names, MatchMode::Exact)? {
        return Ok(Some(InlineStmt::Nme(Box::new(inner))));
    }
    let plain_words = !body.is_empty()
        && body.iter().all(is_text_token)
        && !body.iter().any(|token| literal_token(token).is_some());
    let has_action = output_action_at(body, 0, MatchMode::Exact).is_some()
        || output_action_at(body, 0, MatchMode::Recover).is_some()
        || output_action_ending(body, MatchMode::Exact).is_some()
        || output_action_ending(body, MatchMode::Recover).is_some()
        || ask_action_at(body, 0, MatchMode::Exact).is_some()
        || ask_action_at(body, 0, MatchMode::Recover).is_some()
        || find_ask_shape(body, MatchMode::Exact).is_some()
        || find_ask_shape(body, MatchMode::Recover).is_some();
    if !body.is_empty() && (!plain_words || has_action) {
        if let Some(inner) = classify(source, body, &BlockCtx::Inline, known_names)? {
            if matches!(&inner, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. }) {
                return Err(branch_without_condition_diagnostic(span_of(body)));
            }
            return Ok(Some(InlineStmt::Nme(Box::new(inner))));
        }
    }
    if plain_words {
        let value = parse_value(source, body, known_names, true).map_err(|()| {
            Diagnostic::bilingual(
                DiagnosticCode::RepeatBodyUnparseable,
                "I couldn't understand what to repeat",
                "무엇을 반복할지 이해하지 못했어요",
                span_of(body),
            )
            .with_bilingual_hint(
                "write a sentence such as `3번 안녕하세요` or add `말해줘`",
                "`3번 안녕하세요`처럼 쓰거나 끝에 `말해줘`를 붙여 주세요",
            )
        })?;
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Say { value }))));
    }
    parse_suite_body(
        source,
        body,
        block,
        SuiteKind::Repeat,
        header_span,
        known_names,
    )
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
            DiagnosticCode::RepeatCountUnparseable,
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
        DiagnosticCode::RepeatCountMissing,
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

/// Sentence-level file and JSON forms, so a beginner can read a file into a
/// name and write text to a file without the `use file` module or Python
/// punctuation. Both languages share the same meaning:
///
/// - `read "notes.txt" into memo` / `memo read "notes.txt"`
/// - `memo에 "notes.txt" 읽어서` / `memo에 "notes.txt" 읽어서 저장해`
/// - `write "hello" to "out.txt"` / `"out.txt" 파일에 "hello"를 저장해`
///
/// The path is always a quoted string; the write value is a beginner value.
/// Weak matches (`read the book`, `write hello`) fall through to plain
/// sentence output instead of being claimed as file operations.
#[allow(clippy::too_many_lines)]
fn match_file_io(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let is_string = |token: &Token| matches!(token.tok, Tok::String { .. });
    let path_of = |tokens: &[Token]| -> Option<Code> {
        let token = tokens.first()?;
        if !is_string(token) {
            return None;
        }
        let span = token.span;
        is_valid_python_expression(&source[span.start..span.end]).then_some(Code::Source(span))
    };

    // English action-first read: `read "notes.txt" into memo`.
    if let Some(consumed) = action_phrase_at(tokens, 0, FILE_READ_WORDS_EN, mode) {
        let Some(path) = path_of(&tokens[consumed..]) else {
            return Ok(None);
        };
        let mut rest = &tokens[consumed + 1..];
        if rest
            .first()
            .is_some_and(|token| token_matches_exact(token, &["into", "as", "in"]))
        {
            rest = &rest[1..];
        }
        if let Some(target) = rest
            .first()
            .and_then(|t| name_word(t))
            .map(strip_saved_target)
        {
            if rest.len() == 1 || (rest.len() == 2 && is_command_ending(&rest[1])) {
                return Ok(Some(NmeStmt::FileRead {
                    target: target.to_string(),
                    path,
                }));
            }
        }
        return Err(file_read_target_diagnostic(span_of(tokens)));
    }

    // Korean read and English/Korean target-first read:
    // `memo에 "notes.txt" 읽어서` / `memo read "notes.txt"`. The path sits
    // before a Korean read word but after the English `read`.
    let ko_read_at = tokens.iter().position(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_READ_WORDS_KO, mode).is_some()
    });
    let en_read_at = tokens.iter().position(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_READ_WORDS_EN, mode).is_some()
    });
    if let Some(action_at) = ko_read_at.or(en_read_at) {
        let Some(target) = name_word(&tokens[0]).and_then(update_target_name) else {
            return Ok(None);
        };
        let path_tokens = if ko_read_at.is_some() {
            let mut middle = &tokens[1..action_at];
            if middle.first().is_some_and(|token| {
                is_update_connector(
                    token,
                    &["에", "에서", "에게", "한테", "는", "은", "으로", "로"],
                )
            }) {
                middle = &middle[1..];
            }
            middle
        } else {
            let mut after = &tokens[action_at + 1..];
            if after
                .first()
                .is_some_and(|token| token_matches_exact(token, &["into", "as", "in"]))
            {
                after = &after[1..];
            }
            after
        };
        let Some(path) = path_of(path_tokens) else {
            return Ok(None);
        };
        let tail_ok = if ko_read_at.is_some() {
            let after = &tokens[action_at + 1..];
            after.len() <= 2
                && after.iter().all(|token| {
                    token_matches_exact(token, FILE_WRITE_WORDS_KO) || is_command_ending(token)
                })
        } else {
            let after = &tokens[action_at + 1..];
            path_of(after).is_some()
                && (after.len() == 1 || (after.len() == 2 && is_command_ending(&after[1])))
        };
        if tail_ok {
            return Ok(Some(NmeStmt::FileRead { target, path }));
        }
        return Err(file_read_target_diagnostic(span_of(tokens)));
    }

    // English action-first write: `write "hello" to "out.txt"`.
    if let Some(consumed) = action_phrase_at(tokens, 0, FILE_WRITE_WORDS_EN, mode) {
        let mut end = tokens.len();
        if tokens
            .get(end.saturating_sub(1))
            .is_some_and(is_command_ending)
        {
            end -= 1;
        }
        let Some(to_at) = tokens[consumed..end]
            .iter()
            .position(|token| token_matches_exact(token, &["to", "into"]))
            .map(|at| at + consumed)
        else {
            return Ok(None);
        };
        let value = parse_value(source, &tokens[consumed..to_at], known_names, true)
            .map_err(|()| file_write_diagnostic(span_of(tokens)))?;
        let Some(path) = path_of(&tokens[to_at + 1..end]) else {
            return Err(file_path_diagnostic(span_of(tokens)));
        };
        return Ok(Some(NmeStmt::FileWrite { path, value }));
    }

    // Korean write: `"out.txt" 파일에 "hello"를 저장해`.
    let write_at = tokens.iter().rposition(|token| {
        action_phrase_at(std::slice::from_ref(token), 0, FILE_WRITE_WORDS_KO, mode).is_some()
    });
    if let Some(write_at) = write_at {
        let Some(path) = path_of(&tokens[..1]) else {
            return Ok(None);
        };
        if tokens.get(1).is_some_and(|token| {
            is_update_connector(token, &["파일에", "파일을", "에", "로", "으로", "에다"])
        }) {
            let mut value_tokens = &tokens[2..write_at];
            while value_tokens
                .last()
                .is_some_and(|token| is_update_connector(token, &["을", "를", "만큼"]))
            {
                value_tokens = &value_tokens[..value_tokens.len() - 1];
            }
            if value_tokens.is_empty() {
                return Err(file_write_diagnostic(span_of(tokens)));
            }
            // A Korean particle may be glued to the final word (`점수를`);
            // strip it from the source span so the value stays a Python name.
            let value = if let Some(last) = value_tokens.last() {
                if let Some(word) = name_word(last) {
                    if let Some(stripped) = strip_any_suffix(word, &["을", "를", "만큼"]) {
                        let base = last.span;
                        let end = base.end - (word.len() - stripped.len());
                        Value::Python(Code::Source(Span::new(base.start, end)))
                    } else {
                        parse_value(source, value_tokens, known_names, true)
                            .map_err(|()| file_write_diagnostic(span_of(tokens)))?
                    }
                } else {
                    parse_value(source, value_tokens, known_names, true)
                        .map_err(|()| file_write_diagnostic(span_of(tokens)))?
                }
            } else {
                return Err(file_write_diagnostic(span_of(tokens)));
            };
            return Ok(Some(NmeStmt::FileWrite { path, value }));
        }
        return Err(file_write_diagnostic(span_of(tokens)));
    }

    Ok(None)
}

fn file_read_target_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "the file read needs a target name",
        "파일을 읽어 넣을 이름이 필요해요",
        span,
    )
    .with_bilingual_hint(
        "write `read \"notes.txt\" into memo` or `memo에 \"notes.txt\" 읽어서`",
        "`read \"notes.txt\" into memo` 또는 `memo에 \"notes.txt\" 읽어서`처럼 쓰세요",
    )
}

fn file_write_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::SaveValueUnparseable,
        "I couldn't understand this file write",
        "파일에 저장할 내용을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `write \"hello\" to \"out.txt\"` or `\"out.txt\" 파일에 \"hello\"를 저장해`",
        "`write \"hello\" to \"out.txt\"` 또는 `\"out.txt\" 파일에 \"hello\"를 저장해`처럼 쓰세요",
    )
}

fn file_path_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "the file name must be a quoted path",
        "파일 이름은 따옴표로 감싼 경로여야 해요",
        span,
    )
    .with_bilingual_hint(
        "write the path in quotes, for example `\"notes.txt\"`",
        "`\"notes.txt\"`처럼 경로를 따옴표 안에 적어 주세요",
    )
}

/// `from "helper.nme" import greet, score` — a beginner module import. The
/// quoted path is not valid Python (`from <string>` is a syntax error), so
/// NME can claim it. The explicit name list is the module interface: only
/// those names cross the file boundary.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn match_module_import(
    source: &str,
    tokens: &[Token],
    _known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if !matches!(tokens.first().map(|token| &token.tok), Some(Tok::From))
        || !matches!(
            tokens.get(1).map(|token| &token.tok),
            Some(Tok::String { .. })
        )
        || !matches!(tokens.get(2).map(|token| &token.tok), Some(Tok::Import))
        || mode != MatchMode::Exact
    {
        return Ok(None);
    }
    let path_span = tokens[1].span;
    let path_text = &source[path_span.start..path_span.end];
    let path_stripped = path_text.trim_matches(['\'', '"']);
    if !path_stripped.ends_with(".nme") {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::MissingAction,
            "a module import path must end in .nme",
            "모듈 경로는 .nme로 끝나야 해요",
            path_span,
        )
        .with_bilingual_hint(
            "write the other program's name in quotes, e.g. `from \"helper.nme\" import greet`",
            "다른 프로그램의 이름을 따옴표로 적으세요. 예: `from \"helper.nme\" import greet`",
        ));
    }
    let stem = path_stripped
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(path_stripped)
        .strip_suffix(".nme")
        .unwrap_or(path_stripped);
    let valid_identifier = !stem.is_empty()
        && stem.chars().enumerate().all(|(index, character)| {
            character == '_'
                || character.is_alphanumeric()
                    && (index > 0 || character.is_alphabetic() || character == '_')
        });
    if !valid_identifier {
        return Err(Diagnostic::bilingual(
            DiagnosticCode::MissingAction,
            "the module file name must be a Python identifier",
            "모듈 파일 이름은 Python 식별자여야 해요",
            path_span,
        )
        .with_bilingual_hint(
            "rename the module with letters, numbers, and underscores only, e.g. `shape_math.nme`",
            "모듈 이름은 문자·숫자·밑줄만 사용하세요. 예: `shape_math.nme`",
        ));
    }
    let mut names = Vec::new();
    let mut index = 3;
    let mut expected = true;
    while index < tokens.len() {
        match &tokens[index].tok {
            Tok::Comma => {
                if expected {
                    return Err(module_import_shape_diagnostic(span_of(tokens)));
                }
                expected = true;
            }
            Tok::Name { name } if expected => {
                names.push(name.clone());
                expected = false;
            }
            _ => return Err(module_import_shape_diagnostic(span_of(tokens))),
        }
        index += 1;
    }
    if expected || names.is_empty() {
        return Err(module_import_shape_diagnostic(span_of(tokens)));
    }
    Ok(Some(NmeStmt::ModuleImport {
        path: Code::Source(path_span),
        names,
    }))
}

fn module_import_shape_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::MissingAction,
        "I couldn't understand this module import",
        "모듈 가져오기 문장을 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `from \"helper.nme\" import greet` with simple names after `import`",
        "`from \"helper.nme\" import greet`처럼 import 뒤에 간단한 이름을 적으세요",
    )
}

#[allow(clippy::too_many_lines)]
fn match_use_module(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((action_start, action_end, spelling)) = find_use_action(tokens, mode) else {
        return Ok(None);
    };

    let mut module = None;
    for candidate in BundledModuleId::ALL {
        let positions = tokens
            .iter()
            .enumerate()
            .filter_map(|(index, token)| {
                module_word_matches(token, candidate, mode).then_some(index)
            })
            .collect::<Vec<_>>();
        match positions.as_slice() {
            [] => {}
            [single] => {
                if module.is_some() {
                    return Err(unsupported_module_diagnostic(span_of(tokens)));
                }
                module = Some((candidate, *single));
            }
            _ => return Err(unsupported_module_diagnostic(span_of(tokens))),
        }
    }
    let Some((module, module_at)) = module else {
        return Err(unsupported_module_diagnostic(span_of(tokens)));
    };

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
            DiagnosticCode::LatestAndVersion,
            "choose either latest or one exact module version",
            "최신 버전과 특정 버전 중 하나만 골라 주세요",
            span_of(tokens),
        )
        .with_bilingual_hint(
            format!(
                "write `use {} latest` or `use {} version {}`",
                module.name_en(),
                module.name_en(),
                module.version()
            ),
            format!(
                "`{} 사용 최신` 또는 `{} 사용 버전 {}`처럼 쓰세요",
                module.name_ko(),
                module.name_ko(),
                module.version()
            ),
        ));
    }
    if latest_positions.len() > 1 || version_positions.len() > 1 {
        return Err(module_shape_diagnostic(spelling, span_of(tokens)));
    }

    let mut used = vec![false; tokens.len()];
    for slot in &mut used[action_start..action_end] {
        *slot = true;
    }
    used[module_at] = true;
    for &index in &latest_positions {
        used[index] = true;
    }

    let requested = if !latest_positions.is_empty() {
        ModuleVersion::Latest
    } else if let Some(&version_at) = version_positions.first() {
        if version_at < action_end.max(module_at + 1) {
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
                DiagnosticCode::ModuleVersionMissing,
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {}", module.version()),
                format!("`최신` 또는 버전 {}을 사용하세요", module.version()),
            )
        })?;
        if value_tokens.is_empty() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::ModuleVersionMissing,
                "the module version is missing",
                "모듈 버전이 비어 있어요",
                tokens[version_at].span,
            )
            .with_bilingual_hint(
                format!("use `latest`, or version {}", module.version()),
                format!("`최신` 또는 버전 {}을 사용하세요", module.version()),
            ));
        }
        for slot in &mut used[version_at + 1..value_end] {
            *slot = true;
        }
        let value_span = span_of(value_tokens);
        let raw = &source[value_span.start..value_span.end];
        let version = raw.trim_matches(['\'', '"']).to_string();
        if version != module.version() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::UnbundledVersion,
                format!("{} version {version} is not bundled", module.name_en()),
                format!("{} 버전 {version}은 내장되어 있지 않아요", module.name_ko()),
                value_span,
            )
            .with_bilingual_hint(
                format!("use `latest`; this compiler bundles {}", module.version()),
                format!(
                    "`최신`을 사용하세요. 이 컴파일러에는 {}이 들어 있어요",
                    module.version()
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

    let collisions = module_binding_names(module)
        .iter()
        .filter(|name| known_names.contains(**name))
        .copied()
        .collect::<Vec<_>>();
    if !collisions.is_empty() {
        return Err(module_name_collision_diagnostic(
            module,
            span_of(tokens),
            &collisions,
        ));
    }

    Ok(Some(NmeStmt::UseModule { module, requested }))
}

/// Names a bundled module would bind, so a later `use` can refuse to
/// overwrite an existing value.
fn module_binding_names(module: BundledModuleId) -> &'static [&'static str] {
    match module {
        BundledModuleId::Random => &[
            RANDOM_MODULE,
            RANDOM_MODULE_KO,
            "random_number",
            "random_pick",
            "shuffle",
            "랜덤정수",
            "랜덤선택",
            "섞기",
            "random_version",
            "랜덤버전",
        ],
        BundledModuleId::File => &[
            FILE_MODULE,
            FILE_MODULE_KO,
            "file_read",
            "file_write",
            "json_load",
            "json_save",
            "파일읽기",
            "파일쓰기",
            "json읽기",
            "json저장",
            "file_version",
            "파일버전",
        ],
        BundledModuleId::ZeroKnowledge => &[
            "영지식비밀난수",
            "zk_prime",
            "영지식큰소수",
            "zk_order",
            "영지식부분군크기",
            "zk_generator",
            "영지식생성원",
            "zk_challenge_bits",
            "영지식도전비트",
            "zk_challenge_limit",
            "영지식도전범위",
            "zk_secret",
            "영지식비밀만들기",
            "zk_public",
            "영지식공개값",
            "zk_nonce",
            "영지식일회값만들기",
            "zk_commitment",
            "영지식약속",
            "zk_challenge",
            "영지식도전만들기",
            "zk_challenge_except",
            "영지식다른도전",
            "zk_response",
            "영지식응답",
            "zk_verify",
            "영지식검증",
            "zk_simulated_response",
            "영지식모의응답만들기",
            "zk_simulated_commitment",
            "영지식모의약속",
            "zk_group_bytes",
            "영지식그룹바이트",
            "_nme_zk_context_bytes",
            "_nme_zk_int_bytes",
            "_nme_zk_context_frame",
            "zk_nizk_challenge",
            "영지식비대화도전",
            "zk_nizk_prove",
            "영지식비대화증명",
            "zk_nizk_verify",
            "영지식비대화검증",
            "zero_knowledge_version",
            "영지식버전",
        ],
    }
}

fn module_name_collision_diagnostic(
    module: BundledModuleId,
    span: Span,
    collisions: &[&str],
) -> Diagnostic {
    let names = collisions.join(", ");
    Diagnostic::bilingual(
        DiagnosticCode::ModuleNameCollision,
        format!(
            "the {} module would overwrite existing name(s): {names}",
            module.name_en()
        ),
        format!(
            "{} 모듈이 이미 있는 이름을 덮어쓸 수 있어요: {names}",
            module.name_ko()
        ),
        span,
    )
    .with_bilingual_hint(
        "rename the existing value, or load the module before assigning that name",
        "기존 값을 다른 이름으로 바꾸거나, 그 이름을 쓰기 전에 모듈을 불러오세요",
    )
}

fn module_word_matches(token: &Token, module: BundledModuleId, mode: MatchMode) -> bool {
    name_word(token).is_some_and(|word| {
        word_matches(word, module.name_en(), mode)
            || (module == BundledModuleId::ZeroKnowledge
                && word_matches(word, "zeroknowledge", mode))
            || word == module.name_ko()
            || strip_target_particle(word) == module.name_ko()
    })
}

fn unsupported_module_diagnostic(span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::UnsupportedModule,
        "NME bundles `use random`, `use file`, and `use zero_knowledge`",
        "NME에는 쉬운 `랜덤`, `파일`, `영지식` 모듈이 들어 있어요",
        span,
    )
    .with_bilingual_hint(
        "write one module line such as `use random latest`, `use file latest`, or `use zero_knowledge latest`",
        "`랜덤 사용 최신`, `파일 사용 최신`, `영지식 사용 최신` 중 하나를 적어 주세요",
    )
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

fn recoverable_module_shape(tokens: &[Token]) -> bool {
    let action_recovered = find_use_action(tokens, MatchMode::Exact).is_none()
        && find_use_action(tokens, MatchMode::Recover).is_some();
    let module_exact = tokens
        .iter()
        .filter(|token| {
            BundledModuleId::ALL
                .iter()
                .any(|module| module_word_matches(token, *module, MatchMode::Exact))
        })
        .count();
    let module_recovered = tokens
        .iter()
        .filter(|token| {
            BundledModuleId::ALL
                .iter()
                .any(|module| module_word_matches(token, *module, MatchMode::Recover))
        })
        .count();
    let exact_latest = tokens
        .iter()
        .filter(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Exact))
        .count();
    let recovered_latest = tokens
        .iter()
        .filter(|token| word_matches_any(token, LATEST_WORDS, MatchMode::Recover))
        .count();
    let exact_version = tokens
        .iter()
        .filter(|token| word_matches_any(token, &["version", "버전"], MatchMode::Exact))
        .count();
    let recovered_version = tokens
        .iter()
        .filter(|token| word_matches_any(token, &["version", "버전"], MatchMode::Recover))
        .count();

    action_recovered
        || (module_exact == 0 && module_recovered == 1)
        || (exact_latest == 0 && recovered_latest == 1)
        || (exact_version == 0 && recovered_version == 1)
}

fn module_shape_diagnostic(_spelling: Spelling, span: Span) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::ModuleShapeInvalid,
        "I couldn't understand this module line",
        "이 모듈 문장을 확실하게 이해하지 못했어요",
        span,
    )
    .with_bilingual_hint(
        "write `use random latest`, `use file latest`, or `use zero_knowledge latest`, with an optional version",
        "`랜덤 사용 최신`, `파일 사용 최신`, `영지식 사용 최신`처럼 쓰고, 원하면 버전을 붙이세요",
    )
}

// ------------------------------------------------------------ assignment

#[allow(clippy::too_many_lines)]
fn match_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
    mode: MatchMode,
) -> Result<Option<NmeStmt>, Diagnostic> {
    // A spoken target-first form is often the first bridge from plain
    // sentences to assignments: `이름 저장 민수` / `name save Mina`.  Keep it
    // deliberately strict (the save word must be the second token) so normal
    // prose is not silently turned into a variable assignment.
    if tokens.len() >= 2
        && name_word(&tokens[0]).is_some()
        && set_action_at(tokens, 1, mode).is_some()
    {
        let target_token = &tokens[0];
        let target = strip_saved_target(name_word(target_token).expect("checked name token"));
        let Some((_, consumed)) = set_action_at(tokens, 1, mode) else {
            unreachable!("set action was checked above");
        };
        let mut value_start = 1 + consumed;
        if tokens.get(value_start).is_some_and(|token| {
            token_matches_exact(
                token,
                &["to", "as", "is", "로", "으로", "을", "를", "은", "는", "에"],
            )
        }) {
            value_start += 1;
        }
        if value_start >= tokens.len() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveValueMissing,
                "the value to save is missing",
                "저장할 값이 비어 있어요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `name save Mina` or `이름 저장 민수`",
                "`이름 저장 민수` 또는 `name save Mina`처럼 값을 뒤에 적어 주세요",
            ));
        }
        let value =
            parse_value(source, &tokens[value_start..], known_names, true).map_err(|()| {
                Diagnostic::bilingual(
                    DiagnosticCode::SaveValueUnparseable,
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

    if let Some(first) = name_word(&tokens[0]) {
        if let Some(target) = strip_assignment_particle(first) {
            if tokens.len() == 1 {
                return Err(Diagnostic::bilingual(
                    DiagnosticCode::SaveValueMissing,
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
                    DiagnosticCode::SaveValueUnparseable,
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
                DiagnosticCode::SaveValueUnparseable,
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

    if let Some((spelling, consumed)) = set_action_at(tokens, 0, mode) {
        let Some(target_token) = tokens.get(consumed) else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveNameMissing,
                "the name to save is missing",
                "값을 저장할 이름이 비어 있어요",
                tokens[0].span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        };
        let Some(target_word) = name_word(target_token) else {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveNameNotSimple,
                "use a simple name here",
                "여기에는 간단한 이름을 써 주세요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        };
        let target = if spelling == Spelling::Korean {
            strip_saved_target(target_word)
        } else {
            target_word
        };
        let mut value_start = consumed + 1;
        if tokens.get(value_start).is_some_and(|token| {
            token_matches_exact(
                token,
                &["to", "as", "is", "로", "으로", "을", "를", "은", "는", "에"],
            )
        }) {
            value_start += 1;
        }
        if value_start >= tokens.len() {
            return Err(Diagnostic::bilingual(
                DiagnosticCode::SaveValueMissing,
                "the value to save is missing",
                "저장할 값이 비어 있어요",
                target_token.span,
            )
            .with_bilingual_hint(
                "write `set greeting to Hello` or `저장 인사 Hello`",
                "`저장 인사 안녕하세요` 또는 `set greeting to Hello`처럼 쓰세요",
            ));
        }
        let value =
            parse_value(source, &tokens[value_start..], known_names, true).map_err(|()| {
                Diagnostic::bilingual(
                    DiagnosticCode::SaveValueUnparseable,
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

fn set_action_at(tokens: &[Token], start: usize, mode: MatchMode) -> Option<(Spelling, usize)> {
    action_phrase_at(tokens, start, SET_WORDS_EN, mode)
        .map(|consumed| (Spelling::English, consumed))
        .or_else(|| {
            action_phrase_at(tokens, start, SET_WORDS_KO, mode)
                .map(|consumed| (Spelling::Korean, consumed))
        })
}

fn strip_saved_target(word: &str) -> &str {
    strip_assignment_particle(word).unwrap_or_else(|| {
        [
            "에게", "한테", "에서", "으로", "로", "을", "를", "에", "은", "는",
        ]
        .iter()
        .find_map(|particle| word.strip_suffix(particle).filter(|base| !base.is_empty()))
        .unwrap_or(word)
    })
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
    if let Some(value) = parse_elapsed_value(tokens, known_names) {
        return Ok(value);
    }
    if let Some(value) = parse_zero_knowledge_value(tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_random_integer(source, tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_random_choice(source, tokens) {
        return Ok(value);
    }
    if let Some(value) = parse_list_value(source, tokens, known_names) {
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

/// `list of Mina, Ada` / `목록 민수, 지안`.
///
/// The marker word is required. Without it a comma-separated sentence stays
/// ordinary text, which is what a learner writing `Mina, Ada and Grace` means.
fn parse_list_value(source: &str, tokens: &[Token], known_names: &HashSet<String>) -> Option<Value> {
    let mut start = 0;
    if token_matches_exact(tokens.first()?, LIST_WORDS_EN) {
        start = 1;
        if tokens
            .get(1)
            .is_some_and(|token| token_matches_exact(token, &["of"]))
        {
            start = 2;
        }
    } else if token_matches_exact(tokens.first()?, LIST_WORDS_KO) {
        start = 1;
    }
    if start == 0 {
        return None;
    }
    let items = &tokens[start..];
    if items.is_empty() {
        return Some(Value::List(Vec::new()));
    }
    let mut values = Vec::new();
    for part in split_list_items(items) {
        if part.is_empty() {
            continue;
        }
        values.push(parse_value(source, &part, known_names, true).ok()?);
    }
    Some(Value::List(values))
}

/// Words people put between list items, standing alone or attached to the
/// word before them as a Korean particle (`민수와 지안`).
const LIST_JOINERS: &[&str] = &["and", "그리고", "와", "과", "이랑", "랑"];

/// Cuts a list into its items. A Korean joining particle is part of the word
/// it follows, so the particle is trimmed off and the item ends there.
fn split_list_items(tokens: &[Token]) -> Vec<Vec<Token>> {
    let mut items = Vec::new();
    let mut current: Vec<Token> = Vec::new();
    // `사과` ends in `과`, which is also the Korean word for `and`. Once the
    // writer has shown their separator by using a comma, trust it and stop
    // reading a joiner out of the end of a word.
    let comma_separated = tokens.iter().any(|token| matches!(token.tok, Tok::Comma));
    for token in tokens {
        if matches!(token.tok, Tok::Comma | Tok::And) || token_matches_exact(token, LIST_JOINERS) {
            items.push(std::mem::take(&mut current));
            continue;
        }
        if let (false, Tok::Name { name }) = (comma_separated, &token.tok) {
            if let Some(base) = LIST_JOINERS
                .iter()
                .filter(|joiner| joiner.chars().next().is_some_and(|c| !c.is_ascii()))
                .find_map(|joiner| {
                    name.strip_suffix(joiner).filter(|base| !base.is_empty())
                })
            {
                current.push(Token {
                    tok: Tok::Name {
                        name: base.to_string(),
                    },
                    span: Span::new(token.span.start, token.span.start + base.len()),
                });
                items.push(std::mem::take(&mut current));
                continue;
            }
        }
        if is_command_ending(token) {
            continue;
        }
        current.push(token.clone());
    }
    items.push(current);
    items
}

fn parse_zero_knowledge_value(tokens: &[Token]) -> Option<Value> {
    use crate::syntax::ZeroKnowledgeValue as Zk;

    // English sentence spellings mirror the Korean zero-knowledge value
    // grammar without requiring underscores, calls, commas, or parentheses.
    // `zeroknowledge` is reserved only for the module line; value phrases use
    // ordinary words so a complete sentence source can stay letters-only.
    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["challenge"])
        && token_matches_exact(&tokens[6], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkChallenge {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            commitment: zero_knowledge_code_plain(&tokens[1])?,
            context: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[2], &["zero"])
        && token_matches_exact(&tokens[3], &["knowledge"])
        && token_matches_exact(&tokens[4], &["proof"])
        && token_matches_exact(&tokens[5], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkProof {
            secret: zero_knowledge_code_plain(&tokens[0])?,
            context: zero_knowledge_code_plain(&tokens[1])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["zero"])
        && token_matches_exact(&tokens[4], &["knowledge"])
        && token_matches_exact(&tokens[5], &["verify"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkVerify {
            public_key: zero_knowledge_code_plain(&tokens[0])?,
            proof: zero_knowledge_code_plain(&tokens[1])?,
            context: zero_knowledge_code_plain(&tokens[2])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["secret"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Secret));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["zero"])
        && token_matches_exact(&tokens[2], &["knowledge"])
        && token_matches_exact(&tokens[3], &["public"])
        && token_matches_exact(&tokens[4], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Public {
            secret: zero_knowledge_code_plain(&tokens[0])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["nonce"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Nonce));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["zero"])
        && token_matches_exact(&tokens[2], &["knowledge"])
        && token_matches_exact(&tokens[3], &["commitment"])
        && token_matches_exact(&tokens[4], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Commitment {
            nonce: zero_knowledge_code_plain(&tokens[0])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["zero"])
        && token_matches_exact(&tokens[1], &["knowledge"])
        && token_matches_exact(&tokens[2], &["challenge"])
        && token_matches_exact(&tokens[3], &["make"])
    {
        return Some(Value::ZeroKnowledge(Zk::Challenge));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["도전"])
        && token_matches_exact(&tokens[6], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkChallenge {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            commitment: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[2], &["영지식"])
        && token_matches_exact(&tokens[3], &["비대화"])
        && token_matches_exact(&tokens[4], &["증명"])
        && token_matches_exact(&tokens[5], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkProof {
            secret: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[1], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["검증"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkVerify {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            proof: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["비밀"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Secret));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[1], &["영지식"])
        && token_matches_exact(&tokens[2], &["공개값"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Public {
            secret: zero_knowledge_code_with_particle(&tokens[0], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["일회값"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Nonce));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[1], &["영지식"])
        && token_matches_exact(&tokens[2], &["약속"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Commitment {
            nonce: zero_knowledge_code_with_particle(&tokens[0], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 3
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["도전"])
        && token_matches_exact(&tokens[2], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Challenge));
    }

    if tokens.len() == 5
        && token_matches_exact(&tokens[1], &["다른"])
        && token_matches_exact(&tokens[2], &["영지식"])
        && token_matches_exact(&tokens[3], &["도전"])
        && token_matches_exact(&tokens[4], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::ChallengeExcept {
            excluded: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["응답"])
        && token_matches_exact(&tokens[5], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::Response {
            nonce: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            secret: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[4], &["영지식"])
        && token_matches_exact(&tokens[5], &["검증"])
    {
        return Some(Value::ZeroKnowledge(Zk::Verify {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            commitment: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[2], &["과", "와"])?,
            response: zero_knowledge_code_with_particle(&tokens[3], &["으로", "로"])?,
        }));
    }

    if tokens.len() == 4
        && token_matches_exact(&tokens[0], &["영지식"])
        && token_matches_exact(&tokens[1], &["모의"])
        && token_matches_exact(&tokens[2], &["응답"])
        && token_matches_exact(&tokens[3], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedResponse));
    }

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["모의"])
        && token_matches_exact(&tokens[5], &["약속"])
        && token_matches_exact(&tokens[6], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::SimulatedCommitment {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"])?,
            challenge: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"])?,
            response: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"])?,
        }));
    }

    None
}

fn zero_knowledge_code_plain(token: &Token) -> Option<Code> {
    name_word(token)?;
    Some(Code::Source(token.span))
}

fn zero_knowledge_code_with_particle(token: &Token, particles: &[&str]) -> Option<Code> {
    let word = name_word(token)?;
    let stripped = particles
        .iter()
        .find_map(|particle| word.strip_suffix(particle).filter(|base| !base.is_empty()))?;
    let removed = word.len() - stripped.len();
    Some(Code::Source(Span::new(
        token.span.start,
        token.span.end - removed,
    )))
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
    if branch_shape(body).is_some() {
        return Err(branch_without_condition_diagnostic(body_span));
    }
    // Korean `멈춰` is a valid Python identifier, so the Python-wins check in
    // `classify` intentionally leaves a bare top-level name alone. Inside an
    // already recognized NME suite, however, the documented Korean break
    // spelling is unambiguous and must lower to `break` rather than leaking
    // the identifier into generated Python.
    if is_korean_break_alias(body) {
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Break))));
    }
    if is_skip_alias(body) {
        return Ok(Some(InlineStmt::Nme(Box::new(NmeStmt::Continue))));
    }
    if let Some(inner) = classify(source, body, &BlockCtx::Inline, known_names)? {
        if matches!(&inner, NmeStmt::ElseIf { .. } | NmeStmt::Else { .. }) {
            return Err(branch_without_condition_diagnostic(body_span));
        }
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
            DiagnosticCode::IndentationRequired,
            "the lines that should repeat must be indented",
            "반복할 다음 줄은 들여써야 해요",
            span,
        )
        .with_bilingual_hint(
            "or keep it on one line: `repeat 3 times and show Hello`",
            "한 줄로 `3번 반복해서 안녕 말해줘`라고 써도 돼요",
        ),
        SuiteKind::Condition => Diagnostic::bilingual(
            DiagnosticCode::ColonRequired,
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
        DiagnosticCode::BlockWithoutStatement,
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
        DiagnosticCode::OneStatementPerLine,
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
        DiagnosticCode::BodyUnparseable,
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

#[derive(Clone, Copy)]
enum BindingScopeKind {
    Root,
    Function,
    AsyncFunction,
    Class,
    Other,
}

struct BindingScope {
    body_indent: usize,
    names: HashSet<String>,
    kind: BindingScopeKind,
}

struct AsyncFunctionContext {
    body_scope_depth: usize,
    has_yield: bool,
    return_value_spans: Vec<Span>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PythonDeclarationKind {
    Global,
    Nonlocal,
}

struct PythonDeclaration {
    kind: PythonDeclarationKind,
    names: Vec<(String, usize)>,
}

struct PythonDeclarationContext {
    body_scope_depth: usize,
    seen_names: HashSet<String>,
    annotation_targets: HashSet<String>,
    declarations: HashMap<String, PythonDeclarationKind>,
}

struct PendingScope {
    header_indent: usize,
    names: HashSet<String>,
    kind: BindingScopeKind,
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
                kind: BindingScopeKind::Root,
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
                    kind: pending.kind,
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

    fn push_explicit_scope(&mut self, body_indent: usize) {
        self.scopes.push(BindingScope {
            body_indent,
            names: HashSet::new(),
            kind: BindingScopeKind::Other,
        });
    }

    fn inside_function(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction => return true,
                BindingScopeKind::Class => return false,
                BindingScopeKind::Root | BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn inside_async_function(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::AsyncFunction => return true,
                BindingScopeKind::Function | BindingScopeKind::Class => return false,
                BindingScopeKind::Root | BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn inside_non_module_scope(&self) -> bool {
        for scope in self.scopes.iter().rev() {
            match scope.kind {
                BindingScopeKind::Root => return false,
                BindingScopeKind::Function
                | BindingScopeKind::AsyncFunction
                | BindingScopeKind::Class => return true,
                BindingScopeKind::Other => {}
            }
        }
        false
    }

    fn python_scope_depth(&self) -> usize {
        self.scopes
            .iter()
            .filter(|scope| {
                matches!(
                    scope.kind,
                    BindingScopeKind::Function
                        | BindingScopeKind::AsyncFunction
                        | BindingScopeKind::Class
                )
            })
            .count()
    }

    fn has_enclosing_function(&self) -> bool {
        let Some((current_index, current_scope)) = self
            .scopes
            .iter()
            .enumerate()
            .rev()
            .find(|(_, scope)| !matches!(scope.kind, BindingScopeKind::Other))
        else {
            return false;
        };
        if !matches!(
            current_scope.kind,
            BindingScopeKind::Function | BindingScopeKind::AsyncFunction | BindingScopeKind::Class
        ) {
            return false;
        }
        self.scopes[..current_index].iter().any(|scope| {
            matches!(
                scope.kind,
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction
            )
        })
    }

    fn has_function_scope(&self) -> bool {
        self.scopes.iter().any(|scope| {
            matches!(
                scope.kind,
                BindingScopeKind::Function | BindingScopeKind::AsyncFunction
            )
        })
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
            if python_inline_suite_body(tokens).is_none() {
                self.pending = Some(PendingScope {
                    header_indent: indent,
                    names: parameters,
                    kind: if is_python_async_function_header(tokens) {
                        BindingScopeKind::AsyncFunction
                    } else if is_python_function_header(tokens) {
                        BindingScopeKind::Function
                    } else if is_python_class_header(tokens) {
                        BindingScopeKind::Class
                    } else {
                        BindingScopeKind::Other
                    },
                });
            }
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

fn python_inline_suite_body(tokens: &[Token]) -> Option<&[Token]> {
    python_scope_header(tokens)?;
    let depths = token_depths(tokens);
    let colon_index = tokens.iter().enumerate().find_map(|(index, token)| {
        (depths[index] == 0 && matches!(token.tok, Tok::Colon)).then_some(index)
    })?;
    let body = tokens.get(colon_index + 1..)?;
    (!body.is_empty()).then_some(body)
}

fn python_inline_function_body(tokens: &[Token]) -> Option<&[Token]> {
    is_python_function_header(tokens).then(|| python_inline_suite_body(tokens))?
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

    remember_import_bindings(tokens, names);
}

fn remember_import_bindings(tokens: &[Token], names: &mut HashSet<String>) {
    let import_at = if matches!(tokens.first().map(|token| &token.tok), Some(Tok::Import)) {
        Some(0)
    } else if matches!(tokens.first().map(|token| &token.tok), Some(Tok::From)) {
        tokens
            .iter()
            .position(|token| matches!(token.tok, Tok::Import))
    } else {
        None
    };
    let Some(import_at) = import_at else {
        return;
    };

    let mut index = import_at + 1;
    while index < tokens.len() {
        if matches!(tokens[index].tok, Tok::Comma | Tok::Lpar | Tok::Rpar) {
            index += 1;
            continue;
        }
        let Some(name) = name_word(&tokens[index]) else {
            index += 1;
            continue;
        };
        let default_name = name.to_string();
        index += 1;
        while index + 1 < tokens.len()
            && matches!(tokens[index].tok, Tok::Dot)
            && name_word(&tokens[index + 1]).is_some()
        {
            index += 2;
        }
        let binding = if tokens
            .get(index)
            .is_some_and(|token| token_matches_exact(token, &["as"]))
        {
            index += 1;
            let alias = tokens.get(index).and_then(name_word).map(str::to_string);
            index += usize::from(alias.is_some());
            alias
        } else {
            Some(default_name)
        };
        if let Some(binding) = binding {
            names.insert(binding);
        }
        while index < tokens.len() && !matches!(tokens[index].tok, Tok::Comma) {
            index += 1;
        }
    }
}

fn remember_bindings(stmt: &NmeStmt, names: &mut HashSet<String>) {
    match stmt {
        NmeStmt::Ask { target, .. } | NmeStmt::Set { target, .. } => {
            names.insert(target.clone());
        }
        NmeStmt::FileRead { target, .. } => {
            names.insert(target.clone());
        }
        // The stopwatch and each cooldown bind one Python name apiece. They
        // are remembered like any other name so the parser can tell a
        // program that reads them from one that never set them.
        NmeStmt::StartTimer => {
            names.insert(TIMER_NAME.to_string());
        }
        NmeStmt::Cooldown { target, .. } => {
            names.insert(format!("{COOLDOWN_PREFIX}{target}"));
        }
        NmeStmt::ForEach { name, inline, .. } => {
            names.insert(name.clone());
            if let Some(InlineStmt::Nme(inner)) = inline {
                remember_bindings(inner, names);
            }
        }
        NmeStmt::ModuleImport {
            names: imported, ..
        } => {
            for name in imported {
                names.insert(name.clone());
            }
        }
        NmeStmt::UseModule { module, .. } => {
            names.extend(
                module_binding_names(*module)
                    .iter()
                    .map(|name| (*name).to_string()),
            );
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
        // Concatenating separate English words is useful for attached Korean
        // endings, but in recovery it turns ordinary prose such as `I am`
        // into the one-edit condition starter `if`. Keep typo recovery local
        // to one ASCII token; exact multi-word legacy aliases still work.
        if mode == MatchMode::Recover
            && consumed > 1
            && tokens[start..start + consumed]
                .iter()
                .all(|token| token_word(token).is_some_and(str::is_ascii))
        {
            continue;
        }
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
        let exact_matches = expected
            .iter()
            .filter(|candidate| actual.eq_ignore_ascii_case(candidate))
            .count();
        if exact_matches == 1 {
            return Some(consumed);
        }
        if exact_matches > 1 {
            continue;
        }
        let recovered_matches = expected
            .iter()
            .filter(|candidate| word_matches(&actual, candidate, mode))
            .count();
        if recovered_matches == 1 {
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
    action_typo_away(
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

/// Action words tolerate one edit, plus the common two-keystroke typo where a
/// single extra/missing character is combined with a swap or replacement.
/// The match remains candidate-unique in `action_phrase_at`, so broad prose
/// is never silently assigned an arbitrary action.
fn action_typo_away(actual: &str, expected: &str) -> bool {
    if one_typo_away(actual, expected) {
        return true;
    }
    let actual_chars = actual.chars().collect::<Vec<_>>();
    let expected_chars = expected.chars().collect::<Vec<_>>();
    if actual_chars.len().abs_diff(expected_chars.len()) > 2 {
        return false;
    }
    for index in 1..actual_chars.len() {
        let mut shortened = actual_chars.clone();
        shortened.remove(index);
        if adjacent_transposition_away(&shortened, &expected_chars) {
            return true;
        }
    }
    for index in 0..expected_chars.len() {
        let mut shortened = expected_chars.clone();
        shortened.remove(index);
        if adjacent_transposition_away(&actual_chars, &shortened) {
            return true;
        }
    }
    false
}

fn adjacent_transposition_away(left: &[char], right: &[char]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let differences = left
        .iter()
        .zip(right)
        .enumerate()
        .filter_map(|(index, (a, b))| (a != b).then_some(index))
        .collect::<Vec<_>>();
    differences.len() == 2
        && differences[1] == differences[0] + 1
        && left[differences[0]] == right[differences[1]]
        && left[differences[1]] == right[differences[0]]
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

fn looks_like_future_python(tokens: &[Token]) -> bool {
    tokens.windows(2).any(|pair| {
        matches!(pair[0].tok, Tok::Name { .. })
            && matches!(pair[1].tok, Tok::String { .. })
            && pair[0].span.end == pair[1].span.start
    })
}

fn looks_like_plain_prose(tokens: &[Token]) -> bool {
    tokens.iter().all(|token| {
        token_word(token).is_some() || is_command_ending(token) || matches!(token.tok, Tok::Comma)
    })
}

fn has_recoverable_sentence_shape(tokens: &[Token]) -> bool {
    has_recoverable_repeat_shape(tokens)
        || output_action_at(tokens, 0, MatchMode::Recover).is_some()
        || output_action_ending(tokens, MatchMode::Recover).is_some()
        || find_ask_shape(tokens, MatchMode::Recover).is_some()
        || (set_action_at(tokens, 0, MatchMode::Recover).is_some() && tokens.len() > 1)
        || recoverable_module_shape(tokens)
        || (action_phrase_at(tokens, 0, USE_WORDS_EN, MatchMode::Recover).is_some()
            && tokens.len() > 1)
        || (action_phrase_at(tokens, 0, USE_WORDS_KO, MatchMode::Recover).is_some()
            && tokens.len() > 1)
}

fn ambiguous_action_diagnostic(tokens: &[Token]) -> Diagnostic {
    Diagnostic::bilingual(
        DiagnosticCode::AmbiguousAction,
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
        DiagnosticCode::MissingAction,
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

fn attached_korean_times_sentence(source: &str, tokens: &[Token]) -> Option<(Code, usize)> {
    let Token {
        tok: Tok::Name { name },
        span,
    } = tokens.first()?
    else {
        return None;
    };
    if tokens
        .get(1)
        .is_some_and(|token| matches!(token.tok, Tok::Colon))
    {
        return None;
    }
    let count = name.strip_suffix(TIMES_KEYWORD_KO)?;
    if count.is_empty() {
        return None;
    }
    let count_span = Span::new(span.start, span.end - TIMES_KEYWORD_KO.len());
    is_valid_python_expression(&source[count_span.start..count_span.end])
        .then_some((Code::Source(count_span), 1))
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

fn is_python_loop_header(tokens: &[Token]) -> bool {
    matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::For | Tok::While)
    ) || (matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::For)))
}

fn is_python_function_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Def))
        || (matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
            && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def)))
}

fn is_python_async_function_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Def))
}

fn is_python_async_for_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::For))
}

fn is_python_async_with_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Async))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::With))
}

fn contains_python_nonlocal(tokens: &[Token]) -> bool {
    python_declarations(tokens)
        .iter()
        .any(|declaration| matches!(declaration.kind, PythonDeclarationKind::Nonlocal))
}

fn is_python_import_star_line(tokens: &[Token]) -> bool {
    let depths = token_depths(tokens);
    tokens.iter().enumerate().any(|(start, token)| {
        if depths[start] != 0
            || !matches!(token.tok, Tok::From)
            || (start > 0
                && !(depths[start - 1] == 0 && matches!(tokens[start - 1].tok, Tok::Semi)))
        {
            return false;
        }
        let end = (start + 1..tokens.len())
            .find(|&index| depths[index] == 0 && matches!(tokens[index].tok, Tok::Semi))
            .unwrap_or(tokens.len());
        let statement = &tokens[start..end];
        let Some(import_index) = statement
            .iter()
            .position(|token| matches!(token.tok, Tok::Import))
        else {
            return false;
        };
        statement[import_index + 1..]
            .iter()
            .any(|token| matches!(token.tok, Tok::Star))
    })
}

fn is_python_except_star_control_line(tokens: &[Token]) -> bool {
    has_direct_python_statement(tokens, |tok| {
        matches!(tok, Tok::Break | Tok::Continue | Tok::Return)
    })
}

fn is_python_except_star_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Except))
        && matches!(tokens.get(1).map(|token| &token.tok), Some(Tok::Star))
}

fn is_python_try_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Try))
}

fn is_python_try_clause_header(tokens: &[Token]) -> bool {
    matches!(
        tokens.first().map(|token| &token.tok),
        Some(Tok::Except | Tok::Else | Tok::Finally)
    )
}

fn is_python_class_header(tokens: &[Token]) -> bool {
    matches!(tokens.first().map(|token| &token.tok), Some(Tok::Class))
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

#[cfg(test)]
mod zero_knowledge_tests {

    use crate::diagnostics::DiagnosticCode;
    use crate::transpile;

    #[test]
    fn zero_knowledge_nizk_sentences_bind_an_explicit_context() {
        let source = "영지식 사용 최신
비밀값은 영지식 비밀 만들기
공개값은 비밀값으로 영지식 공개값 만들기
문맥값은 결제 승인 요청
증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기
검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증
일회값은 영지식 일회값 만들기
약속값은 일회값으로 영지식 약속 만들기
도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기
";
        let python = transpile(source).expect("context-bound NIZK sentences must transpile");
        assert!(
            python.contains("증명값 = zk_nizk_prove(비밀값, 문맥값)"),
            "{python}"
        );
        assert!(
            python.contains("검증값 = zk_nizk_verify(공개값, 증명값, 문맥값)"),
            "{python}"
        );
        assert!(
            python.contains("도전값 = zk_nizk_challenge(공개값, 약속값, 문맥값)"),
            "{python}"
        );
    }

    #[test]
    fn zero_knowledge_sentence_values_lower_without_python_punctuation() {
        let source = "영지식 사용 최신
비밀값은 영지식 비밀 만들기
공개값은 비밀값으로 영지식 공개값 만들기
일회값은 영지식 일회값 만들기
약속값은 일회값으로 영지식 약속 만들기
도전값은 영지식 도전 만들기
응답값은 일회값과 비밀값과 도전값으로 영지식 응답 만들기
검증값은 공개값과 약속값과 도전값과 응답값으로 영지식 검증
";
        let python = transpile(source).expect("zero-knowledge sentences must transpile");
        assert!(python.contains("import secrets as 영지식비밀난수"));
        assert!(python.contains(r#"비밀값 = __import__("secrets").randbelow"#));
        assert!(python.contains("공개값 = pow(2, 비밀값, 0x"));
        assert!(python.contains("검증값 = (1 < (공개값)"));
    }

    #[test]
    fn zero_knowledge_module_protects_helper_names() {
        let source = "영지식검증 = 1
영지식 사용 최신
";
        let problems = transpile(source).expect_err("helper collision must be rejected");
        assert!(problems
            .iter()
            .any(|problem| problem.code == DiagnosticCode::ModuleNameCollision));
    }
}
