# NME 0.0.1-beta.42

Beta.42 closes a documented CLI output-safety gap.

- Make `nme build -o` reject an existing output with the stable bilingual E9009
  diagnostic.
- Leave the existing Python artifact unchanged for English and Korean commands.
- Keep build validation and generated Python behavior unchanged for new output
  paths.
