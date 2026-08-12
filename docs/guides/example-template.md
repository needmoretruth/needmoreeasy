# NME example template

[한국어](example-template.ko.md) | English

[How to write strong examples](example-authoring.md) | [Guide index](index.md)

Copy this page when starting a new example and fill in the relevant sections.
You may remove sections that do not apply, but keep the learning goal, run
commands, failure case, learner experiments, and verification method whenever
possible.

## 1. Design card

```text
Project name:
One-sentence goal:
Primary syntax level: sentence / beginner / advanced
Language: Korean / English / both
Prerequisites:
Visible result after running:
Intentional failure case:
What is implemented for real:
What is simplified or omitted:
External dependencies: none / name and reason
```

## 2. File set

Start with one file when one level is enough. When a six-way comparison is
useful, prefer:

```text
examples/project-sentence.ko.nme
examples/project-sentence.en.nme
examples/project-beginner.ko.nme
examples/project-beginner.en.nme
examples/project-advanced.ko.nme
examples/project-advanced.en.nme
```

## 3. Korean sentence skeleton

```text
이름은 예제
값은 0

이름 시작합니다 말해줘

동안 값이 3보다 작을 동안
값 말해줘
값에 1 더해
끝

만약에 값이 3과 같으면
정상 결과입니다 말해줘
끝
```

If you claim a strict pure-sentence source, define the allowed character or
token set and enforce it in a regression test.

## 4. English sentence skeleton

```text
name save Example
value save 0

show name is starting

while value is less than 3
show value
add 1 to value
end

if value is equal to 3
show The result is valid
end
```

For a strict English sentence example, define an allowed source alphabet such
as ASCII letters, decimal digits, and whitespace, then enforce it in tests. The
bundled zero-knowledge adapter also has punctuation-free forms such as `use
zeroknowledge latest` and `zero knowledge secret make`.

## 5. Korean beginner skeleton

```text
저장 이름 "예제"
저장 값 0

말해 f"{이름} 시작"

동안 값 < 3:
    말해 값
    저장 값 값 + 1

만약 값 == 3:
    말해 "정상 결과"
```

## 6. English beginner skeleton

```text
set name to "Example"
set value to 0

say f"{name} starts"

while value < 3:
    set value to value + 1
end

when value == 3:
    say "The result is valid"
```

## 7. Korean advanced skeleton

Advanced NME is Python. Keep real Python keywords and use Korean identifiers,
strings, and comments when the example's language is Korean.

```python
from dataclasses import dataclass


@dataclass
class 기록:
    값: int


def 검증(기록값: 기록) -> bool:
    return 기록값.값 >= 0


현재 = 기록(3)
assert 검증(현재)
print(f"검증 결과 {현재.값}")
```

## 8. English advanced skeleton

```python
from dataclasses import dataclass


@dataclass
class Record:
    value: int


def validate(record: Record) -> bool:
    return record.value >= 0


current = Record(3)
assert validate(current)
print(f"Validated value {current.value}")
```

## 9. Guide document skeleton

```markdown
# Project name

- Difficulty:
- Prerequisites:
- Result:

## What you are building

Describe the result in one paragraph.

## What is real

- item 1
- item 2

## What is simplified

- item 1

## Run it

Use exact commands.

## Steps

1. smallest initial state
2. core feature
3. validation
4. failure case

## Try changing it

1. change one number
2. add one condition
3. extend one feature

## Properties to verify

- invariant that must remain true
- attack or invalid input that must fail
```

## 10. Regression-test skeleton

```rust
use nme_core::transpile;

#[test]
fn example_transpiles() {
    let source = include_str!("../../../examples/project.nme");
    let python = transpile(source).expect("example should transpile");
    assert!(!python.is_empty());
}
```

For strict sentence examples, add an allow-list and a check that core source
lines do not escape unchanged as Python. For security examples, test invariants
or rejection conditions rather than one random output value.

## 11. Final repository checks

```sh
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Also run `nme check` and `nme run` for each example.

The goal is not to fill every template section mechanically. Remove anything
that does not help the reader reach the central idea quickly and accurately.
