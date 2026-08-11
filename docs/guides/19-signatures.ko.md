# 19 — 서명 — 신원 증명

[English](19-signatures.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [17 — 블록체인](17-blockchain.ko.md)
- 주제 (Topic): 서명 / signatures
- 결과물 (Result): hmac으로 메시지 서명하고 검증하기 / sign and verify a message with hmac

해시는 메시지가 바뀌지 않았음을 증명합니다. 서명은 누가 썼는지를
증명합니다. 프로그램 `examples/signatures.ko.nme`는 Python 표준 라이브러리
`hmac`을 써서 공유 비밀로 메시지를 서명하고 검증합니다. 실제 코인은
cryptography 라이브러리의 공개키 서명을 사용합니다. 이 프로젝트는 학습용이며
투자 조언이 아닙니다.

```sh
nme run examples/signatures.ko
```

```text
메시지: 민수에게 10 지불
서명: c67631d90b5381a8...
올바른 서명 검증: True
바뀐 메시지 검증: False
틀린 키로는 위조 불가: False
```

## 단계

1. 서명하려면 보내는 사람과 받는 사람만 아는 비밀이 필요합니다. `sign`
   함수는 그 비밀을 메시지 해시에 섞어 넣습니다:

   ```text
   # examples/signatures.ko.nme의 일부
   import hmac
   import hashlib
   secret = "공유-비밀-키"
   message = "민수에게 10 지불"

   def sign(text):
       return hmac.new(secret.encode(), text.encode(), hashlib.sha256).hexdigest()

   signature = sign(message)
   말해 "메시지: " + message
   말해 "서명: " + signature[:16] + "..."
   ```

   `hmac`은 [17](17-blockchain.ko.md) 가이드의 같은 `sha256` 해시를 쓰지만
   비밀이 섞이므로 비밀 없이는 서명을 위조할 수 없습니다.

2. 검증할 때 받는 사람은 같은 비밀로 메시지를 다시 서명해 비교합니다.
   `compare_digest`는 시간 정보가 새지 않게 비교합니다:

   ```text
   # examples/signatures.ko.nme의 일부
   signature = sign(message)
   말해 "올바른 서명 검증: " + str(hmac.compare_digest(sign(message), signature))
   tampered = "민수에게 100 지불"
   말해 "바뀐 메시지 검증: " + str(hmac.compare_digest(sign(tampered), signature))
   ```

   같은 메시지는 `True`로 검증되고, 메시지의 숫자를 한 글자만 바꿔도
   `False`가 됩니다. 예제의 마지막 줄은 다른 비밀로 서명해 비교합니다 —
   틀린 키로도 서명을 위조할 수 없습니다.

3. 영어 쌍둥이 `examples/signatures.nme`는 `"pay Mina 10"`을 서명하고
   `show`로 출력합니다 — 실행하면 같은 다섯 줄을 영어로 볼 수 있습니다.

## 직접 해보기

예제를 내 폴더에 복사하고 `secret`을 나만의 키로 바꿔 보세요. 다른 비밀은
다른 서명을 만들고, 옛 비밀로 서명한 메시지는 검증을 통과하지 못합니다.

## 배운 것

- 서명은 메시지를 쓴 사람을 증명합니다. 해시만으로는 바뀌지 않았음을 증명할
  뿐입니다.
- `hmac.new(키, 메시지, hashlib.sha256)`은 공유 비밀로 서명합니다.
- `hmac.compare_digest(...)`는 서명을 안전하게 검증합니다.
- 틀린 키나 바뀐 메시지는 모두 검증에 실패합니다.
