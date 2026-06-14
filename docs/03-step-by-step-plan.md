# Exact Steps To Follow

Use this as the only run guide.

## 1. Open project in WSL

Use a WSL shell or another terminal where Docker Desktop is available on `PATH`. Do not use MSYS/Git Bash for these commands.
If Docker Desktop says you have no WSL 2 distros installed, install or convert a WSL 2 distro first, then enable WSL integration for that distro.

```bash
cd /mnt/c/Users/Sahar/filler
```

## 2. Check that engine files exist in this project

```bash
ls -1 engine_files
ls -1 engine_files/maps
```

You should see `linux_game_engine`, `linux_robots`, `maps`, and map files `map00`, `map01`, `map02`.

## 3. Build engine image from local engine_files

```bash
docker build -t filler-local ./engine_files
```

If `docker` is not found here, stop and reopen this repo from WSL after Docker Desktop integration is enabled.

## 4. Build your player inside the container

```bash
docker run --rm --entrypoint /bin/bash -v "$(pwd)":/filler/solution filler-local -lc '
set -e
export PATH="/usr/local/cargo/bin:$PATH"
cd /filler/solution
rm -f Cargo.lock
cargo build --release
'
```

## 5. Run one match on each map

```bash
docker run --rm --entrypoint /bin/bash -v "$(pwd)":/filler/solution filler-local -lc '
set -e
cd /filler
echo "=== map00 ==="
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
echo "=== map01 ==="
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
echo "=== map02 ==="
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
'
```

## 5.1 Show the board (non-quiet mode)

Use this when you want to see the map and placements on screen.

```bash
docker run --rm --entrypoint /bin/bash -v "$(pwd)":/filler/solution filler-local -lc '
set -e
cd /filler
./linux_game_engine -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
'
```

Optional slower visual refresh:

```bash
docker run --rm --entrypoint /bin/bash -v "$(pwd)":/filler/solution filler-local -lc '
set -e
cd /filler
./linux_game_engine -r -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
'
```

## 6. Prove the 3 maps are different

```bash
sha256sum engine_files/maps/map00 engine_files/maps/map01 engine_files/maps/map02
```

Expected result: 3 different hashes.

## 7. Run tests

```bash
cargo test
```

## 7.1 Audit.md exact checks (inside container)

Use this to match the audit questions exactly.

```bash
docker run --rm --entrypoint /bin/bash -it -v "$(pwd)":/filler/solution filler-local
```

Then run these commands inside `/filler`:

```bash
cd /filler

# Engine sanity check from Audit.md wording (linux variant here)
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator

# Student player basic run
./linux_game_engine -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e

# map00 vs wall_e, 5 runs with side swapping
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
./linux_game_engine -q -f maps/map00 -p1 linux_robots/wall_e -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e

# map01 vs h2_d2, 5 runs with side swapping
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2
./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2
./linux_game_engine -q -f maps/map01 -p1 linux_robots/h2_d2 -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map01 -p1 /filler/solution/target/release/filler -p2 linux_robots/h2_d2

# map02 vs bender, 5 runs with side swapping
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
./linux_game_engine -q -f maps/map02 -p1 linux_robots/bender -p2 /filler/solution/target/release/filler
./linux_game_engine -q -f maps/map02 -p1 /filler/solution/target/release/filler -p2 linux_robots/bender
```

## 8. If a command fails

Run:

```bash
docker --version
ls -l engine_files/linux_game_engine
ls -l engine_files/linux_robots/wall_e
```

If needed:

```bash
chmod +x engine_files/linux_game_engine
```
