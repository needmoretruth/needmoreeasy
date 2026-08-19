//! The three NME syntax levels.
//!
//! * **Advanced** is ordinary Python and is always left byte-identical.
//! * **Beginner** is the compact bilingual syntax (`say`, `말해`, `times:`).
//! * **Sentence** accepts conversational Korean and English without requiring
//!   quotes, commas, parentheses, or colons for the common first-program tasks.
//!
//! The levels are not modes. A source file may mix all three, and it may mix
//! Korean and English words on every line. The parser still applies the
//! Python-wins rule before considering either easier level.

use crate::diagnostics::Span;

pub const SAY_KEYWORD: &str = "say";
pub const SAY_KEYWORD_KO: &str = "말해";
pub(crate) const SAY_WORDS_EN: &[&str] = &["say", "show", "display", "tell", "print"];
pub const ASK_KEYWORD: &str = "ask";
pub const ASK_KEYWORD_KO: &str = "물어봐";
pub const TIMES_KEYWORD: &str = "times";
pub const TIMES_KEYWORD_KO: &str = "번";
pub const WHEN_KEYWORD: &str = "when";
pub const WHEN_KEYWORD_KO: &str = "만약";
pub const USE_KEYWORD: &str = "use";
pub const RANDOM_MODULE: &str = "random";
pub const RANDOM_MODULE_KO: &str = "랜덤";
pub const FILE_MODULE: &str = "file";
pub const FILE_MODULE_KO: &str = "파일";
pub const ZERO_KNOWLEDGE_MODULE: &str = "zero_knowledge";
pub const ZERO_KNOWLEDGE_MODULE_KO: &str = "영지식";
/// The four modules a beginner reaches for first. Every one of their names is
/// an ordinary word in both languages, which is why the parser only reads one
/// as a module when it stands directly beside the `use`/`사용` word and
/// nothing else on the line is left over. See `match_use_module`.
pub const LIST_MODULE: &str = LIST_KEYWORD;
pub const LIST_MODULE_KO: &str = LIST_KEYWORD_KO;
pub const TEXT_MODULE: &str = "text";
pub const TEXT_MODULE_KO: &str = "글자";
pub const MATH_MODULE: &str = "math";
pub const MATH_MODULE_KO: &str = "수학";
pub const DATE_MODULE: &str = "date";
pub const DATE_MODULE_KO: &str = "날짜";
pub const USE_KEYWORD_KO: &str = "사용";
pub const WAIT_KEYWORD: &str = "wait";
pub const WAIT_KEYWORD_KO: &str = "기다려";
pub const SKIP_KEYWORD: &str = "skip";
pub const SKIP_KEYWORD_KO: &str = "건너뛰어";
pub const LIST_KEYWORD: &str = "list";
pub const LIST_KEYWORD_KO: &str = "목록";
pub const EACH_KEYWORD: &str = "each";
pub const EACH_KEYWORD_KO: &str = "마다";
pub(crate) const FILE_READ_WORDS_EN: &[&str] = &["read"];
pub(crate) const FILE_WRITE_WORDS_EN: &[&str] = &["write"];
pub(crate) const FILE_READ_WORDS_KO: &[&str] = &["읽어서", "읽고", "읽어"];
pub(crate) const FILE_WRITE_WORDS_KO: &[&str] = &["저장해", "저장해줘", "써줘", "적어"];

/// The Python name the stopwatch statement binds, and the reading taken from
/// it. Both the parser and the lowering stage need the exact same text: the
/// parser so it can tell a program that reads the stopwatch without starting
/// it, the lowering stage so it can emit it.
pub const TIMER_NAME: &str = "_nme_clock";
/// Prefix of the Python name one cooldown binds, completed by the NME name.
pub const COOLDOWN_PREFIX: &str = "_nme_cool_";
/// `elapsed` / `잰시간`, rounded so a printed stopwatch stays readable.
pub const ELAPSED_PYTHON: &str = "round(__import__(\"time\").time() - _nme_clock, 2)";

/// How many parts one whole chance is counted in: `30%` is 300 of these,
/// `30.5%` is 305. Percentages are exact to one decimal place, so permille
/// arithmetic keeps every accepted chance a whole number.
pub const CHANCE_SCALE: u32 = 1000;
/// The largest chance anyone may write, in permille (`100%`).
pub const CHANCE_MAX_PERMILLE: u32 = CHANCE_SCALE;

