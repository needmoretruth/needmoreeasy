# 26 — 모험 — 작은 텍스트 게임

[English](26-adventure.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [22 — 터미널 메뉴](22-terminal-menu.ko.md), [06 — 조건](06-if.ko.md), [10 — 랜덤](10-random.ko.md)
- 주제: 대형 프로젝트
- 결과물: 방을 이동하며 고르는 텍스트 모험

텍스트 모험은 장소를 설명하고 다음에 무엇을 할지 묻는 프로그램입니다. 앞
가이드에서 배운 것이 모두 한곳에 모입니다: `물어봐`가 선택을 읽고,
`만약`/`elif`/`else`가 선택을 나누고, 목록이 인벤토리가 되며, `랜덤 사용`이
위험한 방을 주사위로 결정합니다. 게임 전체가 `.nme` 파일 하나에 들어갑니다.

## 단계

1. 게임 전체를 `adventure.ko.nme` 한 파일로 저장합니다:

   ```text
   # 작은 텍스트 모험: 동굴을 지나 금을 찾아라.
   # 실행: nme 실행 adventure.ko

   랜덤 사용 최신

   inventory = []

   보여줘 "어두운 동굴에서 눈을 뜬다. 횃불 하나가 깜빡인다."

   while True:
       물어봐 action, "무엇을 할까? (look, east, dice, quit) "
       만약 action == "quit":
           보여줘 "동굴을 떠난다. 안녕!"
           break
       elif action == "look":
           보여줘 "입구 동굴: 축축한 돌과 동쪽으로 나는 터널."
           if "torch" not in inventory:
               보여줘 "벽에서 횃불을 집는다."
               inventory.append("torch")
           else:
               보여줘 "벽은 비어 있고 조용하다."
       elif action == "east":
           보여줘 "터널을 따라 돌방에 들어선다."
           if "key" not in inventory:
               보여줘 "선반에 녹슨 열쇠가 놓여 있다. 집는다."
               inventory.append("key")
           else:
               보여줘 "이제 선반은 비어 있다."
           보여줘 "북쪽 벽에 잠긴 문이 있다. 서쪽으로 통로가 난다."
           물어봐 move, "북쪽, 서쪽, 아니면 돌아갈까? "
           if move == "north":
               if "key" in inventory and "rope" in inventory:
                   보여줘 "열쇠가 자물쇠를 돌린다. 문이 열린다!"
                   보여줘 "그 너머: 금이 가득한 상자. 승리!"
                   보여줘 f"모은 것: {inventory}"
                   break
               else:
                   보여줘 "문은 잠겨 있다. 열쇠와 밧줄이 필요하다."
           elif move == "west":
               보여줘 "통로가 열려 작은 강의 방에 들어선다."
               if "rope" not in inventory:
                   보여줘 "물가에 밧줄이 감겨 있다. 집는다."
                   inventory.append("rope")
               else:
                   보여줘 "이미 밧줄을 갖고 있다."
               roll = 랜덤정수(1, 6)
               보여줘 f"개울을 건너려고 한다. 주사위: {roll}."
               if roll >= 4:
                   보여줘 "물을 건너며 동전을 하나 발견한다."
                   inventory.append("coin")
               else:
                   보여줘 "물이 너무 깊다. 마른 몸으로 돌아선다."
           else:
               보여줘 "입구 동굴로 돌아간다."
       elif action == "dice":
           roll = 랜덤정수(1, 6)
           보여줘 f"주사위가 굴러간다: {roll}."
           if roll >= 4:
               보여줘 "운이 좋다! 먼지 속에서 동전이 반짝인다. 집는다."
               inventory.append("coin")
           else:
               보여줘 "아무 일도 없다. 동굴은 조용하다."
       else:
           보여줘 "이해하지 못했다. look, east, dice, quit 중 하나를 입력해."
   ```

2. 실행합니다. 파이프 입력이 답을 하나씩 넣습니다: 동쪽으로 가고, 잠긴 문을
   시도하고, 다시 동쪽으로 가고, 밧줄을 집고, 주사위를 굴린 다음, 돌아와
   문을 엽니다:

   ```sh
   printf 'east\nnorth\neast\nwest\ndice\neast\nnorth\n' | nme 실행 adventure.ko
   ```

   ```text
   어두운 동굴에서 눈을 뜬다. 횃불 하나가 깜빡인다.
   무엇을 할까? (look, east, dice, quit) 터널을 따라 돌방에 들어선다.
   선반에 녹슨 열쇠가 놓여 있다. 집는다.
   북쪽 벽에 잠긴 문이 있다. 서쪽으로 통로가 난다.
   북쪽, 서쪽, 아니면 돌아갈까? 문은 잠겨 있다. 열쇠와 밧줄이 필요하다.
   무엇을 할까? (look, east, dice, quit) 터널을 따라 돌방에 들어선다.
   이제 선반은 비어 있다.
   북쪽 벽에 잠긴 문이 있다. 서쪽으로 통로가 난다.
   북쪽, 서쪽, 아니면 돌아갈까? 통로가 열려 작은 강의 방에 들어선다.
   물가에 밧줄이 감겨 있다. 집는다.
   개울을 건너려고 한다. 주사위: 4.
   물을 건너며 동전을 하나 발견한다.
   무엇을 할까? (look, east, dice, quit) 주사위가 굴러간다: 6.
   운이 좋다! 먼지 속에서 동전이 반짝인다. 집는다.
   무엇을 할까? (look, east, dice, quit) 터널을 따라 돌방에 들어선다.
   이제 선반은 비어 있다.
   북쪽 벽에 잠긴 문이 있다. 서쪽으로 통로가 난다.
   북쪽, 서쪽, 아니면 돌아갈까? 열쇠가 자물쇠를 돌린다. 문이 열린다!
   그 너머: 금이 가득한 상자. 승리!
   모은 것: ['key', 'rope', 'coin', 'coin']
   ```

   주사위 줄 두 개는 무작위입니다 — 실행마다 다른 숫자가 나옵니다.

3. 게임 세계는 [22](22-terminal-menu.ko.md) 가이드의 터미널 메뉴와 똑같이
   끝없는 반복 하나와 작은 메뉴로 이뤄집니다. `while True:`는 스스로 멈추지
   않으므로 방을 나가는 유일한 길은 `quit`에 답하거나 이기는 것입니다:

   ```text
   while True:
       물어봐 action, "무엇을 할까? (look, east, dice, quit) "
       만약 action == "quit":
           보여줘 "동굴을 떠난다. 안녕!"
           break
       elif action == "east":
           보여줘 "터널을 따라 돌방에 들어선다."
           물어봐 move, "북쪽, 서쪽, 아니면 돌아갈까? "
           if move == "north":
               보여줘 "문을 연다."
           elif move == "west":
               보여줘 "강의 방을 찾는다."
           else:
               보여줘 "입구 동굴로 돌아간다."
   ```

   `물어봐`마다 답이 저장되고 `만약` 사슬이 갈래 하나를 고릅니다. 안쪽
   `물어봐 move`가 돌방에 작은 메뉴를 만들어 줍니다.

4. 인벤토리는 목록입니다. `inventory.append("key")`가 물건을 더하고,
   `"key" not in inventory` 확인이 물건을 한 번만 집게 해 줍니다:

   ```text
   inventory = []
   if "key" not in inventory:
       inventory.append("key")
       보여줘 "열쇠를 집는다."
   ```

   `보여줘 f"모은 것: {inventory}"`는 승리 줄에서 목록 전체를 출력합니다.

5. 주사위가 위험한 방을 결정합니다. `랜덤 사용 최신`이 `랜덤정수(가, 나)`를
   불러오고, `가`와 `나` 사이를 굴리며, `if roll >= 4`가 성공과 실패를
   나눕니다:

   ```text
   랜덤 사용 최신
   roll = 랜덤정수(1, 6)
   보여줘 f"주사위가 굴러간다: {roll}."
   if roll >= 4:
       보여줘 "운이 좋다! 동전을 발견한다."
       inventory.append("coin")
   else:
       보여줘 "아무 일도 없다."
   ```

   굴림이 무작위라 두 게임이 같게 나오지 않습니다 — 주사위 마주침의
   의미입니다.

6. 승리 조건은 모은 물건 두 개를 합칩니다. `and`가 [09](09-and-or.ko.md)
   가이드의 확인을 이어 붙여, 열쇠와 밧줄이 모두 인벤토리에 있을 때만 문이
   열리고 `break`가 반복을 떠납니다:

   ```text
   while True:
       물어봐 move, "북쪽, 서쪽, 아니면 돌아갈까? "
       if move == "north":
           if "key" in inventory and "rope" in inventory:
               보여줘 "열쇠가 자물쇠를 돌린다. 문이 열린다!"
               보여줘 f"모은 것: {inventory}"
               break
           else:
               보여줘 "문은 잠겨 있다. 열쇠와 밧줄이 필요하다."
       else:
           break
   ```

   영어 쌍둥이 `adventure.nme`는 같은 게임을 `use random latest`,
   `ask`, `show`, `random_number(1, 6)`로 씁니다. 답 `east`/`north`/`west`는
   같으므로 같은 파이프 입력으로 두 언어 모두 이깁니다.

## 직접 해보기

네 번째 방향을 추가해 보세요: 강의 방에 `south` 선택지를 만들어 "메아리
동굴"로 이어지게 하고 그곳에 물건을 하나 두고, 승리 문이 그 물건도
요구하게 만드세요. 메뉴 줄에 선택지를 더하고 `elif action == "south":`
갈래를 새로 만드세요.

## 배운 것

- `물어봐` 메뉴가 달린 반복이 글을 작은 게임 세계로 만듭니다.
- 목록은 인벤토리입니다: `append`는 모으고, `in`/`not in`은 확인합니다.
- `랜덤 사용`은 실행마다 다른 주사위 마주침을 만듭니다.
- 승리 조건은 물건을 `and`로 합치고 `break`로 반복을 떠납니다.
