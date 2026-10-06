# docs/plan — the working plan

This is the **committed** plan for PrintCraft, maintained by the team currently developing it.
Start every session at [`STATUS.md`](STATUS.md).

| File | What it holds |
|---|---|
| [`STATUS.md`](STATUS.md) | Session entry point: current phase, next unchecked task, blockers, open decisions |
| [`architecture.md`](architecture.md) | The crate graph, the layering rules and the boundaries that must not move |
| [`execution-plan.md`](execution-plan.md) | Ordered tasks with acceptance tests, and the operating protocol for a session |

Progress and estimates live in [`../../ROADMAP.md`](../../ROADMAP.md); the machine-readable
feature checklist lives in [`../../parity/acrobat-features.toml`](../../parity/acrobat-features.toml)
and is validated by `cargo xtask parity`.

## Relationship to the gitignored `plan/`

The original authors kept their planning notes in a top-level `plan/` directory that is
**gitignored and local-only**, so it never travels with a clone. Roughly two dozen references
across the code and docs still point into it (`plan/architecture.md`, `plan/execution-plan.md §3`
and `§7`, `plan/acrobat/02-ui-ux.md`, `plan/adr/0001`).

- **`docs/plan/` is canonical for current development.** It is committed, so every machine and
  every agent has it.
- **`plan/`, where it exists, is the original team's historical notes.** Read it if you have it;
  it is the only source for the Acrobat black-box observations (`plan/acrobat/`) and the ADRs.
  Nothing here overwrites it, and nothing here depends on it.
- Code comments that cite `plan/architecture.md §3` and similar are left alone. The equivalent
  material is in [`architecture.md`](architecture.md); treat the two as describing the same rules,
  because `xtask layers` enforces them either way.

## What stays where

- **Clean-room rules, asset policy, never-crash rules:** [`../../AGENTS.md`](../../AGENTS.md). It
  overrides everything here.
- **Day-to-day working instructions:** [`../../CLAUDE.md`](../../CLAUDE.md).
- **Acrobat observation notes:** `plan/acrobat/` only, never committed (AGENTS.md §1.1). Nothing
  observed from Acrobat is reproduced in this directory.
