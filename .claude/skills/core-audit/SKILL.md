---
name: core-audit
description: Use when reviewing webapp actions for logic that belongs in Rust core instead of TypeScript. Invoke before opening a PR, when asked to review the webapp actions, or when you notice potential business logic in TypeScript while working.
---

# Core Audit

Audit `webapp/src/actions/index.ts` for logic that belongs in `core/` instead of TypeScript.

## What to look for

Read `webapp/src/actions/index.ts` in full, then check each action for:

1. **ID generation** — any inline `Math.max(...items.map(i => i.id)) + 1` or similar. Check if a `next_id()` method already exists on the relevant core type.
2. **Sorting or ordering** — sorting participants, reordering lists, deriving display order. Check if `core/` already has a method (e.g. `initiative_order()`).
3. **Multi-field mutations** — actions that set more than one field on a struct inline. These should be encapsulated as a core method.
4. **Derived state** — computing totals, counts, status flags, or any value that could be a method on a core type.
5. **Validation** — any guards or checks on data values that aren't purely about I/O.

## How to audit

For each potential violation:
1. Grep `core/src/` to check if the logic already exists there before assuming it doesn't.
2. Report: what the TypeScript is doing, which file and line, and what the fix should be (new core method, or use existing one).

## Output format

Produce a checklist of findings:

- [ ] `actions/index.ts:145` — ID generation for participant; `Encounter::next_participant_id()` should be added to `core/encounter.rs`
- [ ] `actions/index.ts:177` — initiative sorting duplicates `Encounter::initiative_order()`; use that instead

If nothing is found, say so explicitly. Do not flag things that are legitimately TypeScript (file I/O, session reads, HTTP).
