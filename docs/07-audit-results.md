# Filler Audit Results

Date: 2026-04-05

## Functional

### Image and container setup

Status: PASS

Evidence:
- Docker is installed and available (`docker --version`, `docker info`).
- Official engine image was built and container was started successfully.

### Student player runs correctly

Status: PASS

Evidence:
- Player binary built inside the container at `/filler/solution/target/release/filler`.
- Engine completed games without EOF/pipe errors after in-container build.

### Correct piece placement (exactly one self-overlap)

Status: PASS

Evidence:
- Runtime games complete successfully with valid moves.
- Placement rules are enforced by validator tests and pass.

### Required win-rate checks

#### map00 vs wall_e (5 runs, side swapped)

Status: PASS

Result:
- Wins: 5/5

#### map01 vs h2_d2 (5 runs, side swapped)

Status: PASS

Result:
- Wins: 5/5

#### map02 vs bender (5 runs, side swapped)

Status: PASS

Result:
- Wins: 5/5

## Unit Tests

### All tests pass

Status: PASS

Result:
- `cargo test` passed.

### Input parsing tests exist

Status: PASS

Evidence:
- `tests/parser_tests.rs`
- `tests/stream_parser_tests.rs`

### Placement validation tests exist

Status: PASS

Evidence:
- `tests/board_tests.rs`

### Boundary detection tests exist

Status: PASS

Evidence:
- Out-of-bounds cases covered in `tests/board_tests.rs`.

### Coordinate output format tests exist

Status: PASS

Evidence:
- `tests/protocol_tests.rs`

## Basic

### Good practices

Status: PASS (subjective checklist item)

Evidence:
- Modular Rust structure (`parser`, `board`, `strategy`, `protocol`, `app`).
- Deterministic core logic and dedicated tests.

### Test files present

Status: PASS

Evidence:
- Multiple test files under `tests/`.

### Tests cover key cases

Status: PASS

Evidence:
- Parser, overlap rules, opponent overlap rejection, boundaries, and output formatting are all explicitly tested.

## Bonus

### Visualizer

Status: NOT IMPLEMENTED

### vs terminator (optional)

Status: NOT EXECUTED

## Notes

- The provided engine image uses an older Rust toolchain. To compile reliably inside that image, use container build flow and regenerate `Cargo.lock` there if needed.
- Final audit runs were executed using:
  - Engine: `./linux_game_engine`
  - Robots: `linux_robots/*`
  - Player: `/filler/solution/target/release/filler`

## Reproduction Commands

Use these commands if you want to reproduce the audit exactly.

### Host setup

1. `cd /mnt/c/Users/xs7ar/Downloads/filler/docker_image`
2. `mkdir -p solution/bin solution/scripts`
3. `cp /mnt/c/Projects/filler/scripts/audit_runner.sh solution/scripts/audit_runner.sh`
4. `docker build -t filler .`
5. `docker run -v "$(pwd)/solution":/filler/solution -it filler`

### Inside the container

6. `cd /filler/solution`
7. `rm -f Cargo.lock`
8. `cargo build --release`
9. `cd /filler`
10. `./linux_game_engine -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e`

### Required match matrix

11. `./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e`
12. `./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler`
13. `./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e`
14. `./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler`
15. `./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e`

16. `./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2`
17. `./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler`
18. `./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2`
19. `./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler`
20. `./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2`

21. `./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender`
22. `./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler`
23. `./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender`
24. `./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler`
25. `./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender`

### Unit tests

26. `cargo test`
