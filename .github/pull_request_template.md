<!--
Branch naming and the full workflow: docs/plan/execution-plan.md §1.1
Keep a PR to one concern. If describing the diff needs "and" twice, it is two PRs.
-->

## What and why

<!-- What changed, and the reason. Link the task id (e.g. F3, M2.4, D2) or issue. -->

Closes #

## How it was verified

<!-- Name the tests that would fail without this change, and any manual check. -->

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo xtask layers`
- [ ] `cargo xtask parity` (if a feature's status changed)
- [ ] Looked at the result in the running app (UI changes only)

## Parity

<!-- Which parity/acrobat-features.toml entries moved, and what their notes now say is still
     missing. Write "none" if this PR changes no feature status. -->

## Checklist

- [ ] New behaviour has a test that fails without the change — not a smoke test
- [ ] Any crash fix has a synthetic regression test that panicked before it
- [ ] No `unwrap`/`expect`/`panic!`/indexing on input-derived positions in non-test code (AGENTS.md §4)
- [ ] Every user-facing feature is reachable headlessly through an `automation` tool (AGENTS.md §3)
- [ ] No asset from an Adobe product; any new asset has an `ATTRIBUTION.toml` entry (AGENTS.md §1)
- [ ] Clean-room respected: behaviour from the spec, public docs or black-box observation only
