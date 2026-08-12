# NME 0.0.1-beta.60

Beta.60 fixes native binding analysis across sibling branches.

- A branch cannot read a name introduced only by another `if`/`else` branch.
- Branch-specific bindings are kept for C declaration/type checking, then
  conservatively become maybe-initialized after uncertain control flow.
