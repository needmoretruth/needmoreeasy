# 42 — 비교: 두 숫자 묶음

[English](42-compare.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [30 — 데이터](30-data.ko.md), [14 — JSON](14-json.ko.md)
- 주제: 데이터/비교
- 결과물: 두 JSON 숫자 목록을 불러와 평균과 최댓값을 비교하는 프로그램

[30](30-data.ko.md) 가이드는 숫자 목록 하나를 요약했습니다. 목록 두 개를
비교하는 것은 같은 질문을 두 번 하고 어느 묶음이 더 큰지 정하는 일입니다.

## 단계

1. 데이터 파일 두 개를 만들고 불러옵니다. 각 파일은 JSON 목록 하나를 담습니다 ([14](14-json.ko.md) 가이드가 `json저장`으로 저장한 모양):

   ```text
   # a.json
   [5, 2, 9, 1, 7, 3]
   # b.json
   [8, 4, 6, 10, 3]
   ```

   ```text
   파일 사용 최신
   a = json읽기("a.json")
   b = json읽기("b.json")
   보여줘 f"a.json: {a}"
   보여줘 f"b.json: {b}"
   ```

   `a.json: [5, 2, 9, 1, 7, 3]`과 `b.json: [8, 4, 6, 10, 3]`이 출력됩니다.

2. 평균을 손으로, 라이브러리로, 그리고 최댓값. `sum(a) / len(a)`는 평균이고,
   `statistics.mean`([30](30-data.ko.md) 가이드)이 확인하며, `max`는 내장 함수입니다:

   ```text
   from statistics import mean
   a = [5, 2, 9, 1, 7, 3]
   a평균 = sum(a) / len(a)
   보여줘 f"a 평균: {a평균}"
   보여줘 f"a statistics 평균: {mean(a)}"
   보여줘 f"a 최대: {max(a)}"
   ```

   a는 4.5입니다 (27을 6으로). b는 합 31을 5로 나눠 평균 6.2, 최댓값 10입니다.

3. 비교 프로그램입니다. 두 JSON 파일과 함께 `비교.nme`로 저장하고 실행합니다:

   ```text
   # 비교.nme — 두 숫자 묶음 중 어느 쪽이 더 클까요?
   # 실행: nme 실행 비교
   # a.json과 b.json 파일이 같은 폴더에 있어야 합니다.

   파일 사용 최신
   from statistics import mean

   a = json읽기("a.json")
   b = json읽기("b.json")

   # 손으로 쓴 평균, 내장 함수 최댓값.
   a평균 = sum(a) / len(a)
   b평균 = sum(b) / len(b)
   a최대 = max(a)
   b최대 = max(b)
   보여줘 f"a: 평균 {a평균}, 최대 {a최대}"
   보여줘 f"b: 평균 {b평균}, 최대 {b최대}"

   # statistics.mean이 손으로 쓴 답을 확인해 줍니다.
   보여줘 f"a statistics 평균: {mean(a)}"
   보여줘 f"b statistics 평균: {mean(b)}"

   # 두 묶음을 비교합니다.
   if a평균 > b평균:
       보여줘 "a의 평균이 더 높습니다"
   else:
       보여줘 "b의 평균이 더 높습니다"
   if a최대 > b최대:
       보여줘 "a의 최댓값이 더 높습니다"
   else:
       보여줘 "b의 최댓값이 더 높습니다"
   ```

   ```sh
   nme 실행 비교
   ```
   ```text
   a: 평균 4.5, 최대 9
   b: 평균 6.2, 최대 10
   a statistics 평균: 4.5
   b statistics 평균: 6.2
   a의 평균이 더 높습니다
   b의 최댓값이 더 높습니다
   ```

   `b`가 두 비교 모두 이깁니다. 영어 가이드는 `json_load`와 `show`로 씁니다 — 전체 쌍은
   [42-compare.md](42-compare.md)에 있습니다.

## 직접 해보기

`b.json`을 `[3, 1, 4, 1, 5, 9]`로 바꾸고 다시 실행하면 답이 뒤집힙니다.

## 배운 것

- `json읽기`가 JSON 목록 파일 하나를 Python 목록으로 바꿉니다.
- `sum(숫자들) / len(숫자들)`이 손으로 쓴 평균입니다.
- `statistics.mean`이 같은 답을 호출 하나로 줍니다.
- `max(숫자들)`은 내장 함수이고, `if`/`else`가 숫자를 문장으로 바꿉니다.
