#!/usr/bin/env python3
from pathlib import Path


def replace(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(f"anchor changed: {path}: {old[:60]!r}")
    file.write_text(text.replace(old, new, 1), encoding="utf-8")


replace(
    "docs/guides/cryptocurrency.ko.md",
    """영어 문장형은 현재 영지식 모듈의 암호 연산을 호출할 때 `zk_secret()` 같은
정확한 함수 표현식을 사용합니다. 제어 흐름, 저장, 변경, 출력은 영어 문장형으로
씁니다. 현재 컴파일러에는 한국어처럼 구두점 없는 영어 영지식 문구가 따로 없기
때문입니다. 이 차이는 예제에서 숨기지 않습니다.
""",
    """`needmorecoin-sentence.en.nme`도 같은 수준의 순수성 규칙을 적용합니다. 실행
소스에는 **영문자, 십진 숫자, 공백만** 들어가며 밑줄, 따옴표, 괄호, 쉼표,
콜론, 등호, 연산자, Python 함수 호출이나 주석을 넣지 않습니다. `zero knowledge
secret make`, `secret zero knowledge public make` 같은 영어 문장형 암호 문구를
컴파일러가 실제 영지식 연산으로 낮춥니다. 한국어판과 마찬가지로 모든 비어 있지
않은 줄이 실제 NME 문장으로 변환되는지 회귀 테스트가 확인합니다.
""",
)

replace(
    "docs/guides/cryptocurrency.md",
    """### Strong purity rule for the Korean sentence file

`needmorecoin-sentence.ko.nme` contains only Hangul, decimal digits, and
whitespace in executable source. It has no ASCII identifiers, underscores,
quotes, parentheses, commas, colons, operators, Python `import`, or even source
comments. A regression test checks both the character set and that every
non-empty source line is actually lowered by NME, so a Python statement cannot
be hidden behind Korean identifiers and pass the test unchanged.

The English sentence example currently uses exact helper expressions such as
`zk_secret()` when it reaches the cryptographic adapter. Its storage, control
flow, updates, and output use English sentence forms. The compiler currently
has punctuation-free Korean phrases for the zero-knowledge primitives but no
matching punctuation-free English phrases, and the example documents rather
than hides that boundary.
""",
    """### Strong purity rules for both sentence files

`needmorecoin-sentence.ko.nme` contains only Hangul, decimal digits, and
whitespace in executable source. It has no ASCII identifiers, underscores,
quotes, parentheses, commas, colons, operators, Python `import`, or even source
comments.

`needmorecoin-sentence.en.nme` is equally strict: executable source contains
only ASCII letters, decimal digits, and whitespace. It has no underscores,
quotes, parentheses, commas, colons, operators, Python calls, or source
comments. Phrases such as `zero knowledge secret make` and `secret zero
knowledge public make` lower to the real bundled zero-knowledge operations.
Regression tests check both character sets and verify that every non-empty line
in both sentence files is actually lowered by NME, so Python cannot be hidden
inside a file that claims to be pure sentence syntax.
""",
)

replace(
    "docs/guides/example-authoring.ko.md",
    """`needmorecoin-sentence.ko.nme`처럼 특별히 엄격한 한국어 문장형 예제는 한글,
숫자, 공백만 허용할 수도 있습니다. 이런 제약은 설명이 아니라 자동 테스트로
보장하세요.
""",
    """`needmorecoin-sentence.ko.nme`처럼 특별히 엄격한 한국어 문장형 예제는 한글,
숫자, 공백만 허용할 수 있습니다. 영어도 같은 원칙으로
`needmorecoin-sentence.en.nme`처럼 영문자, 숫자, 공백만 허용할 수 있습니다.
이런 제약은 설명이 아니라 자동 테스트로 보장하고, 모든 비어 있지 않은 줄이 실제
NME로 변환되는지도 검사하세요.
""",
)

replace(
    "docs/guides/example-authoring.md",
    """A deliberately strict file such as `needmorecoin-sentence.ko.nme` can go
further and allow only Hangul, decimal digits, and whitespace. Constraints like
that should be enforced by tests, not by comments alone.
""",
    """A deliberately strict Korean file such as `needmorecoin-sentence.ko.nme`
can allow only Hangul, decimal digits, and whitespace. A strict English twin
such as `needmorecoin-sentence.en.nme` can allow only ASCII letters, decimal
digits, and whitespace. Enforce these claims in tests and also verify that every
non-empty line is actually lowered by NME instead of escaping unchanged as
Python.
""",
)

replace(
    "docs/guides/example-template.ko.md",
    """```text
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
""",
    """```text
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

순수 영어 문장형을 주장한다면 영문자·숫자·공백만 허용하는 것처럼 규칙을 먼저
정하고 테스트로 강제하세요. 내장 영지식 기능도 `use zeroknowledge latest`,
`zero knowledge secret make` 같은 구두점 없는 문장형을 사용할 수 있습니다.
""",
)

replace(
    "docs/guides/example-template.md",
    """```text
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

If a bundled feature has no punctuation-free English phrase, document the exact
helper-expression boundary instead of describing a mixed file as fully pure.
""",
    """```text
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
""",
)

replace(
    "docs/language.ko.md",
    """`random`(`랜덤`), 읽기·쓰기·JSON용 `file`(`파일`), 실제 슈노르 지식 증명
계산을 제공하는 `zero_knowledge`(`영지식`)입니다. 각 모듈의 내장 버전은
`0.0.1`입니다. 모듈마다 `사용` 줄 하나면 충분하며 같은 모듈을 두 번
가져오면 충돌 오류가 납니다.
""",
    """`random`(`랜덤`), 읽기·쓰기·JSON용 `file`(`파일`), 실제 슈노르 지식 증명
계산을 제공하는 `zero_knowledge`(`영지식`)입니다. 랜덤과 파일 어댑터는
`0.0.1`, 영지식 어댑터는 `0.0.2`가 내장됩니다. 모듈마다 `사용` 줄 하나면
충분하며 같은 모듈을 두 번 가져오면 충돌 오류가 납니다.
""",
)

replace(
    "docs/language.ko.md",
    """`파일 사용 버전 \"0.0.1\"`. 영지식 어댑터는 `zero_knowledge`/`영지식`을
같은 방식으로 받으며 `영지식 사용 최신`도 사용할 수 있습니다.
""",
    """`파일 사용 버전 \"0.0.1\"`. 영지식 어댑터는 `zero_knowledge`/`영지식`을
같은 방식으로 받으며 `영지식 사용 최신`도 사용할 수 있습니다. 구두점 없는 영어
문장형 소스에서는 밑줄 없는 별칭 `use zeroknowledge latest`도 같은 모듈을
불러옵니다.
""",
)

replace(
    "docs/language.ko.md",
    """완전한 증명 흐름은 괄호 없이 한국어 문장형으로도 쓸 수 있습니다.
`examples/zk-schnorr-relay.ko.nme`를 보세요. 검증 함수는 슈노르 등식을
확인하기 전에 공개값과 약속값의 부분군 소속, 응답 범위, 도전 범위를
검사합니다.
""",
    """완전한 증명 흐름은 괄호 없이 한국어와 영어 문장형으로도 쓸 수 있습니다.
영어에서는 `zero knowledge secret make`, `secret zero knowledge public make`,
`zero knowledge nonce make`, `nonce zero knowledge commitment make`,
`secret context zero knowledge proof make`, `public proof context zero knowledge verify`,
`public commitment context zero knowledge challenge make`를 사용합니다. 순수 영어
문장형 예시는 `examples/needmorecoin-sentence.en.nme`, 한국어 증명 흐름은
`examples/zk-schnorr-relay.ko.nme`를 보세요. 검증 함수는 슈노르 등식을 확인하기
전에 공개값과 약속값의 부분군 소속, 응답 범위, 도전 범위를 검사합니다.
""",
)

replace(
    "docs/language.md",
    """Three beginner modules ship with NME: `random` (dice and picks), `file`
(reading, writing, and JSON), and `zero_knowledge` / `영지식` (a Schnorr
proof-of-knowledge reference implementation). Each has one bundled version,
`0.0.1`. One `use` line per module is enough; importing the same module twice
is a collision error:
""",
    """Three beginner modules ship with NME: `random` (dice and picks), `file`
(reading, writing, and JSON), and `zero_knowledge` / `영지식` (a Schnorr
proof-of-knowledge reference implementation). Random and file are bundled at
`0.0.1`; zero knowledge is bundled at `0.0.2`. One `use` line per module is
enough; importing the same module twice is a collision error:
""",
)

replace(
    "docs/language.md",
    """same forms with `file` / `파일`: `파일 사용`, `파일 사용 최신`, `파일 사용
버전 \"0.0.1\"`. The zero-knowledge adapter uses `zero_knowledge` / `영지식`
with the same forms, including `영지식 사용 최신`.
""",
    """same forms with `file` / `파일`: `파일 사용`, `파일 사용 최신`, `파일 사용
버전 \"0.0.1\"`. The zero-knowledge adapter uses `zero_knowledge` / `영지식`
with the same forms, including `영지식 사용 최신`. Strict punctuation-free
English sentence source may use the alias `use zeroknowledge latest`.
""",
)

replace(
    "docs/language.md",
    """The Korean sentence surface removes function punctuation for the complete
proof flow; see `examples/zk-schnorr-relay.ko.nme`. The verifier validates
subgroup membership and all scalar/challenge ranges before checking the
Schnorr equation.
""",
    """Both Korean and English sentence surfaces can remove function punctuation
for the complete proof flow. English forms include `zero knowledge secret
make`, `secret zero knowledge public make`, `zero knowledge nonce make`, `nonce
zero knowledge commitment make`, `secret context zero knowledge proof make`,
`public proof context zero knowledge verify`, and `public commitment context
zero knowledge challenge make`. See `examples/needmorecoin-sentence.en.nme` for
a strict ASCII-letters/digits/whitespace example and
`examples/zk-schnorr-relay.ko.nme` for the Korean proof flow. The verifier
validates subgroup membership and scalar/challenge ranges before checking the
Schnorr equation.
""",
)

replace(
    "README.ko.md",
    "표시될 버전은 `nme 0.0.1-beta.20`입니다.",
    "표시될 버전은 `nme 0.0.1-beta.22`입니다.",
)
replace(
    "README.ko.md",
    """문장형 파일은 한글·숫자·공백만 사용하도록 자동 검사합니다. 지갑, 문맥에 묶인
거래 증명, 수수료, 거래번호 재전송 방지, SHA-256 작업증명, 이전 해시 연결,
""",
    """문장형 한국어 파일은 한글·숫자·공백만, 문장형 영어 파일은 영문자·숫자·공백만
사용하도록 자동 검사합니다. 두 파일 모두 모든 비어 있지 않은 줄이 실제 NME로
변환되는지도 검사합니다. 지갑, 문맥에 묶인 거래 증명, 수수료, 거래번호 재전송
방지, SHA-256 작업증명, 이전 해시 연결,
""",
)

replace(
    "README.md",
    "Expected version: `nme 0.0.1-beta.20`.",
    "Expected version: `nme 0.0.1-beta.22`.",
)
replace(
    "README.md",
    """The Korean sentence source is regression-tested to contain only Hangul, decimal
digits, and whitespace. The [NeedMoreCoin guide](docs/guides/cryptocurrency.md)
""",
    """The Korean sentence source is regression-tested to contain only Hangul,
decimal digits, and whitespace; the English sentence source is tested to
contain only ASCII letters, decimal digits, and whitespace. Both tests also
require every non-empty line to be lowered by NME. The
[NeedMoreCoin guide](docs/guides/cryptocurrency.md)
""",
)

print("materialized beta22 documentation updates")
