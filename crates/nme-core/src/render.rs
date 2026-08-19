//! Writes one parsed NME statement back out as NME.
//!
//! This is the half of the tidier that chooses words. Every spelling here is
//! taken from `docs/syntax.md` and `docs/syntax.ko.md`, which are generated
//! from the compiler itself, so the words the tidier writes are words the
//! parser is known to read. Nothing is invented: a shape with no row in those
//! files answers `None`, and [`crate::tidy`] then leaves the line as the
//! writer wrote it.

use crate::convert::{Language, SyntaxLevel};
use crate::syntax::{
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, ItemPosition, ListOrder,
    Literal, LogicalOp, ModuleVersion, NmeStmt, Reading, SplitBy, TextPart, TextTemplate, UpdateOp,
    Value, CHANCE_SCALE, COOLDOWN_PREFIX, ELAPSED_PYTHON,
};

/// One statement, written in one level of one language.
///
/// Every method answers `None` for a shape that language has no documented
/// spelling for. `None` means "leave this line exactly as the writer wrote
/// it", which is always safe and is how the levels stay honest: beginner
/// syntax is a smaller surface than sentence syntax on purpose, and no
/// spelling is invented to fill a gap.
pub(crate) struct Rewrite<'a> {
    pub(crate) source: &'a str,
    pub(crate) level: SyntaxLevel,
    pub(crate) language: Language,
    /// True when the line being written asked for a length rather than a
    /// count. `how many friends` and `the length of name` are one value to
    /// the parser and two sentences to a reader, and only one of them is the
    /// same program: `name 개수` reads as words to print, not as `len(name)`.
    /// The line as written is the only thing that can tell them apart.
    pub(crate) length_not_count: bool,
}

/// The words that ask for a length rather than a count, in both languages,
/// taken from the same rows of `docs/syntax.md` and `docs/syntax.ko.md` the
/// spellings come from.
pub(crate) fn asks_for_a_length(written: &str) -> bool {
    ["length", "size", "길이", "글자수"]
        .iter()
        .any(|word| written.contains(word))
}

