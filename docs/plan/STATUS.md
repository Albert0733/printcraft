# Status

**Read this first every session.** Then the task in
[`execution-plan.md`](execution-plan.md), the relevant part of
[`architecture.md`](architecture.md), and the README of the crate you are touching.

- **Current phase:** H (fidelity harness) and D (standing debt). D items are small and can be
  taken between H tasks.
- **Next unchecked task:** **H1 — `printcraft-testkit`** (synthetic fixture builders).
- **Phase F (font engine) is parked** by owner decision, 2026-10-06. See §Decisions. Nothing in
  D or H depends on it; M2 and M7 still do, so F has to be unparked before either starts.
- **Last verified green:** 2026-10-06.

## Verified baseline (2026-10-06, Windows 10, this machine)

Measured, not quoted. Re-measure rather than trusting this table if it is more than a few
sessions old.

| Check | Result |
|---|---|
| `cargo test --workspace` | 718 passed, 0 failed, 4 ignored (101 suites) |
| `cargo fmt --check` | clean |
| `cargo clippy --workspace --all-targets` | clean, no warnings |
| `cargo xtask layers` | 31 crates, 0 violations |
| `cargo xtask parity` | 805 counted features: 404 shipped, 57 partial — 50.2% (53.7% weighted) |

Parity by tier: P0 88.4% shipped, P1 52.9%, P2 5.4%, P3 0%.

Not re-measured this session: the 983-file corpus sweep (`xtask check`), the nightly fuzz job,
and the wasm32 check. `cargo xtask demo-pdf` needs Chrome; `xtask models` fetches the OCR models.

## Blockers

1. **No font engine.** `crates/fonts` reads font dictionaries but never a font *program*; glyph
   outlines come from inside `hayro`. This blocks both M2 (renderer) and M7 (editing existing
   text) — together roughly a third of the remaining effort. Phase F exists to clear it, and is
   **parked** (§Decisions), so M2 and M7 are parked with it.
2. **Fidelity is unmeasured.** No side-by-side harness exists. `shipped` means "has at least one
   cited test", and 159 of 404 shipped features rest on exactly one, with 150 of the citations in
   a single file (`crates/automation/tests/automation.rs`). Treat the 50.2% as a coverage map,
   not a quality claim. Phase H exists to fix the measurement.
3. **92 shipped features have no automation tool** (64 have neither tool nor command), which
   contradicts AGENTS.md §3. Tracked as D2. `cargo xtask parity` lists them by name.
4. **The original `plan/` is not present on this machine.** It is gitignored and local-only, so
   the Acrobat observation notes (`plan/acrobat/`) and the ADRs are unavailable here. UI-fidelity
   work therefore has no local reference to compare against; see §Decisions.

## Decisions

Do not guess these; they are listed in `execution-plan.md` §1 as owner calls.

### Parked by the owner (2026-10-06)

Deferred deliberately, not forgotten. Do not start work that depends on either, and do not
re-raise them as blockers — raise them again only when the work they gate becomes the priority.

- **Fonts — standard-14 metrics (gates F8, and therefore M2.8 and M7.3).** AGENTS.md §1.1
  permits exactly one list of Adobe-authored non-visual data, and that list is **closed**. It
  currently covers the standard-14 metrics "embedded as code in hayro". Retiring hayro means
  re-homing that data into our own crate, which is a change to the closed list and needs
  explicit approval plus an `ATTRIBUTION.toml` entry with `adobe_data = true`. The alternative
  is deriving metrics from an openly licensed metric-compatible face, at some fidelity cost.
  **Until this is decided, phase F stays parked and the bootstrap renderer stays.**
- **Outbound network access (gates M9 timestamps, LTV, OCSP/CRL).** Held over from session 14
  (#63). RFC 3161 timestamping and revocation checking require outbound requests from an app
  that currently makes none and advertises "no telemetry, no cloud". Signature work that needs
  no network (FieldMDP, lock-after-signing, OS trust stores, the signature-properties dialog)
  is **not** parked and can proceed.

### Open

- [ ] **Acrobat reference material.** Phase H can measure against the specification and
      independent implementations, but not against Acrobat. If visual parity with Acrobat is to
      be gated rather than eyeballed, we need either access to the existing `plan/acrobat/`
      notes or a fresh black-box observation pass under the AGENTS.md §1.1 rules (synthetic
      fixtures only, nothing committed). H1–H5 do not block on this; H3's goldens pin *our*
      output against regressions, which is a different question from matching Acrobat.

## Conventions reminder

- One task id per commit (`F3: CFF charstrings through skrifa`). Commit only green states.
- Update `parity/acrobat-features.toml` in the same commit as the feature, and say in `notes`
  what is *still* missing. `shipped` is a strong claim — see "Definition of done".
- Add a line to the `ROADMAP.md` log at the end of every session, and keep its
  §Honest assessment and §Where we're lacking true.
- Parallel agents: a separate `CARGO_TARGET_DIR` and a separate git worktree each.
