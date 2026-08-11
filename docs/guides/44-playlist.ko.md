# 44 — 플레이리스트: 랜덤 음악 재생

[English](44-playlist.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [10 — 랜덤](10-random.ko.md), [14 — JSON](14-json.ko.md)
- 주제 (Topic): 게임/랜덤 / game & random
- 결과물 (Result): JSON에서 불러온 플레이리스트에 셔플·다음 곡·반복 재생을 더한 프로그램 / a playlist loaded from JSON with shuffle, next, and a loop of songs

음악 플레이어는 노래 목록을 넘겨 가며 움직입니다 — JSON으로 불러와 랜덤 도우미로 섞고, 끝낼 때까지 반복하는 메뉴를 돌립니다.

## 단계

1. `songs.json`에 노래 다섯 곡을 저장합니다 — `title`과 `artist`를 가진
   딕셔너리들의 JSON 목록입니다:

   ```text
   [
     {"title": "Hello", "artist": "Adele"},
     {"title": "Dynamite", "artist": "BTS"},
     {"title": "Love Dive", "artist": "IVE"},
     {"title": "Life Goes On", "artist": "BTS"},
     {"title": "Butter", "artist": "BTS"}
   ]
   ```

2. 목록을 `json읽기`로 불러오고 `섞기`로 섞으며, `랜덤선택`으로 곡을
   뛰어넘습니다. 전체 플레이어를 `playlist.ko.nme`으로 저장합니다:

   ```text
   # playlist.ko.nme — 랜덤 음악 플레이어.
   # 실행: nme r playlist.ko

   랜덤 사용 최신
   파일 사용 최신

   노래들 = json읽기("songs.json")
   섞기(노래들)

   현재 = 0

   말해 f"플레이리스트 로드: {len(노래들)}곡"

   while True:
       말해 ""
       말해 "명령: next, prev, list, quit"
       물어봐 명령, "? "
       if 명령 == "next":
           현재 = 현재 + 1
           if 현재 >= len(노래들):
               현재 = 0
           노래 = 노래들[현재]
           말해 f"재생 중: {노래['title']} - {노래['artist']}"
       elif 명령 == "prev":
           현재 = 현재 - 1
           if 현재 < 0:
               현재 = len(노래들) - 1
           노래 = 노래들[현재]
           말해 f"재생 중: {노래['title']} - {노래['artist']}"
       elif 명령 == "list":
           말해 f"플레이리스트 ({len(노래들)}곡):"
           for i in range(len(노래들)):
               mark = "> " if i == 현재 else "  "
               말해 f"{mark}{i + 1}. {노래들[i]['title']} - {노래들[i]['artist']}"
       elif 명령 == "quit":
           말해 "안녕!"
           break
   ```

3. 파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'next\nnext\nlist\nquit\n' | nme r playlist.ko
   ```

   ```text
   플레이리스트 로드: 5곡

   명령: next, prev, list, quit
   ? 재생 중: Butter - BTS

   명령: next, prev, list, quit
   ? 재생 중: Life Goes On - BTS

   명령: next, prev, list, quit
   ? 플레이리스트 (5곡):
     1. Love Dive - IVE
     2. Butter - BTS
   > 3. Life Goes On - BTS
     4. Dynamite - BTS
     5. Hello - Adele

   명령: next, prev, list, quit
   ? 안녕!
   ```

   `next`는 끝을 지나면 0으로, `prev`는 마지막 곡으로 되돌아갑니다. `섞기`가 순서를 섞어 실행마다 다르게 보여 줍니다.

## 직접 해보기

가수를 물어보고 그 가수의 곡이 몇 곡인지 알려 주는 `count` 명령을 더해 보세요 — `노래들`을 돌며 카운터를 늘립니다.

## 배운 것

- `랜덤 사용 최신`과 `파일 사용 최신`을 함께 쓰면 두 도우미가 모두 로드됩니다.
- `json읽기("songs.json")`이 노래 목록을 읽고 `섞기(노래들)`이 섞으며 `랜덤선택(노래들)`이 하나를 고릅니다.
- `노래들[현재]`가 곡 하나를 읽고, 인덱스를 되돌리면 플레이리스트가 반복됩니다.
- `while True` 메뉴와 `물어봐`·`말해`·`break`가 플레이어를 움직입니다.
