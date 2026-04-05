# Step-by-Step Plan

## Milestone 1: Input parsing only

Deliverables:
- parse one full turn into structures,
- print debug summary to confirm fields.

Checklist:
- read player id line,
- read anfield header and rows,
- read piece header and rows,
- map symbols correctly for p1/p2.

Done when:
- parser test fixtures pass,
- no off-by-one row/column bugs.

## Milestone 2: Placement validator

Deliverables:
- function `isValidMove(state, piece, x, y)`.

Validation rules:
- piece occupied cells only are checked,
- occupied cells must remain inside board,
- overlaps with opponent must be zero,
- overlaps with own territory must be exactly one.

Done when:
- positive and negative unit tests pass.

## Milestone 3: Baseline strategy

Deliverables:
- enumerate all `(x, y)` top-left candidates,
- select first valid move.

Done when:
- robot plays complete games without protocol errors.

## Milestone 4: Better scoring strategy

Deliverables:
- score-based move selection,
- compare against baseline by running several matches.

Done when:
- win rate against weaker robots is clearly improved.

## Milestone 5: Hardening

Deliverables:
- strict output formatting,
- graceful fallback when no move exists,
- lightweight logging mode (disabled in normal engine run).

Done when:
- no crashes/timeouts in repeated runs.
