# 58 — Compiler tier: a tiny bytecode runner

English | [한국어](58-bytecode.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [49 — Tokens](49-tokens.md), [29 — Bootstrap](29-bootstrap.md)
- Topic: compiler
- Result: compiling simple instructions into a list of steps and running them one by one like a tiny virtual machine

Guide [29](29-bootstrap.md) translated source text into Python and ran it;
[49](49-tokens.md) split lines into tokens and dispatched them. The next step
is **bytecode**: instructions already compiled into small data steps. A runner
— a tiny **virtual machine** — walks them with a program counter, the way
Python runs its code.

## Steps

1. A compiled program is data: a list of instructions, each a list whose first
   element is the operation and the rest its arguments:

   ```text
   program = [
       ["set", "x", "0"],
       ["add", "x", "2"],
       ["add", "x", "3"],
       ["show", "x"],
   ]
   ```

   `set x 0` stores 0 into a variable named `x`; `add x 2` adds 2. No line runs
   yet — this is a *description* of steps.

2. The runner steps through with a `pc` (program counter) and a `vars` dict,
   the machine's memory. Each loop turn fetches the instruction at `pc`, does
   it, and moves `pc` forward. `jnz` jumps to another `pc` while a variable is
   not zero — that is how a bytecode loop is built. The full runner, saved as
   `bytecode.nme`:

   ```text
   # bytecode.nme — a tiny bytecode runner, a mini virtual machine.
   # Run: nme r bytecode
   # Each instruction is a list; run() steps through with a program counter.

   def run(program):
       vars = {}
       pc = 0
       step = 0
       while pc < len(program):
           instr = program[pc]
           op = instr[0]
           step = step + 1
           if op == "set":
               vars[instr[1]] = int(instr[2])
           elif op == "add":
               vars[instr[1]] = vars[instr[1]] + int(instr[2])
           elif op == "sub":
               vars[instr[1]] = vars[instr[1]] - int(instr[2])
           elif op == "show":
               show f"step {step} pc {pc}: {instr[1]} = {vars[instr[1]]}"
           elif op == "jnz":
               if vars[instr[1]] != 0:
                   pc = int(instr[2])
                   continue
           pc = pc + 1
       show f"program finished in {step} steps"

   countdown = [
       ["set", "x", "0"],
       ["add", "x", "2"],
       ["add", "x", "3"],
       ["show", "x"],
   ]

   show "first program:"
   run(countdown)

   show "loop with a jump:"
   loop = [
       ["set", "n", "3"],
       ["show", "n"],
       ["sub", "n", "1"],
       ["jnz", "n", "1"],
   ]
   run(loop)
   show "done"
   ```

   `step` counts every fetched instruction; `show` reports the current `pc`.
   In the loop, `pc` returns to 1 while `n` is not zero, then falls off the
   end when `n` reaches 0.

3. Run it:

   ```sh
   nme r bytecode
   ```

   ```text
   first program:
   step 4 pc 3: x = 5
   program finished in 4 steps
   loop with a jump:
   step 2 pc 1: n = 3
   step 5 pc 1: n = 2
   step 8 pc 1: n = 1
   program finished in 10 steps
   done
   ```

   The first program ran four steps: set, add, add, show. The loop ran ten:
   `jnz` sent the machine back to `pc` 1 three times, then let it fall through.

## Try it yourself

Add a `cmp` (compare) instruction that stores 1 or 0, then a `jz` that jumps
when a variable *is* zero.

## What you learned

- Bytecode is source already compiled into a list of small data steps.
- A program counter (`pc`) says which instruction the machine runs next.
- `vars`, a dict, is the machine's memory; each op reads and writes it.
- `jnz` jumps by changing `pc`, which is how loops work inside a virtual
  machine.
