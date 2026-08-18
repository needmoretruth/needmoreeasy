# 74 — 네트워크: 파일 받아 저장하기

[English](74-download.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [72 — 날씨](72-weather.ko.md), [37 — 파일](37-files.ko.md)
- 주제: 네트워크/파일
- 결과물: 로컬 HTTP 서버에서 파일을 받아 진행 상황을 보여 주며 저장하기

[76](76-net.ko.md)는 데이터를 가져와 보여 주고, [73](73-poll.ko.md)은 같은
것을 반복해서 가져왔습니다. 이번 가이드는 파일을 한 번 받아서
**저장**합니다 — 다운로더의 핵심입니다. 웹 페이지를 읽는 것과 파일을
받는 것은 같은 호출이며, 새로 나오는 개념은 조금씩 읽어 나가는 것과
디스크에 쓰는 것 두 가지뿐입니다.

## 단계

1. 받을 파일을 만드세요. `story.txt`는 실제 웹사이트에서 가져온다고
   상상하는 파일입니다:

   ```text
   옛날옛날에, 한 학습자가 터미널을 열었습니다.
   터미널은 한 번에 작은 프로그램 하나씩만 물었습니다.
   어느 날 학습자는 다운로더를 만들었고,
   파일은 무사히 디스크에 도착했습니다.
   ```

2. [76](76-net.ko.md)에서처럼 폴더를 Python 내장 HTTP 서버로 띄우세요:

   ```sh
   python3 -m http.server 8000
   ```

3. `urlopen`은 응답 객체를 줍니다. `.read()`가 본문 전체를 한 번에
   돌려줍니다:

   ```nme
   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   data = urlopen(url).read()
   text = data.decode("utf-8")
   ```

   응답은 원시 바이트라서 `.decode("utf-8")`로 텍스트로 바꿔야 합니다.
   [72](72-weather.ko.md)에서 JSON을 풀 때 쓴 것과 같은 호출입니다.

4. 저장은 `file_write` 한 번입니다 — [37](37-files.ko.md). 전체 프로그램이
   다운로드하고 저장합니다. `download.nme`로 저장하세요:

   ```nme
   # download.nme — 서버에서 파일을 받아 저장하기.
   # 실행: nme 실행 download
   # 먼저 이 폴더에서 python3 -m http.server 8000 실행

   use file latest

   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   text = urlopen(url).read().decode("utf-8")
   show f"downloaded {len(text)} characters"
   file_write("story-copy.txt", text)
   show "saved as story-copy.txt"
   ```

5. 서버가 켜져 있는 동안 실행하세요:

   ```sh
   nme 실행 download
   ```

   ```text
   downloaded 99 characters
   saved as story-copy.txt
   ```

   `story-copy.txt`를 열어 보세요. 원본과 똑같은 복사본이고, 여러분의
   프로그램이 받아 쓴 것입니다 — 첫 번째 다운로드 파일입니다.

6. 큰 파일을 받을 때 조용히 기다리는 것은 좋지 않습니다. 응답은 조금씩
   읽을 수 있고, 서버가 보내는 헤더 `Content-Length`가 전체 크기를
   알려줍니다. 조각은 마지막까지 **원시 바이트**로 두세요. 64바이트 조각은
   한글 한 글자를 중간에서 자를 수 있고, 반으로 잘린 문자를 풀면
   실패합니다. 바이트를 통째로 모은 뒤 한 번만 풉니다.
   `download-progress.nme`로 저장하세요:

   ```nme
   # download-progress.nme — 조금씩 받으며 진행 상황 보여 주기.
   # 실행: nme 실행 download-progress
   # 먼저 이 폴더에서 python3 -m http.server 8000 실행

   use file latest

   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   response = urlopen(url)
   size = int(response.headers["Content-Length"])
   show f"size: {size} bytes"

   chunks = []
   received = 0
   while True:
       chunk = response.read(64)
       if not chunk:
           break
       received = received + len(chunk)
       chunks.append(chunk)
       show f"received {received} / {size}"

   text = b"".join(chunks).decode("utf-8")
   file_write("story-copy.txt", text)
   show "saved"
   ```

   64바이트씩 읽으므로 데이터가 도착할 때마다 진행 줄이 갱신되고, 빈
   조각이 오면 `break`로 멈춥니다. `b"".join(chunks)`가 바이트 조각을
   이어 붙이고, `.decode("utf-8")`은 완전한 파일에 대해 한 번만
   실행됩니다. 이 루프는 [73](73-poll.ko.md)에서 서버를 폴링한 것과 같은
   패턴인데, 여기서는 서버가 끝날 때까지 보냅니다.

7. 실행하세요:

   ```sh
   nme 실행 download-progress
   ```

   ```text
   size: 245 bytes
   received 64 / 245
   received 128 / 245
   received 192 / 245
   received 245 / 245
   saved
   ```

## 직접 해보기

`story.txt` 옆에 `content.txt`와 `notes.txt`를 만들고, 프로그램이 폴더에
있는 파일을 전부 받도록 바꿔 보세요 — [48](48-files-folder.ko.md)가 폴더를
나열하는 방법을 보여 주고, 파일 이름마다 `urlopen` 호출 하나가 됩니다.
또는 [73](73-poll.ko.md)의 `status.json`처럼 변하는 페이지를 받아서
폴링할 때마다 스냅샷을 저장해 보세요.

## 배운 것

- 다운로드는 `urlopen` 호출 하나와 `file_write` 호출 하나입니다.
- 응답 본문은 바이트이고, `.decode("utf-8")`가 텍스트로 바꿉니다.
- `Content-Length`는 데이터가 오기 전에 크기를 알려 줍니다.
- 조금씩 읽으면 프로그램이 작업 중에도 진행 상황을 알려 줄 수 있습니다.
- 조각 하나가 글자를 반으로 자를 수 있으므로, 바이트를 모두 모은 뒤에만
  풀어야 합니다.
- 서버를 폴링한 루프-`break` 패턴이 파일을 받는 데도 그대로 쓰입니다.
