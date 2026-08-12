# NME 0.0.1-beta.112

Beta112 adds logical conditions to the restricted native backend.

- Native NME block conditions can combine supported operands with `and` and
  `or`; the shared parser keeps Python precedence, and generated C preserves
  short-circuit evaluation.
- Korean `그리고` and `또는` use the same language-neutral condition tree and
  native lowering path. Python-colon conditions remain outside the native core.
- Add six English/Korean sentence, beginner, and advanced regression cases,
  bilingual short-circuit tests, paired runnable examples, and synchronized
  native references and guides.
