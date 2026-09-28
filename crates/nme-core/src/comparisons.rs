//! The one check that runs after parsing: a comparison no input can make true.
//!
//! `만약에 선택이 수호룬과 같으면` reads as a name whenever `수호룬` was set
//! further up, and that is the rule the whole language rests on. It is also
//! how a writer loses a branch without being told: with `수호룬은 거짓` above
//! it, the line compiles to `if (선택 == 수호룬):` — text the reader types
//! against `False` — and never runs, for any input, ever.
//!
//! Nothing here changes what a program *means*. It only refuses the shapes
//! that already meant nothing, and says which two kinds could not meet. All
//! 116 example programs that ship with NME pass it untouched.
//!
//! ## What it will not do
//!
//! The check speaks only when it is certain. A name counts as one kind only
//! when every `Set`/`Ask` in the whole file gives it that same kind; a name
//! any other statement writes to, or that a plain Python line so much as
//! mentions, is dropped from the map and never reported. A number against
//! true/false is left alone on purpose, because Python really does say
//! `True == 1`.

use std::collections::{HashMap, HashSet};

use crate::diagnostics::{korean_particle, Diagnostic, DiagnosticCode, Span};
use crate::lexer::LogicalLine;
use crate::syntax::{
    Code, CompareOp, Condition, ConditionValue, InlineStmt, InputKind, Literal, NmeLine, NmeStmt,
    Reading, Value,
};

/// What a name is known to hold. Absence from the map means "not known", and
/// that is the only other state — there is no `Unknown` variant, because a
/// name the check cannot read must be indistinguishable from one it never saw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// Anything the program will hand to Python as `str`.
    Text,
    Number,
    TrueOrFalse,
    List,
    Record,
    /// A name the program made a job. Nothing a reader types is ever equal to
    /// one, so a menu word that is also a job name is a dead branch.
    Job,
}

impl Kind {
    /// English and Korean for the reader, in the words the guides use.
    fn words(self) -> (&'static str, &'static str) {
        match self {
            Self::Text => ("text", "글"),
            Self::Number => ("a number", "숫자"),
            Self::TrueOrFalse => ("true or false", "참/거짓"),
            Self::List => ("a list", "목록"),
            Self::Record => ("a record", "표"),
            Self::Job => ("a job", "일"),
        }
    }
}

/// Kinds that can meet, grouped. Two kinds in different groups can never be
/// equal to each other, whatever they hold.
///
/// `Number` and `TrueOrFalse` share a group on purpose: Python answers
/// `True == 1` with `True`, so that pair really can meet and is none of our
/// business.
fn family(kind: Kind) -> u8 {
    match kind {
        Kind::Text => 0,
        Kind::Number | Kind::TrueOrFalse => 1,
        Kind::List => 2,
        Kind::Record => 3,
        Kind::Job => 4,
    }
}

/// True when no value of one kind can ever equal a value of the other.
fn can_never_meet(one: Kind, other: Kind) -> bool {
    family(one) != family(other)
}

/// What the program's names hold: one kind each, and for a list written out
/// in one go, the one kind all its items share.
#[derive(Debug, Default)]
struct Names {
    kinds: HashMap<String, Kind>,
    /// Only for a list made in a single statement and never added to. A list
    /// anything else touches is dropped from `kinds` as well, so this map is
    /// never consulted for one.
    items: HashMap<String, Kind>,
    /// Names a question filled. `ask number` is the answer to text weighed
    /// against a number *only* for these; a name the program set itself has
    /// no question to ask differently.
    asked: HashSet<String>,
}

/// Every comparison in the program that cannot come out true.
pub fn comparisons_that_never_meet(
    source: &str,
    lines: &[LogicalLine],
    nme_lines: &[NmeLine],
) -> Vec<Diagnostic> {
    let names = name_kinds(source, lines, nme_lines);
    let mut found = Vec::new();
    for (at, line) in nme_lines.iter().enumerate() {
        let Some(condition) = condition_of(&line.stmt) else {
            continue;
        };
        let place = Place {
            span: line.span,
            has_other_branch: has_other_branch(lines, nme_lines, at),
        };
        collect(condition, place, source, &names, &mut found);
    }
    found
}

