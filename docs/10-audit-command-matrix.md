# Audit Command Matrix

Use these commands to run the required audit matches on your machine.

## map00 vs wall_e

Run these five commands:

```bash
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
```

## map01 vs h2_d2

Run these five commands:

```bash
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2
./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2
./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2
```

## map02 vs bender

Run these five commands:

```bash
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
```

## Required check summary

- map00: at least 4/5 wins
- map01: at least 4/5 wins
- map02: at least 4/5 wins
- `cargo test` passes
- the quick sanity match runs without EOF or broken pipe errors
