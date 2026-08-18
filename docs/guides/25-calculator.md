# 25 — Calculator — a command-line project

English | [한국어](25-calculator.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [22 — Terminal menu](22-terminal-menu.md), [08 — If](08-if.md)
- Topic: a project
- Result: a repeat-until-quit calculator with functions and a module file

A calculator that reads `3 + 4` and answers, then asks again until you type
`quit`, is a complete small project. It uses all three levels in one program:
beginner `ask` for input, plain Python `while True:` and `def` for the loop and
the math, and NME `show` for output.

## Steps

1. Save the whole calculator in one file, `calculator.nme`, and run it:

   ```nme
   # A command-line calculator: type 3 + 4, or quit.
   # Run: nme r calculator

   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])


   show "Calculator — type a command like 3 + 4, or quit."

   while True:
       ask command, "Your command? "
       if command == "quit":
           show "Bye!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           show f"{command} = {answer}"
       else:
           show "Use the form: number operator number"
   ```

   ```sh
   printf '3 + 4\n10 - 3\n7 * 6\n10 / 4\nquit\n' | nme r calculator
   ```

   ```text
   Calculator — type a command like 3 + 4, or quit.
   Your command? 3 + 4 = 7
   Your command? 10 - 3 = 7
   Your command? 7 * 6 = 42
   Your command? 10 / 4 = 2.5
   Your command? Bye!
   ```

2. The math lives in a function. `def` names it, `parts` holds the split
   command, and `return` sends the answer back. `parts[1]` is the operator and
   `int(parts[0])` turns the text `"3"` into the number `3`:

   ```nme
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])
   ```

   `elif` chains one branch per operator; the last `else` is division. This
   block is advanced Python written directly inside the NME file.

3. `split()` cuts the command into words. `"3 + 4".split()` becomes
   `['3', '+', '4']`, so `parts[0]` is the first number, `parts[1]` the
   operator, `parts[2]` the second number. `len(parts) == 3` rejects anything
   that is not a well-formed command:

   ```nme
   parts = command.split()
   if len(parts) == 3:
       answer = calculate(parts)
       show f"{command} = {answer}"
   else:
       show "Use the form: number operator number"
   ```

   `show f"{command} = {answer}"` prints the line you typed and the result on
   one row.

4. The loop is the same shape as the menu in [22](22-terminal-menu.md):
   `while True:` never ends on its own, so `quit` must `break` out. Everything
   between is one turn of the calculator:

   ```nme
   while True:
       ask command, "Your command? "
       if command == "quit":
           show "Bye!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           show f"{command} = {answer}"
       else:
           show "Use the form: number operator number"
   ```

5. Once the function works, move it into its own module as guide
   [61](61-modules.md) explains. Save the function in `calc.nme`:

   ```nme
   # calc.nme — the calculate function only
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])
   ```

   Then import it at the top of `calculator.nme`. The main file now only
   describes the loop; the math stays in `calc.nme`:

   ```nme
   from "calc.nme" import calculate

   show "Calculator — type a command like 3 + 4, or quit."

   while True:
       ask command, "Your command? "
       if command == "quit":
           show "Bye!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           show f"{command} = {answer}"
       else:
           show "Use the form: number operator number"
   ```

   `nme check calculator` checks both files, and `nme r calculator` runs the
   import just like before.

6. The Korean twin `calculator.ko.nme` keeps the same `def` and writes the
   loop with `물어봐`, `만약`, and `말해`:

   ```nme
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])


   while True:
       물어봐 command, "명령을 입력하세요? "
       만약 command == "quit":
           말해 "안녕!"
           break
       parts = command.split()
       answer = calculate(parts)
       말해 f"{command} = {answer}"
   ```

   The same piped input produces the same answers in both languages.

## Try it yourself

Add a fifth operator: change the `else` branch so `//` means floor division
(`int(parts[0]) // int(parts[2])`), then test `17 // 5`. Or make the unknown
operator a friendly message instead of dividing.

## What you learned

- `def`/`return` package the math into one reusable function.
- `command.split()` cuts a line into parts; `int(parts[0])` reads a number.
- `while True:` with `break` on `quit` makes a repeat-until-quit loop.
- Moving the function to a `.nme` module keeps the project clean.
