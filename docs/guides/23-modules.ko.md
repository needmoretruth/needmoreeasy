# 23 — 모듈: 프로그램을 여러 파일로 나누기

[English](23-modules.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [15 — High score](15-high-score.ko.md), [13 — Files](13-files.ko.md)
- 주제 (Topic): 모듈 / modules
- 결과물 (Result): 프로그램을 여러 .nme 파일로 나누기 / splitting a program across .nme files

작은 프로그램은 파일 하나로 충분합니다. 프로젝트가 커지면 파일을 나눠 각
파일에 집중하게 만듭니다. NME는 목록에 적은 이름만 가져오므로 모든 모듈이
명확한 인터페이스를 갖고, 파일 사이에 다른 것은 새지 않습니다.

## 단계

1. 공용 함수를 모듈 파일에 둡니다. 모듈은 전체 프로그램을 실행하는 대신 값을
   정의하는 평범한 `.nme` 파일입니다:

   ```text
   # shapes.nme
   def rect(width, height):
       return width * height


   def circle(radius):
       return 3.14 * radius * radius
   ```

2. 주 프로그램에서 필요한 이름을 가져옵니다:

   ```text
   # area.nme
   from "shapes.nme" import rect, circle

   말해 rect(4, 5)
   말해 circle(2)
   ```

3. 주 프로그램을 실행하면 NME가 옆에 있는 `shapes.nme`를 찾습니다:

   ```sh
   nme r area
   ```

   ```text
   20
   12.56
   ```

   준비된 예제 쌍은 [`examples/modules/`](../../examples/modules/)에
   있습니다 — `area.nme`와 `shapes.nme`, 그리고 `_ko` 한국어 쌍.

4. 가져오기 이름 목록이 곧 인터페이스입니다. `rect`와 `circle`만 `area.nme`로
   넘어오고, 모듈의 다른 것은 비공개로 남습니다. 가져온 이름도 다른 값처럼
   문장에서 쓸 수 있습니다:

   ```text
   from "shapes.nme" import rect
   말해 rect(3, 7)
   ```

## 기억할 규칙

- 모듈 파일은 주 프로그램 옆에 있습니다.
- 파일 이름은 Python 식별자입니다: `my-shapes.nme`가 아니라 `shapes.nme`.
- 가져오기는 이어질 수 있습니다. 모듈이 다른 모듈을 가져올 수 있어요.
- `nme 검사`와 `nme 빌드`는 가져온 모듈도 확인합니다.

## 직접 해보기

`shapes.nme`에 `perimeter(width, height)` 함수를 추가하고, `area.nme`에서
가져와 가로 4, 세로 5 직사각형의 둘레를 보여 주세요.

## 배운 것

- `from "helper.nme" import name1, name2`는 목록에 적은 이름만 가져옵니다.
- 모듈은 같은 폴더에 있는 평범한 `.nme` 파일입니다.
- 가져온 이름은 문장과 호출에서 로컬 값처럼 씁니다.
- 명확한 인터페이스는 파일 사이에 숨은 전역 상태가 없음을 뜻합니다.
