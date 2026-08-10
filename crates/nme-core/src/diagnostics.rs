//! Beginner-friendly diagnostics.
//!
//! NME's audience is people who find Python intimidating, so an error
//! message must answer three questions in plain language:
//!
//! 1. *What* is wrong?  (`message`)
//! 2. *Where* exactly?  (`span`, rendered as a caret under the source line)
//! 3. *What should I try instead?*  (`hint`)
//!
//! Diagnostics are plain data. Rendering is separated from reporting so the
//! CLI (and future tools, e.g. an LSP server) can present them differently
//! without touching the compiler.

use std::fmt::Write as _;

use unicode_width::UnicodeWidthChar;

const TAB_WIDTH: usize = 4;

/// A byte range in the original source text (`start..end`, end exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn len(self) -> usize {
        self.end - self.start
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// A stable error code. Codes are assigned once and never reused; new
/// diagnostics append new codes so `nme ko <CODE>` lookups stay valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// The input is neither valid Python nor an NME form the parser knows.
    UnrecognizedInput,
    /// A whole-statement `end`/`끝` with no open NME block.
    StrayEnd,
    /// `break` outside a loop.
    BreakOutsideLoop,
    /// `else`/`elif`/`아니면` with no open condition block.
    BranchWithoutCondition,
    /// A second `else` in one condition.
    DuplicateElse,
    /// A block that never got its closing `end`/`끝`.
    MissingEnd,
    /// `say` value could not be understood.
    SayValueBroken,
    /// `say` expression is not valid.
    SayValueUnparseable,
    /// The sentence to show could not be understood.
    SaySentenceUnparseable,
    /// `say` with nothing to show.
    SayMissing,
    /// `ask ..., ` with an empty question.
    AskQuestionMissing,
    /// The question itself could not be understood.
    AskQuestionUnparseable,
    /// `ask` target is not a usable variable name.
    AskTargetInvalid,
    /// A value change (`x add 1`) could not be understood.
    UpdateUnparseable,
    /// A sentence `break` command could not be understood.
    BreakCommandUnparseable,
    /// `if`/`while`/`만약` with no condition.
    ConditionMissing,
    /// The condition could not be understood.
    ConditionInvalid,
    /// `times` body could not be understood.
    RepeatBodyUnparseable,
    /// `times` count could not be understood.
    RepeatCountUnparseable,
    /// `times` with no count.
    RepeatCountMissing,
    /// `use` line asks for a module NME does not bundle.
    UnsupportedModule,
    /// `use random latest` and an exact version on one line.
    LatestAndVersion,
    /// `use random version` with no version value.
    ModuleVersionMissing,
    /// A version number NME does not bundle.
    UnbundledVersion,
    /// `use random` would overwrite existing names.
    RandomNameCollision,
    /// The `use` line shape is not understood.
    ModuleShapeInvalid,
    /// `set`/`save` with no value.
    SaveValueMissing,
    /// The value to save could not be understood.
    SaveValueUnparseable,
    /// `set`/`save` with no target name.
    SaveNameMissing,
    /// The save target is not a simple variable name.
    SaveNameNotSimple,
    /// A repeated block body without indentation.
    IndentationRequired,
    /// An inline condition without `:`.
    ColonRequired,
    /// A block that starts without a statement.
    BlockWithoutStatement,
    /// More than one statement on one physical line.
    OneStatementPerLine,
    /// The block body is not a statement the parser knows.
    BodyUnparseable,
    /// The sentence could mean more than one action.
    AmbiguousAction,
    /// No known action found on the line.
    MissingAction,
    /// A sentence-style statement across several physical lines.
    MultilineSentence,
    /// The Python source given to the converter is not valid.
    ConvertInvalidPython,
}

