# Changelog

English | [한국어](CHANGELOG.ko.md)

All notable changes to NME are recorded here.

## Unreleased
- **The corpus of "ways a beginner writes the same thing" is more than twice
  the size.** It covered twelve intentions and now covers twenty-six — value
  changes, waiting, taking one thing back out of a list, writing into a
  record, looping over a list, `else`, and `while`, in both languages. That is
  211 spellings measured instead of 103, and it is what the entries below were
  found by.
- **Putting something into a name that is not a list is named, not left to run
  time.** `점수는 0` followed by `점수에 1 추가해` compiled to `점수.append(1)`,
  which reads perfectly and dies with `AttributeError`. The new E0240 says
  which name and what is in it.
- **`친구들에서 민수를 빼줘` takes out `민수`, not `민수를`.** The object mark
  was being kept as part of the word, so the item taken out was one nobody put
  in.
- **Four more ways to take one thing out of a list.** `delete`, `erase` and
  `drop` in English; `삭제해`, `제거해`, `지워` and `없애` in Korean. They are
  everyday verbs, so they only make a statement when the name is one the
  program already made a list: `delete the file` and `기록을 지워 주세요` stay
  sentences.
- **Four more ways to write one named value into a record.**
  `store Mina as 90 in ages`, `save Mina as 90 in ages`,
  `record Mina as 90 in ages` and `add Mina at 90 to ages`, plus Korean
  `나이표에 민수를 90으로 저장해`, `기억해`, `적어` and `넣기`. `set X to Y in R`
  is deliberately left alone: it already means *read `Y` out of `R`*.
- **Six more ways to wait.** `hold`, `delay` and `rest` in English; `대기해`
  and `대기` in Korean.
- **Fifteen Python-level error explanations rewritten in plain words**
  (E0106–E0120), and the two places NME called itself `I` now say `NME`.
- **Twenty-five more ways of writing the same thing now do it.** Over the
  211-variant corpus, 163 worked when it was first measured and 203 do now,
  with nothing printing itself instead of running. Among them: `for name in
  names` and `for each name of names`; `이름들의 이름마다` with the lines to
  repeat written underneath, and `이름들의 각 이름마다`; `otherwise`, `or else`,
  `그렇지 않으면`, `안 그러면`, `그 외에는`, `아니라면`; `as long as n < 3`,
  `repeat while n < 3`, `keep going while n < 3` and `수가 3보다 작은동안` with
  the space left out; `민수를 90으로 나이표에 넣어` and `나이표에 민수 90 넣어`;
  `in ages put Mina at 90` and `put Mina 90 in ages`; `민수를 친구들에서 빼`;
  `take Mina out of friends`; and `grow`/`bump`/`up score by 1`.
- **`점수에 1 추가해` adds one to the score.** The word puts an item into a
  list, and the name says which is meant: a list takes an item, a number takes
  arithmetic. Something the program never made is refused rather than guessed.
- **Eight spellings are deliberately still refused**, each for a reason
  written down beside the corpus: `name is 5` is a comparison in Python,
  `set Mina to 90 in ages` already means reading a value out, and `한` and
  `계속` are too common in Korean to claim.
- **Narrower carets.** A value change that could not be read underlines the
  amount rather than the whole line; a list line underlines what was to go in;
  a loop over a list underlines the part that could not be read, and says when
  the missing word is `in`.

## 0.3.0
- **Five more ways of writing the same thing now do it.** Measured against the
  103-variant corpus, 96 of them worked; 102 do now. `5를 이름에 저장해` says
  the value first, `이름을 5로 두어` uses the everyday verb for putting
  something down, `친구들에 민수 넣기` uses the naming form of the verb,
  `do it 3 times` and `그거 3번 반복해` say out loud what is being repeated,
  `수가 3보다 클 때` closes the condition with `때`, and English `ask for name`
  asks *for* something. `name is 5` is still refused: in Python that line is a
  comparison, and reading it as a save would change what valid Python means.
- **`사진을 폴더에 저장해` no longer becomes `사진 = "폴더에"`.** Korean marks
  where something goes with `에`, so that is the name being saved into — and a
  line whose value is not a value is a sentence about saving, which prints
  itself.

## 0.2.0
- **Errors say what is wrong, where, and why.** Twenty of the messages a
  beginner meets most often were rewritten. `I don't know what \`echo\` does`
  now says that the word is standing where the action word goes, and the hint
  hands back the reader's own line with the word put right — but only when
  that line really compiles, so following the hint cannot fail again. An
  unclosed block points at the line that opened it instead of at the end of
  the file; an unclosed quotation mark points at the mark that was opened, not
  one column past the end of the line; and the lines that could not be read
  quote the part they could not read.
- **Eight programs that used to compile into something else now say so.**
  `set x.count to 3` saved the words `.count to 3` into `x`; `show a; show b`
  became one message reading `a; show b`; `item -1 of friends` and `wait -3
  seconds` went through to a run-time failure; `put Mina in ages` on a record
  printed itself; loading the same module twice was reported as ten names
  colliding. Each of them is now refused with the line named.
- **A semicolon is punctuation in writing.** `It was late; nobody spoke` was
  handed to Python, which answered with a syntax error, and the Korean
  `그는 웃었다; 나는 울었다` compiled into an assignment that meant something
  else. Both now print themselves.
- **`do` opens a job as well as a repeat.** `do greet with Mina` was answered
  with "the repeat count is missing" on a line that repeats nothing. Running a
  job with the wrong number of things is now named as that, and `do the
  washing up` prints itself.
- **Five file and module problems have codes of their own** — `E0238`,
  `E0239`, `E0407`, `E0408` — instead of borrowing the code for a line with no
  action on it, plus `E0409` for a module loaded twice. Looking a code up no
  longer explains something else.
- **Each language keeps its own examples.** A hint written in English no
  longer offers a Korean line to copy, or the other way round, and a hint that
  names your own name writes it the way your program writes it. Korean
  particles now follow the name they attach to, in tidied programs as well as
  in messages: `score는`, `name을`, `table은`.
- **CPython's own error is said in plain words first.** A build that fails
  CPython's check now opens with the line number and what is wrong with it, in
  both languages, and keeps the original report underneath as evidence.
- **A reading may stand inside a sentence.** `how many`, `the total of`, `the
  biggest of` and their Korean twins only worked as the whole of a line, so
  `show You carry how many bag things` printed `You carry how many ['a', 'b']
  things` — the list itself, dropped into the middle of the words. It now
  reads `You carry 2 things`. The words the writer typed are kept, so tidying
  a program into the other language leaves the sentence saying what it said.

## 0.1.0
- **A word English already has is never read as a misspelling.** NME repairs
  one typo, and one typo is all that separates most short words: `shop milk`
  printed `milk`, `well done` printed `done`, `bell rings` printed `rings`.
  The compiler now carries the 3,030 real English words that sit one typo away
  from a word of its own, generated from the system spelling dictionary by
  `scripts/build-common-english-words.py` and checked on every build. Repair
  still catches what it is for — `shwo`, `tel`, `sya` and `pirnt` are not
  English words.
- **A sentence that ends in an output word keeps its verb.** English tolerates
  the message-first order (`Hello world show`), and read without care it
  claimed the last word of every sentence that ends in one: `time will tell`
  printed `time will`, `what did she say` printed `what did she`, and `I have
  nothing to say` lost its verb. A subject, a modal, `to` or a conjunction in
  front of the word settles which one it is.
- **A line that opens with `in`, `is`, `and`, `or` or `as` is a sentence.** It
  used to be handed to Python, so `in the beginning there was light` came back
  as CPython's `SyntaxError` with a caret under the first two letters. None of
  those five can begin a Python statement, so a line that starts with one is
  not Python that went wrong.
- **`사과` is not `사` and `과`.** Korean picks its joining particle from the
  sound before it — `과` after a consonant, `와` after a vowel — and reading
  every trailing `과` as *and* cut the word for apple in half: `가방은 목록
  사과` built a bag holding `사`. `감과 배` is still two items.
- **`점수에 1` counts, and no longer wipes the score.** `에` says *into* and
  `에서` says *out of*, so everyday Korean drops the verb when the marks
  already carry the direction. It was read as a save — `점수 = 1` — and the
  count being kept was gone with nothing on screen to show it.
- **A number added to a list goes into it.** `add 1 to bag` compiled to `bag =
  bag + 1`, which is a `TypeError` in every Python there has ever been. Two
  lists still join, because that is what Python does and what the words say.
- **`score+1` is refused.** Python works out the answer and drops it, so the
  program ran, printed nothing and left `score` exactly as it was — the one
  shape where doing nothing is indistinguishable from working. `E0604` now
  says so and points at `add 1 to score`.
- **A job can count something made outside it.** Python decides for a whole
  function at once whether a name is local, so `to tally: add 1 to total end`
  built a function that read `total` before it had one and died with
  `UnboundLocalError` — a word the reader has never met, on a line that looks
  right. The declaration now rides on the same physical line as the change,
  because one NME statement is one line of Python and a line of its own would
  move every traceback number after it. Reading the name before changing it
  cannot be written at all, and `E0236` says why.
- **`save entry to the file "diary.txt"` writes the file.** `save` is both
  NME's saving word and the everyday English for writing a file, and the two
  collided in silence: the diary was never written and `entry` quietly became
  the text `diary.txt`. Saying `file` settles it, exactly as `파일에` does in
  Korean. The bare `save entry to "diary.txt"` still saves the file name into
  the name, which is a real thing to do.
- **The joining words between a header and the line under it are not the
  message.** `repeat 3 times after that show Again` printed `after that show
  Again`, `3번 반복: 다시 말해줘` printed `: 다시`, and `만약에 점수가 1보다
  크면, 좋아 말해줘` printed `, 좋아`. Nothing is dropped unless what is left
  still reads as a command, so `repeat 3 times next week` keeps saying it.
- **A Korean loop written on one line loops.** `점수가 5보다 작은 동안 안녕
  말해줘` was claimed by the condition matcher, which then failed on a
  condition ending in `작은`; with `동안에` it came out as an `if` whose message
  began with the loop word. `동안` after the verb is still part of what gets
  said (`커서가 깜빡이는 동안 말해줘`).
- **A name written as two words is named.** `set full name to Mina` made a name
  called `full` holding the words `name to Mina`, and printing it showed all of
  them. The connector standing further along is the evidence, so `E0230` now
  fires and offers `full_name`. `set greeting Hello world`, which has no
  connector anywhere, still saves the greeting.
