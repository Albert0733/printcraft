# Status

**Read this first every session.** Then the task in
[`execution-plan.md`](execution-plan.md), the relevant part of
[`architecture.md`](architecture.md), and the README of the crate you are touching.

- **Current phase:** F 4.1 (font engine, dictionary level), with H (fidelity harness) and D
  (standing debt) alongside.
- **Next unchecked task:** **F1 — exact standard-14 metrics** (`crates/fonts`). It replaces a
  guess table that six shipping crates depend on, needs no renderer and no new dependency.
- **Then:** F2, F3, then H1 (`printcraft-testkit`).
- **Phase F is not parked.** It was, briefly, on a mistaken reading of AGENTS.md §1.1; the
  standard-14 metrics and glyphs are both already allowlisted and already in the repository.
  See §Decisions. Only outbound network access stays parked.
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
   outlines exist nowhere outside `hayro`. This blocks M2 (renderer) and M7 (editing existing
   text) — together roughly a third of the remaining effort. Phase F clears it: tranche 4.1 is
   active now, tranche 4.2 follows the harness.
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

Deferred deliberately, not forgotten. Do not start work that depends on it, and do not re-raise
it as a blocker — raise it again only when the work it gates becomes the priority.

- **Outbound network access (gates M9 timestamps, LTV, OCSP/CRL).** Held over from session 14
  (#63). RFC 3161 timestamping and revocation checking require outbound requests from an app that
  currently makes none and advertises "no telemetry, no cloud". Signature work needing no network
  — FieldMDP, lock-after-signing, OS trust stores, signature appearances, per-certificate trust —
  is **not** parked and can proceed.

### Resolved: standard-14 font data is not gated (2026-10-06)

Recorded because it was briefly logged here as a blocker and should not be re-raised. Both halves
were already permitted and already present:

- **Glyphs:** the 14 Foxit `.pfb` substitute faces under `vendor/hayro-interpret/assets/`, author
  Foxit Software / PDFium Authors — **not Adobe** — BSD-3-Clause, with licence file and SHA-256
  entries in `ATTRIBUTION.toml` that already pass `cargo xtask assets`.
- **Metrics:** AGENTS.md §1.1 permits "the standard-14 font metrics and encoding tables in the
  PDF specification… data, not typefaces", and they ship today as
  `vendor/hayro-interpret/src/font/generated/metrics.rs`, already carrying `adobe_data = true`.
  Writing our own tables from ISO 32000-2 Annex D is inside that existing permission. Retiring
  hayro (M2.8) needs that entry's path reworded, not a new approval.

Only going **beyond** Annex D to the fuller Adobe Core14 AFM set would be worth re-checking, and
even then it is the same already-listed item.

### Open

- [ ] **Acrobat reference material.** Phase H can measure against the specification and
      independent implementations, but not against Acrobat. If visual parity with Acrobat is to
      be gated rather than eyeballed, we need either access to the existing `plan/acrobat/`
      notes or a fresh black-box observation pass under the AGENTS.md §1.1 rules (synthetic
      fixtures only, nothing committed). H1–H5 do not block on this; H3's goldens pin *our*
      output against regressions, which is a different question from matching Acrobat.

## Conventions reminder

- One task id per commit (`F6: Type 1 charstrings without skrifa`). Commit only green states.
- Update `parity/acrobat-features.toml` in the same commit as the feature, and say in `notes`
  what is *still* missing. `shipped` is a strong claim — see "Definition of done".
- Add a line to the `ROADMAP.md` log at the end of every session, and keep its
  §Honest assessment and §Where we're lacking true.
- Parallel agents: a separate `CARGO_TARGET_DIR` and a separate git worktree each.
