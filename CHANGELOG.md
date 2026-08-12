# Changelog

English | [한국어](CHANGELOG.ko.md)

All notable changes to NME are recorded here.

## Unreleased

## 0.0.1-beta.53 — 2026-08-12

- Reject duplicate native function definitions and unsupported default or
  varargs headers before C generation.
- Reject keyword arguments in native calls so no AST arguments are silently
  dropped.

## 0.0.1-beta.52 — 2026-08-12

- Validate native function calls against integer function definitions and their
  declared arity before emitting C, with bilingual diagnostics for unknown or
  mismatched calls.

## 0.0.1-beta.51 — 2026-08-12

- Reject native functions that can fall through without an unconditional
  integer return, preventing undefined C return values after conditional-only
  branches.

## 0.0.1-beta.50 — 2026-08-12

- Check native signed 32-bit integer literals and arithmetic instead of
  allowing undefined C overflow or zero-divisor behavior.
- Report bilingual native runtime errors for integer overflow and modulo by
  zero, while documenting the bounded native integer range.
- Reject float arguments and return values in native functions instead of
  silently converting them through C `int` parameters and returns.

## 0.0.1-beta.49 — 2026-08-12

- Track conditional native bindings and reject reads or value changes after a
  possibly skipped block unless the name was initialized beforehand.
- Keep statically true `if true` blocks usable while reporting a precise
  bilingual diagnostic for uncertain initialization.

## 0.0.1-beta.48 — 2026-08-12

- Reject native assignments that change a binding from integer, float, or
  string to another type before generating incompatible C.
- Require a prior numeric binding for `add`/`subtract` value changes and report
  a bilingual diagnostic for uninitialized or string targets.

## 0.0.1-beta.47 — 2026-08-12

- Hoist native scalar and string declarations to the active function scope when
  assignments occur inside control blocks, while preserving source control
  flow and later binding use.
- Prevent nested-block assignments from producing out-of-scope C declarations.

## 0.0.1-beta.46 — 2026-08-12

- Keep native function-local scalar bindings separate from main-program
  bindings, so names can be reused without generating invalid C.
- Document the scope behavior in both native backend references.

## 0.0.1-beta.45 — 2026-08-12

- Reject generated native-runtime names in function parameters before emitting C,
  including unused parameters that would otherwise shadow runtime helpers.
- Keep the same precise bilingual diagnostic used for native variables and
  function names.

## 0.0.1-beta.44 — 2026-08-12

- Reject C keywords and generated native-runtime names before C lowering, with
  precise bilingual diagnostics instead of allowing namespace collisions.
- Reserve helper names such as `nme_copy`, `nme_cat`, `len`, and `_nme_i`
  without silently renaming user identifiers.

## 0.0.1-beta.43 — 2026-08-12

- Replace unbounded native string copies and concatenation with checked helpers.
- Stop oversized stored or concatenated strings with a bilingual runtime error
  instead of allowing fixed-buffer overflow.
- Document the native string capacity and keep the CPython path available for
  unrestricted text.

## 0.0.1-beta.42 — 2026-08-12

- Make `nme build -o` refuse to overwrite an existing Python output with E9009.
- Keep the existing artifact unchanged for English and Korean build commands,
  matching `nme compile` and `nme native build`.

## 0.0.1-beta.41 — 2026-08-12

- Own imported-module, native, and Nuitka staging directories for the whole
  operation and remove them on both success and early failure.
- Prevent partial Python or C staging files from being left behind when a write
  fails.

## 0.0.1-beta.40 — 2026-08-12

- Use fresh per-invocation temporary directories for imported-module, native,
  and Nuitka staging instead of reusing process-ID-only folders.
- Prevent stale Python files left by a crashed run or PID reuse from shadowing
  ordinary imports in a later program.

## 0.0.1-beta.39 — 2026-08-12

- Make `nme native build` refuse to overwrite an existing executable or
  companion C source with E9009, matching the other build commands.
- Reject `.c` output paths with E9003 so the executable and generated C source
  cannot target the same file.
