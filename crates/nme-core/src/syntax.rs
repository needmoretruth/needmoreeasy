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
pub const USE_KEYWORD_KO: &str = "사용";
pub(crate) const FILE_READ_WORDS_EN: &[&str] = &["read"];
pub(crate) const FILE_WRITE_WORDS_EN: &[&str] = &["write"];
pub(crate) const FILE_READ_WORDS_KO: &[&str] = &["읽어서", "읽고", "읽어"];
pub(crate) const FILE_WRITE_WORDS_KO: &[&str] = &["저장해", "저장해줘", "써줘", "적어"];

/// Version of the easy random adapter bundled with this compiler.
pub const RANDOM_MODULE_VERSION: &str = "0.0.1";
/// Version of the easy file adapter bundled with this compiler.
pub const FILE_MODULE_VERSION: &str = "0.0.1";
/// Version of the Schnorr zero-knowledge adapter bundled with this compiler.
pub const ZERO_KNOWLEDGE_MODULE_VERSION: &str = "0.0.2";

/// One bundled beginner module. Both languages are always exposed after one
/// import, and each module has one explicit local version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundledModuleId {
    Random,
    File,
    ZeroKnowledge,
}

impl BundledModuleId {
    pub const ALL: [BundledModuleId; 3] = [Self::Random, Self::File, Self::ZeroKnowledge];

    pub fn name_en(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE,
            Self::File => FILE_MODULE,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE,
        }
    }

    pub fn name_ko(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE_KO,
            Self::File => FILE_MODULE_KO,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE_KO,
        }
    }

    pub fn version(self) -> &'static str {
        match self {
            Self::Random => RANDOM_MODULE_VERSION,
            Self::File => FILE_MODULE_VERSION,
            Self::ZeroKnowledge => ZERO_KNOWLEDGE_MODULE_VERSION,
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Code {
    Source(Span),
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
    RandomInteger { low: Code, high: Code },
    RandomChoice { choices: Vec<String> },
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

/// One operand in a conversational condition. The parser records meaning;
/// only the lowering stage chooses Python operators and string syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionValue {
    Python(Code),
    Name(String),
    Text(String),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Equal,
    Greater,
    Less,
    LessOrEqual,
    GreaterOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOp {
    Add,
    Subtract,
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
