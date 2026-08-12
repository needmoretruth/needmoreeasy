# NME 0.0.1-beta.57

Beta.57 hardens native handling of Python comments.

- Emit source comments as inert C comments before compiling the generated C.
- Prevent comment text from becoming a C preprocessor directive or affecting
  the scan that hoists native functions to file scope.
