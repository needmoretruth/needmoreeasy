# 51 — Grid: a board of lists

English | [한국어](51-grid.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [50 — Strings](50-strings.md), [16 — Name list](16-name-list.md)
- Topic: nested data
- Result: a tic-tac-toe style 3x3 board stored as a list of lists, reading and writing cells with `board[row][col]`

A list can hold lists. The outer list is the board, each inner list is one
row, and every cell is a text mark. Reading and writing a cell uses two
indexes — first the row, then the column.
## Steps
1. A 3x3 board is three rows, each a list of three marks. `board[0]` is the first row, `board[1]` the second. Reading a cell needs both indexes — row first, column second — and writing uses the same with `=`. All indexes start at `0`, so `board[0][0]` is the top-left corner:
   ```text
   board = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]]
   show board[0]
   board[0][0] = "X"
   board[1][2] = "O"
   show board[0][0]
   show board[1][2]
   ```
   It prints the first row, then `X` and `O`. A `-` means an empty cell.
2. A `for` loop visits every row, so printing a whole board is three short lines:
   ```text
   board = [["X", "O", "-"], ["-", "-", "-"], ["-", "-", "-"]]
   for row in board:
       show row
   ```
   The loop reads the outer list top to bottom; each `row` is one inner list.
3. Now a program that builds the board, prints it, places marks, reads them back, and counts them. The counting loop uses a second, inner loop so every cell of every row is visited. Save it as `grid.nme`:

   ```text
   # grid.nme — a 3x3 board made of lists.
   # Run: nme r grid
   # A grid is a list of rows; each row is a list of cells.

   board = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]]

   def show_board(board):
       for row in board:
           show row

   show "Empty board:"
   show_board(board)

   board[0][0] = "X"
   board[0][1] = "O"
   board[1][1] = "X"
   board[2][2] = "X"

   show "After four marks:"
   show_board(board)

   show f"Top-left corner: {board[0][0]}"
   show f"Center: {board[1][1]}"
   show f"Bottom-right corner: {board[2][2]}"
   show f"Row 0, column 2 is empty: {board[0][2]}"

   board[1][1] = "-"
   show "Center cleared, then filled again:"
   board[1][1] = "O"
   show_board(board)

   x_count = 0
   o_count = 0
   for row in board:
       for cell in row:
           if cell == "X":
               x_count = x_count + 1
           if cell == "O":
               o_count = o_count + 1
   show f"The board holds {x_count} X marks and {o_count} O marks"
   ```

4. Run it:
   ```sh
   nme r grid
   ```
   ```text
   Empty board:
   ['-', '-', '-']
   ['-', '-', '-']
   ['-', '-', '-']
   After four marks:
   ['X', 'O', '-']
   ['-', 'X', '-']
   ['-', '-', 'X']
   Top-left corner: X
   Center: X
   Bottom-right corner: X
   Row 0, column 2 is empty: -
   Center cleared, then filled again:
   ['X', 'O', '-']
   ['-', 'O', '-']
   ['-', '-', 'X']
   The board holds 2 X marks and 2 O marks
   ```
   Writing then reading back a cell gives exactly what was stored — that is the whole point of a grid. Guide [52](52-tic-tac-toe.md) turns this board into a playable game.
## Try it yourself

Place marks so the first column reads `X` top to bottom, print the board, and read `board[2][0]` back. Then make the counting loop print how many cells are still empty.
## What you learned
- A grid is a list of rows, and each row is a list of cells.
- `board[row][col]` reads a cell; `board[row][col] = "X"` writes one.
- `for row in board:` visits each row; a second loop visits each cell.
- A `-` marks an empty cell, so the board can tell a mark from a gap.