/// Version of the easy random adapter bundled with this compiler.
pub const RANDOM_MODULE_VERSION: &str = "0.0.1";
/// Version of the easy file adapter bundled with this compiler.
pub const FILE_MODULE_VERSION: &str = "0.0.1";
/// Version of the Schnorr zero-knowledge adapter bundled with this compiler.
pub const ZERO_KNOWLEDGE_MODULE_VERSION: &str = "0.0.2";
/// Version of the easy list adapter bundled with this compiler.
pub const LIST_MODULE_VERSION: &str = "0.0.1";
/// Version of the easy text adapter bundled with this compiler.
pub const TEXT_MODULE_VERSION: &str = "0.0.1";
/// Version of the easy maths adapter bundled with this compiler.
pub const MATH_MODULE_VERSION: &str = "0.0.1";
/// Version of the easy date adapter bundled with this compiler.
pub const DATE_MODULE_VERSION: &str = "0.0.1";

/// One bundled beginner module. Both languages are always exposed after one
/// import, and each module has one explicit local version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundledModuleId {
    Random,
    File,
    ZeroKnowledge,
    List,
    Text,
    Math,
    Date,
}

impl BundledModuleId {
    pub const ALL: [BundledModuleId; 7] = [
        Self::Random,
        Self::File,
        Self::ZeroKnowledge,
        Self::List,
        Self::Text,
        Self::Math,
        Self::Date,
    ];

    pub fn name_en(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE,
            Self::File => FILE_MODULE,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE,
            Self::List => LIST_MODULE,
            Self::Text => TEXT_MODULE,
            Self::Math => MATH_MODULE,
            Self::Date => DATE_MODULE,
        }
    }

    pub fn name_ko(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE_KO,
            Self::File => FILE_MODULE_KO,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE_KO,
            Self::List => LIST_MODULE_KO,
            Self::Text => TEXT_MODULE_KO,
            Self::Math => MATH_MODULE_KO,
            Self::Date => DATE_MODULE_KO,
        }
    }

    pub fn version(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE_VERSION,
            Self::File => FILE_MODULE_VERSION,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE_VERSION,
            Self::List => LIST_MODULE_VERSION,
            Self::Text => TEXT_MODULE_VERSION,
            Self::Math => MATH_MODULE_VERSION,
            Self::Date => DATE_MODULE_VERSION,
        }
    }

    /// True when the module's own name is a word people write in ordinary
    /// sentences. `list`, `text`, `math`, `date`, `목록`, `글자`, `수학` and
    /// `날짜` are all such words: `get the list of names`,
    /// `장 볼 목록을 사용해 보세요` and `이 날짜 사용법을 알려 주세요` are
    /// sentences, not module lines. The parser therefore reads these four
    /// only when the name stands beside the `use`/`사용` word **and** every
    /// other word on the line is module wording; anything else is not a module
    /// line at all, and goes on to be read as the sentence it is.
    pub fn name_is_an_ordinary_word(self) -> bool {
        matches!(self, Self::List | Self::Text | Self::Math | Self::Date)
    }
}

/// Which beginner-facing vocabulary led the parser to a statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    English,
    Korean,
}

/// Python code copied from the source. Expressions are never reformatted or
/// reconstructed by NME.
///
/// A few sentence forms have no source expression to point at — the
/// stopwatch reading and the cooldown comparisons are written by the
/// compiler itself — so they carry their finished Python text instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Code {
    Source(Span),
    Generated(String),
}

/// Language-neutral literal values shared by English and Korean sentence
/// spellings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal {
    True,
    False,
    None,
}

/// One piece of sentence-style text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextPart {
    Literal(String),
    Variable(String),
}

/// Text written without quotes. Variables introduced earlier with `ask` or a
/// sentence assignment are interpolated; all remaining words stay literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextTemplate {
    pub parts: Vec<TextPart>,
}