/// Where a condition stands, and whether anything of the reader's dies if it
/// always comes out the same way.
#[derive(Clone, Copy)]
struct Place {
    span: Span,
    /// True when an `아니면` / `else` / `else if` hangs off this condition.
    has_other_branch: bool,
}

/// True when this condition has a branch below it that runs when it is false.
///
/// The two spellings put the branch in different places — one indents its
/// body, the other keeps it flat and closes with `끝` — so depth is counted as
/// the real indentation plus the indentation the flat form stands for. That
/// number is the same for a header and its own `아니면`, and larger for
/// everything in between.
///
/// A loop can never have one, so this is always false for `while`, and an
/// always-true loop condition is therefore never reported. That is right for
/// now — a loop that only leaves through `그만` is ordinary, and
/// `계속 반복해` says the same thing outright — but it falls out of this
/// function rather than being chosen, so it is the first thing to look at if
/// the check ever learns to read where a loop ends.
fn has_other_branch(lines: &[LogicalLine], nme_lines: &[NmeLine], at: usize) -> bool {
    let depth = |line: &NmeLine| {
        lines
            .get(line.line_index)
            .map_or(0, |logical| logical.indent)
            + line.virtual_indent
    };
    let Some(header) = nme_lines.get(at) else {
        return false;
    };
    let mine = depth(header);
    for line in &nme_lines[at + 1..] {
        if depth(line) > mine {
            continue;
        }
        return matches!(line.stmt, NmeStmt::Else { .. } | NmeStmt::ElseIf { .. });
    }
    false
}

fn condition_of(stmt: &NmeStmt) -> Option<&Condition> {
    match stmt {
        NmeStmt::When { condition, .. }
        | NmeStmt::While { condition, .. }
        | NmeStmt::ElseIf { condition, .. } => Some(condition),
        _ => None,
    }
}

fn collect(
    condition: &Condition,
    place: Place,
    source: &str,
    names: &Names,
    found: &mut Vec<Diagnostic>,
) {
    match condition {
        Condition::Logical { left, right, .. } => {
            collect(left, place, source, names, found);
            collect(right, place, source, names, found);
        }
        Condition::Compare {
            left,
            operator: CompareOp::Equal,
            right,
            negated,
        } => {
            let (Some(one), Some(other)) = (
                condition_kind(left, source, names),
                condition_kind(right, source, names),
            ) else {
                return;
            };
            if !can_never_meet(one, other) {
                return;
            }
            if *negated && !place.has_other_branch {
                // Always true, and nothing hangs off it: the body runs, which
                // is what the line says. Nothing of the reader's is lost, so
                // there is nothing here worth stopping them for.
                return;
            }
            found.push(never_meet_diagnostic(
                place.span,
                Shape::Equal(*negated),
                side_of(left, source),
                one,
                side_of(right, source),
                other,
                was_asked(left, names) || was_asked(right, names),
            ));
        }
        // `만약에 가방에 검이 있으면` / `if bag contains sword` lowers to
        // `right in left`. Looking for true-or-false in a list of words is the
        // same dead branch wearing different words.
        Condition::Compare {
            left: ConditionValue::Name(container),
            operator: CompareOp::Contains,
            right,
            negated,
        } => {
            let (Some(&Kind::List), Some(&item)) =
                (names.kinds.get(container), names.items.get(container))
            else {
                return;
            };
            let Some(member) = condition_kind(right, source, names) else {
                return;
            };
            if !can_never_meet(item, member) {
                return;
            }
            if *negated && !place.has_other_branch {
                return;
            }
            found.push(never_meet_diagnostic(
                place.span,
                Shape::Contains(*negated),
                Side::Named(container),
                item,
                side_of(right, source),
                member,
                was_asked(right, names),
            ));
        }
        // The middle level writes its condition as Python — `만약 선택 == 수호룬:`
        // — while everything around it is still sentences. The names are the
        // same names, so the same dead branch has to be caught there too.
        Condition::Python(Code::Source(at)) => {
            let Some(text) = source.get(at.start..at.end) else {
                return;
            };
            let Some((left, negated, right)) = python_comparison(text) else {
                return;
            };
            let (Some(one), Some(other)) = (atom_kind(left, names), atom_kind(right, names)) else {
                return;
            };
            if !can_never_meet(one, other) {
                return;
            }
            if negated && !place.has_other_branch {
                return;
            }
            found.push(never_meet_diagnostic(
                place.span,
                Shape::Equal(negated),
                atom_side(left, names),
                one,
                atom_side(right, names),
                other,
                names.asked.contains(left) || names.asked.contains(right),
            ));
        }
        _ => {}
    }
}