- Extend the E9009 English/Korean lookup explanation to cover native artifacts.

## 0.0.1-beta.38 — 2026-08-12

- Make `nme native` classify directory arguments as E9014 instead of reporting
  them as unreadable files E9007, matching the CPython-backed commands.
- Add English/Korean native-command coverage for the shared folder diagnostic.

## 0.0.1-beta.37 — 2026-08-12

- Keep existing but unreadable program files on E9007 instead of reporting them
  as missing programs E9015 for `nme run` and `nme native`.
- Preserve E9015 for paths that actually cannot be found.

## 0.0.1-beta.36 — 2026-08-12

- Keep `nme compile` temporary-folder failures on E9027 instead of reporting
  them as native compiler startup failures E9011.
- Classify temporary Python-source write failures as E9008 while preserving
  E9011 for failures to start the external compiler process.

## 0.0.1-beta.35 — 2026-08-12

- Report unreadable imported `.nme` modules with the existing file-read
  diagnostic E9007 instead of the top-level program-resolution code E9015.
- Expand E9007’s bilingual explanation to cover imported module files.

## 0.0.1-beta.34 — 2026-08-12

- Give `nme install` without a package name its own stable diagnostic, E9030,
  instead of reusing the option-value code E9003.
- Add English/Korean lookup coverage for the missing-package argument path.

## 0.0.1-beta.33 — 2026-08-12

- Give imported module-name collisions their own stable diagnostic, E9028,
  instead of reporting them as invalid option values.
- Give the current `nme compile` module-import limitation its own stable
  diagnostic, E9029, with bilingual lookup and CLI regression coverage.

## 0.0.1-beta.32 — 2026-08-12

- Add E9027 for temporary working-folder creation failures instead of labeling
  them as current-folder read errors.
- Preserve Korean-first bilingual diagnostics when imported modules are staged
  for execution.

## 0.0.1-beta.31 — 2026-08-12

- Give native executable startup failures their own stable diagnostic, E9026,
  instead of reporting them as Python startup errors.
- Add a deterministic Unix CLI regression and bilingual public lookup coverage
  for the native-program startup path.

## 0.0.1-beta.30 — 2026-08-12

- Clarify E9010 and E9011 in both languages so their recovery guidance covers
  the Nuitka `compile` path and the system-C-compiler `native` path.
- Add public lookup regressions that keep the two backend toolchains visible to
  beginners.

## 0.0.1-beta.29 — 2026-08-12

- Give failed Python package installs their own appended diagnostic code,
  E9025, instead of reusing the native-compiler code E9010.
- Add bilingual lookup and network-independent CLI regression coverage for the
  package-install failure path.

## 0.0.1-beta.28 — 2026-08-12

- Route the Korean `네이티브` and `설치` command aliases through the same
  Korean-first bilingual diagnostics as the other Korean CLI commands.
- Add regression coverage for both failure paths while preserving English-only
  output for the English commands.

## 0.0.1-beta.27 — 2026-08-12

- Fix the English beginner skeleton in both example-template twins so its loop
  stops at three instead of becoming an accidental infinite loop.
- Add a parity regression guard for the bounded English and Korean template
  loops.

## 0.0.1-beta.26 — 2026-08-12

- Fill the remaining English code examples in the main NeedMoreCoin guide and
  the example-authoring guide so paired guides expose equivalent teaching
  material.
- Extend the documentation parity check to every guide pair, not only numbered
  guides, and require matching code-block coverage.

## 0.0.1-beta.25 — 2026-08-12

- Complete the missing Topic metadata in the four NeedMoreCoin sequence guides
  in both languages.
- Add equivalent English sentence-level proof-of-work and transaction-proof
  snippets, and enforce numbered-guide code-block parity in CI.

## 0.0.1-beta.24 — 2026-08-12

- Repair Korean documentation links so local pages lead to their Korean twins,
  while deliberate English comparison links remain available.
- Complete consistent bilingual navigation for the guide sequence and add a CI
  parity check that catches wrong-language links and missing navigation rows.

