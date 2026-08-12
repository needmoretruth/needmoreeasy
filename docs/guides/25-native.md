# 25 — Native: compile to machine code

English | [한국어](25-native.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.md), [07 — While](07-while.md)
- 주제 (Topic): 네이티브 컴파일 / native compilation
- 결과물 (Result): CPython 없이 기계어로 실행하기 / running a program as machine code without CPython

So far every program ran on CPython: NME compiles to Python and Python runs
it. A small part of the language — the native core — can go further and
become machine code directly. `nme native` turns that core into C and then
into a native executable with your system's C compiler.

## Steps

1. Write a program that stays inside the native core. The core covers
   boolean, integer, and finite-float values, string literals, `while`/`if`/
   `else`, logical `and`/`or`, `break`, functions with integer `return`, and
   `say`/`show`/`말해`. Beginner `times:`/`번:` loops and sentence repeat
   forms can also use one-line NME output bodies. A native `if`/`while` or
   branch body may put one NME output statement after `then`/`그러면` on the
   same line:

   ```text
   score = 0
   while score is less than 10
       score add 1
   end
   show score
   ```

   ```text
   ready save true
   if ready then show "ready"
   ```

   Here are the one-line repeat forms:

   ```text
   repeat 2 times and show Hi
   2번 반복해서 안녕 말해줘
   2 times: say "beginner"
   2번: 말해 "초급"
   ```

   Ordinary Python `for` loops, Python inline bodies, and inline value changes
   remain outside this restricted native subset; use the CPython path for
   those forms.

   Native string variables use a checked 8192-byte buffer. If a stored or
   concatenated value is larger than 8191 UTF-8 bytes, the native program stops
   with a bilingual runtime error; escaped newlines and tabs are supported, but
   embedded NUL characters are rejected. Use the CPython path for unrestricted
   text. `len` counts Unicode characters even though the storage limit is in
   UTF-8 bytes.
   Native integers are signed 32-bit values (`-2147483648` through
   `2147483647`); overflow and modulo by zero stop with a bilingual runtime
   error. Native functions currently accept and return integers only, and each
   function needs a top-level integer `return`; an arm may return early, but
   every path that continues after a control block must reach that final
   return. A nested conditional with no reachable fall-through arm also
   terminates its enclosing branch. A branch that breaks out of its enclosing
   native loop is also a terminating path. Calls may name a function defined later in the same file, with the
   declared number of positional arguments.
   Use simple integer parameters in the header; defaults, varargs, and keyword
   arguments are outside the native core, as are nested function definitions.
   Float literals must be finite. Native float arithmetic uses C `double`; an
   arithmetic result outside the finite range stops with a bilingual runtime
   error. `%g` output may print `5.0` as `5`; `-0.0` retains its sign as `-0`.
   Native expressions may use a literal, a name assigned earlier, or call a
   declared function. Function values, duplicate parameters, and reusing a
   function name for a variable or parameter are rejected; use the CPython path for
   dynamic Python name behavior.
   A name assigned only in an unreachable `else` or `else if` after `if true`
   is not available after the block. A name assigned in every branch of an
   `if`/`else` chain is available afterward; a branch that returns early or
   breaks out of its enclosing loop does not need to assign it, even when the
   terminating path contains a nested conditional. One continuing branch cannot
   read a name first assigned in a sibling branch, and a loop-created name
   remains conditional if the loop may not run.

   Boolean names are distinct from integer names even though the generated C
   stores both in an `int`. They can be assigned, compared with `==`/`!=`, used
   directly as conditions, and shown as `True` or `False`:

   ```text
   ready = True
   show ready
   if ready
       show "ready"
   end
   ready = False
   show ready
   ```

   The same value can be written through the easier English and Korean forms:

   ```text
   ready save true
   set ready to True
   준비는 참
   저장 준비 True
   ```

   Boolean arithmetic, `add`/`subtract` updates, and boolean function
   parameters or returns stay on the CPython path.

2. Compile and run it natively:

   ```sh
   nme native run count
   ```

   ```text
   10
   ```

   The short form `nme native count` runs the same way.

3. Keep the C source and the executable:

   ```sh
   nme native build count -o count
   ```

   `nme native build` writes `count.c` next to the executable. Without `-o`, a
   `.ko` source keeps that suffix in its C name (`count.ko.c`), so English and
   Korean twins can be built in one folder. On Windows, implicit outputs also
   receive `.exe` when the source stem ends in `.ko`. Reading the C is how you
   see what your program really becomes. A source named `count.c.nme` uses
   `count.c` on Unix or `count.c.exe` on Windows as its default executable, and
   `count.c.c` as its generated source; only an explicit `-o count.c` is rejected
   as a C-source collision.
   The `-o` option belongs to `build`; `nme native run count -o count` is
   rejected with E9031 because `run` does not save an artifact.
   Choose one action word: writing both `run` and `build` is rejected with
   E9032 instead of letting the last word silently win.

4. Functions and recursion work inside the core — the example
   `examples/native-factorial.nme` (Korean twin
   `examples/native-factorial.ko.nme`) computes factorials on both backends:

   ```text
   # part of examples/native-factorial.nme
   def fact(n):
       if n is less than 2
           return 1
       end
       return n * fact(n - 1)


   show fact(5)
   ```

   ```sh
   nme run examples/native-factorial
   nme native examples/native-factorial
   ```

   Both print `120`.

5. Anything outside the core is rejected with a clear error and still runs on
   CPython. The native backend never silently miscompiles a program:

   ```sh
   nme native ask-demo    # prints a "not supported" diagnostic
   nme run ask-demo       # still works on CPython
   ```

## How it works

`nme-native` (a Rust crate) takes the same frontend AST as the Python path,
checks every statement against the documented core, and emits C. On macOS and
Linux, `cc` turns that into machine code with `-O2`; on Windows, Microsoft's
`cl` does the same with `/O2` and `/utf-8` from a Developer PowerShell for
Visual Studio. The [native core reference](../native-reference.md) lists the
accepted surface; the [architecture memo](../native-backend.md) compares this
C backend with LLVM and Cranelift and explains why C is the first backend.

Performance is honest and measured: on this machine, a 50-million-iteration
integer loop runs about 60× faster natively than on CPython. That is one
micro-benchmark of a tight loop, not a claim about every program.

## Try it yourself

Change the countdown to count up to 100, or write a `square(n)` function and
print `square(7)`, then run it with `nme native`.

## What you learned

- The native core compiles to C and to a native executable with `cc` on
  macOS/Linux or MSVC `cl` on Windows.
- `nme native run` runs it; `nme native build` keeps the C and executable.
- Functions, loops, branches, and `say`/`show`/`말해` output all work inside
  the core, including the documented one-line NME output forms for conditions
  and repeats.
- Outside the core, the backend rejects the program instead of miscompiling.
