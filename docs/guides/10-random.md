# 10 — Random: dice and picks

English | [한국어](10-random.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [06 — If](06-if.md)
- 주제 (Topic): 랜덤 / random
- 결과물 (Result): 주사위를 굴리고 색을 고르는 프로그램 / a program that rolls a die and picks a color

Games need surprises. NME bundles a random helper that Python provides, loaded
with one `use` line.

## Steps

1. Create `dice.nme`:

   ```text
   use random latest
   set die to random number from 1 to 6
   show die
   ```

   Run it a few times: `die` is a random whole number from 1 to 6.

2. Picking one of several choices works the same way:

   ```text
   use random latest
   set color to pick from red or green or blue
   show color
   ```

3. Korean loads the helper with `랜덤 사용 최신` and uses sentence endings:

   ```text
   랜덤 사용 최신
   주사위는 1부터 6까지 랜덤정수
   주사위 보여줘
   색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
   색 보여줘
   ```

4. The helper names work in expressions too: `랜덤정수(a, b)` is a random
   number and `랜덤선택(values)` picks from a list.

   ```text
   use random latest
   set roll to 랜덤정수(1, 6)
   show roll
   ```

## Try it yourself

Roll two dice and show both:

```text
랜덤 사용 최신
첫번째는 1부터 6까지 랜덤정수
두번째는 1부터 6까지 랜덤정수
첫번째 보여줘
두번째 보여줘
```

## What you learned

- `use random latest` / `랜덤 사용 최신` loads the bundled random helper.
- `random number from 1 to 6` / `1부터 6까지 랜덤정수` roll a die.
- `pick from red or green or blue` / `... 중에서 랜덤선택` pick one choice.
- Only one `use` line is allowed per program; run `nme modules` to list
  versions.
