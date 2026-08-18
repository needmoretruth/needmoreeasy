# 68 — 예외: 문제 다루기

[English](68-errors.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [55 — 네트워크](55-net.ko.md), [13 — 파일](13-files.ko.md)
- 주제: 예외
- 결과물: 없는 파일을 읽거나 숫자가 아닌 입력을 바꿔도 멈추지 않는 프로그램

파일이 없거나 숫자가 아닌 줄을 만나면 프로그램은 그 자리에서 멈춥니다.
`try` / `except`가 그 문제를 잡아 친절한 대신 내용을 실행합니다.

## 단계

1. 없는 파일은 `파일읽기`가 `FileNotFoundError`를 일으키게 합니다. `except` 블록이 멈추는 대신 실행됩니다:
   ```text
   파일 사용 최신
   try:
       내용 = 파일읽기("notes.txt")
   except FileNotFoundError:
       "notes.txt 파일이 없어요." 말해줘
   ```
2. `int()`는 `일곱` 같은 입력에서 `ValueError`를 일으킵니다. `except` 뒤의 `else`는 아무 문제가 없었을 때만 실행됩니다:
   ```text
   답을 물어봐 "두 배로 만들 숫자: "
   try:
       숫자 = int(답)
   except ValueError:
       "그건 숫자가 아니에요." 말해줘
   else:
       f"두 배는 {숫자 * 2}입니다." 말해줘
   ```
3. 전체 프로그램은 없을 수도 있는 파일을 읽고, 숫자를 물어봐 답이 바뀔 때까지 다시 묻습니다. 먼저 `notes.txt`를 두 줄로 만드세요:
   ```text
   # 안전읽기.nme — 없는 파일이어도 안전하게 읽습니다.
   # 실행: nme 실행 안전읽기
   # try / except는 데이터가 나빠도 프로그램이 멈추지 않게 합니다.

   파일 사용 최신

   이름을 물어봐 "읽을 문서 이름을 입력하세요: "
   try:
       내용 = 파일읽기(이름)
   except FileNotFoundError:
       "그 문서는 아직 없어요." 말해줘
       "먼저 file_write로 만든 뒤 다시 실행하세요." 말해줘
   else:
       f"{len(내용)}글자를 읽었어요." 말해줘
       f"단어는 {len(내용.split())}개예요." 말해줘
       줄들 = 내용.splitlines()
       f"첫 줄은: {줄들[0]}" 말해줘
       "내용:" 말해줘
       내용 보여줘

   "이제 두 배로 만들 숫자를 주세요." 말해줘
   계속 = True
   동안 계속:
       답을 물어봐 "두 배로 만들 숫자: "
       try:
           숫자 = int(답)
       except ValueError:
           f"'{답}'은(는) 숫자가 아니에요 — 다시 주세요." 말해줘
       else:
           계속 = False

   f"두 배는 {숫자 * 2}입니다." 말해줘
   ```
4. 파일이 있는 상태에서 실행합니다:
   ```sh
   printf 'notes.txt\n일곱\n12\n' | nme 실행 안전읽기
   ```
   ```text
   읽을 문서 이름을 입력하세요: 30글자를 읽었어요.
   단어는 6개예요.
   첫 줄은: Today is sunny.
   내용:
   Today is sunny.
   We study NME.

   이제 두 배로 만들 숫자를 주세요.
   두 배로 만들 숫자: '일곱'은(는) 숫자가 아니에요 — 다시 주세요.
   두 배로 만들 숫자: 두 배는 24입니다.
   ```
   없는 파일 이름으로 실행합니다:
   ```sh
   printf 'nope.txt\n12\n' | nme 실행 안전읽기
   ```
   ```text
   읽을 문서 이름을 입력하세요: 그 문서는 아직 없어요.
   먼저 file_write로 만든 뒤 다시 실행하세요.
   이제 두 배로 만들 숫자를 주세요.
   두 배로 만들 숫자: 두 배는 24입니다.
   ```
5. 그냥 `except:` 대신 이름 있는 오류(`FileNotFoundError`, `ValueError`)를 잡으세요. 진짜 버그는 크게 드러나야 합니다.

## 직접 해보기

`json읽기`로 JSON 파일을 읽고 `json.JSONDecodeError`를 잡아 보거나, 숫자 반복을 두 숫자를 더하는 것으로 바꿔 보세요.

## 배운 것

- `try:`가 위험한 코드를 실행하고, `except 어떤오류:`가 바로 그 오류만 잡습니다.
- `FileNotFoundError`는 파일이 없을 때, `ValueError`는 `int()`가 바꾸지 못할 때 생깁니다.
- `except` 뒤의 `else:`는 오류가 없었을 때만 실행됩니다.
- 이름 있는 오류를 잡으세요. 뜻밖의 버그는 크게 드러나야 합니다.
