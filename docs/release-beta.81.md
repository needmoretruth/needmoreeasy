# NME 0.0.1-beta.81

Beta.81 preserves UTF-8 text in Windows native builds.

- MSVC native build and test invocations now pass `/utf-8`.
- Generated Korean and English runtime messages and string literals therefore
  retain their intended text under the Windows compiler path.