## 0.0.1-beta.23 — 2026-08-12

- Keep the proof-of-work difficulty labels consistent across the six
  NeedMoreCoin examples (Korean/English sentence, beginner, and advanced
  surfaces), with a regression test for the shared learning contract.

## 0.0.1-beta.22 — 2026-08-12

- Make the six-way NeedMoreCoin learning matrix fast and deterministic enough
  for regular example validation, and keep the sentence examples genuinely
  punctuation-free where their surface promises that progression.
- Add parser and regression coverage for the pure English sentence proof
  expressions and validate the Korean and English sentence sources separately.

## 0.0.1-beta.21 — 2026-08-12

- Repair the locked release metadata and keep CLI and cryptocurrency example
  regression checks aligned with the current beta package versions.

## 0.0.1-beta.20 — 2026-08-12

- Replace the earlier standalone blockchain demonstrations with the
  NeedMoreCoin learning project: complete Korean/English sentence, beginner,
  and advanced examples, a shared construction guide, and an authoring
  standard for six-way examples.
- Add automated coverage that checks all six examples, their intended syntax
  surfaces, and their shared observable behavior.

## 0.0.1-beta.19 — 2026-08-12

- Converge the public beta Git topology with `main`: the final beta.19 release commit keeps beta.18 as its first parent and records the current main tip as its second parent. The beta first-parent release line still advances exactly one version per public commit, while `main` becomes an actual ancestor of the next-generation `beta` branch.
- Keep the beta.17 release guard, locked Cargo validation, three-OS gate, and CPython 3.10/3.12/3.14 compatibility matrix unchanged.

## 0.0.1-beta.18 — 2026-08-12

- Extend the bundled Schnorr adapter to version `0.0.2` with context-bound Fiat-Shamir non-interactive proofs. The SHA-256 challenge binds the Group 15 generator, commitment, public key, and a length-prefixed explicit context under an NME domain tag.
- Add `zk_nizk_challenge`, `zk_nizk_prove`, and `zk_nizk_verify` plus Korean sentence forms. Proofs are JSON-friendly `[commitment, response]` values and cross-context reuse is rejected.
- Add Korean/English executable examples, parser/lowering and CLI end-to-end coverage, and explicit documentation that context binding does not replace same-context freshness/replay controls.

## 0.0.1-beta.17 — 2026-08-12

- Make `beta` the enforced next-generation release line. Every public beta push must advance the workspace beta number by exactly one, name that version in the commit subject, and keep the workspace package versions in `Cargo.lock` synchronized.
- Upgrade CI to `actions/checkout@v6` and `actions/setup-python@v6`, and run Cargo checks and tests with `--locked`.
- Add CPython 3.10, 3.12, and 3.14 compatibility jobs for beta and pull requests while retaining the full Ubuntu, Windows, and macOS quality gate.

## 0.0.1-beta.16 — 2026-08-11

- Add the bundled `zero_knowledge` / `영지식` adapter (version `0.0.1`) with a
  finite-field Schnorr proof-of-knowledge reference implementation: secure
  randomness from Python `secrets`, RFC 3526 3072-bit MODP Group 15 subgroup
  parameters, 256-bit verifier challenges, subgroup/range checks, transcript
  simulation helpers, helper-name collision protection, and Korean
  sentence-only proof expressions. Add matching Korean/English A→B examples
  with malicious relay C showing saved-transcript replay failure, transcript
  simulation, and the separate live-relay case. Document the security scope:
  mathematically faithful learning/reference code, not a side-channel-hardened
  production cryptography library.