/// A value accepted by output, input prompts, and sentence assignments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Python(Code),
    Text(TextTemplate),
    Literal(Literal),
    RandomInteger {
        low: Code,
        high: Code,
    },
    RandomChoice {
        choices: Vec<String>,
    },
    /// `list of Mina, Ada` / `목록 민수, 지안`. Each item is read the same way a
    /// single sentence value is, so numbers stay numbers and words become text
    /// without the writer choosing quotes.
    List(Vec<Value>),
    /// `set ages to an empty record` / `나이표는 빈 표` — one name holding many
    /// named values, which Python calls a dictionary.
    ///
    /// There is no spelling for a record with things already in it. A record
    /// is filled the way a beginner fills one: a line at a time, with
    /// [`NmeStmt::RecordPut`].
    EmptyRecord,
    /// `Mina in ages` / `나이표의 민수` — the one value a record keeps under
    /// that name.
    ///
    /// The key is a whole sentence value rather than a piece of text, because
    /// the name a loop is holding is exactly what a beginner writes there:
    /// `for each name in ages` then `show name in ages`.
    Entry {
        of: String,
        key: Box<Value>,
    },
    /// `친구들 개수` / `how many friends` — a reading taken from a name the
    /// program already made. The name is kept as a name rather than a span,
    /// because the writer may have attached a Korean particle to it.
    Reading {
        of: String,
        reading: Reading,
    },
    /// `친구들 첫 번째` / `the first of friends` — one item of a list, counted
    /// from **one** the way the sentence says it.
    Item {
        of: String,
        position: ItemPosition,
    },
    /// `친구들을 쉼표로 이어` / `friends joined by comma` — every item of a
    /// list in one piece of text. `separator` is the finished text, so the
    /// named separators, a written one, and the empty one (`친구들을 붙여` /
    /// `friends joined together`) all lower through the same path.
    Joined {
        of: String,
        separator: String,
    },
    /// `메모를 줄마다 나눈 것` / `memo split by line` — the opposite of
    /// `Joined`: one piece of text cut into a list.
    Split {
        of: String,
        by: SplitBy,
    },
    /// `별표를 5개 붙인 것` / `star repeated 5 times` — one piece of text, that
    /// many times over, which is how a row of stars or a bar of a chart is
    /// drawn.
    Repeated {
        of: String,
        times: Code,
    },
    /// `쌓인돌을 4로 나눈 나머지` / `the remainder of pile divided by 4` —
    /// what is left over, which is how most counting games are decided.
    Remainder {
        of: String,
        by: Code,
    },
    /// `elapsed` / `잰시간` — how many seconds the stopwatch has been running.
    Elapsed,
    /// `30% 확률` / `a 30% chance` — true that share of the time.
    ///
    /// Kept as permille (thousandths, 0…1000) rather than a percentage, so
    /// that one decimal place is an exact whole number and the lowered
    /// Python never compares floating-point numbers.
    Chance {
        permille: u32,
    },
    ZeroKnowledge(ZeroKnowledgeValue),
}

/// One sentence-level value from the bundled Schnorr proof-of-knowledge tools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZeroKnowledgeValue {
    Secret,
    Public {
        secret: Code,
    },
    Nonce,
    Commitment {
        nonce: Code,
    },
    Challenge,
    ChallengeExcept {
        excluded: Code,
    },
    Response {
        nonce: Code,
        secret: Code,
        challenge: Code,
    },
    Verify {
        public_key: Code,
        commitment: Code,
        challenge: Code,
        response: Code,
    },
    /// Fiat-Shamir challenge bound to the public key, commitment, and explicit context.
    NizkChallenge {
        public_key: Code,
        commitment: Code,
        context: Code,
    },
    /// JSON-friendly non-interactive Schnorr proof `[commitment, response]`.
    NizkProof {
        secret: Code,
        context: Code,
    },
    /// Verify a context-bound non-interactive Schnorr proof.
    NizkVerify {
        public_key: Code,
        proof: Code,
        context: Code,
    },
    SimulatedResponse,
    SimulatedCommitment {
        public_key: Code,
        challenge: Code,
        response: Code,
    },
}

/// One reading a sentence may take from a list or a piece of text.
///
/// Every one of them is a plain Python builtin, so the generated program
/// stays something a reader can look up. They are one enum rather than one
/// statement each because they share the whole of their grammar: a name, and
/// a word saying what to read from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading {
    /// `친구들 개수` / `how many friends`, and `이름 길이` /
    /// `the length of name`. Both are `len(...)`.
    Count,
    /// `점수들 합` / `the total of scores` — `sum(...)`.
    Total,
    /// `점수들 중 가장 큰 것` / `the biggest of scores` — `max(...)`.
    Largest,
    /// `점수들 중 가장 작은 것` / `the smallest of scores` — `min(...)`.
    Smallest,
    /// `이름 대문자로` / `name in capitals` — `str(...).upper()`.
    Capitals,
    /// `이름 소문자로` / `name in small letters` — `str(...).lower()`.
    SmallLetters,
}

/// Which item of a list a sentence is pointing at.
///
/// The sentence counts from **one**, because that is what the words mean:
/// `첫 번째` is the first one. Python counts from zero, and the lowering
/// stage is where that difference is paid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemPosition {
    /// `친구들 첫 번째` / `the first of friends`.
    First,
    /// `친구들 마지막` / `the last of friends`.
    Last,
    /// `친구들 3번째` / `item 3 of friends`. One-based; `0` is refused.
    Numbered(Code),
}

