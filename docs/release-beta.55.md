# NME 0.0.1-beta.55

Beta.55 hardens native string literals.

- Escape newlines, tabs, quotes, backslashes, and other control characters for
  valid generated C.
- Reject embedded NUL strings because the native C string runtime cannot
  preserve them faithfully.
