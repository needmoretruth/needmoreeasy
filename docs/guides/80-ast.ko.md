# 80 — 컴파일러: 식을 트리로 만들기

[English](80-ast.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [78 — 표현식](78-expressions.ko.md), [79 — 토큰](79-tokens.ko.md)
- 주제: 컴파일러/AST
- 결과물: 식을 트리로 파싱하고 재귀로 평가해 우선순위를 지키는 계산기

[78](78-expressions.ko.md)은 `2 + 3 * 4`를 한 번에 평가했고,
[82](82-bytecode.ko.md)은 명령을 데이터로 납작하게 만듭니다. 진짜
컴파일러는 그 중간을 합니다: 소스를 **트리**로 바꾸고 — 추상 구문
트리, AST — 트리를 평가합니다. 곱셈이 덧셈보다 트리의 깊은 곳에
매달리므로, 그래서 `*`가 먼저 계산됩니다.

## 단계

1. 노드는 작은 목록입니다. 숫자는 `["num", 값]`, 연산은
   `["bin", 연산자, 왼쪽, 오른쪽]`. 식 `2 + 3 * 4`는 이렇게 됩니다:

   ```nme
   ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   ```

   안에서 밖으로 읽어 보세요: `3 * 4`가 `+` 아래에 매달린 부분 트리라서
   먼저 평가됩니다 — 트리가 우선순위를 모양으로 만듭니다.

2. 트리를 만들려면 함수가 두 개 필요합니다. `*`가 `+`보다 더 강하게
   붙기 때문입니다. `parse_term`이 `*`와 `/` 사슬을 모으고,
   `parse_expr`이 그 항(term)들의 `+`와 `-` 사슬을 모읍니다.
   `ast.nme`로 저장하세요:

   ```nme
   # ast.nme — 식을 트리로 만들고 트리를 평가하기.
   # 실행: nme 실행 ast

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

   def evaluate(node):
       kind = node[0]
       if kind == "num":
           return node[1]
       op = node[1]
       left = evaluate(node[2])
       right = evaluate(node[3])
       if op == "+":
           return left + right
       if op == "-":
           return left - right
       if op == "*":
           return left * right
       return left // right

   line = "2 + 3 * 4"
   tokens = tokenize(line)
   tree = parse_expr(tokens)
   show f"tree: {tree}"
   show f"value: {evaluate(tree)}"
   ```

   각 `while` 루프는 숫자에서 시작해 같은 연산자를 왼쪽에 이어 붙이므로
   `8 / 2 / 2`는 `((8 / 2) / 2)`가 됩니다 — Python처럼 왼쪽에서 오른쪽.
   `evaluate`는 자식에게 자기 자신을 부르는 재귀로,
   [66](66-native.ko.md)에서 팩토리얼을 계산한 것과 같은 방식입니다.

3. 실행하세요:

   ```sh
   nme 실행 ast
   ```

   ```text
   tree: ['bin', '+', ['num', 2], ['bin', '*', ['num', 3], ['num', 4]]]
   value: 14
   ```

   `*` 부분 트리가 `+` 아래에 매달려 있어서 `evaluate`가 `3 * 4`를 먼저
   계산하고 `2 + 12`로 `14`가 됩니다. 곱셈이 먼저인 이유는 트리의
   모양 때문입니다 — `evaluate`에는 특별한 코드가 없습니다.

4. 모양 규칙을 두 줄 더 확인하세요:

   ```nme
   show f"left-assoc: {evaluate(parse_expr(tokenize('8 / 2 / 2')))}"
   ```

   ```text
   left-assoc: 2
   ```

   `8 / 2 / 2`는 `((8 / 2) / 2)`이지 `8 / (2 / 2)`가 아닙니다. 파서 두
   함수가 만든 트리와 `evaluate`가 걷는 트리는 항상 같은 모양입니다 —
   그것이 바로 AST의 핵심입니다.

## 직접 해보기

`parse_term`의 while에 `"%"`를 넣고 `if op == "%"` 분기를 추가해 `%`
연산자도 항(term) 수준에서 받아 보세요. 또는 한 줄 대신 들여쓰기로
트리를 출력하는 `show_tree` 함수를 만들어 보세요. 그리고 소스를
`sys.argv[1]`에서 읽게 바꿔 보세요([63](63-argv.ko.md)): `nme 실행 ast "2 + 3 * 4"`.

## 배운 것

- AST 노드는 그냥 목록입니다: `["num", 값]` 또는 `["bin", 연산자, 왼쪽, 오른쪽]`.
- 파싱 단계 두 개가 우선순위를 모양으로 만듭니다: `*` 부분 트리가
  `+` 아래에 매달립니다.
- `evaluate`가 트리를 재귀로 걸으며, 트리가 순서를 정합니다.
- 왼쪽에서 오른쪽 사슬은 각 루프가 왼쪽에 이어 붙여 만들어집니다.
- 파싱 후 평가 — 진짜 컴파일러 뒤에 있는 두 단계 파이프라인입니다.
