# NME native backend: research and design

English | [한국어](native-backend.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md) | [Native core reference](native-reference.md) | [Guides](guides/index.md)

> Status: v0 is implemented (`nme native run`/`nme native build`). The
> statically typed core subset — integer and finite-float values, strings, and arithmetic,
> sentence `while`/`if`/`else`/`else if` over comparisons, beginner `times:` loops, `break`, functions over integer
> scalar parameters with an unconditional integer `return` (recursion works),
> and `say` — compiles to
> C and then to a native executable through the system C compiler. Everything
> outside the core is rejected with a clear diagnostic and still runs on
> CPython. This document is the honest technical plan for the full backend;
> it does not call the current Python pipeline or Nuitka an "NME native
> compiler".

## What NME compiles to today

The NME compiler (`nme-core`) is a source-to-source transpiler. It lowers the
sentence, beginner, and advanced levels to ordinary Python, preserving one
physical line per source line, and CPython runs the result. `nme compile`
asks the installed Nuitka to build that Python into an executable. Nuitka is a
mature Python-to-C compiler, but it is **not** an NME-native backend: NME
uses that path for the full Python-compatible pipeline, so its artifact still
has Python runtime semantics. The separate `nme native` backend compiles its
documented restricted NME subset to C and a system-compiler executable; it does
not claim to compile all NME or all Python.

The goal of this document is to define and extend that restricted backend
honestly while keeping the full language on CPython.

The CLI uses `cc` on macOS and Linux, and Microsoft's `cl` on Windows. On
Windows, start a Developer PowerShell for Visual Studio (or another shell
where `cl.exe` is on `PATH`) before using `nme native`. NME passes `/utf-8` so
Korean and English strings in generated C keep their UTF-8 meaning. The CPython
commands do not need that compiler shell.

For the current CLI, `nme native run <file>` compiles and runs a temporary
executable without saving artifacts. Use `nme native build <file> -o <path>`
to keep the executable and generated C source; passing `-o` to `run` is
rejected with E9031. With no `-o`, NME appends `.c` to the full source stem,
so `count.ko.nme` produces `count.ko.c` instead of colliding with `count.c`.
On Windows, implicit outputs also receive `.exe` even when the source stem ends
in `.ko`.
A source named `count.c.nme` therefore uses `count.c` as its default executable
on Unix and `count.c.exe` on Windows, with `count.c.c` as its generated source;
only an explicit `-o count.c` is rejected as a C-source collision.
Choose only one action word; passing both `run` and `build` is rejected with
E9032.

## The core design decision: a restricted native core, not "all of NME"

NME's semantics are Python's semantics. A native compiler must choose which
semantics it targets, because the full Python object model (dynamic dispatch,
arbitrary attribute access, metaprogramming, the whole standard library) is a
vast runtime. Every serious Python-family native compiler makes the same
choice: compile a **statically analyzable core** and keep everything else on
CPython.

The NME-native backend therefore targets a **restricted, statically typed core
subset** with semantics defined independently of CPython.
Implemented so far:

- integers and finite floats with `+ - * %` arithmetic (checked signed 32-bit native
  integers from `-2147483648` through `2147483647`; integer modulo only, with
  overflow and zero-divisor errors reported by the bilingual native runtime;
  float literals must be finite, float arithmetic uses C `double`, and whole
  floats print with C's `%g`, which may differ cosmetically from Python's `5.0`);
- string variables with `+` concatenation into variables (checked fixed
  8192-byte buffers), escaped string output, string `==`/`!=` comparisons
  through `strcmp`, and a Unicode-character-counting `len` builtin; nested
  concatenation, embedded NUL characters, and ordering text are rejected rather
  than miscompiled. A stored
  or concatenated value beyond 8191 UTF-8 bytes stops with a bilingual
  native-runtime error instead of overflowing the buffer;
- control flow: sentence `while`/`if`/`else`/`else if` over integer,
  float, and string comparisons (`<`, `>`, `<=`, `>=`, `==`, `!=`, plus
  the natural-language "or equal" connectors), over integer and finite-float
  truthiness (`if ready`, `while turns`; zero is false), and over boolean
  literals; the beginner
  `times:` loop; `break` inside a native loop only. A `break` nested only in
  an `if` is rejected with `E0102` before C is emitted;
- functions over integer scalar parameters with an unconditional integer
  `return` (recursion works); calls may target any function in the file,
  including one defined later, with its declared arity and positional
  arguments. Simple positional integer
  parameters only are accepted in headers; duplicate definitions, defaults,
  varargs, nested definitions, keyword arguments, float or string function
  values, branch-only returns, and top-level `return` (`E0106`) are rejected
  rather than converted or left to C fallthrough;
- function-local scalar assignments remain scoped to their function;
- value changes require an existing integer or float binding, and assignments
  cannot change a native name from one type to another;
- a name first assigned in a possibly skipped control block must be assigned
  before that block or used after assignment inside it; a literal `if true`
  branch is known to run, while names assigned only in its unreachable
  `else`/`else if` alternatives are not exported; sibling branches do not
  make each other's new bindings visible;
- `say`/`show`/`말해` of an integer expression, a float, a string variable,
  or a string literal;
- Korean and English spellings both lower to the same C;
- identifiers that collide with C keywords, C implementation-reserved forms,
  or generated runtime names are rejected, never silently renamed. Names
  beginning `__`, names beginning `_` followed by an uppercase letter, and
  file-scope function names beginning `_` are reserved; an ordinary local name
  such as `_value` remains usable. Runtime names also include `nme_copy`,
  `nme_cat`, `NME_STRING_CAPACITY`, `NME_UNUSED`, `_nme_i`, the checked integer
  helpers, and the C library symbols exposed by the generated headers;
