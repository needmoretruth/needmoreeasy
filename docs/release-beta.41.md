# NME 0.0.1-beta.41

Beta.41 makes temporary staging cleanup reliable across all CLI outcomes.

- Own imported-module, native, and Nuitka staging directories until the
  operation ends.
- Remove partial staging files when module, Python, or C source writes fail.
- Keep the existing bilingual diagnostics and generated program behavior
  unchanged.
