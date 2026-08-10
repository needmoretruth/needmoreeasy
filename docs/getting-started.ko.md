# NME 5분 시작

[English](getting-started.md) | 한국어

아직 `nme --version`이 동작하지 않으면 먼저
[Windows, macOS, Linux 설치 안내](install.ko.md)를 따라 하세요.

## 1. Hello World

UTF-8 파일 `hello.nme`를 만듭니다.

```text
안녕하세요! 말해줘
```

실행합니다.

```sh
nme 실행 hello.nme
```

완전한 프로그램을 만들었습니다. 따옴표와 괄호가 하나도 없습니다.

## 2. 프로그램과 대화하기

파일 내용을 바꿉니다.

```text
이름을 물어봐 이름이 뭐예요?
안녕하세요 이름! 말해줘
```

NME는 `이름`에 대답이 들어 있다는 것을 기억하므로 두 번째 문장의 `이름`
자리에 실제 값을 자동으로 넣습니다.

바로 영어와 섞어도 됩니다.

```text
ask 이름 What is your name?
show Hello 이름!
```

## 3. 콜론 없이 반복하기

문장 하나를 반복합니다.

```text
3번 반복해서 NME에 오신 것을 환영합니다 말해줘
```

여러 문장은 다음 줄을 스페이스 네 칸으로 들여씁니다.

```text
3번 반복해
    첫 번째 문장 말해줘
    show second sentence
```

## 4. 숫자 게임 만들기

```text
정답은 1부터 10까지 랜덤정수
추측을 숫자로 물어봐 1부터 10까지 숫자를 맞혀 보세요

만약에 추측이 정답과 같으면
    정답입니다! 말해줘

만약에 추측이 정답보다 작으면
    더 큰 수예요 말해줘

만약에 추측이 정답보다 크면
    더 작은 수예요 말해줘
```

저장소의 완성된 예제를 실행합니다.

```sh
nme 실행 examples/guessing-game.ko.nme
```

랜덤 숫자, 숫자 입력, 비교, 출력이 평범한 Python으로 컴파일됩니다. 목록,
함수 호출, 등호, 콜론을 알 필요가 없었습니다.

## 5. 정확한 문법과 고급 문법으로 자라기

문장형은 가장 쉬운 시작입니다. 초급 문법은 짧고 정확합니다.

```text
물어봐 이름, "이름이 뭐예요? "
만약 이름:
    말해 f"안녕하세요, {이름}!"
```

고급 문법은 Python과 같습니다.

```python
for 숫자 in range(1, 4):
    print(숫자**2)
```

세 단계를 필요에 따라 섞으세요.

```text
숫자들 = [1, 2, 3]

for 숫자 in 숫자들:
    show 숫자

2 times: 말해 "완료"
```

## 6. 검사, 빌드, 컴파일

```sh
nme 검사 hello.nme
nme 빌드 hello.nme -o hello.py
python3 hello.py
```

선택적으로 독립 실행 파일을 만듭니다.

```sh
python3 -m pip install nuitka
nme 컴파일 hello.nme -o hello
```

운영체제의 Python 명령이 `python`이나 `py`라면 `python3` 대신 사용하고,
NME에는 `--python`으로 전달하세요.

## 7. Python을 NME로 쉽게 바꾸기

```sh
nme 변환 old_program.py --level 문장형 --language 한국어 -o easier.nme
nme 변환 old_program.py --level 초급 --language 영어 -o easier.en.nme
```

## 다음에 볼 문서

- [학습 과정](tutorial.ko.md): Hello World부터 컴파일러까지 다섯 프로젝트
- [문법 레퍼런스](language.ko.md): 세 단계의 정확한 규칙
- [편집기](editors.ko.md): VS Code, Cursor, Zed
- [AI 코딩 도우미](ai-assistants.ko.md): 도우미에게 문서 링크 하나만 주기