impl Rewrite<'_> {
    /// The whole line, or `None` to keep what is there.
    ///
    /// `written` is the statement as it stands in the source, which one case
    /// needs: a line inside a story block is prose that prints itself, and the
    /// parser records it as ordinary output. Putting an output word in front
    /// of it would print that word, so a sentence that is already nothing but
    /// its own text is left alone.
    pub(crate) fn statement(&self, stmt: &NmeStmt, written: &str) -> Option<String> {
        let told_as_written = match stmt {
            NmeStmt::Say {
                value: Value::Text(text),
            }
            | NmeStmt::SaySlowly {
                value: Value::Text(text),
                ..
            } => plain_text(text) == written,
            _ => false,
        };
        if told_as_written {
            return None;
        }
        self.any_statement(stmt)
    }

    /// The statement in the level asked for, falling back to the sentence
    /// spelling wherever beginner syntax has no row of its own.
    fn any_statement(&self, stmt: &NmeStmt) -> Option<String> {
        if self.level == SyntaxLevel::Beginner {
            if let Some(beginner) = self.beginner_statement(stmt) {
                return Some(beginner);
            }
        }
        self.sentence_statement(stmt)
    }

    // ------------------------------------------------------------ beginner

    /// The compact spelling, for the statements `docs/syntax*.md` gives a
    /// beginner row. Everything else answers `None` and is written as a
    /// sentence instead.
    fn beginner_statement(&self, stmt: &NmeStmt) -> Option<String> {
        match stmt {
            NmeStmt::Say { value } => {
                let value = self.python_of(value)?;
                Some(self.either(&format!("say {value}"), &format!("말해 {value}")))
            }
            NmeStmt::Set { target, value } => {
                let value = self.python_of(value)?;
                Some(self.either(
                    &format!("save {target} to {value}"),
                    &format!("저장 {target} {value}"),
                ))
            }
            NmeStmt::Ask {
                target,
                prompt,
                kind: InputKind::Text,
            } => {
                let question = match prompt {
                    None => String::new(),
                    // The beginner form asks exactly what it is given, while
                    // the sentence form adds the space that keeps the answer
                    // off the question mark. Writing that space out here is
                    // what keeps the two spellings the same program.
                    Some(Value::Text(text)) if !template_ends_with_whitespace(text) => {
                        format!(", {} + \" \"", lower_template(text))
                    }
                    Some(prompt) => format!(", {}", self.python_of(prompt)?),
                };
                Some(self.either(
                    &format!("ask {target}{question}"),
                    &format!("물어봐 {target}{question}"),
                ))
            }
            NmeStmt::Times { count, inline } => {
                let count = self.code(count);
                let header = self.either(&format!("{count} times:"), &format!("{count} 번:"));
                self.with_inline(header, " ", inline.as_ref())
            }
            NmeStmt::ForEach {
                name,
                items,
                position,
                inline: None,
            } => {
                let items = self.code(items);
                Some(match (self.language, position) {
                    (Language::English, None) => format!("for each {name} in {items}:"),
                    (Language::English, Some(position)) => {
                        format!("for each {name} in {items} with {position}:")
                    }
                    (Language::Korean, None) => {
                        format!("{}의 {name}마다:", korean_name(items.as_str())?)
                    }
                    (Language::Korean, Some(position)) => format!(
                        "{}의 {name}마다 {position}와 함께:",
                        korean_name(items.as_str())?
                    ),
                })
            }
            NmeStmt::When { condition, inline } => {
                let condition = self.python_condition(condition)?;
                let header =
                    self.either(&format!("when {condition}:"), &format!("만약 {condition}:"));
                self.with_inline(header, " ", inline.as_ref())
            }
            NmeStmt::While { condition, inline } => {
                let condition = self.python_condition(condition)?;
                let header =
                    self.either(&format!("while {condition}"), &format!("동안 {condition}"));
                self.with_inline(header, ": ", inline.as_ref())
            }
            NmeStmt::ModuleImport { path, names } => {
                let path = self.code(path);
                let names = names.join(", ");
                Some(self.either(
                    &format!("from {path} import {names}"),
                    &format!("{path}에서 {names} 가져오기"),
                ))
            }
            _ => None,
        }
    }

    /// The Python text a value lowers to, for the beginner forms that take an
    /// expression. `None` when the value has no expression of its own — a
    /// list, a record, a chance and their like are sentence shapes.
    fn python_of(&self, value: &Value) -> Option<String> {
        match value {
            Value::Python(code) => Some(self.code(code)),
            Value::Text(text) => (!text.parts.is_empty()).then(|| lower_template(text)),
            Value::Literal(literal) => Some(literal_python(*literal).to_string()),
            _ => None,
        }
    }

    /// The Python text a condition lowers to, for the beginner forms that take
    /// an expression.
    fn python_condition(&self, condition: &Condition) -> Option<String> {
        match condition {
            Condition::Python(code) => Some(self.code(code)),
            _ => None,
        }
    }

    // ------------------------------------------------------------ sentence

    #[allow(clippy::too_many_lines)]
    fn sentence_statement(&self, stmt: &NmeStmt) -> Option<String> {
        match stmt {
            NmeStmt::Say { value } => {
                let written = self.value(value)?;
                Some(match (self.language, value) {
                    (Language::English, _) => format!("show {written}"),
                    // Korean puts its verb at the end of a sentence, and a
                    // sentence is what this level writes. A Python expression
                    // is not a sentence, so the verb goes in front of it —
                    // which is also where the Python converter puts it, so a
                    // converted line and a written one come out the same.
                    //
                    // One name on its own is not an expression to a reader.
                    // `점수 말해줘` is the first line of the first guide and
                    // the most written line in the language; turning it into
                    // `보여줘 점수` would put English word order in the middle
                    // of a Korean program.
                    (Language::Korean, Value::Python(_)) if !is_one_plain_name(&written) =>
                    {
                        format!("보여줘 {written}")
                    }
                    (Language::Korean, _) => format!("{written} 말해줘"),
                })
            }
            NmeStmt::Ask {
                target,
                prompt,
                kind,
            } => self.ask(target, prompt.as_ref(), *kind),
            NmeStmt::Set { target, value } => self.set(target, value),
            NmeStmt::Update {
                target,
                amount,
                operation,
            } => {
                let amount = self.code(amount);
                Some(match (self.language, operation) {
                    (Language::English, UpdateOp::Add) => format!("add {amount} to {target}"),
                    (Language::English, UpdateOp::Subtract) => {
                        format!("subtract {amount} from {target}")
                    }
                    (Language::English, UpdateOp::Multiply) => {
                        format!("multiply {target} by {amount}")
                    }
                    (Language::English, UpdateOp::Divide) => format!("divide {target} by {amount}"),
                    (Language::Korean, UpdateOp::Add) => format!("{target}에 {amount} 더해"),
                    (Language::Korean, UpdateOp::Subtract) => {
                        format!("{target}에서 {amount} 빼줘")
                    }
                    (Language::Korean, UpdateOp::Multiply) => format!("{target}에 {amount} 곱해"),
                    (Language::Korean, UpdateOp::Divide) => {
                        format!("{target}를 {amount}로 나눠")
                    }
                })
            }
            NmeStmt::Times { count, inline } => {
                let count = self.code(count);
                match self.language {
                    Language::English => {
                        self.with_inline(format!("repeat {count} times"), " and ", inline.as_ref())
                    }
                    Language::Korean => {
                        let count = korean_counted(&count, "번");
                        match inline {
                            None => Some(format!("{count} 반복해")),
                            Some(_) => {
                                self.with_inline(format!("{count} 반복해서"), " ", inline.as_ref())
                            }
                        }
                    }
                }
            }
            NmeStmt::ForEach {
                name,
                items,
                position,
                inline,
            } => {
                let items = self.code(items);
                match self.language {
                    Language::English => {
                        let turn = position
                            .as_ref()
                            .map_or_else(String::new, |position| format!(" with {position}"));
                        self.with_inline(
                            format!("for each {name} in {items}{turn}"),
                            " and ",
                            inline.as_ref(),
                        )
                    }
                    Language::Korean => {
                        let items = korean_name(items.as_str())?;
                        let turn = position
                            .as_ref()
                            .map_or_else(String::new, |position| format!(" {position}와 함께"));
                        match inline {
                            None => Some(format!("{items}의 {name}마다{turn} 반복해")),
                            Some(_) => self.with_inline(
                                format!("{items}의 {name}마다{turn} 반복해서"),
                                " ",
                                inline.as_ref(),
                            ),
                        }
                    }
                }
            }
            NmeStmt::Wait { seconds } => {
                let seconds = self.code(seconds);
                Some(match self.language {
                    Language::English if is_written_number(&seconds) => {
                        format!("wait {}", english_seconds(&seconds))
                    }
                    Language::English => format!("wait {seconds}"),
                    Language::Korean => format!("{} 기다려", korean_counted(&seconds, "초")),
                })
            }
            NmeStmt::SaySlowly { value, seconds } => self.say_slowly(value, seconds),
            NmeStmt::ClearScreen => Some(self.either("clear the screen", "화면 지워")),
            NmeStmt::DrawLine => Some(self.either("draw a line", "줄 그어")),
            NmeStmt::SayInBox { value } => {
                let value = self.value(value)?;
                Some(self.either(
                    &format!("say in a box {value}"),
                    &format!("상자로 말해줘 {value}"),
                ))
            }
            NmeStmt::SayInMiddle { value } => {
                let value = self.value(value)?;
                Some(self.either(
                    &format!("say in the middle {value}"),
                    &format!("가운데 말해줘 {value}"),
                ))
            }
            NmeStmt::StartTimer => Some(self.either("start the timer", "시간 재기 시작해")),
            NmeStmt::Cooldown { target, seconds } => {
                let seconds = self.code(seconds);
                Some(match self.language {
                    Language::English => {
                        format!("put {target} on cooldown for {}", english_seconds(&seconds))
                    }
                    Language::Korean => {
                        format!("{target} 쿨타임 {} 걸어", korean_counted(&seconds, "초"))
                    }
                })
            }
            NmeStmt::WaitForCooldown { target } => Some(self.either(
                &format!("wait for {target}"),
                &format!("{target} 쿨타임 끝날때까지 기다려"),
            )),
            NmeStmt::Append { target, value } => {
                let value = self.value(value)?;
                Some(self.either(
                    &format!("append {value} to {target}"),
                    &format!("{target}에 {value} 넣어"),
                ))
            }
            NmeStmt::Remove { target, value } => {
                let value = self.value(value)?;
                Some(self.either(
                    &format!("remove {value} from {target}"),
                    &format!("{target}에서 {value} 빼"),
                ))
            }
            NmeStmt::RecordPut { target, key, value } => {
                let key = self.value(key)?;
                let value = self.value(value)?;
                Some(self.either(
                    &format!("put {key} at {value} in {target}"),
                    &format!(
                        "{target}에 {} {} 넣어",
                        korean_marked(&key, "를"),
                        korean_marked(&value, "으로")
                    ),
                ))
            }
            NmeStmt::RecordRemove { target, key } => {
                let key = self.value(key)?;
                Some(self.either(
                    &format!("remove {key} from {target}"),
                    &format!("{target}에서 {key} 빼"),
                ))
            }
            NmeStmt::Arrange { target, order } => Some(match (self.language, order) {
                (Language::English, ListOrder::Sorted) => format!("sort {target}"),
                (Language::English, ListOrder::Reversed) => format!("reverse {target}"),
                (Language::English, ListOrder::Shuffled) => format!("shuffle {target}"),
                (Language::Korean, ListOrder::Sorted) => format!("{target} 정렬해"),
                (Language::Korean, ListOrder::Reversed) => format!("{target} 거꾸로 해"),
                (Language::Korean, ListOrder::Shuffled) => format!("{target} 섞어"),
            }),
            NmeStmt::Forever { inline } => match self.language {
                Language::English => {
                    self.with_inline("repeat forever".to_string(), " and ", inline.as_ref())
                }
                Language::Korean => match inline {
                    None => Some("계속 반복해".to_string()),
                    Some(_) => self.with_inline("계속 반복해서".to_string(), " ", inline.as_ref()),
                },
            },
            NmeStmt::Chance { permille, inline } => {
                let chance = percentage(*permille);
                let header =
                    self.either(&format!("{chance}% chance"), &format!("{chance}% 확률로"));
                self.with_inline(header, " ", inline.as_ref())
            }
            NmeStmt::Story { seconds } => Some(match (self.language, seconds) {
                (Language::English, None) => "story:".to_string(),
                (Language::Korean, None) => "이야기:".to_string(),
                (language, Some(seconds)) => {
                    let seconds = self.code(seconds);
                    match (language, seconds.as_str()) {
                        (Language::English, SLOW_SECONDS) => "slow story:".to_string(),
                        (Language::English, VERY_SLOW_SECONDS) => "very slow story:".to_string(),
                        (Language::English, seconds) => {
                            format!("slow story every {}:", english_seconds(seconds))
                        }
                        (Language::Korean, SLOW_SECONDS) => "천천히 이야기:".to_string(),
                        (Language::Korean, VERY_SLOW_SECONDS) => "아주 천천히 이야기:".to_string(),
                        (Language::Korean, seconds) => format!("{seconds}초씩 천천히 이야기:"),
                    }
                }
            }),
            NmeStmt::Job { name, parameters } => {
                let given = parameters.first();
                Some(match (self.language, given) {
                    (Language::English, None) => format!("to {name}:"),
                    (Language::English, Some(given)) => format!("to {name} {given}:"),
                    (Language::Korean, None) => format!("{name}라는 일:"),
                    (Language::Korean, Some(given)) => format!("{given}에게 {name}라는 일:"),
                })
            }
            NmeStmt::RunJob { name, arguments } => {
                let given = match arguments.first() {
                    None => None,
                    Some(argument) => Some(self.value(argument)?),
                };
                if arguments.len() > 1 {
                    return None;
                }
                Some(match (self.language, given) {
                    (Language::English, None) => format!("do {name}"),
                    (Language::English, Some(given)) => format!("do {name} with {given}"),
                    (Language::Korean, None) => format!("{name} 해줘"),
                    (Language::Korean, Some(given)) => {
                        format!("{} {name} 해줘", korean_marked(&given, "에게"))
                    }
                })
            }
            NmeStmt::When { condition, inline } => {
                self.branch("if", "만약에", condition, inline.as_ref())
            }
            NmeStmt::ElseIf { condition, inline } => {
                self.branch("else if", "아니면 만약에", condition, inline.as_ref())
            }
            NmeStmt::While { condition, inline } => self.while_loop(condition, inline.as_ref()),
            NmeStmt::Else { inline } => {
                self.with_inline(self.either("else", "아니면"), " ", inline.as_ref())
            }
            NmeStmt::Break => Some(self.either("break", "멈춰")),
            NmeStmt::Continue => Some(self.either("skip", "건너뛰어")),
            NmeStmt::End => Some(self.either("end", "끝")),
            NmeStmt::UseModule { module, requested } => {
                let version = match requested {
                    ModuleVersion::Bundled => String::new(),
                    ModuleVersion::Latest => self.either(" latest", " 최신"),
                    ModuleVersion::Exact(version) => self.either(
                        &format!(" version \"{version}\""),
                        &format!(" 버전 \"{version}\""),
                    ),
                };
                Some(self.either(
                    &format!("use {}{version}", module.name_en()),
                    &format!("{} 사용{version}", module.name_ko()),
                ))
            }
            NmeStmt::FileRead { target, path } => {
                let path = self.code(path);
                Some(self.either(
                    &format!("read {path} into {target}"),
                    &format!("{target}에 {path} 읽어서"),
                ))
            }
            NmeStmt::FileWrite { path, value } => {
                let path = self.code(path);
                let value = self.value(value)?;
                Some(self.either(
                    &format!("write {value} to {path}"),
                    &format!("{path} 파일에 {} 저장해", korean_marked(&value, "를")),
                ))
            }
            NmeStmt::ModuleImport { path, names } => {
                let path = self.code(path);
                let names = names.join(", ");
                Some(self.either(
                    &format!("use {names} from {path}"),
                    &format!("{path}에서 {names} 가져와"),
                ))
            }
        }
    }

    fn ask(&self, target: &str, prompt: Option<&Value>, kind: InputKind) -> Option<String> {
        let question = match prompt {
            None => String::new(),
            Some(Value::Text(text)) => {
                // A prompt that already ends in a space lowers without the
                // space the sentence form adds, so writing it as words would
                // ask a different question.
                if template_ends_with_whitespace(text) {
                    return None;
                }
                format!(" {}", plain_text(text))
            }
            Some(value) => format!(" {}", self.value(value)?),
        };
        Some(match (self.language, kind) {
            (Language::English, InputKind::Text) => format!("ask {target}{question}"),
            (Language::English, InputKind::Number) => format!("ask number {target}{question}"),
            (Language::Korean, InputKind::Text) => format!("{target}을 물어봐{question}"),
            (Language::Korean, InputKind::Number) => {
                format!("{target}을 숫자로 물어봐{question}")
            }
        })
    }

    fn set(&self, target: &str, value: &Value) -> Option<String> {
        // Three values are spelled by what they are rather than by a value
        // phrase: the two empty containers, and a chance, which English saves
        // with `is` instead of `set … to …`.
        match value {
            Value::List(items) if items.is_empty() => {
                return Some(self.either(
                    &format!("set {target} to an empty list"),
                    &format!("{target}은 빈 목록"),
                ))
            }
            Value::EmptyRecord => {
                return Some(self.either(
                    &format!("set {target} to an empty record"),
                    &format!("{target}은 빈 표"),
                ))
            }
            Value::Chance { permille } => {
                let chance = percentage(*permille);
                return Some(self.either(
                    &format!("{target} is a {chance}% chance"),
                    &format!("{target}은 {chance}% 확률"),
                ));
            }
            _ => {}
        }
        let value = self.value(value)?;
        Some(self.either(
            &format!("set {target} to {value}"),
            &format!("{target}은 {value}"),
        ))
    }

    fn say_slowly(&self, value: &Value, seconds: &Code) -> Option<String> {
        let told = self.value(value)?;
        let seconds = self.code(seconds);
        Some(match (self.language, seconds.as_str()) {
            (Language::English, SLOW_SECONDS) => format!("say slowly {told}"),
            (Language::English, VERY_SLOW_SECONDS) => format!("say very slowly {told}"),
            (Language::English, seconds) => {
                format!("say slowly every {} {told}", english_seconds(seconds))
            }
            (Language::Korean, SLOW_SECONDS) => format!("천천히 말해줘 {told}"),
            (Language::Korean, VERY_SLOW_SECONDS) => format!("아주 천천히 말해줘 {told}"),
            (Language::Korean, seconds) => format!("{seconds}초씩 천천히 말해줘 {told}"),
        })
    }

    fn branch(
        &self,
        english: &str,
        korean: &str,
        condition: &Condition,
        inline: Option<&InlineStmt>,
    ) -> Option<String> {
        match self.language {
            Language::English => {
                let condition = self.english_condition(condition)?;
                self.with_inline(format!("{english} {condition}"), " then ", inline)
            }
            Language::Korean => {
                let (condition, ends_the_clause) = self.korean_condition(condition)?;
                let header = format!("{korean} {condition}");
                match inline {
                    None => Some(header),
                    // Only a Korean condition ending can carry the body
                    // straight after it. A Python condition has no ending, so
                    // it takes the `이면` the grammar gives it.
                    Some(_) if ends_the_clause => self.with_inline(header, " ", inline),
                    Some(_) => self.with_inline(format!("{header} 이면"), " ", inline),
                }
            }
        }
    }

    fn while_loop(&self, condition: &Condition, inline: Option<&InlineStmt>) -> Option<String> {
        match self.language {
            Language::English => {
                let condition = self.english_condition(condition)?;
                self.with_inline(format!("while {condition}"), " then ", inline)
            }
            Language::Korean => {
                let named = self.condition_on_a_name(condition);
                let condition = named.as_ref().unwrap_or(condition);
                let header = match condition {
                    // The beginner spelling puts the loop word first, which is
                    // the only Korean shape a Python condition has.
                    Condition::Python(code) => format!("동안 {}", self.code(code)),
                    _ => format!("{} 동안", self.korean_while_condition(condition)?),
                };
                self.with_inline(header, " ", inline)
            }
        }
    }

    // ----------------------------------------------------------- conditions

    fn english_condition(&self, condition: &Condition) -> Option<String> {
        if let Some(truthy) = self.condition_on_a_name(condition) {
            return self.english_condition(&truthy);
        }
        match condition {
            Condition::Python(code) => Some(self.code(code)),
            Condition::Truthy { value, negated } => {
                if let Some((target, ready)) = cooldown_reading(value) {
                    if *negated {
                        return None;
                    }
                    return Some(if ready {
                        format!("{target} is ready")
                    } else {
                        format!("{target} is on cooldown")
                    });
                }
                let value = self.condition_value(value)?;
                Some(if *negated {
                    format!("{value} missing")
                } else {
                    format!("{value} exists")
                })
            }
            Condition::Compare {
                left,
                operator,
                right,
                negated,
            } => {
                let left = self.condition_value(left)?;
                let right = self.condition_value(right)?;
                let words = match (operator, negated) {
                    (CompareOp::Equal, false) => "equals",
                    (CompareOp::Equal, true) => "is not equal to",
                    (CompareOp::Greater, false) => "is greater than",
                    (CompareOp::Less, false) => "is less than",
                    (CompareOp::GreaterOrEqual, false) => "is greater than or equal to",
                    (CompareOp::LessOrEqual, false) => "is less than or equal to",
                    (CompareOp::Contains, false) => "contains",
                    (CompareOp::Contains, true) => "does not contain",
                    // `not (score > 10)` has no sentence spelling: the words
                    // for it are `is less than or equal to`, which is a
                    // different comparison in Python.
                    _ => return None,
                };
                Some(format!("{left} {words} {right}"))
            }
            Condition::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.english_condition(left)?;
                let right = self.english_condition(right)?;
                let word = match operator {
                    LogicalOp::And => "and",
                    LogicalOp::Or => "or",
                };
                Some(format!("{left} {word} {right}"))
            }
        }
    }

    /// The Korean condition, and whether it ends in one of the endings that
    /// close a clause (`있으면`, `크면`, …).
    fn korean_condition(&self, condition: &Condition) -> Option<(String, bool)> {
        if let Some(truthy) = self.condition_on_a_name(condition) {
            return self.korean_condition(&truthy);
        }
        match condition {
            Condition::Python(code) => Some((self.code(code), false)),
            Condition::Truthy { value, negated } => {
                if let Some((target, ready)) = cooldown_reading(value) {
                    if *negated {
                        return None;
                    }
                    let ending = if ready {
                        "끝났으면"
                    } else {
                        "남았으면"
                    };
                    return Some((format!("{target} 쿨타임이 {ending}"), true));
                }
                // Korean reads the subject of a truthy condition as one word
                // carrying a particle, so a reading or an entry standing there
                // has no spelling — and neither has a literal, because `거짓이`
                // is the *name* `거짓` once the particle comes off.
                let ConditionValue::Name(value) = value else {
                    return None;
                };
                let ending = if *negated { "없으면" } else { "있으면" };
                Some((format!("{value}이 {ending}"), true))
            }
            Condition::Compare {
                left,
                operator,
                right,
                negated,
            } => {
                let right = self.condition_value(right)?;
                if *operator == CompareOp::Contains {
                    let ConditionValue::Name(container) = left else {
                        return None;
                    };
                    let ending = if *negated { "없으면" } else { "있으면" };
                    return Some((format!("{container}에 {right}가 {ending}"), true));
                }
                let left = self.condition_value(left)?;
                let comparison = match (operator, negated) {
                    (CompareOp::Equal, false) => format!("{right}과 같으면"),
                    (CompareOp::Equal, true) => format!("{right}과 같지 않으면"),
                    (CompareOp::Greater, false) => format!("{right}보다 크면"),
                    (CompareOp::Less, false) => format!("{right}보다 작으면"),
                    (CompareOp::GreaterOrEqual, false) => format!("{right}보다 크거나 같으면"),
                    (CompareOp::LessOrEqual, false) => format!("{right}보다 작거나 같으면"),
                    _ => return None,
                };
                Some((format!("{} {comparison}", korean_marked(&left, "가")), true))
            }
            Condition::Logical {
                left,
                operator,
                right,
            } => {
                let (left, _) = self.korean_condition(left)?;
                let (right, ends_the_clause) = self.korean_condition(right)?;
                let word = match operator {
                    LogicalOp::And => "그리고",
                    LogicalOp::Or => "또는",
                };
                Some((format!("{left} {word} {right}"), ends_the_clause))
            }
        }
    }

    /// The Korean spelling a loop condition takes, without its `동안`.
    ///
    /// Korean closes a loop condition with a different ending from the one a
    /// branch takes, and only some of those endings are written down. The
    /// rest answer `None` and keep the line the writer wrote.
    fn korean_while_condition(&self, condition: &Condition) -> Option<String> {
        match condition {
            Condition::Truthy {
                value: ConditionValue::Name(name),
                negated: false,
            } => Some(format!("{name}하는")),
            Condition::Compare {
                left,
                operator,
                right,
                negated,
            } => {
                let left = self.condition_value(left)?;
                let right = self.condition_value(right)?;
                let comparison = match (operator, negated) {
                    (CompareOp::Greater, false) => format!("{right}보다 큰"),
                    (CompareOp::Less, false) => format!("{right}보다 작을"),
                    (CompareOp::Equal, true) => format!("{right}과 같지 않을"),
                    _ => return None,
                };
                Some(format!("{} {comparison}", korean_marked(&left, "가")))
            }
            Condition::Logical {
                left,
                operator,
                right,
            } => {
                // Only the shape the reference writes down: two plain names,
                // one loop ending for both of them.
                let (
                    Condition::Truthy {
                        value: left,
                        negated: false,
                    },
                    Condition::Truthy {
                        value: right,
                        negated: false,
                    },
                ) = (left.as_ref(), right.as_ref())
                else {
                    return None;
                };
                let (ConditionValue::Name(left), ConditionValue::Name(right)) = (left, right)
                else {
                    return None;
                };
                let word = match operator {
                    LogicalOp::And => "그리고",
                    LogicalOp::Or => "또는",
                };
                Some(format!("{left} {word} {right}"))
            }
            _ => None,
        }
    }

    /// A condition that is nothing but a name the program made is the very
    /// same condition as `name exists`, and that is how the sentence level
    /// reads it back. Writing it that way is what keeps a second tidying from
    /// moving the line again.
    fn condition_on_a_name(&self, condition: &Condition) -> Option<Condition> {
        let Condition::Python(code) = condition else {
            return None;
        };
        let name = bare_name(&self.code(code))?;
        Some(Condition::Truthy {
            value: ConditionValue::Name(name),
            negated: false,
        })
    }

    fn condition_value(&self, value: &ConditionValue) -> Option<String> {
        match value {
            ConditionValue::Python(Code::Generated(python)) if python == ELAPSED_PYTHON => {
                Some(self.either("elapsed", "잰시간"))
            }
            ConditionValue::Python(code) => Some(self.code(code)),
            ConditionValue::Name(name) => Some(name.clone()),
            ConditionValue::Text(text) => (!text.is_empty()).then(|| text.clone()),
            ConditionValue::Literal(literal) => Some(self.literal(*literal).to_string()),
            ConditionValue::Reading { of, reading } => Some(self.reading(of, *reading)),
            ConditionValue::Remainder { of, by } => Some(self.remainder(of, by)),
            ConditionValue::Entry { of, key } => {
                let key = self.value(key)?;
                Some(self.either(&format!("{key} in {of}"), &format!("{of}의 {key}")))
            }
        }
    }

    // --------------------------------------------------------------- values

    /// A value written the way a sentence writes it.
    fn value(&self, value: &Value) -> Option<String> {
        match value {
            Value::Python(Code::Generated(python)) if python == ELAPSED_PYTHON => {
                Some(self.either("elapsed", "잰시간"))
            }
            Value::Python(code) => Some(self.code(code)),
            Value::Text(text) => {
                let words = plain_text(text);
                (!words.is_empty()).then_some(words)
            }
            Value::Literal(literal) => Some(self.literal(*literal).to_string()),
            Value::RandomInteger { low, high } => {
                let low = self.code(low);
                let high = self.code(high);
                Some(self.either(
                    &format!("random number from {low} to {high}"),
                    &format!("{low}부터 {high}까지 랜덤정수"),
                ))
            }
            Value::RandomChoice { choices } => {
                if choices.is_empty() {
                    return None;
                }
                Some(self.either(
                    &format!("pick from {}", choices.join(" or ")),
                    &format!("{} 중에서 랜덤선택", choices.join(" 또는 ")),
                ))
            }
            // An empty list and an empty record are values only where a name
            // is being given one, so `set` writes them and nothing else does.
            Value::List(items) if items.is_empty() => None,
            Value::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Option<Vec<_>>>()?
                    .join(", ");
                Some(self.either(&format!("list of {items}"), &format!("목록 {items}")))
            }
            Value::EmptyRecord => None,
            Value::Entry { of, key } => {
                let key = self.value(key)?;
                Some(self.either(&format!("{key} in {of}"), &format!("{of}의 {key}")))
            }
            Value::Reading { of, reading } => Some(self.reading(of, *reading)),
            Value::Item { of, position } => Some(match (self.language, position) {
                (Language::English, ItemPosition::First) => format!("the first of {of}"),
                (Language::English, ItemPosition::Last) => format!("the last of {of}"),
                (Language::English, ItemPosition::Numbered(at)) => {
                    format!("item {} of {of}", self.code(at))
                }
                (Language::Korean, ItemPosition::First) => format!("{of} 첫 번째"),
                (Language::Korean, ItemPosition::Last) => format!("{of} 마지막"),
                (Language::Korean, ItemPosition::Numbered(at)) => {
                    format!("{of} {}", korean_counted(&self.code(at), "번째"))
                }
            }),
            Value::Joined { of, separator } => {
                let (english, korean) = separator_words(separator)?;
                Some(match (self.language, separator.as_str()) {
                    (Language::English, "") => format!("{of} joined together"),
                    (Language::English, _) => format!("{of} joined by {english}"),
                    (Language::Korean, "") => format!("{of}을 붙여"),
                    (Language::Korean, _) => format!("{of}을 {korean} 이어"),
                })
            }
            Value::Split { of, by } => {
                let (english, korean) = match by {
                    SplitBy::Lines => ("line", "줄마다"),
                    SplitBy::Text(text) => {
                        let (english, korean) = split_words(text)?;
                        (english, korean)
                    }
                };
                Some(match self.language {
                    Language::English => format!("{of} split by {english}"),
                    Language::Korean => format!("{of}을 {korean} 나눈 것"),
                })
            }
            Value::Repeated { of, times } => {
                let times = self.code(times);
                Some(self.either(
                    &format!("{of} repeated {times} times"),
                    &format!("{of}을 {} 붙인 것", korean_counted(&times, "개")),
                ))
            }
            Value::Remainder { of, by } => Some(self.remainder(of, by)),
            Value::Elapsed => Some(self.either("elapsed", "잰시간")),
            Value::Chance { permille } => {
                let chance = percentage(*permille);
                Some(self.either(&format!("a {chance}% chance"), &format!("{chance}% 확률")))
            }
            // The Schnorr values have no row in the syntax reference, so there
            // is no spelling this may take from it.
            Value::ZeroKnowledge(_) => None,
        }
    }

    fn reading(&self, of: &str, reading: Reading) -> String {
        match (self.language, reading) {
            (Language::English, Reading::Count) if self.length_not_count => {
                format!("the length of {of}")
            }
            (Language::English, Reading::Count) => format!("how many {of}"),
            (Language::English, Reading::Total) => format!("the total of {of}"),
            (Language::English, Reading::Largest) => format!("the biggest of {of}"),
            (Language::English, Reading::Smallest) => format!("the smallest of {of}"),
            (Language::English, Reading::Capitals) => format!("{of} in capitals"),
            (Language::English, Reading::SmallLetters) => format!("{of} in small letters"),
            (Language::Korean, Reading::Count) if self.length_not_count => format!("{of} 길이"),
            (Language::Korean, Reading::Count) => format!("{of} 개수"),
            (Language::Korean, Reading::Total) => format!("{of} 합"),
            (Language::Korean, Reading::Largest) => format!("{of} 중 가장 큰 것"),
            (Language::Korean, Reading::Smallest) => format!("{of} 중 가장 작은 것"),
            (Language::Korean, Reading::Capitals) => format!("{of} 대문자로"),
            (Language::Korean, Reading::SmallLetters) => format!("{of} 소문자로"),
        }
    }

    fn remainder(&self, of: &str, by: &Code) -> String {
        let by = self.code(by);
        self.either(
            &format!("the remainder of {of} divided by {by}"),
            &format!("{of}을 {} 나눈 나머지", korean_counted(&by, "로")),
        )
    }

    fn literal(&self, literal: Literal) -> &'static str {
        match (self.language, literal) {
            (Language::English, Literal::True) => "true",
            (Language::English, Literal::False) => "false",
            (Language::English, Literal::None) => "none",
            (Language::Korean, Literal::True) => "참",
            (Language::Korean, Literal::False) => "거짓",
            (Language::Korean, Literal::None) => "없음",
        }
    }

    // -------------------------------------------------------------- helpers

    /// Python copied from the source. NME never reformats an expression.
    fn code(&self, code: &Code) -> String {
        match code {
            Code::Source(span) => self.source[span.start..span.end].to_string(),
            Code::Generated(text) => text.clone(),
        }
    }

    fn either(&self, english: &str, korean: &str) -> String {
        match self.language {
            Language::English => english.to_string(),
            Language::Korean => korean.to_string(),
        }
    }

    fn with_inline(
        &self,
        header: String,
        connector: &str,
        inline: Option<&InlineStmt>,
    ) -> Option<String> {
        let Some(inline) = inline else {
            return Some(header);
        };
        let body = match inline {
            InlineStmt::Nme(stmt) => self.any_statement(stmt)?,
            InlineStmt::Python(span) => self.source[span.start..span.end].to_string(),
        };
        Some(format!("{header}{connector}{body}"))
    }
}

