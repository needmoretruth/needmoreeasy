//! Reads a line of Python back into the statement that would have written it.
//!
//! The tidier can write any statement NME has, in either language and at any
//! level, because [`crate::render`] takes its words from the syntax reference.
//! What it could not do was *arrive* at a statement: a program written in
//! ordinary Python — NME's advanced level — stayed Python, so
//! `advanced → sentence` moved almost nothing. This module is the way in.
//!
//! Nothing here is trusted. Every reading it proposes is lowered again with
//! [`lower_stmt`] and kept only when the Python that comes back is the line
//! that went in, character for character. A reading that is nearly right is
//! thrown away rather than shipped, so the promise the tidier makes — the
//! program still means what it meant — holds by construction. That is also
//! why the readings may be generous: a wrong guess cannot survive the check.

use crate::lower::{lower_condition, lower_stmt, lower_value};
use crate::syntax::{
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, ItemPosition, ListOrder,
    Literal, LogicalOp, NmeStmt, Reading, SplitBy, TextPart, TextTemplate, UpdateOp, Value,
    COOLDOWN_PREFIX, ELAPSED_PYTHON, TIMER_NAME,
};

/// The statement whose Python is exactly `text`, if NME has one.
///
/// `text` is one logical line with its indentation already taken off.
pub(crate) fn statement_from_python(text: &str) -> Option<NmeStmt> {
    let text = text.trim();
    if text.is_empty() || text.starts_with('#') {
        return None;
    }
    statements(text)
        .into_iter()
        .find(|stmt| lower_stmt(stmt, "") == text)
}

/// The value this piece of Python is really naming, when it names one.
///
/// A beginner writes `say len(friends)`, and the sentence for that line is
/// `show how many friends`. The parser keeps the expression as Python because
/// that is what was written, so the words are found here instead — and, as
/// everywhere in this module, only when lowering the reading again gives back
/// the very expression that was read.
pub(crate) fn value_from_python(text: &str) -> Option<Value> {
    let text = text.trim();
    values(text)
        .into_iter()
        .find(|value| !matches!(value, Value::Python(_)) && lower_value(value, "") == text)
}

/// The condition this piece of Python is really asking, when it asks one.
pub(crate) fn condition_from_python(text: &str) -> Option<Condition> {
    let text = text.trim();
    conditions(text).into_iter().find(|condition| {
        // A condition that is only Python wrapped in a word is no richer than
        // the Python it came from, and reading `not ready` as a value that
        // stands on its own would have the sentence say `not ready exists`.
        !matches!(
            condition,
            Condition::Python(_)
                | Condition::Truthy {
                    value: ConditionValue::Python(_),
                    ..
                }
        ) && lower_condition(condition, "") == text
    })
}

/// Every statement worth asking about for this line, most telling first.
///
/// Order only decides which of two readings that both lower to the same Python
/// wins, so it is about the nicer sentence rather than about correctness.
#[allow(clippy::too_many_lines)]
fn statements(text: &str) -> Vec<NmeStmt> {
    let mut found = Vec::new();
    match text {
        "break" => found.push(NmeStmt::Break),
        "continue" => found.push(NmeStmt::Continue),
        "# end" => found.push(NmeStmt::End),
        _ => {}
    }
    // A block header, with or without its body written after the colon.
    if let Some((header, body)) = suite(text) {
        let inline = match body {
            None => Some(None),
            Some(body) => statement_from_python(body).map(|stmt| Some(Box::new(stmt))),
        };
        if let Some(inline) = inline {
            let inline = inline.map(InlineStmt::Nme);
            found.extend(block_statements(header, inline.as_ref()));
        }
    }
    // `x = ...` in all its shapes.
    if let Some((target, value)) = assignment(text) {
        if is_name(target) {
            found.extend(assignment_statements(target, value));
        }
        if let Some((of, key)) = subscript(target) {
            for key in values(key) {
                for value in values(value) {
                    found.push(NmeStmt::RecordPut {
                        target: of.to_string(),
                        key: key.clone(),
                        value,
                    });
                }
            }
        }
    }
    if let Some(rest) = text.strip_prefix("del ") {
        if let Some((of, key)) = subscript(rest.trim()) {
            for key in values(key) {
                found.push(NmeStmt::RecordRemove {
                    target: of.to_string(),
                    key,
                });
            }
        }
    }
    // A call on a name the program made: `friends.append(...)` and friends.
    if let Some((target, method, argument)) = method_call(text) {
        match (method, argument) {
            ("append", Some(argument)) => found.extend(values(argument).into_iter().map(|value| {
                NmeStmt::Append {
                    target: target.to_string(),
                    value,
                }
            })),
            ("remove", Some(argument)) => found.extend(values(argument).into_iter().map(|value| {
                NmeStmt::Remove {
                    target: target.to_string(),
                    value,
                }
            })),
            ("sort", Some("")) => found.push(NmeStmt::Arrange {
                target: target.to_string(),
                order: ListOrder::Sorted,
            }),
            ("reverse", Some("")) => found.push(NmeStmt::Arrange {
                target: target.to_string(),
                order: ListOrder::Reversed,
            }),
            _ => {}
        }
    }
    found.extend(screen_statements(text));
    found.extend(waiting_statements(text));
    // `print(...)`, which is every way NME has of saying something.
    if let Some(inside) = call_argument(text, "print") {
        found.extend(values(inside).into_iter().map(|value| NmeStmt::Say { value }));
    }
    // A job being run: `greet()`, `greet("Mina")`.
    if let Some((name, arguments)) = plain_call(text) {
        if is_name(name) {
            let arguments = arguments
                .iter()
                .map(|argument| first_value(argument))
                .collect::<Vec<_>>();
            found.push(NmeStmt::RunJob {
                name: name.to_string(),
                arguments,
            });
        }
    }
    found
}

