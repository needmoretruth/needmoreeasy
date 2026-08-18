# 83 — 컴파일러: 트리에서 바이트코드로

[English](83-bytecode-compiler.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [80 — AST](80-ast.ko.md), [82 — 바이트코드](82-bytecode.ko.md)
- 주제: 컴파일러/바이트코드
- 결과물: 식 트리를 납작한 명령 목록으로 컴파일하고 스택 가상 머신으로 실행하는 컴파일러

[80](80-ast.ko.md)는 트리를 직접 평가했고, [82](82-bytecode.ko.md)은 미리
만든 명령을 실행했습니다. 진짜 컴파일러는 둘을 잇습니다: 트리를 납작한
명령 목록으로 **컴파일**하고, 작은 가상 머신이 실행합니다. 트리의
중첩이 명령의 순서가 됩니다 — 모든 진짜 언어 뒤에 있는 파이프라인입니다.

## 단계

1. 스택 머신은 `PUSH 숫자`와 `ADD` 같은 명령을 실행합니다.
   `2 + 3 * 4`는 다섯 명령으로 컴파일됩니다: 두 숫자를 넣고, 나머지
   두 개를 넣고, 곱하고, 더합니다:

   ```nme
   ['PUSH 2', 'PUSH 3', 'PUSH 4', 'MUL', 'ADD']
   ```

   VM은 스택을 유지합니다: `PUSH`는 숫자를 맨 위에 놓고, `MUL`은 위
   두 개를 꺼내 곱한 뒤 결과를 다시 놓습니다. `3 * 4`의 명령이 먼저
   오므로 `+`보다 먼저 계산됩니다 — 트리의 깊이가 명령 순서가
   되었습니다.

2. 컴파일은 [80](80-ast.ko.md) 트리의 재귀 순회입니다. 숫자는 `PUSH`
   하나가 되고, 연산은 왼쪽, 오른쪽, 연산자 순서로 컴파일됩니다:

   ```nme
   def compile(node):
       if node[0] == "num":
           return ["PUSH " + str(node[1])]
       op = node[1]
       left = compile(node[2])
       right = compile(node[3])
       names = {"+": "ADD", "-": "SUB", "*": "MUL", "/": "DIV"}
       return left + right + [names[op]]
   ```

   `left`, `right`, 연산자 순서는 우연이 아닙니다. 스택이 필요한
   바로 그 순서이고, 트리의 모양이 명령 순서로 나오는 이유입니다.

3. VM은 작은 스택을 가진 루프입니다. [80](80-ast.ko.md)의 토크나이저와
   파서와 함께 `bytecode.nme`로 저장하세요:

   ```nme
   # bytecode.nme — 전체 파이프라인: 토큰 -> 트리 -> 명령 -> 실행.
   # 실행: nme 실행 bytecode

   def tokenize(line):
       return line.split()

   def parse_term(tokens):
       node = ["num", int(tokens.pop(0))]
       while tokens and tokens[0] in ("*", "/"):
           op = tokens.pop(0)
           right = ["num", int(tokens.pop(0))]
           node = ["bin", op, node, right]
       return node

   def parse_expr(tokens):
       node = parse_term(tokens)
       while tokens and tokens[0] in ("+", "-"):
           op = tokens.pop(0)
           right = parse_term(tokens)
           node = ["bin", op, node, right]
       return node

   def compile(node):
       if node[0] == "num":
           return ["PUSH " + str(node[1])]
       op = node[1]
       left = compile(node[2])
       right = compile(node[3])
       names = {"+": "ADD", "-": "SUB", "*": "MUL", "/": "DIV"}
       return left + right + [names[op]]

   def run(instructions):
       stack = []
       for ins in instructions:
           parts = ins.split()
           if parts[0] == "PUSH":
               stack.append(int(parts[1]))
           elif parts[0] == "ADD":
               right = stack.pop()
               left = stack.pop()
               stack.append(left + right)
           elif parts[0] == "SUB":
               right = stack.pop()
               left = stack.pop()
               stack.append(left - right)
           elif parts[0] == "MUL":
               right = stack.pop()
               left = stack.pop()
               stack.append(left * right)
           else:
               right = stack.pop()
               left = stack.pop()
               stack.append(left // right)
       return stack[0]

   line = "2 + 3 * 4"
   tree = parse_expr(tokenize(line))
   instructions = compile(tree)
   show f"tree: {tree}"
   show f"instructions: {instructions}"
   show f"value: {run(instructions)}"
   ```

4. 실행하세요:

   ```sh
   nme 실행 bytecode
   ```

   ```text
   tree: ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   instructions: ['PUSH 2', 'PUSH 3', 'PUSH 4', 'MUL', 'ADD']
   value: 14
   ```

   명령 하나씩 따라가 보세요: `PUSH 2` → 스택 `[2]`; `PUSH 3` →
   `[2, 3]`; `PUSH 4` → `[2, 3, 4]`; `MUL`이 3과 4를 꺼내 12를
   놓음 → `[2, 12]`; `ADD`가 2와 12를 꺼내 14를 놓음 → `[14]`.
   최종 답이 스택에 혼자 남습니다.

5. 왼쪽에서 오른쪽 사슬을 시험하는 줄도 실행해 보세요:

   ```nme
   show run(compile(parse_expr(tokenize("8 / 2 / 2"))))
   ```

   ```text
   2
   ```

   명령 목록은 `['PUSH 8', 'PUSH 2', 'DIV', 'PUSH 2', 'DIV']` —
   `((8 / 2) / 2)`, 파서가 만든 것과 같은 트리 모양입니다.

## 직접 해보기

스택 맨 위를 부호 바꾸는 `NEG` 명령이나 복사하는 `DUP`를 추가해
보세요. 그리고 파서를 확장해 식 앞의 `-3`이 `PUSH 3` + `NEG`로
컴파일되게 하세요. `sys.argv[1]`에서 식을 읽으면([63](63-argv.ko.md))
전체 파이프라인이 작은 계산기 명령이 됩니다.

## 배운 것

- 컴파일은 트리를 명령으로 납작하게 만들고, 깊이가 순서가 됩니다.
- 노드마다 `left + right + [연산자]`가 트리→바이트코드의 전부입니다.
- 스택 머신은 `PUSH`와 연산자당 명령 하나만 있으면 됩니다.
- VM은 둘을 꺼내 계산하고 하나를 놓습니다 — `ADD`와 `SUB`의 차이는
  분기 하나입니다.
- 토큰 → 트리 → 명령 → 실행이 완전한 컴파일러 파이프라인입니다.
