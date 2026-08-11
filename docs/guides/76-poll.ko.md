# 76 — 네트워크: 서버 폴링

[English](76-poll.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [55 — Network](55-net.md), [70 — Reaction](70-reaction.md)
- 주제 (Topic): 네트워크/시간 / network & timing
- 결과물 (Result): 로컬 서버의 status.json을 몇 초마다 다시 받아 변화를 보고하기 / repeatedly fetching a status.json from a local server every few seconds and reporting changes

서버를 지켜보는 프로그램은 최신 상태를 계속 물어봅니다. 이것을 폴링이라고
합니다. 루프는 간단합니다: 받고, 보여 주고, 기다리고, 반복 — 서버가 끝났다고
말할 때 멈춥니다.

## 단계

1. 작업자가 바꿀 상태를 담은 `status.json`을 만드세요:

   ```text
   {"status": "working", "step": 1}
   ```

2. 가이드 [55](55-net.md)처럼 폴더를 서비스하세요:

   ```sh
   python3 -m http.server 8000
   ```

3. 상태를 받는 것은 가이드 [55](55-net.md)와 같은 `urlopen` 호출입니다:

   ```text
   from urllib.request import urlopen
   from json import loads

   url = "http://localhost:8000/status.json"
   status = loads(urlopen(url).read().decode("utf-8"))
   ```

4. 전체 프로그램은 루프로 폴링하며 각 상태를 보고하고, 서버가 `"done"`을
   알리면 멈춥니다. `poll.nme`로 저장하세요:

   ```text
   # poll.nme — status.json이 done이 될 때까지 지켜보기.
   # 실행: nme r poll.ko
   # 먼저 이 폴더를 서비스하세요: python3 -m http.server 8000
   # 실행 중에 status.json을 {"status": "done", "step": 2}로 바꾸세요.

   from urllib.request import urlopen
   from json import loads
   from time import sleep

   url = "http://localhost:8000/status.json"

   while True:
       status = loads(urlopen(url).read().decode("utf-8"))
       show f"step {status['step']}: {status['status']}"
       if status["status"] == "done":
           show "worker finished"
           break
       sleep(1)
   ```

   매 루프가 현재 파일을 받아 출력하고 1초 기다립니다. 변화를 보려면
   실행 중에 `status.json`을 편집하세요(HTTP 서버는 매 요청마다 파일을
   다시 읽습니다). 프로그램이 새 step을 보고한 뒤 멈춥니다.

5. 실행 중에 `status.json`을 편집하며 실행하세요:

   ```sh
   nme r poll.ko
   ```

   ```text
   step 1: working
   step 2: done
   worker finished
   ```

## 직접 해보기

`status.json`에 `started` 타임스탬프를 추가하고 작업자가 몇 초 동안
돌았는지 보고하거나, 두 엔드포인트를 폴링해 둘 중 하나가 끝나면
보고하세요.

## 배운 것

- 폴링은 시도 사이에 기다림을 두고 최신 상태를 루프로 받는 것입니다.
- HTTP 서버는 매 요청마다 파일을 다시 읽으므로 편집이 실시간으로 보입니다.
- `"done"` 같은 종료 값이 루프를 멈추게 합니다.
- `time.sleep`이 얼마나 자주 물어볼지 정합니다.
