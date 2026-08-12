# NME 0.0.1-beta.77

Beta.77 keeps default native artifacts distinct for English and Korean twins.

- Append `.c` to the full source stem when `nme native build` has no `-o`, so
  `count.ko.nme` produces `count.ko.c` rather than colliding with `count.c`.
- Add `.exe` to implicit Windows outputs even when the source stem ends in
  `.ko`.
- Preserve the existing extension replacement behavior for explicit `-o`
  paths and cover the bilingual sibling workflow with a CLI regression test.
