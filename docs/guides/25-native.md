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
   integers, finite floats, and string literals, `while`/`if`/`else`, `break`,
   functions with `return`, and `say`:

   ```text
   score = 0
   while score is less than 10
       score add 1
   end
   show score
   ```

   Native string variables use a checked 8192-byte buffer. If a stored or
   concatenated value is larger than 8191 UTF-8 bytes, the native program stops
   with a bilingual runtime error; escaped newlines and tabs are supported, but
   embedded NUL characters are rejected. Use the CPython path for unrestricted
   text.
   Native integers are signed 32-bit values (`-2147483648` through
   `2147483647`); overflow and modulo by zero stop with a bilingual runtime
   error. Native functions currently accept and return integers only, and each
   function needs an unconditional top-level integer `return`; calls must name
   a function in the file with the declared number of positional arguments.
   Use simple integer parameters in the header; defaults, varargs, and keyword
   arguments are outside the native core, as are nested function definitions.
   Float literals must be finite. Native float arithmetic uses C `double`, and
   `%g` output may print `5.0` as `5`; `-0.0` retains its sign as `-0`.
   Native expressions must use a name assigned earlier or call a declared
   function. Function values, duplicate parameters, and reusing a function
   name for a variable or parameter are rejected; use the CPython path for
   dynamic Python name behavior.
   A name assigned only in an unreachable `else` or `else if` after `if true`
   is not available after the block.

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

   `nme native build` writes `count.c` next to the executable. Reading the C
   is how you see what your program really becomes.

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
checks every statement against the documented core, and emits C. The system
C compiler (`cc`) turns that into machine code with `-O2`. The
[architecture memo](../native-backend.md) compares this C backend with LLVM
and Cranelift and explains why C is the first backend.

Performance is honest and measured: on this machine, a 50-million-iteration
integer loop runs about 60× faster natively than on CPython. That is one
micro-benchmark of a tight loop, not a claim about every program.

## Try it yourself

Change the countdown to count up to 100, or write a `square(n)` function and
print `square(7)`, then run it with `nme native`.

## What you learned

- The native core compiles to C and to a native executable with `cc`.
- `nme native run` runs it; `nme native build` keeps the C and executable.
- Functions, loops, branches, and `say` all work inside the core.
- Outside the core, the backend rejects the program instead of miscompiling.
