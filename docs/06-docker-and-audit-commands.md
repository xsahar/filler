# Docker And Audit Commands

This project now includes scripts to build and run the player for audit scenarios.

## Option A: Build player on host (WSL/Linux)

1. Build and export binary:
- ./scripts/build_player.sh

Output:
- solution/bin/filler

## Option B: Build player using Docker

1. Build and export Linux binary through Docker:
- ./scripts/docker_build_player.sh

Output:
- solution/bin/filler

## Run in official filler container

From the folder that contains the official `game_engine`, `maps`, and `robots`:

1. Build and start official container (from provided engine package):
- docker build -t filler .
- docker run -v "$(pwd)/solution":/filler/solution -it filler

2. Inside container, verify player path is executable:
- ls -l /filler/solution/bin/filler

3. Quick single match:
- ./game_engine -f maps/map00 -p1 /filler/solution/bin/filler -p2 robots/wall_e

## Run full audit win-rate matrix

Inside the same container and engine directory:

- /filler/solution/scripts/audit_runner.sh /filler/solution/bin/filler

This runs:
- map00 vs wall_e, 5 runs with side swapping
- map01 vs h2_d2, 5 runs with side swapping
- map02 vs bender, 5 runs with side swapping

It prints PASS/FAIL against >= 4/5 and writes logs to `audit_logs/`.

## Unit tests

- cargo test
