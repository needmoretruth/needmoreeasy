# How to write strong NME examples

[한국어](example-authoring.ko.md) | English

[Guide index](index.md) | [Example template](example-template.md) | [Language reference](../language.md)

This is the baseline for adding examples that are easy to read, run, compare,
and extend. The goal is not to maximize the amount of code. A strong example
teaches one central idea accurately, and its claimed syntax level matches the
source that users actually see.

## 1. Write the learning goal first

Complete this sentence before writing code:

> After running this example, a learner can see ___ and change ___ themselves.

If the blanks contain several unrelated concepts, split the example. A larger
capstone may combine skills, but it should still have one central result.

## 2. Choose a primary syntax level and language

NME levels are not modes; a file can mix them. Educational examples should
still have a clear primary level so comparisons remain useful.

### Sentence

- A first-time reader should be able to read it aloud.
- Avoid quotes, parentheses, commas, colons, and operators when the language
  already has a sentence form.
- Prefer conversational storage, output, input, conditions, loops, and `end`.
- If one feature requires an exact Python expression, document that boundary.
- If the example claims to be *pure* sentence syntax, enforce the claim in a test.

A deliberately strict Korean file such as `needmorecoin-sentence.ko.nme`
can allow only Hangul, decimal digits, and whitespace. A strict English twin
such as `needmorecoin-sentence.en.nme` can allow only ASCII letters, decimal
digits, and whitespace. Enforce these claims in tests and also verify that every
non-empty line is actually lowered by NME instead of escaping unchanged as
Python.

### Beginner

- Use compact, explicit NME rather than full conversational sentences.
- Quotes, simple operators, colons, indentation, `set`, and `say` are appropriate.
- Avoid complex Python data structures or function definitions when they do
  not make the lesson clearer.
- When a Python expression is used as a value, make sure it clarifies rather
  than hides the concept.

### Advanced

- Advanced NME is valid Python and should be idiomatic Python.
- Functions, classes, type hints, and standard-library modules are appropriate.
- A Korean advanced example should not pretend Python keywords have Korean
  replacements. Use Korean identifiers, strings, and comments around real
  Python syntax.
- An English advanced example should follow normal Python naming and style.

## 3. Match observable meaning across variants

Korean/English and sentence/beginner/advanced twins should preserve meaning,
not line count. Keep these aligned:

- initial state
- important inputs
- success conditions
- failure conditions
- important output
- security or data-validation rules

Do not force identical variable names or decomposition. An advanced version may
be clearer when it introduces functions and data structures.

## 4. Name multi-level example sets predictably

For a new six-file project, prefer:

```text
project-sentence.ko.nme
project-sentence.en.nme
project-beginner.ko.nme
project-beginner.en.nme
project-advanced.ko.nme
project-advanced.en.nme
```

Follow an existing local convention when compatibility matters. New sets should
make language and level obvious from the filename.

## 5. Make every example checkable and runnable

At minimum, these should succeed:

```sh
nme check examples/project
nme run examples/project
```

If an example depends on interaction or a network, separate a deterministic
core or provide a fixed-input path for automated testing.

## 6. Show failure paths, not just success

If the example teaches a rule, include something that violates that rule.

- file example: malformed or missing data
- authentication example: wrong credentials
- blockchain example: tampering and replay
- parser example: rejected input

A security claim is much easier to understand when learners can see the attack
that the rule rejects.

## 7. Separate facts from simplifications

Avoid claims such as:

- “secure” when security was not actually tested
- “distributed network” for a single process
- “production cryptography” for a toy signature
- “faster” without measurement

State explicitly what is real, what is simplified, what is omitted, and what a
next production-oriented step would require.

## 8. Keep dependencies small

Prefer bundled NME features and the Python standard library. If an external
package is necessary, document the install command, supported version range,
and why the dependency is justified.

## 9. Test invariants instead of random outputs

Do not assert one random hash or one random number. Assert properties that must
remain true:

- a mined hash satisfies the target
- tampered data fails validation
- a balance sum equals supply
- a parser rejects an invalid shape

A bad test asserts one accidental output:

```text
assert observed_hash == "1234"
```

A better test checks properties that survive changing random inputs:

```text
assert mined_hash < work_target
assert not validate(tampered_data)
assert sum(balances) == total_supply
```

## 10. Give learners specific edits to try

End a guide with at least three experiments, such as:

- change one number
- add one condition
- extend one feature

Explain what should change when each experiment is correct.

## 11. Minimum regression-test standard

Where practical, a new example set should test:

1. Every file transpiles successfully.
2. Advanced files remain valid Python.
3. Translation/level pairs retain their essential feature markers.
4. A claimed pure-sentence file has an allow-list or token-level purity check.
5. A claimed pure-NME file does not silently pass core lines through unchanged as Python.
6. Security examples reject at least one tampered or invalid case.

When exact syntax classification matters, prefer parser/transpiler evidence over
fragile substring heuristics.

## 12. Review checklist

- [ ] The learning goal fits in one sentence.
- [ ] Filename communicates language and level.
- [ ] `nme check` succeeds.
- [ ] `nme run` exits normally.
- [ ] Documentation matches actual output and behavior.
- [ ] The level label matches the syntax used.
- [ ] Korean and English variants preserve the same core meaning.
- [ ] At least one failure or invalid-input path is exercised.
- [ ] Security, performance, and networking scope is not overstated.
- [ ] Learners are given concrete edits to try.
- [ ] Compiler tests accompany any new language feature.

## Recommended workflow

1. Copy the [example template](example-template.md).
2. Finish one language and one syntax level first.
3. Lock down baseline behavior with `nme check` and `nme run`.
4. Add a failure case.
5. Translate the meaning into the other language.
6. Expand sentence → beginner → advanced when the project benefits from a level matrix.
7. Add shared regression tests.
8. Link the set from README and the guide index.
9. Re-read every claim in the documentation against the final code.

Examples are executable explanations of the language. Small, precise, and
testable examples age better than large examples that only look impressive.
