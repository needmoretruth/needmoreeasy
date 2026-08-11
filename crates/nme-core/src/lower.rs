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
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, Literal, LogicalOp, NmeLine,
    NmeStmt, TextPart, TextTemplate, UpdateOp, Value, BundledModuleId, FILE_MODULE_VERSION,
    RANDOM_MODULE_VERSION,
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
            };
            format!(
                "{target} = {target} {operator} {}",
                lower_code(amount, source)
            )
        }
        NmeStmt::Times { count, inline } => {
            let header = format!("for _ in range({}):", lower_code(count, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::When { condition, inline } => {
            let header = format!("if ({}):", lower_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::While { condition, inline } => {
            let header = format!("while ({}):", lower_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::ElseIf { condition, inline } => {
            let header = format!("elif ({}):", lower_condition(condition, source));
            lower_suite(header, inline.as_ref(), source)
        }
        NmeStmt::Else { inline } => lower_suite("else:".to_string(), inline.as_ref(), source),
        NmeStmt::Break => "break".to_string(),
        NmeStmt::End => "# end".to_string(),
        NmeStmt::UseModule { module, .. } => match module {
            BundledModuleId::Random => {
                format!("{BILINGUAL_RANDOM_TOOLS_PREFIX}\"{RANDOM_MODULE_VERSION}\"")
            }
            BundledModuleId::File => {
                format!("{BILINGUAL_FILE_TOOLS_PREFIX}\"{FILE_MODULE_VERSION}\"")
            }
        },
        NmeStmt::FileRead { target, path } => {
            format!("{target} = __import__(\"pathlib\").Path({}).read_text()", lower_code(path, source))
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
            };
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
            let values = choices
                .iter()
                .map(|choice| python_string(choice))
                .collect::<Vec<_>>()
                .join(", ");
            format!("__import__(\"random\").choice(({values},))")
        }
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
