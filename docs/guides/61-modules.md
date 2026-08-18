# 61 — Modules: split your program into files

English | [한국어](61-modules.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [23 — High score](23-high-score.md), [37 — Files](37-files.md)
- Topic: modules
- Result: splitting a program across .nme files

One file works for a small program. As a project grows, splitting it into
modules keeps each file focused. NME imports only the names you list, so every
module has a clear interface and nothing else leaks between files.

## Steps

1. Put shared functions in a module file. A module is an ordinary `.nme` file
   that defines values instead of running a whole program:

   ```nme
   # shapes.nme
   def rect(width, height):
       return width * height


   def circle(radius):
       return 3.14 * radius * radius
   ```

2. Import the names you need in the main program:

   ```nme
   # area.nme
   from "shapes.nme" import rect, circle

   show rect(4, 5)
   show circle(2)
   ```

3. Run the main program; NME finds `shapes.nme` next to it:

   ```sh
   nme run area
   ```

   ```text
   20
   12.56
   ```

   The ready-made pair lives in
   [`examples/modules/`](../../examples/modules/) — `area.nme` and
   `shapes.nme`, with `_ko` Korean twins.

4. The import name list is the interface. Only `rect` and `circle` cross into
   `area.nme`; anything else in the module stays private. Imported names can
   be used in sentences like any other value:

   ```nme
   from "shapes.nme" import rect
   show rect(3, 7)
   ```

## Rules to remember

- The module file sits next to the main program.
- The file name is a Python identifier: `shapes.nme`, not `my-shapes.nme`.
- Imports can chain: a module may import another module.
- `nme check` and `nme build` check imported modules too.

## Try it yourself

Add a `perimeter(width, height)` function to `shapes.nme`, import it in
`area.nme`, and show the perimeter of a 4 by 5 rectangle.

## What you learned

- `from "helper.nme" import name1, name2` imports only the listed names.
- The module is a normal `.nme` file in the same folder.
- Imported names work in sentences and calls like local values.
- A clear interface means no hidden global state between files.
