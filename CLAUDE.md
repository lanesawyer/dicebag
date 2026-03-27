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

Webapp:
```bash
cd webapp
pnpm dev        # Builds WASM then starts Astro dev server
pnpm build      # Builds WASM then builds Astro for production
pnpm wasm       # Rebuild WASM only (runs wasm-pack)
```

## Architecture

Cargo workspace with three crates plus a webapp:

- **`core/`** — Library crate. All business logic and data structures. No file I/O — kept pure so it can compile to WASM.
- **`cli/`** — Binary crate. Thin CLI wrapper over `core` using `clap`. Owns all file I/O via `cli/src/persistence.rs`.
- **`wasm/`** — WASM crate. Exposes `core` to the browser via `wasm-bindgen`. Built with `wasm-pack --target web` into `webapp/src/wasm/`.
- **`webapp/`** — Astro SSR app (Node adapter). Uses `astro-bulma` for UI. Node.js handles file I/O; WASM handles all data parsing/serialization.

### Core modules

- **`dice.rs`** — `DiceType` enum (D4–D100) and `Roll` struct. `Roll::roll()` returns a `Vec<i64>`; `Roll::roll_one()` rolls a single die.
- **`campaign.rs`** — `Campaign` struct with id, name, description, players, and entities.
- **`player.rs`** — `Player` struct (minimal; intended to grow into a character sheet).
- **`entity.rs`** — `Entity` struct with kind (Player/Monster).
- **`encounter.rs`** — `Encounter` struct with participants and combatant references.

### CLI commands

| Subcommand | Key flags | Behavior |
|---|---|---|
| `campaign` | `--name`, `--description` | Creates a campaign, saves to RON file |
| `player` | `--name` | Currently prints a greeting (persistence is a TODO) |
| `roll` | `--dice`, `--number` | Rolls dice, prints individual results and total |

### Persistence

Files are saved as `<name-with-dashes>.ron` in the XDG data directory (`~/.local/share/dicebag` on Linux/Mac). The CLI uses `cli/src/persistence.rs`; the webapp reads files in Node.js and passes content to WASM for parsing/serialization.

## Guidelines

### All crates
- When adding new types or features to `core/`, always wire up CLI commands for them in `cli/src/main.rs` in the same task — don't leave new core functionality unreachable from the CLI.

### Webapp
- Use **Astro Actions** for all form mutations (create, update, delete). Do not handle POST logic with `if (Astro.request.method === 'POST')` checks in page frontmatter.
- **TypeScript/JavaScript is only allowed for I/O** (reading/writing files, sessions, HTTP). All business logic — data structures, validation, mutations, derived state, anything that could be reused in another app — belongs in `core/` (Rust) and is exposed via WASM. If you find yourself writing logic in TypeScript, stop and put it in `core/` instead.
- All data parsing and serialization goes through the WASM module. Never reimplement in TypeScript what `core` already does or should do.
