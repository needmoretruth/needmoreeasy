//! # `nme-core` — the `NeedMoreEasy` (NME) compiler core
//!
//! NME is a language for people who find even Python hard. Every valid
//! Python program is already a valid NME program; NME only adds a tiny set
//! of easier statements on top (see [`syntax`]).
//!
//! ## Compilation pipeline
//!
//! ```text
//! source text (.nme)
//!   │
//!   ▼
//! ┌──────────────┐   logical lines, using Python's own token rules
//! │    lexer     │   (strings / comments / f-strings are never seen as code)
//! └──────────────┘
//!   │
//!   ▼
//! ┌──────────────┐   recognizes NME statements; *valid Python always wins*
//! │    parser    │
//! └──────────────┘
//!   │
//!   ▼
//! ┌──────────────┐   turns each NME statement into a Python source edit
//! │    lower     │   (line-preserving: line N of input stays line N)
//! └──────────────┘
//!   │
//!   ▼
//! ┌──────────────┐   applies the edits
//! │  transpile   │   → ordinary Python source, run by the real Python runtime
//! └──────────────┘
//! ```
//!
//! The core is **pure**: it maps source text to source text and never does
//! IO, never spawns processes and never executes Python. All of that lives
//! in the `nme-cli` crate, keeping this crate trivially testable.
//!
//! See `docs/architecture.md` in the repository for the full design
//! rationale and the rules every contributor must follow.

pub mod diagnostics;
pub mod convert;
pub mod lexer;
pub mod lower;
pub mod parser;
pub mod syntax;
pub mod transpile;

pub use transpile::transpile;
pub use convert::{convert_python, Conversion, Language, SyntaxLevel};