/// The statements a `header:` line can be.
fn block_statements(header: &str, inline: Option<&InlineStmt>) -> Vec<NmeStmt> {
    let mut found = Vec::new();
    let inline = inline.cloned();
    if header == "while True" {
        found.push(NmeStmt::Forever {
            inline: inline.clone(),
        });
    }
    if header == "else" {
        found.push(NmeStmt::Else {
            inline: inline.clone(),
        });
    }
    if let Some(rest) = header.strip_prefix("def ") {
        if let Some((name, parameters)) = plain_call(rest.trim()) {
            if is_name(name) && parameters.iter().all(|p| is_name(p.trim())) {
                found.push(NmeStmt::Job {
                    name: name.to_string(),
                    parameters: parameters.iter().map(|p| p.trim().to_string()).collect(),
                });
            }
        }
    }
    if let Some(rest) = header.strip_prefix("for ") {
        if let Some((names, items)) = rest.split_once(" in ") {
            let items = items.trim();
            if names.trim() == "_" {
                // `range(start, stop)` counts from somewhere other than zero,
                // and `repeat start, stop times` is not a sentence anybody
                // means. One count, or the line stays Python.
                if let Some(count) = call_argument(items, "range") {
                    // `times = 2` then `repeat times times` is a line nobody
                    // can read, so a count that is spelled like the repeat
                    // marker itself keeps its Python.
                    let marker = matches!(count.trim(), "times" | "time" | "번" | "회");
                    if position_outside(count, ",").is_none() && !marker {
                        found.push(NmeStmt::Times {
                            count: code(count),
                            inline: inline.clone(),
                        });
                    }
                }
            }
            // `_` is Python's way of saying the loop never looks at what it
            // is holding, and `for each _ in ...` is not a sentence.
            if is_name(names.trim()) && names.trim() != "_" {
                found.push(NmeStmt::ForEach {
                    name: names.trim().to_string(),
                    items: code(items),
                    position: None,
                    inline: inline.clone(),
                });
            }
            if let Some((position, name)) = names.split_once(',') {
                if is_name(position.trim()) && is_name(name.trim()) {
                    if let Some(arguments) = call_argument(items, "enumerate") {
                        if let Some((list, start)) = arguments.rsplit_once(',') {
                            if start.trim() == "1" {
                                found.push(NmeStmt::ForEach {
                                    name: name.trim().to_string(),
                                    items: code(list.trim()),
                                    position: Some(position.trim().to_string()),
                                    inline: inline.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    for (word, build) in [
        (
            "if ",
            &(|condition, inline| NmeStmt::When { condition, inline })
                as &dyn Fn(Condition, Option<InlineStmt>) -> NmeStmt,
        ),
        ("elif ", &|condition, inline| NmeStmt::ElseIf {
            condition,
            inline,
        }),
        ("while ", &|condition, inline| NmeStmt::While {
            condition,
            inline,
        }),
    ] {
        let Some(rest) = header.strip_prefix(word) else {
            continue;
        };
        if word == "if " {
            if let Some(permille) = chance_permille(rest.trim()) {
                found.push(NmeStmt::Chance {
                    permille,
                    inline: inline.clone(),
                });
            }
        }
        for condition in conditions(rest.trim()) {
            found.push(build(condition, inline.clone()));
        }
    }
    found
}

/// The statements `name = value` can be.
fn assignment_statements(target: &str, value: &str) -> Vec<NmeStmt> {
    let mut found = Vec::new();
    if target == TIMER_NAME {
        found.push(NmeStmt::StartTimer);
    }
    if let Some(name) = target.strip_prefix(COOLDOWN_PREFIX) {
        if let Some((_, seconds)) = value.rsplit_once(" + ") {
            found.push(NmeStmt::Cooldown {
                target: name.to_string(),
                seconds: code(seconds.trim()),
            });
        }
    }
    // `age = int(input("Age? "))` and `name = input("Name? ")`.
    for (kind, inner) in [
        (
            InputKind::Number,
            call_argument(value, "int").and_then(|inner| call_argument(inner.trim(), "input")),
        ),
        (InputKind::Text, call_argument(value, "input")),
    ] {
        let Some(inner) = inner else { continue };
        let inner = inner.trim();
        if inner.is_empty() {
            found.push(NmeStmt::Ask {
                target: target.to_string(),
                prompt: None,
                kind,
            });
            continue;
        }
        // The sentence form adds the space after the question, so the Python
        // it writes ends in `+ " "`; a prompt without that was written at the
        // beginner level, where the writer chose the spacing.
        // A question spread out of a list, or written with a comma left
        // hanging, is not one a sentence can carry.
        if inner.starts_with('*') || inner.ends_with(',') {
            continue;
        }
        let (asked, spaced) = match inner.strip_suffix(" + \" \"") {
            Some(asked) => (asked.trim(), true),
            None => (inner, false),
        };
        for prompt in values(asked) {
            if spaced && !matches!(prompt, Value::Text(_)) {
                continue;
            }
            found.push(NmeStmt::Ask {
                target: target.to_string(),
                prompt: Some(prompt),
                kind,
            });
        }
    }
    // `score = score + 1` is one sentence, not two.
    for (operator, operation) in [
        (" + ", UpdateOp::Add),
        (" - ", UpdateOp::Subtract),
        (" * ", UpdateOp::Multiply),
        (" / ", UpdateOp::Divide),
    ] {
        if let Some(rest) = value.strip_prefix(target) {
            if let Some(amount) = rest.strip_prefix(operator) {
                let amount = amount.trim();
                let bare = amount
                    .strip_prefix('(')
                    .and_then(|inner| inner.strip_suffix(')'))
                    .unwrap_or(amount);
                found.push(NmeStmt::Update {
                    target: target.to_string(),
                    amount: code(bare),
                    operation,
                });
            }
        }
    }
    found.extend(values(value).into_iter().map(|value| NmeStmt::Set {
        target: target.to_string(),
        value,
    }));
    found
}

/// The three statements that draw on the screen, and the slow one.
fn screen_statements(text: &str) -> Vec<NmeStmt> {
    let mut found = Vec::new();
    found.push(NmeStmt::ClearScreen);
    found.push(NmeStmt::DrawLine);
    for build in [
        &(|value| NmeStmt::SayInBox { value }) as &dyn Fn(Value) -> NmeStmt,
        &|value| NmeStmt::SayInMiddle { value },
    ] {
        for inner in printable_inside(text, build) {
            found.push(build(inner));
        }
    }
    // `say slowly Hello` — the pause is the one number in the wrapper.
    if let Some(rest) = text.strip_prefix("[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep(") {
        if let Some((seconds, rest)) = rest.split_once(") for _ch in ") {
            if let Some(printed) = rest.strip_suffix("]; print()") {
                for value in printable_values(printed) {
                    found.push(NmeStmt::SaySlowly {
                        value,
                        seconds: code(seconds),
                    });
                }
            }
        }
    }
    found
}

/// Waiting, and waiting out a cooldown.
fn waiting_statements(text: &str) -> Vec<NmeStmt> {
    let mut found = Vec::new();
    let Some(inside) = call_argument(text, "__import__(\"time\").sleep") else {
        return found;
    };
    let inside = inside.trim();
    // The cooldown wait is asked about first: it lowers to a `sleep` of the
    // time that is left, so a plain wait reads it too, and the sentence the
    // writer meant is the one that names the cooldown.
    if let Some(inner) = call_argument(inside, "max") {
        if let Some((_, rest)) = inner.split_once(", ") {
            if let Some((name, _)) = rest.split_once(" - ") {
                if let Some(target) = name.trim().strip_prefix(COOLDOWN_PREFIX) {
                    found.push(NmeStmt::WaitForCooldown {
                        target: target.to_string(),
                    });
                }
            }
        }
    }
    found.push(NmeStmt::Wait {
        seconds: code(inside),
    });
    found
}

// ---------------------------------------------------------------- values

/// The first reading of an expression, for places that take exactly one.
fn first_value(text: &str) -> Value {
    values(text)
        .into_iter()
        .next()
        .unwrap_or_else(|| Value::Python(code(text)))
}

/// Every value an expression could be, most telling first.
#[allow(clippy::too_many_lines)]
fn values(text: &str) -> Vec<Value> {
    let text = text.trim();
    let mut found = Vec::new();
    if text.is_empty() {
        return found;
    }
    if text == ELAPSED_PYTHON {
        found.push(Value::Elapsed);
    }
    if let Some(permille) = chance_permille(text) {
        found.push(Value::Chance { permille });
    }
    for (word, reading) in [
        ("len", Reading::Count),
        ("sum", Reading::Total),
        ("max", Reading::Largest),
        ("min", Reading::Smallest),
    ] {
        if let Some(of) = call_argument(text, word) {
            if is_name(of.trim()) {
                found.push(Value::Reading {
                    of: of.trim().to_string(),
                    reading,
                });
            }
        }
    }
    // `int(answer)` — text read back as a number. `int(input(...))` is the
    // question that asks for one, and its argument is a call rather than a
    // name, so the two never meet.
    if let Some(of) = call_argument(text, "int") {
        if is_name(of.trim()) {
            found.push(Value::AsNumber {
                of: of.trim().to_string(),
            });
        }
    }
    // `str(name).upper()`, and the other things done to a piece of text.
    if let Some((wrapped, method, argument)) = method_call_expression(text) {
        if let Some(of) = call_argument(wrapped, "str") {
            let of = of.trim();
            if is_name(of) {
                match (method, argument) {
                    ("upper", Some("")) => found.push(Value::Reading {
                        of: of.to_string(),
                        reading: Reading::Capitals,
                    }),
                    ("lower", Some("")) => found.push(Value::Reading {
                        of: of.to_string(),
                        reading: Reading::SmallLetters,
                    }),
                    ("splitlines", Some("")) => found.push(Value::Split {
                        of: of.to_string(),
                        by: SplitBy::Lines,
                    }),
                    ("split", Some(argument)) => {
                        if let Some(separator) = python_text(argument) {
                            found.push(Value::Split {
                                of: of.to_string(),
                                by: SplitBy::Text(separator),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        // `", ".join(map(str, friends))`.
        if method == "join" {
            if let (Some(separator), Some(argument)) = (python_text(wrapped), argument) {
                if let Some(inner) = call_argument(argument.trim(), "map") {
                    if let Some((first, of)) = inner.split_once(',') {
                        if first.trim() == "str" && is_name(of.trim()) {
                            found.push(Value::Joined {
                                of: of.trim().to_string(),
                                separator,
                            });
                        }
                    }
                }
            }
        }
    }
    // `friends[0]`, `friends[-1]`, `friends[2]`, `prices["Mina"]`.
    if let Some((of, index)) = subscript(text) {
        if is_name(of) {
            let index = index.trim();
            match index {
                "0" => found.push(Value::Item {
                    of: of.to_string(),
                    position: ItemPosition::First,
                }),
                "-1" => found.push(Value::Item {
                    of: of.to_string(),
                    position: ItemPosition::Last,
                }),
                _ => {}
            }
            if let Ok(number) = index.parse::<i64>() {
                if number >= 0 {
                    found.push(Value::Item {
                        of: of.to_string(),
                        position: ItemPosition::Numbered(Code::Generated(
                            (number + 1).to_string(),
                        )),
                    });
                }
            }
            for key in values(index) {
                if matches!(key, Value::Text(_) | Value::Python(_)) {
                    found.push(Value::Entry {
                        of: of.to_string(),
                        key: Box::new(key),
                    });
                }
            }
        }
    }
    // `str(greeting) * 3` and `score % 4`.
    for (operator, build) in [
        (
            " * ",
            &(|of: &str, by: &str| {
                call_argument(of, "str").filter(|inner| is_name(inner.trim())).map(|inner| {
                    Value::Repeated {
                        of: inner.trim().to_string(),
                        times: code(by),
                    }
                })
            }) as &dyn Fn(&str, &str) -> Option<Value>,
        ),
        (
            " % ",
            &|of: &str, by: &str| {
                is_name(of).then(|| Value::Remainder {
                    of: of.to_string(),
                    by: code(by.trim_matches(['(', ')'])),
                })
            },
        ),
        (
            " // ",
            &|of: &str, by: &str| {
                is_name(of).then(|| Value::Quotient {
                    of: of.to_string(),
                    by: code(by.trim_matches(['(', ')'])),
                })
            },
        ),
    ] {
        if let Some((left, right)) = split_once_outside(text, operator) {
            if let Some(value) = build(left.trim(), right.trim()) {
                found.push(value);
            }
        }
    }
    if let Some(arguments) = call_argument(text, "__import__(\"random\").randint") {
        if let Some((low, high)) = split_once_outside(arguments, ",") {
            found.push(Value::RandomInteger {
                low: code(low.trim()),
                high: code(high.trim()),
            });
        }
    }
    if let Some(arguments) = call_argument(text, "__import__(\"random\").choice") {
        let inner = arguments.trim();
        if let Some(inner) = inner.strip_prefix('(').and_then(|i| i.strip_suffix(')')) {
            let choices = split_outside(inner.trim_end().trim_end_matches(','), ",")
                .into_iter()
                .map(|piece| {
                    let piece = piece.trim();
                    python_text(piece).unwrap_or_else(|| piece.to_string())
                })
                .collect::<Vec<_>>();
            if !choices.is_empty() {
                found.push(Value::RandomChoice { choices });
            }
        }
    }
    // A list written out, and the empty one.
    if let Some(inner) = text.strip_prefix('[').and_then(|i| i.strip_suffix(']')) {
        let inner = inner.trim();
        if inner.is_empty() {
            found.push(Value::List(Vec::new()));
        } else {
            let items = split_outside(inner, ",")
                .into_iter()
                .map(first_value)
                .collect::<Vec<_>>();
            found.push(Value::List(items));
        }
    }
    if text == "{}" {
        found.push(Value::EmptyRecord);
    }
    match text {
        "True" => found.push(Value::Literal(Literal::True)),
        "False" => found.push(Value::Literal(Literal::False)),
        "None" => found.push(Value::Literal(Literal::None)),
        _ => {}
    }
    if let Some(template) = text_template(text) {
        found.push(Value::Text(template));
    }
    found.push(Value::Python(code(text)));
    found
}

/// A piece of Python that a sentence would have written as words.
///
/// `"You carry " + str(len(friends)) + " things"` is one sentence with a
/// reading in the middle of it, and writing it back as Python would be a
/// worse answer than the sentence the writer could have typed.
fn text_template(text: &str) -> Option<TextTemplate> {
    let pieces = split_outside(text, " + ");
    let mut parts = Vec::new();
    let mut saw_words = false;
    for piece in pieces {
        let piece = piece.trim();
        if let Some(words) = python_text(piece) {
            saw_words = true;
            parts.push(TextPart::Literal(words));
            continue;
        }
        let inner = call_argument(piece, "str")?;
        let inner = inner.trim();
        if is_name(inner) {
            parts.push(TextPart::Variable(inner.to_string()));
            continue;
        }
        let reading = values(inner).into_iter().find_map(|value| match value {
            Value::Reading { of, reading } => Some(TextPart::Reading {
                of,
                reading,
                written: String::new(),
            }),
            _ => None,
        })?;
        parts.push(reading);
    }
    (saw_words && !parts.is_empty()).then_some(TextTemplate { parts })
}

/// The values that could stand inside a statement that prints text.
fn printable_values(text: &str) -> Vec<Value> {
    match call_argument(text, "str") {
        Some(inner) => values(inner)
            .into_iter()
            .filter(|value| !matches!(value, Value::Text(_)))
            .collect(),
        None => values(text)
            .into_iter()
            .filter(|value| matches!(value, Value::Text(_)))
            .collect(),
    }
}

/// What a screen statement was given, taken from the lowering itself.
///
/// The wrapper is worked out by lowering the statement once around a word
/// nothing else can hold, so this file never keeps its own copy of a template
/// that lives in `lower.rs`.
fn printable_inside(text: &str, build: &dyn Fn(Value) -> NmeStmt) -> Vec<Value> {
    const MARK: &str = "\u{1}nme\u{1}";
    let lowered = lower_stmt(&build(Value::Python(Code::Generated(MARK.to_string()))), "");
    let Some((before, after)) = lowered.split_once(MARK) else {
        return Vec::new();
    };
    // The wrapper for a value carries `str(` around it; text does not need it.
    let (before_text, after_text) = (
        before.strip_suffix("str(").unwrap_or(before),
        after.strip_prefix(')').unwrap_or(after),
    );
    let mut found = Vec::new();
    for (before, after, want_text) in [(before, after, false), (before_text, after_text, true)] {
        let Some(inside) = text.strip_prefix(before).and_then(|rest| rest.strip_suffix(after))
        else {
            continue;
        };
        found.extend(
            values(inside)
                .into_iter()
                .filter(|value| matches!(value, Value::Text(_)) == want_text),
        );
    }
    found
}

// ---------------------------------------------------------------- conditions

/// Every condition a Python header could be carrying.
fn conditions(text: &str) -> Vec<Condition> {
    let text = text.trim();
    let mut found = Vec::new();
    // The header wraps its condition in parentheses unless it already had a
    // pair of its own, so both readings are worth asking about.
    let bare = text
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .filter(|inner| balanced(inner));
    // The bare form is asked first, because a whole comparison in brackets
    // reads as a value standing on its own as well, and `if score > 10` is
    // the sentence the writer meant.
    for text in [bare, Some(text)].into_iter().flatten() {
        found.extend(one_condition(text));
    }
    found
}

fn one_condition(text: &str) -> Vec<Condition> {
    let text = text.trim();
    let mut found = Vec::new();
    for (word, operator) in [(" and ", LogicalOp::And), (" or ", LogicalOp::Or)] {
        let Some(inner) = text
            .strip_prefix('(')
            .and_then(|inner| inner.strip_suffix(')'))
            .filter(|inner| balanced(inner))
        else {
            continue;
        };
        if let Some((left, right)) = split_once_outside(inner, word) {
            for left in conditions(left) {
                for right in conditions(right) {
                    found.push(Condition::Logical {
                        left: Box::new(left.clone()),
                        operator,
                        right: Box::new(right),
                    });
                }
            }
        }
    }
    // `not (...)` is how a negated reading and a negated comparison both come
    // out, so the inside is asked both questions.
    if let Some(inner) = call_argument(text, "not") {
        for condition in conditions(inner) {
            match condition {
                Condition::Truthy { value, negated: false } => {
                    found.push(Condition::Truthy {
                        value,
                        negated: true,
                    });
                }
                Condition::Compare {
                    left,
                    operator,
                    right,
                    negated: false,
                } => found.push(Condition::Compare {
                    left,
                    operator,
                    right,
                    negated: true,
                }),
                _ => {}
            }
        }
    }
    // `Mina in friends` puts the member first; the sentence puts the list
    // first, so the two sides change places here.
    for (word, negated) in [(" not in ", true), (" in ", false)] {
        if let Some((member, container)) = split_once_outside(text, word) {
            for left in condition_values(container) {
                for right in condition_values(member) {
                    found.push(Condition::Compare {
                        left: left.clone(),
                        operator: CompareOp::Contains,
                        right,
                        negated,
                    });
                }
            }
        }
    }
    for (word, operator, negated) in [
        (" == ", CompareOp::Equal, false),
        (" != ", CompareOp::Equal, true),
        (" >= ", CompareOp::GreaterOrEqual, false),
        (" <= ", CompareOp::LessOrEqual, false),
        (" > ", CompareOp::Greater, false),
        (" < ", CompareOp::Less, false),
    ] {
        let Some((left, right)) = split_once_outside(text, word) else {
            continue;
        };
        for left in condition_values(left) {
            for right in condition_values(right) {
                found.push(Condition::Compare {
                    left: left.clone(),
                    operator,
                    right,
                    negated,
                });
            }
        }
    }
    found.extend(
        condition_values(text)
            .into_iter()
            .map(|value| Condition::Truthy {
                value,
                negated: false,
            }),
    );
    found.push(Condition::Python(code(text)));
    found
}

fn condition_values(text: &str) -> Vec<ConditionValue> {
    let text = text.trim();
    let mut found = Vec::new();
    if is_name(text) {
        found.push(ConditionValue::Name(text.to_string()));
    }
    for value in values(text) {
        match value {
            Value::Text(template) => {
                if let [TextPart::Literal(words)] = template.parts.as_slice() {
                    found.push(ConditionValue::Text(words.clone()));
                }
            }
            Value::Literal(literal) => found.push(ConditionValue::Literal(literal)),
            Value::Reading { of, reading } => found.push(ConditionValue::Reading { of, reading }),
            Value::Remainder { of, by } => found.push(ConditionValue::Remainder { of, by }),
            Value::Quotient { of, by } => found.push(ConditionValue::Quotient { of, by }),
            Value::AsNumber { of } => found.push(ConditionValue::AsNumber { of }),
            Value::Entry { of, key } => found.push(ConditionValue::Entry { of, key }),
            _ => {}
        }
    }
    found.push(ConditionValue::Python(code(text)));
    found
}

/// `__import__("random").randrange(1000) < 300` — the share of the time.
fn chance_permille(text: &str) -> Option<u32> {
    let (call, permille) = split_once_outside(text, " < ")?;
    let inside = call_argument(call.trim(), "__import__(\"random\").randrange")?;
    (inside.trim() == crate::syntax::CHANCE_SCALE.to_string())
        .then(|| permille.trim().parse::<u32>().ok())
        .flatten()
}

// ---------------------------------------------------------------- shapes

fn code(text: &str) -> Code {
    Code::Generated(text.trim().to_string())
}

/// A Python name, and nothing else.
fn is_name(text: &str) -> bool {
    let text = text.trim();
    !text.is_empty()
        && !text.starts_with(|character: char| character.is_ascii_digit())
        && text
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_')
}

/// `header: body` or `header:`, when the colon really closes a header.
fn suite(text: &str) -> Option<(&str, Option<&str>)> {
    let at = position_outside(text, ":")?;
    let header = &text[..at];
    let body = text[at + 1..].trim();
    Some((header, (!body.is_empty()).then_some(body)))
}

/// `name = value`, when the `=` is an assignment and not a comparison.
fn assignment(text: &str) -> Option<(&str, &str)> {
    let at = position_outside(text, " = ")?;
    Some((text[..at].trim(), text[at + 3..].trim()))
}

/// `name[index]`.
fn subscript(text: &str) -> Option<(&str, &str)> {
    let text = text.trim();
    if !text.ends_with(']') {
        return None;
    }
    // The offsets are taken from the whole line, brackets closed and all: a
    // half of an expression does not balance, and nothing outside a bracket
    // can be found in text that never closes one.
    let at = last_position_outside(text, "[")?;
    Some((text[..at].trim(), text[at + 1..text.len() - 1].trim()))
}

/// `word(arguments)` — the arguments, with the brackets taken off.
fn call_argument<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let text = text.trim();
    let rest = text.strip_prefix(word)?;
    let rest = rest.trim_start();
    let inner = rest.strip_prefix('(')?.strip_suffix(')')?;
    balanced(inner).then_some(inner)
}

/// `name(a, b)` — the name and its arguments.
fn plain_call(text: &str) -> Option<(&str, Vec<&str>)> {
    let text = text.trim();
    if !text.ends_with(')') {
        return None;
    }
    let at = last_position_outside(text, "(")?;
    let arguments = text[at + 1..text.len() - 1].trim();
    if !balanced(arguments) {
        return None;
    }
    let text = &text[..=at];
    let arguments = if arguments.is_empty() {
        Vec::new()
    } else {
        split_outside(arguments, ",")
    };
    Some((text[..at].trim(), arguments))
}

/// `name.method(argument)` on a plain name.
fn method_call(text: &str) -> Option<(&str, &str, Option<&str>)> {
    let (target, method, argument) = method_call_expression(text)?;
    is_name(target).then_some((target, method, argument))
}

/// `<anything>.method(argument)`.
fn method_call_expression(text: &str) -> Option<(&str, &str, Option<&str>)> {
    let text = text.trim();
    if !text.ends_with(')') {
        return None;
    }
    let open = last_position_outside(text, "(")?;
    let argument = text[open + 1..text.len() - 1].trim();
    if !balanced(argument) {
        return None;
    }
    let before = &text[..open];
    let dot = before.rfind('.')?;
    let method = &before[dot + 1..];
    if !is_name(method) {
        return None;
    }
    Some((before[..dot].trim(), method, Some(argument)))
}

/// The words a Python string literal holds, or `None` when it is not one.
fn python_text(text: &str) -> Option<String> {
    let text = text.trim();
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut words = String::with_capacity(inner.len());
    let mut characters = inner.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            // A bare quote would have closed the string, so this is not one
            // literal but two with something between them.
            if character == '"' {
                return None;
            }
            words.push(character);
            continue;
        }
        match characters.next()? {
            '\\' => words.push('\\'),
            '"' => words.push('"'),
            'n' => words.push('\n'),
            'r' => words.push('\r'),
            't' => words.push('\t'),
            _ => return None,
        }
    }
    Some(words)
}

/// Whether every bracket and quote in the text closes.
fn balanced(text: &str) -> bool {
    depths(text).is_some()
}

/// The byte offset of `needle` outside every bracket and quote.
fn position_outside(text: &str, needle: &str) -> Option<usize> {
    positions_outside(text, needle).into_iter().next()
}

/// The last offset at which `needle` stands outside brackets and quotes.
///
/// The last one is what a call wants: in `str(name).upper()` the bracket that
/// closes the line is the second one, and reading the first would take the
/// whole of `name).upper(` for an argument.
fn last_position_outside(text: &str, needle: &str) -> Option<usize> {
    positions_outside(text, needle).into_iter().next_back()
}

fn split_once_outside<'a>(text: &'a str, needle: &str) -> Option<(&'a str, &'a str)> {
    let at = position_outside(text, needle)?;
    Some((&text[..at], &text[at + needle.len()..]))
}

fn split_outside<'a>(text: &'a str, needle: &str) -> Vec<&'a str> {
    let mut pieces = Vec::new();
    let mut start = 0;
    for at in positions_outside(text, needle) {
        pieces.push(&text[start..at]);
        start = at + needle.len();
    }
    pieces.push(&text[start..]);
    pieces
}

/// Every offset at which `needle` stands outside brackets and quotes.
fn positions_outside(text: &str, needle: &str) -> Vec<usize> {
    let Some(open) = depths(text) else {
        return Vec::new();
    };
    open.iter()
        .enumerate()
        .filter(|(at, depth)| {
            **depth == 0 && text.is_char_boundary(*at) && text[*at..].starts_with(needle)
        })
        .map(|(at, _)| at)
        .collect()
}

/// How deep in brackets each byte of the text is, or `None` when it does not
/// close. A byte inside a string counts as deep, so a bracket in a message is
/// never read as code.
fn depths(text: &str) -> Option<Vec<i32>> {
    let mut depth = 0_i32;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut answer = Vec::with_capacity(text.len());
    for (at, character) in text.char_indices() {
        while answer.len() < at {
            answer.push(depth.max(1));
        }
        if let Some(mark) = quote {
            answer.push(depth.max(1));
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == mark {
                quote = None;
            }
            continue;
        }
        match character {
            '(' | '[' | '{' => {
                answer.push(depth);
                depth += 1;
            }
            ')' | ']' | '}' => {
                depth -= 1;
                if depth < 0 {
                    return None;
                }
                answer.push(depth);
            }
            '"' | '\'' => {
                quote = Some(character);
                answer.push(depth.max(1));
            }
            _ => answer.push(depth),
        }
    }
    while answer.len() < text.len() {
        answer.push(depth.max(1));
    }
    (depth == 0 && quote.is_none()).then_some(answer)
}