/// Seconds between characters for the plain and the very slow spellings, as
/// the parser writes them.
const SLOW_SECONDS: &str = "0.04";
const VERY_SLOW_SECONDS: &str = "0.12";

/// A text template as the words it was written from: the literal parts as
/// they stand, and each name in its place.
fn plain_text(template: &TextTemplate) -> String {
    template
        .parts
        .iter()
        .map(|part| match part {
            TextPart::Literal(text) => text.as_str(),
            TextPart::Variable(name) => name.as_str(),
        })
        .collect()
}

fn template_ends_with_whitespace(template: &TextTemplate) -> bool {
    matches!(
        template.parts.last(),
        Some(TextPart::Literal(text)) if text.chars().last().is_some_and(char::is_whitespace)
    )
}

/// The Python a template lowers to, for the beginner forms that take an
/// expression. It has to match [`crate::lower`] exactly.
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

fn literal_python(literal: Literal) -> &'static str {
    match literal {
        Literal::True => "True",
        Literal::False => "False",
        Literal::None => "None",
    }
}

/// The `>=`/`<` comparison the cooldown conditions are written from, read back
/// into the name it was written for.
fn cooldown_reading(value: &ConditionValue) -> Option<(String, bool)> {
    let ConditionValue::Python(Code::Generated(python)) = value else {
        return None;
    };
    for (operator, ready) in [(">=", true), ("<", false)] {
        let opening = format!("__import__(\"time\").time() {operator} {COOLDOWN_PREFIX}");
        if let Some(target) = python.strip_prefix(&opening) {
            return Some((target.to_string(), ready));
        }
    }
    None
}

