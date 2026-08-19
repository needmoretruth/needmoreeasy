# 65 — 변환: Python을 NME로

[English](65-convert.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [16 — 확인과 빌드](16-check-build.ko.md), [64 — Python 패키지](64-python-packages.ko.md)
- 주제: 도구 다루기
- 결과물: 작은 Python 파일을 NME로 변환하기

`nme 변환`은 반대 방향입니다. 안전한 Python 패턴을 선택한 단계와 언어로
다시 써 줍니다. 안전하게 바꿀 수 없는 줄은 Python 그대로 둡니다.

## 단계

1. 익숙한 몇 줄로 `old.py`를 만듭니다:

   ```python
   print("hi")
   name = input("Name: ")
   if name:
       print("hi", name)
   ```

2. 한국어 초급 문법으로 변환합니다:

   ```sh
   nme 변환 old.py --level beginner --language ko -o easy.ko.nme
   ```

   `if`는 Python으로 남고, 바꿀 수 있는 부분만 다시 써집니다. 변환기는
   항상 따옴표를 그대로 둡니다:

   ```nme
   말해 "hi"
   물어봐 name, "Name: "
   if name:
       print("hi", name)
   ```

3. 영어 명령은 `convert`이고, 문장형으로도 바꿀 수 있습니다:

   ```sh
   nme 변환 old.py --level sentence --language en -o easy.nme
   ```

   결과:

   ```nme
   show "hi"
   ask name "Name: "
   if name:
       print("hi", name)
   ```

4. 변환된 프로그램을 확인하고 실행합니다:

   ```sh
   nme 검사 easy.ko
   nme 실행 easy.ko
   ```

   변환기는 절대 추측하지 않습니다. 뜻이 바뀔 수 있는 것은 일반 Python으로
   남습니다.

## 직접 해보기

`old.py`에 `for i in range(3): print(i)` 줄을 추가하고 `--level beginner`로
다시 변환해 보세요. 반복은 `3 times: say i`가 됩니다 — 결과를 읽은 뒤에
말입니다.

## 배운 것

- `nme 변환`은 안전한 `print`, `input`, 간단한 패턴을 다시 써 줍니다.
- `--level sentence|beginner|advanced`와 `--language en|ko`로 목표를
  고릅니다.
- 영어 명령은 `nme convert`입니다.
- 확실하지 않은 것은 전부 Python으로 남습니다. 손실 있는 추측은 없습니다.
