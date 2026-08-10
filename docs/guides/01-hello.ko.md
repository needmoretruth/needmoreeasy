# 01 — 인사: 첫 문장 말하기

[English](01-hello.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★☆☆☆☆ (1/5)
- 선수 지식 (Prerequisites): 없음
- 주제 (Topic): 첫 프로그램과 출력 / first program and output
- 결과물 (Result): `nme run`으로 메시지를 출력하는 프로그램 / a program that prints a message with `nme run`

첫 프로그램입니다. NME는 `말해줘` 뒤에 오는 내용을 그대로 출력합니다.
따옴표도 괄호도 필요 없습니다.

## 단계

1. 빈 폴더에 `hello.nme`라는 파일을 만들고 다음과 같이 적습니다:

   ```text
   안녕하세요! 말해줘
   ```

2. 실행합니다:

   ```sh
   nme run hello
   ```

   콘솔에 `안녕하세요!`가 출력됩니다. `말해줘`가 동작이고, 나머지가
   메시지입니다.

3. 영어도 같은 방식입니다. 파일 내용을 다음과 같이 바꿔 봅니다:

   ```text
   show Hello world!
   ```

   `show`가 영어 동작이고, 한국어와 영어를 한 파일에 섞어 써도 됩니다.

4. 자연스러운 한 문장이면 동작어 없이도 출력됩니다:

   ```text
   오늘도 반가워요!
   Hello everyone!
   ```

## 직접 해보기

메시지를 여러분의 이름이나 좋아하는 곳으로 바꾸고 저장한 뒤 다시
`nme r hello`를 실행해 보세요. `nme r`은 `nme run`의 짧은 명령입니다.

## 배운 것

- `nme run hello`는 `hello.nme`를 실행하고, `nme r hello`가 짧은 명령입니다.
- `말해줘 메시지`는 줄의 나머지를 출력하고, `show`가 영어 동작입니다.
- 문장에는 따옴표, 쉼표, 괄호가 필요 없습니다.
- 자연스러운 한 줄 문장은 그대로 출력됩니다.
