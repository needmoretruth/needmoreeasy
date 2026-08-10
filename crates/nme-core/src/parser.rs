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
    Code, InlineStmt, InputKind, ModuleVersion, NmeLine, NmeStmt, Spelling, TextPart, TextTemplate,
    Value, RANDOM_MODULE, RANDOM_MODULE_KO, RANDOM_MODULE_VERSION, SAY_KEYWORD, SAY_KEYWORD_KO,
    TIMES_KEYWORD, TIMES_KEYWORD_KO,
};

const SAY_WORDS_EN: &[&str] = &["say", "show", "display", "tell", "print"];
const SAY_WORDS_KO: &[&str] = &[
    "말해",
    "말해줘",
    "말해주세요",
    "보여줘",
    "보여주세요",
    "출력해",
    "출력해줘",
];
const ASK_WORDS_EN: &[&str] = &["ask", "prompt", "question"];
const ASK_WORDS_KO: &[&str] = &[
    "물어봐",
    "물어봐줘",
    "물어보세요",
    "질문해",
    "질문해줘",
    "입력받아",
];
const REPEAT_WORDS_EN: &[&str] = &["repeat", "again", "do"];
const REPEAT_WORDS_KO: &[&str] = &["반복", "반복해", "반복해줘", "반복하세요", "반복해서"];
const WHEN_WORDS_EN: &[&str] = &["when", "if"];
const WHEN_WORDS_KO: &[&str] = &["만약", "만약에", "만일", "혹시"];
const USE_WORDS_EN: &[&str] = &["use", "load", "get", "import"];
const USE_WORDS_KO: &[&str] = &["사용", "사용해", "사용해줘", "불러와", "가져와", "받아"];
const LATEST_WORDS: &[&str] = &["latest", "newest", "최신", "최신판", "최신버전"];
const NUMBER_WORDS: &[&str] = &["number", "numeric", "숫자", "숫자로", "수로"];
const QUOTE_PARTICLES: &[&str] = &["라고", "이라고", "하고"];
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
];

/// Parse all logical lines, collecting independent beginner-facing errors.
pub fn parse(source: &str, lines: &[LogicalLine]) -> Result<Vec<NmeLine>, Vec<Diagnostic>> {
    let mut found = Vec::new();
    let mut problems = Vec::new();
    let mut known_names = discover_python_bindings(lines);

    for (index, line) in lines.iter().enumerate() {
        let next_indent = lines.get(index + 1).map(|next| next.indent);
        let block = BlockCtx::TopLevel { line, next_indent };
        match classify(source, &line.tokens, &block, &known_names) {
            Ok(Some(stmt)) => {
                remember_bindings(&stmt, &mut known_names);
                found.push(NmeLine {
                    span: line.span,
                    stmt,
                });
            }
            Ok(None) => remember_python_binding(&line.tokens, &mut known_names),
            Err(problem) => problems.push(problem),
        }
    }

    if problems.is_empty() {
        Ok(found)
    } else {
        Err(problems)
    }
}