- source comments are emitted as inert C comments, so comment text cannot turn
  into a C preprocessor directive or change native function hoisting;
- native expressions require a prior binding or a declared function call;
  bare function values, duplicate parameters, and bindings or parameters that
  shadow a native function name are rejected before C emission;

Still planned: real boolean variables as a distinct type from integer
truthiness. The currently accepted surface is documented in the [native core
reference](native-reference.md).

Everything outside the core — dynamic Python, classes, imports, packages,
`use random`/`use file` adapters — stays on the **Python compatibility
backend**. The two backends are separate compiler paths with an explicit
boundary. This is not a promise that "all Python packages work natively"; it
is the honest separation the language needs.

## Backend candidates

### 1. C backend (generate C, compile with a system C compiler)

Lower the core subset to C and call `cc`/`clang` on Unix-like systems or
Microsoft's `cl` on Windows. This is the approach taken by Cython (for typed
regions), Nuitka, and many small language experiments.

| | |
| --- | --- |
| Maturity | C optimizers (gcc, clang) are the most mature compilers in existence |
| Dependencies | none beyond a system C compiler (already required for Nuitka); use `cl` from a Windows developer shell |
| Build-time model | classic AOT: C source is an artifact the learner can read |
| Runtime | small: string/collection helpers, integer policy; written once |
| Verification | fully testable in this environment (gcc is present) |
| Risks | C is a large surface with undefined behavior; generated C must be
  simple and reviewed; a C compiler is required to build |

### 2. LLVM via `inkwell` / `llvm-sys`

Drive LLVM IR from Rust.

| | |
| --- | --- |
| Maturity | strongest optimizer; used by Rust, Swift, Clang |
| Dependencies | `llvm-sys`/`inkwell` must match a system libLLVM version; version
  churn is a known pain; no libLLVM present in this environment to evaluate |
| Build-time model | AOT or JIT |
| Runtime | still needed for strings/collections — LLVM does not provide one |
| Risks | heavy dependency for a small core; the optimizer matters far less
  than the runtime when the core is small; not testable here |

### 3. Cranelift

Rust-native code generation (the Wasmtime/Wasmer backend).

| | |
| --- | --- |
| Maturity | actively maintained, production use in WebAssembly engines |
| Dependencies | pure Rust — no external C++/LLVM, no version pinning pain |
| Build-time model | JIT-oriented; AOT is possible via `cranelift-object` but is
  the less-trodden path |
| Runtime | still needed for strings/collections |
| Risks | optimizer is far weaker than gcc/LLVM for AOT; smaller ecosystem;
  an entire extra crate graph for what the C backend already gives |

### 4. Direct machine-code generation

Hand-writing assembly or a mini codegen.

| | |
| --- | --- |
| Verdict | rejected for the same reason the language rules reject it: the
  maintenance and correctness cost dwarfs any benefit when mature C/LLVM
  optimizers exist. Revisit only if portability demands exclude a C toolchain. |

## Recommendation for the implemented v0 backend

**Generate C and compile with the system C compiler for the v0 NME-native
backend**, and keep LLVM/Cranelift as measured upgrades later.

Rationale, in order of weight:

1. **Smallest honest step.** The restricted core is tiny; the C backend needs
   one small runtime and no new Rust dependency. It produces real native
   machine code through a mature optimizer (gcc), not an interpreter.
2. **Testable here.** gcc is present; an end-to-end native smoke test
   (compile `say`/arithmetic/loops to an executable, run it, diff the output)
   can be added to the workspace gates immediately.
3. **Readable artifact.** `nme native build hello` emits `hello.c` and an
   executable — the beginner sees C and learns where the machine code comes
   from, matching the "grow into Python" story (sentence → beginner →
   advanced → native core → C).
4. **LLVM/Cranelift add little for the core.** Both still need the same
   string/collection runtime; their optimizer advantage is negligible on
   scalar-heavy beginner programs and only matters once the core grows
   substantially. When that happens, the C backend's structure (runtime +
   lowering) ports to either backend with the frontend unchanged.

The honest boundary, stated plainly: **a native build only covers the
documented core subset.** Any program using features outside it is rejected
with a clear "not supported by the native backend — run with CPython" error,
never silently miscompiled. Performance claims are forbidden until measured.

## Architecture

```text
            nme-core (frontend: lexer/parser/AST/lowering to Python)
                          │
        ┌─────────────────┴──────────────────┐
        │                                    │
  Python compatibility backend          nme-native backend
  (lowering + CPython, unchanged)      (core subset → C → executable)
```

The frontend stays shared. The native backend is a separate crate
(`nme-native`), a separate compiler path, and a separate set of diagnostics
reusing the `DiagnosticCode` registry. A program is accepted by the native
path only if every statement belongs to the documented core.

## Status and next milestones

The v0 baseline is implemented: the restricted `nme-native` compiler, its
UTF-8 string and checked-integer runtime, the `nme native run`/`build` CLI
entry points, bilingual diagnostics, and end-to-end tests are all in the
workspace. The next milestones are:

1. Extend the core only when a shared semantic definition, bilingual coverage,
   native/CPython comparison, and memory-safety tests are ready.
2. Measure the core across supported operating systems and compilers before
   making any broader performance or portability claim.

Measured 2026-08-11 on this machine: a 50,000,000-iteration integer count-up
loop runs in about `0.03 s` as a native `-O2` binary versus about `2.0 s` on
CPython — roughly 60× on this one micro-benchmark, compile time included in
the native figure. This is a single measurement of a tight integer loop, not
a blanket claim about all programs.

## References

- Nuitka and Cython both demonstrate the C-backend viability for typed
  regions of Python; neither is an NME-native compiler.
- LLVM and Cranelift are documented, mature code generators; the choice above
  is about dependency weight and core size, not their capability.
