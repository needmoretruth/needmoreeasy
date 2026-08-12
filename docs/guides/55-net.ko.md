# 55 — 네트워크: 미니 채팅

[English](55-net.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [32 — Weather](32-weather.ko.md), [44 — Playlist](44-playlist.ko.md)
- 주제 (Topic): 네트워크 / network
- 결과물 (Result): 로컬 HTTP 서버에서 메시지를 받아 미니 채팅처럼 보여 주기 / fetching messages from a local HTTP server and showing them like a mini chat

채팅 클라이언트는 서버에서 메시지를 읽고 새 메시지를 보냅니다. 이 가이드는
로컬 서버에서 JSON 메시지 목록을 받아 채팅 기록처럼 출력하고, 새 답장이
어떻게 생겼는지 보여 줍니다. 일반 정적 서버로 메시지를 제공할 수 있고,
답장 단계에서는 보내려면 서버가 받아줘야 한다는 점을 설명합니다.

## 단계

1. 짧은 채팅 기록을 담은 `messages.json`을 만드세요:

   ```text
   [
     {"who": "Mina", "text": "hello"},
     {"who": "Jun", "text": "hi Mina"},
     {"who": "Mina", "text": "ready to code?"}
   ]
   ```

2. 가이드 [32](32-weather.ko.md)처럼 Python 내장 HTTP 서버로 폴더를
   서비스하세요:

   ```sh
   python3 -m http.server 8000
   ```

3. 프로그램이 목록을 받아 각 메시지를 출력합니다:

   ```text
   from urllib.request import urlopen
   from json import loads

   url = "http://localhost:8000/messages.json"
   messages = loads(urlopen(url).read().decode("utf-8"))

   for message in messages:
       show f"{message['who']}: {message['text']}"
   ```

   각 dict는 `who`와 `text`를 담고, `message["who"]`가 한 필드를 읽습니다.
   페이지를 읽는 것은 GET이라 어떤 웹 서버든 응답할 수 있습니다.

4. 답장도 같은 모양입니다: 목록에 추가될 dict. 전체 프로그램은 답장을
   만들고 보낼 내용을 그대로 출력합니다:

   ```text
   # chat.nme — 채팅 기록을 받아, 보낼 답장을 보여 줍니다.
   # 실행: nme r chat.ko
   # 먼저 이 폴더를 서비스하세요: python3 -m http.server 8000

   from urllib.request import urlopen
   from json import loads, dumps

   url = "http://localhost:8000/messages.json"
   messages = loads(urlopen(url).read().decode("utf-8"))

   show "chat log:"
   for message in messages:
       show f"  {message['who']}: {message['text']}"

   reply = {"who": "you", "text": "nice to meet you"}
   payload = dumps(reply)
   show ""
   show "your reply would be POSTed as:"
   show payload
   ```

   서버가 켜진 상태에서 실행하세요:

   ```sh
   nme r chat.ko
   ```

   ```text
   chat log:
     Mina: hello
     Jun: hi Mina
     Mina: ready to code?
   your reply would be POSTed as:
   {"who": "you", "text": "nice to meet you"}
   ```

5. 답장을 보내려면 `urllib.request.Request(url, data=..., method="POST")`가
   필요합니다. 일반 `http.server`는 POST를 받지 않으므로 이 가이드는 보낸
   척하지 않고 payload를 출력합니다. 진짜 채팅 서버는 POST를 받아 메시지를
   추가할 것입니다 — 여기 데이터 형식이 바로 그 모양입니다.

## 직접 해보기

`messages.json`에 메시지 하나를 더 추가하고 채팅을 다시 실행하세요 — 기록이
늘어납니다. 답장의 `text`를 바꾸고 payload가 바뀌는 것을 확인하세요.

## 배운 것

- `urlopen(url).read().decode("utf-8")`이 페이지를 텍스트로 받습니다.
- `loads(...)`가 JSON 텍스트를 Python dict 목록으로 바꿉니다.
- `message["who"]`가 dict 한 개의 필드를 읽습니다.
- 답장은 dict이며, POST로 보내려면 받아 줄 서버가 필요합니다.
