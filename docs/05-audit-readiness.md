# Audit Readiness

This file maps the project to the audit checklist in Audit.md.

## Functional checks

1. Container and engine launch
- Run inside the provided filler docker container.
- Sanity command:
  - ./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator

2. Student player execution
- Build player binary:
  - cargo build --release
- Example run versus a robot:
  - ./game_engine -f maps/map00 -p1 solution/target/release/filler -p2 robots/wall_e

3. Placement correctness
- Core rule enforcement is in src/board/mod.rs.
- Valid move requires exactly one overlap with own territory and zero overlap with opponent.

4. Win-rate checks
- Must be measured in the engine environment with repeated runs and swapped positions.
- Required scenarios from audit:
  - map00 vs wall_e, 5 runs, at least 4 wins.
  - map01 vs h2_d2, 5 runs, at least 4 wins.
  - map02 vs bender, 5 runs, at least 4 wins.

## Unit test checks

- Test command:
  - cargo test
- Covered now:
  - Input parsing tests in tests/parser_tests.rs and tests/stream_parser_tests.rs.
  - Placement validation and boundary tests in tests/board_tests.rs.
  - Output format tests in tests/protocol_tests.rs.

## Known gap for full audit confirmation

Win-rate and container checks cannot be confirmed until the official game_engine, maps, and robots are run in docker.