impl DiagnosticCode {
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnrecognizedInput => "E0001",
            Self::StrayEnd => "E0101",
            Self::BreakOutsideLoop => "E0102",
            Self::BranchWithoutCondition => "E0103",
            Self::DuplicateElse => "E0104",
            Self::MissingEnd => "E0105",
            Self::SayValueBroken => "E0201",
            Self::SayValueUnparseable => "E0202",
            Self::SaySentenceUnparseable => "E0203",
            Self::SayMissing => "E0204",
            Self::AskQuestionMissing => "E0211",
            Self::AskQuestionUnparseable => "E0212",
            Self::AskTargetInvalid => "E0213",
            Self::UpdateUnparseable => "E0221",
            Self::BreakCommandUnparseable => "E0222",
            Self::ConditionMissing => "E0301",
            Self::ConditionInvalid => "E0302",
            Self::RepeatBodyUnparseable => "E0303",
            Self::RepeatCountUnparseable => "E0304",
            Self::RepeatCountMissing => "E0305",
            Self::UnsupportedModule => "E0401",
            Self::LatestAndVersion => "E0402",
            Self::ModuleVersionMissing => "E0403",
            Self::UnbundledVersion => "E0404",
            Self::RandomNameCollision => "E0405",
            Self::ModuleShapeInvalid => "E0406",
            Self::SaveValueMissing => "E0411",
            Self::SaveValueUnparseable => "E0412",
            Self::SaveNameMissing => "E0413",
            Self::SaveNameNotSimple => "E0414",
            Self::IndentationRequired => "E0501",
            Self::ColonRequired => "E0502",
            Self::BlockWithoutStatement => "E0503",
            Self::OneStatementPerLine => "E0504",
            Self::BodyUnparseable => "E0505",
            Self::AmbiguousAction => "E0601",
            Self::MissingAction => "E0602",
            Self::MultilineSentence => "E0701",
            Self::ConvertInvalidPython => "E0702",
        }
    }

    /// All codes in display order (the order of the enum above).
    pub const ALL: [DiagnosticCode; 39] = [
        Self::UnrecognizedInput,
        Self::StrayEnd,
        Self::BreakOutsideLoop,
        Self::BranchWithoutCondition,
        Self::DuplicateElse,
        Self::MissingEnd,
        Self::SayValueBroken,
        Self::SayValueUnparseable,
        Self::SaySentenceUnparseable,
        Self::SayMissing,
        Self::AskQuestionMissing,
        Self::AskQuestionUnparseable,
        Self::AskTargetInvalid,
        Self::UpdateUnparseable,
        Self::BreakCommandUnparseable,
        Self::ConditionMissing,
        Self::ConditionInvalid,
        Self::RepeatBodyUnparseable,
        Self::RepeatCountUnparseable,
        Self::RepeatCountMissing,
        Self::UnsupportedModule,
        Self::LatestAndVersion,
        Self::ModuleVersionMissing,
        Self::UnbundledVersion,
        Self::RandomNameCollision,
        Self::ModuleShapeInvalid,
        Self::SaveValueMissing,
        Self::SaveValueUnparseable,
        Self::SaveNameMissing,
        Self::SaveNameNotSimple,
        Self::IndentationRequired,
        Self::ColonRequired,
        Self::BlockWithoutStatement,
        Self::OneStatementPerLine,
        Self::BodyUnparseable,
        Self::AmbiguousAction,
        Self::MissingAction,
        Self::MultilineSentence,
        Self::ConvertInvalidPython,
    ];

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|candidate| candidate.code() == code)
    }
}

/// Long-form bilingual explanation of one error code, for `nme ko <CODE>` /
/// `nme en <CODE>` lookup pages. Plain language for beginners, with the
/// recovery steps written out.
#[derive(Debug, Clone, Copy)]
pub struct CodeExplanation {
    pub code: &'static str,
    pub title_en: &'static str,
    pub title_ko: &'static str,
    pub detail_en: &'static str,
    pub detail_ko: &'static str,
}

