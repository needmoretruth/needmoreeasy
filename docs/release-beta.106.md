# NME 0.0.1-beta.106

Beta106 fixes native branch data-flow analysis.

- A name assigned in every `if`/`else` branch is now available after the block.
- One-sided assignments and names created inside loops that may not run remain
  conditional, with English and Korean regression coverage.
