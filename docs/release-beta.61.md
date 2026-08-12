# NME 0.0.1-beta.61

Beta.61 aligns native string length with Python text semantics.

- Native `len` counts UTF-8 Unicode characters instead of raw storage bytes.
- The native string capacity remains an 8191-byte UTF-8 payload limit.