impl DiagnosticCode {
    pub fn explanation(self) -> CodeExplanation {
        let (code, title_en, title_ko, detail_en, detail_ko) = match self {
            Self::UnrecognizedInput => (
                "E0001",
                "this line is not valid Python or NME",
                "이 줄은 올바른 Python도 NME도 아닙니다",
                "NME first checks whether a line is ordinary Python; then it looks for known NME forms. This line is neither. Check for a missing quote or bracket, or for an unfinished sentence.",
                "NME는 먼저 줄이 일반 Python인지 확인하고, 그다음 알고 있는 NME 형식을 찾습니다. 이 줄은 둘 다 아닙니다. 따옴표나 괄호가 빠졌는지, 문장이 끝나지 않았는지 확인하세요.",
            ),
            Self::StrayEnd => (
                "E0101",
                "an `end` with no open block",
                "열린 블록이 없는 `끝`",
                "Every `end` (or `끝`) must close a block that is still open. This one appears after every block has already been closed, or before any block started.",
                "모든 `end`(`끝`)는 아직 열려 있는 블록을 닫아야 합니다. 여기의 `끝`은 모든 블록이 이미 닫힌 뒤이거나, 블록이 시작되기 전에 나왔습니다.",
            ),
            Self::BreakOutsideLoop => (
                "E0102",
                "`break` outside a loop",
                "반복문 밖의 `break`",
                "`break` (or `멈춰`) stops the nearest loop, so it can only be written inside a `while` loop. Move it inside the loop, or remove it.",
                "`break`(`멈춰`)는 가장 가까운 반복문을 멈추므로 `while` 반복문 안에서만 쓸 수 있습니다. 반복문 안으로 옮기거나 지우세요.",
            ),
            Self::BranchWithoutCondition => (
                "E0103",
                "an `else` or `elif` with no open condition",
                "열린 조건이 없는 `아니면`",
                "`else`, `else if`, `elif`, `아니면` are branches of an `if` (or `만약`) block. Write the `if` line first and keep the `else` inside the same block, before its `end`.",
                "`else`, `else if`, `elif`, `아니면`은 `if`(`만약`) 블록의 가지입니다. 먼저 `if` 줄을 쓰고, `else`는 같은 블록 안에서 `끝`보다 앞에 두세요.",
            ),
            Self::DuplicateElse => (
                "E0104",
                "two `else` branches in one condition",
                "한 조건에 `else`가 두 번",
                "A condition can only have one `else` (or `아니면`) branch. Turn the second one into an `else if`/`elif`/`아니면 만약에` branch instead.",
                "조건에는 `else`(`아니면`) 가지가 하나만 올 수 있습니다. 두 번째는 `else if`/`elif`/`아니면 만약에` 가지로 바꾸세요.",
            ),
            Self::MissingEnd => (
                "E0105",
                "a block without its closing `end`",
                "닫는 `end`가 없는 블록",
                "This loop or condition block is still open at the end of the file. Add a closing `end` (or `끝`) line after the block's last line.",
                "이 반복문·조건 블록이 파일 끝까지 열려 있습니다. 블록의 마지막 줄 뒤에 닫는 `end`(`끝`) 줄을 추가하세요.",
            ),
            Self::SayValueBroken => (
                "E0201",
                "`say` value could not be understood",
                "`say`의 값을 이해하지 못했습니다",
                "After `say` (or `말해`) NME expects a value to show: text, a number, a known variable, or a simple expression. The words here do not form any of those.",
                "`say`(`말해`) 뒤에는 보여 줄 값이 와야 합니다: 글자, 숫자, 알고 있는 변수, 또는 간단한 식. 여기의 단어들은 그 중 어떤 형태도 아닙니다.",
            ),
            Self::SayValueUnparseable => (
                "E0202",
                "the `say` expression is not valid",
                "`say`의 식이 올바르지 않습니다",
                "The value after `say` is a name or expression Python cannot read. Check for missing quotes around text and correct variable names.",
                "`say` 뒤의 값은 Python이 읽지 못하는 이름이나 식입니다. 글자 주변의 따옴표와 변수 이름을 확인하세요.",
            ),
            Self::SaySentenceUnparseable => (
                "E0203",
                "the sentence to show is not valid",
                "보여 줄 문장이 올바르지 않습니다",
                "NME could not turn this sentence into something Python can print. Known variable names can be mixed into the sentence; everything else must be plain text.",
                "NME가 이 문장을 Python으로 출력할 수 있는 형태로 바꾸지 못했습니다. 알고 있는 변수 이름은 문장에 섞일 수 있고, 나머지는 일반 글자여야 합니다.",
            ),
            Self::SayMissing => (
                "E0204",
                "`say` has nothing to show",
                "`say`에 보여 줄 내용이 없습니다",
                "`say` (or `말해`) needs something after it. Write `say Hello` or `say \"Hello\"`.",
                "`say`(`말해`) 뒤에 내용이 필요합니다. `say Hello` 또는 `say \"Hello\"`처럼 적으세요.",
            ),
            Self::AskQuestionMissing => (
                "E0211",
                "the question after the comma is missing",
                "쉼표 뒤의 질문이 비어 있습니다",
                "An `ask` with a comma needs a question after it: `ask name, What is your name?`. Add the question or remove the comma.",
                "쉼표가 있는 `ask`는 뒤에 질문이 필요합니다: `ask name, What is your name?`. 질문을 추가하거나 쉼표를 지우세요.",
            ),
            Self::AskQuestionUnparseable => (
                "E0212",
                "the question could not be understood",
                "질문을 이해하지 못했습니다",
                "NME could not turn this `ask` question into something Python can print. Keep it as plain text, optionally with a known variable name.",
                "NME가 이 `ask` 질문을 Python으로 출력할 수 있는 형태로 바꾸지 못했습니다. 일반 글자로 쓰고, 원하면 알고 있는 변수 이름을 섞으세요.",
            ),
            Self::AskTargetInvalid => (
                "E0213",
                "the `ask` target is not a variable name",
                "`ask`의 대상이 변수 이름이 아닙니다",
                "`ask name, ...` saves the answer into `name`. Write a simple variable name there (letters and numbers, starting with a letter), not a sentence or a keyword.",
                "`ask name, ...`는 답을 `name`에 저장합니다. 문장이나 키워드가 아니라 간단한 변수 이름(문자로 시작하는 문자·숫자 조합)을 적으세요.",
            ),
            Self::UpdateUnparseable => (
                "E0221",
                "the value change could not be understood",
                "값 변경을 이해하지 못했습니다",
                "A value change looks like `score add 1` or `점수에 1 더해` or `점수는 점수 + 1`. Check the target name, the action word, and the amount.",
                "값 변경은 `score add 1`, `점수에 1 더해`, `점수는 점수 + 1` 같은 형태입니다. 대상 이름, 동작 단어, 그리고 값이 맞는지 확인하세요.",
            ),
            Self::BreakCommandUnparseable => (
                "E0222",
                "the break command could not be understood",
                "break 명령을 이해하지 못했습니다",
                "Inside a loop, write `break here`, `멈춰`, or just `break`. Nothing else belongs on that line.",
                "반복문 안에서 `break here`, `멈춰` 또는 그냥 `break`라고 쓰세요. 그 줄에는 다른 내용이 들어가지 않아야 합니다.",
            ),
            Self::ConditionMissing => (
                "E0301",
                "the condition is missing",
                "조건이 비어 있습니다",
                "`if`/`when`/`while`/`만약` needs a condition after it, for example `if score is greater than 10` or `만약 점수가 10보다 크면`.",
                "`if`/`when`/`while`/`만약` 뒤에는 조건이 필요합니다. 예: `if score is greater than 10`, `만약 점수가 10보다 크면`.",
            ),
            Self::ConditionInvalid => (
                "E0302",
                "the condition could not be understood",
                "조건을 이해하지 못했습니다",
                "NME could not read this condition. Use a comparison such as `is less than`, `equals`, `!=`, `보다 크면`, `와 같으면`, joined with `and`/`or` (`그리고`/`또는`).",
                "NME가 이 조건을 읽지 못했습니다. `is less than`, `equals`, `!=`, `보다 크면`, `와 같으면` 같은 비교를 `and`/`or`(`그리고`/`또는`)로 연결해 쓰세요.",
            ),
            Self::RepeatBodyUnparseable => (
                "E0303",
                "the repeated body could not be understood",
                "반복할 내용을 이해하지 못했습니다",
                "After `3 times:` (or `3 번:`) the body line must be an NME or Python statement. Write the body on its own indented line instead of sharing the header line.",
                "`3 times:`(`3 번:`) 뒤의 본문 줄은 NME 또는 Python 문장이어야 합니다. 헤더 줄에 함께 쓰지 말고 본문을 따로 들여쓴 줄에 쓰세요.",
            ),
            Self::RepeatCountUnparseable => (
                "E0304",
                "the repeat count could not be understood",
                "반복 횟수를 이해하지 못했습니다",
                "`N times:` needs a number for N, for example `3 times:` or `3 번:`. Words and expressions are not accepted here.",
                "`N times:`의 N 자리에는 숫자가 필요합니다. 예: `3 times:`, `3 번:`. 이 자리에 단어나 식은 쓸 수 없습니다.",
            ),
            Self::RepeatCountMissing => (
                "E0305",
                "the repeat count is missing",
                "반복 횟수가 비어 있습니다",
                "`times:` needs a count before the colon. Write the number of repetitions, for example `3 times: show Hello`.",
                "`times:`는 콜론 앞에 횟수가 필요합니다. 반복할 횟수를 적으세요. 예: `3 times: show Hello`.",
            ),
            Self::UnsupportedModule => (
                "E0401",
                "NME only bundles `use random`",
                "NME에는 `랜덤 사용`만 내장되어 있습니다",
                "NME ships a small set of beginner modules. `random` (or `랜덤`) is the only one so far. Anything else is a Python import: write `import name` on its own line.",
                "NME는 초보자용 모듈을 소수만 제공합니다. 현재는 `random`(`랜덤`)뿐입니다. 다른 것은 Python import로 쓸 수 있습니다: `import name`을 한 줄로 쓰세요.",
            ),
            Self::LatestAndVersion => (
                "E0402",
                "latest and an exact version on one line",
                "최신과 정확한 버전을 함께 적었습니다",
                "A `use random` line either asks for the newest bundled version (`use random latest`) or for one exact version (`use random version \"0.0.1\"`), never both.",
                "`use random` 줄은 내장 버전 중 최신(`use random latest`) 또는 정확한 버전 하나(`use random version \"0.0.1\"`) 중 하나만 요청할 수 있습니다. 둘은 함께 쓸 수 없습니다.",
            ),
            Self::ModuleVersionMissing => (
                "E0403",
                "the module version is missing",
                "모듈 버전이 비어 있습니다",
                "`use random version` needs a version number after it, for example `use random version \"0.0.1\"`.",
                "`use random version` 뒤에는 버전 번호가 필요합니다. 예: `use random version \"0.0.1\"`.",
            ),
            Self::UnbundledVersion => (
                "E0404",
                "this module version is not bundled",
                "내장되어 있지 않은 모듈 버전입니다",
                "NME only ships specific versions of `random`. Run `nme modules` to see the bundled version, then write `use random version \"<that version>\"` or simply `use random latest`.",
                "NME는 `random`의 특정 버전만 제공합니다. `nme 모듈`로 내장 버전을 확인한 뒤 `use random version \"<그 버전>\"` 또는 그냥 `use random latest`라고 쓰세요.",
            ),
            Self::RandomNameCollision => (
                "E0405",
                "the random module would overwrite your names",
                "random 모듈이 기존 이름을 덮어씁니다",
                "The bundled random adapter needs helper names such as `random_number` and `랜덤선택`. One of those names is already in use in this file. Rename your variable or import the adapter before using that name.",
                "내장 random 어댑터는 `random_number`, `랜덤선택` 같은 도우미 이름을 사용합니다. 그 이름 중 하나가 이 파일에서 이미 쓰이고 있습니다. 변수 이름을 바꾸거나 그 이름을 쓰기 전에 어댑터를 가져오세요.",
            ),
            Self::ModuleShapeInvalid => (
                "E0406",
                "the use line shape is not understood",
                "use 줄의 형태를 이해하지 못했습니다",
                "Write the module line as `use random` (or `랜덤 사용`), optionally with `latest` or `version \"0.0.1\"`. Other word orders are not accepted.",
                "모듈 줄은 `use random`(`랜덤 사용`) 형태로 쓰고, 원하면 `latest`나 `version \"0.0.1\"`를 붙이세요. 다른 단어 순서는 받아들이지 않습니다.",
            ),
            Self::SaveValueMissing => (
                "E0411",
                "the value to save is missing",
                "저장할 값이 비어 있습니다",
                "`set x to ...` (or `x은 ...`) needs a value after it: a number, text, or an expression. Add the value or remove the line.",
                "`set x to ...`(`x은 ...`) 뒤에는 값이 필요합니다: 숫자, 글자, 또는 식. 값을 추가하거나 줄을 지우세요.",
            ),
            Self::SaveValueUnparseable => (
                "E0412",
                "the value to save could not be understood",
                "저장할 값을 이해하지 못했습니다",
                "NME could not turn the saved value into a Python expression. Check quotes around text, number spelling, and known variable names.",
                "NME가 저장할 값을 Python 식으로 바꾸지 못했습니다. 글자의 따옴표, 숫자 표기, 변수 이름을 확인하세요.",
            ),
            Self::SaveNameMissing => (
                "E0413",
                "the name to save into is missing",
                "저장할 이름이 비어 있습니다",
                "`set x to 3` saves into `x`. Write a variable name after `set` (or before `은`/`는`).",
                "`set x to 3`은 `x`에 저장합니다. `set` 뒤(`은`/`는` 앞)에 변수 이름을 쓰세요.",
            ),
            Self::SaveNameNotSimple => (
                "E0414",
                "the save target is not a simple name",
                "저장 대상이 단순한 이름이 아닙니다",
                "NME can only save into a simple variable name (letters and numbers, starting with a letter). Attributes like `x.count` stay Python: write them as a Python line.",
                "NME는 간단한 변수 이름(문자로 시작하는 문자·숫자 조합)에만 저장할 수 있습니다. `x.count` 같은 속성은 Python 줄로 그대로 쓰세요.",
            ),
            Self::IndentationRequired => (
                "E0501",
                "the repeated block is not indented",
                "반복할 블록이 들여쓰지 않았습니다",
                "The body lines of `N times:` must be indented (press Tab or four spaces), or the block can use a sentence-level form with `end`/`끝`. NME never guesses which lines belong to the block.",
                "`N times:`의 본문 줄은 들여쓰기(Tab 또는 스페이스 4칸)해야 합니다. 또는 `end`/`끝`을 쓰는 문장형 블록을 사용하세요. NME는 어떤 줄이 블록에 속하는지 추측하지 않습니다.",
            ),
            Self::ColonRequired => (
                "E0502",
                "the condition needs a colon",
                "조건 뒤에 콜론이 필요합니다",
                "A beginner `if`/`while` header with no indented body needs `:` to stay valid Python, or use the sentence form that ends with `end`/`끝`.",
                "본문이 들여쓰기로 없는 초급형 `if`/`while` 헤더는 `:`가 있어야 올바른 Python이 됩니다. 또는 `end`/`끝`으로 끝나는 문장형을 쓰세요.",
            ),
            Self::BlockWithoutStatement => (
                "E0503",
                "a block that starts without a statement",
                "문장 없이 시작하는 블록",
                "A line like `if ...:` or `3 times:` must be followed by the statement it applies to. Write the body on the next line, or on the same line after a space.",
                "`if ...:`나 `3 times:` 같은 줄 뒤에는 적용할 문장이 와야 합니다. 다음 줄에 쓰거나, 한 칸 띄고 같은 줄에 쓰세요.",
            ),
            Self::OneStatementPerLine => (
                "E0504",
                "one statement per line",
                "한 줄에 문장 하나",
                "NME sentences must stay on one physical line. Split the line into two sentences or keep the Python version (`;` is Python, not NME).",
                "NME 문장은 한 줄에 하나여야 합니다. 두 문장으로 나누거나 Python 형태(`;`)로 쓰세요.",
            ),
            Self::BodyUnparseable => (
                "E0505",
                "the block body is not a statement NME knows",
                "블록 본문이 NME가 아는 문장이 아닙니다",
                "The indented body of this block is neither ordinary Python nor an NME form. Check the spelling of the sentence, or write it as Python.",
                "이 블록의 들여쓴 본문은 일반 Python도 NME 형식도 아닙니다. 문장 철자를 확인하거나 Python으로 쓰세요.",
            ),
            Self::AmbiguousAction => (
                "E0601",
                "the sentence could mean more than one action",
                "여러 동작으로 해석될 수 있는 문장입니다",
                "More than one NME action (say, ask, save, ...) fits this sentence, and NME never guesses. Rewrite the sentence with the action word spelled exactly, for example `say Hello`.",
                "이 문장에 여러 NME 동작(say, ask, 저장 등)이 맞을 수 있습니다. NME는 추측하지 않습니다. `say Hello`처럼 동작 단어를 정확히 적어 다시 쓰세요.",
            ),
            Self::MissingAction => (
                "E0602",
                "no NME action was found on this line",
                "이 줄에서 NME 동작을 찾지 못했습니다",
                "This line is not valid Python and does not start with a known NME action. Start with a documented action word such as `show`, `say`, `ask`, `말해`, `물어봐`, or write the line as Python.",
                "이 줄은 올바른 Python도 아니고 알려진 NME 동작으로 시작하지도 않습니다. `show`, `say`, `ask`, `말해`, `물어봐` 같은 문서에 있는 동작 단어로 시작하거나 Python으로 쓰세요.",
            ),
            Self::MultilineSentence => (
                "E0701",
                "a sentence-style line across several physical lines",
                "여러 줄에 걸친 문장형 줄",
                "NME sentences must fit on one physical line so error line numbers stay exact. Join the sentence into one line.",
                "오류 줄 번호를 정확히 유지하기 위해 NME 문장은 한 줄에 맞아야 합니다. 문장을 한 줄로 합치세요.",
            ),
            Self::ConvertInvalidPython => (
                "E0702",
                "the Python source is not valid",
                "Python 소스가 올바르지 않습니다",
                "`nme convert` only rewrites valid Python. Fix the syntax error shown in the message, then convert again.",
                "`nme 변환`은 올바른 Python만 변환합니다. 메시지에 표시된 문법 오류를 고친 뒤 다시 변환하세요.",
            ),
        };
        CodeExplanation {
            code,
            title_en,
            title_ko,
            detail_en,
            detail_ko,
        }
    }
}

