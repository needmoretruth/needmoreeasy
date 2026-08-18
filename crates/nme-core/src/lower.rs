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

use crate::syntax::{
    BundledModuleId, Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, Literal,
    LogicalOp, NmeLine, NmeStmt, TextPart, TextTemplate, UpdateOp, Value, ZeroKnowledgeValue,
    FILE_MODULE_VERSION, RANDOM_MODULE_VERSION, ZERO_KNOWLEDGE_MODULE_VERSION,
};

const BILINGUAL_RANDOM_TOOLS_PREFIX: &str = concat!(
    "import random as 랜덤; ",
    "random = 랜덤; ",
    "random_number = 랜덤.randint; ",
    "random_pick = 랜덤.choice; ",
    "shuffle = 랜덤.shuffle; ",
    "랜덤정수 = 랜덤.randint; ",
    "랜덤선택 = 랜덤.choice; ",
    "섞기 = 랜덤.shuffle; ",
    "random_version = 랜덤버전 = ",
);

const BILINGUAL_FILE_TOOLS_PREFIX: &str = concat!(
    "import pathlib as 파일경로; ",
    "file_read = lambda 경로: 파일경로.Path(경로).read_text(); ",
    "file_write = lambda 경로, 내용: 파일경로.Path(경로).write_text(내용); ",
    "json_load = lambda 경로: __import__(\"json\").loads(파일경로.Path(경로).read_text()); ",
    "json_save = lambda 경로, 값: 파일경로.Path(경로).write_text(__import__(\"json\").dumps(값, ensure_ascii=False)); ",
    "파일읽기 = file_read; ",
    "파일쓰기 = file_write; ",
    "json읽기 = json_load; ",
    "json저장 = json_save; ",
    "file_version = 파일버전 = ",
);

const SCHNORR_GROUP15_PRIME: &str = "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B18177B200CBBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFCE0FD108E4B82D120A93AD2CAFFFFFFFFFFFFFFFF";

