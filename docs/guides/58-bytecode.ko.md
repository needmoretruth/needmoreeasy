# 58 — 컴파일러: 작은 바이트코드 실행기

[English](58-bytecode.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [49 — Tokens](49-tokens.ko.md), [29 — Bootstrap](29-bootstrap.ko.md)
- 주제: 컴파일러
- 결과물: 간단한 명령을 단계 목록으로 컴파일하고 가상 머신처럼 한 단계씩 실행하기

가이드 [29](29-bootstrap.ko.md)는 소스 텍스트를 Python으로 번역해 실행했고,
[49](49-tokens.ko.md)는 줄을 토큰으로 나누어 실행을 보냈습니다. 다음 단계는
**바이트코드**입니다: 이미 작은 데이터 단계로 컴파일된 명령. 실행기 — 작은
**가상 머신** — 은 프로그램 카운터로 그 단계들을 걸어갑니다. Python이 자기
코드를 실행하는 방식입니다.

## 단계

1. 컴파일된 프로그램은 데이터입니다: 명령들의 목록. 각 명령은 목록이며 첫
   요소가 연산, 나머지가 그 인자입니다:

   ```text
   program = [
       ["set", "x", "0"],
       ["add", "x", "2"],
       ["add", "x", "3"],
       ["show", "x"],
   ]
   ```

   `set x 0`는 `x`라는 변수에 0을 저장하고, `add x 2`는 2를 더합니다. 아직
   아무 줄도 실행되지 않습니다 — 이것은 단계들의 *설명*입니다.

2. 실행기가 `pc`(프로그램 카운터)와 `vars` dict(기계의 메모리)로 한 걸음씩
   진행합니다. 반복 한 바퀴마다 `pc`의 명령을 가져오고, 실행하고, `pc`를
   앞으로 움직입니다. `jnz`는 변수가 0이 아닌 동안 다른 `pc`로 점프합니다 —
   이것이 바이트코드 반복이 만들어지는 방식입니다. 전체 실행기,
   `bytecode.ko.nme`로 저장:

   ```text
   # bytecode.ko.nme — 작은 바이트코드 실행기, 미니 가상 머신.
   # 실행: nme 실행 bytecode.ko
   # 각 명령은 목록이며, run()이 프로그램 카운터로 한 걸음씩 실행합니다.

   def run(program):
       vars = {}
       pc = 0
       step = 0
       while pc < len(program):
           instr = program[pc]
           op = instr[0]
           step = step + 1
           if op == "set":
               vars[instr[1]] = int(instr[2])
           elif op == "add":
               vars[instr[1]] = vars[instr[1]] + int(instr[2])
           elif op == "sub":
               vars[instr[1]] = vars[instr[1]] - int(instr[2])
           elif op == "show":
               말해 f"step {step} pc {pc}: {instr[1]} = {vars[instr[1]]}"
           elif op == "jnz":
               if vars[instr[1]] != 0:
                   pc = int(instr[2])
                   continue
           pc = pc + 1
       말해 f"{step}단계 만에 프로그램이 끝났습니다"

   countdown = [
       ["set", "x", "0"],
       ["add", "x", "2"],
       ["add", "x", "3"],
       ["show", "x"],
   ]

   말해 "첫 프로그램:"
   run(countdown)

   말해 "점프로 만든 반복:"
   loop = [
       ["set", "n", "3"],
       ["show", "n"],
       ["sub", "n", "1"],
       ["jnz", "n", "1"],
   ]
   run(loop)
   말해 "완료"
   ```

   `step`은 가져온 명령마다 하나씩 세고, `show`는 현재 `pc`를 알려 줍니다.
   반복에서 `pc`는 `n`이 0이 아닌 동안 1로 돌아가고, `n`이 0이 되면 끝을
   지나 떨어져 나갑니다.

3. 실행하세요:

   ```sh
   nme 실행 bytecode.ko
   ```

   ```text
   첫 프로그램:
   step 4 pc 3: x = 5
   4단계 만에 프로그램이 끝났습니다
   점프로 만든 반복:
   step 2 pc 1: n = 3
   step 5 pc 1: n = 2
   step 8 pc 1: n = 1
   10단계 만에 프로그램이 끝났습니다
   완료
   ```

   첫 프로그램은 네 단계를 실행했습니다: set, add, add, show. 반복은 열
   단계를 실행했습니다: `jnz`가 기계를 `pc` 1로 세 번 되돌려 보냈고, 그다음
   떨어져 나가게 했습니다.

## 직접 해보기

`cmp`(비교) 명령을 추가해 1이나 0을 저장하고, 변수가 0일 때 점프하는 `jz`를
추가해 보세요.

## 배운 것

- 바이트코드는 이미 작은 데이터 단계의 목록으로 컴파일된 소스입니다.
- 프로그램 카운터(`pc`)가 기계가 다음에 실행할 명령을 말해 줍니다.
- `vars`, 즉 dict가 기계의 메모리입니다. 각 연산이 그것을 읽고 씁니다.
- `jnz`는 `pc`를 바꿔 점프하며, 이것이 가상 머신 안에서 반복이 동작하는
  방식입니다.
