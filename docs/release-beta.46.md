# NME 0.0.1-beta.46

Beta.46 fixes native function scope tracking.

- Keep function-local scalar bindings separate from main-program bindings.
- Allow a name such as `local` to be used independently inside a function and
  in the main program without emitting invalid C.
