---
name: parity-check
description: Use when verifying that CLI commands and webapp actions are in sync. Invoke before opening a PR, when adding a new feature, or when asked to check parity between the CLI and webapp.
---

# Parity Check

Audit CLI commands and webapp actions to find anything reachable from one interface but not the other.

## How to audit

1. Read `webapp/src/actions/index.ts` and list every action (create, update, delete, and any other mutations).
2. Read `cli/src/main.rs` and the files under `cli/src/commands/` and list every subcommand.
3. Cross-reference the two lists.

## Output format

Produce three sections:

### In both (parity confirmed)
- `createCampaign` ↔ `campaign new`
- ...

### Webapp only (missing CLI command)
- [ ] `editCampaign` — no CLI equivalent for renaming/editing a campaign

### CLI only (missing webapp action)
- [ ] `encounter set-initiative` — no webapp action to set initiative directly

For each gap, suggest what the missing counterpart should look like (command name and flags, or action name and input schema). Do not suggest skipping parity — every feature should be reachable from both interfaces.