const BILINGUAL_ZERO_KNOWLEDGE_TOOLS_PREFIX: &str = concat!(
    "import secrets as 영지식비밀난수; ",
    "zk_prime = 영지식큰소수 = 0xFFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B18177B200CBBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFCE0FD108E4B82D120A93AD2CAFFFFFFFFFFFFFFFF; ",
    "zk_order = 영지식부분군크기 = (zk_prime - 1) // 2; ",
    "zk_generator = 영지식생성원 = 2; ",
    "zk_challenge_bits = 영지식도전비트 = 256; ",
    "zk_challenge_limit = 영지식도전범위 = 1 << zk_challenge_bits; ",
    "zk_secret = 영지식비밀만들기 = lambda: 영지식비밀난수.randbelow(zk_order - 1) + 1; ",
    "zk_public = 영지식공개값 = lambda 비밀값: pow(zk_generator, 비밀값, zk_prime); ",
    "zk_nonce = 영지식일회값만들기 = lambda: 영지식비밀난수.randbelow(zk_order); ",
    "zk_commitment = 영지식약속 = lambda 일회값: pow(zk_generator, 일회값, zk_prime); ",
    "zk_challenge = 영지식도전만들기 = lambda: 영지식비밀난수.randbelow(zk_challenge_limit); ",
    "zk_challenge_except = 영지식다른도전 = lambda 제외: ((lambda 후보: 후보 + (1 if 후보 >= 제외 else 0))(영지식비밀난수.randbelow(zk_challenge_limit - 1))); ",
    "zk_response = 영지식응답 = lambda 일회값, 비밀값, 도전값: (일회값 - 비밀값 * 도전값) % zk_order; ",
    "zk_simulated_response = 영지식모의응답만들기 = lambda: 영지식비밀난수.randbelow(zk_order); ",
    "zk_simulated_commitment = 영지식모의약속 = lambda 공개값, 도전값, 응답값: (pow(zk_generator, 응답값, zk_prime) * pow(공개값, 도전값, zk_prime)) % zk_prime; ",
    "zk_verify = 영지식검증 = lambda 공개값, 약속값, 도전값, 응답값: (1 < 공개값 < zk_prime and pow(공개값, zk_order, zk_prime) == 1 and 1 <= 약속값 < zk_prime and pow(약속값, zk_order, zk_prime) == 1 and 0 <= 도전값 < zk_challenge_limit and 0 <= 응답값 < zk_order and 약속값 == (pow(zk_generator, 응답값, zk_prime) * pow(공개값, 도전값, zk_prime)) % zk_prime); ",
    "zk_group_bytes = 영지식그룹바이트 = (zk_prime.bit_length() + 7) // 8; ",
    "_nme_zk_context_bytes = lambda 문맥값: 문맥값 if isinstance(문맥값, bytes) else str(문맥값).encode(\"utf-8\"); ",
    "_nme_zk_int_bytes = lambda 값: int(값).to_bytes(zk_group_bytes, \"big\"); ",
    "_nme_zk_context_frame = lambda 문맥값: (lambda 바이트: len(바이트).to_bytes(8, \"big\") + 바이트)(_nme_zk_context_bytes(문맥값)); ",
    "zk_nizk_challenge = 영지식비대화도전 = lambda 공개값, 약속값, 문맥값: int.from_bytes(__import__(\"hashlib\").sha256(b\"NME-SCHNORR-GROUP15-NIZK-v1\\0\" + _nme_zk_int_bytes(zk_generator) + _nme_zk_int_bytes(약속값) + _nme_zk_int_bytes(공개값) + _nme_zk_context_frame(문맥값)).digest(), \"big\"); ",
    "zk_nizk_prove = 영지식비대화증명 = lambda 비밀값, 문맥값: (lambda 일회값: (lambda 약속값: (lambda 도전값: [약속값, (일회값 - 비밀값 * 도전값) % zk_order])(zk_nizk_challenge(zk_public(비밀값), 약속값, 문맥값)))(zk_commitment(일회값)))(zk_nonce()); ",
    "zk_nizk_verify = 영지식비대화검증 = lambda 공개값, 증명값, 문맥값: (isinstance(증명값, (list, tuple)) and len(증명값) == 2 and zk_verify(공개값, 증명값[0], zk_nizk_challenge(공개값, 증명값[0], 문맥값), 증명값[1])); ",
    "zero_knowledge_version = 영지식버전 = ",
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
            replacement: format!(
                "{}{}",
                indent_prefix(line.virtual_indent),
                lower_stmt(&line.stmt, source)
            ),
        })
        .collect()
}

