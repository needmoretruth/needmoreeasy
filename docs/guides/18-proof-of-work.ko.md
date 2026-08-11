# 18 — 작업 증명 — 채굴

[English](18-proof-of-work.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [17 — 블록체인](17-blockchain.ko.md), [07 — 동안](07-while.ko.md)
- 주제 (Topic): 채굴 / mining
- 결과물 (Result): nonce를 찾아 블록을 채굴하고 사슬을 검증하기 / mine a block by finding a nonce and verify the chain

[17](17-blockchain.ko.md) 가이드가 해시 사슬을 만들었지만 아무도 일하지
않도록 강제하지는 못했습니다. 작업 증명은 퍼즐을 더합니다: 블록의 해시가
0으로 시작할 때만 인정되므로, 채굴자는 nonce를 많이 시도해야 합니다.
프로그램은 `examples/proof-of-work.ko.nme`이며 학습용이고
투자 조언이 아닙니다.

```sh
nme run examples/proof-of-work.ko
```

```text
채굴됨 00c2d1d1c8e357 nonce 375
채굴됨 001d8f68bd9102 nonce 359
채굴됨 0058476d84c676 nonce 58
모든 해시는 00로 시작합니다
사슬 검증: True
첫 블록을 바꾼 뒤: False
```

해시와 nonce는 실행마다 다르지만, 채굴된 세 해시는 항상 `00`으로 시작하고
사슬은 검증을 통과하며 변조는 실패합니다.

## 단계

1. 난이도는 해시가 가져야 하는 앞쪽 0의 개수를 정합니다. 난이도 2에서
   `target`은 `"00"`입니다 — 난이도가 높을수록 퍼즐이 어려워집니다:

   ```text
   # examples/proof-of-work.ko.nme의 일부
   import hashlib
   difficulty = 2
   target = "0" * difficulty
   ```

2. `mine` 함수는 `previous_hash + data + nonce`의 해시가 target으로 시작할
   때까지 `nonce = 0, 1, 2, ...`를 시도합니다. [07](07-while.ko.md)
   가이드의 `while True:` 반복이 계속 시도하고, 조건에 맞으면 `return`이
   나옵니다:

   ```text
   # examples/proof-of-work.ko.nme의 일부
   import hashlib
   difficulty = 2
   target = "0" * difficulty

   def mine(data, previous_hash):
       nonce = 0
       while True:
           text = previous_hash + data + str(nonce)
           block_hash = hashlib.sha256(text.encode()).hexdigest()
           if block_hash.startswith(target):
               return {"data": data, "nonce": nonce, "hash": block_hash}
           nonce = nonce + 1
   ```

   nonce는 채굴자가 맞히려는 숫자입니다. 하나씩 시도하는 것보다 빠른 방법이
   없기 때문에 이 일은 진짜 작업입니다.

3. 사슬은 모든 블록이 정직하게 채굴됐을 때만 유효합니다. `is_valid`는
   처음부터 모든 블록을 다시 해시해 비교합니다. 실행은 첫 블록의 데이터를
   바꾸고 다시 확인합니다 — `False`가 나오므로 변조가 드러납니다.

4. 영어 쌍둥이 `examples/proof-of-work.nme`는 `"block "` 데이터를 채굴하고
   `show`로 출력합니다 — 실행하면 같은 여섯 줄을 영어로 볼 수 있습니다.

## 직접 해보기

예제를 내 폴더에 복사하고 `difficulty = 2`를 `difficulty = 3`으로 올려
보세요. 0이 하나 늘 때마다 퍼즐이 약 16배 어려워져서 nonce가 커집니다.

## 배운 것

- 난이도는 블록 해시가 시작해야 하는 앞쪽 0의 개수를 정합니다.
- 채굴은 `sha256(previous + data + nonce)`가 target에 맞을 때까지 nonce를
  시도하는 일입니다.
- `is_valid`는 사슬 전체를 다시 해시하므로 바뀐 블록이 드러납니다.
- nonce를 찾는 지름길은 없습니다 — 그래서 이 작업은 진짜입니다.