/// The two shapes a dead comparison takes, and whether it was written the
/// other way round, which flips *never* into *always*.
#[derive(Clone, Copy)]
enum Shape {
    Equal(bool),
    Contains(bool),
}

/// `a == b` or `a != b` and nothing else, with a plain name or a written-down
/// value on each side.
///
/// Deliberately narrow. Anything with a second operator, a call, a subscript
/// or a word like `and` in it is left alone: the point is to read the one
/// shape a reader at the middle level writes, not to understand Python.
fn python_comparison(text: &str) -> Option<(&str, bool, &str)> {
    let mut body = text.trim();
    if let Some(inner) = body
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
    {
        if !inner.contains(['(', ')']) {
            body = inner.trim();
        }
    }
    if body.contains([
        '(', ')', '[', ']', '<', '>', '+', '-', '*', '/', '%', ',', '.',
    ]) {
        return None;
    }
    let (at, negated) = match (body.find("=="), body.find("!=")) {
        (Some(at), None) => (at, false),
        (None, Some(at)) => (at, true),
        _ => return None,
    };
    let left = body.get(..at)?.trim();
    let right = body.get(at + 2..)?.trim();
    if left.contains('=') || right.contains('=') {
        return None;
    }
    for word in [" and ", " or ", " not ", " in ", " is "] {
        if body.contains(word) {
            return None;
        }
    }
    (!left.is_empty() && !right.is_empty()).then_some((left, negated, right))
}

/// The kind of one side of a Python comparison: a name the sentences made, or
/// a value written down where it stands.
fn atom_kind(atom: &str, names: &Names) -> Option<Kind> {
    code_kind(&Code::Generated(atom.to_string()), "").or_else(|| {
        atom.chars()
            .all(|letter| letter.is_alphanumeric() || letter == '_')
            .then(|| names.kinds.get(atom).copied())
            .flatten()
    })
}

fn atom_side<'a>(atom: &'a str, names: &Names) -> Side<'a> {
    if names.kinds.contains_key(atom) {
        Side::Named(atom)
    } else {
        Side::Written(atom)
    }
}

/// True when this side is a name a question filled.
fn was_asked(value: &ConditionValue, names: &Names) -> bool {
    matches!(value, ConditionValue::Name(name) if names.asked.contains(name))
}

