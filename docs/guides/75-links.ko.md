# 75 — 웹: 페이지에서 링크 뽑아내기

[English](75-links.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [70 — 정규식](70-patterns.ko.md), [72 — 날씨](72-weather.ko.md)
- 주제: 웹/텍스트
- 결과물: 로컬 서버의 HTML 페이지를 받아 그 안의 링크를 모두 찾아 절대 주소로 보여 주기

[76](76-net.ko.md)가 JSON을 가져오고, [70](70-patterns.ko.md)가 정규식으로
텍스트를 찾았습니다. 웹 페이지도 그냥 텍스트입니다 — 내용을 태그로
감싼 HTML일 뿐입니다. 링크를 뽑는 것은 원시 페이지에 정규식 하나이고,
서버 주소를 이어 붙이면 브라우저가 쓰는 완전한 주소가 됩니다.

## 단계

1. 링크 세 개가 있는 작은 페이지를 만드세요. `page.html`로 저장:

   ```text
   <html>
   <head><title>my page</title></head>
   <body>
   <h1>Links</h1>
   <a href="story.txt">the story</a>
   <a href="notes.txt">notes</a>
   <a href="about.html">about us</a>
   </body>
   </html>
   ```

2. [76](76-net.ko.md)에서처럼 폴더를 서빙하세요:

   ```sh
   python3 -m http.server 8000
   ```

3. 페이지를 받는 것은 JSON을 받은 것과 같은 `urlopen` 호출입니다.
   페이지는 텍스트라서 `.decode("utf-8")`이 HTML을 줍니다:

   ```nme
   from urllib.request import urlopen
   import re

   base = "http://localhost:8000"
   html = urlopen(base + "/page.html").read().decode("utf-8")
   ```

4. 링크는 `href="`와 다음 `"` 사이에 있습니다. 정규식
   `r'href="([^"]+)"'`이 전부 찾습니다 — `[^"]+`는 "따옴표가 아닌 문자
   하나 이상"이고, `findall`은 [70](70-patterns.ko.md)에서처럼 잡힌 부분만
   돌려줍니다. `links.nme`로 저장하세요:

   ```nme
   # links.nme — 웹 페이지에서 링크 찾기.
   # 실행: nme 실행 links
   # 먼저 이 폴더에서 python3 -m http.server 8000 실행

   from urllib.request import urlopen
   import re

   base = "http://localhost:8000"
   html = urlopen(base + "/page.html").read().decode("utf-8")
   links = re.findall(r'href="([^"]+)"', html)

   show f"found {len(links)} links:"
   for link in links:
       show base + "/" + link
   ```

   페이지는 `story.txt` — 자기 자신 기준의 짧은 이름만 알고 있습니다.
   앞에 `base + "/"`를 붙이면 브라우저가 열 수 있는 완전한 주소가
   됩니다: 같은 서버의 페이지는 짧은 이름으로 링크하고, 클라이언트가
   채워 넣습니다.

5. 실행하세요:

   ```sh
   nme 실행 links
   ```

   ```text
   found 3 links:
   http://localhost:8000/story.txt
   http://localhost:8000/notes.txt
   http://localhost:8000/about.html
   ```

   세 개의 `href`가 완전한 주소 세 개가 되었습니다. 링크 검사기나 웹
   크롤러가 다음 페이지를 받기 전에 하는 것과 똑같은 추출 단계입니다.

## 직접 해보기

링크가 있는 다른 페이지로 가는 링크를 하나 추가하고, 프로그램이 찾은
각 링크를 받아 존재하는지 세어 보세요 (`404 Not Found` 응답은 링크가
깨졌다는 뜻 — HTTP 오류는 예외로 나타나고, [59](59-errors.ko.md)이
잡는 법을 보여 줍니다). 또는 같은 패턴으로 페이지의 `src="..."`
이미지 소스를 뽑아 보세요.

## 배운 것

- HTML은 텍스트입니다: `urlopen`으로 받고, 풀고, 정규식으로 찾습니다.
- `r'href="([^"]+)"'`가 링크 대상을 찾고, `[^"]+`가 닫는 따옴표에서
  멈춥니다.
- 상대 링크는 짧은 이름이고, 클라이언트가 서버 주소를 앞에 붙입니다.
- 링크 추출은 정규식 하나 — 링크 검사기의 첫 단계입니다.
