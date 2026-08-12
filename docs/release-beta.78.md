# NME 0.0.1-beta.78

Beta.78 allows default native builds for source stems ending in `.c`.

- A source named `count.c.nme` can use `count.c` as its implicit executable and
  `count.c.c` as its generated C source.
- Keep the explicit `-o count.c` collision guard so a requested executable
  path cannot overwrite the generated C artifact.