/// How one side of the comparison is put to the reader.
#[derive(Clone, Copy)]
enum Side<'a> {
    /// A name the program filled further up.
    Named(&'a str),
    /// Something written right there in the line.
    Written(&'a str),
    /// A reading or a piece of Python with no short way to say it.
    Unnamed,
}

/// The longest piece of written code worth quoting back. Past this the line
/// itself, already printed above the message, says it better.
const QUOTE_LIMIT: usize = 24;

fn side_of<'a>(value: &'a ConditionValue, source: &'a str) -> Side<'a> {
    match value {
        ConditionValue::Name(name) => Side::Named(name.as_str()),
        ConditionValue::Text(text) if text.chars().count() <= QUOTE_LIMIT => {
            Side::Written(text.as_str())
        }
        ConditionValue::Python(Code::Source(span)) => match source.get(span.start..span.end) {
            Some(text) if text.trim().chars().count() <= QUOTE_LIMIT => Side::Written(text.trim()),
            _ => Side::Unnamed,
        },
        _ => Side::Unnamed,
    }
}

/// The message. It names both sides where they have names, because a reader
/// who wrote `수호룬` twice needs to be told *which* `수호룬` was read.
fn never_meet_diagnostic(
    span: Span,
    shape: Shape,
    left_side: Side<'_>,
    left: Kind,
    right_side: Side<'_>,
    right: Kind,
    was_asked: bool,
) -> Diagnostic {
    let (left_en, left_ko) = left.words();
    let (right_en, right_ko) = right.words();
    let (message_en, message_ko) = match shape {
        Shape::Equal(false) => (
            "this comparison can never be true",
            "이 비교는 참이 될 수 없습니다",
        ),
        Shape::Equal(true) => (
            "this comparison is true no matter what",
            "이 비교는 무엇을 넣어도 참입니다",
        ),
        Shape::Contains(false) => (
            "this list can never hold that",
            "이 목록에는 그것이 들어 있을 수 없습니다",
        ),
        Shape::Contains(true) => (
            "this list holds that no more than it ever could",
            "이 목록에 그것이 없다는 것은 언제나 참입니다",
        ),
    };
    let side_en = |side: &Side<'_>, kind: &str| match side {
        Side::Named(name) => format!("`{name}` holds {kind}"),
        Side::Written(text) => format!("`{text}` is {kind}"),
        Side::Unnamed => format!("one side is {kind}"),
    };
    let holder_en = |side: &Side<'_>, kind: &str| match (shape, side) {
        (Shape::Contains(_), Side::Named(name)) => format!("everything in `{name}` is {kind}"),
        _ => side_en(side, kind),
    };
    // A name takes `에 담긴 것은` rather than a marked subject, so the reader's
    // own name needs no particle chosen for it and reads the same either way.
    let side_ko = |side: &Side<'_>, kind: &str| match side {
        Side::Named(name) => format!("`{name}`에 담긴 것은 {kind}"),
        Side::Written(text) => {
            format!("`{text}`{} {kind}", korean_particle(text, "은", "는"))
        }
        Side::Unnamed => format!("한쪽은 {kind}"),
    };
    let holder_ko = |side: &Side<'_>, kind: &str| match (shape, side) {
        (Shape::Contains(_), Side::Named(name)) => format!("`{name}` 안은 전부 {kind}"),
        _ => side_ko(side, kind),
    };
    let mut hint_en = format!(
        "{} and {}. Those two never match.",
        holder_en(&left_side, left_en),
        side_en(&right_side, right_en)
    );
    let mut hint_ko = format!(
        "{}이고, {}입니다. {left_ko}{} {right_ko}{} 절대 같아지지 않습니다.",
        holder_ko(&left_side, left_ko),
        side_ko(&right_side, right_ko),
        korean_particle(left_ko, "과", "와"),
        korean_particle(right_ko, "은", "는"),
    );
    // Two pairs have a one-line fix worth naming.
    //
    // A question asked as text and weighed against a number is the commonest
    // slip there is, and `ask number` is the whole answer to it.
    if was_asked
        && ([left, right] == [Kind::Text, Kind::Number]
            || [left, right] == [Kind::Number, Kind::Text])
    {
        hint_en.push_str(" If the answer is meant to be a number, ask for it with `ask number`.");
        hint_ko.push_str(" 답을 숫자로 받으려면 `숫자로 물어봐`로 물어보세요.");
    }
    // A job is the one kind the reader did not choose to put in a name: they
    // wrote `저장하기라는 일:` at the top and `저장하기` in a menu, and the two
    // are the same word by accident. Saying which kinds do not match is true
    // but useless here; what they need is that the word is doing two jobs.
    if left == Kind::Job || right == Kind::Job {
        hint_en.push_str(
            " That word names a job as well as standing in this comparison.              Give the job and the word being compared different names.",
        );
        hint_ko.push_str(
            " 그 낱말은 이 비교에 쓰이면서 일의 이름이기도 합니다.              일 이름과 견주는 낱말을 다른 말로 지으세요.",
        );
    }
    Diagnostic::bilingual(
        DiagnosticCode::ComparisonNeverTrue,
        message_en,
        message_ko,
        span,
    )
    .with_bilingual_hint(hint_en, hint_ko)
}

/* --- what each name holds ------------------------------------------------ */

