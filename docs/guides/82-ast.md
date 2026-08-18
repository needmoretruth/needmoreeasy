# 82 — Compiler tier: expressions as trees

English | [한국어](82-ast.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [46 — Expressions](46-expressions.md), [49 — Tokens](49-tokens.md)
- Topic: compiler & AST
- Result: a calculator that parses an expression into a tree and evaluates the tree recursively, respecting precedence

Guide [46](46-expressions.md) evaluated `2 + 3 * 4` in one pass; guide
[58](58-bytecode.md) flattened instructions into data. Real compilers do
something in between: they turn the source into a **tree** — the abstract
syntax tree, or AST — and evaluate the tree. Multiplication hangs deeper
in the tree than addition, which is exactly how `*` wins.

## Steps

1. A node is a small list. A number is `["num", value]`; an operation is
   `["bin", op, left, right]`. The expression `2 + 3 * 4` becomes:

   ```text
   ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   ```

   Reading it from the inside out: `3 * 4` is a subtree hanging below
   `+`, so it is evaluated first — the tree makes precedence a shape.

2. Building the tree needs two functions, because `*` binds tighter than
   `+`. `parse_term` collects `*` and `/` chains; `parse_expr` collects
   `+` and `-` chains of terms. Save `ast.nme`:

   ```text
   # ast.nme — expressions as trees: parse, then evaluate the tree.
   # Run: nme r ast

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

   def evaluate(node):
       kind = node[0]
       if kind == "num":
           return node[1]
       op = node[1]
       left = evaluate(node[2])
       right = evaluate(node[3])
       if op == "+":
           return left + right
       if op == "-":
           return left - right
       if op == "*":
           return left * right
       return left // right

   line = "2 + 3 * 4"
   tokens = tokenize(line)
   tree = parse_expr(tokens)
   show f"tree: {tree}"
   show f"value: {evaluate(tree)}"
   ```

   Each `while` loop starts from a number and glues the same operator onto
   the left, so `8 / 2 / 2` builds `((8 / 2) / 2)` — left to right, the
   way Python does. `evaluate` calls itself for the children, a recursion
   like the one that computed factorials in guide [25](25-native.md).

3. Run it:

   ```sh
   nme r ast
   ```

   ```text
   tree: ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   value: 14
   ```

   The `*` subtree hangs under `+`, so `evaluate` computes `3 * 4` first
   and `2 + 12` gives `14`. Multiply happens first because of the tree's
   shape — no special-case code in `evaluate`.

4. Check the shape rules with two more lines:

   ```text
   show f"left-assoc: {evaluate(parse_expr(tokenize('8 / 2 / 2')))}"
   ```

   ```text
   left-assoc: 2
   ```

   `8 / 2 / 2` is `((8 / 2) / 2)`, not `8 / (2 / 2)`. The tree built by
   the two parser functions and the tree walked by `evaluate` always
   agree, because both are the same shape — that is the whole point of an
   AST.

## Try it yourself

Add `%` as a term-level operator (`"%"` in the `parse_term` while loop
and an `if op == "%"` branch), or a `show_tree` function that prints the
tree with indentation instead of one long line. Then change the source to
read from `sys.argv[1]` (guide [80](80-argv.md)): `nme r ast "2 + 3 * 4"`.

## What you learned

- An AST node is just a list: `["num", value]` or `["bin", op, left, right]`.
- Two parsing levels make precedence a shape: `*` subtrees hang below `+`.
- `evaluate` walks the tree recursively — the tree decides the order.
- Left-to-right chains come from building each loop onto the left.
- Parse-then-evaluate is the two-stage pipeline behind real compilers.
