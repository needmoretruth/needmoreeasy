# 81 — Testing: checking the functions you wrote

English | [한국어](81-testing.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [23 — Modules](23-modules.md), [46 — Expressions](46-expressions.md)
- Topic: testing
- Result: a tiny test runner that calls your own functions, compares each result with the expected value, and reports pass or fail

"Wait — I said it works without actually testing it, didn't I?" Most
programmers ask themselves that question. Tests answer it: a program that
calls your functions with expected values and compares the results. Write
it once, and every later change gets re-checked by rerunning it.

## Steps

1. A test is a row of data: a name, a function, the arguments, and the
   expected result. `add(2, 3)` should return `5`:

   ```nme
   ["adds numbers", add, 2, 3, 5]
   ```

2. The runner walks the rows, calls each function, and compares. The
   function lives in the row as a value — the same way `random_number`
   was a value in guide [10](10-random.md). Save `testcalc.nme`:

   ```nme
   # testcalc.nme — check the functions you wrote with a tiny test runner.
   # Run: nme r testcalc

   def add(a, b):
       return a + b

   def mul(a, b):
       return a * b

   tests = [
       ["adds numbers", add, 2, 3, 5],
       ["adds negatives", add, -1, 1, 0],
       ["multiplies", mul, 3, 4, 12],
       ["multiplies by zero", mul, 9, 0, 0],
   ]

   passed = 0
   for test in tests:
       name = test[0]
       function = test[1]
       result = function(test[2], test[3])
       expected = test[4]
       if result == expected:
           passed = passed + 1
           show f"PASS {name}"
       else:
           show f"FAIL {name}: got {result}, expected {expected}"

   show f"{passed} of {len(tests)} tests passed"
   ```

3. Run it:

   ```sh
   nme r testcalc
   ```

   ```text
   PASS adds numbers
   PASS adds negatives
   PASS multiplies
   PASS multiplies by zero
   4 of 4 tests passed
   ```

   Every `PASS` line is a promise kept: the function did what the test
   expected. `FAIL` would print the real result next to the expected one,
   so a broken function names itself instead of failing silently.

4. Now break something on purpose to see the runner catch it. Change
   `mul` to `return a - b` and run again:

   ```text
   FAIL multiplies: got -1, expected 12
   FAIL multiplies by zero: got 9, expected 0
   PASS adds numbers
   PASS adds negatives
   2 of 4 tests passed
   ```

   The runner found the bug and even pointed at which expectations broke —
   no manual checking, no silent surprise. Undo the change before
   continuing.

5. A test runner protects a growing project. Put the functions in a module
   (guide [23](23-modules.md)) and the tests in the main file, then run the
   tests after every change:

   ```nme
   # from "calc.nme" import add, mul   (the module version)
   ```

   The test rows stay exactly the same; only the import line changes. Now
   every improvement to `calc.nme` gets checked by the same runner.

## Try it yourself

Add rows for subtraction and division (`add` a `sub` and a `div` function),
including edge cases like dividing by zero — decide first what `div` should
do with 0, then write a test that says it. Add a counter that prints
`all tests passed` only when `passed == len(tests)`.

## What you learned

- A test row is data: name, function, arguments, expected result.
- A test runner calls each function, compares, and reports PASS or FAIL.
- A failing test shows the real result next to the expected one.
- Tests turn "I think it works" into "the runner says it works".
- Running the same tests after every change catches regressions early.
