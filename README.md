## Filler

This project is a command-line robot for the Filler game engine.
The engine sends board + piece on stdin, and your bot must print coordinates on stdout.

## What You Must Implement

1. Parse every turn input correctly.
2. Validate placements with these rules:
- Exactly one overlap with your own territory.
- Zero overlap with opponent territory.
- Piece must stay inside board bounds.
3. Output one move in this exact format:
- X Y
4. Keep responding every turn, even when no valid move exists.

## Run The Project

Use WSL with Docker Desktop integration enabled. This repository is verified with the Linux engine layout.

1. Open a WSL shell and change into the repo:

```bash
cd /mnt/c/Users/Sahar/filler
```

2. Build the engine image from the bundled `engine_files` folder:

```bash
docker build -t filler-local ./engine_files
```

3. Build the player binary inside Docker:

```bash
docker run --rm --entrypoint /bin/bash -v "$(pwd)":/filler/solution filler-local -lc '
set -e
export PATH="/usr/local/cargo/bin:$PATH"
cd /filler/solution
rm -f Cargo.lock
cargo build --release
'
```

4. Start the engine container with the repo mounted as `/filler/solution`:

```bash
docker run --rm -it -v "$(pwd)":/filler/solution filler-local
```

5. Inside the container, run the audit helper against the built player:

```bash
cd /filler
bash /filler/solution/scripts/audit_runner.sh /filler/solution/target/release/filler
```

If you need a manual smoke test, run one match directly:

```bash
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
```

Note: some official packages use `game_engine`/`robots`, while this repository uses `linux_game_engine`/`linux_robots`.

## Required Audit Checks

1. Container/image creation works.
2. Player runs without runtime failure.
3. Placement logic is correct.
4. Win rate at least 4/5 for:
- map00 vs wall_e
- map01 vs h2_d2
- map02 vs bender
5. Unit tests pass for:
- parsing
- placement validation
- boundaries
- output format

## Tests

Run:

```bash
cargo test
```

## Documentation For Auditors

- [docs/README.md](docs/README.md): full documentation index.
- [docs/03-step-by-step-plan.md](docs/03-step-by-step-plan.md): exact command sequence to run build, matches, and checks.
- [docs/05-audit-readiness.md](docs/05-audit-readiness.md): audit checklist and readiness notes.
- [docs/07-audit-results.md](docs/07-audit-results.md): recorded pass/fail outcomes.

Quick auditor path:
1. [docs/03-step-by-step-plan.md](docs/03-step-by-step-plan.md)
2. [docs/05-audit-readiness.md](docs/05-audit-readiness.md)
3. run commands and record outcomes
4. compare with [docs/07-audit-results.md](docs/07-audit-results.md)

## Auditor Checklist (Fast Version)

Use this quick list if you need a pass or fail decision fast:

1. Confirm Docker is available on host machine.
2. Confirm the official engine package is extracted and includes maps and robots.
3. Build container image successfully.
4. Start container successfully with mounted solution folder.
5. Build student player inside container.
6. Run one non-quiet match to verify no runtime crash.
7. Run all required quiet match matrix commands.
8. Verify each required map set reaches at least 4 wins out of 5.
9. Run tests and confirm full pass.
10. Compare results with [docs/07-audit-results.md](docs/07-audit-results.md).

## Pass Criteria Interpretation

This section clarifies how to interpret the audit rules:

1. Container setup is PASS if image build and interactive container start both succeed.
2. Runtime behavior is PASS if player does not exit early and engine does not report EOF or broken pipe for valid runs.
3. Placement correctness is PASS if runtime matches proceed normally and placement tests pass.
4. Win-rate rule is PASS per map when wins are greater than or equal to 4 in 5 runs.
5. Unit test section is PASS when parser, placement, boundary, and output format tests all pass.

## Evidence To Keep For Review

To make review easy, keep these outputs:

1. Output of container build command.
2. Output of first full non-quiet match.
3. Output summaries for all 15 required quiet runs.
4. Output of test command.
5. Final result summary copied into [docs/07-audit-results.md](docs/07-audit-results.md).

## Engine Naming Variants

Official packages can differ by naming:

1. Variant A uses game_engine and robots.
2. Variant B uses linux_game_engine and linux_robots.

This repository was verified with the linux naming variant.
If your package uses the other variant, use equivalent commands with those names.

## Common Issues And Fixes

1. Docker command not found on WSL:
- install and start Docker Desktop, then enable WSL integration.
- if you are in MSYS/Git Bash, switch to WSL or another terminal where `docker` is on `PATH`.

2. Docker permission denied on socket:
- add user to docker group, reopen shell, verify with docker info.

3. Player exits with EOF at runtime:
- build the player inside the same container used by the engine.

4. Cargo lock format mismatch in container:
- remove Cargo.lock in container and regenerate by running cargo build there.

## Final Notes For Auditors

1. This README is intentionally short and review-focused.
2. The full implementation details remain in docs and source modules.
3. For reproducibility, always follow the command order in [docs/03-step-by-step-plan.md](docs/03-step-by-step-plan.md).
4. For scoring decisions, use the required map/opponent checks listed in [Audit.md](Audit.md).
