# 14 — JSON: 데이터 저장하고 불러오기

[English](14-json.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★☆☆ (3/5)
- 선수 지식: [13 — 파일](13-files.ko.md)
- 주제: JSON
- 결과물: 이름과 점수를 저장하고 불러오는 프로그램

파일 하나에 기록 전체를 담을 수 있습니다. JSON은 여러 값을 한 덩어리의
글로 저장해서, 이름과 점수가 나중에 — 프로그램이 끝난 뒤에도 — 함께
돌아옵니다.

## 단계

1. `record.nme` 파일을 만들고 기록을 저장합니다:

   ```text
   파일 사용 최신
   기록 = {"이름": "민수", "점수": 7}
   json저장("기록.json", 기록)
   ```

   `json저장`이 딕셔너리를 `기록.json`에 JSON 글로 씁니다.

2. 실행하고 새 파일을 봅니다:

   ```sh
   nme 실행 record
   cat 기록.json
   ```

   파일에는 `{"이름": "민수", "점수": 7}`로 데이터가 들어 있습니다.

3. 새 프로그램 `load.nme`에서 다시 불러옵니다:

   ```text
   파일 사용 최신
   기록 = json읽기("기록.json")
   말해 f"{기록['이름']}: {기록['점수']}점"
   ```

   `nme 실행 load`를 실행하면 `민수: 7점`이 출력됩니다 — 값은 첫 프로그램이
   끝난 뒤에도 살아 있었습니다.

4. 영어는 `json_save`와 `json_load`입니다:

   ```text
   use file latest
   record = {"name": "Mina", "score": 7}
   json_save("record.json", record)
   record = json_load("record.json")
   say f"{record['name']} scored {record['score']}"
   ```

## 직접 해보기

도시와 나이를 담은 딕셔너리를 저장하고, 한 프로그램에서 다시 불러와
보세요:

```text
파일 사용 최신
내정보 = {"도시": "서울", "나이": 12}
json저장("내정보.json", 내정보)
돌아온 = json읽기("내정보.json")
말해 f"{돌아온['도시']}에 살고 나이는 {돌아온['나이']}살"
```

## 배운 것

- `json저장(경로, 값)` / `json_save(path, value)`가 딕셔너리를 JSON 파일로
  씁니다.
- `json읽기(경로)` / `json_load(path)`가 파일을 읽어 딕셔너리로 되돌립니다.
- JSON 파일은 프로그램이 끝난 뒤에도 남고, 다른 실행에서 불러올 수
  있습니다.
- 딕셔너리 안의 값은 `기록["이름"]` 같은 Python 대괄호로 꺼냅니다.
