# Filler Docs

This folder breaks the Rust implementation into small steps.

## Read in this order

1. `01-project-breakdown.md`
2. `02-architecture.md`
3. `03-step-by-step-plan.md`
4. `04-testing-plan.md`
5. `05-audit-readiness.md`
6. `06-docker-and-audit-commands.md`
7. `07-audit-results.md`
8. `08-host-setup.md`
9. `09-reproduction-commands.md`
10. `10-audit-command-matrix.md`

## Goal

Build a reliable Rust CLI robot for the Filler game engine that:
- parses engine input correctly every turn,
- outputs exactly one coordinate per turn,
- respects placement rules,
- uses a scoring strategy to improve win rate,
- includes unit tests for parser, validator, boundaries, and output format.

## Commands

- Build: `cargo build`
- Test: `cargo test`
