# 81 — Compiler tier: functions in the mini language

English | [한국어](81-functions.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [80 — AST](80-ast.md), [61 — Modules](61-modules.md)
- Topic: compiler & functions
- Result: a compiler that translates a mini language with `def`, `return`, and calls into Python and runs it

Guide [86](86-capstone.md) compiles five verbs into Python. Real languages
have functions, so this compiler grows one: `def name params` opens a
function, `return expr` ends it, and `say name(args)` calls it. The new
piece is a **signature table** — the compiler remembers every function's
parameters so it can emit real Python `def` lines.

## Steps

1. The mini language grows two verbs. `def` names a function and lists its
   parameters; `return` gives the answer; a `say` may call a function:

   ```nme
   [
       "def double n",
       "    return n * 2",
       "say double(21)",
       "def add a b",
       "    return a + b",
       "say add(2, 3)",
       "say done",
   ]
   ```

   `double(21)` means "call the double function with 21". The indented
   `return` marks the body, exactly like the `while` body in guide
   [86](86-capstone.md).

2. The compiler keeps two tables. `known` still lists variables (guide
   [86](86-capstone.md)); a new dict `functions` maps each function name to
   its parameter list:

   ```nme
   functions = {}
   ```

   On a `def` line the compiler stores the signature and emits a Python
   header; on `return` it emits the return statement and leaves the body
   block:

   ```text
   elif verb == "def":
       name = parts[1]
       params = parts[2:]
       functions[name] = params
       lines.append(" " * indent + f"def {name}({', '.join(params)}):")
       indent = indent + 4
   elif verb == "return":
       expr = " ".join(parts[1:])
       lines.append(" " * indent + "return " + expr)
       indent = indent - 4
   ```

   `', '.join(params)` turns `["a", "b"]` into the text `a, b` — the same
   join that made CSV rows in guide [40](40-csv.md).

3. `say` must now recognize calls. The word before `(` is the function
   name; if it is in `functions`, the whole call is an expression and
   `print` receives it without quotes:

   ```text
   elif verb == "say":
       text = raw.split(None, 1)[1]
       name = text.split("(")[0]
       if name in functions or name in known:
           lines.append(" " * indent + f"print({text})")
       else:
           lines.append(" " * indent + f'print("{text}")')
   ```

   `raw.split(None, 1)[1]` takes everything after the first space, so
   `say add(2, 3)` keeps its comma — a plain `split()` would cut it at the
   space. This is why real compilers do not just split every line into
   words. `say done` has no `(`; `done` is in neither table, so it prints
   as a quoted text — the same fallback as before.

4. The full compiler writes `out.py` and runs it with `exec`, exactly like
   the capstone. Save `functions.nme`:

   ```nme
   # functions.nme — a mini language with functions, compiled to Python.
   # Run: nme r functions
   # Reads the mini language, compiles it to Python source,
   # writes out.py, then runs out.py with exec.

   use file latest

   program = [
       "def double n",
       "    return n * 2",
       "say double(21)",
       "def add a b",
       "    return a + b",
       "say add(2, 3)",
       "say done",
   ]

   known = []
   functions = {}
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "def":
           name = parts[1]
           params = parts[2:]
           functions[name] = params
           lines.append(" " * indent + f"def {name}({', '.join(params)}):")
           indent = indent + 4
       elif verb == "return":
           expr = " ".join(parts[1:])
           lines.append(" " * indent + "return " + expr)
           indent = indent - 4
       elif verb == "say":
           text = raw.split(None, 1)[1]
           name = text.split("(")[0]
           if name in functions or name in known:
               lines.append(" " * indent + f"print({text})")
           else:
               lines.append(" " * indent + f'print("{text}")')
       else:
           lines.append(" " * indent + "# unknown: " + raw)

   source = "\n".join(lines)
   file_write("out.py", source)

   show "compiled mini language:"
   show source
   show ""
   show "running out.py:"
   exec(open("out.py").read())
   ```

5. Run it — no server and no input needed:

   ```sh
   nme r functions
   ```

   ```text
   compiled mini language:
   def double(n):
       return n * 2
   print(double(21))
   def add(a, b):
       return a + b
   print(add(2, 3))
   print("done")

   running out.py:
   42
   5
   done
   ```

   The generated Python defines both functions, calls them, and prints
   `42` and `5` — the mini language now has functions, compiled into real
   Python and run.

6. Korean writes the same compiler with `파일 사용 최신`, `말해`, and
   Korean report lines; the mini language keeps its English verbs. The full
   Korean program is in the [Korean guide](81-functions.ko.md).

## Try it yourself

Add a `call` verb that stores a result (`call double 21 -> r` becomes
`r = double(21)`), or a `sub` verb that lowers to `-`. Then make `def`
usable after other verbs — the signature table makes that a two-line
change — and open `out.py`: it is plain Python, runnable with
`python out.py`.

## What you learned

- A signature table (`functions`) records each function's parameters.
- `','.join(params)` turns a parameter list into a Python `def` header.
- `text.split("(")[0]` tells a call apart from a plain word.
- `raw.split(None, 1)[1]` keeps the whole argument — a space-splitting
  tokenizer would lose commas, which is why real ones don't split blindly.
- Functions are the step that turns a verb list into a real language.
