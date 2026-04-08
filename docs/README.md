# Filler Docs

This folder is simplified to one run guide plus reference docs.

## Read in this order

1. `01-project-breakdown.md`
2. `02-architecture.md`
3. `03-step-by-step-plan.md`
4. `04-testing-plan.md`
5. `05-audit-readiness.md`
6. `07-audit-results.md`

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

## Important

Use only `03-step-by-step-plan.md` for setup and run commands.
