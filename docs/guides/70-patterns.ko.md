# 70 — 정규식: 패턴 찾기

[English](70-patterns.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [59 — 예외](59-errors.ko.md), [51 — 문자열](51-strings.ko.md)
- 주제: 정규식
- 결과물: 텍스트 파일에서 전화번호와 이메일을 찾는 프로그램

"전화번호를 모두 찾아"는 고정된 단어가 아니라 패턴이 필요합니다. 표준
`re` 라이브러리는 숫자 셋, 하이픈, 숫자 넷 같은 모양 — 그리고 이메일처럼
보이는 것 — 을 찾습니다.

## 단계

1. `\d`는 아무 숫자 하나, `{3}`은 정확히 셋을 뜻하므로 `\d{3}-\d{4}`는
   숫자 셋, 하이픈, 숫자 넷입니다. `re.findall`이 모든 매치를 돌려줍니다:
   ```nme
   import re
   text = "전화는 010-1234-5678입니다"
   전화들 = re.findall(r"\d{3}-\d{4}", text)
   show 전화들
   ```
   ```text
   ['010-1234']
   ```
2. 이메일은 허용되는 글자 하나 이상, `@`, 그다음 글자 하나 이상입니다.
   `+`는 하나 이상을 뜻하고, 대괄호는 허용되는 글자를 나열합니다:
   ```nme
   import re
   text = "메일은 mina@nme.kr 또는 jun@example.com으로"
   이메일들 = re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", text)
   show 이메일들
   ```
   ```text
   ['mina@nme.kr', 'jun@example.com']
   ```
3. 작은 연락처 파일을 만드세요:
   ```text
   Mina 010-1234-5678 mina@nme.kr
   Jun 010-9876-5432 jun@example.com
   Office 02-3456-7890 hello@example.org
   ```
4. 전체 프로그램은 `open(...).read()`로 파일을 읽습니다. 가이드
   [59](59-errors.ko.md)의 `try` / `except`가 없는 파일을 알려 줍니다:
   ```nme
   # 연락처.nme — 텍스트 파일에서 전화번호와 이메일을 찾습니다.
   # 실행: nme 실행 연락처
   # re.findall은 패턴 하나로 파일 전체를 찾습니다.

   import re

   이름을 물어봐 "찾을 문서 이름: "
   try:
       내용 = open(이름).read()
   except FileNotFoundError:
       f"{이름} 문서가 이 폴더에 없어요." 말해줘
   else:
       전화들 = re.findall(r"\d{3}-\d{4}", 내용)
       이메일들 = re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", 내용)

       f"전화번호 {len(전화들)}개를 찾았어요:" 말해줘
       for 전화 in 전화들:
           show f"  {전화}"

       f"이메일 {len(이메일들)}개를 찾았어요:" 말해줘
       for 이메일 in 이메일들:
           show f"  {이메일}"

       둘다 = 0
       for 줄 in 내용.splitlines():
           if re.search(r"\d{3}-\d{4}", 줄) and re.search(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", 줄):
               둘다 = 둘다 + 1
       f"전화와 이메일이 모두 있는 줄은 {둘다}줄입니다." 말해줘
   ```
5. 실행합니다:
   ```sh
   printf 'contacts.txt\n' | nme 실행 연락처
   ```
   ```text
   찾을 문서 이름: 전화번호 3개를 찾았어요:
     010-1234
     010-9876
     456-7890
   이메일 3개를 찾았어요:
     mina@nme.kr
     jun@example.com
     hello@example.org
   전화와 이메일이 모두 있는 줄은 3줄입니다.
   ```
6. `re.findall`은 전체 텍스트에서 모든 매치를 모읍니다. `re.search(패턴,
   줄)`은 대신 예/아니오 하나를 답합니다 — 찾으면 매치 객체를, 못 찾으면
   `None`을 돌려주며, 둘 다 있는 줄을 셀 때 그렇게 사용합니다.

## 직접 해보기

두 번째 전화 형식의 줄을 추가하고 패턴을 넓혀 보거나, `Mina`처럼 이름을
찾아 보세요. 전화는 있는데 이메일이 없는 줄을 보고하도록 바꿔 보세요.

## 배운 것

- `import re`가 표준 정규식 라이브러리를 불러옵니다.
- `re.findall(패턴, 텍스트)`가 텍스트 안의 모든 매치를 목록으로 돌려줍니다.
- `\d`는 아무 숫자, `{3}`은 정확히 셋, `+`는 하나 이상입니다.
- `re.search(패턴, 줄)`은 한 줄을 시험하고 매치가 없으면 `None`을 돌려줍니다.