- Extend the NME-native core: integer `%` modulo in arithmetic (float modulo is rejected honestly); conditions using `%` are a frontend follow-up.
- Fix the native backend so the very first string assignment can be a concatenation (`greeting = "hello" + " world"`): a C array cannot be initialized from a function call, so the emitter declares the buffer first and copies with strcpy.
- Extend the NME-native core: float literals, float variables, float arithmetic (mixed int/float promotes to double), and float comparisons.
- Extend the NME-native core: the beginner `times:` loop (block and inline forms) lowers to a C for-loop.
- Extend the NME-native core: boolean literals in truthy conditions (`if true`/`if false` lower to 1/0), alongside integer truthiness.
- Extend the NME-native core: truthy conditions (`if ready`, `while turns`) over integer values, so counters and flags work natively without comparisons.
- Add the natural-language `<=`/`>=` connectors: `if x is less than or equal to 3` and Korean `만약에 점수가 10보다 작거나 같으면` lower to `<=`/`>=` on both backends. The `or equal` phrase is kept out of logical-`or` splitting and typo recovery.
- Extend the NME-native core: `+` concatenation into string variables (fixed buffers via `strcpy`), so strings can be built up step by step; nested concatenation stays rejected.
- Extend the NME-native core: string `==`/`!=` comparisons through `strcmp` (both the Python condition form and the natural Korean form), a `len` builtin mapped to `strlen`, and string equality in sentence conditions.
- Extend the NME-native core: string variables (literals), string output, and one binary `+` concatenation through a small runtime helper, with nested concatenation honestly rejected; expressions now carry static types (int vs string) through lowering.
- Extend the NME-native core: functions over scalar parameters with `return` (recursion works), `else`/`else if` branches, calls in `say`, and honest rejection of C-keyword identifiers; the compiler now builds with `-O2`. Measured on this machine: a 50M-iteration integer loop is ~60x faster natively than on CPython (one micro-benchmark, documented in the memo).
- Implement the first slice of the NME-native AOT backend (`nme-native` crate + `nme native run`/`nme native build`): a restricted, statically typed core subset (integer values, sentence `while`/`if` over comparisons, `break`, `say`) lowers to C and compiles to a native executable with the system C compiler; anything outside the core is rejected with a clear bilingual diagnostic and still runs on CPython. Korean spellings work; end-to-end tests compile, run, and compare output.
- Add the bootstrap example (an NME program that transpiles a tiny language to Python and runs it) with a Korean twin, guide 29 on bootstrapping/self-hosting, and a CLI test that runs both.
- Add guide 25 (native compilation): teaches `nme native run`/`nme native build`, the documented core subset, functions and recursion, the C artifact, and the honest measured benchmark.
- Teach `nme install` in the READMEs and getting-started (guide 24).
- Add `nme install` / `nme 설치` as a friendly pip wrapper: it installs a Python package and tells the beginner the `import` line to use in an `.nme` file, with clear bilingual messages when pip is missing.
- Add the native-backend research memo (`docs/native-backend.md`): an honest evaluation of a C backend vs LLVM vs Cranelift vs direct codegen, recommending C for the first NME-native AOT compiler targeting a restricted statically-typed core subset, explicitly separated from the Python compatibility backend and from Nuitka.
- Add a `birthday.nme` countdown example that uses the `datetime` standard package from inside NME (with a Korean twin) and guide 24 on the standard library and pip-installed packages.
- Add `.nme` module imports: `from "helper.nme" import greet, score` imports
  only the listed names from a sibling `.nme` file, so a project can split
  into several files with an explicit interface and no shared global state.
  `nme run`/`check`/`build` transpile imported modules transitively and make
  them importable at runtime (via a temporary module folder on `sys.path`);
  module errors report the module's file name. File names must be Python
  identifiers, two modules may not share a name, and `nme compile` defers
  module support. Includes a two-file example pair (`examples/modules/`).
- Add an `http-client.nme` example that fetches a page from a local server
  with `urllib`, and a `terminal-menu.nme` TUI menu loop (both with Korean
  twins); a CLI test runs the menu with scripted input.
- Teach `nme convert` the file sentence forms: `x = open("f").read()` and
  `x = Path("f").read_text()` convert to `read "f" into x`, and
  `open("f", "w").write(v)` / `Path("f").write_text(v)` to
  `write v to "f"` (Korean spellings for Korean output). Beginner conversion
  keeps file IO as Python since the beginner file surface is `use file`; the
  converted sentence source round-trips through the compiler.
