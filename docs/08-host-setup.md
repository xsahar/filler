# Host Setup

Use this guide if you want to run the Filler audit on your own machine.

## Prerequisites

- Windows with WSL or a Linux machine
- Docker installed and running
- The official Filler package downloaded and extracted
- This project repository cloned locally

## What you need on disk

You need two folders:

1. The official Filler engine package folder, containing:
- `docker_image/` or the engine files such as `linux_game_engine`, `linux_robots`, and `maps`

2. Your student project folder, containing:
- Rust source code
- `Cargo.toml`
- `src/`
- `scripts/`
- `docs/`

## Why both folders are needed

- The official Filler package provides the game engine, maps, and built-in robots.
- Your project provides the student player binary.
- The audit runs the engine from the official package and mounts your player into it.

## Docker check

From a terminal, confirm Docker works:

```bash
docker --version
docker info
```

If both commands work, you can proceed.
