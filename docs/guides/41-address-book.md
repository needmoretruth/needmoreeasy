# 41 — Records: a small address book

English | [한국어](41-address-book.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [39 — JSON](39-json.md), [37 — Files](37-files.md)
- Topic: records
- Result: a JSON-file address book that adds, lists, and searches contacts

A record is one dict with several fields. An address book is a list of those
records saved as JSON. This guide builds a menu-driven book that adds a
contact, lists everyone, and searches by name — each change saved to a file.

## Steps

1. One contact is a dict with a `name` and a `phone`. The whole book is a list
   of those dicts:

   ```nme
   mina = {"name": "Mina", "phone": "010-1234"}
   contacts = [mina]
   show f"{contacts[0]['name']}: {contacts[0]['phone']}"
   ```

   Run it; it prints `Mina: 010-1234`. `contacts[0]` is the first dict and
   `['name']` picks the field out of it.

2. A real book loads a saved file, or begins empty when the file does not exist
   yet. Guide [23](23-high-score.md) used `os.path.exists` the same way:

   ```nme
   import os
   use file latest

   if os.path.exists("address.json"):
       contacts = json_load("address.json")
   else:
       contacts = []
   ```

   `json_load` returns the whole list of dicts, exactly as it was saved.

3. `add` grows the list with `append` and writes it back. `json_save` accepts a
   list, not just a dict:

   ```nme
   contacts.append({"name": name, "phone": phone})
   json_save("address.json", contacts)
   ```

4. `list` walks the list with a `for` loop and prints both fields of each
   record:

   ```nme
   for contact in contacts:
       show f"{contact['name']}: {contact['phone']}"
   ```

5. `search` filters the same loop. `find in contact["name"]` is true when the
   typed text appears anywhere in the name, so `Min` finds `Mina`:

   ```nme
   for contact in contacts:
       if find in contact["name"]:
           show f"{contact['name']}: {contact['phone']}"
   ```

6. The whole menu in one file. Save `address.nme`:

   ```nme
   # address.nme — a small address book in a JSON file.
   # Run: nme r address
   # Type add, list, search, or quit.

   import os
   use file latest

   # Load saved contacts, or start with an empty list.
   if os.path.exists("address.json"):
       contacts = json_load("address.json")
   else:
       contacts = []

   while True:
       show ""
       show "Commands: add, list, search, quit"
       ask command, "? "
       if command == "add":
           ask name, "Name? "
           ask phone, "Phone? "
           contacts.append({"name": name, "phone": phone})
           json_save("address.json", contacts)
           show f"Saved {name}: {phone}"
       elif command == "list":
           show f"{len(contacts)} contacts"
           for contact in contacts:
               show f"{contact['name']}: {contact['phone']}"
       elif command == "search":
           ask find, "Search? "
           for contact in contacts:
               if find in contact["name"]:
                   show f"{contact['name']}: {contact['phone']}"
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   Run it and feed the commands through a pipe:

   ```sh
   printf 'add\nMina\n010-1234\nlist\nsearch\nMin\nquit\n' | nme r address
   ```

   ```text
   Commands: add, list, search, quit
   ? Name? Phone? Saved Mina: 010-1234

   Commands: add, list, search, quit
   ? 1 contacts
   Mina: 010-1234

   Commands: add, list, search, quit
   ? Search? Mina: 010-1234

   Commands: add, list, search, quit
   ? Bye!
   ```

   `add` saves the new contact to `address.json`; the next `nme r address`
   loads it back, so the book keeps its contacts between runs. `while True:`
   never ends on its own, so `quit` must `break` out — the menu shape from
   guide [22](22-terminal-menu.md).

7. Korean writes the same menu with `파일 사용 최신`, `물어봐`, and `json저장`.
   The full Korean program is in the [Korean guide](41-address-book.ko.md);
   this snippet loads the saved book:

   ```nme
   파일 사용 최신
   if os.path.exists("address.json"):
       연락처 = json읽기("address.json")
   else:
       연락처 = []
   ```

## Try it yourself

Add an `email` field to every contact: ask for it in `add`, save it in the
dict, and print it in `list`. The saved JSON changes shape, and old files
without the field still load — a missing field simply prints nothing.

## What you learned

- A record is a dict; a book is a list of dicts saved as JSON.
- `json_save` writes a list of dicts just like a single dict.
- `while True:` with a `quit` command and `break` makes a menu that never
  ends on its own.
- `find in contact["name"]` searches for text inside a field.
- `os.path.exists` lets the first run start with an empty list.