- Add four educational blockchain learning projects (learning only, never
  investment advice), each with a Korean twin: `blockchain-ledger.nme`
  (beginner, blocks linked by hashes), `proof-of-work.nme` (intermediate,
  mining with difficulty and a chain-integrity check), `signatures.nme`
  (advanced, HMAC signing and verification), and `consensus.nme` (expert, a
  two-node fork and longest-chain rule simulation).
- Add sentence-level file forms: `read "notes.txt" into memo`,
  `memo read "notes.txt"`, `memo에 "notes.txt" 읽어서 (저장해)`,
  `write "hello" to "out.txt"`, and `"out.txt" 파일에 "hello"를 저장해`
  lower to `pathlib` lines without the `use file` module. Read targets become
  known names for sentence interpolation, and weak matches like `read the
  book` or `write hello` stay plain sentence output.
- Bundle a `use file` / `파일 사용` module (version `0.0.1`) for reading,
  writing, and JSON, next to `use random`. One import exposes both
  vocabularies: `file_read`/`파일읽기`, `file_write`/`파일쓰기`,
  `json_load`/`json읽기`, `json_save`/`json저장`, plus version names. The
  `use` line parser is now shared by both modules (same latest/version forms,
  same collision protection, same diagnostics), and `nme modules` lists both.
  Sentence-level file wrappers are the next step.
- Extend stable error codes to command-line diagnostics: `nme ko <CODE>` and
  `nme en <CODE>` now also explain CLI errors (`E9001` unknown command,
  `E9015` missing program, `E9013` Python startup, ...). Compiler codes stay
  `E0001`+; CLI codes use the `E9xxx` range and render the same way
  (`error[E9015]:`). Every `fail()` path in the CLI now carries a code.
- Fix explicit `end`/`끝` block parsing when an indented sentence block is
  followed by a flat block: an indented body that cannot be closed by the
  remaining `end` lines now closes at the dedent, so `만약 ...` with an
  indented body followed by a flat `if ... end` block no longer reports a
  missing `end`. Every previously valid program keeps its exact output;
  nested headers with enough closing `end`s still stay nested, and a flat
  block still requires its own `end`.
- Give every compiler diagnostic a stable error code printed next to the
  message, e.g. `error[E0102]:`. `nme ko <CODE>` reads the long Korean
  explanation with an English translation, `nme en <CODE>` the English one,
  and `nme ko` (or `nme 에러` / `nme error`) with no code lists every code.
  Each code documents what went wrong, why, and the recovery steps; the code
  list and lookup pages are also taught in the help text, both READMEs, and
  both language references.
- Split the installation guide into independent per-OS sections (Windows 11,
  Windows 10, older Windows, macOS, Debian/Ubuntu, Fedora, Arch Linux), each
  with copy-paste install commands, PATH, version check, first run, and common
  errors.
- Start the 100-guide curriculum: `docs/guides/` now has an index (difficulty
  legend, learn-in-order path, topic lookup, full table) and the first twelve
  beginner guides (hello → ask → set → update → repeat → if → while → break →
  and/or → random → check/build → convert), each labeled with difficulty,
  prerequisites, topic, and result in both languages; every code block is
  verified with `nme check`.
- Accept shortened unique program names everywhere: `nme r gue` runs
  `guessing-game.nme`, and the same prefix rule works for `run`/`실행`,
  `check`/`검사`, `build`/`빌드`, `compile`, `convert`, the bare run shortcut
  (`nme gue`), and the numbered pick (bare names and prefixes answer the
  "Which one?" question). Case-insensitive exact stems win, then a unique
  prefix; when several programs match, NME lists the candidates and asks for
  more of the name instead of guessing.
- Long outputs (help, error-code lists) no longer panic when the reader
  closes the pipe early, e.g. `nme ko | head`.

## 0.0.1-beta.15 — 2026-08-11