/// `300` → `30`, `305` → `30.5`. The scale is thousandths, so one decimal
/// place is always exact.
fn percentage(permille: u32) -> String {
    let whole = permille / (CHANCE_SCALE / 100);
    let tenth = permille % (CHANCE_SCALE / 100);
    if tenth == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{tenth}")
    }
}

/// A Korean counter is written against the number (`3초`, `5개`), which is one
/// word. A name cannot take one that way — `쉬는시간초` would be a different
/// name — so it keeps its space.
fn korean_counted(amount: &str, counter: &str) -> String {
    if is_written_number(amount) {
        format!("{amount}{counter}")
    } else {
        format!("{amount} {counter}")
    }
}

/// A Korean particle goes on the end of the word it marks, but only when that
/// word is one the reader (and the lexer) will still see whole. Anything with
/// a quote, a bracket or a space in it keeps the particle a word away.
fn korean_marked(word: &str, particle: &str) -> String {
    if word
        .chars()
        .all(|character| character.is_alphanumeric() || character == '_' || character == '.')
    {
        format!("{word}{particle}")
    } else {
        format!("{word} {particle}")
    }
}

/// The name a Korean loop or reading can attach a particle to: one plain word,
/// never an expression.
fn korean_name(code: &str) -> Option<&str> {
    let name = code.trim();
    (!name.is_empty()
        && !name.starts_with(|character: char| character.is_ascii_digit())
        && name
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_'))
    .then_some(name)
}

/// The one name a condition is written on, when that is the whole of it.
/// The words for true, false and nothing are not names: they are read as the
/// values they are, and have their own spelling.
fn bare_name(text: &str) -> Option<String> {
    let text = text.trim();
    let is_name = !text.is_empty()
        && !text.starts_with(|character: char| character.is_ascii_digit())
        && text
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_');
    let is_literal = [
        "True", "False", "None", "true", "false", "none", "null", "참", "거짓", "없음",
    ]
    .contains(&text);
    (is_name && !is_literal).then(|| text.to_string())
}

/// `1 second`, and `3 seconds`. English counts its unit word, and a tidier
/// that writes `1 seconds` is not tidy.
fn english_seconds(count: &str) -> String {
    if count == "1" {
        "1 second".to_string()
    } else {
        format!("{count} seconds")
    }
}

fn is_written_number(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|character| character.is_ascii_digit() || character == '.')
}

