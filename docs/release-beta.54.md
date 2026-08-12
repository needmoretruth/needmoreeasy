# NME 0.0.1-beta.54

Beta.54 makes the native function scope explicit.

- Accept native function definitions at file scope only.
- Reject nested definitions before generating C and point users to CPython for
  the broader Python function model.
