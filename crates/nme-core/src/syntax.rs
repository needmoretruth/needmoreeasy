//! The NME surface syntax — a deliberately small bilingual starter set.
//!
//! ```text
//! say <expr> / 말해 <expr>                 show a value
//! ask <name>, <prompt> / 물어봐 <name>, ... read text
//! <expr> times: / <expr> 번:                repeat a block
//! when <expr>: / 만약 <expr>:               run a conditional block
//! use random / 랜덤 사용                    enable random helpers
//! ```
//!
//! ## Design rules that keep this language safe to grow
//!
//! 1. **NME expressions are opaque Python expressions.** NME never parses,
//!    rewrites or re-implements them. The parser only checks that the text
//!    *is* a valid Python expression and then copies it verbatim. Every
//!    Python feature — f-strings, comprehensions, walrus, match — keeps
//!    working inside NME statements for free, forever.
//!
//! 2. **Python wins.** A line that is valid Python is always treated as
//!    Python, even if it looks like NME (`say(x)` stays a function call).
//!    NME only claims lines Python itself rejects. Enforced in
//!    [`crate::parser`], documented in `docs/architecture.md`.
//!
//! 3. **One logical line in, one line out.** Every NME statement lowers to
//!    Python text on the same line, so Python tracebacks point at the same
//!    line numbers the user wrote.
//!
//! English and Korean spellings always lower to the same Python semantics.
//! When adding a new construct, update both language references and follow
//! the recipe in `docs/architecture.md` ("Adding a new NME construct").

use crate::diagnostics::Span;

/// The keyword that starts a [`NmeStmt::Say`] statement.
pub const SAY_KEYWORD: &str = "say";
/// Korean spelling of [`SAY_KEYWORD`].
pub const SAY_KEYWORD_KO: &str = "말해";
/// The keyword that starts a [`NmeStmt::Ask`] statement.
pub const ASK_KEYWORD: &str = "ask";
/// Korean spelling of [`ASK_KEYWORD`].
pub const ASK_KEYWORD_KO: &str = "물어봐";
/// The keyword that marks a [`NmeStmt::Times`] loop.
pub const TIMES_KEYWORD: &str = "times";
/// Korean spelling of [`TIMES_KEYWORD`].
pub const TIMES_KEYWORD_KO: &str = "번";
/// The keyword that starts a [`NmeStmt::When`] statement.
pub const WHEN_KEYWORD: &str = "when";
/// Korean spelling of [`WHEN_KEYWORD`].
pub const WHEN_KEYWORD_KO: &str = "만약";
/// The English module-loading keyword.
pub const USE_KEYWORD: &str = "use";
/// The one beginner module NME exposes with easy helpers.
pub const RANDOM_MODULE: &str = "random";
/// Korean spelling of [`RANDOM_MODULE`].
pub const RANDOM_MODULE_KO: &str = "랜덤";
/// Korean postfix spelling of [`USE_KEYWORD`].
pub const USE_KEYWORD_KO: &str = "사용";

/// Which beginner-facing vocabulary was used for a statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    English,
    Korean,
}

/// One recognized NME statement. Spans always refer to the original source,
/// so lowering can copy expression text without ever re-printing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NmeStmt {
    /// `say <expr>` — the span covers exactly the expression text.
    Say {
        /// Byte span of the expression to print.
        expr: Span,
    },
    /// `ask name, prompt` / `물어봐 이름, 안내문` — read text into a name.
    Ask {
        /// Byte span of the simple variable name receiving the text.
        target: Span,
        /// Optional byte span of the prompt expression.
        prompt: Option<Span>,
    },
    /// `<expr> times:` — the span covers exactly the count expression.
    Times {
        /// Byte span of the repetition-count expression.
        count: Span,
        /// `Some(..)` for the inline form (`5 times: say "hi"`),
        /// `None` for the block form (body is the following indented lines,
        /// which are parsed as independent logical lines).
        inline: Option<InlineStmt>,
    },
    /// `when <expr>:` / `만약 <expr>:` — a beginner-friendly condition.
    When {
        /// Byte span of the Python condition expression.
        condition: Span,
        /// Optional inline body, matching the two `times` forms.
        inline: Option<InlineStmt>,
    },
    /// `use random` / `랜덤 사용` — expose Python's bundled random tools.
    UseRandom {
        /// Selects the helper names made available by lowering.
        spelling: Spelling,
    },
}

/// The single statement after `:` in an inline NME block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineStmt {
    /// An NME statement, lowered recursively (`5 times: say "hi"`).
    Nme(Box<NmeStmt>),
    /// An ordinary Python statement, copied verbatim (`5 times: print("hi")`).
    Python(Span),
}

/// A fully parsed NME statement plus the source span it replaces.
///
/// This is the parser's output and the lowering's input: lowering rewrites
/// `span` into Python while the rest of the file stays byte-identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmeLine {
    /// Byte span of the whole NME statement (first token .. last token).
    pub span: Span,
    /// What the statement means.
    pub stmt: NmeStmt,
}
