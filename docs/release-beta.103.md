# NME 0.0.1-beta.103

Beta103 keeps the native finite-float subset finite at runtime.

- Generated C checks `+`, `-`, and `*` results before storing, printing, or
  branching on them.
- A non-finite result exits with a bilingual runtime error instead of exposing
  C `double` infinity.
- One six-case native regression covers sentence, beginner, and advanced
  English and Korean surfaces.