/// Where a piece of text is cut when it is split into a list.
///
/// Line by line is its own variant rather than the separator `"\n"`, because
/// `splitlines()` is what a file read from disk needs: it copes with the
/// Windows line ending and with a file that ends in a newline, and neither of
/// those is something a beginner should have to know about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitBy {
    /// `줄마다 나눈 것` / `split by line` — `str(...).splitlines()`.
    Lines,
    /// `쉼표로 나눈 것` / `split by comma` — the finished separator text.
    ///
    /// A comma is `","` here and `", "` in [`Value::Joined`], and that is on
    /// purpose: reading a line back out of a file has to find the comma that
    /// is actually there, while reading a list out loud wants the space.
    Text(String),
}

/// How a list is put back in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListOrder {
    /// `친구들 정렬해` / `sort friends` — smallest first.
    Sorted,
    /// `친구들 거꾸로 해` / `reverse friends` — back to front.
    Reversed,
    /// `친구들 섞어` / `shuffle friends` — a different order every run.
    Shuffled,
}

/// One operand in a conversational condition. The parser records meaning;
/// only the lowering stage chooses Python operators and string syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionValue {
    Python(Code),
    Name(String),
    Text(String),
    Literal(Literal),
    /// `만약에 친구들 개수가 3보다 크면` / `if how many friends is greater than 3`
    /// — the same readings a value may take, on one side of a comparison.
    Reading {
        of: String,
        reading: Reading,
    },
    /// `만약에 쌓인돌을 4로 나눈 나머지가 0과 같으면` — a remainder on one side of
    /// a comparison, which is how a counting game is decided.
    Remainder {
        of: String,
        by: Code,
    },
    /// `만약에 나이표의 민수가 90보다 크면` / `if Mina in ages is greater than 90`
    /// — one value out of a record, on one side of a comparison.
    Entry {
        of: String,
        key: Box<Value>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Equal,
    Greater,
    Less,
    LessOrEqual,
    GreaterOrEqual,
    /// `if names contains Mina` / `만약에 이름들에 민수가 있으면`. The container is
    /// on the left, so lowering emits `right in left`.
    Contains,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Python(Code),
    Truthy {
        value: ConditionValue,
        negated: bool,
    },
    Compare {
        left: ConditionValue,
        operator: CompareOp,
        right: ConditionValue,
        negated: bool,
    },
    Logical {
        left: Box<Condition>,
        operator: LogicalOp,
        right: Box<Condition>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Text,
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleVersion {
    Bundled,
    Latest,
    Exact(String),
}

/// One recognized NME statement. Every variant lowers to exactly one Python
/// line so traceback line numbers remain stable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NmeStmt {
    Say {
        value: Value,
    },
    Ask {
        target: String,
        prompt: Option<Value>,
        kind: InputKind,
    },
    Set {
        target: String,
        value: Value,
    },
    Update {
        target: String,
        amount: Code,
        operation: UpdateOp,
    },
    Times {
        count: Code,
        inline: Option<InlineStmt>,
    },
    /// `for each name in names` / `이름들의 이름마다 반복해`. The loop name is the
    /// only name this statement introduces, exactly like a Python `for` target.
    ForEach {
        name: String,
        items: Code,
        /// `for each friend in friends with place` /
        /// `친구들의 친구마다 순서와 함께 반복해` — a second name holding which
        /// turn the loop is on, counted from **one**, because that is what
        /// `친구들 3번째` means. `None` is the plain loop.
        position: Option<String>,
        inline: Option<InlineStmt>,
    },
    /// `wait 3 seconds` / `3초 기다려`.
    Wait {
        seconds: Code,
    },
    /// `say slowly Hello` / `천천히 말해줘 안녕` — one character at a time.
    /// `seconds` is how long to pause between characters.
    SaySlowly {
        value: Value,
        seconds: Code,
    },
    /// `clear the screen` / `화면 지워`.
    ClearScreen,
    /// `draw a line` / `줄 그어` — a rule right across the screen.
    DrawLine,
    /// `say in a box Hello` / `상자로 말해줘 안녕`.
    SayInBox {
        value: Value,
    },
    /// `say in the middle Hello` / `가운데 말해줘 안녕`.
    SayInMiddle {
        value: Value,
    },
    /// `start the timer` / `시간 재기 시작해`.
    StartTimer,
    /// `put door on cooldown for 3 seconds` / `문 쿨타임 3초 걸어`.
    Cooldown {
        target: String,
        seconds: Code,
    },
    /// `wait for door` / `문 쿨타임 끝날때까지 기다려`.
    WaitForCooldown {
        target: String,
    },
    /// `append Mina to friends` / `친구들에 민수 넣어`.
    Append {
        target: String,
        value: Value,
    },
    /// `remove Mina from friends` / `친구들에서 민수 빼`.
    ///
    /// Taking something away from a list is spelled the same way as taking a
    /// number away from a number, so the two are told apart by the name: a
    /// name the program made into a list can only mean this one.
    Remove {
        target: String,
        value: Value,
    },
    /// `put Mina at 90 in ages` / `나이표에 민수를 90으로 넣어`.
    ///
    /// Putting something into a record is spelled with the same verb as
    /// putting something into a list, so the two are told apart by the name
    /// and by the shape: a record line marks a key **and** a value, a list
    /// line marks only the thing being added.
    RecordPut {
        target: String,
        key: Value,
        value: Value,
    },
    /// `remove Mina from ages` / `나이표에서 민수 빼` — Python's `del`.
    ///
    /// A list gives back the item it took out with `.remove(...)`; a record
    /// has no such method, and `.remove` on a dictionary is an
    /// `AttributeError` a beginner cannot read.
    RecordRemove {
        target: String,
        key: Value,
    },
    /// `sort friends` / `친구들 정렬해`, and its two companions.
    Arrange {
        target: String,
        order: ListOrder,
    },
    /// `repeat forever` / `계속 반복해` — a block that never ends on its own.
    /// `break` / `멈춰` is the way out.
    Forever {
        inline: Option<InlineStmt>,
    },
    /// `30% 확률로` / `30% chance` — a block, or one statement, that runs
    /// only that share of the time.
    Chance {
        /// Thousandths, so `30%` is 300 and `30.5%` is 305.
        permille: u32,
        inline: Option<InlineStmt>,
    },
    /// `이야기:` / `story:` — opens a block in which every line is text.
    ///
    /// Nothing inside a story block is ever a command. That is the whole
    /// point of the form: a story is prose, and a line of prose that
    /// silently turns into a statement is the worst thing this compiler can
    /// do to a program.
    Story {
        /// `None` for the plain block; `Some(seconds)` for the slow
        /// spellings, and then every line inside is told one character at a
        /// time with that pause between them.
        seconds: Option<Code>,
    },
    /// `to greet:` / `인사하기라는 일:` — opens a block that is a piece of
    /// program with a name. Python calls it a function.
    ///
    /// Like the story block, it is recognized by **structure** and never by a
    /// word: `일`, `하기`, `to` and `do` are ordinary words in both languages.
    /// The colon, the noun or the opening `to`, and a body underneath are all
    /// required together, so `할 일이 많습니다` and `to be honest` stay prose.
    Job {
        name: String,
        /// The names the job is given when it is run. Empty today: the
        /// spelling for a job that takes something is not built yet.
        parameters: Vec<String>,
    },
    /// `do greet` / `인사하기 해줘` — run a job that was defined earlier.
    ///
    /// `do`, `해` and `해줘` are among the most ordinary words either language
    /// has, so the gate is not the word: the name has to be one the program
    /// has already made a job.
    RunJob {
        name: String,
        arguments: Vec<Value>,
    },
    When {
        condition: Condition,
        inline: Option<InlineStmt>,
    },
    While {
        condition: Condition,
        inline: Option<InlineStmt>,
    },
    ElseIf {
        condition: Condition,
        inline: Option<InlineStmt>,
    },
    Else {
        inline: Option<InlineStmt>,
    },
    Break,
    /// `skip` / `건너뛰어` — Python's `continue`, spelled the way a first-week
    /// learner would say it.
    Continue,
    End,
    UseModule {
        module: BundledModuleId,
        requested: ModuleVersion,
    },
    FileRead {
        target: String,
        path: Code,
    },
    FileWrite {
        path: Code,
        value: Value,
    },
    /// `from "helper.nme" import greet, score` — imports named values from
    /// another `.nme` module. The explicit name list is the module interface;
    /// nothing else leaks between files.
    ModuleImport {
        path: Code,
        names: Vec<String>,
    },
}

/// The single statement in an inline NME block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineStmt {
    Nme(Box<NmeStmt>),
    Python(Span),
}

/// A fully parsed NME statement plus the source span it replaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmeLine {
    /// Index in the lexer's logical-line list.
    pub line_index: usize,
    pub span: Span,
    pub stmt: NmeStmt,
    /// Indentation inserted by an explicit `end`/`끝` block.
    pub virtual_indent: usize,
}
