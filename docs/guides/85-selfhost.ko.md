# 85 — 셀프호스트: NME가 NME를 실행

[English](85-selfhost.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [84 — 부트스트랩](84-bootstrap.ko.md)
- 주제: 언어 만들기
- 결과물: say/set/while로 된 아주 작은 NME형 부분집합을 Python으로 컴파일하는 NME 프로그램

[84](84-bootstrap.ko.md)은 BML이라는 아주 작은 언어를 컴파일했습니다. 이 가이드는
이미 NME 단어인 `say`, `set`, `while`, `end`로 된 부분집합을 컴파일합니다 —
NME가 NME를 실행하는 씨앗입니다.

## 단계

1. 미니 언어 프로그램입니다 — 줄마다 명령 하나, NME 단어로:

   ```text
   set count 0
   while count 3
     say 안녕
     add count 1
   end
   say 끝
   ```

   `while count 3`은 "count < 3인 동안 반복"이고, `end`가 블록을 닫습니다.

2. 컴파일러 전체입니다. `selfhost.ko.nme`로 저장합니다:

   ```nme
   # selfhost.ko.nme — 아주 작은 NME형 언어를 컴파일하는 NME
   # 미니 언어는 NME 단어를 씁니다: say <글자>, set <이름> <정수>,
   # add <이름> <정수>, while <이름> <정수>, end
   # 실행: nme 실행 selfhost.ko

   program = [
       "set count 0",
       "while count 3",
       "  say 안녕",
       "  add count 1",
       "end",
       "say 끝",
   ]

   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "say":
           lines.append(" " * indent + f'print("{parts[1]}")')
       elif verb == "set":
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
   말해 "만들어진 Python:"
   말해 source
   말해 ""
   말해 "실행 결과:"
   exec(source)
   ```

   `split()`이 각 줄을 단어로 나누고, `indent`가 블록 깊이를 관리하며, `exec(source)`가 만들어진 Python을 실행합니다 — [84](84-bootstrap.ko.md)의 방법입니다.

3. 서버도 입력도 없이 실행합니다:

   ```sh
   nme 실행 selfhost.ko
   ```

   ```text
   만들어진 Python:
   count = 0
   while count < 3:
       print("안녕")
       count += 1
   print("끝")

   실행 결과:
   안녕
   안녕
   안녕
   끝
   ```

   NME로 쓴 컴파일러가 NME 단어를 읽어 CPython에서 실행했습니다 — NME가 NME를 컴파일하는 씨앗입니다.

4. 영어는 같은 컴파일러를 `show`로 씁니다; 전체 프로그램은 [영어 가이드](85-selfhost.md)에 있습니다.

## 직접 해보기

`say hi 3`처럼 세 번 출력하는 형식을 더해 보세요. 힌트: `for _ in range(3): print("hi")`로 번역.

## 배운 것

- NME 단어로 된 미니 언어는 NME 자신에 더 가깝습니다.
- `say`, `set`, `add`, `while`, `end`가 각각 Python 한 줄로 번역됩니다.
- NME형 소스를 읽는 컴파일러는 NME가 NME를 실행하는 씨앗입니다.
