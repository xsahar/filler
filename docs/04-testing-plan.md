# Testing Plan

Focus on the exact requirements from the subject.

## 1) Input parsing tests

Verify that parser converts stdin turn blocks into:
- board dimensions,
- board rows,
- piece dimensions,
- piece occupied offsets.

Use fixture text files in `tests/fixtures`.

## 2) Valid placement tests

For carefully crafted board states, test:
- exactly one overlap with own territory -> valid,
- zero own overlaps -> invalid,
- more than one own overlap -> invalid,
- any opponent overlap -> invalid.

## 3) Boundary tests

Test placements where piece would exceed:
- top boundary,
- left boundary,
- right boundary,
- bottom boundary.

All must be rejected.

## 4) Output format tests

Ensure formatter returns exactly:
- `X Y\n`

No extra text, tabs, or debug output.

## 5) Strategy regression tests

Given fixed states and pieces, assert expected chosen coordinate.
This prevents accidental strategy regressions.

## Test command

Use `cargo test` as the standard command in this project.
