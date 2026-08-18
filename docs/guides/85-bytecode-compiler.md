# 85 — Compiler tier: from tree to bytecode

English | [한국어](85-bytecode-compiler.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [82 — AST](82-ast.md), [58 — Bytecode](58-bytecode.md)
- Topic: compiler & bytecode
- Result: a compiler that flattens an expression tree into instruction lines and runs them on a stack machine

Guide [82](82-ast.md) evaluated a tree directly; guide [58](58-bytecode.md)
ran pre-made instructions. Real compilers connect the two: **compile** the
tree into a flat instruction list, then let a tiny virtual machine run it.
The tree's nesting becomes the order of the instructions — this is the
pipeline behind every real language.

## Steps

1. A stack machine runs `PUSH number` and `ADD`-style instructions.
   `2 + 3 * 4` compiles to five instructions: push both numbers, push
   the other two, multiply, then add:

   ```text
   ['PUSH 2', 'PUSH 3', 'PUSH 4', 'MUL', 'ADD']
   ```

   The VM keeps a stack: `PUSH` puts a number on top, `MUL` takes the
   top two, multiplies, and pushes the result back. `3 * 4` happens
   before `+` because its instructions come first — the tree's depth
   became the instruction order.

2. Compiling is a recursive walk of the tree from guide [82](82-ast.md).
   A number becomes one `PUSH`; an operation compiles its left side, then
   its right side, then its operator:

   ```text
   def compile(node):
       if node[0] == "num":
           return ["PUSH " + str(node[1])]
       op = node[1]
       left = compile(node[2])
       right = compile(node[3])
       names = {"+": "ADD", "-": "SUB", "*": "MUL", "/": "DIV"}
       return left + right + [names[op]]
   ```

   The order `left`, then `right`, then the operator is not an accident:
   it is exactly what the stack needs, and it is what makes the tree's
   shape come out as instruction order.

3. The VM is a loop with a small stack. Save `bytecode.nme` with the
   tokenizer and parsers of guide [82](82-ast.md):

   ```text
   # bytecode.nme — the full pipeline: tokens -> tree -> instructions -> run.
   # Run: nme r bytecode

   def tokenize(line):
       return line.split()

   def parse_term(tokens):
       node = ["num", int(tokens.pop(0))]
       while tokens and tokens[0] in ("*", "/"):
           op = tokens.pop(0)
           right = ["num", int(tokens.pop(0))]
           node = ["bin", op, node, right]
       return node

   def parse_expr(tokens):
       node = parse_term(tokens)
       while tokens and tokens[0] in ("+", "-"):
           op = tokens.pop(0)
           right = parse_term(tokens)
           node = ["bin", op, node, right]
       return node

   def compile(node):
       if node[0] == "num":
           return ["PUSH " + str(node[1])]
       op = node[1]
       left = compile(node[2])
       right = compile(node[3])
       names = {"+": "ADD", "-": "SUB", "*": "MUL", "/": "DIV"}
       return left + right + [names[op]]

   def run(instructions):
       stack = []
       for ins in instructions:
           parts = ins.split()
           if parts[0] == "PUSH":
               stack.append(int(parts[1]))
           elif parts[0] == "ADD":
               right = stack.pop()
               left = stack.pop()
               stack.append(left + right)
           elif parts[0] == "SUB":
               right = stack.pop()
               left = stack.pop()
               stack.append(left - right)
           elif parts[0] == "MUL":
               right = stack.pop()
               left = stack.pop()
               stack.append(left * right)
           else:
               right = stack.pop()
               left = stack.pop()
               stack.append(left // right)
       return stack[0]

   line = "2 + 3 * 4"
   tree = parse_expr(tokenize(line))
   instructions = compile(tree)
   show f"tree: {tree}"
   show f"instructions: {instructions}"
   show f"value: {run(instructions)}"
   ```

4. Run it:

   ```sh
   nme r bytecode
   ```

   ```text
   tree: ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   instructions: ['PUSH 2', 'PUSH 3', 'PUSH 4', 'MUL', 'ADD']
   value: 14
   ```

   Watch one instruction at a time: `PUSH 2` → stack `[2]`; `PUSH 3` →
   `[2, 3]`; `PUSH 4` → `[2, 3, 4]`; `MUL` pops 3 and 4, pushes 12 →
   `[2, 12]`; `ADD` pops 2 and 12, pushes 14 → `[14]`. The final answer
   sits alone on the stack.

5. Try a line that exercises left-to-right chains:

   ```text
   show run(compile(parse_expr(tokenize("8 / 2 / 2"))))
   ```

   ```text
   2
   ```

   The instruction list is `['PUSH 8', 'PUSH 2', 'DIV', 'PUSH 2', 'DIV']`
   — `((8 / 2) / 2)`, the same tree shape the parser built.

## Try it yourself

Add a `NEG` instruction that negates the top of the stack, or an `DUP`
that copies it — then extend the parser so a `-3` at the start of an
expression compiles to `PUSH 3` + `NEG`. Read the expression from
`sys.argv[1]` (guide [80](80-argv.md)) to turn the whole pipeline into a
tiny calculator command.

## What you learned

- Compiling flattens a tree into instructions; the depth becomes the order.
- `left + right + [op]` per node is the whole of tree-to-bytecode.
- A stack machine needs only `PUSH` and one instruction per operator.
- The VM pops two, computes, pushes one — `ADD` vs `SUB` is one branch.
- Tokens → tree → instructions → run is the complete compiler pipeline.
