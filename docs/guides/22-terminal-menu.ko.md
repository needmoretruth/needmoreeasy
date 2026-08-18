# 22 — 터미널 메뉴 — 작은 TUI

[English](22-terminal-menu.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★☆☆ (3/5)
- 선수 지식: [10 — 랜덤](10-random.ko.md), [06 — 조건](06-if.ko.md), [07 — 동안](07-while.ko.md)
- 주제: 터미널 메뉴
- 결과물: 반복으로 움직이는 터미널 메뉴

TUI(text user interface)는 키보드로 조종하는 메뉴입니다. 예제
`examples/terminal-menu.ko.nme`는 선택지 세 개를 보여 주고, 답을 기다리고,
고른 일을 하고, 다시 메뉴를 보여 줍니다. 이 프로젝트는 학습용입니다.

실행하고 답 두 개를 넣어 보세요 — `1`은 인사하고, `3`은 종료합니다.
첫 답 뒤에 반복이 메뉴로 돌아가므로 메뉴가 두 번 보입니다:

```sh
printf '1\n3\n' | nme 실행 examples/terminal-menu.ko
```

```text
1) 인사
2) 주사위
3) 종료
고르세요: 안녕하세요!
1) 인사
2) 주사위
3) 종료
고르세요: 안녕히 가세요
```

## 단계

1. `use random latest` 줄이 주사위 함수를 불러오고, 글자 값이 메뉴를
   담습니다. `\n`은 "새 줄"입니다 — 한 문자열이 세 줄이 되는 이유입니다:

   ```nme
   # examples/terminal-menu.ko.nme의 일부
   use random latest

   menu = "1) 인사\n2) 주사위\n3) 종료"
   ```

2. `while True:`는 끝없는 반복을 만들고, `show menu`가 선택지를 출력하며
   `ask 선택, "고르세요: "`가 답을 저장합니다:

   ```nme
   # examples/terminal-menu.ko.nme의 일부
   while True:
       show menu
       ask 선택, "고르세요: "
   ```

   블록은 들여쓰기로 시작하며 Python과 똑같습니다.

3. [06](06-if.ko.md) 가이드의 `if`/`elif`/`else`가 답마다 한 갈래를
   실행합니다. 순수 Python 헤더 `if 선택 == "1":`가 NME 줄
   `show`/`break`와 같은 블록 안에 자유롭게 섞입니다:

   ```nme
   # examples/terminal-menu.ko.nme의 일부
   while True:
       show menu
       ask 선택, "고르세요: "
       if 선택 == "1":
           show "안녕하세요!"
       elif 선택 == "2":
           show random_number(1, 6)
       else:
           show "안녕히 가세요"
           break
   ```

   `show random_number(1, 6)`은 그 자리에서 [10](10-random.ko.md) 가이드의
   주사위를 굴립니다. 다른 답은 `else`로 떨어져 "안녕히 가세요"를 출력하고
   `break`로 반복을 떠납니다 — `while True:`에서 나가는 유일한 길입니다.

4. `nme 검사`는 반복을 실행하지 않고 문법만 확인합니다:

   ```sh
   nme 검사 examples/terminal-menu.ko
   ```

   영어 쌍둥이 `examples/terminal-menu.nme`는 같은 `while True:` 반복에
   `ask choice, "choose: "`를 씁니다. `nme 실행 examples/terminal-menu`로
   실행하고 같은 숫자를 고르면 영어로 같은 흐름이 나옵니다.

## 직접 해보기

`menu`에 `4) 동전` 줄을 추가하고, `elif 선택 == "4":` 갈래를 새로 만들어
두 면 중 무작위로 고른 쪽을 보여 주세요 — [10](10-random.ko.md) 가이드에
방법이 있습니다. `break`는 그대로 작동하고, 다른 숫자는 갈래만 하나
늘어납니다.

## 배운 것

- `while True:`는 영원히 반복하며 `break`가 나가는 길입니다.
- 메뉴는 보여 주기 → 물어보기 → 갈라지기 → 다시 반복입니다.
- `show`/`ask` NME 줄이 순수 Python `if 선택 == "1":` 헤더와 섞입니다.
- 문자열 안의 `\n`은 새 줄을 만듭니다.
