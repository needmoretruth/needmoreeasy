# 52 — Game: tic-tac-toe

English | [한국어](52-tic-tac-toe.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [51 — Grid](51-grid.md), [26 — Adventure](26-adventure.md)
- 주제 (Topic): 게임 / game
- 결과물 (Result): 승리 확인이 있는 두 명이 하는 틱택토 / a playable two-player tic-tac-toe with a win check

Guide [51](51-grid.md) built a board; guide [26](26-adventure.md) showed how `ask`, `while True`, and `break` make a game loop. Tic-tac-toe joins them: two players take turns picking a row and a column, and after every move a function checks for three in a row.
## Steps
1. The whole game in one file, `tictactoe.nme`. The board and `show_board` come from [51](51-grid.md); `check_winner` scans rows, columns, and both diagonals and returns the winning mark, or `None` while no line is complete; `ask number` reads a row and a column as integers, `continue` re-asks a taken square, and `moves == 9` declares a draw:
   ```text
   # tictactoe.nme — a two-player tic-tac-toe game.
   # Run: nme r tictactoe
   # The board is a list of rows (guide 51). A turn places a mark,
   # and check_winner looks for three in a row, column, or diagonal.

   board = [["-", "-", "-"], ["-", "-", "-"], ["-", "-", "-"]]

   def show_board(board):
       for row in board:
           show row

   def check_winner(board):
       # Rows: three equal marks side by side.
       for row in board:
           if row[0] == row[1] and row[1] == row[2] and row[0] != "-":
               return row[0]
       # Columns: three equal marks straight down.
       for col in range(3):
           if board[0][col] == board[1][col] and board[1][col] == board[2][col] and board[0][col] != "-":
               return board[0][col]
       # Diagonals: corner to corner.
       if board[0][0] == board[1][1] and board[1][1] == board[2][2] and board[0][0] != "-":
           return board[0][0]
       if board[0][2] == board[1][1] and board[1][1] == board[2][0] and board[0][2] != "-":
           return board[0][2]
       return None

   player = "X"
   moves = 0
   show "Tic-tac-toe! Rows and columns are 0, 1, or 2."
   show "Line up three marks to win. X goes first."

   while True:
       show_board(board)
       ask number row, f"Player {player}, row (0-2): "
       ask number col, f"Player {player}, column (0-2): "
       if board[row][col] != "-":
           show "That square is taken. Pick another."
           continue
       board[row][col] = player
       moves = moves + 1
       winner = check_winner(board)
       if winner is not None:
           show f"After {moves} moves:"
           show_board(board)
           show f"Player {winner} wins!"
           break
       if moves == 9:
           show f"After {moves} moves:"
           show_board(board)
           show "It is a draw."
           break
       if player == "X":
           player = "O"
       else:
           player = "X"
   ```
2. Run a whole game through a pipe — row then column for five moves. X fills a row and wins on the last move:
   ```sh
   printf '0\n0\n1\n0\n0\n1\n2\n0\n0\n2\n' | nme r tictactoe
   ```
   ```text
   Tic-tac-toe! Rows and columns are 0, 1, or 2.
   Line up three marks to win. X goes first.
   ['-', '-', '-']
   ['-', '-', '-']
   ['-', '-', '-']
   Player X, row (0-2): Player X, column (0-2): ['X', '-', '-']
   ['-', '-', '-']
   ['-', '-', '-']
   Player O, row (0-2): Player O, column (0-2): ['X', '-', '-']
   ['O', '-', '-']
   ['-', '-', '-']
   Player X, row (0-2): Player X, column (0-2): ['X', 'X', '-']
   ['O', '-', '-']
   ['-', '-', '-']
   Player O, row (0-2): Player O, column (0-2): ['X', 'X', '-']
   ['O', '-', '-']
   ['O', '-', '-']
   Player X, row (0-2): Player X, column (0-2): After 5 moves:
   ['X', 'X', 'X']
   ['O', '-', '-']
   ['O', '-', '-']
   Player X wins!
   ```
   The last board shows the winning row: `X` across the top. The game ends with `break`, so the winning move is the last move.
## Try it yourself
Play a game O wins and one that ends in a draw, then make `check_winner` also name the winning line, such as `"row 0"`.
## What you learned
- `ask number` reads a row and column as integers; `board[row][col]` writes a mark.
- A function checks rows, columns, and diagonals for three equal marks.
- `continue` rejects a taken square; `break` ends the game.
- `moves == 9` with no winner declares a draw.
