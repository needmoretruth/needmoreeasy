# Changelog

English | [한국어](CHANGELOG.ko.md)

All notable changes to NME are recorded here.

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
