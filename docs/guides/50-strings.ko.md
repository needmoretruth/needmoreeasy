# 50 — 문자열: 텍스트 자르고 바꾸기

[English](50-strings.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [01 — 인사](01-hello.ko.md), [03 — 저장](03-set.ko.md)
- 주제: 문자열
- 결과물: 문장에서 `text[start:end]` 자르기와 `.upper()/.lower()`, `.replace()`, `.strip()`

문자열은 `0`부터 세는 글자의 줄입니다. 슬라이스가 조각을 잘라 내고,
몇 가지 메서드가 글을 바꿉니다 — 모두 평범한 Python이라 NME가 그대로
통과시킵니다.

## 단계

1. `text[start:end]`는 `start`부터 `end` **전까지** 자릅니다. 음수 인덱스는 끝에서 세므로 `text[-3:]`이 마지막 세 글자입니다. `hello`, `NME`, 그다음 `NME`와 ` NME`가 출력됩니다:

   ```nme
   text = "hello NME"
   말해 text[0:5]
   말해 text[6:9]
   말해 text[-3:]
   말해 text[-4:]
   ```

2. `.upper()`와 `.lower()`는 새 문자열로 대소문자를 바꾸고, `.replace("a", "o")`는 모든 글자를 맞바꿉니다 — 찾을 글자 먼저, 넣을 글자 그다음. `HELLO NME`, `hello nme`, `bononos`가 출력됩니다:

   ```nme
   text = "hello NME"
   말해 text.upper()
   말해 text.lower()
   ```
   ```nme
   text = "bananas"
   말해 text.replace("a", "o")
   ```

3. `.strip()`은 양끝 공백을 제거해 입력을 깨끗하게 만듭니다. `hello`가 출력됩니다:

   ```nme
   text = "  hello  "
   말해 text.strip()
   ```

4. 전부 하나의 프로그램에 넣습니다. `strings.ko.nme`으로 저장합니다. 진짜 입력 줄은 양끝 공백을 지니므로, 프로그램이 공백을 제거하고 조각을 자르고 깨끗한 문장을 단어로 나눕니다:

   ```nme
   # strings.ko.nme — 문장을 자르고 바꿉니다.
   # 실행: nme 실행 strings.ko
   # Python 문자열 도구: 자르기, 대소문자, 바꾸기, 공백 제거.

   문장을 물어봐 f"문장을 입력하세요: "

   말해 f"원문: {문장}"
   말해 f"길이: {len(문장)}글자"
   말해 f"첫 다섯 글자: {문장[0:5]}"
   말해 f"마지막 세 글자: {문장[-3:]}"
   말해 f"2번부터 10번까지: {문장[2:10]}"
   말해 f"대문자: {문장.upper()}"
   말해 f"소문자: {문장.lower()}"
   말해 f"양끝 공백 제거: {문장.strip()}"
   말해 f"a를 o로 바꾸기: {문장.replace('a', 'o')}"

   깨끗함 = 문장.strip()
   단어들 = 깨끗함.split()
   말해 f"공백 제거 후 {len(깨끗함)}글자: {깨끗함}"
   말해 f"제거한 문장의 첫 글자: {깨끗함[0]}"
   말해 f"제거한 문장의 마지막 글자: {깨끗함[-1]}"
   말해 f"단어 {len(단어들)}개; 처음은 {단어들[0]}, 마지막은 {단어들[-1]}"
   말해 "단어를 한 줄씩:"
   for 단어 in 단어들:
       말해 단어
   ```

5. 문장을 파이프로 넣어 실행합니다 — 입력 줄의 앞뒤 공백이 그대로 남아 있습니다:
   ```sh
   printf '  I love bananas  \n' | nme 실행 strings.ko
   ```
   ```text
   문장을 입력하세요: 원문:   I love bananas  
   길이: 18글자
   첫 다섯 글자:   I l
   마지막 세 글자: s  
   2번부터 10번까지: I love b
   대문자:   I LOVE BANANAS  
   소문자:   i love bananas  
   양끝 공백 제거: I love bananas
   a를 o로 바꾸기:   I love bononos  
   공백 제거 후 14글자: I love bananas
   제거한 문장의 첫 글자: I
   제거한 문장의 마지막 글자: s
   단어 3개; 처음은 I, 마지막은 bananas
   단어를 한 줄씩:
   I
   love
   bananas
   ```
   `bananas`는 모든 `a`가 `o`로 바뀌어 `bononos`가 됩니다. 나머지 공백은 그대로인데, 슬라이스와 `.replace()`가 원래 공백 있는 문자열에서 동작하기 때문입니다.
## 직접 해보기

문장을 하나 더 물어보고, `strip`한 뒤 첫 단어를 출력해 보세요. 두 글자를 맞바꾸고 `len`으로 `strip` 전후의 글자 수를 세어 보세요.
## 배운 것

- `text[start:end]`는 조각을 자르고, `text[-3:]`는 끝에서부터 자릅니다.
- `.upper()` / `.lower()`는 대소문자를 바꾸어 새 문자열을 만듭니다.
- `.replace("a", "o")`는 모든 글자를 맞바꿉니다.
- `.strip()`은 문자열 양끝의 공백을 제거합니다.
