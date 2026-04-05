# Architecture

This is a language-agnostic module layout.

## Data model

Suggested in-memory structures:
- `GameState`: board width/height, board cells, turn number, player symbols.
- `Piece`: width/height and occupied offsets.
- `Move`: x, y, score, valid flag.

## Modules

### `parser`
Responsibilities:
- parse player id line,
- parse anfield dimensions and rows,
- parse piece dimensions and rows,
- convert piece rows into occupied offsets.

Output:
- complete `GameState` and `Piece` for one turn.

### `board`
Responsibilities:
- classify cells: self, opponent, empty,
- validate candidate placement,
- enforce exactly one overlap with self,
- enforce zero overlap with opponent,
- enforce board boundaries.

Output:
- validity result for `(x, y)`.

### `strategy`
Responsibilities:
- iterate all candidate coordinates,
- keep only valid candidates,
- assign a score,
- choose best move.

Starter scoring ideas:
- minimize distance to opponent frontier,
- maximize future expansion space,
- avoid corner traps.

### `protocol`
Responsibilities:
- format output exactly as `X Y\n`.
- provide fallback move if no valid move exists.

### `app`
Responsibilities:
- main turn loop,
- call parser -> strategy -> protocol in order,
- handle EOF cleanly.

## Flow diagram

1. Read turn input.
2. Parse to structures.
3. Enumerate candidate moves.
4. Validate each candidate.
5. Score valid candidates.
6. Print best move.
