# Project Breakdown

## What you are building

You are not building the game engine.
You are building one robot program that the engine runs.

The engine sends:
- player identity,
- full board state,
- current piece.

Your robot sends back:
- exactly one line: `X Y` followed by a newline.

## Runtime model

Each turn:
1. Read input from standard input.
2. Parse board and piece.
3. Compute a valid coordinate.
4. Print coordinate to standard output.
5. Flush output and wait for next turn.

If no valid move exists, print fallback coordinates (commonly `0 0`).

## Success criteria

1. No crashes or malformed output.
2. Valid move rules are always respected.
3. Boundaries are never exceeded.
4. Unit tests cover parser + placement rules.
5. Strategy beats weaker sample robots consistently.

## Terminal vs browser

- Required: terminal workflow (Docker + CLI).
- Optional bonus: browser visualizer.

So the core project is terminal-based.
