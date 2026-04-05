# Reproduction Commands

Use these commands to reproduce the audit locally.

## Step 1: Go to the official engine package

```bash
cd /mnt/c/Users/xs7ar/Downloads/filler/docker_image
```

## Step 2: Build the Docker image

```bash
docker build -t filler .
```

## Step 3: Start container with project source mounted

```bash
docker run -v /mnt/c/Projects/filler:/filler/solution -it filler
```

## Step 4: Build the player inside the container

```bash
cd /filler/solution
rm -f Cargo.lock
cargo build --release
```

## Step 5: Enter the engine directory inside the container

```bash
cd /filler
```

## Step 6: Quick sanity match

```bash
./linux_game_engine -f maps/map00 -p1 /filler/solution/target/release/filler -p2 linux_robots/wall_e
```

## Optional alternative: run with prebuilt binary only

Use this only if you already built a compatible Linux binary and do not need `cargo build` in container.

```bash
mkdir -p solution/bin
cp /mnt/c/Projects/filler/solution/bin/filler solution/bin/filler
chmod +x solution/bin/filler
docker run -v "$(pwd)/solution":/filler/solution -it filler
# inside container:
./linux_game_engine -f maps/map00 -p1 /filler/solution/bin/filler -p2 linux_robots/wall_e
```

## Notes

- The official image uses an older Rust toolchain.
- Building inside the container avoids lockfile and compiler mismatch issues.
- Use `/filler/solution/target/release/filler` for the player binary after building inside the container.
- If you mount only `$(pwd)/solution`, `Cargo.toml` is usually missing, so `cargo build` will fail.
