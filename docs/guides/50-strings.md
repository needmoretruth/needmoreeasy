# 50 — Strings: slicing and changing text

English | [한국어](50-strings.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★☆ (4/5)
- 선수 지식 (Prerequisites): [01 — Hello](01-hello.md), [03 — Set](03-set.md)
- 주제 (Topic): 문자열 / strings
- 결과물 (Result): 문장에서 `text[start:end]` 자르기와 `.upper()/.lower()`, `.replace()`, `.strip()` / slicing `text[start:end]`, `.upper()/.lower()`, `.replace()`, and `.strip()` on a sentence

A string is a row of characters counted from `0`; slicing cuts a piece out,
and a few methods change the text — all ordinary Python, passed through unchanged.

## Steps

1. `text[start:end]` cuts from `start` up to but **not** including `end`; negative indexes count from the end, so `text[-3:]` is the last three. It prints `hello`, `NME`, then `NME` and ` NME`:

   ```text
   text = "hello NME"
   show text[0:5]
   show text[6:9]
   show text[-3:]
   show text[-4:]
   ```

2. `.upper()` and `.lower()` change case into a new string; `.replace("a", "o")` swaps every occurrence, find first and replacement second. They print `HELLO NME`, `hello nme`, and `bononos`:

   ```text
   text = "hello NME"
   show text.upper()
   show text.lower()
   ```
   ```text
   text = "bananas"
   show text.replace("a", "o")
   ```

3. `.strip()` removes spaces from both ends, which cleans up typed input; it prints `hello`:

   ```text
   text = "  hello  "
   show text.strip()
   ```

4. All together, saved as `strings.nme`. A real typed line keeps edge spaces, so the program strips it, slices pieces, and splits the clean text into words:

   ```text
   # strings.nme — slice and change a sentence.
   # Run: nme r strings
   # Python string tools: slicing, case, replace, and strip.

   ask text, "Type a sentence: "

   show f"Original: {text}"
   show f"Length: {len(text)} characters"
   show f"First five characters: {text[0:5]}"
   show f"Last three characters: {text[-3:]}"
   show f"Slice from index 2 to 10: {text[2:10]}"
   show f"Uppercase: {text.upper()}"
   show f"Lowercase: {text.lower()}"
   show f"Without edge spaces: {text.strip()}"
   show f"Swap a for o: {text.replace('a', 'o')}"

   clean = text.strip()
   words = clean.split()
   show f"Trimmed to {len(clean)} characters: {clean}"
   show f"First character of the trimmed text: {clean[0]}"
   show f"Last character of the trimmed text: {clean[-1]}"
   show f"{len(words)} words; the first is {words[0]} and the last is {words[-1]}"
   show "Each word, one per line:"
   for word in words:
       show word
   ```

5. Run it with a sentence piped in — the typed line keeps its edge spaces:
   ```sh
   printf '  I love bananas  \n' | nme r strings
   ```
   ```text
   Type a sentence: Original:   I love bananas  
   Length: 18 characters
   First five characters:   I l
   Last three characters: s  
   Slice from index 2 to 10: I love b
   Uppercase:   I LOVE BANANAS  
   Lowercase:   i love bananas  
   Without edge spaces: I love bananas
   Swap a for o:   I love bononos  
   Trimmed to 14 characters: I love bananas
   First character of the trimmed text: I
   Last character of the trimmed text: s
   3 words; the first is I and the last is bananas
   Each word, one per line:
   I
   love
   bananas
   ```
   `bananas` becomes `bononos` when every `a` is swapped for `o`; the other spaces stay because the slices and `.replace()` use the untrimmed original.
## Try it yourself

Ask for a second sentence, strip it, and print its first word. Swap two letters and count the characters with `len` before and after `strip`.

## What you learned

- `text[start:end]` cuts a slice; `text[-3:]` starts from the end.
- `.upper()` / `.lower()` change case and make a new string.
- `.replace("a", "o")` swaps every occurrence of one text for another.
- `.strip()` removes spaces at both ends of a string.