- Accept the Korean `!=` sentence comparison `같지 않으면`, `같지 않다면`,
  `같지 않을` (also written `같지않으면` and friends), matching the existing
  English `is not equal to`.
- Fix `while` + Korean sentence condition + `동안` endings (for example
  `while 점수가 3보다 작을 동안`): the ending is now consumed as a block
  marker instead of being lowered as the loop's inline body, and every
  logical operand may carry its own ending (`while 점수가 10과 같지 않을
  동안 그리고 점수가 3보다 클 동안`).
- Fix Korean logical conditions: comparison endings may now combine with
  `그리고`/`또는` (`점수가 0보다 크면 그리고 점수가 3보다 작으면`), and
  malformed conditions report a diagnostic instead of crashing the parser.
- Fix the English roulette companion to use `ask number` for numeric menus,
  bets, and wheel picks.
- Add command shortcuts (`nme r`/`c`/`b`/`m`/`v`/`h`, `nme comp`/`nme conv`)
  and bare-file discovery: `nme r` runs the single `.nme` program in the
  current folder, lists and asks for a numbered pick when several exist, and
  explains what to do when none do.
- Add an English and Korean twin for every beginner example.
- Fix beginner-path documentation in both languages, close English/Korean
  parity gaps, and link the new examples from the tutorials.
- Teach the new shortcuts and show friendlier file hints in the CLI and both
  language guides.

## 0.0.1-beta.14 — 2026-08-11

- Track Python import bindings so the random adapter also protects names
  imported before `use random`.

## 0.0.1-beta.13 — 2026-08-11

- Refuse to load the bundled random adapter when its generated helper names
  would overwrite an existing value.

## 0.0.1-beta.12 — 2026-08-11

- Fix the indentation of the Korean beginner time-loop example so every
  published example passes `nme check`.

## 0.0.1-beta.11 — 2026-08-11

- Let compact `3 times:` / `3번:` beginner repeat blocks close with `end` / `끝`
  without requiring physical indentation.
- Accept the natural beginner spelling `repeat 3 times:` and keep ordinary
  colon-bearing Python suites on Python's indentation rules.
- Infer common age questions (`How old are you?`, `몇 살이에요?`) and accept
  spoken Korean loop endings such as `준비하는동안`.
- Treat polite show requests such as `Please show me hello` as the same simple
  output sentence instead of printing the request word.
- Document the sentence-to-beginner path with matching English and Korean
  flat-block examples.

## 0.0.1-beta.10 — 2026-08-11

- Recover common Korean condition-starter typos (`만악에`), spaced Korean
  particles/endings (`이름 이 철수 면`), and the spoken `그러면` connector
  without turning the right-hand value into text.
- Recover clear module typos such as `use random lates` and `랜덤 사요 최신`.
- Natural questions accept bare or separated targets (`나이 몇 살이에요`,
  `이름 은 뭐예요`) while preserving noun names that end in `이`.
- Korean `nme 버전` now prints Korean and English version information.

## 0.0.1-beta.9 — 2026-08-11

- Let a first program ask naturally with `What is your name?`, `What's your
  city?`, `이름이 뭐예요?`, or `나이는 몇 살이에요?` without `ask`, commas, or
  quotes; the final question mark is optional and `내 이름은 뭐예요?` is also
  understood.
- Accept target-first saves such as `name save Mina` and `이름 저장 민수`, and
  virtual-indent an ordinary Python `if`/`for` suite inside a flat NME block.
- Accept short Korean equality endings such as `이름이 철수면`, `이라면`, and
  `준비가 거짓이면`, plus bounded spoken typos such as `있으먄` and `철수먄`.
- Rewrite the first-run examples and tutorials around the sentence-to-Python
  learning bridge.

## 0.0.1-beta.8 — 2026-08-11

- Accept subject-first conversational conditions such as `color equals red
  then show yes` and their natural Korean equivalents, including flat `end`
  blocks.
- Recover common logical connector typos (`그리거`, `an`) and spoken Korean
  condition endings such as `같먄` without hijacking ordinary output sentences.
