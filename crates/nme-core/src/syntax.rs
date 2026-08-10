//! The NME surface syntax — deliberately tiny (v0.1).
//!
//! ```text
//! say <expr>              print the value of any Python expression
//! <expr> times:           repeat the indented block that follows <expr> times
//! <expr> times: <stmt>    same, with a single inline statement
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
//! When adding a new construct, also update `docs/language.md` and follow
//! the recipe in `docs/architecture.md` ("Adding a new NME construct").

use crate::diagnostics::Span;

/// The keyword that starts a [`NmeStmt::Say`] statement.
pub const SAY_KEYWORD: &str = "say";
/// The keyword that marks a [`NmeStmt::Times`] loop.
pub const TIMES_KEYWORD: &str = "times";

/// One recognized NME statement. Spans always refer to the original source,
/// so lowering can copy expression text without ever re-printing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NmeStmt {
    /// `say <expr>` — the span covers exactly the expression text.
    Say {
        /// Byte span of the expression to print.
        expr: Span,
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
}

/// The single statement after `:` in an inline `times` loop.
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
