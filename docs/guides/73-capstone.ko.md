# 73 — 캡스톤: Python으로 컴파일하는 언어

[English](73-capstone.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [58 — 바이트코드](58-bytecode.ko.md), [34 — 셀프호스트](34-selfhost.ko.md)
- 주제: 컴파일러/캡스톤
- 결과물: 아주 작은 언어(say/set/add/while/end)를 읽어 Python 소스로 컴파일하고 파일로 저장한 뒤 실행하는 NME 프로그램

[34](34-selfhost.ko.md)은 NME 단어를 Python으로 컴파일해 메모리에서
실행했습니다. [58](58-bytecode.ko.md)은 명령을 데이터 단계로 바꾸었습니다.
캡스톤은 전체 길을 마무리합니다: 아주 작은 언어를 읽고 Python **소스**로
컴파일하고, 그 소스를 `out.py`에 저장하고, 그 파일을 실행합니다 — 컴파일러
프로젝트 전체가 NME 프로그램 하나에 담겨 있습니다.

## 단계

1. 입력은 동사 다섯 개로 된 아주 작은 언어입니다. `say`는 출력하고, `set`은
   숫자를 저장하고, `add`는 더하고, `while 이름 N`은 `이름 < N`인 동안
   반복하며, `end`는 반복을 닫습니다. 프로그램은 줄들의 목록으로
   존재합니다:

   ```text
   [
       "set count 0",
       "while count 3",
       "  add count 1",
       "  say count",
       "end",
       "say done",
   ]
   ```

   이것은 아직 실행되지 않은 소스 텍스트입니다 — 정확히 컴파일러가 읽는
   것입니다.

2. 컴파일러는 각 줄을 Python 한 줄로 바꿉니다. `split()`이 줄을 단어로
   나누고, 첫 단어가 동사, 나머지는 인자입니다. `indent`가 블록 깊이를
   관리합니다: `while`이 4만큼 늘리고, `end`가 줄이며, `" " * indent`가
   Python이 필요로 하는 앞 공백을 씁니다 — [71](71-chart.ko.md)에서 막대를
   그렸던 것과 같은 문자열 곱셈입니다. 전체 컴파일러는 완성된 소스를
   `file_write`로 `out.py`에 저장하고([13](13-files.ko.md)), 파일을 다시 읽어
   `exec`로 실행합니다 — [34](34-selfhost.ko.md)의 실행기가 이제 진짜 파일을
   읽는 것입니다. `capstone.ko.nme`로 저장합니다:

   ```text
   # capstone.ko.nme — Python으로 컴파일하는 언어.
   # 실행: nme 실행 capstone.ko
   # 미니 언어를 읽어 Python 소스로 컴파일하고,
   # out.py로 저장한 뒤 exec로 실행합니다.

   파일 사용 최신

   program = [
       "set count 0",
       "while count 3",
       "  add count 1",
       "  say count",
       "end",
       "say done",
   ]

   known = []
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "say":
           text = parts[1]
           if text in known:
               lines.append(" " * indent + f"print({text})")
           else:
               lines.append(" " * indent + f'print("{text}")')
       elif verb == "set":
           known.append(parts[1])
           lines.append(" " * indent + f"{parts[1]} = {parts[2]}")
       elif verb == "add":
           lines.append(" " * indent + f"{parts[1]} += {parts[2]}")
       elif verb == "while":
           lines.append(" " * indent + f"while {parts[1]} < {parts[2]}:")
           indent = indent + 4
       elif verb == "end":
           indent = indent - 4
       else:
           lines.append(" " * indent + "# 알 수 없는 명령: " + raw)

   source = "\n".join(lines)
   file_write("out.py", source)

   말해 "컴파일된 Python:"
   말해 source
   말해 ""
   말해 "out.py 실행:"
   exec(open("out.py").read())
   ```

   `known` 목록은 심볼 테이블입니다: `set`이 자기 대상들을 기록하고, `say`가
   그것을 확인합니다. 그래서 `say count`는 `print(count)`가 되고, `say done`은
   `print("done")`이 됩니다 — [72](72-project-files.ko.md)에서 가져왔던 이름
   목록의 아주 작은 버전입니다.

3. 서버도 입력도 없이 실행합니다:

   ```sh
   nme 실행 capstone.ko
   ```

   ```text
   컴파일된 Python:
   count = 0
   while count < 3:
       count += 1
       print(count)
   print("done")

   out.py 실행:
   1
   2
   3
   done
   ```

   만들어진 Python은 진짜 Python입니다 — 들여쓰기가 안전하고, `out.py`로
   저장되며, 단독으로 실행할 수 있습니다. 반복이 1, 2, 3을 센 다음 `done`을
   출력합니다.

4. 영어는 같은 컴파일러를 `use file latest`, `show`로 씁니다. 전체 영어
   프로그램은 [영어 가이드](73-capstone.md)에 있습니다.

## 직접 해보기

`-=`로 낮추는 `sub` 동사와, `text`를 N번 출력하는 `say text N` 형식을 더해
보세요 — 반복 번역 힌트는 [34](34-selfhost.ko.md)에 있습니다. 그런 다음
`out.py`를 열어 보세요: 평범한 Python이라 `python out.py`로 단독 실행할 수
있습니다.

## 배운 것

- 컴파일러는 소스 언어의 명령 하나를 목표 언어의 줄 하나로 바꿉니다.
- `indent` 관리가 `while`/`end`를 들여쓰기된 Python 블록으로 만듭니다.
- `known` 목록은 심볼 테이블입니다: `say`가 변수를 단어에서 구분합니다.
- `file_write` 다음 `exec(open(...))`이 컴파일-실행 길을 마무리합니다.
- 동사 다섯 개, 파이프라인 하나 — 컴파일러 프로젝트 전체가 프로그램 하나에.
