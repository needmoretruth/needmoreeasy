# NME native backend: research and design

English | [한국어](native-backend.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md) | [Guides](guides/index.md)

> Status: v0 is implemented (`nme native run`/`nme native build`). A small
> statically typed core subset — integer values, `while`/`if` over
> comparisons, `break`, and `say` — compiles to C and then to a native
> executable through the system C compiler. Everything outside the core is
> rejected with a clear diagnostic and still runs on CPython. This document
> is the honest technical plan for the full backend; it does not call the
> current Python pipeline or Nuitka an "NME native compiler".

## What NME compiles to today

The NME compiler (`nme-core`) is a source-to-source transpiler. It lowers the
sentence, beginner, and advanced levels to ordinary Python, preserving one
physical line per source line, and CPython runs the result. `nme compile`
asks the installed Nuitka to build that Python into an executable. Nuitka is a
mature Python-to-C compiler, but it is **not** an NME-native backend: NME
still has no compiler that turns NME source into machine code itself, and the
native artifact remains Python in disguise.

The goal of this document is to plan that missing backend honestly.

## The core design decision: a restricted native core, not "all of NME"

NME's semantics are Python's semantics. A native compiler must choose which
semantics it targets, because the full Python object model (dynamic dispatch,
arbitrary attribute access, metaprogramming, the whole standard library) is a
vast runtime. Every serious Python-family native compiler makes the same
choice: compile a **statically analyzable core** and keep everything else on
CPython.

The proposed NME-native backend therefore targets a **restricted, statically
typed core subset** with semantics defined independently of CPython:

- scalars: integers (v1: `i64` with explicit overflow diagnostics; arbitrary
  precision is a later bignum runtime), IEEE floats, booleans;
- UTF-8 strings with a small runtime (`len`, concatenation, `show`);
- control flow: `while`, `if`/`else`, `break`, functions over scalars,
  sentence/beginner `say`/`show`/`ask` for the core types;
- a `native.nme` surface document listing exactly what is in and out.

Everything outside the core — dynamic Python, classes, imports, packages,
`use random`/`use file` adapters — stays on the **Python compatibility
backend**. The two backends are separate compiler paths with an explicit
boundary. This is not a promise that "all Python packages work natively"; it
is the honest separation the language needs.

## Backend candidates

### 1. C backend (generate C, compile with a system C compiler)

Lower the core subset to C and call `cc`/`clang`. This is the approach taken
by Cython (for typed regions), Nuitka, and many small language experiments.

| | |
| --- | --- |
| Maturity | C optimizers (gcc, clang) are the most mature compilers in existence |
| Dependencies | none beyond a system C compiler (already required for Nuitka) |
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

## Recommendation

**Generate C and compile with the system C compiler for the first NME-native
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

## Milestones

1. Document `native.nme` core surface (types, statements, functions) in this
   repository with examples.
2. `nme-native`: core-subset parser gate + C lowering for scalars, `say`,
   arithmetic, `while`/`if`/`break`, and functions.
3. Runtime: UTF-8 string helpers and integer policy (smallest correct set).
4. `nme native run`/`nme native build` CLI entry points; a workspace test
   that compiles, runs, and compares output against the CPython path.
5. Benchmark the core subset against CPython honestly; only then publish
   numbers. Extend the core only with measured evidence.

## References

- Nuitka and Cython both demonstrate the C-backend viability for typed
  regions of Python; neither is an NME-native compiler.
- LLVM and Cranelift are documented, mature code generators; the choice above
  is about dependency weight and core size, not their capability.
