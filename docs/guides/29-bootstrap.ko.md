# 29 — 부트스트랩: NME가 아주 작은 언어를 컴파일

[English](29-bootstrap.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [28 — Compiler](28-compiler.md), [23 — Modules](23-modules.md)
- 주제 (Topic): 부트스트랩 / bootstrap
- 결과물 (Result): NME로 쓴 미니 컴파일러 / a tiny compiler written in NME

가이드 [28](28-compiler.md)은 줄을 직접 해석했습니다. 다음 단계는 진짜
컴파일러의 씨앗입니다: 작은 언어를 다른 언어로 **번역**하고 결과를
실행하는 프로그램. NME 안에서 컴파일러를 쓰는 것을 부트스트래핑이라고
하며, 모든 진짜 컴파일러가 이렇게 자랍니다. `examples/bootstrap.nme`가
정확히 그 일을 합니다.

## 단계

1. 다섯 명령의 아주 작은 언어를 정의하세요. BML(초보자 미니 언어)은 줄마다
   명령 하나를 유지합니다:

   ```text
   set count 0
   while count 3
     show hello
     add count 1
   end
   show done
   ```

2. 줄마다 읽어 각 명령을 Python으로 번역하세요. `show`는 `print`가 되고,
   `set`은 대입이 되며, `while`은 블록을 엽니다. 들여쓰기가 블록 깊이를
   관리합니다:

   ```text
   # part of examples/bootstrap.nme
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "set":
           lines.append(" " * indent + f"{parts[1]} = {parts[2]}")
       elif verb == "show":
           lines.append(" " * indent + f'print("{parts[1]}")')
       elif verb == "while":
           lines.append(" " * indent + f"while {parts[1]} < {parts[2]}:")
           indent = indent + 4
       elif verb == "end":
           indent = indent - 4
   ```

3. 번역한 줄을 이어 붙여 실행하세요. NME가 CPython 위에서 실행되므로
   컴파일러가 자신이 만든 결과를 실행할 수 있습니다:

   ```text
   # part of examples/bootstrap.nme
   source = "\n".join(lines)
   exec(source)
   ```

4. 전체 프로그램을 실행하세요:

   ```sh
   nme r examples/bootstrap
   ```

   ```text
   generated Python:
   count = 0
   while count < 3:
       print("hello")
       count += 1
   print("done")
   running it:
   hello
   hello
   hello
   done
   ```

   NME로 쓴 컴파일러가 BML을 Python으로 번역했고, Python이 그것을
   실행했습니다.

## 직접 해보기

번역기에 `sub <name> <int>` 명령(Python에서 `-=`)을 추가하고, 5부터 1까지
거꾸로 세는 BML 프로그램을 써 보세요.

## 배운 것

- 컴파일러는 소스 텍스트를 다른 언어로 번역한 뒤 실행합니다.
- `split()`이 줄을 단어로, `f"..."`가 출력 줄을 만듭니다.
- 들여쓰기 깊이가 중첩 블록을 관리합니다.
- NME 안에서 컴파일러를 쓰는 것이 부트스트래핑 — 셀프호스팅의 씨앗입니다.