- **A name the language itself needs is refused (`E0237`).** `the total of
  marks` is Python's `sum(...)` and `how many friends` is `len(...)`, so `set
  sum to 3` took the reading away from every later line — and the error landed
  on one of those lines as `'int' object is not callable`. Python written as
  Python is left alone: whoever writes `sum = 0` in Python has said what they
  mean.
- **`이번` is a word, not the number two and a counter.** `이번 달 예산 말해줘`
  looped twice and printed `달 예산`, the writer's first word gone. `두번` and
  `3번` are still counts.
- **The remainder can be taken by a name.** `총점을 인원으로 나눈 나머지` came
  out as writing: only a number was read, because a number has its particle cut
  off where a name keeps it attached. `docs/syntax.ko.md` §15 had promised
  both.
- **A name cannot take a loaded module's word away.** `use date` and then `set
  today to Monday` replaced the module's `today`, and `show today()` died with
  `'str' object is not callable`. The other order was refused from the start;
  this is that rule from the other side. A question that happens to end on one
  of those words is still a question — `use date` may not change what a line
  means.
- **`show Today is today()` is a sentence.** It is valid Python — `is`
  compares — so it came out as one and printed the words back unevaluated after
  dying on `Today`, a name nothing ever made. `is` is an English verb far more
  often than it is Python's identity test.
- **More ways to say the same thing.** `rep 3 times`, `go round 2 times`, `run
  through 2 times`; `should n be greater than 3`, `whenever`, `incase`; and
  Korean `친구들마다 반복해`, which loops over `친구들` with `친구` as the name
  for each one.
- **A date module.** One line — `use date latest` / `날짜 사용 최신` — brings
  `today()`, `now()`, `year()`, `month()`, `day_of_month()`, `weekday()` and
  `days_after(7)`. It is the seventh bundled module, and it replaces the
  `from datetime import date` that the diary, log, habit and report guides all
  used to open with. **It runs on the site too** — only the `monotonic` family
  of clocks kills the browser engine, and `datetime` was checked in the shipped
  one. That clock is UTC.
- **The weekday name is the one thing the two languages answer differently.**
  `weekday()` gives `Wednesday` and `요일()` gives `수요일`. Every other helper
  binds one value to both names; a weekday **is a word**, so it has to be in
  some language. It is the only place in any bundled module where the name you
  write chooses the language of the answer.
- **The bare names `date` and `날짜` are not bound.** They are what people call
  their own values, and binding them would make the module collide with exactly
  the programs that want it. `날짜는 5` and `날짜 사용 최신` live together.
- **`No date has been set for the repairs.` prints again.** A module name that
  is also an ordinary word — `date`, `list` — was counted **anywhere on the
  line** when recovering a mistyped module line, so that sentence was refused
  with `E0305`. It now counts only beside the `use`/`사용` word. The defect
  predates this module; `No list has been set.` had it too.
- **A job's body may be indented.** Indenting under `to greet:` and closing with
  `end` is the shape every guide in this repository teaches, and it was the one
  shape refused — with `E0101` pointing at a header that was already correct.
  A job now closes the way `repeat 3 times` and `story:` do: an indented body
  ends where its indentation ends, a flat one waits for `end`.
- **A Korean amount no longer keeps its particle.** `결과를 둘째수로 나눠`
  produced `결과 = 결과 / 둘째수로` and died at run time with `NameError`.
  `둘째수로` is a valid Python expression too — a name nothing ever set — so
  "does Python read it" cannot tell the two apart. Which one the program
  actually made is asked first now. A written number (`2로 나눠`) is unchanged.
- **A list line written on a record is refused.** `표에 사과 넣어` and
  `append apple to ages` compiled to `표.append("사과")` and died at run time
  with `AttributeError: 'dict' object has no attribute 'append'` on a line
  that reads perfectly. The opposite direction was refused from the start
  (`E0234`); this one was not. It now says to write the name to put it under.
- **A record — one name holding many named values.** `set ages to an empty
  record` / `나이표는 빈 표`, then `put Mina at 90 in ages` /
  `나이표에 민수를 90으로 넣어`, `show Mina in ages` / `나이표의 민수 말해줘`,
  and `remove Mina from ages` / `나이표에서 민수 빼`, which is Python's `del`
  because a dictionary has no `.remove`. Python calls it a dictionary; the
  twenty guides of Part 2 use one eighty-seven times, and there was no way to
  write one at all.
- **The list spellings mean the record thing when the name holds a record.**
  `how many` / `개수`, `contains` / `…에 …가 있으면`, `for each` /
  `…마다 반복해` and `remove` / `빼` are **the same words for both**, and the
  compiler decides from the kind the name holds, never from the wording. A
  reader should not have to remember which spelling belongs to which
  container, and that was the whole argument for building it this way. Looping
  over a record hands back its names, exactly as Python does, so
  `show name in ages` reads the value back inside the loop.
- **A record line written on a list is refused (`E0234`).** Left alone,
  `친구들에 민수를 90으로 넣어` appended the whole of `민수를 90` to the list as
  one piece of text — a program that runs, looks right, and is not what anybody
  wrote. `the total of`, `the biggest of`, `the first of` and `sort` stay
  list-only: a record has no order and nothing to add up, and refusing says so.
- **A record is only a record where a value is being saved.** Everywhere else
  `record`, `table` and `표` are words somebody wrote, so `I keep a record of
  everything I read` and `표는 두 장 남았습니다` still print. A name ordinary
  Python wrote as `ages = {}` is a record to every one of these statements; a
  set literal such as `{1, 2}` is not.
- **A named job — a piece of program with a name.** `to greet:` /
  `인사하기라는 일:`, a block, `end` / `끝`, and then `do greet` /
  `인사하기 해줘` runs it. It becomes an ordinary `def`. `to`, `do`, `일`,
  `하기` and `해줘` are among the most ordinary words either language has, so a
  job is recognized by **structure and never by a word**: the opening `to`, or
  the `라는` on the name and the `일` after it, **and** the closing colon,
  **and** a block underneath. Without the block there is no job, so `To do:`,
  `오늘의 할 일:` and `to be honest` print as the lines they are. There is no
  one-line form either, so a colon in the middle of a sentence cannot open one.
- **The line that runs a job is gated on the job existing.** `do` and `해줘`
  decide nothing on their own; the name has to be one this program already
  made. A Python `def` that takes no arguments counts, so the three levels mix.
  A name saved inside a job stays inside it and a Python `return` written in
  there is accepted, because the body is a real function scope.
- **A job may be given one thing.** `to greet someone:` /
  `이름에게 인사하기라는 일:`, and then `do greet with Mina` /
  `민수에게 인사하기 해줘`. How many things a job takes is remembered with its
  name, so running it the other way round is refused with `E0235` instead of
  becoming a Python `TypeError` at run time on a line that looks right.
- **Not yet: a job that takes two things, and a job that hands something
  back.** Write a Python `def` for either; advanced NME is ordinary Python.
- **A repaired list word no longer invents the container as well.**
  `너에게 하고 싶은 말이 있어` became `너.append("하고 싶은 말이")`, because
  `있어` is one character from `넣어`; so did `창고에 상자를 세로로 두어`. A
  guess at the verb may not also invent the name it puts something in, so a
  repaired list word now needs a name the program already made. The exact
  spellings are untouched.
- **`그릇에 설탕을 한 스푼으로 넣어` prints again.** A Korean line that marks a
  name *and* a value is the record shape, and when the container is something
  the program never made, the list statement used to take the whole of
  `설탕을 한 스푼으로` and append it as one piece of text. Somebody cooking now
  keeps their sentence.
- **Text can be cut into a list.** `set names to memo split by line` /
  `이름들은 메모를 줄마다 나눈 것`, and the same with `by comma`, `by space` and
  `by newline` (`쉼표로`, `빈칸으로`, `줄바꿈으로`). `join` landed in round 2 and
  its opposite did not, which is where the whole of Part 3 stopped: reading a
  file gives you one piece of text, and every next step needs a list. `by line`
  is Python's `splitlines()`. **A comma splits on `","` and joins with `", "`**,
  and that difference is deliberate — a line read back out of a file says
  `Mina,Ada`, and looking for `", "` there would find nothing. What a split
  saves is a list, so `how many names` works on it at once. It is gated on a
  name the program has already saved, so `이야기를 둘로 나눈 것이 좋겠습니다` and
  `The vote split by party.` stay sentences.
- **Joining with nothing between the items.** `show friends joined together`,
  `friends joined by nothing`, `친구들을 붙여`, `친구들을 그대로 이어`. Drawing a
  row of stars needed it and there was no way to say it.
- **A join that does not say what goes between is refused (`E0233`).**
  `show stars joined` used to print `stars joined` back at you and
  `별들을 이어 말해줘` printed `별들을 이어`, both of which look like success. The
  refusal is gated on the list name and on nothing else being left on the line,
  so `friends join us at noon` and `별들을 이어 갔습니다` still print.
  `friends joined with comma` also works now; `with` is a Python keyword, so it
  had never reached the separator it named, although the reference said it did.
- **One piece of text, that many times over.** `set bar to star repeated 5
  times` / `막대는 별표를 5개 붙인 것`, which is what draws ASCII art, a progress
  bar and a chart. An earlier round left this out because `별표를 5번 이어` cannot
  be told from the counted loop, where `5번` already means *five times*. It is
  safe now because it is a **noun phrase and not a command**: a name the program
  saved, then a count, then a counting word, then `붙인 것`, which no loop ever
  says. So `5번` may be written here after all. The text is wrapped in
  `str(...)` first, so a name holding `3` gives `"33333"` and not `15`.
- **A list loop that knows which turn it is on.** `for each friend in friends
  with place` / `친구들의 친구마다 순서와 함께 반복해` becomes
  `for place, friend in enumerate(friends, 1):`. Counting starts at **one**,
  because `친구들 3번째` already means the third. The name is the writer's to
  choose — the word after `with`, and the word before `와 함께`. Comparing two
  lists position by position is what mastermind and tic-tac-toe are made of,
  and there was no way to hold the position at all.
- **Three more bundled modules: `use list`, `use text` and `use math`.** One
  line each, and both languages of every name are ready at once —
  `말해 개수(친구들)` and `say count(friends)` are the same program. `list`
  binds `count`/`개수`, `sort`/`정렬`, `reverse`/`뒤집기`, `remove`/`빼기`,
  `first`/`첫번째`, `last`/`마지막`, `sum`/`합계`, `largest`/`최대`,
  `smallest`/`최소`; `text` binds `upper`/`대문자`, `lower`/`소문자`,
  `trim`/`공백없애기`, `split`/`나누기`, `join`/`합치기`, `replace`/`바꾸기`,
  `starts_with`/`로시작`, `length`/`길이`; `math` binds `root`/`제곱근`,
  `round_to`/`반올림`, `pi`/`원주율`, `power`/`거듭제곱`, `absolute`/`절댓값`,
  `floor`/`내림`, `ceil`/`올림`. Each has the same version story as `use
  random`: `use list latest`, `목록 사용 버전 "0.0.1"`. Everything inside them
  is a plain Python builtin or one call into `math`, so a program that uses
  them runs in the browser unchanged.
- **`list`, `text` and `math` still print when they are just words.** They are
  the first module names that are also ordinary words, in both languages, so
  they name a module only when they stand directly beside `use` / `사용` and no
  other word is left over on the line. `get the list of names`,
  `I use text messages every day` and `장 볼 목록을 사용해 보세요` are
  sentences, and they print. They are also never repaired from a typo, because
  `list` is one letter from `last` and `math` one from `path`.
- **A name a bundled module bound is never shown inside a sentence.** A
  sentence shows the value of a name the *program* made — that is what the text
  form is for — but the writer never wrote `floor` or `길이`, and mostly does
  not know they exist. After `use math`, `the floor is cold` printed
  `the <built-in function floor> is cold`; after `글자 사용`,
  `길이가 조금 짧습니다` showed a function where the length should have been.
  This was true of `use random` too — `shuffle the cards` — and is fixed for
  every bundled module at once. The names still work as values: `show pi`,
  `말해줘 원주율` and `say count(friends)` are unchanged.
- **`sort`, `reverse` and `remove` from `use list` hand back a new list.** The
  sentence statements `sort friends` / `친구들 정렬해` still change the list
  itself. Both exist because both are things people mean, and a helper that
  answered `None` while quietly reordering your names would be the worse
  surprise.
- **Ordinary Korean sentences print themselves far more often.** Measured over
  353 sentences a person really types — a line of a story, a note to self, a
  message in a game — 294 printed themselves character for character and 38
  compiled into a different program. It is now 350 and 3. The measurement is
  `scripts/mistake-probes/korean_prose.py` and it is a checked number.
- **A polite `… 주세요` request is a sentence again.** All twenty-three of them
  were wrong: `물 좀 주세요` printed `물`, `설탕을 조금만 넣어 주세요` printed
  `설탕을 조금만 넣어`, and `물어봐 주셔서 감사합니다` stopped the program at a
  question nobody wrote. `주세요` attaches to any Korean verb, so it is no
  longer repaired into an output word, and it is glued to the word before it
  only when that word is already an action word: `말해 주세요`,
  `보여 주세요` and `2초 기다려 주세요` are unchanged.
- **A line that ends the way a written Korean sentence ends keeps its own
  meaning.** `이유는 저도 잘 모릅니다` became `이유 = "저도 잘 모릅니다"`, a
  program that runs and says nothing; `3층에서 내리면 됩니다` became a
  comparison cut inside a word; `1번 출구에서 만납시다` became a loop; and
  `카드를 잘 섞어 나눠 주세요` became a division by a name nothing ever set.
  The list of endings that close a Korean sentence now covers the `-ㅂ니다`
  family (`모릅니다`, `나옵니다`), `-ㅂ시다`, `-니까` and the `-요` endings, and
  every shape of assignment, loop, comparison and value change asks it — not
  only the one after a `은`/`는`. `정답은 7입니다` still saves the number seven
  and `인사는 안녕하세요` still saves the greeting.
- **A helper verb after an action word is part of that verb.**
  `말해 봐야 소용없는 일이었습니다` printed only its second half and
  `저장해 둔 사진을 다시 봤습니다` made a value called `둔`. `말해줘 비가
  쏟아졌습니다` still prints the rain.
- **A helper verb inside a question belongs to the question.** The rule above
  reached one line too far. `주문을 물어봐 마법의 주문을 말해 보세요` names what
  it is asking for before it asks, so everything after `물어봐` is the text
  shown while it waits — `말해 보세요` included. Read as a sentence, the line
  printed itself instead of asking, and the loop written around it in the
  site's `ko/password` example never got an answer and never ended.
  `물어봐 주셔서 감사합니다` is still a thank-you: no name stands in front of
  the asking word there, so the helper verb is the line's own.
- **A Korean sentence with a hyphen, a slash, a wave dash or a bracket in it
  prints.** `K-POP을 좋아합니다`, `A/S 센터에 맡겼습니다`,
  `오전 9시~오후 6시까지 문을 엽니다` and `(괄호 안은 나중에 지우겠습니다)` were
  handed to Python, which answered a Korean sentence with an English
  `SyntaxError` whose caret landed inside a Hangul syllable. Valid Python still
  wins; this is asked only of a line Python has already refused.
- **A program written in Korean is explained in Korean.** Error messages
  followed the spelling of the command, so `nme build` on a Hangul file
  answered in English only. The Korean half is now printed whenever the program
  itself holds Korean, and the English half stays.
- **Ordinary English sentences print themselves far more often.** Measured over
  302 sentences a person really types — a line of a story, a note to somebody, a
  message in a game — 184 printed themselves word for word and 44 compiled into
  a different program. It is now 249 and 12. The measurement is
  `scripts/mistake-probes/english_prose.py` and it is a checked number.
- **A sentence is no longer read as a command because one of its words is one
  letter from one.** `Today is a good day` lost its last word to `say`,
  `Clear a path through the snow.` lost `snow` to `show`, and `end of the road`
  was told to choose a module version because `road` is one letter from `load`.
  A repaired English output word now claims a single word of message, which is
  what a real misspelling looks like (`shwo hello`, `hello sya`); a module line
  must name a module; and a word NME already refuses and explains, such as
  `let`, is never repaired into the action it is refused in favour of.
- **A number in a sentence no longer switches the sentence off.** `The soup
  needs cream.` printed and `The soup needs 250 ml of cream.` did not. Prices,
  ages, times, dates, room and chapter numbers now stay in the sentence they
  were written in. A number written beside a command word is still a command,
  so `wait 3 seconds` and `set score to 0` are unchanged.
- **A line holding one word prints it.** `Hello` on its own was left as a bare
  Python name, and the program said nothing and then died with a `NameError`
  pointing at a line that is not the mistake. A name the program set earlier,
  and a word NME spells out itself (`say`, `end`, `skip`, `목록`), keep their
  Python meaning.
- **A command word at the start of a line does not make the rest of the line its
  argument.** `set the table for four people` made a value called `the` and
  printed nothing at all; `ask me anything you like` stopped the program at a
  question nobody wrote; `List the ingredients on the back.` was wrapped in
  Python list brackets. All three are sentences and print. `set then to 1`,
  `ask your "hi"` and `list of Mina, Ada` still mean what they say, because a
  `to`, a pair of quotes, or a comma says a name or a list was meant.
- **A random pick needs its choices marked off from each other.**
  `마음에 드는 것을 골라 보세요` printed one word of itself at random, a different
  one every run, and nothing in the line ever named a choice; so did
  `여러 개 중에서 뽑아` and `pick a flower from the garden`. A pick now needs
  `또는`/`or` or a comma between the choices. The documented spellings
  `색은 빨강 또는 초록 중에서 골라` and `set color to pick from red or green` are
  unchanged.
- **A word is only taken apart into words.** `doctor` was read as `do ctor`,
  `finished` as `finish ed`, `friend` as `fri end` and `telling` as `tell ing`,
  so `story of a small town doctor` was refused with a suggestion nobody could
  act on. A space really left out is still named: `sayhello` and `안녕말해줘`.
- **Lists can be built and read entirely in sentences.** An empty list
  (`set friends to an empty list` / `친구들은 빈 목록`), how many items it holds
  (`show how many friends` / `친구들 개수 말해줘`), one item by its position
  (`the first of friends`, `the last of friends`, `item 3 of friends` /
  `친구들 첫 번째`, `친구들 마지막`, `친구들 3번째`), the total, the biggest and
  the smallest (`the total of scores` / `점수들 합`), putting the list in order
  (`sort friends`, `reverse friends`, `shuffle friends` / `친구들 정렬해`,
  `친구들 거꾸로 해`, `친구들 섞어`), taking an item back out
  (`remove Mina from friends` / `친구들에서 민수 빼`), joining every item into
  one piece of text (`show friends joined by comma` /
  `친구들을 쉼표로 이어 말해줘`), and asking whether a list holds something or
  holds nothing (`if friends contains Mina`, `if friends is empty` /
  `만약에 친구들에 민수가 있으면`, `만약에 친구들이 비었으면`).
- **Positions are counted from one.** `the first of friends` is
  `item 1 of friends`, and both become `friends[0]`. `item 0 of friends` is
  refused with `E0229` instead of quietly handing back the last item.
- **`remove Mina from friends` no longer compiles to `friends = friends - Mina`.**
  That line looked right, compiled, and then died at run time. Subtracting from
  a name the program made a list is now the removal it plainly means, and
  subtracting a word that was never saved from a name that is not a list is
  refused with the reason.
- **Every list statement is gated on a name that was already made a list.**
  `sort out your things`, `the first of many`, `count me in` and
  `친구들 이야기를 들었습니다` therefore stay ordinary sentences and print
  themselves; using one of the statements on a name that is not a list is
  refused with `E0231` rather than guessed at.
- **A loop with no counter**: `repeat forever` / `계속 반복해`, closed by
  `end` / `끝` and left with `break` / `멈춰`.
- **Text has a length and a case**: `the length of name` / `이름 길이`,
  `name in capitals` / `이름 대문자로`, `name in small letters` /
  `이름 소문자로`. All three read any saved name and are values, so they work in
  output, in a saved name, and in a condition.
- **The five missing English zero-knowledge sentence forms exist.**
  `challenge different zero knowledge challenge make`,
  `nonce secret challenge zero knowledge response make`,
  `public commitment challenge response zero knowledge verify`,
  `zero knowledge simulated response make` and
  `public challenge response zero knowledge simulated commitment make`. Until
  now an attempt at one of them was saved as a *sentence*:
  `set ok to p c e z zero knowledge verify` stored a string, so the program
  ran, verified nothing, and said nothing.
- **`skip` and `멈춰` work inside an indented `3 times:` block.** They used to
  be left there as bare Python names, so the program compiled and then raised
  `NameError`; Python's own `continue` was refused in the same place even
  though `break` was accepted.
- **`아니면:` and `아니면 만약에 …:` work inside an indented block**, the way
  English got `else:` and `elif …:` for free from Python. `else if …:` now
  works there too.
- **Another `.nme` program can be imported in a sentence, in both languages**:
  `use greet from "helper.nme"` / `"helper.nme"에서 greet 가져와`. The quoted
  path ending in `.nme` is what makes the line an import, so `use random`
  still loads the bundled module.
- **`ask` followed by a question needs no name in between.**
  `ask what is your name`, `ask how old are you`, `물어봐 이름이 뭐예요?` and
  `물어봐 몇 살이에요?` now save into the name those questions already answer
  (`name`, `age`, `이름`, `나이`). The Korean forms used to keep the particle
  and save into `이름이`, or into `몇`. A question with no name to save into,
  such as `ask who is there`, is still refused.
- **`할 일은 목록` is refused instead of printed.** A name is one word, so
  `할 일` is two and the line quietly became its own sentence — guide 05
  taught exactly that. The refusal names the spelling that works (`할일`).
- The mistake corpus grew from 587 short programs to 723: the list and text
  grammar, the parity holes this release closes, and the ordinary prose that
  contains each new word. The number that matters — accepted but compiled to
  something the writer did not mean — is **still 11**, and all thirty prose
  sentences still print themselves.
- **What is left over after a division has a spelling**:
  `the remainder of pile divided by 4` / `쌓인돌을 4로 나눈 나머지`. It is a
  value, so it works in output, in a saved name, and in a condition — which is
  what a counting game such as Nim is decided by.
- **`set left to score divided by 4` no longer becomes `set = set / 4`.** A
  line that opens with a saving word is a save, not arithmetic on a name
  called `set`.
- **A story that opens with nothing in it says so.** `story:` alone used to
  become `if True:` with no body, and the reader met CPython's own
  `SyntaxError` pointing into generated code; `얘기: 그만하자` was a valid
  Python annotation that compiled, did nothing, and said nothing. Both are
  now named, with the two readings spelled out.
- **`목록을 보여 주세요` no longer prints `[]`.** A bare `목록` / `list` is an
  empty list only where a value is being **saved into a name**; given to an
  output word the same words are the ones somebody wrote, so
  `목록 보여줘` prints `목록` and `show list` prints `list`. `친구들은 목록`,
  `친구들은 빈 목록` and `set friends to an empty list` are unchanged.
- **Two ordinary words stopped being read as misspelled commands.** `가장`
  ("most") is one keystroke from `저장` ("save"), so `가장 좋은 하루였습니다`
  quietly became `좋 = "하루였습니다"`; `how` is `show` without its `s`, so
  `how are you` printed `are you`. A word the sentence grammar spells out
  itself is no longer read as a misspelling of something else.
- **Far more of what a beginner actually types is accepted, and what cannot be
  read is said out loud instead of guessed at.** Measured against a 566-mistake
  corpus (`scripts/mistake-probes/`), the number that compiled to something the
  writer plainly did not mean **fell from 141 to 11**. Raw CPython syntax errors
  reaching the reader fell from 27 to 2, and all thirty sentences in the prose
  corpus now print themselves.
- **Ordinary Korean sentences no longer become variable assignments in silence.**
  `좋은 아침입니다` ("good morning") compiled to `좋 = "아침입니다"` and said
  nothing. A value that ends in a Korean sentence ending is now read as prose.
  `이름은 민수`, `점수는 0` and `점수는 0이다` still assign, as before.
- One-character typo repair now covers waiting, appending, break, skip and the
  list loop. Number words (`three times`, `세 번`, `일초`) and counters
  (`times`, `번`, `회`, `차례`) are understood, Korean has as many ways to store
  a value as English, word order may vary (`to score add 1`), and politeness
  words are ignored wherever they appear.
- Loop-control synonyms (`stop`, `quit`, `keep going`, `그만해`, `계속해`) and
  block-closing words (`finish`, `done`, `종료`, `마침`) are accepted.
  `add Mina to friends` becomes an append when `friends` is a list, and is
  refused pointing at `append` when it is not.
- A word the compiler does not understand never reaches `range()`. A line that
  opens with a word no one writes in prose (`output hello`) is refused with the
  action it probably meant; ordinary prose keeps printing.
- An `end` with nothing to close names the header it could not read.
- A middle dot, an em dash, full-width punctuation and emoji are ordinary
  characters in a message. Curly quotes are refused, naming the straight quote
  to use instead.
- A random pick no longer accepts a typo, which is what turned
  `강을 따라 집으로 갑니다` into a random choice.

- Sentence syntax gained **story, screen and time** statements: `say slowly`
  (one letter at a time), `say very slowly`, `say slowly every 0.2 seconds`,
  `clear the screen`, `draw a line`, `say in a box`, `say in the middle`,
  `start the timer` with the value `elapsed`, and named cooldowns
  (`put attack on cooldown for 3 seconds`, `when attack is ready` /
  `is on cooldown`, `wait for attack`). Korean has all of them too. One NME
  statement is still exactly one line of Python.
- Reading `elapsed` before starting the timer is now refused at compile time
  with `E0226`.
- The box and the centred line count a Korean letter as two columns wide, so
  neither comes out crooked.
- Three new guides. The reordering below made them 03 (letters one at a time),
  14 (screen) and 15 (time), all inside Part 1.
- **The guides were reordered from scratch for a beginner.** Eight parts, and
  all 88 guides renumbered and renamed. The old order carried the repository
  README's shape — files at 13, a cryptocurrency ledger at 17, writing a story
  at 86. Every cross-guide link, title and prerequisite was rewritten, and no
  prerequisite points forward any more. Part 1 (01–16) is sentences only: its
  296 lines of code contain no quote, no bracket and no equals sign.
- **Two guides were written**: 02 (story blocks) and 13 (chance). Ninety in all.
- **Part 1 no longer assumes an installation.** It used to open with "create a
  file and run `nme run hello`"; it now opens in the writing box on
  needmoreeasy.com. The command line is kept, marked "skip this for now", at the
  end of guides 01 and 16. The five-minute guide and the tutorial match.
- **A list example guide 05 taught was really being printed as words.** A name
  cannot contain a space, so `할 일은 목록` came out as its own sentence. It is
  `할일` now, and the rule is stated.
- Three new checks. `scripts/check-guide-silent.py` finds a line meant as a
  command that quietly became text, across every guide;
  `scripts/report-guide-tier.py` measures how much of each part is really
  sentence grammar; `scripts/check-tier-parity.py` compiles 480 cells across 80
  capabilities (sentence/beginner/advanced × English/Korean) and compares them,
  including whether a refusal carries the same code in both languages.
- The three prompts for an AI now carry story blocks and chance.

- The five-minute guide and the tutorial no longer teach beginner syntax or
  Python in their early sections; a sentence-level project on lists and waiting
  takes that place.

- Sentence syntax gained a **story block**. `story:` / `이야기:` opens a block
  in which **every line is text**, so a page of prose needs no output word on
  every line. Nothing inside it is a command — `wait 3 seconds` prints those
  words, and so does `if ready` — because a line of a story that quietly became
  a statement is the worst mistake this compiler can make. A blank line prints
  an empty line, names saved earlier are still put into the text, and
  `slow story:`, `very slow story:` and `slow story every 0.2 seconds:` tell the
  whole block one character at a time. The block closes at `end` / `끝`, or
  where its indentation ends. Korean writes `천천히 이야기:`,
  `아주 천천히 이야기:` and `0.2초씩 천천히 이야기:`, and the colon may be the
  full-width `：` a Korean keyboard produces.
- Sentence syntax gained a **chance in percent**: `30% chance show You win` /
  `30% 확률로 말해줘 당첨`, the same words alone opening a block, and
  `luck is a 30% chance` / `운은 30% 확률` saving a true/false value that the
  ordinary conditions can ask about (`if luck then ...`). `with a 30% chance`,
  `a 30% chance`, `30 percent chance`, `30% of the time`, `30%의 확률로`,
  `확률 30%로` and `30퍼센트 확률로` all mean the same thing.
- A chance is counted in thousandths, so `30.5%` is exactly 305 out of 1000 and
  nothing is decided by comparing two nearly equal decimal numbers. One decimal
  place is the finest anyone may write: `30.25%` is refused as `E0227` rather
  than rounded, because a program must never quietly mean something its writer
  did not write. Outside `0%` to `100%` is `E0228`.
- A percentage on its own is still not a chance and a sentence about a story is
  still a sentence: `I am 100% sure`, `전체의 30%가 왔습니다`, `story time`,
  `tell me a story` and `옛날 이야기` are unchanged.
- **A sentence with a percentage in it is a sentence.** `%` is Python's modulo
  operator and a Korean word is a valid Python name, so `100% 확신합니다` really
  is a valid Python expression: it was handed to Python and answered with a
  `NameError` at run time. `전체의 30%가 왔습니다` reached CPython as a syntax
  error, and `나는 100% 동의합니다` was quietly saved under the name `나`. All
  three print now, and `합니다` and `됩니다` joined the Korean sentence endings
  the compiler knows.
- **The words after a chance have to be a command.** `a 30% chance of rain` and
  `확률 30%는 낮습니다` no longer run their own tail three times in ten, and
  `a 20% chance remains` no longer compiles to `if …: remains`. The word-first
  Korean spelling needs its particle — `확률 30%로` — which is the whole
  difference between a command and a remark about a percentage.
- **A label and the words after it prints.** `재미있는 이야기: 시작` was being
  saved under the name `재미있`, because its first word ends in the Korean
  topic particle. A value NME cannot read that carries a colon is writing, not
  a value; a colon Python can read (a dict, a slice, a lambda) is still a value.
- **A number with a unit after it is a sentence, not a value.**
  `할인율은 30%입니다` was saved as `할인율 = 30%입니다`, which Python reads as
  `30 % 입니다`: a modulo against a name nothing ever bound. It compiled and
  then raised `NameError` the moment it ran. `점수는 30점입니다`,
  `가격은 1000원입니다` and `거리는 3km입니다` print for the same reason. A bare
  number still saves: `정답은 7입니다` is the number seven, spoken as a
  sentence.
- One Korean noun one edit away from the file-reading word — `경고`, `참고`,
  `보고` — crashed the compiler outright. It no longer does; those lines are
  ordinary writing and print.
- `scripts/check-prose-blocks.py` now compiles a documented line together with
  the story block it is shown inside, which is the program the document
  actually shows. The mistake-probe corpus gained twenty-one lines about
  percentages, labels, and units (566 → 587); the count of programs that
  compile to something their writer plainly did not mean is unchanged at 11.

## 0.0.1-beta.160 — 2026-08-12

- Fetch the pull-request parent commit before checking changed Rust files, so
  the platform Format jobs can run their scoped rustfmt gate in a shallow CI
  checkout.

## 0.0.1-beta.159 — 2026-08-12

- Keep full-tree formatting checks on branch pushes while pull-request checks
  format only Rust files changed by the PR, avoiding failures from pre-existing
  formatting drift on the base branch.

## 0.0.1-beta.158 — 2026-08-12

- Detect star imports and `except*` control-flow statements after earlier
  semicolon-separated statements, preserving the shared `E0114` and `E0115`
  diagnostics in one-line Python suites.

## 0.0.1-beta.157 — 2026-08-12

- Detect `return`, `break`, and `continue` after earlier semicolon-separated
  statements in one-line Python suites, preserving the shared scope diagnostics.

## 0.0.1-beta.156 — 2026-08-12

- Extend Python control-flow diagnostics to one-line function and class suites,
  so `return`, `break`, and `continue` do not inherit invalid outer contexts.

## 0.0.1-beta.155 — 2026-08-12

- Extend shared Python scope diagnostics to one-line function and class suites,
  including `global`/`nonlocal` conflicts, annotated targets, `nonlocal`
  placement, and star imports.

## 0.0.1-beta.154 — 2026-08-12

- Preserve valid contextual keywords in one-line Python function suites and
  report async-generator value returns there with the shared `E0118` diagnostic.

## 0.0.1-beta.153 — 2026-08-12

- Correct `E0119` handling for names used in Python annotations while keeping
  f-string validation with the CPython backend.

## 0.0.1-beta.152 — 2026-08-12

- Add shared bilingual `E0119`/`E0120` for conflicting `global` and `nonlocal`
  declarations, while preserving declarations placed before valid uses.

## 0.0.1-beta.151 — 2026-08-12

- Add shared bilingual `E0118` for return values inside async generators,
  while preserving bare returns and nested function scopes.

## 0.0.1-beta.150 — 2026-08-12

- Add shared bilingual `E0117` for asynchronous comprehensions outside an
  `async def` function, while preserving valid async-comprehension bodies.

## 0.0.1-beta.149 — 2026-08-12

- Add shared bilingual `E0116` for `yield` inside Python comprehensions,
  while preserving ordinary `yield` expressions and generator lambdas where
  Python permits them.

## 0.0.1-beta.148 — 2026-08-12

- Add shared bilingual `E0115` for `break`, `continue`, and `return` inside
  Python `except*` blocks, while preserving nested function bodies and control
  flow after the handler suite.

## 0.0.1-beta.147 — 2026-08-12

- Add shared bilingual `E0114` for `from ... import *` inside Python functions
  and classes, while preserving valid module-level star imports.

## 0.0.1-beta.146 — 2026-08-12

- Add shared bilingual `E0113` for Python `nonlocal` without an enclosing
  function, while preserving valid nested function/class uses and leaving
  missing outer-name binding validation to CPython.

## 0.0.1-beta.145 — 2026-08-12

- Add shared bilingual `E0111`/`E0112` diagnostics for `async for` and
  `async with` outside `async def` functions.

## 0.0.1-beta.144 — 2026-08-12

- Keep generator lambdas byte-identical while Python-context diagnostics inspect
  their own function scope, including normal lambdas nested in `async def`.

## 0.0.1-beta.143 — 2026-08-12

- Diagnose `yield from` inside `async def` with shared bilingual `E0110`, while
  preserving it unchanged in ordinary generator functions.

## 0.0.1-beta.142 — 2026-08-12

- Add shared bilingual `E0108`/`E0109` diagnostics for `yield` outside a
  function and `await` outside an `async def`, and keep `return`/`yield` from
  inheriting an outer function context through nested class bodies.

## 0.0.1-beta.141 — 2026-08-12

- Extend the shared inline-branch `E0103` guard to sentence-repeat bodies and
  keep Korean repeat shapes ahead of subject-first condition recovery.

## 0.0.1-beta.140 — 2026-08-12

- Reject inline `else`/`elif` bodies without an open condition with the shared
  bilingual `E0103` diagnostic instead of lowering invalid Python.

## 0.0.1-beta.139 — 2026-08-12

- Report top-level and inline Python `continue` outside a loop with the shared
  bilingual `E0107` diagnostic while preserving valid loop bodies.

## 0.0.1-beta.138 — 2026-08-12

- Report top-level and inline `return` outside a Python function with the
  shared bilingual `E0106` diagnostic while preserving valid function bodies.

## 0.0.1-beta.137 — 2026-08-12

- Clarify bilingual `E0102` recovery guidance so it covers NME `repeat` and
  valid Python loops alongside NME `while`.

## 0.0.1-beta.136 — 2026-08-12

- Report one-line `break` outside a loop with the stable bilingual `E0102`
  diagnostic in the shared parser while preserving valid Python loop bodies.

## 0.0.1-beta.135 — 2026-08-12

- Extend documentation parity checks to validate local example and directory
  links, not only Markdown targets.

## 0.0.1-beta.134 — 2026-08-12

- Add six-surface native regression coverage for one-line `else if` branches
  that terminate with `break`.

## 0.0.1-beta.133 — 2026-08-12

- Add six-surface native regression coverage for one-line `while` bodies that
  terminate with `break`.

## 0.0.1-beta.132 — 2026-08-12

- Recognize sentence-repeat `break here`/`여기서 멈춰` bodies in the shared
  parser and cover them through the native backend's six-surface break tests.

## 0.0.1-beta.131 — 2026-08-12

- Support one-line native `break` bodies inside loops, with six-surface
  coverage and bilingual rejection outside native loops.

## 0.0.1-beta.130 — 2026-08-12

- Document native one-line NME output bodies for sentence repeats alongside
  `times:`/`번:` loops, and add six-surface support/boundary regression coverage.

## 0.0.1-beta.129 — 2026-08-12

- Add native six-surface regression coverage for one-line `say` output bodies,
  alongside the existing `show` and `말해` spellings.

## 0.0.1-beta.128 — 2026-08-12

- Add a Python-wins regression for the valid Korean call shape `만약 (준비)`.
- Clarify in the English and Korean language, native-reference, and AI guides
  how to select an NME block when a Korean keyword-like identifier is bound.

## 0.0.1-beta.127 — 2026-08-12

- Add native regression coverage for one-line NME `else if`/`else` bodies across
  sentence, beginner, and advanced syntax in both English and Korean.
- Exercise both branch outcomes while keeping the Python-wins boundary explicit
  for the advanced Korean condition fixture.

## 0.0.1-beta.126 — 2026-08-12

- Add native regression coverage for one-line NME `while` output bodies across
  sentence, beginner, and advanced syntax in both English and Korean.
- Include the spoken Korean comparison ending alongside symbolic and natural
  English conditions so all six surfaces exercise the same native path.

## 0.0.1-beta.125 — 2026-08-12

- Teach the native one-line NME output-body form in the English and Korean
  native learning guides, including its CPython-only boundary.

## 0.0.1-beta.124 — 2026-08-12

- Document beta123's native one-line NME control bodies in the backend design
  memo so the architecture notes match the implemented subset.

## 0.0.1-beta.123 — 2026-08-12

- Extend the native backend to lower one-line NME `say`/`show` bodies after
  `then`/`그러면` for `if`/`while` and branch chains.
- Preserve native branch-flow tracking across inline `elif`/`else` bodies while
  keeping unsupported Python inline statements rejected.

## 0.0.1-beta.122 — 2026-08-12

- Add shared-parser and native regression coverage for Korean comparison endings
  before mixed-language `and`/`or` connectors inside wrapped conditions.

## 0.0.1-beta.121 — 2026-08-12

- Fix parenthesized Korean `while` conditions when a comparison ending comes
  before an inner `and`/`그리고` or `or`/`또는` connector.
- Keep the spoken `동안` ending inside the shared condition tree and preserve
  the actual loop-body boundary after the wrapper.

## 0.0.1-beta.120 — 2026-08-12

- Add explicit core and native regression coverage for parenthesized Korean
  comparison endings before the `또는`/`or` connector.

## 0.0.1-beta.119 — 2026-08-12

- Fix parenthesized Korean logical conditions where a comparison ending comes
  before an inner `and`/`그리고` or `or`/`또는` connector.
- Scan fully wrapped conditions at their effective logical depth while keeping
  nested operand parentheses opaque, with core and native regression coverage.

## 0.0.1-beta.118 — 2026-08-12

- Extend regression coverage for Korean comparison endings inside parenthesized
  `elif` conditions, including exact core lowering and native execution.
- Keep the branch case on the same shared condition-span path as `if` and
  `while`; no separate English/Korean implementation is introduced.

## 0.0.1-beta.117 — 2026-08-12

- Preserve the closing parenthesis and body boundary when Korean sentence
  comparison endings appear inside parenthesized `if`/`elif`/`while` conditions.
- Cover the corrected form in core transpilation and native execution while
  keeping the shared English/Korean condition path.

## 0.0.1-beta.116 — 2026-08-12

- Fix Korean sentence `while`/`동안` headers whose natural ending appears
  inside a parenthesized logical condition, preserving the matching closing
  bracket and the real body span.
- Cover parenthesized logical `while` conditions across English and Korean
  sentence, beginner, and advanced native surfaces.
- Keep the shared condition grammar and valid-Python boundary documented in
  both language references, native references, and AI guidance.

## 0.0.1-beta.115 — 2026-08-12

- Accept parentheses around a whole colon-free NME logical condition, such as
  `if (ready and score > 2)`, while preserving the shared English/Korean
  condition tree.
- Keep valid Python calls such as `when(ready and score > 2)` byte-identical
  instead of treating them as NME headers.
- Cover the English/Korean sentence, beginner, and advanced surfaces in the
  native matrix and synchronize the language, native-reference, and AI guides.

## 0.0.1-beta.114 — 2026-08-12

- Extend native logical-condition regression coverage to `while` blocks across
  English and Korean sentence, beginner, and advanced surfaces.
- Document that the native reference covers logical conditions in both `if` and
  `while` control flow.

## 0.0.1-beta.113 — 2026-08-12

- Align the native-backend status summary with the implemented logical
  `and`/`or` condition support and its Python precedence and short-circuit
  behavior.
- Keep the English and Korean backend overview banners synchronized with the
  detailed native capability list.

## 0.0.1-beta.112 — 2026-08-12

- Add native logical conditions with `and`/`or` and the Korean spellings
  `그리고`/`또는`, preserving Python precedence and short-circuit evaluation.
- Lower the shared condition tree recursively while keeping unsupported
  Python-colon conditions and native operands outside the restricted subset.
- Cover the six English/Korean sentence, beginner, and advanced surfaces with
  short-circuit regression tests, paired examples, and synchronized native
  documentation.

## 0.0.1-beta.111 — 2026-08-12

- Fix native fall-through analysis for a terminating nested conditional, so a
  name assigned on every path that can actually reach a later return is not
  rejected as conditionally initialized.
- Preserve conservative loop analysis and the required top-level integer
  return for native functions.
- Cover English and Korean sentence, beginner, and mixed advanced native
  forms, with synchronized native references, guides, and language docs.

## 0.0.1-beta.110 — 2026-08-12

- Add real native boolean bindings as a static type distinct from integers;
  `True`/`False` and sentence `true`/`false`/`참`/`거짓` values can be assigned,
  compared, used as conditions, and shown as `True` or `False`.
- Reject boolean arithmetic, value changes, and integer-only native function
  arguments or returns instead of relying on their C `int` representation.
- Cover the behavior across English and Korean sentence, beginner, and
  advanced surfaces, with synchronized native references, guides, AI guidance,
  and installation/version documentation.

## 0.0.1-beta.109 — 2026-08-12

- Exclude a branch that breaks out of its enclosing native loop from
  fall-through binding analysis, matching the existing early-return behavior.
- Cover the shared English and Korean behavior across sentence, beginner, and
  mixed advanced native surfaces, and align the native documentation.

## 0.0.1-beta.108 — 2026-08-12

- Allow a native `if`/`else` branch that returns early to be excluded from
  fall-through binding analysis, so a later return can use a name assigned on
  every path that reaches it.
- Cover the behavior across English and Korean sentence, beginner, and mixed
  advanced native surfaces, and align the native reference and guides.

## 0.0.1-beta.107 — 2026-08-12

- Upgrade a previously conditional binding when every arm of a later
  `if`/`else` assigns it, without weakening the conservative loop and
  one-sided-branch checks.
- Extend the English and Korean native branch-merge regression coverage.

## 0.0.1-beta.106 — 2026-08-12

- Treat a name assigned in every `if`/`else` branch as definitely initialized
  after the block while keeping one-sided and possibly skipped loop bindings
  conditional.
- Cover the shared English and Korean branch-merge behavior in native tests and
  references.

## 0.0.1-beta.105 — 2026-08-12

- Fix native comparisons of two string concatenations so each operand keeps
  its own checked runtime buffer instead of comparing the second result with
  itself.
- Cover the corrected behavior in English and Korean native sentence syntax.

## 0.0.1-beta.104 — 2026-08-12

- Correct the native-backend contract to document that `break` works inside an
  `if` nested in a native loop and is rejected only outside loops.
- Clarify the native expression-lowering comment for checked integer and
  finite-float helpers.

## 0.0.1-beta.103 — 2026-08-12

- Reject non-finite results from native finite-float arithmetic with a bilingual
  runtime error instead of allowing C `double` overflow to produce `inf`.
- Cover the runtime boundary across sentence, beginner, and advanced English
  and Korean native surfaces.
- Align native reference, guide, language-reference, and backend comments with
  the checked finite-float result policy.

## 0.0.1-beta.102 — 2026-08-12

- Add a dedicated English/Korean native-core reference and link it from the
  README, language reference, native guide, and backend memo.
- Cover equivalent sentence, beginner, and advanced native programs in both
  languages with one end-to-end six-case acceptance test.

## 0.0.1-beta.101 — 2026-08-12

- Include finite-float values in the generic native-backend recovery hint and
  cover the English and Korean hints with a regression test.

## 0.0.1-beta.100 — 2026-08-12

- Align the native-backend opening summary with the implemented function
  subset: integer scalar parameters and an unconditional integer `return`.

## 0.0.1-beta.99 — 2026-08-12

- Make Korean-first CLI diagnostics use Korean command spellings in their
  recovery examples and code explanations, while keeping English invocations
  English-only.

## 0.0.1-beta.98 — 2026-08-12

- Align the language reference with the native function subset: integer scalar
  parameters and an unconditional integer `return`.

## 0.0.1-beta.97 — 2026-08-12

- Document finite-float truthiness in the native subset and cover its English
  and Korean behavior with an end-to-end native regression test.

## 0.0.1-beta.96 — 2026-08-12

- Align Korean reference, installation, and native-backend workflows with the
  Korean CLI commands, while labeling intentional English command spellings.

## 0.0.1-beta.95 — 2026-08-12

- Align Korean example-authoring, example-template, and calculator verification
  instructions with `nme 검사` and `nme 실행`.

## 0.0.1-beta.94 — 2026-08-12

- Align Korean file, project, HTTP, terminal-menu, and native guide workflows
  with their Korean CLI commands while retaining labeled English companions.

## 0.0.1-beta.93 — 2026-08-12

- Align the Korean index and first two beginner guides with the Korean CLI
  path, using `nme 실행` and `nme 검사` from the first run.

## 0.0.1-beta.92 — 2026-08-12

- Align the Korean file and JSON guide workflows with Korean CLI commands and
  give their try-it examples Korean primary paths.

## 0.0.1-beta.91 — 2026-08-12

- Align the Korean check/build and conversion guide commands with the Korean
  CLI paths, and document the Korean-first bilingual diagnostic output.

## 0.0.1-beta.90 — 2026-08-12

- Align the English and Korean condition/random guide examples with their
  named language paths, while retaining explicit mixed-language examples.

## 0.0.1-beta.89 — 2026-08-12

- Align the English and Korean break-loop guide examples with their named
  language paths, while retaining a separate mixed-language example.

## 0.0.1-beta.88 — 2026-08-12

- Align the English and Korean while-loop guide examples with their named
  language paths, while retaining a separate mixed-language example.

## 0.0.1-beta.87 — 2026-08-12

- Align the Korean Python-packages guide with `birthday.ko.nme`, Korean
  beginner spellings, and the equivalent English learning path.

## 0.0.1-beta.86 — 2026-08-12

- Run the Windows CLI install smoke test under PowerShell so Git Bash's
  `link.exe` cannot shadow the MSVC linker configured by CI.

## 0.0.1-beta.85 — 2026-08-12

- Teach the Python-packages guide to install third-party libraries through the
  bilingual `nme install` / `nme 설치` wrapper and explain its E9025 failure path.

## 0.0.1-beta.84 — 2026-08-12

- Keep MSVC compiler banners out of successful `nme native` output and preserve
  `.c`/`.ko` source stems in Windows default executable names (`.c.exe` and
  `.ko.exe`).
- Align native-backend documentation with the corrected cross-platform names.

## 0.0.1-beta.83 — 2026-08-12

- Configure the Windows CI job with a Visual Studio developer environment so
  native C tests can find `cl.exe` and exercise the real MSVC path.

## 0.0.1-beta.82 — 2026-08-12

- Reject blank package names before invoking pip so `nme install` cannot report
  a successful no-op when a newer pip ignores an empty requirement.
- Apply the stable Rust formatter used by CI to the workspace.
- Keep the native backend and its tests clean under the current Clippy gate.

## 0.0.1-beta.81 — 2026-08-12

- Pass MSVC `/utf-8` for Windows native builds and tests so generated Korean
  and English C strings retain their intended text.

## 0.0.1-beta.80 — 2026-08-12

- Make generated native C runtime helpers warning-safe for GCC, Clang, and
  MSVC, and reserve the generated `NME_UNUSED` macro from user identifiers.

## 0.0.1-beta.79 — 2026-08-12

- Select MSVC `cl` with Windows-compatible flags for `nme native`, keep `cc`
  on macOS/Linux, and document the required Windows developer shell.

## 0.0.1-beta.78 — 2026-08-12

- Allow default native builds for source stems ending in `.c`, while keeping
  the explicit `-o <path>.c` collision guard.

## 0.0.1-beta.77 — 2026-08-12

- Keep default native artifact names distinct for English and `.ko` sibling
  programs, add `.exe` to implicit Windows outputs even for `.ko` stems, and
  preserve explicit `-o` naming behavior.

## 0.0.1-beta.76 — 2026-08-12

- Emit C prototypes before native function definitions so forward calls and
  mutual recursion compile correctly, including zero-argument functions.

## 0.0.1-beta.75 — 2026-08-12

- Update both README compiler descriptions to identify the implemented v0
  native C backend and its roadmap, rather than describing it as only a plan.

## 0.0.1-beta.74 — 2026-08-12

- Update the native-backend memo to separate the implemented v0 baseline from
  future milestones and measured extensions.

## 0.0.1-beta.73 — 2026-08-12

- Correct the native-backend memo so its description matches the implemented
  restricted NME-to-C path and the separate Python/Nuitka path.
- Cover blank-line and comment layouts between native function headers and
  bodies in the backend regression suite.

## 0.0.1-beta.72 — 2026-08-12

- Make the documentation parity check verify that both sequential guide indexes
  list every numbered English/Korean guide exactly once and in order.

## 0.0.1-beta.71 — 2026-08-12

- Reject repeated `run`/`build` action words in `nme native` with stable
  bilingual diagnostic E9032, and make native CLI test directories unique.

## 0.0.1-beta.70 — 2026-08-12

- Reject `-o` on `nme native run` with stable bilingual diagnostic E9031;
  keep `-o` for `nme native build`.

## 0.0.1-beta.69 — 2026-08-12

- Enforce English/Korean twin files for every numbered learning guide in the
  documentation parity check.

## 0.0.1-beta.68 — 2026-08-12

- Extend the documentation parity check to validate local Markdown fragment
  links against their target headings as well as checking target files.

## 0.0.1-beta.67 — 2026-08-12

- Make the documentation parity check fail when a local Markdown link points
  to a missing file.

## 0.0.1-beta.66 — 2026-08-12

- Use overlap-safe copying for native string assignment so valid self-assignment
  cannot invoke undefined `memcpy` behavior.

## 0.0.1-beta.65 — 2026-08-12

- Reject C implementation-reserved identifier forms in native bindings and
  file-scope functions before they can produce non-portable or invalid C.

## 0.0.1-beta.64 — 2026-08-12

- Reject top-level native `return` with the stable `E0106` diagnostic instead
  of emitting it as a return from `main`.

## 0.0.1-beta.63 — 2026-08-12

- Reject native `break` statements outside a loop with the stable `E0102`
  diagnostic before generating invalid C.

## 0.0.1-beta.62 — 2026-08-12

- Reserve macros, typedefs, and declarations exposed by generated C headers so
  native identifiers cannot be changed by preprocessing or C library clashes.

## 0.0.1-beta.61 — 2026-08-12

- Make native `len` count UTF-8 Unicode characters rather than storage bytes,
  matching Python behavior for non-ASCII text while retaining the byte buffer
  limit.

## 0.0.1-beta.60 — 2026-08-12

- Isolate native binding analysis between sibling branches so a read in one
  branch cannot use a name assigned only in another branch.
- Preserve C declaration reuse and conservative maybe-initialized tracking after
  uncertain control blocks.

## 0.0.1-beta.59 — 2026-08-12

- Keep bindings from unreachable `else`/`else if` alternatives after `if true`
  out of the definite native scope, preventing reads of uninitialized C values.

## 0.0.1-beta.58 — 2026-08-12

- Reject unresolved native names, bare native function values, duplicate
  parameters, and bindings that shadow native function names before C emission.

## 0.0.1-beta.57 — 2026-08-12

- Emit Python comments as inert C comments during native lowering so comment
  text cannot become a C preprocessor directive or confuse function hoisting.

## 0.0.1-beta.56 — 2026-08-12

- Reject non-finite native float literals before C emission and normalize finite
  whole-number literals as C `double` values, preserving signed zero.

## 0.0.1-beta.55 — 2026-08-12

- Escape native string literals for valid C output, including control
  characters, and reject embedded NUL strings that C APIs cannot preserve.

## 0.0.1-beta.54 — 2026-08-12

- Reject nested native function definitions before C emission; the native
  function surface is explicitly file-scope only.

## 0.0.1-beta.53 — 2026-08-12

- Reject duplicate native function definitions and unsupported default or
  varargs headers before C generation.
- Reject keyword arguments in native calls so no AST arguments are silently
  dropped.

## 0.0.1-beta.52 — 2026-08-12

- Validate native function calls against integer function definitions and their
  declared arity before emitting C, with bilingual diagnostics for unknown or
  mismatched calls.

## 0.0.1-beta.51 — 2026-08-12

- Reject native functions that can fall through without an unconditional
  integer return, preventing undefined C return values after conditional-only
  branches.

## 0.0.1-beta.50 — 2026-08-12

- Check native signed 32-bit integer literals and arithmetic instead of
  allowing undefined C overflow or zero-divisor behavior.
- Report bilingual native runtime errors for integer overflow and modulo by
  zero, while documenting the bounded native integer range.
- Reject float arguments and return values in native functions instead of
  silently converting them through C `int` parameters and returns.

## 0.0.1-beta.49 — 2026-08-12

- Track conditional native bindings and reject reads or value changes after a
  possibly skipped block unless the name was initialized beforehand.
- Keep statically true `if true` blocks usable while reporting a precise
  bilingual diagnostic for uncertain initialization.

## 0.0.1-beta.48 — 2026-08-12

- Reject native assignments that change a binding from integer, float, or
  string to another type before generating incompatible C.
- Require a prior numeric binding for `add`/`subtract` value changes and report
  a bilingual diagnostic for uninitialized or string targets.

## 0.0.1-beta.47 — 2026-08-12

- Hoist native scalar and string declarations to the active function scope when
  assignments occur inside control blocks, while preserving source control
  flow and later binding use.
- Prevent nested-block assignments from producing out-of-scope C declarations.

## 0.0.1-beta.46 — 2026-08-12

- Keep native function-local scalar bindings separate from main-program
  bindings, so names can be reused without generating invalid C.
- Document the scope behavior in both native backend references.

## 0.0.1-beta.45 — 2026-08-12

- Reject generated native-runtime names in function parameters before emitting C,
  including unused parameters that would otherwise shadow runtime helpers.
- Keep the same precise bilingual diagnostic used for native variables and
  function names.

## 0.0.1-beta.44 — 2026-08-12

- Reject C keywords and generated native-runtime names before C lowering, with
  precise bilingual diagnostics instead of allowing namespace collisions.
- Reserve helper names such as `nme_copy`, `nme_cat`, `len`, and `_nme_i`
  without silently renaming user identifiers.

## 0.0.1-beta.43 — 2026-08-12

- Replace unbounded native string copies and concatenation with checked helpers.
- Stop oversized stored or concatenated strings with a bilingual runtime error
  instead of allowing fixed-buffer overflow.
- Document the native string capacity and keep the CPython path available for
  unrestricted text.

## 0.0.1-beta.42 — 2026-08-12

- Make `nme build -o` refuse to overwrite an existing Python output with E9009.
- Keep the existing artifact unchanged for English and Korean build commands,
  matching `nme compile` and `nme native build`.

## 0.0.1-beta.41 — 2026-08-12

- Own imported-module, native, and Nuitka staging directories for the whole
  operation and remove them on both success and early failure.
- Prevent partial Python or C staging files from being left behind when a write
  fails.

## 0.0.1-beta.40 — 2026-08-12

- Use fresh per-invocation temporary directories for imported-module, native,
  and Nuitka staging instead of reusing process-ID-only folders.
- Prevent stale Python files left by a crashed run or PID reuse from shadowing
  ordinary imports in a later program.

## 0.0.1-beta.39 — 2026-08-12

- Make `nme native build` refuse to overwrite an existing executable or
  companion C source with E9009, matching the other build commands.
- Reject `.c` output paths with E9003 so the executable and generated C source
  cannot target the same file.
- Extend the E9009 English/Korean lookup explanation to cover native artifacts.

## 0.0.1-beta.38 — 2026-08-12

- Make `nme native` classify directory arguments as E9014 instead of reporting
  them as unreadable files E9007, matching the CPython-backed commands.
- Add English/Korean native-command coverage for the shared folder diagnostic.

## 0.0.1-beta.37 — 2026-08-12

- Keep existing but unreadable program files on E9007 instead of reporting them
  as missing programs E9015 for `nme run` and `nme native`.
- Preserve E9015 for paths that actually cannot be found.

## 0.0.1-beta.36 — 2026-08-12

- Keep `nme compile` temporary-folder failures on E9027 instead of reporting
  them as native compiler startup failures E9011.
- Classify temporary Python-source write failures as E9008 while preserving
  E9011 for failures to start the external compiler process.

## 0.0.1-beta.35 — 2026-08-12

- Report unreadable imported `.nme` modules with the existing file-read
  diagnostic E9007 instead of the top-level program-resolution code E9015.
- Expand E9007’s bilingual explanation to cover imported module files.

## 0.0.1-beta.34 — 2026-08-12

- Give `nme install` without a package name its own stable diagnostic, E9030,
  instead of reusing the option-value code E9003.
- Add English/Korean lookup coverage for the missing-package argument path.

## 0.0.1-beta.33 — 2026-08-12

- Give imported module-name collisions their own stable diagnostic, E9028,
  instead of reporting them as invalid option values.
- Give the current `nme compile` module-import limitation its own stable
  diagnostic, E9029, with bilingual lookup and CLI regression coverage.

## 0.0.1-beta.32 — 2026-08-12

- Add E9027 for temporary working-folder creation failures instead of labeling
  them as current-folder read errors.
- Preserve Korean-first bilingual diagnostics when imported modules are staged
  for execution.

## 0.0.1-beta.31 — 2026-08-12

- Give native executable startup failures their own stable diagnostic, E9026,
  instead of reporting them as Python startup errors.
- Add a deterministic Unix CLI regression and bilingual public lookup coverage
  for the native-program startup path.

## 0.0.1-beta.30 — 2026-08-12

- Clarify E9010 and E9011 in both languages so their recovery guidance covers
  the Nuitka `compile` path and the system-C-compiler `native` path.
- Add public lookup regressions that keep the two backend toolchains visible to
  beginners.

## 0.0.1-beta.29 — 2026-08-12

- Give failed Python package installs their own appended diagnostic code,
  E9025, instead of reusing the native-compiler code E9010.
- Add bilingual lookup and network-independent CLI regression coverage for the
  package-install failure path.

## 0.0.1-beta.28 — 2026-08-12

- Route the Korean `네이티브` and `설치` command aliases through the same
  Korean-first bilingual diagnostics as the other Korean CLI commands.
- Add regression coverage for both failure paths while preserving English-only
  output for the English commands.

## 0.0.1-beta.27 — 2026-08-12

- Fix the English beginner skeleton in both example-template twins so its loop
  stops at three instead of becoming an accidental infinite loop.
- Add a parity regression guard for the bounded English and Korean template
  loops.

## 0.0.1-beta.26 — 2026-08-12

- Fill the remaining English code examples in the main NeedMoreCoin guide and
  the example-authoring guide so paired guides expose equivalent teaching
  material.
- Extend the documentation parity check to every guide pair, not only numbered
  guides, and require matching code-block coverage.

## 0.0.1-beta.25 — 2026-08-12

- Complete the missing Topic metadata in the four NeedMoreCoin sequence guides
  in both languages.
- Add equivalent English sentence-level proof-of-work and transaction-proof
  snippets, and enforce numbered-guide code-block parity in CI.

## 0.0.1-beta.24 — 2026-08-12

- Repair Korean documentation links so local pages lead to their Korean twins,
  while deliberate English comparison links remain available.
- Complete consistent bilingual navigation for the guide sequence and add a CI
  parity check that catches wrong-language links and missing navigation rows.

## 0.0.1-beta.23 — 2026-08-12

- Keep the proof-of-work difficulty labels consistent across the six
  NeedMoreCoin examples (Korean/English sentence, beginner, and advanced
  surfaces), with a regression test for the shared learning contract.

## 0.0.1-beta.22 — 2026-08-12

- Make the six-way NeedMoreCoin learning matrix fast and deterministic enough
  for regular example validation, and keep the sentence examples genuinely
  punctuation-free where their surface promises that progression.
- Add parser and regression coverage for the pure English sentence proof
  expressions and validate the Korean and English sentence sources separately.

## 0.0.1-beta.21 — 2026-08-12

- Repair the locked release metadata and keep CLI and cryptocurrency example
  regression checks aligned with the current beta package versions.

## 0.0.1-beta.20 — 2026-08-12

- Replace the earlier standalone blockchain demonstrations with the
  NeedMoreCoin learning project: complete Korean/English sentence, beginner,
  and advanced examples, a shared construction guide, and an authoring
  standard for six-way examples.
- Add automated coverage that checks all six examples, their intended syntax
  surfaces, and their shared observable behavior.

## 0.0.1-beta.19 — 2026-08-12

- Converge the public beta Git topology with `main`: the final beta.19 release commit keeps beta.18 as its first parent and records the current main tip as its second parent. The beta first-parent release line still advances exactly one version per public commit, while `main` becomes an actual ancestor of the next-generation `beta` branch.
- Keep the beta.17 release guard, locked Cargo validation, three-OS gate, and CPython 3.10/3.12/3.14 compatibility matrix unchanged.

## 0.0.1-beta.18 — 2026-08-12

- Extend the bundled Schnorr adapter to version `0.0.2` with context-bound Fiat-Shamir non-interactive proofs. The SHA-256 challenge binds the Group 15 generator, commitment, public key, and a length-prefixed explicit context under an NME domain tag.
- Add `zk_nizk_challenge`, `zk_nizk_prove`, and `zk_nizk_verify` plus Korean sentence forms. Proofs are JSON-friendly `[commitment, response]` values and cross-context reuse is rejected.
- Add Korean/English executable examples, parser/lowering and CLI end-to-end coverage, and explicit documentation that context binding does not replace same-context freshness/replay controls.

## 0.0.1-beta.17 — 2026-08-12

- Make `beta` the enforced next-generation release line. Every public beta push must advance the workspace beta number by exactly one, name that version in the commit subject, and keep the workspace package versions in `Cargo.lock` synchronized.
- Upgrade CI to `actions/checkout@v6` and `actions/setup-python@v6`, and run Cargo checks and tests with `--locked`.
- Add CPython 3.10, 3.12, and 3.14 compatibility jobs for beta and pull requests while retaining the full Ubuntu, Windows, and macOS quality gate.

## 0.0.1-beta.16 — 2026-08-11

- Add the bundled `zero_knowledge` / `영지식` adapter (version `0.0.1`) with a
  finite-field Schnorr proof-of-knowledge reference implementation: secure
  randomness from Python `secrets`, RFC 3526 3072-bit MODP Group 15 subgroup
  parameters, 256-bit verifier challenges, subgroup/range checks, transcript
  simulation helpers, helper-name collision protection, and Korean
  sentence-only proof expressions. Add matching Korean/English A→B examples
  with malicious relay C showing saved-transcript replay failure, transcript
  simulation, and the separate live-relay case. Document the security scope:
  mathematically faithful learning/reference code, not a side-channel-hardened
  production cryptography library.


- Extend the NME-native core: integer `%` modulo in arithmetic (float modulo is rejected honestly); conditions using `%` are a frontend follow-up.
- Fix the native backend so the very first string assignment can be a concatenation (`greeting = "hello" + " world"`): a C array cannot be initialized from a function call, so the emitter declares the buffer first and copies with strcpy.
- Extend the NME-native core: float literals, float variables, float arithmetic (mixed int/float promotes to double), and float comparisons.
- Extend the NME-native core: the beginner `times:` loop (block and inline forms) lowers to a C for-loop.
- Extend the NME-native core: boolean literals in truthy conditions (`if true`/`if false` lower to 1/0), alongside integer truthiness.
- Extend the NME-native core: truthy conditions (`if ready`, `while turns`) over integer values, so counters and flags work natively without comparisons.
- Add the natural-language `<=`/`>=` connectors: `if x is less than or equal to 3` and Korean `만약에 점수가 10보다 작거나 같으면` lower to `<=`/`>=` on both backends. The `or equal` phrase is kept out of logical-`or` splitting and typo recovery.
- Extend the NME-native core: `+` concatenation into string variables (fixed buffers via `strcpy`), so strings can be built up step by step; nested concatenation stays rejected.
- Extend the NME-native core: string `==`/`!=` comparisons through `strcmp` (both the Python condition form and the natural Korean form), a `len` builtin mapped to `strlen`, and string equality in sentence conditions.
- Extend the NME-native core: string variables (literals), string output, and one binary `+` concatenation through a small runtime helper, with nested concatenation honestly rejected; expressions now carry static types (int vs string) through lowering.
- Extend the NME-native core: functions over scalar parameters with `return` (recursion works), `else`/`else if` branches, calls in `say`, and honest rejection of C-keyword identifiers; the compiler now builds with `-O2`. Measured on this machine: a 50M-iteration integer loop is ~60x faster natively than on CPython (one micro-benchmark, documented in the memo).
- Implement the first slice of the NME-native AOT backend (`nme-native` crate + `nme native run`/`nme native build`): a restricted, statically typed core subset (integer values, sentence `while`/`if` over comparisons, `break`, `say`) lowers to C and compiles to a native executable with the system C compiler; anything outside the core is rejected with a clear bilingual diagnostic and still runs on CPython. Korean spellings work; end-to-end tests compile, run, and compare output.
- Add the bootstrap example (an NME program that transpiles a tiny language to Python and runs it) with a Korean twin, guide 29 on bootstrapping/self-hosting, and a CLI test that runs both.
- Add guide 25 (native compilation): teaches `nme native run`/`nme native build`, the documented core subset, functions and recursion, the C artifact, and the honest measured benchmark.
- Teach `nme install` in the READMEs and getting-started (guide 24).
- Add `nme install` / `nme 설치` as a friendly pip wrapper: it installs a Python package and tells the beginner the `import` line to use in an `.nme` file, with clear bilingual messages when pip is missing.
- Add the native-backend research memo (`docs/native-backend.md`): an honest evaluation of a C backend vs LLVM vs Cranelift vs direct codegen, recommending C for the first NME-native AOT compiler targeting a restricted statically-typed core subset, explicitly separated from the Python compatibility backend and from Nuitka.
- Add a `birthday.nme` countdown example that uses the `datetime` standard package from inside NME (with a Korean twin) and guide 24 on the standard library and pip-installed packages.
- Add `.nme` module imports: `from "helper.nme" import greet, score` imports
  only the listed names from a sibling `.nme` file, so a project can split
  into several files with an explicit interface and no shared global state.
  `nme run`/`check`/`build` transpile imported modules transitively and make
  them importable at runtime (via a temporary module folder on `sys.path`);
  module errors report the module's file name. File names must be Python
  identifiers, two modules may not share a name, and `nme compile` defers
  module support. Includes a two-file example pair (`examples/modules/`).
- Add an `http-client.nme` example that fetches a page from a local server
  with `urllib`, and a `terminal-menu.nme` TUI menu loop (both with Korean
  twins); a CLI test runs the menu with scripted input.
- Teach `nme convert` the file sentence forms: `x = open("f").read()` and
  `x = Path("f").read_text()` convert to `read "f" into x`, and
  `open("f", "w").write(v)` / `Path("f").write_text(v)` to
  `write v to "f"` (Korean spellings for Korean output). Beginner conversion
  keeps file IO as Python since the beginner file surface is `use file`; the
  converted sentence source round-trips through the compiler.
- Add four educational blockchain learning projects (learning only, never
  investment advice), each with a Korean twin: `blockchain-ledger.nme`
  (beginner, blocks linked by hashes), `proof-of-work.nme` (intermediate,
  mining with difficulty and a chain-integrity check), `signatures.nme`
  (advanced, HMAC signing and verification), and `consensus.nme` (expert, a
  two-node fork and longest-chain rule simulation).
- Add sentence-level file forms: `read "notes.txt" into memo`,
  `memo read "notes.txt"`, `memo에 "notes.txt" 읽어서 (저장해)`,
  `write "hello" to "out.txt"`, and `"out.txt" 파일에 "hello"를 저장해`
  lower to `pathlib` lines without the `use file` module. Read targets become
  known names for sentence interpolation, and weak matches like `read the
  book` or `write hello` stay plain sentence output.
- Bundle a `use file` / `파일 사용` module (version `0.0.1`) for reading,
  writing, and JSON, next to `use random`. One import exposes both
  vocabularies: `file_read`/`파일읽기`, `file_write`/`파일쓰기`,
  `json_load`/`json읽기`, `json_save`/`json저장`, plus version names. The
  `use` line parser is now shared by both modules (same latest/version forms,
  same collision protection, same diagnostics), and `nme modules` lists both.
  Sentence-level file wrappers are the next step.
- Extend stable error codes to command-line diagnostics: `nme ko <CODE>` and
  `nme en <CODE>` now also explain CLI errors (`E9001` unknown command,
  `E9015` missing program, `E9013` Python startup, ...). Compiler codes stay
  `E0001`+; CLI codes use the `E9xxx` range and render the same way
  (`error[E9015]:`). Every `fail()` path in the CLI now carries a code.
- Fix explicit `end`/`끝` block parsing when an indented sentence block is
  followed by a flat block: an indented body that cannot be closed by the
  remaining `end` lines now closes at the dedent, so `만약 ...` with an
  indented body followed by a flat `if ... end` block no longer reports a
  missing `end`. Every previously valid program keeps its exact output;
  nested headers with enough closing `end`s still stay nested, and a flat
  block still requires its own `end`.
- Give every compiler diagnostic a stable error code printed next to the
  message, e.g. `error[E0102]:`. `nme ko <CODE>` reads the long Korean
  explanation with an English translation, `nme en <CODE>` the English one,
  and `nme ko` (or `nme 에러` / `nme error`) with no code lists every code.
  Each code documents what went wrong, why, and the recovery steps; the code
  list and lookup pages are also taught in the help text, both READMEs, and
  both language references.
- Split the installation guide into independent per-OS sections (Windows 11,
  Windows 10, older Windows, macOS, Debian/Ubuntu, Fedora, Arch Linux), each
  with copy-paste install commands, PATH, version check, first run, and common
  errors.
- Start the 100-guide curriculum: `docs/guides/` now has an index (difficulty
  legend, learn-in-order path, topic lookup, full table) and the first twelve
  beginner guides (hello → ask → set → update → repeat → if → while → break →
  and/or → random → check/build → convert), each labeled with difficulty,
  prerequisites, topic, and result in both languages; every code block is
  verified with `nme check`.
- Accept shortened unique program names everywhere: `nme r gue` runs
  `guessing-game.nme`, and the same prefix rule works for `run`/`실행`,
  `check`/`검사`, `build`/`빌드`, `compile`, `convert`, the bare run shortcut
  (`nme gue`), and the numbered pick (bare names and prefixes answer the
  "Which one?" question). Case-insensitive exact stems win, then a unique
  prefix; when several programs match, NME lists the candidates and asks for
  more of the name instead of guessing.
- Long outputs (help, error-code lists) no longer panic when the reader
  closes the pipe early, e.g. `nme ko | head`.

## 0.0.1-beta.15 — 2026-08-11

- Accept the Korean `!=` sentence comparison `같지 않으면`, `같지 않다면`,
  `같지 않을` (also written `같지않으면` and friends), matching the existing
  English `is not equal to`.
- Fix `while` + Korean sentence condition + `동안` endings (for example
  `while 점수가 3보다 작을 동안`): the ending is now consumed as a block
  marker instead of being lowered as the loop's inline body, and every
  logical operand may carry its own ending (`while 점수가 10과 같지 않을
  동안 그리고 점수가 3보다 클 동안`).
- Fix Korean logical conditions: comparison endings may now combine with
  `그리고`/`또는` (`점수가 0보다 크면 그리고 점수가 3보다 작으면`), and
  malformed conditions report a diagnostic instead of crashing the parser.
- Fix the English roulette companion to use `ask number` for numeric menus,
  bets, and wheel picks.
- Add command shortcuts (`nme r`/`c`/`b`/`m`/`v`/`h`, `nme comp`/`nme conv`)
  and bare-file discovery: `nme r` runs the single `.nme` program in the
  current folder, lists and asks for a numbered pick when several exist, and
  explains what to do when none do.
- Add an English and Korean twin for every beginner example.
- Fix beginner-path documentation in both languages, close English/Korean
  parity gaps, and link the new examples from the tutorials.
- Teach the new shortcuts and show friendlier file hints in the CLI and both
  language guides.

## 0.0.1-beta.14 — 2026-08-11

- Track Python import bindings so the random adapter also protects names
  imported before `use random`.

## 0.0.1-beta.13 — 2026-08-11

- Refuse to load the bundled random adapter when its generated helper names
  would overwrite an existing value.

## 0.0.1-beta.12 — 2026-08-11

- Fix the indentation of the Korean beginner time-loop example so every
  published example passes `nme check`.

## 0.0.1-beta.11 — 2026-08-11

- Let compact `3 times:` / `3번:` beginner repeat blocks close with `end` / `끝`
  without requiring physical indentation.
- Accept the natural beginner spelling `repeat 3 times:` and keep ordinary
  colon-bearing Python suites on Python's indentation rules.
- Infer common age questions (`How old are you?`, `몇 살이에요?`) and accept
  spoken Korean loop endings such as `준비하는동안`.
- Treat polite show requests such as `Please show me hello` as the same simple
  output sentence instead of printing the request word.
- Document the sentence-to-beginner path with matching English and Korean
  flat-block examples.

## 0.0.1-beta.10 — 2026-08-11

- Recover common Korean condition-starter typos (`만악에`), spaced Korean
  particles/endings (`이름 이 철수 면`), and the spoken `그러면` connector
  without turning the right-hand value into text.
- Recover clear module typos such as `use random lates` and `랜덤 사요 최신`.
- Natural questions accept bare or separated targets (`나이 몇 살이에요`,
  `이름 은 뭐예요`) while preserving noun names that end in `이`.
- Korean `nme 버전` now prints Korean and English version information.

## 0.0.1-beta.9 — 2026-08-11

- Let a first program ask naturally with `What is your name?`, `What's your
  city?`, `이름이 뭐예요?`, or `나이는 몇 살이에요?` without `ask`, commas, or
  quotes; the final question mark is optional and `내 이름은 뭐예요?` is also
  understood.
- Accept target-first saves such as `name save Mina` and `이름 저장 민수`, and
  virtual-indent an ordinary Python `if`/`for` suite inside a flat NME block.
- Accept short Korean equality endings such as `이름이 철수면`, `이라면`, and
  `준비가 거짓이면`, plus bounded spoken typos such as `있으먄` and `철수먄`.
- Rewrite the first-run examples and tutorials around the sentence-to-Python
  learning bridge.

## 0.0.1-beta.8 — 2026-08-11

- Accept subject-first conversational conditions such as `color equals red
  then show yes` and their natural Korean equivalents, including flat `end`
  blocks.
- Recover common logical connector typos (`그리거`, `an`) and spoken Korean
  condition endings such as `같먄` without hijacking ordinary output sentences.
- Preserve future Python shapes that the bundled parser does not know yet,
  including CPython 3.14 t-strings, and add a Windows/macOS/Linux CI matrix.

## 0.0.1-beta.7 — 2026-08-11

- Make ordinary multiword sentences and contractions such as `Hello world!`
  and `I'm ready` print naturally without an output keyword.
