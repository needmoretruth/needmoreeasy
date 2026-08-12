# NME 0.0.1-beta.123

Beta123 extends the restricted native backend with one-line NME output bodies
for condition blocks and branch chains.

- Lower `if`/`while` bodies after `then` and `else if`/`else` bodies after
  `then`/`그러면` when the statement is `say`/`show`/`말해`.
- Keep inline branches in the same native flow tracker as indented blocks.
- Continue rejecting Python inline bodies and inline value updates outside the
  documented native subset.