enum BlockCtx<'a> {
    TopLevel {
        line: &'a LogicalLine,
        next_indent: Option<usize>,
    },
    Inline,
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

    if is_python_keyword(&tokens[0].tok) && !matches!(tokens[0].tok, Tok::If) {
        return Ok(None);
    }
    if let Some(stmt) = match_when(source, tokens, block, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_times(source, tokens, block, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_ask(source, tokens, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_say(source, tokens, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_set(source, tokens, known_names)? {
        return Ok(Some(stmt));
    }
    if let Some(stmt) = match_use_random(source, tokens)? {
        return Ok(Some(stmt));
    }

    if tokens.iter().any(is_sentence_punctuation) {
        return Err(Diagnostic::new(
            "`?` and `!` can be used in sentence-style NME, but this line was ambiguous",
            span_of(tokens),
        )
        .with_hint(
            "add `show` / `말해줘` or `ask` / `물어봐` so the sentence has one clear meaning",
        ));
    }

    // Invalid Python led by another Python keyword belongs to Python. This
    // preserves its own context-sensitive diagnostics (`elif`, `except`, ...)
    // while still allowing the deliberately supported mixed `if 조건` form.
    if is_python_keyword(&tokens[0].tok) {
        return Ok(None);
    }
    Ok(None)
}

// ---------------------------------------------------------------- output

fn match_say(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(spelling) = output_word(&tokens[0]) {
        if tokens.len() == 1 {
            return Err(say_missing(spelling, tokens[0].span));
        }
        let body = &tokens[1..];
        let prefer_text = !token_is_exact_name(&tokens[0], SAY_KEYWORD)
            && !token_is_exact_name(&tokens[0], SAY_KEYWORD_KO);
        if !prefer_text {
            let span = span_of(body);
            let text = &source[span.start..span.end];
            if looks_like_broken_expression(body) && !is_valid_python_expression(text) {
                return Err(match spelling {
                    Spelling::English => {
                        Diagnostic::new("I couldn't understand what you want to `say`", span)
                            .with_hint(
                                "finish the value, or use plain words such as `show Hello world`",
                            )
                    }
                    Spelling::Korean => Diagnostic::new("`말해` 뒤의 값을 이해하지 못했어요", span)
                        .with_hint(
                            "값을 완성하거나 `안녕하세요 말해줘`처럼 평범한 문장으로 쓰세요",
                        ),
                });
            }
        }
        let value = parse_value(source, body, known_names, prefer_text).map_err(|()| {
            Diagnostic::new(
                if spelling == Spelling::Korean {
                    "무엇을 말할지 이해하지 못했어요"
                } else {
                    "I couldn't understand what to show"
                },
                span_of(body),
            )
            .with_hint(if spelling == Spelling::Korean {
                "`안녕하세요 말해줘`처럼 평범한 문장으로 적어도 돼요"
            } else {
                "write a value, or a sentence such as `show Hello world`"
            })
        })?;
        return Ok(Some(NmeStmt::Say { value }));
    }

    let Some(spelling) = output_word(&tokens[tokens.len() - 1]) else {
        return Ok(None);
    };
    let mut end = tokens.len() - 1;
    if end > 0 && token_matches_any(&tokens[end - 1], QUOTE_PARTICLES) {
        end -= 1;
    }
    if end == 0 {
        return Err(say_missing(spelling, tokens[tokens.len() - 1].span));
    }
    let value = parse_value(source, &tokens[..end], known_names, true).map_err(|()| {
        Diagnostic::new(
            if spelling == Spelling::Korean {
                "말할 문장을 이해하지 못했어요"
            } else {
                "I couldn't understand the sentence to show"
            },
            span_of(&tokens[..end]),
        )
        .with_hint(if spelling == Spelling::Korean {
            "`안녕하세요 말해줘`처럼 쓰세요"
        } else {
            "write it like `Hello world show`"
        })
    })?;
    Ok(Some(NmeStmt::Say { value }))
}

fn say_missing(spelling: Spelling, span: Span) -> Diagnostic {
    match spelling {
        Spelling::English => {
            Diagnostic::new("there is nothing to show", span).with_hint("write `show Hello world`")
        }
        Spelling::Korean => Diagnostic::new("말할 내용이 비어 있어요", span)
            .with_hint("`안녕하세요 말해줘`처럼 내용을 함께 적어 주세요"),
    }
}

fn output_word(token: &Token) -> Option<Spelling> {
    if token_matches_any(token, SAY_WORDS_EN) {
        Some(Spelling::English)
    } else if token_matches_any(token, SAY_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

// ---------------------------------------------------------------- input

fn match_ask(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some((ask_at, spelling)) = tokens
        .iter()
        .take(4)
        .enumerate()
        .find_map(|(index, token)| ask_word(token).map(|spelling| (index, spelling)))
    else {
        return Ok(None);
    };

    let (target_at, kind, prompt_start) = if ask_at == 0 {
        let mut cursor = 1;
        let kind = if tokens
            .get(cursor)
            .is_some_and(|token| token_matches_any(token, NUMBER_WORDS))
        {
            cursor += 1;
            InputKind::Number
        } else {
            InputKind::Text
        };
        (cursor, kind, cursor + 1)
    } else {
        let kind = if tokens[..ask_at]
            .iter()
            .any(|token| token_matches_any(token, NUMBER_WORDS))
        {
            InputKind::Number
        } else {
            InputKind::Text
        };
        (0, kind, ask_at + 1)
    };

    let Some(target_token) = tokens.get(target_at) else {
        return Err(ask_target_diagnostic(spelling, tokens[ask_at].span));
    };
    let Some(target_word) = name_word(target_token) else {
        return Err(ask_target_diagnostic(spelling, target_token.span));
    };
    let target = strip_target_particle(target_word).to_string();
    if target.is_empty() {
        return Err(ask_target_diagnostic(spelling, target_token.span));
    }

    let prompt = if prompt_start >= tokens.len() {
        None
    } else if matches!(tokens[prompt_start].tok, Tok::Comma) {
        let expression_tokens = &tokens[prompt_start + 1..];
        if expression_tokens.is_empty() {
            return Err(match spelling {
                Spelling::English => Diagnostic::new(
                    "the question after the comma is missing",
                    tokens[prompt_start].span,
                )
                .with_hint("add a question after the comma"),
                Spelling::Korean => {
                    Diagnostic::new("쉼표 뒤의 질문이 비어 있어요", tokens[prompt_start].span)
                        .with_hint("쉼표 뒤에 질문을 적어 주세요")
                }
            });
        }
        let span = span_of(expression_tokens);
        if !is_valid_python_expression(&source[span.start..span.end]) {
            return Err(match spelling {
                Spelling::English => Diagnostic::new("I couldn't understand the question", span)
                    .with_hint("remove the comma to write a plain sentence without quotes"),
                Spelling::Korean => Diagnostic::new("질문 내용을 이해하지 못했어요", span)
                    .with_hint("쉼표를 빼면 따옴표 없는 평범한 문장으로 쓸 수 있어요"),
            });
        }
        Some(Value::Python(Code::Source(span)))
    } else {
        // A comma means precise beginner syntax. Without one, the remainder is
        // deliberately sentence text and therefore needs no quotes.
        let prompt_tokens = &tokens[prompt_start..];
        let prompt_span = span_of(prompt_tokens);
        if is_valid_python_expression(&source[prompt_span.start..prompt_span.end])
            && !matches!(prompt_tokens[0].tok, Tok::Name { .. })
        {
            Some(Value::Python(Code::Source(prompt_span)))
        } else {
            Some(Value::Text(make_text_template(
                source,
                prompt_tokens,
                &HashSet::new(),
            )))
        }
    };

    // The prompt may refer to names known before this input, but never treats
    // the target being declared as an interpolation accidentally.
    let prompt = prompt.map(|value| interpolate_existing(value, source, known_names));
    Ok(Some(NmeStmt::Ask {
        target,
        prompt,
        kind,
    }))
}

fn ask_word(token: &Token) -> Option<Spelling> {
    if token_matches_any(token, ASK_WORDS_EN) {
        Some(Spelling::English)
    } else if token_matches_any(token, ASK_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

fn ask_target_diagnostic(spelling: Spelling, span: Span) -> Diagnostic {
    match spelling {
        Spelling::English => Diagnostic::new("write the name that should hold the answer", span)
            .with_hint("for example: `ask name What is your name`"),
        Spelling::Korean => Diagnostic::new("대답을 담을 이름이 필요해요", span)
            .with_hint("`이름을 물어봐 이름이 뭐예요`처럼 쓰세요"),
    }
}

// -------------------------------------------------------------- condition

fn match_when(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    let Some(spelling) = when_word(&tokens[0]) else {
        return Ok(None);
    };
    let starter_exact = matches!(tokens[0].tok, Tok::If)
        || token_word(&tokens[0])
            .is_some_and(|word| WHEN_WORDS_EN.contains(&word) || WHEN_WORDS_KO.contains(&word));
    if tokens.len() == 1 {
        return Err(condition_missing(spelling, tokens[0].span));
    }

    if let Some(colon_at) = find_condition_colon(source, tokens) {
        if colon_at == 1 {
            return Err(condition_missing(spelling, tokens[colon_at].span));
        }
        let condition_span = Span::new(tokens[1].span.start, tokens[colon_at - 1].span.end);
        if !is_valid_python_expression(&source[condition_span.start..condition_span.end]) {
            return Err(condition_invalid(spelling, condition_span));
        }
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Condition(spelling),
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::When {
            condition: Code::Source(condition_span),
            inline,
        }));
    }

    let natural = find_condition_connector(&tokens[1..]);
    if !starter_exact && natural.is_none() && matches!(block, BlockCtx::Inline) {
        // A short sentence word may be one edit away from a condition alias.
        // Without a connector, colon, or following block there is not enough
        // evidence to recover it as a typo, so let another construct decide.
        return Ok(None);
    }
    let (condition_tokens, connector, body) = match natural {
        Some((relative_at, connector)) => {
            let at = relative_at + 1;
            (&tokens[1..at], Some(connector), &tokens[at + 1..])
        }
        None => (&tokens[1..], None, &tokens[tokens.len()..]),
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
        SuiteKind::Condition(spelling),
        span_of(tokens),
        known_names,
    )?;
    Ok(Some(NmeStmt::When { condition, inline }))
}

fn when_word(token: &Token) -> Option<Spelling> {
    if matches!(token.tok, Tok::If) || token_matches_any(token, WHEN_WORDS_EN) {
        Some(Spelling::English)
    } else if token_matches_any(token, WHEN_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum ConditionConnector {
    Then,
    Exists,
    Missing,
    Equals,
    Greater,
    Less,
}

fn find_condition_connector(tokens: &[Token]) -> Option<(usize, ConditionConnector)> {
    // `then` / `그러면` is the suite boundary. Prefer it over words such as
    // `exists` that belong to the condition immediately before it.
    if let Some((index, _)) = tokens.iter().enumerate().find(|(_, token)| {
        token_word(token).is_some_and(|word| matches!(word, "then" | "그러면" | "그럼"))
    }) {
        return Some((index, ConditionConnector::Then));
    }
    for (index, token) in tokens.iter().enumerate() {
        let Some(word) = token_word(token) else {
            continue;
        };
        let connector = match word {
            "경우" | "때" | "일때" => ConditionConnector::Then,
            "exists" if index + 1 == tokens.len() => ConditionConnector::Exists,
            "missing" if index + 1 == tokens.len() => ConditionConnector::Missing,
            "있으면" | "있다면" => ConditionConnector::Exists,
            "없으면" | "없다면" => ConditionConnector::Missing,
            "같으면" | "같다면" | "이면" | "라면" => ConditionConnector::Equals,
            "크면" | "크다면" => ConditionConnector::Greater,
            "작으면" | "작다면" => ConditionConnector::Less,
            _ => continue,
        };
        return Some((index, connector));
    }
    None
}

fn parse_natural_condition(
    source: &str,
    tokens: &[Token],
    connector: Option<ConditionConnector>,
    known_names: &HashSet<String>,
    spelling: Spelling,
) -> Result<Code, Diagnostic> {
    let cleaned: Vec<&Token> = tokens
        .iter()
        .filter(|token| !token_matches_any(token, &["정말", "혹시", "please", "really", "the"]))
        .collect();
    if cleaned.is_empty() {
        return Err(condition_missing(spelling, span_of(tokens)));
    }

    if let Some(condition) = parse_english_condition(source, &cleaned, known_names) {
        return Ok(condition);
    }

    match connector {
        Some(ConditionConnector::Missing) => {
            let subject = natural_subject(cleaned[0], known_names);
            return Ok(Code::Generated(format!("not ({subject})")));
        }
        Some(ConditionConnector::Exists) => {
            let subject = natural_subject(cleaned[0], known_names);
            return Ok(Code::Generated(subject));
        }
        Some(ConditionConnector::Greater | ConditionConnector::Less) => {
            if let Some((left, right)) = comparison_sides(source, &cleaned, known_names) {
                let operator = if matches!(connector, Some(ConditionConnector::Greater)) {
                    ">"
                } else {
                    "<"
                };
                return Ok(Code::Generated(format!("{left} {operator} {right}")));
            }
        }
        Some(ConditionConnector::Equals) if cleaned.len() >= 2 => {
            let left = natural_subject(cleaned[0], known_names);
            let mut right = &cleaned[1..];
            if right
                .last()
                .is_some_and(|token| token_matches_any(token, &["과", "와", "to"]))
            {
                right = &right[..right.len() - 1];
            }
            if right.is_empty() {
                return Err(condition_invalid(spelling, span_of(tokens)));
            }
            if right.len() == 1 {
                let right = name_word(right[0]).map_or_else(
                    || source[right[0].span.start..right[0].span.end].to_string(),
                    |_| natural_subject(right[0], known_names),
                );
                return Ok(Code::Generated(format!("{left} == {right}")));
            }
            let right_tokens: Vec<Token> = right.iter().map(|token| (*token).clone()).collect();
            let right_span = span_of(&right_tokens);
            if is_valid_python_expression(&source[right_span.start..right_span.end]) {
                return Ok(Code::Generated(format!(
                    "{left} == {}",
                    &source[right_span.start..right_span.end]
                )));
            }
        }
        _ => {}
    }

    if cleaned.len() == 1 {
        if let Some(word) = name_word(cleaned[0]) {
            if let Some(base) = resolve_known_particle(word, known_names) {
                return Ok(Code::Generated(base.to_string()));
            }
        }
    }

    let condition_span = Span::new(cleaned[0].span.start, cleaned[cleaned.len() - 1].span.end);
    let condition_text = &source[condition_span.start..condition_span.end];
    if is_valid_python_expression(condition_text) {
        return Ok(Code::Source(condition_span));
    }

    Err(condition_invalid(spelling, condition_span))
}

fn comparison_sides(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
) -> Option<(String, String)> {
    if tokens.len() < 2 {
        return None;
    }
    let left = natural_subject(tokens[0], known_names);
    if tokens.len() == 2 {
        return Some((left, natural_subject(tokens[1], known_names)));
    }
    let mut right_start = 1;
    while right_start < tokens.len()
        && token_matches_any(tokens[right_start], &["is", "than", "보다", "더"])
    {
        right_start += 1;
    }
    let mut right_end = tokens.len();
    while right_end > right_start
        && token_matches_any(tokens[right_end - 1], &["is", "than", "보다", "더"])
    {
        right_end -= 1;
    }
    if right_start == right_end {
        return None;
    }
    let span = Span::new(
        tokens[right_start].span.start,
        tokens[right_end - 1].span.end,
    );
    let text = &source[span.start..span.end];
    is_valid_python_expression(text).then(|| (left, text.to_string()))
}

fn parse_english_condition(
    source: &str,
    tokens: &[&Token],
    known_names: &HashSet<String>,
) -> Option<Code> {
    let last_word = tokens.last().and_then(|token| token_word(token));
    if matches!(last_word, Some("exists" | "present")) {
        return Some(Code::Generated(natural_subject(tokens[0], known_names)));
    }
    if matches!(last_word, Some("missing" | "absent")) {
        return Some(Code::Generated(format!(
            "not ({})",
            natural_subject(tokens[0], known_names)
        )));
    }

    let (operator_at, operator) =
        tokens
            .iter()
            .enumerate()
            .find_map(|(index, token)| match token_word(token) {
                Some("greater" | "above") => Some((index, ">")),
                Some("less" | "below") => Some((index, "<")),
                Some("equals" | "equal") => Some((index, "==")),
                _ => None,
            })?;
    if operator_at == 0 {
        return None;
    }
    let left = natural_subject(tokens[0], known_names);
    let mut right_start = operator_at + 1;
    while right_start < tokens.len()
        && token_matches_any(tokens[right_start], &["to", "than", "is"])
    {
        right_start += 1;
    }
    if right_start >= tokens.len() {
        return None;
    }
    let right = if right_start + 1 == tokens.len() {
        name_word(tokens[right_start])
            .and_then(|word| resolve_known_particle(word, known_names))
            .map_or_else(
                || source[tokens[right_start].span.start..tokens[right_start].span.end].to_string(),
                ToString::to_string,
            )
    } else {
        let span = Span::new(
            tokens[right_start].span.start,
            tokens[tokens.len() - 1].span.end,
        );
        let text = &source[span.start..span.end];
        if !is_valid_python_expression(text) {
            return None;
        }
        text.to_string()
    };
    Some(Code::Generated(format!("{left} {operator} {right}")))
}

fn natural_subject(token: &Token, known_names: &HashSet<String>) -> String {
    name_word(token)
        .and_then(|word| resolve_known_particle(word, known_names))
        .unwrap_or_else(|| name_word(token).unwrap_or("False"))
        .to_string()
}

fn condition_missing(spelling: Spelling, span: Span) -> Diagnostic {
    match spelling {
        Spelling::English => Diagnostic::new("the condition is missing", span)
            .with_hint("write `if ready` or `if score > 10` and indent the next line"),
        Spelling::Korean => Diagnostic::new("조건이 비어 있어요", span)
            .with_hint("`만약에 준비됐으면`처럼 적고 다음 줄을 들여쓰세요"),
    }
}

fn condition_invalid(spelling: Spelling, span: Span) -> Diagnostic {
    match spelling {
        Spelling::English => Diagnostic::new("I couldn't understand this condition", span)
            .with_hint("try `if ready`, `if score > 10`, or `if name exists`"),
        Spelling::Korean => Diagnostic::new("이 조건을 확실하게 이해하지 못했어요", span)
            .with_hint("`만약에 이름이 있으면` 또는 `만약 점수 > 10`처럼 적어 보세요"),
    }
}

// --------------------------------------------------------------- repeat

fn match_times(
    source: &str,
    tokens: &[Token],
    block: &BlockCtx<'_>,
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some((times_at, spelling)) = find_times_colon(tokens) {
        let count = parse_count(source, &tokens[..times_at], spelling)?;
        let colon_at = times_at + 1;
        let inline = parse_suite_body(
            source,
            &tokens[colon_at + 1..],
            block,
            SuiteKind::Repeat(spelling),
            Span::new(tokens[0].span.start, tokens[colon_at].span.end),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    // Sentence order: `3번 반복해 ...` / `3 times repeat ...`.
    if let Some((marker_at, spelling)) = find_count_marker(tokens) {
        if let Some(repeat_token) = tokens.get(marker_at + 1) {
            if repeat_word(repeat_token).is_some() {
                if marker_at == 0 {
                    return Err(repeat_count_missing(spelling, repeat_token.span));
                }
                let count = parse_count(source, &tokens[..marker_at], spelling)?;
                let mut body_start = marker_at + 2;
                if tokens.get(body_start).is_some_and(is_connector_word) {
                    body_start += 1;
                }
                let inline = parse_suite_body(
                    source,
                    &tokens[body_start..],
                    block,
                    SuiteKind::Repeat(spelling),
                    span_of(&tokens[..body_start]),
                    known_names,
                )?;
                return Ok(Some(NmeStmt::Times { count, inline }));
            }
        }
    }

    // English-first and freely mixed order: `repeat 3 times` / `반복해 3 times`.
    if let Some(spelling) = repeat_word(&tokens[0]) {
        let Some((relative_marker, marker_spelling)) = find_count_marker(&tokens[1..]) else {
            return Err(repeat_count_missing(spelling, tokens[0].span));
        };
        let marker_at = relative_marker + 1;
        if marker_at == 1 {
            return Err(repeat_count_missing(spelling, tokens[0].span));
        }
        let count = parse_count(source, &tokens[1..marker_at], marker_spelling)?;
        let mut body_start = marker_at + 1;
        if tokens.get(body_start).is_some_and(is_connector_word) {
            body_start += 1;
        }
        let inline = parse_suite_body(
            source,
            &tokens[body_start..],
            block,
            SuiteKind::Repeat(spelling),
            span_of(&tokens[..body_start]),
            known_names,
        )?;
        return Ok(Some(NmeStmt::Times { count, inline }));
    }

    Ok(None)
}

fn repeat_word(token: &Token) -> Option<Spelling> {
    if token_matches_any(token, REPEAT_WORDS_EN) {
        Some(Spelling::English)
    } else if token_matches_any(token, REPEAT_WORDS_KO) {
        Some(Spelling::Korean)
    } else {
        None
    }
}

fn parse_count(source: &str, tokens: &[Token], spelling: Spelling) -> Result<Code, Diagnostic> {
    if tokens.is_empty() {
        return Err(repeat_count_missing(spelling, Span::new(0, 0)));
    }
    let span = span_of(tokens);
    if !is_valid_python_expression(&source[span.start..span.end]) {
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("I couldn't understand how many times to repeat", span)
                    .with_hint("write a number, like `repeat 3 times`")
            }
            Spelling::Korean => Diagnostic::new("몇 번 반복할지 이해하지 못했어요", span)
                .with_hint("`3번 반복해`처럼 횟수를 적어 주세요"),
        });
    }
    Ok(Code::Source(span))
}

fn repeat_count_missing(spelling: Spelling, span: Span) -> Diagnostic {
    match spelling {
        Spelling::English => {
            Diagnostic::new("the repeat count is missing", span).with_hint("write `repeat 3 times`")
        }
        Spelling::Korean => Diagnostic::new("반복 횟수가 비어 있어요", span)
            .with_hint("`3번 반복해`처럼 숫자를 함께 적어 주세요"),
    }
}

fn find_count_marker(tokens: &[Token]) -> Option<(usize, Spelling)> {
    tokens.iter().enumerate().find_map(|(index, token)| {
        if token_is_exact_name(token, TIMES_KEYWORD) {
            Some((index, Spelling::English))
        } else if token_is_exact_name(token, TIMES_KEYWORD_KO) {
            Some((index, Spelling::Korean))
        } else {
            None
        }
    })
}

// --------------------------------------------------------------- modules

fn match_use_random(source: &str, tokens: &[Token]) -> Result<Option<NmeStmt>, Diagnostic> {
    let has_use = tokens.iter().any(|token| {
        token_matches_any(token, USE_WORDS_EN) || token_matches_any(token, USE_WORDS_KO)
    });
    if !has_use {
        return Ok(None);
    }

    let has_random = tokens.iter().any(is_random_word);
    if !has_random {
        let has_exact_use = tokens.iter().any(|token| {
            token_word(token)
                .is_some_and(|word| USE_WORDS_EN.contains(&word) || USE_WORDS_KO.contains(&word))
        });
        if !has_exact_use {
            return Ok(None);
        }
        let spelling = if tokens
            .iter()
            .any(|token| token_matches_any(token, USE_WORDS_KO))
        {
            Spelling::Korean
        } else {
            Spelling::English
        };
        return Err(match spelling {
            Spelling::English => {
                Diagnostic::new("NME only bundles `use random` for now", span_of(tokens))
                    .with_hint("use `random use latest`, or use an ordinary Python import")
            }
            Spelling::Korean => {
                Diagnostic::new("이 쉬운 모듈은 아직 들어 있지 않아요", span_of(tokens))
                    .with_hint("`랜덤 사용 최신`을 쓰거나 평범한 Python import를 사용하세요")
            }
        });
    }

    let requested = if tokens
        .iter()
        .any(|token| token_matches_any(token, LATEST_WORDS))
    {
        ModuleVersion::Latest
    } else if let Some(version_at) = tokens
        .iter()
        .position(|token| token_matches_any(token, &["version", "버전"]))
    {
        let value = tokens.get(version_at + 1).ok_or_else(|| {
            Diagnostic::new(
                "모듈 버전이 비어 있어요 / module version is missing",
                tokens[version_at].span,
            )
            .with_hint(format!(
                "use `latest` / `최신`, or version {RANDOM_MODULE_VERSION}"
            ))
        })?;
        let raw = &source[value.span.start..value.span.end];
        let version = raw.trim_matches(['\'', '"']).to_string();
        if version != RANDOM_MODULE_VERSION {
            return Err(Diagnostic::new(
                format!("random version {version} is not bundled"),
                value.span,
            )
            .with_hint(format!(
                "use `latest` / `최신`; this compiler bundles {RANDOM_MODULE_VERSION}"
            )));
        }
        ModuleVersion::Exact(version)
    } else {
        ModuleVersion::Bundled
    };

    Ok(Some(NmeStmt::UseRandom { requested }))
}

fn is_random_word(token: &Token) -> bool {
    name_word(token).is_some_and(|word| {
        word == RANDOM_MODULE
            || word == RANDOM_MODULE_KO
            || strip_target_particle(word) == RANDOM_MODULE_KO
    })
}

// ------------------------------------------------------------ assignment

fn match_set(
    source: &str,
    tokens: &[Token],
    known_names: &HashSet<String>,
) -> Result<Option<NmeStmt>, Diagnostic> {
    if let Some(first) = name_word(&tokens[0]) {
        if let Some(target) = strip_assignment_particle(first) {
            if tokens.len() == 1 {
                return Err(Diagnostic::new("저장할 값이 비어 있어요", tokens[0].span)
                    .with_hint("`인사는 안녕하세요`처럼 값을 뒤에 적어 주세요"));
            }
            let value = parse_value(source, &tokens[1..], known_names, true).map_err(|()| {
                Diagnostic::new("저장할 값을 이해하지 못했어요", span_of(&tokens[1..]))
                    .with_hint("숫자, 이름, 또는 평범한 문장을 적어 주세요")
            })?;
            return Ok(Some(NmeStmt::Set {
                target: target.to_string(),
                value,
            }));
        }
    }

    if token_matches_any(&tokens[0], &["set", "save", "remember"]) {
        let Some(target_token) = tokens.get(1) else {
            return Err(
                Diagnostic::new("the name to save is missing", tokens[0].span)
                    .with_hint("write `set greeting to Hello`"),
            );
        };
        let Some(target) = name_word(target_token) else {
            return Err(Diagnostic::new("use a simple name here", target_token.span)
                .with_hint("write `set greeting to Hello`"));
        };
        let mut value_start = 2;
        if tokens
            .get(value_start)
            .is_some_and(|token| token_matches_any(token, &["to", "as", "is"]))
        {
            value_start += 1;
        }
        if value_start >= tokens.len() {
            return Err(
                Diagnostic::new("the value to save is missing", target_token.span)
                    .with_hint("write `set greeting to Hello`"),
            );
        }
        let value =
            parse_value(source, &tokens[value_start..], known_names, true).map_err(|()| {
                Diagnostic::new(
                    "I couldn't understand the value to save",
                    span_of(&tokens[value_start..]),
                )
                .with_hint("write a number, name, or plain sentence")
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
    if is_valid_python_expression(text) && (!prefer_text || single_known_name || clearly_code) {
        return Ok(Value::Python(Code::Source(span)));
    }
    Ok(Value::Text(make_text_template(source, tokens, known_names)))
}

fn parse_random_integer(source: &str, tokens: &[Token]) -> Option<Value> {
    let random_at = tokens.iter().position(|token| {
        token_matches_any(
            token,
            &[
                "랜덤",
                "랜덤정수",
                "무작위",
                "무작위숫자",
                "random",
                "randomnumber",
            ],
        )
    })?;

    // Korean/mixed order: `1부터 6까지 랜덤정수`.
    if random_at > 0 {
        let from_at = tokens[..random_at]
            .iter()
            .position(|token| token_matches_any(token, &["부터", "에서", "from"]))?;
        let to_at = tokens[from_at + 1..random_at]
            .iter()
            .position(|token| token_matches_any(token, &["까지", "to"]))?
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
        .position(|token| token_matches_any(token, &["from", "부터", "에서"]))?;
    let to_at = tokens[from_at + 1..]
        .iter()
        .position(|token| token_matches_any(token, &["to", "까지"]))?
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
        token_matches_any(
            token,
            &["랜덤선택", "하나골라", "골라", "randomchoice", "pick"],
        )
    })?;
    let choices_tokens = if pick_at == 0 {
        let start = tokens
            .iter()
            .position(|token| token_matches_any(token, &["from", "중에서"]))?
            + 1;
        &tokens[start..]
    } else {
        &tokens[..pick_at]
    };
    let choices: Vec<String> = choices_tokens
        .iter()
        .filter(|token| {
            !token_matches_any(token, &["or", "and", "또는", "이나", "중", "중에서"])
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

fn interpolate_existing(value: Value, _source: &str, _known_names: &HashSet<String>) -> Value {
    value
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
    Repeat(Spelling),
    Condition(Spelling),
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
        SuiteKind::Repeat(Spelling::English) => {
            Diagnostic::new("the lines that should repeat must be indented", span)
                .with_hint("or keep it on one line: `repeat 3 times and show Hello`")
        }
        SuiteKind::Repeat(Spelling::Korean) => {
            Diagnostic::new("반복할 다음 줄은 들여써야 해요", span)
                .with_hint("한 줄로 `3번 반복해서 안녕 말해줘`라고 써도 돼요")
        }
        SuiteKind::Condition(Spelling::English) => {
            Diagnostic::new("this condition needs `:` or an indented next line", span)
                .with_hint("or put one statement after `then`")
        }
        SuiteKind::Condition(Spelling::Korean) => {
            Diagnostic::new("조건 다음에는 실행할 줄이나 `:`이 필요해요", span)
                .with_hint("한 문장은 `있으면` 뒤에 바로 적어도 돼요")
        }
    }
}

fn inline_block_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    let spelling = suite_spelling(kind);
    match spelling {
        Spelling::English => Diagnostic::new("a block can't start here without a statement", span)
            .with_hint("put one statement here, or use an indented block on the next line"),
        Spelling::Korean => Diagnostic::new("이 한 줄 블록에 실행할 문장이 없어요", span)
            .with_hint("실행할 문장을 이어 쓰거나 다음 줄에 들여쓰세요"),
    }
}

fn one_statement_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match suite_spelling(kind) {
        Spelling::English => Diagnostic::new("only one statement fits on this line", span)
            .with_hint("put multiple statements on separate indented lines"),
        Spelling::Korean => Diagnostic::new("한 줄에는 문장 하나만 넣을 수 있어요", span)
            .with_hint("여러 문장은 다음 줄부터 하나씩 들여쓰세요"),
    }
}

fn body_diagnostic(kind: SuiteKind, span: Span) -> Diagnostic {
    match suite_spelling(kind) {
        Spelling::English => Diagnostic::new("I couldn't understand the statement here", span)
            .with_hint("write one Python, beginner, or sentence-style statement"),
        Spelling::Korean => Diagnostic::new("여기 있는 문장을 이해하지 못했어요", span)
            .with_hint("Python, 초급, 문장형 문법 중 한 문장을 적어 주세요"),
    }
}

fn suite_spelling(kind: SuiteKind) -> Spelling {
    match kind {
        SuiteKind::Repeat(spelling) | SuiteKind::Condition(spelling) => spelling,
    }
}

// --------------------------------------------------------------- helpers

pub(crate) fn discover_python_bindings(lines: &[LogicalLine]) -> HashSet<String> {
    let mut names = HashSet::new();
    for line in lines {
        remember_python_binding(&line.tokens, &mut names);
        if matches!(line.tokens.first().map(|token| &token.tok), Some(Tok::Def)) {
            let mut inside_parameters = false;
            for token in &line.tokens {
                match &token.tok {
                    Tok::Lpar => inside_parameters = true,
                    Tok::Rpar => inside_parameters = false,
                    Tok::Name { name } if inside_parameters => {
                        names.insert(name.clone());
                    }
                    _ => {}
                }
            }
        }
    }
    names
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
        || token_matches_any(token, &["and", "then", "해서", "그리고", "그러면"])
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

fn find_times_colon(tokens: &[Token]) -> Option<(usize, Spelling)> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate() {
        match &token.tok {
            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => depth += 1,
            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => depth = depth.saturating_sub(1),
            Tok::Name { name }
                if (name == TIMES_KEYWORD || name == TIMES_KEYWORD_KO)
                    && depth == 0
                    && index > 0
                    && matches!(
                        tokens.get(index + 1).map(|next| &next.tok),
                        Some(Tok::Colon)
                    ) =>
            {
                return Some((
                    index,
                    if name == TIMES_KEYWORD {
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

fn token_matches_any(token: &Token, expected: &[&str]) -> bool {
    let Some(actual) = token_word(token) else {
        return false;
    };
    expected.iter().any(|candidate| {
        actual == *candidate || (actual.chars().count() >= 2 && one_typo_away(actual, candidate))
    })
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