- Preserve future Python shapes that the bundled parser does not know yet,
  including CPython 3.14 t-strings, and add a Windows/macOS/Linux CI matrix.

## 0.0.1-beta.7 — 2026-08-11

- Make ordinary multiword sentences and contractions such as `Hello world!`
  and `I'm ready` print naturally without an output keyword.
- Recover common transposed action and condition typos, including `shwoe` and
  `thne`, while keeping ambiguous Python-shaped input untouched.
- Accept an unquoted comma prompt such as `ask name, What is your name?`.

## 0.0.1-beta.6 — 2026-08-11

- Add an easier sentence bridge for value changes (`add 1 to score`,
  `점수에 1 더해`) and repeat plain words without a colon or output marker.
- Accept spaced and attached Korean condition endings, polite sentence fillers,
  and explicit Korean beginner save words such as `저장` and `설정`.
- Refresh the first-run examples and tutorials so learners can move from
  sentences through beginner control flow into ordinary Python without a
  forced indentation jump.

## 0.0.1-beta.5 — 2026-08-11

- Make the three learning levels easier to mix inside one flat block, with
  regression coverage for Korean beginner spellings and ordinary Python.
- Accept attached Korean condition endings such as `이름있으면` and the natural
  `아니면만약에` branch spelling, plus small polite sentence fillers.
- Keep top-level Python identifiers such as `end` and `끝` untouched and avoid
  accidentally opening a colon-based Python block merely because a later NME
  block has an `end`.
- Refresh the bilingual language reference and local continuation handoff.

## 0.0.1-beta.4 — 2026-08-11

- Add an indentation-free control-flow bridge: `while`, `break`, `and`/`or`,
  `elif`/`else`, and `end`/`끝` can be mixed with sentence, beginner, and
  ordinary Python lines.
- Add Korean spellings for the new control-flow forms, virtual indentation for
  flat blocks, structural diagnostics, and regression examples.
- Expand the English/Korean learning path and AI handoff around growing from
  the easiest sentences into Python.

## 0.0.1-beta.3 — 2026-08-11

- Center NME on growing from ordinary sentences, through compact beginner
  syntax, into Python inside the same project.
- Add extensionless `nme run program`, `nme 실행 program`, and `nme program`
  commands with automatic platform Python selection.
- Make Korean CLI flows substantively bilingual while English flows remain
  English-only, including syntax messages, hints, and command failures.
- Make `check` and `build` validate generated source with CPython; failed
  builds never create an output file.
- Fix ambiguous action recovery, condition negation and literals, lexical
  scope leakage, Korean particles/actions, module validation, apostrophes in
  English sentences, and physical-line preservation.
- Make Python conversion conservative around calls, multiline statements,
  aliases, scopes, prompts, expressions, and ordinary `import random`.
- Fix Cargo PATH instructions for Fedora and package-manager installations.

## 0.0.1-beta.2 — 2026-08-10

- Add freely mixable advanced Python, compact beginner, and conversational
  sentence syntax in English and Korean.
- Add punctuation-light sentence input, output, assignment, repetition,
  conditions, numeric input, random integers, and random choices.
- Recover bounded one-character action-word typos and report ambiguous prose
  with a caret and repair hint.
- Add the locally versioned bilingual random adapter and module listing.
- Add safe Python-to-NME conversion for a chosen level and output language.
- Add optional standalone native compilation through an installed Nuitka.
- Add runnable greeting, number-guessing, three-level, and tiny-compiler
  examples plus matching bilingual tutorials and platform/editor guides.
- Preserve the Python-wins and line-preserving compiler contracts across the
  new syntax.

## 0.0.1-beta.1 — 2026-08-10

- Establish the first public beta version line.
- Add bilingual output, text input, repetition, and conditional syntax.
- Keep all valid Python source compatible and byte-identical.
- Provide ready-to-use English and Korean helpers backed by Python's bundled
  `random` module.
- Add matching English/Korean tutorials, exact language references, examples,
  and release policy documentation.
