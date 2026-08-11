# NME 예제 템플릿

[English](example-template.md) | 한국어

[좋은 예제 만드는 법](example-authoring.ko.md) | [가이드 목록](index.ko.md)

새 예제를 만들 때 이 문서를 복사해서 빈칸을 채우세요. 필요 없는 항목은 지워도
되지만, **학습 목표·실행 방법·실패 사례·직접 바꿔 보기·검증 방법**은 남기는 것을
권장합니다.

## 1. 설계 카드

```text
프로젝트 이름:
한 문장 목표:
주 문법 단계: 문장형 / 초급 / 고급
언어: 한국어 / 영어 / 둘 다
선수 지식:
실행 후 보이는 결과:
일부러 실패시킬 사례:
실제로 구현하는 부분:
단순화하거나 생략하는 부분:
외부 의존성: 없음 / 이름과 이유
```

## 2. 파일 세트

한 단계만 필요하면 한 파일로 시작합니다. 세 단계와 두 언어를 모두 비교할 가치가
있다면 다음 틀을 사용합니다.

```text
examples/프로젝트-sentence.ko.nme
examples/프로젝트-sentence.en.nme
examples/프로젝트-beginner.ko.nme
examples/프로젝트-beginner.en.nme
examples/프로젝트-advanced.ko.nme
examples/프로젝트-advanced.en.nme
```

## 3. 문장형 한국어 뼈대

아래는 구조 예시입니다. 프로젝트에 필요한 줄만 남기세요.

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

순수 문장형을 주장한다면 먼저 허용할 문자를 정하고 테스트로 고정합니다.

## 4. 문장형 영어 뼈대

```text
set name to Example
set value to 0

show name is starting

while value is less than 3
show value
add 1 to value
end

if value is equal to 3
show The result is valid
end
```

필요한 내장 기능이 구두점 없는 영어 문구를 제공하지 않는다면 정확한 helper
표현식이 들어가는 경계를 가이드에 적으세요.

## 5. 초급 한국어 뼈대

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

문장형보다 구조가 짧고 정확해지는 대신 복잡한 Python 코드를 갑자기 넣지 않습니다.

## 6. 초급 영어 뼈대

```text
set name to "Example"
set value to 0

say f"{name} starts"

while value
    set value to value + 1
end

when value == 3:
    say "The result is valid"
```

## 7. 고급 한국어 뼈대

고급 NME는 Python입니다. Python 키워드를 번역하지 않고 한국어 식별자와 설명을
사용합니다.

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

## 8. 고급 영어 뼈대

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

## 9. 가이드 문서 틀

```markdown
# 프로젝트 이름

- 난이도:
- 선수 지식:
- 결과물:

## 무엇을 만드는가

한 문단으로 결과를 설명합니다.

## 실제로 구현하는 것

- 항목 1
- 항목 2

## 단순화한 것

- 항목 1

## 실행

실제 명령을 적습니다.

## 단계

1. 가장 작은 시작 상태
2. 핵심 기능
3. 검증
4. 실패 사례

## 직접 바꿔 보기

1. 숫자 하나 변경
2. 조건 하나 추가
3. 기능 하나 확장

## 확인할 성질

- 항상 참이어야 하는 조건
- 실패해야 하는 공격 또는 잘못된 입력
```

## 10. 테스트 틀

Rust 회귀 테스트를 추가할 수 있다면 다음 구조를 출발점으로 사용합니다.

```rust
use nme_core::transpile;

#[test]
fn example_transpiles() {
    let source = include_str!("../../../examples/project.nme");
    let python = transpile(source).expect("example should transpile");
    assert!(!python.is_empty());
}
```

순수 문장형을 검증하려면 문자 허용 목록과 “변환되지 않고 그대로 빠져나간 줄” 검사도
추가합니다. 보안 예제라면 가능하면 실행 결과보다 내부 불변식 또는 거부 조건을
테스트합니다.

## 11. 올리기 전 마지막 확인

```sh
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

그리고 예제마다 `nme check`와 `nme run`을 직접 실행합니다.

템플릿을 그대로 채우는 것이 목표는 아닙니다. 독자가 가장 짧은 경로로 핵심 개념을
이해하도록 불필요한 부분을 지우는 것도 좋은 예제 설계입니다.
