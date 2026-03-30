---
name: add-feature
description: Use when implementing any new feature in dicebag. Enforces the full implementation checklist: core Rust logic first, then WASM bindings, CLI command, webapp action, UI, and TypeScript interface sync. Invoke whenever the user asks to add, build, or implement something new.
---

# Add Feature

Implement the following feature: $ARGUMENTS

Work through these steps in order. Complete each layer before moving to the next — every feature must be reachable from both the CLI and the webapp.

## Implementation checklist

### 1. Core (`core/src/`)
- Add the data structures, methods, and business logic in the appropriate module(s)
- No file I/O, no side effects — keep core pure so it compiles to WASM
- Write unit tests for any non-trivial logic
- If mutating more than one field, encapsulate it as a method rather than doing it inline elsewhere

### 2. WASM bindings (`wasm/src/lib.rs`)
- Expose any new core types or methods needed by the webapp via `wasm-bindgen`
- Prefer generic bindings over per-variant functions (e.g. one function with a parameter, not one function per case)
- Run `pnpm wasm` from `webapp/` to rebuild and verify it compiles

### 3. CLI (`cli/src/`)
- Add a subcommand (or extend an existing one) in `cli/src/main.rs` and the relevant file under `cli/src/commands/`
- The CLI must be able to exercise the new core functionality end-to-end
- Update the CLI commands table in CLAUDE.md if adding a new subcommand

### 4. Webapp action (`webapp/src/actions/index.ts`)
- Add an Astro Action for any mutation (create, update, delete)
- I/O only in the action: read file → pass to WASM → mutate via core method → serialize via WASM → write file
- No business logic in TypeScript — if you find yourself computing something, stop and put it in core

### 5. Webapp UI (`webapp/src/pages/`)
- Wire up the new action to the relevant page(s)
- Use existing UI patterns (Bulma, astro-bulma components)

### 6. TypeScript interfaces (`webapp/src/lib/campaigns.ts`)
- If core types changed, update the corresponding TypeScript interfaces here

### 7. Final checks
- `cargo test` passes
- Both the CLI and the webapp can exercise the feature
