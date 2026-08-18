# 70 — Patterns: finding matches with regex

English | [한국어](70-patterns.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [59 — Errors](59-errors.md), [51 — Strings](51-strings.md)
- Topic: regular expressions
- Result: a program that finds phone numbers and email addresses in a text file

"Find all phone numbers" needs a pattern, not a fixed word. The standard
`re` library matches shapes: three digits, a hyphen, four digits — and
anything that looks like an email address.

## Steps

1. `\d` matches any digit and `{3}` means exactly three, so `\d{3}-\d{4}`
   is three digits, a hyphen, four digits. `re.findall` returns every match:
   ```nme
   import re
   text = "Call 010-1234-5678 now"
   phones = re.findall(r"\d{3}-\d{4}", text)
   show phones
   ```
   ```text
   ['010-1234']
   ```
2. An email is one-or-more allowed characters, `@`, then one-or-more more.
   `+` means one or more; brackets list the allowed characters:
   ```nme
   import re
   text = "Write to mina@nme.kr or jun@example.com"
   emails = re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", text)
   show emails
   ```
   ```text
   ['mina@nme.kr', 'jun@example.com']
   ```
3. Create a small file of contacts:
   ```text
   Mina 010-1234-5678 mina@nme.kr
   Jun 010-9876-5432 jun@example.com
   Office 02-3456-7890 hello@example.org
   ```
4. The full program reads the file with `open(...).read()`. The `try` /
   `except` from guide [59](59-errors.md) reports a missing file:
   ```nme
   # contacts.nme — find phone numbers and emails in a text file.
   # Run: nme r contacts
   # re.findall searches a whole file with one pattern.

   import re

   ask file_name, "Text file to scan (for example contacts.txt): "

   try:
       text = open(file_name).read()
   except FileNotFoundError:
       show f"{file_name} is not in this folder."
   else:
       phones = re.findall(r"\d{3}-\d{4}", text)
       emails = re.findall(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", text)

       show f"Found {len(phones)} phone numbers:"
       for phone in phones:
           show f"  {phone}"

       show f"Found {len(emails)} email addresses:"
       for email in emails:
           show f"  {email}"

       both = 0
       for line in text.splitlines():
           if re.search(r"\d{3}-\d{4}", line) and re.search(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+", line):
               both = both + 1
       show f"{both} of {len(text.splitlines())} lines have both a phone and an email."
   ```
5. Run it:
   ```sh
   printf 'contacts.txt\n' | nme r contacts
   ```
   ```text
   Text file to scan (for example contacts.txt): Found 3 phone numbers:
     010-1234
     010-9876
     456-7890
   Found 3 email addresses:
     mina@nme.kr
     jun@example.com
     hello@example.org
   3 of 3 lines have both a phone and an email.
   ```
6. `re.findall` collects every match in the text. `re.search(pattern,
   line)` answers one yes/no — it returns a match object or `None` — which
   is how the program counts lines that have both.

## Try it yourself

Add a row with a second phone style and widen the pattern, or search for a
name like `Mina`. Report lines that have a phone but no email.

## What you learned

- `import re` loads the standard regular-expression library.
- `re.findall(pattern, text)` returns every match in the text as a list.
- `\d` is any digit, `{3}` means exactly three, and `+` means one or more.
- `re.search(pattern, line)` tests one line and returns `None` when there
  is no match.
