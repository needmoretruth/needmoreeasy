# AGENTS.md — working on NeedMoreEasy

This file is the practical checklist for repository work. The design rationale
and compiler invariants live in [`docs/architecture.md`](docs/architecture.md).

## Before editing

1. Read this file completely.
2. Read `docs/architecture.md` completely before changing compiler behavior.
3. Run `git status --short` and preserve unrelated or user-owned changes.
4. Inspect the relevant implementation, tests, examples, and documentation.

Do not guess how a command or language form works. Confirm it in the code or
tests first.

## Project scope

NME is a thin syntax layer over Python. Valid Python is valid NME; NME adds a
small beginner-friendly syntax and transpiles it to ordinary Python for
CPython to execute.

The project values a small, maintainable foundation over feature count. Make
the smallest change that solves the current problem. Do not add speculative
features, abstractions, dependencies, or configuration.

## Non-negotiable rules

1. **Python wins.** A valid Python line must never be rewritten. Every NME
   matcher must keep the real Python statement/header validity checks.
2. **Tokens, never text scanning.** Python understanding comes only from
   `rustpython-parser` tokens and parses. Do not use regexes, `str::replace`,
   or hand-written scanning to interpret source.
3. **Expressions stay opaque.** Validate NME expressions as Python, store byte
   spans, and copy their original text. Do not parse or reformat them.
4. **Preserve lines.** One logical input line must lower to one output line so
   CPython traceback line numbers remain correct. Preserve indentation,
   comments, blank lines, and line endings.
5. **Keep the core pure.** `nme-core` is `&str -> Result<String,
   Vec<Diagnostic>>`: no file IO, process execution, global state, or Python
   runtime calls. Those belong in `nme-cli`.
6. **Errors are for beginners.** User-facing diagnostics need a plain message,
   an exact caret span, and an actionable hint. Collect independent problems
   instead of stopping at the first one.
7. **No `unsafe`.** It is forbidden workspace-wide and must remain forbidden.
8. **Apache-2.0 only.** Project code and documentation use no other license.
9. **Dependencies need a present reason.** Prefer the standard library and the
   existing `rustpython-parser`. Keep `Cargo.lock` tracked because the
   workspace ships a CLI application.

## Repository map

| Path | Responsibility |
| --- | --- |
| `crates/nme-core/src/diagnostics.rs` | diagnostic data and rendering |
| `crates/nme-core/src/lexer.rs` | Python tokens to logical lines |
| `crates/nme-core/src/syntax.rs` | NME AST and keywords |
| `crates/nme-core/src/parser.rs` | Python-wins classification and NME matching |
| `crates/nme-core/src/lower.rs` | line-preserving Python edits |
| `crates/nme-core/src/transpile.rs` | pure pipeline entry point |
| `crates/nme-core/tests/` | transpilation, compatibility, and error contracts |
| `crates/nme-cli/` | CLI parsing, file IO, Python execution, integration tests |
| `examples/` | runnable programs used by tests and documentation |
| `docs/language*.md` | English and Korean user-facing language reference |
| `docs/architecture.md` | canonical contributor-facing design decisions |

## Change workflow

### Language syntax or behavior

Follow this order:

1. Define the Python meaning and prove it can preserve line count.
2. Add or adjust the AST in `syntax.rs`.
3. Match tokens in `parser.rs`, after the Python-wins checks.
4. Lower the statement in `lower.rs`.
5. Add friendly diagnostics for each beginner mistake.
6. Add tests for the construct, mixed Python/NME, look-alike valid Python,
   source preservation, and every error case.
7. Update an example when it helps a beginner.
8. Update both English and Korean README/language documentation together.

Do not land a construct with only a happy-path test.

### Core compiler changes

Keep module boundaries from `docs/architecture.md`. Add focused unit tests for
the changed stage and end-to-end tests when observable output can change.

### CLI changes

Keep all IO and process work in `nme-cli`. Test exit status, stdout, stderr,
files written, and real `python3` execution when applicable. Update both
getting-started documents when commands or flags change.

### Documentation-only changes

Check relative links, commands, examples, and behavior claims against the
implementation. Keep English and Korean user documents aligned. The full Rust
suite is not required unless the documentation change exposes uncertainty
that needs executable verification; run the smallest relevant command then.

### Ignore or repository configuration changes

Use narrow patterns that do not hide source or reproducibility files. Verify
representative paths with `git check-ignore -v` and confirm expected files
remain visible in `git status`.

## Git and commits

- Commit completed work at small, coherent milestones. Keep each commit
  independently understandable; do not combine unrelated changes or create a
  separate commit for every trivial edit.
- Before staging, inspect `git status` and the relevant diffs. Stage explicit
  paths only, never `git add .`, `git add -A`, or `git add --all`.
- Use an imperative, specific subject in the form `<type>: <summary>`, such as
  `feat: add ...`, `fix: preserve ...`, `docs: explain ...`, or
  `chore: configure ...`. Add a body when the reason or tradeoff is not clear
  from the diff. Avoid vague messages such as `update`, `changes`, or `fix`.
- Use the repository's configured user name and email. Do not add AI
  attribution or `Co-authored-by` trailers unless the user explicitly asks.
- Never commit secrets, local settings, build output, or a known broken state.
- Do not amend, rebase, force-push, or rewrite existing history without
  explicit authorization. Push only when the user has requested it.

## Validation

Choose validation by the files and behavior changed.

For Rust, Cargo manifest, dependency, or runtime behavior changes, run all of:

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Tests use the system `python3` for real execution. If it is unavailable, report
that clearly rather than claiming full verification.

For documentation-only or `.gitignore` changes, normally run:

- `git diff --check`;
- a relative-link and documented-command check; and
- targeted `git check-ignore -v` checks when ignore rules changed.

Always report what was verified. Do not claim checks that were not run.

## Completion checklist

A change is done only when:

- it stays within the requested scope and preserves the invariants above;
- relevant tests or targeted checks pass;
- user-facing behavior changes are documented in English and Korean;
- no unrelated work was overwritten; and
- the final handoff states the changed files, validation performed, and any
  remaining limitation.
