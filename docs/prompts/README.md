# Prompts to hand to an AI

English | [한국어](README.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Syntax list](../syntax.md)

Each file here is one document you **copy whole and paste at the start of a chat
with an AI**, so that the assistant can write NME programs even though it has
never seen the language. They work in an ordinary chat window — ChatGPT, Claude,
anything. No agent, no tool configuration.

After pasting, just ask:

> Write me an NME program that prints the five times table

## Which one to use

| File | What is in it | Length | Use it when |
| --- | --- | --- | --- |
| [Sentence level (recommended)](nme-sentence.md) | 100% of the sentence syntax + short examples | shortest | **You are new to programming. This is usually all you need.** |
| [Full syntax](nme-all-levels.md) | sentence + beginner + advanced + modules + error codes | medium | Working with someone who knows Python, or the sentence level cannot say it |
| [Full syntax with examples](nme-complete.md) | all of the above + 12 complete programs | longest | If it fits in the chat window, this gives the most accurate results |

All three are written **around the sentence level**, because the reader we
designed them for is someone learning to program for the first time.

## How they are made

All three are generated from the compiler source
(`python scripts/build-ai-prompts.py`). The tables are what the compiler
actually accepts, and every example program is **compiled by the real compiler**
each time the files are written, with its exact output pasted in. If one stops
compiling, the build fails — so these documents cannot lie.

## Checking the answer

A program the AI writes can be pasted straight into **needmoreeasy.com** and
run. Nothing to install, and it works on a phone. If the result looks wrong,
paste the error message back to the AI: every message carries a stable number
such as `E0102`, which tells the AI exactly what went wrong.