- Recover common transposed action and condition typos, including `shwoe` and
  `thne`, while keeping ambiguous Python-shaped input untouched.
- Accept an unquoted comma prompt such as `ask name, What is your name?`.

## 0.0.1-beta.6 — 2026-08-11

- Add an easier sentence bridge for value changes (`add 1 to score`,
  `점수에 1 더해`) and repeat plain words without a colon or output marker.
- Accept spaced and attached Korean condition endings, polite sentence fillers,
  and explicit Korean beginner save words such as `저장` and `설정`.
- Refresh the first-run examples and tutorials so learners can move from
  sentences through beginner control flow into ordinary Python without a
  forced indentation jump.

## 0.0.1-beta.5 — 2026-08-11

- Make the three learning levels easier to mix inside one flat block, with
  regression coverage for Korean beginner spellings and ordinary Python.
- Accept attached Korean condition endings such as `이름있으면` and the natural
  `아니면만약에` branch spelling, plus small polite sentence fillers.
- Keep top-level Python identifiers such as `end` and `끝` untouched and avoid
  accidentally opening a colon-based Python block merely because a later NME
  block has an `end`.
- Refresh the bilingual language reference and local continuation handoff.

## 0.0.1-beta.4 — 2026-08-11

- Add an indentation-free control-flow bridge: `while`, `break`, `and`/`or`,
  `elif`/`else`, and `end`/`끝` can be mixed with sentence, beginner, and
  ordinary Python lines.