/// One kind per name, over the whole file rather than line by line.
///
/// Line order would be the sharper reading, but it is also the wrong one for a
/// loop: a name set at the bottom of a `계속 반복해` block is already that kind
/// when the test at the top runs the second time round. Reading the file whole
/// costs a few reports and cannot invent one.
fn name_kinds(source: &str, lines: &[LogicalLine], nme_lines: &[NmeLine]) -> Names {
    let mut names = Names::default();
    let mut spoiled: HashSet<String> = HashSet::new();
    for line in nme_lines {
        read_statement(&line.stmt, source, &mut names, &mut spoiled);
    }
    // A name a plain Python line so much as mentions is beyond this check: one
    // `score = 7` under `점수는 안녕` would turn a correct program into a
    // refused one. Every word of every non-NME line is dropped, which is far
    // more than assignment targets and exactly as cheap.
    let nme_indexes = nme_lines
        .iter()
        .map(|line| line.line_index)
        .collect::<HashSet<_>>();
    for (index, line) in lines.iter().enumerate() {
        if nme_indexes.contains(&index) {
            continue;
        }
        for word in words_of(&source[line.span.start..line.span.end]) {
            spoiled.insert(word);
        }
    }
    for name in spoiled {
        names.kinds.remove(&name);
        names.items.remove(&name);
    }
    names
}

/// Every identifier-shaped run of characters in a piece of source.
fn words_of(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut word = String::new();
    for character in text.chars() {
        if character.is_alphanumeric() || character == '_' {
            word.push(character);
        } else if !word.is_empty() {
            found.push(std::mem::take(&mut word));
        }
    }
    if !word.is_empty() {
        found.push(word);
    }
    found
}

fn read_statement(stmt: &NmeStmt, source: &str, names: &mut Names, spoiled: &mut HashSet<String>) {
    match stmt {
        NmeStmt::Ask { target, kind, .. } => {
            let known = match kind {
                InputKind::Text => Kind::Text,
                InputKind::Number => Kind::Number,
            };
            note(target, Some(known), names, spoiled);
            names.asked.insert(target.clone());
        }
        NmeStmt::Set { target, value } => {
            note(target, value_kind(value, source), names, spoiled);
            // `가방은 목록 밧줄, 램프` says what the whole list holds, and that
            // is the only shape that does. A list built up a line at a time
            // has already put its name out of the check by then.
            if let Value::List(items) = value {
                if let Some(kind) = shared_item_kind(items, source) {
                    names.items.insert(target.clone(), kind);
                }
            }
        }
        // Everything else that writes a name writes something this check does
        // not model. Naming them one by one rather than falling through keeps
        // a new statement kind from quietly joining the list it must not.
        //
        // `Update` is the one that costs reach rather than safety. A counter
        // is almost always written `점수에 1 더해`, and one such line drops
        // `점수` from the check for the whole file — so what this catches is
        // mostly names set once and left alone. Keeping the kind through an
        // update (a number stays a number, text stays text) would widen it a
        // long way, and is the obvious next step; it is left out here so this
        // release carries one rule and not two.
        NmeStmt::Update { target, .. }
        | NmeStmt::Append { target, .. }
        | NmeStmt::Remove { target, .. }
        | NmeStmt::SetItem { target, .. }
        | NmeStmt::RecordPut { target, .. }
        | NmeStmt::RecordRemove { target, .. }
        | NmeStmt::Arrange { target, .. }
        | NmeStmt::Cooldown { target, .. }
        | NmeStmt::WaitForCooldown { target }
        | NmeStmt::FileRead { target, .. } => note(target, None, names, spoiled),
        NmeStmt::ForEach { name, position, .. } => {
            note(name, None, names, spoiled);
            if let Some(position) = position {
                note(position, None, names, spoiled);
            }
        }
        NmeStmt::CountLoop { name, .. } => note(name, None, names, spoiled),
        NmeStmt::Job { name, parameters } => {
            note(name, Some(Kind::Job), names, spoiled);
            for parameter in parameters {
                note(parameter, None, names, spoiled);
            }
        }
        NmeStmt::ModuleImport { names: brought, .. } => {
            for name in brought {
                note(name, None, names, spoiled);
            }
        }
        _ => {}
    }
    if let Some(InlineStmt::Nme(inner)) = inline_of(stmt) {
        read_statement(inner, source, names, spoiled);
    }
}

