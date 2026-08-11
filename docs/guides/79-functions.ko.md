# 79 — 컴파일러: 미니 언어에 함수 넣기

[English](79-functions.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [73 — Capstone](73-capstone.md), [23 — Modules](23-modules.md)
- 주제 (Topic): 컴파일러/함수 / compiler & functions
- 결과물 (Result): `def`/`return`/호출을 가진 미니 언어를 Python으로 컴파일하고 실행하는 컴파일러 / a compiler that translates a mini language with `def`, `return`, and calls into Python and runs it

[73](73-capstone.md)은 다섯 동사를 Python으로 컴파일했습니다. 진짜 언어에는
함수가 있으므로, 이번 컴파일러에는 함수가 생깁니다: `def 이름 매개변수`가
함수를 열고, `return 식`이 함수를 끝내고, `say 이름(인자)`가 함수를
호출합니다. 새로 나오는 것은 **시그니처 표** — 컴파일러가 함수마다
매개변수를 기억해서 진짜 Python `def` 줄을 만들 수 있게 해 주는 표입니다.

## 단계

1. 미니 언어에 동사가 두 개 늘어납니다. `def`는 함수 이름과 매개변수를
   정하고, `return`은 답을 돌려주며, `say`는 함수를 호출할 수 있습니다:

   ```text
   [
       "def double n",
       "    return n * 2",
       "say double(21)",
       "def add a b",
       "    return a + b",
       "say add(2, 3)",
       "say done",
   ]
   ```

   `double(21)`은 "double 함수를 21로 호출한다"는 뜻입니다. 들여쓴
   `return`은 몸통을 표시합니다 — [73](73-capstone.md)의 `while` 몸통과
   똑같습니다.

2. 컴파일러는 표를 두 개 유지합니다. `known`은 여전히 변수를 나열하고
   ([73](73-capstone.md)), 새 dict `functions`는 함수 이름마다 매개변수
   목록을 담습니다:

   ```text
   functions = {}
   ```

   `def` 줄에서는 시그니처를 저장하고 Python 헤더를 만듭니다. `return`
   줄에서는 return 문을 만들고 몸통 블록을 닫습니다:

   ```text
   elif verb == "def":
       name = parts[1]
       params = parts[2:]
       functions[name] = params
       lines.append(" " * indent + f"def {name}({', '.join(params)}):")
       indent = indent + 4
   elif verb == "return":
       expr = " ".join(parts[1:])
       lines.append(" " * indent + "return " + expr)
       indent = indent - 4
   ```

   `', '.join(params)`는 `["a", "b"]`를 텍스트 `a, b`로 바꿉니다 —
   [45](45-csv.md)에서 CSV 줄을 만들 때 쓴 것과 같은 join입니다.

3. `say`는 이제 호출을 알아봐야 합니다. `(` 앞의 단어가 함수 이름이고,
   `functions`에 있으면 호출 전체가 식이므로 `print`가 따옴표 없이
   받습니다:

   ```text
   elif verb == "say":
       text = raw.split(None, 1)[1]
       name = text.split("(")[0]
       if name in functions or name in known:
           lines.append(" " * indent + f"print({text})")
       else:
           lines.append(" " * indent + f'print("{text}")')
   ```

   `raw.split(None, 1)[1]`은 첫 공백 뒤 전부를 가져오므로
   `say add(2, 3)`의 쉼표가 보존됩니다 — 평범한 `split()`은 공백에서
   잘라 버립니다. 진짜 컴파일러가 모든 줄을 단순히 단어로 나누지 않는
   이유입니다. `say done`에는 `(`이 없고 `done`은 두 표 어디에도
   없으므로 따옴표 붙은 텍스트로 출력됩니다 — 이전과 같은 폴백입니다.

4. 전체 컴파일러는 캡스톤과 똑같이 `out.py`를 쓰고 `exec`로 실행합니다.
   `functions.nme`로 저장하세요:

   ```text
   # functions.nme — 함수가 있는 미니 언어를 Python으로 컴파일하기.
   # 실행: nme r functions
   # 미니 언어를 읽어 Python 소스로 컴파일하고,
   # out.py를 쓴 다음 exec로 실행합니다.

   use file latest

   program = [
       "def double n",
       "    return n * 2",
       "say double(21)",
       "def add a b",
       "    return a + b",
       "say add(2, 3)",
       "say done",
   ]

   known = []
   functions = {}
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "def":
           name = parts[1]
           params = parts[2:]
           functions[name] = params
           lines.append(" " * indent + f"def {name}({', '.join(params)}):")
           indent = indent + 4
       elif verb == "return":
           expr = " ".join(parts[1:])
           lines.append(" " * indent + "return " + expr)
           indent = indent - 4
       elif verb == "say":
           text = raw.split(None, 1)[1]
           name = text.split("(")[0]
           if name in functions or name in known:
               lines.append(" " * indent + f"print({text})")
           else:
               lines.append(" " * indent + f'print("{text}")')
       else:
           lines.append(" " * indent + "# unknown: " + raw)

   source = "\n".join(lines)
   file_write("out.py", source)

   show "compiled mini language:"
   show source
   show ""
   show "running out.py:"
   exec(open("out.py").read())
   ```

5. 서버도 입력도 필요 없이 실행하세요:

   ```sh
   nme r functions
   ```

   ```text
   compiled mini language:
   def double(n):
       return n * 2
   print(double(21))
   def add(a, b):
       return a + b
   print(add(2, 3))
   print("done")

   running out.py:
   42
   5
   done
   ```

   생성된 Python은 두 함수를 정의하고 호출하여 `42`와 `5`를 출력합니다 —
   미니 언어에 함수가 생겼고, 진짜 Python으로 컴파일되어 실행되었습니다.

## 직접 해보기

결과를 저장하는 `call` 동사를 추가해 보세요 (`call double 21 -> r`은
`r = double(21)`이 됩니다). 또는 `-`로 낮아지는 `sub` 동사를 넣어
보세요. 그리고 `def`를 다른 동사 뒤에서도 쓸 수 있게 바꿔 보세요 —
시그니처 표가 있으면 두 줄만 바꾸면 됩니다. `out.py`를 열어 보세요:
`python out.py`로 그대로 실행할 수 있는 평범한 Python입니다.

## 배운 것

- 시그니처 표(`functions`)가 함수마다 매개변수를 기록합니다.
- `','.join(params)`가 매개변수 목록을 Python `def` 헤더로 바꿉니다.
- `text.split("(")[0]`이 호출과 평범한 단어를 구분해 줍니다.
- `raw.split(None, 1)[1]`이 인자 전체를 보존합니다 — 공백 기준으로
  나누는 토크나이저는 쉼표를 잃게 되고, 그래서 진짜 컴파일러는 무작정
  나누지 않습니다.
- 함수는 동사 목록을 진짜 언어로 만드는 한 단계입니다.