- Add Korean spellings for the new control-flow forms, virtual indentation for
  flat blocks, structural diagnostics, and regression examples.
- Expand the English/Korean learning path and AI handoff around growing from
  the easiest sentences into Python.

## 0.0.1-beta.3 — 2026-08-11

- Center NME on growing from ordinary sentences, through compact beginner
  syntax, into Python inside the same project.
- Add extensionless `nme run program`, `nme 실행 program`, and `nme program`
  commands with automatic platform Python selection.
- Make Korean CLI flows substantively bilingual while English flows remain
  English-only, including syntax messages, hints, and command failures.
- Make `check` and `build` validate generated source with CPython; failed
  builds never create an output file.
- Fix ambiguous action recovery, condition negation and literals, lexical
  scope leakage, Korean particles/actions, module validation, apostrophes in
  English sentences, and physical-line preservation.
- Make Python conversion conservative around calls, multiline statements,
  aliases, scopes, prompts, expressions, and ordinary `import random`.
- Fix Cargo PATH instructions for Fedora and package-manager installations.

## 0.0.1-beta.2 — 2026-08-10

- Add freely mixable advanced Python, compact beginner, and conversational
  sentence syntax in English and Korean.
- Add punctuation-light sentence input, output, assignment, repetition,
  conditions, numeric input, random integers, and random choices.
- Recover bounded one-character action-word typos and report ambiguous prose
  with a caret and repair hint.
- Add the locally versioned bilingual random adapter and module listing.
- Add safe Python-to-NME conversion for a chosen level and output language.
- Add optional standalone native compilation through an installed Nuitka.
- Add runnable greeting, number-guessing, three-level, and tiny-compiler
  examples plus matching bilingual tutorials and platform/editor guides.
- Preserve the Python-wins and line-preserving compiler contracts across the
  new syntax.

## 0.0.1-beta.1 — 2026-08-10

- Establish the first public beta version line.
- Add bilingual output, text input, repetition, and conditional syntax.
- Keep all valid Python source compatible and byte-identical.
- Provide ready-to-use English and Korean helpers backed by Python's bundled
  `random` module.
- Add matching English/Korean tutorials, exact language references, examples,
  and release policy documentation.
