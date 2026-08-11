# 26 — Adventure — a small text game

English | [한국어](26-adventure.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [22 — Terminal menu](22-terminal-menu.md), [06 — If](06-if.md), [10 — Random](10-random.md)
- 주제 (Topic): 대형 프로젝트 / a small project
- 결과물 (Result): 방을 이동하며 고르는 텍스트 모험 / a room-by-room text adventure with choices

A text adventure describes a place and asks what you do next. Everything from
the earlier guides comes together here: `ask` reads your choices, `if`/`elif`/
`else` routes them, a list is your inventory, and `use random` lets dice decide
a risky room. The whole game fits in one `.nme` file.

## Steps

1. Save the whole game in one file, `adventure.nme`:

   ```text
   # A small text adventure: walk through a cave and find the gold.
   # Run: nme r adventure

   use random latest

   inventory = []

   show "You wake up in a dark cave. A single torch flickers."

   while True:
       ask action, "What now? (look, east, dice, quit) "
       if action == "quit":
           show "You leave the cave. Goodbye!"
           break
       elif action == "look":
           show "Entrance cave: damp stone and a tunnel leading east."
           if "torch" not in inventory:
               show "You take the torch from the wall."
               inventory.append("torch")
           else:
               show "The walls are bare and quiet."
       elif action == "east":
           show "You follow the tunnel into a stone room."
           if "key" not in inventory:
               show "A rusty key lies on a shelf. You take it."
               inventory.append("key")
           else:
               show "The shelf is empty now."
           show "A locked door blocks the north wall. A passage goes west."
           ask move, "Go north, west, or back? "
           if move == "north":
               if "key" in inventory and "rope" in inventory:
                   show "The key turns in the lock. The door opens!"
                   show "Beyond it: a chest full of gold. You win!"
                   show f"You collected: {inventory}"
                   break
               else:
                   show "The door is locked. The key and the rope are needed."
           elif move == "west":
               show "The passage opens into a small river room."
               if "rope" not in inventory:
                   show "A coil of rope lies by the water. You take it."
                   inventory.append("rope")
               else:
                   show "You already have the rope."
               roll = random_number(1, 6)
               show f"You try to cross the stream. Dice roll: {roll}."
               if roll >= 4:
                   show "You splash across safely and find a coin."
                   inventory.append("coin")
               else:
                   show "The water is too deep. You turn back, dry."
           else:
               show "You step back to the entrance cave."
       elif action == "dice":
           roll = random_number(1, 6)
           show f"The dice tumble: {roll}."
           if roll >= 4:
               show "Lucky! A coin shines in the dust. You pocket it."
               inventory.append("coin")
           else:
               show "Nothing happens. The cave stays quiet."
       else:
           show "I do not understand. Try look, east, dice, or quit."
   ```

2. Run it. The piped input plays one whole game: go east, try the locked
   door, go east again, take the rope, roll the dice, then come back and open
   the door:

   ```sh
   printf 'east\nnorth\neast\nwest\ndice\neast\nnorth\n' | nme r adventure
   ```

   ```text
   You wake up in a dark cave. A single torch flickers.
   What now? (look, east, dice, quit) You follow the tunnel into a stone room.
   A rusty key lies on a shelf. You take it.
   A locked door blocks the north wall. A passage goes west.
   Go north, west, or back? The door is locked. The key and the rope are needed.
   What now? (look, east, dice, quit) You follow the tunnel into a stone room.
   The shelf is empty now.
   A locked door blocks the north wall. A passage goes west.
   Go north, west, or back? The passage opens into a small river room.
   A coil of rope lies by the water. You take it.
   You try to cross the stream. Dice roll: 1.
   The water is too deep. You turn back, dry.
   What now? (look, east, dice, quit) The dice tumble: 6.
   Lucky! A coin shines in the dust. You pocket it.
   What now? (look, east, dice, quit) You follow the tunnel into a stone room.
   The shelf is empty now.
   A locked door blocks the north wall. A passage goes west.
   Go north, west, or back? The key turns in the lock. The door opens!
   Beyond it: a chest full of gold. You win!
   You collected: ['key', 'rope', 'coin']
   ```

   The two dice lines are random — your run will show different rolls.

3. The game world is one endless loop with a small menu, exactly like the
   terminal menu from [22](22-terminal-menu.md). `while True:` never stops by
   itself, so the only way out of a room is answering `quit` or winning:

   ```text
   while True:
       ask action, "What now? (look, east, dice, quit) "
       if action == "quit":
           show "You leave the cave. Goodbye!"
           break
       elif action == "east":
           show "You follow the tunnel into a stone room."
           ask move, "Go north, west, or back? "
           if move == "north":
               show "You open the door."
           elif move == "west":
               show "You find the river room."
           else:
               show "You step back to the entrance cave."
   ```

   Each `ask` stores an answer, and the `if` chain picks one branch. The inner
   `ask move` gives the stone room its own small menu.

4. The inventory is a list. `inventory.append("key")` adds an item, and the
   check `"key" not in inventory` makes sure you can take an item only once:

   ```text
   inventory = []
   if "key" not in inventory:
       inventory.append("key")
       show "You take the key."
   ```

   `show f"You collected: {inventory}"` prints the whole list on the winning
   line.

5. Dice decide risky rooms. `use random latest` loads `random_number(a, b)`,
   which rolls between `a` and `b`, and `if roll >= 4` splits success from
   failure:

   ```text
   use random latest
   roll = random_number(1, 6)
   show f"The dice tumble: {roll}."
   if roll >= 4:
       show "Lucky! You find a coin."
       inventory.append("coin")
   else:
       show "Nothing happens."
   ```

   Because the roll is random, no two games play the same — that is the point
   of a dice encounter.

6. The win condition combines two collected items. `and` joins the checks from
   guide [09](09-and-or.md): the door opens only when both the key and the
   rope are in the inventory, and then `break` leaves the loop:

   ```text
   while True:
       ask move, "Go north, west, or back? "
       if move == "north":
           if "key" in inventory and "rope" in inventory:
               show "The key turns in the lock. The door opens!"
               show f"You collected: {inventory}"
               break
           else:
               show "The door is locked. The key and the rope are needed."
       else:
           break
   ```

   The Korean twin `adventure.ko.nme` writes the same game with `랜덤 사용
   최신`, `물어봐`, `보여줘`, and `랜덤정수(1, 6)`; the answers `east`/`north`/
   `west` stay the same, so the same piped input wins in both languages.

## Try it yourself

Add a fourth direction: give the river room a `south` choice that leads to a
new "echo cave" with its own item, then make the winning door require that item
too. Add the choice to the menu line and a new `elif action == "south":`
branch.

## What you learned

- A loop with an `ask` menu turns text into a small game world.
- A list is an inventory: `append` collects, `in`/`not in` checks.
- `use random` makes a dice encounter that differs on every run.
- The win condition joins items with `and` and leaves the loop with `break`.