/// The words for the separator a join puts between the items. The Korean word
/// carries the particle it is written with, because these are four fixed words
/// rather than names: `쉼표로`, but `빈칸으로`.
fn separator_words(separator: &str) -> Option<(&'static str, &'static str)> {
    match separator {
        "" => Some(("nothing", "그대로")),
        ", " => Some(("comma", "쉼표로")),
        " " => Some(("space", "빈칸으로")),
        "\n" => Some(("newline", "줄바꿈으로")),
        // A separator somebody wrote out has no named spelling to go back to.
        _ => None,
    }
}

/// The words for the separator a split cuts on. A comma is `","` here and
/// `", "` in a join, which is why the two tables are apart.
fn split_words(separator: &str) -> Option<(&'static str, &'static str)> {
    match separator {
        "," => Some(("comma", "쉼표로")),
        " " => Some(("space", "빈칸으로")),
        "\n" => Some(("newline", "줄바꿈으로")),
        _ => None,
    }
}

/// True when the written code is a single name and nothing else, so a Korean
/// sentence may put its verb after it.
fn is_one_plain_name(code: &str) -> bool {
    let word = code.trim();
    !word.is_empty()
        && !word.starts_with(|first: char| first.is_ascii_digit())
        && word
            .chars()
            .all(|letter| letter.is_alphanumeric() || letter == '_')
}