/// One problem found in NME source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Stable error code for `nme ko <CODE>` lookups.
    pub code: DiagnosticCode,
    /// What is wrong in English, in plain language. No compiler jargon.
    pub message: String,
    /// The same message in Korean when this is a user-facing diagnostic.
    pub message_ko: Option<String>,
    /// Where it is wrong.
    pub span: Span,
    /// What to try instead in English, if we know.
    pub hint: Option<String>,
    /// The same hint in Korean when this is a user-facing diagnostic.
    pub hint_ko: Option<String>,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            message: message.into(),
            message_ko: None,
            span,
            hint: None,
            hint_ko: None,
        }
    }

    /// Creates one diagnostic with equivalent English and Korean messages.
    pub fn bilingual(
        code: DiagnosticCode,
        message: impl Into<String>,
        message_ko: impl Into<String>,
        span: Span,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            message_ko: Some(message_ko.into()),
            span,
            hint: None,
            hint_ko: None,
        }
    }

    #[must_use]
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Adds equivalent English and Korean recovery advice.
    #[must_use]
    pub fn with_bilingual_hint(
        mut self,
        hint: impl Into<String>,
        hint_ko: impl Into<String>,
    ) -> Self {
        self.hint = Some(hint.into());
        self.hint_ko = Some(hint_ko.into());
        self
    }

    /// 1-based `(line, column)` of the start of the span in `source`.
    pub fn line_col(&self, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (offset, ch) in source.char_indices() {
            if offset >= self.span.start {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    /// Renders the diagnostic in a friendly, rustc-inspired format:
    ///
    /// ```text
    /// error[E0204]: `say` needs something to print
    ///  --> hello.nme:2:1
    ///   |
    /// 2 | say
    ///   | ^^^
    ///   = hint: try `say "Hello"`
    /// ```
    pub fn render(&self, source: &str, path: &str) -> String {
        self.render_with_korean(source, path, false)
    }

    /// Renders Korean first and then the equivalent English when available.
    /// Source locations and carets are shared, so the beginner sees one
    /// problem rather than two duplicate diagnostics.
    pub fn render_bilingual(&self, source: &str, path: &str) -> String {
        self.render_with_korean(source, path, true)
    }

    fn render_with_korean(&self, source: &str, path: &str, bilingual: bool) -> String {
        let (line_no, col) = self.line_col(source);
        let line_text = source_line(source, line_no);
        let rendered_line = expand_tabs(line_text);
        // The underline covers the span, but never spills past the line and
        // is always at least one caret wide so zero-width spans are visible.
        let line_start = source_line_start(source, line_no);
        let line_end = line_start + line_text.len();
        let span_start = self.span.start.clamp(line_start, line_end);
        let span_end = self.span.end.clamp(span_start, line_end);
        let local_start = floor_char_boundary(line_text, span_start - line_start);
        let raw_end = span_end - line_start;
        let local_end = if raw_end == span_start - line_start {
            local_start
        } else {
            ceil_char_boundary(line_text, raw_end)
        };
        let prefix = &line_text[..local_start];
        let highlighted = &line_text[local_start..local_end];
        let underline_start = display_width(prefix, 0);
        let underline_len = display_width(highlighted, underline_start).max(1);

        let mut out = String::new();
        let gutter = line_no.to_string().len();
        if bilingual {
            if let Some(message_ko) = &self.message_ko {
                let _ = writeln!(out, "오류[{}]: {message_ko}", self.code.code());
            }
        }
        let _ = writeln!(out, "error[{}]: {}", self.code.code(), self.message);
        let _ = writeln!(out, "{:>gutter$} --> {path}:{line_no}:{col}", "");
        let _ = writeln!(out, "{:>gutter$} |", "");
        let _ = writeln!(out, "{line_no:>gutter$} | {rendered_line}");
        let _ = writeln!(
            out,
            "{:>gutter$} | {:width$}{}",
            "",
            "",
            "^".repeat(underline_len),
            width = underline_start
        );
        if bilingual {
            if let Some(hint_ko) = &self.hint_ko {
                let _ = writeln!(out, "{:>gutter$} = 도움말: {hint_ko}", "");
            }
        }
        if let Some(hint) = &self.hint {
            let _ = writeln!(out, "{:>gutter$} = hint: {hint}", "");
        }
        out
    }
}

/// Renders several diagnostics, separated by blank lines.
pub fn render_all(diagnostics: &[Diagnostic], source: &str, path: &str) -> String {
    diagnostics
        .iter()
        .map(|d| d.render(source, path))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Renders several diagnostics with Korean first and English immediately
/// after it, separated by blank lines.
pub fn render_all_bilingual(diagnostics: &[Diagnostic], source: &str, path: &str) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render_bilingual(source, path))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns physical line `line_no` (1-based) without the trailing newline,
/// or an empty string when the line does not exist.
fn source_line(source: &str, line_no: usize) -> &str {
    source
        .lines()
        .nth(line_no - 1)
        .unwrap_or("")
        .trim_end_matches('\r')
}

fn source_line_start(source: &str, line_no: usize) -> usize {
    if line_no <= 1 {
        return 0;
    }
    source
        .char_indices()
        .filter_map(|(offset, character)| (character == '\n').then_some(offset + 1))
        .nth(line_no - 2)
        .unwrap_or(source.len())
}

fn floor_char_boundary(text: &str, mut offset: usize) -> usize {
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn ceil_char_boundary(text: &str, mut offset: usize) -> usize {
    while offset < text.len() && !text.is_char_boundary(offset) {
        offset += 1;
    }
    offset
}

fn display_width(text: &str, starting_column: usize) -> usize {
    let mut column = starting_column;
    for character in text.chars() {
        column += if character == '\t' {
            TAB_WIDTH - (column % TAB_WIDTH)
        } else {
            character.width().unwrap_or(0)
        };
    }
    column - starting_column
}

fn expand_tabs(text: &str) -> String {
    let mut expanded = String::with_capacity(text.len());
    let mut column = 0;
    for character in text.chars() {
        if character == '\t' {
            let spaces = TAB_WIDTH - (column % TAB_WIDTH);
            expanded.push_str(&" ".repeat(spaces));
            column += spaces;
        } else {
            expanded.push(character);
            column += character.width().unwrap_or(0);
        }
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn underline(rendered: &str) -> &str {
        rendered
            .lines()
            .find(|line| line.contains('^'))
            .and_then(|line| line.split_once("| "))
            .map(|(_, underline)| underline)
            .expect("rendered diagnostic should contain an underline")
    }

    #[test]
    fn renders_line_column_and_caret() {
        let source = "say \"hi\"\nsay\n";
        let diag = Diagnostic::new(DiagnosticCode::SayMissing, "`say` needs something to print", Span::new(9, 12))
            .with_hint("try `say \"Hello\"`");
        let rendered = diag.render(source, "hello.nme");
        assert!(rendered.contains("error[E0204]: `say` needs something to print"));
        assert!(rendered.contains("hello.nme:2:1"));
        assert!(rendered.contains("2 | say"));
        assert!(rendered.contains("^^^"));
        assert!(rendered.contains("hint: try `say \"Hello\"`"));
    }

    #[test]
    fn bilingual_rendering_puts_korean_before_equivalent_english() {
        let source = "말해\n";
        let diag = Diagnostic::bilingual(
            DiagnosticCode::SayMissing,
            "there is nothing to show",
            "말할 내용이 비어 있어요",
            Span::new(0, 6),
        )
        .with_bilingual_hint("write `show Hello`", "`안녕하세요 말해줘`처럼 적어 주세요");
        let rendered = diag.render_bilingual(source, "hello.nme");

        let korean_at = rendered.find("오류[E0204]: 말할 내용이 비어 있어요").unwrap();
        let english_at = rendered.find("error[E0204]: there is nothing to show").unwrap();
        assert!(korean_at < english_at, "{rendered}");
        assert!(rendered.contains("도움말: `안녕하세요 말해줘`처럼 적어 주세요"));
        assert!(rendered.contains("hint: write `show Hello`"));
        assert!(!diag.render(source, "hello.nme").contains("오류["));
    }

    #[test]
    fn line_col_counts_from_one() {
        let source = "ab\ncd\nef";
        let diag = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(6, 8));
        assert_eq!(diag.line_col(source), (3, 1));
    }

    #[test]
    fn ascii_caret_uses_character_width() {
        let source = "show value\n";
        let start = source.find("value").unwrap();
        let rendered = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(start, start + "value".len()))
            .render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^^^^^");
    }

    #[test]
    fn cjk_caret_uses_display_cell_width() {
        let source = "말해 잘못된값\n";
        let start = source.find("잘못된값").unwrap();
        let rendered = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(start, start + "잘못된값".len()))
            .render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^^^^^^^^");
    }

    #[test]
    fn tabs_are_expanded_to_four_column_stops() {
        let source = "a\tbroken\n";
        let start = source.find("broken").unwrap();
        let rendered = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(start, start + "broken".len()))
            .render(source, "hello.nme");

        assert!(rendered.contains("1 | a   broken"));
        assert_eq!(underline(&rendered), "    ^^^^^^");
    }

    #[test]
    fn tab_inside_span_uses_its_expanded_width() {
        let source = "a\tb\n";
        let rendered = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(1, 3)).render(source, "hello.nme");

        assert_eq!(underline(&rendered), " ^^^^");
    }

    #[test]
    fn zero_width_span_has_one_caret() {
        let source = "say\n";
        let rendered = Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(3, 3)).render(source, "hello.nme");

        assert_eq!(underline(&rendered), "   ^");
    }

    #[test]
    fn partial_unicode_byte_span_covers_the_whole_character() {
        let source = "show …\n";
        let start = source.find('…').unwrap();
        let rendered =
            Diagnostic::new(DiagnosticCode::MissingAction, "x", Span::new(start, start + 1))
                .render(source, "hello.nme");

        assert_eq!(underline(&rendered), "     ^");
    }

    #[test]
    fn every_code_has_a_unique_stable_number() {
        let mut seen = std::collections::HashSet::new();
        for code in DiagnosticCode::ALL {
            assert!(seen.insert(code.code()), "duplicate code {}", code.code());
            assert!(code.code().starts_with('E'), "{}", code.code());
            assert!(code.code().len() == 5, "{}", code.code());
        }
        assert!(DiagnosticCode::ALL.len() >= 30);
    }

    #[test]
    fn every_code_has_a_bilingual_explanation() {
        for code in DiagnosticCode::ALL {
            let explanation = code.explanation();
            assert_eq!(explanation.code, code.code());
            assert!(!explanation.title_en.is_empty(), "{}", code.code());
            assert!(!explanation.title_ko.is_empty(), "{}", code.code());
            assert!(!explanation.detail_en.is_empty(), "{}", code.code());
            assert!(!explanation.detail_ko.is_empty(), "{}", code.code());
        }
    }

    #[test]
    fn codes_round_trip_through_from_code() {
        for code in DiagnosticCode::ALL {
            assert_eq!(
                DiagnosticCode::from_code(code.code()),
                Some(code),
                "{}",
                code.code()
            );
        }
        assert_eq!(DiagnosticCode::from_code("E9999"), None);
    }
}
