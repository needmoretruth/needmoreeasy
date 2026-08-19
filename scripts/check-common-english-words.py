#!/usr/bin/env python3
"""Reports whether `COMMON_ENGLISH_WORDS` still matches the compiler's words.

The list is generated (see `scripts/build-common-english-words.py`), so adding
an English word to the compiler changes which real English words sit one typo
away from it. Without this check the list would quietly drift, and the words
it protects — `shop`, `well`, `bell` — would start being read as misspellings
again.

Skips itself on a machine with no English dictionary installed.
"""

import runpy
import sys

sys.argv = [sys.argv[0], "--check"]
runpy.run_path(
    __file__.replace("check-common-english-words.py", "build-common-english-words.py"),
    run_name="__main__",
)