/// Lowers one NME statement to Python text (without its original indent —
/// the span being replaced never included it, so indentation is preserved
/// automatically).
pub fn lower_stmt(stmt: &NmeStmt, source: &str) -> String {
    match stmt {
        NmeStmt::Say { value } => format!("print({})", lower_value(value, source)),
        NmeStmt::Ask {
            target,
            prompt,
            kind,
        } => {
            let input = match prompt {
                Some(Value::Text(prompt)) => {
                    let ends_with_whitespace = template_ends_with_whitespace(prompt);
                    let lowered = lower_template(prompt);
                    if ends_with_whitespace {
                        format!("input({lowered})")
                    } else {
                        format!("input({lowered} + \" \")")
                    }
                }
                Some(prompt) => format!("input({})", lower_value(prompt, source)),
                None => "input()".to_string(),
            };
            match kind {
                InputKind::Text => format!("{target} = {input}"),
                InputKind::Number => format!("{target} = int({input})"),
            }
        }
        NmeStmt::Set { target, value } => {
            format!("{target} = {}", lower_value(value, source))
        }
        NmeStmt::Update {
            target,
            amount,
            operation,
        } => {
            let operator = match operation {
                UpdateOp::Add => "+",
                UpdateOp::Subtract => "-",
                UpdateOp::Multiply => "*",
                UpdateOp::Divide => "/",
            };
            // `점수에서 1 + 2 빼줘` must mean `score - (1 + 2)`. Without the
            // parentheses Python reads `score - 1 + 2`, which is a different
            // number. Single atoms stay bare so the common case still looks
            // like the arithmetic a learner would write by hand.
            let amount = lower_code(amount, source);
            if is_simple_atom(&amount) {
                format!("{target} = {target} {operator} {amount}")
            } else {
                format!("{target} = {target} {operator} ({amount})")
            }
        }
        NmeStmt::Times { count, inline } => {
            let header = format!("for _ in range({}):", lower_code(count, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::ForEach {
            name,
            items,
            inline,
        } => {
            let header = format!("for {name} in {}:", lower_code(items, source));
            lower_suite(header, inline.as_ref(), source)
        }
        // `time` is imported inline for the same reason the file and random
        // sentence forms import theirs: one NME line must stay one Python
        // line, so there is nowhere to put a separate import statement.
        NmeStmt::Wait { seconds } => format!(
            "__import__(\"time\").sleep({})",
            lower_code(seconds, source)
        ),
        NmeStmt::Append { target, value } => {
            format!("{target}.append({})", lower_value(value, source))
        }
        NmeStmt::When { condition, inline } => {
            let header = format!("if {}:", wrap_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::While { condition, inline } => {
            let header = format!("while {}:", wrap_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::ElseIf { condition, inline } => {
            let header = format!("elif {}:", wrap_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::Else { inline } => lower_suite("else:".to_string(), inline.as_ref(), source),
        NmeStmt::Break => "break".to_string(),
        NmeStmt::Continue => "continue".to_string(),
        NmeStmt::End => "# end".to_string(),
        NmeStmt::UseModule { module, .. } => match module {
            BundledModuleId::Random => {
                format!("{BILINGUAL_RANDOM_TOOLS_PREFIX}\"{RANDOM_MODULE_VERSION}\"")
            }
            BundledModuleId::File => {
                format!("{BILINGUAL_FILE_TOOLS_PREFIX}\"{FILE_MODULE_VERSION}\"")
            }
            BundledModuleId::ZeroKnowledge => {
                format!(
                    "{BILINGUAL_ZERO_KNOWLEDGE_TOOLS_PREFIX}\"{ZERO_KNOWLEDGE_MODULE_VERSION}\""
                )
            }
        },
        NmeStmt::FileRead { target, path } => {
            format!(
                "{target} = __import__(\"pathlib\").Path({}).read_text()",
                lower_code(path, source)
            )
        }
        NmeStmt::FileWrite { path, value } => format!(
            "__import__(\"pathlib\").Path({}).write_text({})",
            lower_code(path, source),
            lower_value(value, source)
        ),
        NmeStmt::ModuleImport { path, names } => {
            let path_text = lower_code(path, source);
            let stripped = path_text.trim_matches(['\'', '"']);
            let stem = stripped
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or(stripped)
                .strip_suffix(".nme")
                .unwrap_or(stripped);
            format!("from {stem} import {}", names.join(", "))
        }
    }
}

/// Puts a condition in the parentheses its header needs, without adding a
/// second pair around a condition that already carries its own.
fn wrap_condition(condition: &Condition, source: &str) -> String {
    let lowered = lower_condition(condition, source);
    if is_wholly_parenthesized(&lowered) {
        lowered
    } else {
        format!("({lowered})")
    }
}

fn is_wholly_parenthesized(text: &str) -> bool {
    if !(text.starts_with('(') && text.ends_with(')')) {
        return false;
    }
    let mut depth = 0_i32;
    for (index, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                // The opening parenthesis closed before the end, so the outer
                // pair does not enclose the whole expression.
                if depth == 0 && index + 1 != text.len() {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

/// True for an expression that cannot change meaning when an operator is put
/// next to it: a name, a number, or a dotted name.
fn is_simple_atom(text: &str) -> bool {
    let text = text.trim();
    !text.is_empty()
        && text
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_' || character == '.')
}

fn lower_code(code: &Code, source: &str) -> String {
    match code {
        Code::Source(span) => slice(source, *span).to_string(),
    }
}

fn lower_condition(condition: &Condition, source: &str) -> String {
    match condition {
        Condition::Python(code) => lower_code(code, source),
        Condition::Truthy { value, negated } => {
            let value = lower_condition_value(value, source);
            if *negated {
                format!("not ({value})")
            } else {
                value
            }
        }
        Condition::Compare {
            left,
            operator,
            right,
            negated,
        } => {
            let left = lower_condition_value(left, source);
            let right = lower_condition_value(right, source);
            let operator = match operator {
                CompareOp::Equal => "==",
                CompareOp::Greater => ">",
                CompareOp::Less => "<",
                CompareOp::LessOrEqual => "<=",
                CompareOp::GreaterOrEqual => ">=",
                CompareOp::Contains => "in",
            };
            // NME puts the container first (`names contains Mina`); Python
            // puts the member first.
            if operator == "in" {
                return if *negated {
                    format!("{right} not in {left}")
                } else {
                    format!("{right} in {left}")
                };
            }
            // A negated equality is `!=`, which is the operator the reference
            // promises and the one a reader of the Python will recognize.
            if *negated && operator == "==" {
                return format!("{left} != {right}");
            }
            let comparison = format!("{left} {operator} {right}");
            if *negated {
                format!("not ({comparison})")
            } else {
                comparison
            }
        }
        Condition::Logical {
            left,
            operator,
            right,
        } => {
            let operator = match operator {
                LogicalOp::And => "and",
                LogicalOp::Or => "or",
            };
            format!(
                "({} {} {})",
                lower_condition(left, source),
                operator,
                lower_condition(right, source)
            )
        }
    }
}

fn lower_condition_value(value: &ConditionValue, source: &str) -> String {
    match value {
        ConditionValue::Python(code) => lower_code(code, source),
        ConditionValue::Name(name) => name.clone(),
        ConditionValue::Text(text) => python_string(text),
        ConditionValue::Literal(literal) => lower_literal(*literal).to_string(),
    }
}

fn lower_value(value: &Value, source: &str) -> String {
    match value {
        Value::Python(code) => lower_code(code, source),
        Value::Text(template) => lower_template(template),
        Value::Literal(literal) => lower_literal(*literal).to_string(),
        Value::RandomInteger { low, high } => format!(
            "__import__(\"random\").randint({}, {})",
            lower_code(low, source),
            lower_code(high, source)
        ),
        Value::RandomChoice { choices } => {
            // A choice written as a number stays a number, so the result can
            // be compared with other numbers.
            let values = choices
                .iter()
                .map(|choice| {
                    if choice.parse::<i128>().is_ok() || choice.parse::<f64>().is_ok() {
                        choice.clone()
                    } else {
                        python_string(choice)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("__import__(\"random\").choice(({values},))")
        }
        Value::List(items) => {
            let values = items
                .iter()
                .map(|item| lower_value(item, source))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{values}]")
        }
        Value::ZeroKnowledge(value) => lower_zero_knowledge(value, source),
    }
}

fn lower_zero_knowledge(value: &ZeroKnowledgeValue, source: &str) -> String {
    let p = format!("0x{SCHNORR_GROUP15_PRIME}");
    let q = format!("(({p} - 1) // 2)");
    let challenge_limit = "(1 << 256)";
    match value {
        ZeroKnowledgeValue::Secret => format!(
            "__import__(\"secrets\").randbelow(({q}) - 1) + 1"
        ),
        ZeroKnowledgeValue::Public { secret } => format!(
            "pow(2, {}, {p})",
            lower_code(secret, source)
        ),
        ZeroKnowledgeValue::NizkChallenge {
            public_key,
            commitment,
            context,
        } => format!(
            "zk_nizk_challenge({}, {}, {})",
            lower_code(public_key, source),
            lower_code(commitment, source),
            lower_code(context, source)
        ),
        ZeroKnowledgeValue::NizkProof { secret, context } => format!(
            "zk_nizk_prove({}, {})",
            lower_code(secret, source),
            lower_code(context, source)
        ),
        ZeroKnowledgeValue::NizkVerify {
            public_key,
            proof,
            context,
        } => format!(
            "zk_nizk_verify({}, {}, {})",
            lower_code(public_key, source),
            lower_code(proof, source),
            lower_code(context, source)
        ),
        ZeroKnowledgeValue::Nonce | ZeroKnowledgeValue::SimulatedResponse => format!(
            "__import__(\"secrets\").randbelow({q})"
        ),
        ZeroKnowledgeValue::Commitment { nonce } => format!(
            "pow(2, {}, {p})",
            lower_code(nonce, source)
        ),
        ZeroKnowledgeValue::Challenge => format!(
            "__import__(\"secrets\").randbelow({challenge_limit})"
        ),
        ZeroKnowledgeValue::ChallengeExcept { excluded } => format!(
            "((lambda 후보: 후보 + (1 if 후보 >= ({}) else 0))(__import__(\"secrets\").randbelow({challenge_limit} - 1)))",
            lower_code(excluded, source)
        ),
        ZeroKnowledgeValue::Response {
            nonce,
            secret,
            challenge,
        } => format!(
            "(({}) - ({}) * ({})) % {q}",
            lower_code(nonce, source),
            lower_code(secret, source),
            lower_code(challenge, source)
        ),
        ZeroKnowledgeValue::Verify {
            public_key,
            commitment,
            challenge,
            response,
        } => {
            let public_key = lower_code(public_key, source);
            let commitment = lower_code(commitment, source);
            let challenge = lower_code(challenge, source);
            let response = lower_code(response, source);
            format!(
                "(1 < ({public_key}) < {p} and pow(({public_key}), {q}, {p}) == 1 and 1 <= ({commitment}) < {p} and pow(({commitment}), {q}, {p}) == 1 and 0 <= ({challenge}) < {challenge_limit} and 0 <= ({response}) < {q} and ({commitment}) == (pow(2, ({response}), {p}) * pow(({public_key}), ({challenge}), {p})) % {p})"
            )
        }
        ZeroKnowledgeValue::SimulatedCommitment {
            public_key,
            challenge,
            response,
        } => format!(
            "(pow(2, ({}), {p}) * pow(({}), ({}), {p})) % {p}",
            lower_code(response, source),
            lower_code(public_key, source),
            lower_code(challenge, source)
        ),
    }
}

fn lower_literal(literal: Literal) -> &'static str {
    match literal {
        Literal::True => "True",
        Literal::False => "False",
        Literal::None => "None",
    }
}

fn lower_template(template: &TextTemplate) -> String {
    let pieces: Vec<String> = template
        .parts
        .iter()
        .map(|part| match part {
            TextPart::Literal(text) => python_string(text),
            TextPart::Variable(name) => format!("str({name})"),
        })
        .collect();
    if pieces.is_empty() {
        "\"\"".to_string()
    } else {
        pieces.join(" + ")
    }
}

fn template_ends_with_whitespace(template: &TextTemplate) -> bool {
    matches!(
        template.parts.last(),
        Some(TextPart::Literal(text))
            if text.chars().last().is_some_and(char::is_whitespace)
    )
}

fn python_string(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push('"');
    for character in text.chars() {
        match character {
            '\\' => quoted.push_str("\\\\"),
            '"' => quoted.push_str("\\\""),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

fn lower_suite(header: String, inline: Option<&InlineStmt>, source: &str) -> String {
    match inline {
        None => header,
        Some(InlineStmt::Nme(inner)) => format!("{header} {}", lower_stmt(inner, source)),
        Some(InlineStmt::Python(span)) => format!("{header} {}", slice(source, *span)),
    }
}

fn indent_prefix(level: usize) -> String {
    "    ".repeat(level)
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
            count: Code::Source(Span::new(0, 1)),
            inline: Some(InlineStmt::Nme(Box::new(NmeStmt::Say {
                value: Value::Python(Code::Source(Span::new(13, 17))),
            }))),
        };
        assert_eq!(
            lower_stmt(&stmt, source),
            "for _ in range(5): print(\"hi\")"
        );
    }
}