fn inline_of(stmt: &NmeStmt) -> Option<&InlineStmt> {
    match stmt {
        NmeStmt::Times { inline, .. }
        | NmeStmt::ForEach { inline, .. }
        | NmeStmt::CountLoop { inline, .. }
        | NmeStmt::Forever { inline }
        | NmeStmt::Chance { inline, .. }
        | NmeStmt::When { inline, .. }
        | NmeStmt::While { inline, .. }
        | NmeStmt::ElseIf { inline, .. }
        | NmeStmt::Else { inline } => inline.as_ref(),
        _ => None,
    }
}

/// Records what a name was given, and spoils it the moment two answers differ.
fn note(name: &str, kind: Option<Kind>, names: &mut Names, spoiled: &mut HashSet<String>) {
    // A second answer of any sort about this name puts the earlier reading of
    // its items out of date, whether or not the two kinds agree.
    names.items.remove(name);
    let Some(kind) = kind else {
        spoiled.insert(name.to_string());
        return;
    };
    match names.kinds.get(name) {
        Some(already) if *already != kind => {
            spoiled.insert(name.to_string());
        }
        _ => {
            names.kinds.insert(name.to_string(), kind);
        }
    }
}

/// The one kind every item of a written-out list has, when they agree.
fn shared_item_kind(items: &[Value], source: &str) -> Option<Kind> {
    let mut shared = None;
    for item in items {
        let kind = value_kind(item, source)?;
        match shared {
            Some(already) if already != kind => return None,
            _ => shared = Some(kind),
        }
    }
    shared
}

fn value_kind(value: &Value, source: &str) -> Option<Kind> {
    match value {
        Value::Literal(Literal::True | Literal::False) => Some(Kind::TrueOrFalse),
        Value::Text(_) | Value::Joined { .. } | Value::Repeated { .. } => Some(Kind::Text),
        Value::List(_) => Some(Kind::List),
        Value::Split { .. } => Some(Kind::List),
        Value::EmptyRecord => Some(Kind::Record),
        Value::RandomInteger { .. } => Some(Kind::Number),
        Value::Reading { reading, .. } => reading_kind(*reading),
        Value::Python(code) => code_kind(code, source),
        _ => None,
    }
}

fn reading_kind(reading: Reading) -> Option<Kind> {
    match reading {
        // `친구들 개수` is `len(...)` and `점수들 합` is `sum(...)`; both are
        // numbers whatever the list holds.
        Reading::Count | Reading::Total => Some(Kind::Number),
        Reading::Capitals | Reading::SmallLetters => Some(Kind::Text),
        // The biggest of a list is whatever the list holds, so it is not ours.
        Reading::Largest | Reading::Smallest => None,
    }
}

/// The kind of a piece of Python the writer typed, when it says so plainly.
fn code_kind(code: &Code, source: &str) -> Option<Kind> {
    let text = match code {
        Code::Source(span) => source.get(span.start..span.end)?,
        Code::Generated(text) => text.as_str(),
    }
    .trim();
    if text == "True" || text == "False" {
        return Some(Kind::TrueOrFalse);
    }
    // `f64::from_str` also accepts `inf` and `NaN`, which are ordinary Python
    // names rather than numbers written down, so only finite ones count.
    if text.parse::<i64>().is_ok() || text.parse::<f64>().is_ok_and(f64::is_finite) {
        return Some(Kind::Number);
    }
    let quoted = |mark: char| {
        text.starts_with(mark) && text.ends_with(mark) && text.chars().count() >= 2
        // A second quote inside would make this two pieces joined by
        // something, and that something decides the kind, not the quotes.
            && text[mark.len_utf8()..text.len() - mark.len_utf8()].find(mark).is_none()
    };
    if quoted('"') || quoted('\'') {
        return Some(Kind::Text);
    }
    None
}

fn condition_kind(value: &ConditionValue, source: &str, names: &Names) -> Option<Kind> {
    match value {
        ConditionValue::Python(code) => code_kind(code, source),
        ConditionValue::Name(name) => names.kinds.get(name).copied(),
        ConditionValue::Text(_) => Some(Kind::Text),
        ConditionValue::Literal(Literal::True | Literal::False) => Some(Kind::TrueOrFalse),
        ConditionValue::Reading { reading, .. } => reading_kind(*reading),
        ConditionValue::Remainder { .. }
        | ConditionValue::Quotient { .. }
        | ConditionValue::AsNumber { .. } => Some(Kind::Number),
        _ => None,
    }
}
