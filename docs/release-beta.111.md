# NME 0.0.1-beta.111

Beta111 fixes native fall-through analysis for nested terminating conditionals.

- A nested conditional with no reachable fall-through arm now terminates its
  enclosing conditional branch for binding analysis. A later return can use a
  name assigned on every path that can reach it.
- Loops remain conservative because they may execute zero times, and native
  functions still require a top-level integer `return`.
- Add English/Korean sentence, beginner, and mixed advanced regression coverage
  and synchronize the native references, guide, and language reference.
