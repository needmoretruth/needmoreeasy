# NME learning guides

English | [한국어](index.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

90 short guides, written to be read in order and grouped into eight parts.
Each one teaches a single idea and ends with something to try and a summary of
what it taught.

**Nothing needs installing.** Every code block has a "run it" link that carries
the program into the writing box on
[needmoreeasy.com](https://needmoreeasy.com/), phone included. Every program
printed here has been through the real compiler.

## Difficulty

- ★☆☆☆☆ (1/5) — first day. A few sentences and it is done.
- ★★☆☆☆ (2/5) — conditions and loops appear. Still sentences only.
- ★★★☆☆ (3/5) — a whole program of your own.
- ★★★★☆ (4/5) — beginner syntax or Python is mixed in.
- ★★★★★ (5/5) — essentially Python.

The star is **how hard it is**, not the order of the numbers. Every part starts
again with the easiest guide of its subject, and inside a part a guide rewritten
in sentences gets easier by that much.

**How far the sentence-only guides go** can be measured at any time:

    python3 scripts/report-guide-tier.py --guides

## Learn in order

### Part 1 — Starting with sentences

For someone who has never written a program: sentences only, one new idea per
guide.

1. [01 — Hello: say your first words](01-hello.md)
2. [02 — Story: writing many lines without repeating yourself](02-story.md)
3. [03 — Story: letters arriving one at a time](03-slow-story.md)
4. [04 — Ask: have a conversation](04-ask.md)
5. [05 — Set: store a value](05-set.md)
6. [06 — Update: change a value](06-update.md)
7. [07 — Repeat: do something many times](07-repeat.md)
8. [08 — If: make a choice](08-if.md)
9. [09 — And / Or: combine conditions](09-and-or.md)
10. [10 — While: keep going](10-while.md)
11. [11 — Break: stop a loop](11-break.md)
12. [12 — Random: dice and picks](12-random.md)
13. [13 — Chance: how often out of a hundred](13-chance.md)
14. [14 — Screen: clearing, ruling and boxing](14-screen.md)
15. [15 — Time: a stopwatch and cooldowns](15-timer.md)
16. [16 — Check & Build: see the Python](16-check-build.md)

### Part 2 — Small programs you can build

For someone who has finished Part 1 and wants games, menus and small projects
to keep. **Guides 17, 18, 19, 20, 22, 23, 27 and 28 are written in sentences alone** — no
quote, no bracket, no equals sign. The rest still mix in beginner syntax and
Python. The
guides written in sentences alone are Part 1 — its code contains no quote, no
bracket and no equals sign, which `scripts/report-guide-tier.py` measures.

17. [17 — Word guess: a hidden-word game](17-word-guess.md)
18. [18 — Adventure — a small text game](18-adventure.md)
19. [19 — Game: how fast are you?](19-reaction.md)
20. [20 — ASCII art — drawing with characters](20-ascii-art.md)
21. [21 — Progress — showing how far along you are](21-progress.md)
22. [22 — Terminal menu — a small TUI](22-terminal-menu.md)
23. [23 — High score: a small project](23-high-score.md)
24. [24 — Quiz — questions from a file](24-quiz.md)
25. [25 — Calculator — a command-line project](25-calculator.md)
26. [26 — Game: tic-tac-toe](26-tic-tac-toe.md)
27. [27 — Game: guessing a hidden set of colours](27-mastermind.md)
28. [28 — Game: an opponent that knows the rule](28-ai.md)
29. [29 — Playlist: a random music player](29-playlist.md)
30. [30 — Shop — an inventory store](30-shop.md)
31. [31 — Project — a mini bank](31-bank.md)
32. [32 — Project — a grade book](32-grade-book.md)
33. [33 — Project: a habit tracker](33-habit.md)
34. [34 — Chart: a bar chart in the terminal](34-chart.md)
35. [35 — Todo: a growing project](35-todo.md)
36. [36 — Diary: notes saved by date](36-diary.md)

### Part 3 — Files and data

For someone who wants a program to remember things after it stops: files,
JSON, CSV and reports. **Guides 37, 38 and 40 are written in sentences** — except
that a file name is wrapped in quotes. That is the only place sentence syntax
uses them, and [guide 37](37-files.md) says why.

⚠ **From this part on, the programs do not run on the site.** There is nowhere
inside a browser to keep a file. [Install](../install.md) NME and try them on
your own computer.

37. [37 — Files: saving and reading again](37-files.md)
38. [38 — Name list: reading a file line by line](38-name-list.md)
39. [39 — JSON: save and load data](39-json.md)
40. [40 — CSV: several fields on one line](40-csv.md)
41. [41 — Records: a small address book](41-address-book.md)
42. [42 — Word count: how often each word appears](42-word-count.md)
43. [43 — Text stats: letters and words](43-text-stats.md)
44. [44 — Log: an event record](44-log.md)
45. [45 — Group: data by category](45-group.md)
46. [46 — Top ten: ranking records](46-top-ten.md)
47. [47 — Search — finding items in JSON](47-search.md)
48. [48 — Files: processing many files](48-files-folder.md)
49. [49 — Report — writing a summary file](49-report.md)
50. [50 — Editor — a tiny text editor](50-editor.md)

### Part 4 — More with lists and text

For someone handling longer lists and longer text: sorting, sets, grids and
search order.

51. [51 — Strings: slicing and changing text](51-strings.md)
52. [52 — Sorting — putting a list in order](52-sorting.md)
53. [53 — Sets: unique values](53-sets.md)
54. [54 — Merge: joining two lists](54-merge.md)
55. [55 — Grid: a board of lists](55-grid.md)
56. [56 — Bubble sort — your first algorithm](56-bubble.md)
57. [57 — Binary search — halving the guess](57-binary-search.md)
58. [58 — Data: statistics on a list](58-data.md)

### Part 5 — When a program grows: errors, tests and files

For someone who wants shorter code and larger programs: beginner syntax,
advanced syntax, modules and tools.

59. [59 — Errors: handling problems](59-errors.md)
60. [60 — Testing: checking the functions you wrote](60-testing.md)
61. [61 — Modules: split your program into files](61-modules.md)
62. [62 — Modules: a project across files](62-project-files.md)
63. [63 — Tools: reading command-line arguments](63-argv.md)
64. [64 — Python packages: the standard library and installed libraries](64-python-packages.md)
65. [65 — Convert: turn Python into NME](65-convert.md)
66. [66 — Native: compile to machine code](66-native.md)

### Part 6 — Analysis and the internet

For someone with data to understand or a server to ask: statistics, patterns
and the network.

67. [67 — Stats: understanding data](67-stats.md)
68. [68 — Compare: two groups of numbers](68-compare.md)
69. [69 — Data: analyzing a month of temperatures](69-analysis.md)
70. [70 — Patterns: finding matches with regex](70-patterns.md)
71. [71 — HTTP — asking a web server](71-http.md)
72. [72 — Weather: reading JSON from the web](72-weather.md)
73. [73 — Network: polling a server](73-poll.md)
74. [74 — Network: downloading files](74-download.md)
75. [75 — Web: extracting links from a page](75-links.md)
76. [76 — Network: a mini chat](76-net.md)

### Part 7 — Building a language of your own

For someone who wants to know how a language is built: a compiler written step
by step.

77. [77 — Your first compiler — a tiny language](77-compiler.md)
78. [78 — Compiler tier: a small expression language](78-expressions.md)
79. [79 — Compiler tier: reading tokens](79-tokens.md)
80. [80 — Compiler tier: expressions as trees](80-ast.md)
81. [81 — Compiler tier: functions in the mini language](81-functions.md)
82. [82 — Compiler tier: a tiny bytecode runner](82-bytecode.md)
83. [83 — Compiler tier: from tree to bytecode](83-bytecode-compiler.md)
84. [84 — Bootstrap: NME compiling a tiny language](84-bootstrap.md)
85. [85 — Self-host: NME running NME](85-selfhost.md)
86. [86 — Capstone: a language that compiles to Python](86-capstone.md)

### Part 8 — Cryptocurrency

For someone who wants to see what a cryptocurrency actually is: a ledger built
and validated by hand.

87. [87 — Cryptocurrency ledger — state and transactions](87-blockchain.md)
88. [88 — Proof of work — mining transaction blocks](88-proof-of-work.md)
89. [89 — Transaction proofs — wallet ownership and replay prevention](89-signatures.md)
90. [90 — Full-chain validation — recompute from genesis](90-consensus.md)

## Topic lookup

- first steps: [01](01-hello.md), [02](02-story.md), [03](03-slow-story.md), [04](04-ask.md), [05](05-set.md), [06](06-update.md), [07](07-repeat.md)
- choosing and repeating: [08](08-if.md), [09](09-and-or.md), [10](10-while.md), [11](11-break.md)
- random and chance: [12](12-random.md), [13](13-chance.md), [29](29-playlist.md)
- screen and time: [14](14-screen.md), [20](20-ascii-art.md), [21](21-progress.md), [34](34-chart.md)
- using the tools: [15](15-timer.md), [16](16-check-build.md), [63](63-argv.md), [65](65-convert.md), [66](66-native.md)
- games: [17](17-word-guess.md), [19](19-reaction.md), [24](24-quiz.md), [26](26-tic-tac-toe.md), [27](27-mastermind.md), [28](28-ai.md)
- projects: [18](18-adventure.md), [22](22-terminal-menu.md), [23](23-high-score.md), [25](25-calculator.md), [30](30-shop.md), [31](31-bank.md), [32](32-grade-book.md), [33](33-habit.md), [35](35-todo.md), [49](49-report.md), [50](50-editor.md)
- files: [36](36-diary.md), [37](37-files.md), [38](38-name-list.md), [39](39-json.md), [40](40-csv.md), [44](44-log.md), [48](48-files-folder.md)
- working with data: [41](41-address-book.md), [45](45-group.md), [46](46-top-ten.md), [47](47-search.md), [54](54-merge.md), [58](58-data.md), [67](67-stats.md), [68](68-compare.md), [69](69-analysis.md)
- lists and text: [42](42-word-count.md), [43](43-text-stats.md), [51](51-strings.md), [52](52-sorting.md), [53](53-sets.md), [55](55-grid.md), [56](56-bubble.md), [57](57-binary-search.md)
- when a program grows: [59](59-errors.md), [60](60-testing.md), [61](61-modules.md), [62](62-project-files.md), [64](64-python-packages.md)
- the internet: [70](70-patterns.md), [71](71-http.md), [72](72-weather.md), [73](73-poll.md), [74](74-download.md), [75](75-links.md), [76](76-net.md)
- building a language: [77](77-compiler.md), [78](78-expressions.md), [79](79-tokens.md), [80](80-ast.md), [81](81-functions.md), [82](82-bytecode.md), [83](83-bytecode-compiler.md), [84](84-bootstrap.md), [85](85-selfhost.md), [86](86-capstone.md)
- cryptocurrency: [87](87-blockchain.md), [88](88-proof-of-work.md), [89](89-signatures.md), [90](90-consensus.md)

## All guides

| # | Difficulty | Topic | Title | Result |
| --- | --- | --- | --- | --- |
| 01 | ★☆☆☆☆ | first steps | [Hello: say your first words](01-hello.md) | a program that puts what you wrote on the screen |
| 02 | ★☆☆☆☆ | first steps | [Story: writing many lines without repeating yourself](02-story.md) | a program that tells a story of several lines in one block |
| 03 | ★☆☆☆☆ | first steps | [Story: letters arriving one at a time](03-slow-story.md) | a program whose text appears one letter at a time, the way a novel unfolds |
| 04 | ★☆☆☆☆ | first steps | [Ask: have a conversation](04-ask.md) | a program that asks a name and greets it in a sentence |
| 05 | ★☆☆☆☆ | first steps | [Set: store a value](05-set.md) | a program that keeps text, numbers and lists in named values |
| 06 | ★☆☆☆☆ | first steps | [Update: change a value](06-update.md) | a program that adds to and subtracts from a score |
| 07 | ★★☆☆☆ | first steps | [Repeat: do something many times](07-repeat.md) | a program that repeats lines, walks a list, and pauses |
| 08 | ★★☆☆☆ | choosing and repeating | [If: make a choice](08-if.md) | a program that runs different lines depending on a condition |
| 09 | ★★☆☆☆ | choosing and repeating | [And / Or: combine conditions](09-and-or.md) | a program that judges combined conditions |
| 10 | ★★☆☆☆ | choosing and repeating | [While: keep going](10-while.md) | a block that loops while a condition is true |
| 11 | ★★☆☆☆ | choosing and repeating | [Break: stop a loop](11-break.md) | a program that leaves a loop early |
| 12 | ★★☆☆☆ | random and chance | [Random: dice and picks](12-random.md) | a program that rolls a die and picks a color |
| 13 | ★★☆☆☆ | random and chance | [Chance: how often out of a hundred](13-chance.md) | a program in which something happens only as often as you decided |
| 14 | ★★☆☆☆ | screen and time | [Screen: clearing, ruling and boxing](14-screen.md) | a program that clears the screen, centres a title and draws a box |
| 15 | ★★☆☆☆ | using the tools | [Time: a stopwatch and cooldowns](15-timer.md) | a program that times itself and stops an action from repeating too soon |
| 16 | ★★☆☆☆ | using the tools | [Check & Build: see the Python](16-check-build.md) | the habit of reading the Python your sentences become |
| 17 | ★★★☆☆ | games | [Word guess: a hidden-word game](17-word-guess.md) | a game of guessing a hidden word one letter at a time |
| 18 | ★★★★☆ | projects | [Adventure — a small text game](18-adventure.md) | a room-by-room text adventure with choices |
| 19 | ★★★☆☆ | games | [Game: how fast are you?](19-reaction.md) | a program that measures how fast you react |
| 20 | ★★★☆☆ | screen and time | [ASCII art — drawing with characters](20-ascii-art.md) | a program that draws shapes by repeating a character |
| 21 | ★★★☆☆ | screen and time | [Progress — showing how far along you are](21-progress.md) | working through a list of jobs while a bar grows to show the progress |
| 22 | ★★★★☆ | projects | [Terminal menu — a small TUI](22-terminal-menu.md) | a menu program that clears the screen and draws it again |
| 23 | ★★★☆☆ | projects | [High score: a small project](23-high-score.md) | a dice game of three rounds that tells you your best |
| 24 | ★★★★☆ | games | [Quiz — questions from a file](24-quiz.md) | a multiple-choice quiz that loads questions from a JSON file, scores answers, and reports the result |
| 25 | ★★★★☆ | projects | [Calculator — a command-line project](25-calculator.md) | a repeat-until-quit calculator with functions and a module file |
| 26 | ★★★★☆ | games | [Game: tic-tac-toe](26-tic-tac-toe.md) | a playable two-player tic-tac-toe with a win check |
| 27 | ★★★☆☆ | games | [Game: guessing a hidden set of colours](27-mastermind.md) | guessing the three colours the computer hid, in five tries |
| 28 | ★★★★☆ | games | [Game: an opponent that knows the rule](28-ai.md) | a game against an opponent that follows a winning rule |
| 29 | ★★★★☆ | random and chance | [Playlist: a random music player](29-playlist.md) | a playlist loaded from JSON with shuffle, next, and a loop of songs |
| 30 | ★★★★☆ | projects | [Shop — an inventory store](30-shop.md) | a JSON-persisted store with buy/sell/stock/list and a money balance |
| 31 | ★★★★☆ | projects | [Project — a mini bank](31-bank.md) | a JSON-persisted bank account with deposit, withdraw, balance, history, and a storage module |
| 32 | ★★★★☆ | projects | [Project — a grade book](32-grade-book.md) | a JSON-persisted grade book with add-student, add-grade, report-averages, and a storage module |
| 33 | ★★★★☆ | projects | [Project: a habit tracker](33-habit.md) | a JSON-persisted habit tracker with add, check, streak, list, quit, and a module file for the storage logic |
| 34 | ★★★★☆ | screen and time | [Chart: a bar chart in the terminal](34-chart.md) | drawing a horizontal bar chart with # blocks from a JSON list, scaled to the largest value |
| 35 | ★★★★☆ | projects | [Todo: a growing project](35-todo.md) | a JSON-persisted todo list with add, done, list, and a module file for the storage logic |
| 36 | ★★★★☆ | files | [Diary: notes saved by date](36-diary.md) | a diary that saves each day's note to a dated file and can read it back |
| 37 | ★★★☆☆ | files | [Files: saving and reading again](37-files.md) | a program that writes text to a file and reads it back |
| 38 | ★★★☆☆ | files | [Name list: reading a file line by line](38-name-list.md) | a program that saves names one per line and reads them back as a list |
| 39 | ★★★☆☆ | files | [JSON: save and load data](39-json.md) | a program that saves a name and a score and loads them back |
| 40 | ★★★☆☆ | files | [CSV: several fields on one line](40-csv.md) | a program that reads a file of comma-separated fields |
| 41 | ★★★★☆ | working with data | [Records: a small address book](41-address-book.md) | a JSON-file address book that adds, lists, and searches contacts |
| 42 | ★★★★☆ | lists and text | [Word count: how often each word appears](42-word-count.md) | reading a text file and counting how often each word appears, using a dict and collections.Counter |
| 43 | ★★★★☆ | lists and text | [Text stats: letters and words](43-text-stats.md) | reading a text file and reporting character count, word count, longest word, and most common word (with collections.Counter) |
| 44 | ★★★★★ | files | [Log: an event record](44-log.md) | appending a dated line to a log file each time the program runs, using datetime and file_write |
| 45 | ★★★★★ | working with data | [Group: data by category](45-group.md) | grouping a list of dicts by a category key into a dict of lists, then reporting counts per category |
| 46 | ★★★★★ | working with data | [Top ten: ranking records](46-top-ten.md) | loading JSON records, sorting by a numeric score with sorted(..., key=...), and showing the top ten |
| 47 | ★★★★★ | working with data | [Search — finding items in JSON](47-search.md) | loading a JSON catalog from a local server or file and searching it by keyword, case-insensitively |
| 48 | ★★★★★ | files | [Files: processing many files](48-files-folder.md) | listing the files in a folder with os.listdir, reading each with file_read, and reporting total words and letters across all of them |
| 49 | ★★★★★ | projects | [Report — writing a summary file](49-report.md) | reading a few JSON data files and writing one text report with file_write |
| 50 | ★★★★★ | projects | [Editor — a tiny text editor](50-editor.md) | a line-based editor with a buffer and add, list, remove, save, and quit commands |
| 51 | ★★★★☆ | lists and text | [Strings: slicing and changing text](51-strings.md) | slicing `text[start:end]`, `.upper()/.lower()`, `.replace()`, and `.strip()` on a sentence |
| 52 | ★★★☆☆ | lists and text | [Sorting — putting a list in order](52-sorting.md) | standing a list up in both directions, smallest first and biggest first |
| 53 | ★★★★★ | lists and text | [Sets: unique values](53-sets.md) | using a Python set to find unique words in a text file, then unique letters in a sentence |
| 54 | ★★★★★ | working with data | [Merge: joining two lists](54-merge.md) | loading two JSON lists and joining records by name key into one report |
| 55 | ★★★★★ | lists and text | [Grid: a board of lists](55-grid.md) | a tic-tac-toe style 3x3 board stored as a list of lists, reading and writing cells with `board[row][col]` |
| 56 | ★★★★★ | lists and text | [Bubble sort — your first algorithm](56-bubble.md) | implementing bubble sort by hand with nested loops and a swap, then comparing the result with Python's built-in sort |
| 57 | ★★★★★ | lists and text | [Binary search — halving the guess](57-binary-search.md) | finding a number in a sorted list by halving the search range each step, showing the step count and the found index |
| 58 | ★★★★★ | working with data | [Data: statistics on a list](58-data.md) | loading numbers from a JSON file and computing mean/median/max with the statistics standard library |
| 59 | ★★★★☆ | when a program grows | [Errors: handling problems](59-errors.md) | a program that reads a missing file and converts bad input without crashing |
| 60 | ★★★★☆ | when a program grows | [Testing: checking the functions you wrote](60-testing.md) | a tiny test runner that calls your own functions, compares each result with the expected value, and reports pass or fail |
| 61 | ★★★★☆ | when a program grows | [Modules: split your program into files](61-modules.md) | splitting a program across .nme files |
| 62 | ★★★★☆ | when a program grows | [Modules: a project across files](62-project-files.md) | a small weather-report project split into three .nme modules (fetch, analyze, report) with clear interfaces, imported by a main program |
| 63 | ★★★★☆ | using the tools | [Tools: reading command-line arguments](63-argv.md) | a todo tool that takes commands like `nme r todo add "buy milk"` on the command line |
| 64 | ★★★★☆ | when a program grows | [Python packages: the standard library and installed libraries](64-python-packages.md) | using the standard library and installed libraries |
| 65 | ★★★★☆ | using the tools | [Convert: turn Python into NME](65-convert.md) | a small Python file converted into NME |
| 66 | ★★★★☆ | using the tools | [Native: compile to machine code](66-native.md) | running a program as machine code without CPython |
| 67 | ★★★★★ | working with data | [Stats: understanding data](67-stats.md) | loading a JSON list and reporting count, mean, median, mode, min, max, and range |
| 68 | ★★★★★ | working with data | [Compare: two groups of numbers](68-compare.md) | loading two JSON number lists and comparing their means and maxima |
| 69 | ★★★★★ | working with data | [Data: analyzing a month of temperatures](69-analysis.md) | loading a month of temperatures, computing statistics and a histogram, and saving a report file |
| 70 | ★★★★★ | the internet | [Patterns: finding matches with regex](70-patterns.md) | a program that finds phone numbers and email addresses in a text file |
| 71 | ★★★★★ | the internet | [HTTP — asking a web server](71-http.md) | fetching a page from a local server |
| 72 | ★★★★★ | the internet | [Weather: reading JSON from the web](72-weather.md) | fetching a local HTTP server's JSON and printing a mini weather report |
| 73 | ★★★★★ | the internet | [Network: polling a server](73-poll.md) | repeatedly fetching a status.json from a local server every few seconds and reporting changes |
| 74 | ★★★★★ | the internet | [Network: downloading files](74-download.md) | downloading a file from a local HTTP server and saving it while showing progress |
| 75 | ★★★★★ | the internet | [Web: extracting links from a page](75-links.md) | fetching an HTML page from a local server and listing every link on it as a full URL |
| 76 | ★★★★★ | the internet | [Network: a mini chat](76-net.md) | fetching messages from a local HTTP server and showing them like a mini chat |
| 77 | ★★★★☆ | building a language | [Your first compiler — a tiny language](77-compiler.md) | a tiny language that reads lines like `add 2 3` and prints the answer |
| 78 | ★★★★☆ | building a language | [Compiler tier: a small expression language](78-expressions.md) | a tiny calculator that evaluates 2 + 3 * 4 respecting precedence, as a step toward a real expression compiler |
| 79 | ★★★★☆ | building a language | [Compiler tier: reading tokens](79-tokens.md) | splitting a command line into tokens and dispatching them, a step toward a real tokenizer and parser |
| 80 | ★★★★★ | building a language | [Compiler tier: expressions as trees](80-ast.md) | a calculator that parses an expression into a tree and evaluates the tree recursively, respecting precedence |
| 81 | ★★★★★ | building a language | [Compiler tier: functions in the mini language](81-functions.md) | a compiler that translates a mini language with `def`, `return`, and calls into Python and runs it |
| 82 | ★★★★★ | building a language | [Compiler tier: a tiny bytecode runner](82-bytecode.md) | compiling simple instructions into a list of steps and running them one by one like a tiny virtual machine |
| 83 | ★★★★★ | building a language | [Compiler tier: from tree to bytecode](83-bytecode-compiler.md) | a compiler that flattens an expression tree into instruction lines and runs them on a stack machine |
| 84 | ★★★★★ | building a language | [Bootstrap: NME compiling a tiny language](84-bootstrap.md) | a tiny compiler written in NME |
| 85 | ★★★★★ | building a language | [Self-host: NME running NME](85-selfhost.md) | an NME program that compiles a tiny NME-like subset (say/set/while) to Python |
| 86 | ★★★★★ | building a language | [Capstone: a language that compiles to Python](86-capstone.md) | an NME program that reads a small custom language (say/set/add/while/end), compiles it to Python source, writes it to a file, and runs it |
| 87 | ★★★☆☆ | cryptocurrency | [Cryptocurrency ledger — state and transactions](87-blockchain.md) | a small cryptocurrency ledger with balances, fees, supply, and transaction nonces |
| 88 | ★★★☆☆ | cryptocurrency | [Proof of work — mining transaction blocks](88-proof-of-work.md) | link transactions to the previous block and perform SHA-256 proof of work |
| 89 | ★★★☆☆ | cryptocurrency | [Transaction proofs — wallet ownership and replay prevention](89-signatures.md) | validate transaction authorization with a public value and reject replay |
| 90 | ★★★☆☆ | cryptocurrency | [Full-chain validation — recompute from genesis](90-consensus.md) | a single-node validator that replays every transaction and block from genesis |

## Where to continue

- [Getting started](../getting-started.md): the five-minute path from hello to a number game
- [Tutorial](../tutorial.md): seven projects from Hello World to a compiler
- [Language reference](../language.md): exact rules for all three levels
- [Syntax list](../syntax.md): every accepted spelling, in one table
- [Prompts to hand to an AI](../prompts/README.md): paste one into a chat and the AI can write NME
- If installing is hard, write and run programs at **needmoreeasy.com** — it works on a phone.
