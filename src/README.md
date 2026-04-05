# Source Layout

This folder follows a Rust module layout.

## Modules

- `app`: entrypoint and game loop.
- `parser`: reads engine input and builds turn structures.
- `board`: validates candidate placements against rules.
- `strategy`: searches and scores candidate moves.
- `protocol`: output formatting (`X Y\n`).

## Rust entry files

- `main.rs`: binary entrypoint.
- `lib.rs`: exports modules for tests and binary.
- `model.rs`: shared data structures.

## Suggested dependency direction

- `app` depends on all other modules.
- `strategy` depends on `board` and shared model types.
- `parser` and `protocol` stay independent from strategy rules.

Keep parsing/validation deterministic so they are easy to test.
