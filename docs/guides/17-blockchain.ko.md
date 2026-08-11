# 17 — 블록체인 — 해시로 연결된 장부

[English](17-blockchain.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★☆ (4/5)
- 선수 지식 (Prerequisites): [13 — 파일](13-files.ko.md), [03 — 저장](03-set.ko.md), [05 — 반복](05-repeat.ko.md)
- 주제 (Topic): 블록체인 / blockchain
- 결과물 (Result): 해시로 연결된 블록들의 이해 / an understanding of blocks linked by hashes

블록체인은 각 기록이 앞의 기록을 가리키는 목록입니다. 예제
`examples/blockchain-ledger.ko.nme`는 해시 함수로 연결된 블록 세 개를
만듭니다. 이 프로젝트는 학습용이며 투자 조언이 아닙니다.

```sh
nme run examples/blockchain-ledger.ko
```

```text
블록 55e0751f -> 이전 0
블록 24752448 -> 이전 55e0751f
블록 97b085ce -> 이전 24752448
연결된 블록이 3개입니다
```

해시는 실행마다 바뀝니다 — 데이터가 무작위이기 때문입니다 — 하지만 각
블록은 여전히 앞 블록을 가리킵니다.

## 단계

1. 해시는 어떤 글자든 고정된 길이의 지문으로 만듭니다. 프로그램은 Python의
   `hashlib`을 가져와 이전 해시와 새 데이터를 합쳐 해시합니다:

   ```text
   # examples/blockchain-ledger.ko.nme의 일부
   import hashlib
   previous_hash = "0"
   data = "메시지 1"
   combined = previous_hash + data
   block_hash = hashlib.sha256(combined.encode()).hexdigest()
   말해 f"블록 {block_hash[:8]} -> 이전 {previous_hash}"
   ```

   `sha256`은 같은 입력에 항상 같은 64글자를 돌려주고, 다른 입력이면 완전히
   다른 해시가 나옵니다.

2. 각 블록은 데이터, 이전 해시, 자기 해시를 저장합니다. `previous_hash`를
   저장하는 것이 바로 연결입니다 — 다음 블록의 `prev`가 이 해시가 됩니다:

   ```text
   # examples/blockchain-ledger.ko.nme의 일부
   import hashlib
   previous_hash = "0"
   data = "메시지 1"
   combined = previous_hash + data
   block_hash = hashlib.sha256(combined.encode()).hexdigest()
   block = {"data": data, "prev": previous_hash[:8], "hash": block_hash[:8]}
   chain = [block]
   말해 f"블록 {block['hash']} -> 이전 {block['prev']}"
   ```

   [05](05-repeat.ko.md) 가이드의 `3 times:` 반복이 한 차례에 블록 하나를
   만들어 `chain.append(block)`으로 모읍니다. 가운데 블록을 바꾸면 다음
   블록의 `prev`가 그 해시와 어긋나는데, 그 어긋남이 사슬을 유용하게
   만드는 변조의 증거입니다.

3. 영어 쌍둥이 `examples/blockchain-ledger.nme`는 `use random latest`,
   `show`, `"message "` 데이터로 같은 프로그램을 씁니다. 같은 방법으로
   실행하면 영어로 같은 모양이 나옵니다.

## 직접 해보기

예제를 내 폴더에 복사하고 `3 times:`를 `5번:`으로 바꿔 보세요 — 다섯
블록이 각자 앞 블록을 가리키는 모습을 볼 수 있습니다.

## 배운 것

- 블록체인은 블록들의 목록이며, 각 블록은 이전 블록의 해시로 연결됩니다.
- `hashlib.sha256(...)`는 글자를 고정된 64글자 지문으로 만듭니다.
- `chain.append(block)`은 블록을 추가하고 `len(chain)`은 개수를 셉니다.
- 각 블록이 이전 해시를 저장하므로 변조가 눈에 보입니다.
