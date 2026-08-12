# NME 0.0.1-beta.44

Beta.44 makes native C namespace conflicts explicit.

- Reject C keywords and generated runtime names before lowering to C.
- Report precise bilingual diagnostics for names such as `nme_copy`, `nme_cat`,
  `len`, and `_nme_i` instead of letting C compilation fail later.
- Keep user identifiers unchanged and point unsupported native names toward the
  documented native subset.
