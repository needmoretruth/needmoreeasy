# NME 0.0.1-beta.115

Beta115 adds parenthesized whole-condition headers to NME's shared logical
condition grammar.

- Accept a colon-free NME header such as `if (ready and score > 2)` and its
  Korean equivalent `만약 (준비 그리고 점수 > 2)`.
- Keep the English and Korean forms on the same condition tree and cover
  sentence, beginner, and advanced surfaces in the native backend matrix.
- Preserve valid Python calls such as `when(ready and score > 2)` byte-for-byte
  rather than interpreting them as NME headers.
- Explain the syntax in the language reference, native reference, AI handoff,
  and their Korean counterparts.
