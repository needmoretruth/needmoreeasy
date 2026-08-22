//! Writes one parsed NME statement back out as NME.
//!
//! This is the half of the tidier that chooses words. Every spelling here is
//! taken from `docs/syntax.md` and `docs/syntax.ko.md`, which are generated
//! from the compiler itself, so the words the tidier writes are words the
//! parser is known to read. Nothing is invented: a shape with no row in those
//! files answers `None`, and [`crate::tidy`] then leaves the line as the
//! writer wrote it.

use crate::convert::{Language, SyntaxLevel};
use crate::from_python;
use crate::diagnostics::korean_particle;
use crate::lower::{lower_condition, lower_reading, lower_value};
use crate::syntax::{
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, ItemPosition, ListOrder,
    Literal, LogicalOp, ModuleVersion, NmeStmt, Reading, SplitBy, TextPart, TextTemplate, UpdateOp,
    ZeroKnowledgeValue,
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
    /// True when the line being written ended with a `:`.
    ///
    /// NME closes a block two ways — with `end`, or with the indentation under
    /// a `:` — and which one a file uses is not the tidier's to change. A `:`
    /// put on a header whose block is closed by `end` leaves the `end` closing
    /// nothing, and taking one off a header whose block is closed by
    /// indentation loses the block. So the mark the writer used comes back.
    pub(crate) colon: bool,
    /// Names the program made into a list or a record.
    ///
    /// `len(x)` is one piece of Python and two sentences: `how many friends`
    /// counts things, `the length of name` counts letters. Which one a line
    /// means cannot be read off the Python, so the answer comes from what the
    /// program put in the name.
    pub(crate) containers: &'a std::collections::HashSet<String>,
    /// Whether a piece of Python that is plainly a message may be written as
    /// the words it says.
    ///
    /// `print("Hello")` is `show Hello`, which is the sentence a beginner
    /// wants — unless the program later makes a name out of one of those
    /// words, and then the sentence would print its value instead. The tidier
    /// asks for the plainer spelling as a second choice, and this is the flag
    /// that gives it one.
    pub(crate) read_messages: bool,
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
        let written_again = self.any_statement(stmt)?;
        Some(self.with_the_writer_s_colon(stmt, written_again))
    }

    /// The header with the `:` the writer used, when this line opens a block
    /// and nothing was written after it.
    fn with_the_writer_s_colon(&self, stmt: &NmeStmt, text: String) -> String {
        let opens_a_block = matches!(
            stmt,
            NmeStmt::Times { inline: None, .. }
                | NmeStmt::ForEach { inline: None, .. }
                | NmeStmt::Forever { inline: None }
                | NmeStmt::Chance { inline: None, .. }
                | NmeStmt::When { inline: None, .. }
                | NmeStmt::While { inline: None, .. }
                | NmeStmt::ElseIf { inline: None, .. }
                | NmeStmt::Else { inline: None }
                | NmeStmt::Story { .. }
                | NmeStmt::Job { .. }
        );
        if !self.colon || !opens_a_block || text.ends_with(':') {
            return text;
        }
        format!("{text}:")
    }

    /// The statement in the level asked for, falling back to the other level
    /// wherever the one asked for has no row of its own.
    ///
    /// The fall back runs both ways. Beginner syntax is a smaller surface than
    /// sentence syntax on purpose, so most of the gaps are on that side; but a
    /// few shapes go the other way — a question whose prompt already ends in a
    /// space cannot be written as a sentence — and leaving those as they stand
    /// meant a line stayed in the language it came in. A spelling from the
    /// other level is still the language the reader asked for, and that is the
    /// half of the answer that matters most.
    fn any_statement(&self, stmt: &NmeStmt) -> Option<String> {
        if self.level == SyntaxLevel::Beginner {
            if let Some(beginner) = self.beginner_statement(stmt) {
                return Some(beginner);
            }
            return self.sentence_statement(stmt);
        }
        self.sentence_statement(stmt)
            .or_else(|| self.beginner_statement(stmt))
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
                kind,
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
                // A question that wants a number says so in the same place at
                // both levels: after the asking word in English, before the
                // name in Korean.
                Some(match (self.language, kind) {
                    (Language::English, InputKind::Text) => format!("ask {target}{question}"),
                    (Language::English, InputKind::Number) => {
                        format!("ask number {target}{question}")
                    }
                    (Language::Korean, InputKind::Text) => format!("물어봐 {target}{question}"),
                    (Language::Korean, InputKind::Number) => {
                        format!("물어봐 숫자로 {target}{question}")
                    }
                })
            }
            // The beginner spellings of the two loops are written with the
            // `:` that opens their block. A file whose blocks are closed with
            // `end` has no place for one, so there the sentence spelling —
            // which is closed the same way — is the one that is written.
            NmeStmt::Times { count, inline } if self.colon || inline.is_some() => {
                let count = self.code(count);
                let header = self.either(&format!("{count} times"), &format!("{count} 번"));
                self.with_inline(header, " ", inline.as_ref())
            }
            NmeStmt::ForEach {
                name,
                items,
                position,
                inline: None,
            } if self.colon => {
                let items = self.code(items);
                Some(match (self.language, position) {
                    (Language::English, None) => format!("for each {name} in {items}"),
                    (Language::English, Some(position)) => {
                        format!("for each {name} in {items} with {position}")
                    }
                    (Language::Korean, None) => {
                        format!("{}의 {name}마다", korean_name(items.as_str())?)
                    }
                    (Language::Korean, Some(position)) => format!(
                        "{}의 {name}마다 {position}와 함께",
                        korean_name(items.as_str())?
                    ),
                })
            }
            NmeStmt::When { condition, inline } => {
                let condition = self.python_condition(condition)?;
                let header =
                    self.either(&format!("when {condition}"), &format!("만약 {condition}"));
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
            Value::Text(text) => (!text.parts.is_empty()).then(|| lower_template(text)),
            // The Schnorr values have no row in the syntax reference and their
            // Python is a page long, so a line holding one is left as it was
            // rather than written out.
            Value::ZeroKnowledge(_) => None,
            // Beginner syntax writes values as Python, and every value NME has
            // is already known to lower to some. Answering `None` here meant a
            // beginner line kept its sentence spelling, so which of the two a
            // program came out in depended on how it had arrived.
            value => Some(lower_value(value, self.source)),
        }
    }

    /// The Python text a condition lowers to, for the beginner forms that take
    /// an expression.
    #[allow(clippy::unnecessary_wraps)]
    fn python_condition(&self, condition: &Condition) -> Option<String> {
        let lowered = lower_condition(condition, self.source);
        // The header puts its own brackets round whatever it is given, and a
        // beginner line that already carries a pair — `when (ready and
        // waiting)` — is not one the parser reads. So the outer pair comes
        // off here, where it is known to be an outer pair.
        Some(match lowered.strip_prefix('(').and_then(|inner| inner.strip_suffix(')')) {
            Some(inner) if is_wholly_inside_brackets(&lowered) => inner.to_string(),
            _ => lowered,
        })
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
                    (Language::Korean, Value::Python(_)) if !is_one_sentence_value(&written) => {
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
                        format!(
                            "{target}{} {amount}로 나눠",
                            korean_particle(target, "을", "를")
                        )
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
                let value = self.printable_value(value)?;
                Some(self.either(
                    &format!("say in a box {value}"),
                    &format!("상자로 말해줘 {value}"),
                ))
            }
            NmeStmt::SayInMiddle { value } => {
                let value = self.printable_value(value)?;
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
                        korean_marked(&key, "을", "를"),
                        korean_marked(&value, "으로", "로")
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
                    (Language::Korean, None) => {
                        format!("{name}{} 일:", korean_particle(name, "이라는", "라는"))
                    }
                    (Language::Korean, Some(given)) => format!(
                        "{given}에게 {name}{} 일:",
                        korean_particle(name, "이라는", "라는")
                    ),
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
                        format!("{} {name} 해줘", korean_marked(&given, "에게", "에게"))
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
                // What is written into a file keeps the form it was written
                // in: reading `"hello"` as the message it is takes the quotes
                // off, and `hello를 저장해` then names a value rather than
                // saying one.
                let value = self.printable_value(value)?;
                Some(self.either(
                    &format!("write {value} to {path}"),
                    &format!("{path} 파일에 {} 저장해", korean_marked(&value, "을", "를")),
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
                format!(" {}", self.words(text))
            }
            // The question is the other place a piece of Python may not be
            // read as the message it looks like: the sentence form adds the
            // space after it and the written one does not.
            Some(value) => format!(" {}", self.printable_value(value)?),
        };
        Some(match (self.language, kind) {
            (Language::English, InputKind::Text) => format!("ask {target}{question}"),
            (Language::English, InputKind::Number) => format!("ask number {target}{question}"),
            (Language::Korean, InputKind::Text) => {
                format!(
                    "{target}{} 물어봐{question}",
                    korean_particle(target, "을", "를")
                )
            }
            (Language::Korean, InputKind::Number) => {
                format!(
                    "{target}{} 숫자로 물어봐{question}",
                    korean_particle(target, "을", "를")
                )
            }
        })
    }

    fn set(&self, target: &str, value: &Value) -> Option<String> {
        // Three values are spelled by what they are rather than by a value
        // phrase: the two empty containers, and a chance, which English saves
        // with `is` instead of `set … to …`.
        let read_back = self.read_back(value);
        let value = &read_back;
        match value {
            Value::List(items) if items.is_empty() => {
                return Some(self.either(
                    &format!("set {target} to an empty list"),
                    &format!("{target}{} 빈 목록", korean_particle(target, "은", "는")),
                ))
            }
            Value::EmptyRecord => {
                return Some(self.either(
                    &format!("set {target} to an empty record"),
                    &format!("{target}{} 빈 표", korean_particle(target, "은", "는")),
                ))
            }
            Value::Chance { permille } => {
                let chance = percentage(*permille);
                return Some(self.either(
                    &format!("{target} is a {chance}% chance"),
                    &format!(
                        "{target}{} {chance}% 확률",
                        korean_particle(target, "은", "는")
                    ),
                ));
            }
            _ => {}
        }
        let value = self.value(value)?;
        Some(self.either(
            &format!("set {target} to {value}"),
            &format!("{target}{} {value}", korean_particle(target, "은", "는")),
        ))
    }

    fn say_slowly(&self, value: &Value, seconds: &Code) -> Option<String> {
        let told = self.printable_value(value)?;
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
        // The same question the values are asked: a condition the parser
        // kept as Python may be one the sentence has words for.
        if let Condition::Python(code) = condition {
            if let Some(richer) = from_python::condition_from_python(&self.code(code)) {
                return self.english_condition(&richer);
            }
        }
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
        // The same question the values are asked: a condition the parser
        // kept as Python may be one the sentence has words for.
        if let Condition::Python(code) = condition {
            if let Some(richer) = from_python::condition_from_python(&self.code(code)) {
                return self.korean_condition(&richer);
            }
        }
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
                Some((
                    format!("{value}{} {ending}", korean_particle(value, "이", "가")),
                    true,
                ))
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
                    return Some((
                        format!(
                            "{container}에 {right}{} {ending}",
                            korean_particle(&right, "이", "가")
                        ),
                        true,
                    ));
                }
                let left = self.condition_value(left)?;
                let comparison = match (operator, negated) {
                    (CompareOp::Equal, false) => {
                        format!("{right}{} 같으면", korean_particle(&right, "과", "와"))
                    }
                    (CompareOp::Equal, true) => {
                        format!("{right}{} 같지 않으면", korean_particle(&right, "과", "와"))
                    }
                    (CompareOp::Greater, false) => format!("{right}보다 크면"),
                    (CompareOp::Less, false) => format!("{right}보다 작으면"),
                    (CompareOp::GreaterOrEqual, false) => format!("{right}보다 크거나 같으면"),
                    (CompareOp::LessOrEqual, false) => format!("{right}보다 작거나 같으면"),
                    _ => return None,
                };
                Some((format!("{} {comparison}", korean_marked(&left, "이", "가")), true))
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
        // The same question the values are asked: a condition the parser
        // kept as Python may be one the sentence has words for.
        if let Condition::Python(code) = condition {
            if let Some(richer) = from_python::condition_from_python(&self.code(code)) {
                return self.korean_while_condition(&richer);
            }
        }
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
                    (CompareOp::Equal, true) => {
                        format!("{right}{} 같지 않을", korean_particle(&right, "과", "와"))
                    }
                    _ => return None,
                };
                Some(format!("{} {comparison}", korean_marked(&left, "이", "가")))
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
            ConditionValue::Quotient { of, by } => Some(self.quotient(of, by)),
            ConditionValue::AsNumber { of } => Some(self.as_number(of)),
            ConditionValue::Entry { of, key } => {
                let key = self.value(key)?;
                Some(self.either(&format!("{key} in {of}"), &format!("{of}의 {key}")))
            }
        }
    }

    // --------------------------------------------------------------- values

    /// A value written the way a sentence writes it.
    fn value(&self, value: &Value) -> Option<String> {
        self.value_read(value, true)
    }

    /// A value for a place that puts `str(...)` around everything but text.
    ///
    /// `say slowly "Hello"` holds a piece of Python, and reading it as the
    /// message it plainly is would take the wrapper away with it — the same
    /// program, but not the same Python, and the tidier would throw the whole
    /// line out rather than hand back a file that had drifted. So in these
    /// places a message stays the expression it was written as, and only the
    /// words around it change.
    fn printable_value(&self, value: &Value) -> Option<String> {
        self.value_read(value, false)
    }

    /// The value as the words it is really naming, when the writer left
    /// Python where a sentence had a shape of its own.
    fn read_back(&self, value: &Value) -> Value {
        if let Value::Python(code) = value {
            let written = self.code(code);
            if written != ELAPSED_PYTHON {
                if let Some(richer) = from_python::value_from_python(&written) {
                    if self.read_messages || !matches!(richer, Value::Text(_)) {
                        return richer;
                    }
                }
            }
        }
        value.clone()
    }

    fn value_read(&self, value: &Value, may_read_as_text: bool) -> Option<String> {
        // A beginner writes `say len(friends)`, and the parser keeps the
        // expression as Python because that is what was written. The sentence
        // for that line is `show how many friends`, so the expression is
        // asked what it is naming before it is written out as itself.
        if let Value::Python(code) = value {
            let written = self.code(code);
            if written != ELAPSED_PYTHON {
                if let Some(richer) = from_python::value_from_python(&written) {
                    let text = matches!(richer, Value::Text(_));
                    if !text || (may_read_as_text && self.read_messages) {
                        return self.value(&richer);
                    }
                }
            }
        }
        match value {
            Value::Python(Code::Generated(python)) if python == ELAPSED_PYTHON => {
                Some(self.either("elapsed", "잰시간"))
            }
            Value::Python(code) => Some(self.code(code)),
            Value::Text(text) => {
                let words = self.words(text);
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
                    (Language::Korean, "") => {
                        format!("{of}{} 붙여", korean_particle(of, "을", "를"))
                    }
                    (Language::Korean, _) => {
                        format!("{of}{} {korean} 이어", korean_particle(of, "을", "를"))
                    }
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
                    Language::Korean => {
                        format!("{of}{} {korean} 나눈 것", korean_particle(of, "을", "를"))
                    }
                })
            }
            Value::Repeated { of, times } => {
                let times = self.code(times);
                Some(self.either(
                    &format!("{of} repeated {times} times"),
                    &format!(
                        "{of}{} {} 붙인 것",
                        korean_particle(of, "을", "를"),
                        korean_counted(&times, "개")
                    ),
                ))
            }
            Value::Remainder { of, by } => Some(self.remainder(of, by)),
            Value::Quotient { of, by } => Some(self.quotient(of, by)),
            Value::AsNumber { of } => Some(self.as_number(of)),
            Value::Elapsed => Some(self.either("elapsed", "잰시간")),
            Value::Chance { permille } => {
                let chance = percentage(*permille);
                Some(self.either(&format!("a {chance}% chance"), &format!("{chance}% 확률")))
            }
            Value::ZeroKnowledge(value) => Some(self.zero_knowledge(value)),
        }
    }

    /// The template written as words in this language.
    ///
    /// A message is never translated — what a program prints has to stay what
    /// it prints — but a reading standing inside one is not message text: it
    /// is a piece of the language, and a reader of the Korean file cannot use
    /// `how many friends`. So the literal pieces are kept exactly and the
    /// readings are written again in the language being asked for.
    fn words(&self, template: &TextTemplate) -> String {
        template
            .parts
            .iter()
            .map(|part| match part {
                TextPart::Literal(text) => text.clone(),
                TextPart::Variable(name) => name.clone(),
                TextPart::Reading {
                    of,
                    reading,
                    written,
                } => self.reading_words(of, *reading, asks_for_a_length(written)),
            })
            .collect()
    }

    /// One of the zero-knowledge values, in the words `docs/syntax*.md` gives
    /// it.
    ///
    /// Korean marks the things a value is made from and then says what to make
    /// with them: `r와 s와 e로 영지식 응답 만들기`. English puts them in front
    /// in the same order and closes with the word for what is being made.
    fn zero_knowledge(&self, value: &ZeroKnowledgeValue) -> String {
        let given = |parts: &[&Code]| -> (String, String) {
            let written: Vec<String> = parts.iter().map(|code| self.code(code)).collect();
            let english = written.join(" ");
            let last = written.last().cloned().unwrap_or_default();
            let mut korean = String::new();
            for word in &written[..written.len() - 1] {
                korean.push_str(word);
                korean.push_str(korean_particle(word, "과", "와"));
                korean.push(' ');
            }
            korean.push_str(&last);
            korean.push_str(korean_particle(&last, "으로", "로"));
            (english, korean)
        };
        let plain = |english: &str, korean: &str| {
            self.either(
                &format!("zero knowledge {english}"),
                &format!("영지식 {korean}"),
            )
        };
        let made_from = |parts: &[&Code], english: &str, korean: &str| {
            let (before, marked) = given(parts);
            self.either(
                &format!("{before} zero knowledge {english}"),
                &format!("{marked} 영지식 {korean}"),
            )
        };
        match value {
            ZeroKnowledgeValue::Secret => plain("secret make", "비밀 만들기"),
            ZeroKnowledgeValue::Nonce => plain("nonce make", "일회값 만들기"),
            ZeroKnowledgeValue::Challenge => plain("challenge make", "도전 만들기"),
            ZeroKnowledgeValue::SimulatedResponse => {
                plain("simulated response make", "모의 응답 만들기")
            }
            ZeroKnowledgeValue::Public { secret } => {
                made_from(&[secret], "public make", "공개값 만들기")
            }
            ZeroKnowledgeValue::Commitment { nonce } => {
                made_from(&[nonce], "commitment make", "약속 만들기")
            }
            ZeroKnowledgeValue::ChallengeExcept { excluded } => {
                let word = self.code(excluded);
                self.either(
                    &format!("{word} different zero knowledge challenge make"),
                    &format!(
                        "{word}{} 다른 영지식 도전 만들기",
                        korean_particle(&word, "과", "와")
                    ),
                )
            }
            ZeroKnowledgeValue::Response {
                nonce,
                secret,
                challenge,
            } => made_from(&[nonce, secret, challenge], "response make", "응답 만들기"),
            ZeroKnowledgeValue::Verify {
                public_key,
                commitment,
                challenge,
                response,
            } => made_from(
                &[public_key, commitment, challenge, response],
                "verify",
                "검증",
            ),
            ZeroKnowledgeValue::NizkChallenge {
                public_key,
                commitment,
                context,
            } => made_from(
                &[public_key, commitment, context],
                "challenge make",
                "비대화 도전 만들기",
            ),
            ZeroKnowledgeValue::NizkProof { secret, context } => {
                made_from(&[secret, context], "proof make", "비대화 증명 만들기")
            }
            ZeroKnowledgeValue::NizkVerify {
                public_key,
                proof,
                context,
            } => made_from(&[public_key, proof, context], "verify", "비대화 검증"),
            ZeroKnowledgeValue::SimulatedCommitment {
                public_key,
                challenge,
                response,
            } => made_from(
                &[public_key, challenge, response],
                "simulated commitment make",
                "모의 약속 만들기",
            ),
        }
    }

    fn reading(&self, of: &str, reading: Reading) -> String {
        self.reading_words(of, reading, self.length_not_count)
    }

    /// A reading in this language, told to say a length rather than a count.
    ///
    /// The flag is separate from the one on the line because a reading may
    /// stand *inside* a sentence, where each one carries its own answer: the
    /// words the writer used for that part are what say whether they meant
    /// how many things there are or how long the word is.
    fn reading_words(&self, of: &str, reading: Reading, as_length: bool) -> String {
        // A name the program never filled with things is being measured, not
        // counted, whatever words the line came in with.
        let as_length = as_length || !self.containers.contains(of);
        match (self.language, reading) {
            (Language::English, Reading::Count) if as_length => {
                format!("the length of {of}")
            }
            (Language::English, Reading::Count) => format!("how many {of}"),
            (Language::English, Reading::Total) => format!("the total of {of}"),
            (Language::English, Reading::Largest) => format!("the biggest of {of}"),
            (Language::English, Reading::Smallest) => format!("the smallest of {of}"),
            (Language::English, Reading::Capitals) => format!("{of} in capitals"),
            (Language::English, Reading::SmallLetters) => format!("{of} in small letters"),
            (Language::Korean, Reading::Count) if as_length => format!("{of} 길이"),
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
            &format!(
                "{of}{} {} 나눈 나머지",
                korean_particle(of, "을", "를"),
                korean_marked(&by, "으로", "로")
            ),
        )
    }

    fn quotient(&self, of: &str, by: &Code) -> String {
        let by = self.code(by);
        self.either(
            &format!("the whole number of {of} divided by {by}"),
            &format!(
                "{of}{} {} 나눈 몫",
                korean_particle(of, "을", "를"),
                korean_marked(&by, "으로", "로")
            ),
        )
    }

    fn as_number(&self, of: &str) -> String {
        self.either(
            &format!("{of} as a number"),
            &format!("{of}{} 숫자로 바꾼 것", korean_particle(of, "을", "를")),
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
/// True when the whole text sits inside one pair of brackets.
fn is_wholly_inside_brackets(text: &str) -> bool {
    let mut depth = 0_i32;
    for (at, character) in text.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && at + 1 != text.len() {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0 && text.starts_with('(') && text.ends_with(')')
}

fn plain_text(template: &TextTemplate) -> String {
    template
        .parts
        .iter()
        .map(|part| match part {
            TextPart::Literal(text) => text.as_str(),
            TextPart::Variable(name) => name.as_str(),
            // The words the writer typed. A reading inside a sentence is
            // written back exactly as it stood, the same way the rest of a
            // message is: what a message *says* is never translated.
            TextPart::Reading { written, .. } => written.as_str(),
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
            TextPart::Reading { of, reading, .. } => {
                format!("str({})", lower_reading(of, *reading))
            }
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
/// A word with the right half of a Korean particle pair on it.
///
/// Which half a word takes depends on the sound it ends with, so the pair is
/// passed in and `korean_particle` chooses: `점수가`, `이름이`, `p는`. An
/// expression rather than a word keeps a space before the particle, because
/// `str(x) + 1가` reads as part of the code.
fn korean_marked(word: &str, after_consonant: &'static str, after_vowel: &'static str) -> String {
    let particle = korean_particle(word, after_consonant, after_vowel);
    // A reading written in words is still words — `친구들 개수`,
    // `총점을 5로 나눈 나머지` — and Korean puts the particle straight onto the
    // last one. Held a space away it came back as `친구들 개수 가`, which is
    // not how anybody writes and not how the syntax reference shows it. A
    // space inside the phrase is therefore fine; what is not is a quote, a
    // bracket or an operator, because those mean the phrase is Python and
    // `str(x) + 1가` would read as part of the code.
    let separable = !word.is_empty()
        && !word.ends_with(' ')
        && word
            .chars()
            .all(|character| character.is_alphanumeric() || matches!(character, '_' | '.' | ' '));
    if separable {
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

/// True when the written code is one whole value a Korean sentence can stand
/// its verb after: a single name, a single number, or a single quoted piece
/// of text.
///
/// `보여줘 "안녕하세요"` put the verb in front of a line whose value is a
/// sentence on its own, so a tidied Korean program had one line in English
/// word order. Anything with an operator in it stays in front.
fn is_one_sentence_value(code: &str) -> bool {
    is_one_plain_name(code) || is_one_literal(code)
}

/// A single number, or a single quoted piece of text with no quotation mark
/// inside it — so `"a" + "b"` is not one.
fn is_one_literal(code: &str) -> bool {
    let written = code.trim();
    let Some(first) = written.chars().next() else {
        return false;
    };
    if first == '"' || first == '\'' {
        return written.len() > 1
            && written.ends_with(first)
            && !written[1..written.len() - 1].contains(first);
    }
    !written.is_empty()
        && written
            .chars()
            .all(|letter| letter.is_ascii_digit() || letter == '.')
        && written.chars().any(|letter| letter.is_ascii_digit())
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
