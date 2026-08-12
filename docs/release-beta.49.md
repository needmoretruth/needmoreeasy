# NME 0.0.1-beta.49

Beta.49 makes native conditional initialization explicit.

- Track names first assigned inside control blocks.
- Reject reads or value changes after a possibly skipped block unless the name
  was initialized beforehand; a literal `if true` block is known to run.
