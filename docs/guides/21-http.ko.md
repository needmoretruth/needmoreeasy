# 21 — HTTP — 웹 서버에 물어보기

[English](21-http.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [20 — 합의](20-consensus.ko.md), [13 — 파일](13-files.ko.md)
- 주제 (Topic): 네트워크/HTTP
- 결과물 (Result): 로컬 서버에서 페이지를 가져오기 / fetching a page from a local server

HTTP는 프로그램이 웹 서버에게 페이지를 묻는 방법입니다. 예제
`examples/http-client.ko.nme`는 `http://localhost:8000`으로 자기 컴퓨터에게
파일을 요청합니다. 이 프로젝트는 학습용이며 조언이 아닙니다.

먼저 폴더 하나를 서비스하고, 두 번째 터미널에서 예제를 실행하세요:

```sh
python3 -m http.server 8000
```

```sh
nme 실행 examples/http-client.ko
```

```text
서버 응답: 안녕하세요, HTTP에서 왔어요!
```

## 단계

1. `urllib`은 Python 표준 라이브러리라 설치가 필요 없습니다. 프로그램은
   서버와 대화하는 부분을 가져오고 URL을 정합니다:

   ```text
   # examples/http-client.ko.nme의 일부
   import urllib.request

   url = "http://localhost:8000/hello.txt"
   ```

   `localhost`는 "이 컴퓨터"라는 뜻이고, `8000`은 첫 터미널의 서버가
   듣는 포트입니다.

2. `urlopen`은 그 URL에 연결을 열고 응답을 돌려줍니다. 응답은 아직
   글이 아니라 바이트이므로 읽고 디코딩해야 합니다:

   ```text
   # examples/http-client.ko.nme의 일부
   with urllib.request.urlopen(url) as response:
       body = response.read().decode("utf-8")
   ```

   `with` 블록은 끝날 때 연결을 닫고, `.decode("utf-8")`이 바이트를
   문자열로 바꿉니다.

3. 마지막 줄이 답을 보여 줍니다. `body.strip()`은 서버가 파일과 함께 보낸
   마지막 새 줄을 없앱니다:

   ```text
   # examples/http-client.ko.nme의 일부
   말해 "서버 응답: " + body.strip()
   ```

4. `nme 검사`는 서버 없이도 프로그램을 확인합니다 — 확인은 문법만
   필요하고, 실행만 서버가 필요합니다:

   ```sh
   nme 검사 examples/http-client.ko
   ```

5. 영어 쌍둥이 `examples/http-client.nme`는 같은 프로그램을 씁니다. 마지막
   `show`만 `show "the server answered: " + body.strip()`로 바뀝니다. 같은
   폴더에서 `nme r examples/http-client`로 실행하면 영어로 같은 결과가
   나옵니다.

## 직접 해보기

서비스하는 폴더에 두 번째 파일 `hello2.txt`를 만들고 `url`을 그 파일로
바꿔 보세요. 다시 시작할 필요는 없습니다 — 서버는 파일을 요청할 때마다
읽습니다 — 그리고 예제를 다시 실행해 새 파일을 확인하세요.

## 배운 것

- `python3 -m http.server 8000`은 현재 폴더를 `localhost:8000`에서 서비스합니다.
- `urllib.request.urlopen(url)`은 연결을 열고 응답을 돌려줍니다.
- 응답은 `.read()`로 읽고 `.decode("utf-8")`로 디코딩해야 합니다.
- `body.strip()`은 서버가 보낸 새 줄을 없앱니다.
