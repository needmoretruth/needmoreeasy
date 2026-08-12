# NME 0.0.1-beta.43

Beta.43 hardens the native backend’s fixed-capacity string runtime.

- Replace unbounded `strcpy`/`strcat` operations with checked copies and
  concatenation.
- Stop stored or concatenated values larger than 8191 UTF-8 bytes with a
  bilingual runtime error instead of overflowing generated C buffers.
- Document the native limit and point unrestricted text to the CPython path.
