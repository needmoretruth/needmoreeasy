# NME 0.0.1-beta.85

Beta.85 makes the Python-packages guide match the shipped package-install
workflow.

- Teach `nme install requests` and the equivalent `nme 설치 requests` command
  instead of sending beginners directly to platform-specific pip syntax.
- Explain that the wrapper chooses the normal Python command, installs one
  package at a time, and reports E9025 when pip fails.
- Keep the English and Korean package-learning paths aligned.

Language semantics and generated Python behavior are unchanged.
