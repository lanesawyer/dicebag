# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build              # Build all crates
cargo test               # Run all tests
cargo fmt --all          # Format all code
cargo clippy -- -D warnings  # Lint (CI treats warnings as errors)
cargo run -p cli -- <args>   # Run the CLI
```

To run a single test:
```bash
cargo test <test_name>
cargo test -p core <test_name>   # Scoped to a crate
```

## Architecture

Cargo workspace with two crates:

- **`core/`** — Library crate. All business logic, data structures, and persistence. No CLI dependencies. Designed to be reused across multiple frontends (CLI today, WASM web app planned).
- **`cli/`** — Binary crate. Thin CLI wrapper over `core` using `clap` (derive macros).

### Core modules

- **`dice.rs`** — `DiceType` enum (D4–D100) and `Roll` struct. `Roll::roll()` returns a `Vec<i64>` of individual results; `Roll::roll_one()` rolls a single die. Parsing from strings via `FromStr` (e.g. `"d20"`).
- **`campaign.rs`** — `Campaign` struct with id, name, description, and a list of players.
- **`player.rs`** — `Player` struct (minimal; intended to grow into a character sheet).
- **`db.rs`** — `Persistable` trait providing `save_to_ron_file` / `load_from_ron_file`. Currently RON-file backed; a real DB is a planned future replacement.

### CLI commands

| Subcommand | Key flags | Behavior |
|---|---|---|
| `campaign` | `--name`, `--description` | Creates a campaign, saves to RON file |
| `player` | `--name` | Currently prints a greeting (persistence is a TODO) |
| `roll` | `--dice`, `--number` | Rolls dice, prints individual results and total |

### Persistence

`Persistable` is implemented on `Campaign`. Files are saved as `<name-with-dashes>.ron` using the `ron` crate for serialization. The `db.rs` comment explicitly flags this as a temporary solution pending a real database.
