# 83 — 게임: 비밀 코드 맞히기 (마스터마인드)

[English](83-mastermind.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [10 — Random](10-random.md), [44 — Playlist](44-playlist.md)
- 주제 (Topic): 게임/논리 / game & logic
- 결과물 (Result): 4가지 색의 비밀 코드를 맞히고 검정·하양 단서로 좁혀 가는 마스터마인드 / a Mastermind-style game that hides a 4-color code and gives black/white feedback for each guess

지금까지의 추측 게임은 "크다" 또는 "작다"만 답했습니다. 마스터마인드는
한 번에 단서를 두 개 줍니다: 색이 맞고 **자리도 맞은** 개수(검정),
색은 맞지만 자리가 다른 개수(하양). 이 단서를 올바르게 계산하는 것은
작지만 진짜 알고리즘 사고입니다.

## 단계

1. 비밀 코드는 색 목록에서 네 번 고른 것입니다 — [10](10-random.md)의
   `random_pick`을 네 번 부릅니다:

   ```text
   use random latest

   colors = ["red", "blue", "green", "yellow"]

   secret = []
   for i in range(4):
       secret.append(random_pick(colors))
   ```

   `append`가 목록을 색 하나씩 키웁니다 — [44](44-playlist.md)에서
   플레이리스트를 만든 것과 같은 목록 쌓기 루프입니다.

2. 검정은 정확히 일치하는 자리를 셉니다 — 같은 색, 같은 위치. 추측
   색마다 비밀 색과 비교하세요:

   ```text
   black = 0
   for i in range(4):
       if parts[i] == secret[i]:
           black = black + 1
   ```

3. 하양이 더 까다롭습니다: 색은 맞지만 자리가 틀린 것들. 정직한
   방법은 두 목록에 모두 있는 색을 센 다음, 검정을 빼는 것입니다:

   ```text
   white = 0
   for color in colors:
       white = white + min(parts.count(color), secret.count(color))
   white = white - black
   ```

   `parts.count("red")`는 한 목록을, `secret.count("red")`는 다른
   목록을 셉니다. `min`이 작은 쪽을 택합니다 — 세 번 추측한 색이 비밀에
   한 번만 있어도 한 번으로 칩니다. 검정을 빼면 자리가 틀린 것만
   남습니다. 이것이 표준 마스터마인드 피드백 알고리즘입니다.

4. 전체 게임은 추측을 받고 검증하며, 검정 4개에서 멈춥니다.
   `mastermind.nme`로 저장하세요:

   ```text
   # mastermind.nme — 4색 비밀 코드 맞히기.
   # 실행: nme r mastermind

   use random latest

   colors = ["red", "blue", "green", "yellow"]

   secret = []
   for i in range(4):
       secret.append(random_pick(colors))

   turns = 0
   while True:
       ask guess, "guess 4 colors (red blue green yellow): "
       parts = guess.split()
       if len(parts) != 4:
           show "please type exactly 4 colors"
           continue
       turns = turns + 1
       black = 0
       for i in range(4):
           if parts[i] == secret[i]:
               black = black + 1
       white = 0
       for color in colors:
           white = white + min(parts.count(color), secret.count(color))
       white = white - black
       show f"black: {black}  white: {white}"
       if black == 4:
           show f"solved in {turns} turns"
           break
   ```

5. 실행하세요. 스크립트로 넣은 한 판(추측 세 개를 파이프로 넣고 게임이
   끝남)에서 비밀이 `["yellow", "red", "blue", "red"]`일 때:

   ```sh
   printf 'red blue green white\nblue yellow red white\nyellow red blue red\n' | nme r mastermind
   ```

   ```text
   guess 4 colors (red blue green yellow): black: 0  white: 2
   guess 4 colors (red blue green yellow): black: 0  white: 3
   guess 4 colors (red blue green yellow): black: 4  white: 0
   solved in 3 turns
   ```

   처음 두 추측은 yellow와 red가 어딘가 잘못된 자리에 있음을 알려
   주고, 세 번째 줄에서 모든 자리가 일치합니다. 파이프 없이 프로그램을
   실행하면 새 코드가 숨으니, 직접 해보세요.

## 직접 해보기

잘못된 색 단어를 다시 묻는 대신 실패로 세어 보세요. 또는 `tries`
제한을 두고 다 쓰면 "you lost"를 출력하세요. 색을 두 개 더 늘리고
5개를 묻게 바꿔 보세요 — 피드백 알고리즘은 전혀 바뀌지 않습니다.

## 배운 것

- 루프 안의 `random_pick`이 어떤 길이의 무작위 비밀도 만듭니다.
- 검정 피드백은 자리별 비교입니다.
- 색마다 `min(개수, 개수)`이 공통 색을 정직하게 세는 방법입니다.
- 하양에서 검정을 빼면 정확히 자리가 틀린 것만 남습니다.
- `continue` 가드가 잘못된 입력을 점수 루프에 못 들어오게 합니다.
