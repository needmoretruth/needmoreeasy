# 31 — 기록: 작은 주소록

[English](31-address-book.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [14 — JSON](14-json.ko.md), [23 — 모듈](23-modules.ko.md)
- 주제 (Topic): 데이터/기록 / records
- 결과물 (Result): 연락처를 추가·목록·검색하는 JSON 파일 주소록 / a JSON-file address book that adds, lists, and searches contacts

기록은 필드가 몇 개 든 딕셔너리 하나입니다. 주소록은 그 기록들의 목록을
JSON으로 저장한 것입니다. 이 가이드는 메뉴로 움직이는 주소록을 만듭니다 —
연락처 추가, 모두 보기, 이름 검색, 그리고 변경할 때마다 파일에 저장까지.

## 단계

1. 연락처 하나는 `name`과 `phone`을 담은 딕셔너리입니다. 책 전체는 그런
   딕셔너리들의 목록입니다:

   ```text
   민아 = {"name": "Mina", "phone": "010-1234"}
   연락처 = [민아]
   말해 f"{연락처[0]['name']}: {연락처[0]['phone']}"
   ```

   실행하면 `Mina: 010-1234`가 출력됩니다. `연락처[0]`이 첫 딕셔너리고
   `['name']`이 그 필드를 꺼냅니다.

2. 실제 주소록은 저장된 파일을 불러오고, 파일이 아직 없으면 빈 목록으로
   시작합니다. [15](15-high-score.ko.md)에서 `os.path.exists`를 같은 방식으로
   썼습니다:

   ```text
   import os
   파일 사용 최신

   if os.path.exists("address.json"):
       연락처 = json읽기("address.json")
   else:
       연락처 = []
   ```

   `json읽기`는 저장된 그대로 딕셔너리 목록 전체를 돌려줍니다.

3. `add`는 `append`로 목록을 키우고 다시 씁니다. `json저장`은 딕셔너리
   하나뿐 아니라 목록도 받습니다:

   ```text
   연락처.append({"name": 이름, "phone": 전화})
   json저장("address.json", 연락처)
   ```

4. `list`는 `for` 반복으로 목록을 훑으며 기록의 두 필드를 모두
   출력합니다:

   ```text
   for contact in 연락처:
       말해 f"{contact['name']}: {contact['phone']}"
   ```

5. `search`는 같은 반복에서 거릅니다. `찾기 in contact["name"]`는 입력한
   글자가 이름 어디에든 있으면 참이므로 `Min`이 `Mina`를 찾습니다:

   ```text
   for contact in 연락처:
       if 찾기 in contact["name"]:
           말해 f"{contact['name']}: {contact['phone']}"
   ```

6. 메뉴 전체를 한 파일에 담습니다. `address.ko.nme`로 저장합니다:

   ```text
   # address.ko.nme — 작은 주소록 (JSON 파일에 저장)
   # 실행: nme r address.ko
   # add, list, search, quit 중 하나를 입력합니다.

   import os
   파일 사용 최신

   if os.path.exists("address.json"):
       연락처 = json읽기("address.json")
   else:
       연락처 = []

   while True:
       말해 ""
       말해 "명령: add, list, search, quit"
       물어봐 명령, "? "
       if 명령 == "add":
           물어봐 이름, "이름? "
           물어봐 전화, "전화? "
           연락처.append({"name": 이름, "phone": 전화})
           json저장("address.json", 연락처)
           말해 f"{이름}: {전화} 저장"
       elif 명령 == "list":
           말해 f"연락처 {len(연락처)}개"
           for contact in 연락처:
               말해 f"{contact['name']}: {contact['phone']}"
       elif 명령 == "search":
           물어봐 찾기, "검색어? "
           for contact in 연락처:
               if 찾기 in contact["name"]:
                   말해 f"{contact['name']}: {contact['phone']}"
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'add\nMina\n010-1234\nlist\nsearch\nMin\nquit\n' | nme r address.ko
   ```

   ```text
   명령: add, list, search, quit
   ? 이름? 전화? Mina: 010-1234 저장

   명령: add, list, search, quit
   ? 연락처 1개
   Mina: 010-1234

   명령: add, list, search, quit
   ? 검색어? Mina: 010-1234

   명령: add, list, search, quit
   ? 안녕!
   ```

   `add`가 새 연락처를 `address.json`에 저장하고, 다음 `nme r address.ko`가
   그 파일을 다시 불러오므로 주소록은 실행 사이에도 연락처를 지킵니다.
   `while True:`는 스스로 끝나지 않으므로 `quit`이 `break`로 나가야
   합니다 — [22](22-terminal-menu.ko.md)의 메뉴 모양입니다.

7. 영어는 같은 메뉴를 `use file latest`, `ask`, `json_save`로 씁니다. 전체
   영어 프로그램은 [영어 가이드](31-address-book.md)에 있고, 이 조각은
   저장된 주소록을 불러옵니다:

   ```text
   use file latest
   if os.path.exists("address.json"):
       contacts = json_load("address.json")
   else:
       contacts = []
   ```

## 직접 해보기

모든 연락처에 `email` 필드를 더해 보세요: `add`에서 물어보고, 딕셔너리에
저장하고, `list`에서 출력합니다. 저장된 JSON 모양이 바뀌지만 필드 없는
옛 파일도 그대로 불러와집니다 — 없는 필드는 그냥 아무것도 출력하지
않습니다.

## 배운 것

- 기록은 딕셔너리이고, 주소록은 그 딕셔너리들의 목록을 JSON으로 저장한
  것입니다.
- `json저장`은 딕셔너리 하나처럼 딕셔너리 목록도 씁니다.
- `while True:`에 `quit` 명령과 `break`가 있으면 스스로 끝나지 않는
  메뉴가 됩니다.
- `찾기 in contact["name"]`은 필드 안에서 글자를 검색합니다.
- `os.path.exists`는 첫 실행을 빈 목록으로 시작하게 해 줍니다.
