# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Dicebag is a D&D tooling suite written in Rust. It is a Cargo workspace with two crates:
- **`core`** — domain logic library (dice rolling, player/campaign entities)
- **`cli`** — command-line interface that uses `core`

The architecture deliberately separates `core` from any frontend so it can be reused across future frontends (web/WASM, TUI, etc.).

## Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run tests for a single crate
cargo test -p core
cargo test -p cli

# Run a single test by name
cargo test -p core tests::it_works -- --nocapture

# Format check (enforced in CI)
cargo fmt --all -- --check

# Lint (enforced in CI, warnings are errors)
cargo clippy -- -D warnings

# Run the CLI
cargo run --bin cli -- roll -d d20 -n 3
cargo run --bin cli -- campaign -n "My Campaign" -d "A description"
cargo run --bin cli -- player -n "Aragorn"
```

## Architecture

### `core` crate

- [core/src/dice.rs](core/src/dice.rs) — `DiceType` enum (D4–D100) and `Roll` struct. `Roll::roll()` returns `Vec<i64>`; `DiceType` parses from strings like `"d20"`. Uses `rand` with a ChaCha backend.
- [core/src/player.rs](core/src/player.rs) — `Player` struct (id, name). Skeletal; intended to hold character sheet data eventually.
- [core/src/campaign.rs](core/src/campaign.rs) — `Campaign` struct (id, name, description). Skeletal; intended to hold campaign state eventually.
- [core/src/lib.rs](core/src/lib.rs) — re-exports `Campaign`, `DiceType`, `Roll`.

### `cli` crate

- [cli/src/main.rs](cli/src/main.rs) — built with `clap` derive API. Three subcommands: `roll` (functional), `campaign` (stub), `player` (stub). Persistence is not yet implemented.

## CI

Three parallel GitHub Actions jobs run on PRs: **format**, **lint** (clippy `-D warnings`), **test**. All three must pass. The toolchain is stable with rustfmt and clippy components.

## Development Notes

- Both crates use the **2024 Rust edition**.
- Clippy is configured to deny all warnings (`-D warnings`) — keep code warning-free.
- `Cargo.lock` is committed for reproducible builds.
- Dependabot runs monthly for cargo dependency updates.
