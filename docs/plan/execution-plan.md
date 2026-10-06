# Execution plan

Ordered work, with the acceptance test each task must bring. Task ids are commit prefixes:
one task per commit, e.g. `F3: CFF charstrings through skrifa`.

Ids used here (**F**onts, **H**arness, **D**ebt) are new and do not collide with the original
`M0`–`M14` milestone ids, which keep their meaning in `ROADMAP.md` and
`parity/acrobat-features.toml`.

---

## 1. How a session runs

1. **Orient.** Read [`STATUS.md`](STATUS.md) for the current phase, the next unchecked task and
   the blockers. Read the task below, the relevant part of [`architecture.md`](architecture.md),
   and the README of the crate you are touching.
2. **Plan.** Decide the smallest change that completes one task id. If the task turns out to be
   two tasks, split it in `STATUS.md` rather than widening the commit.
3. **Implement with the test.** Write the acceptance test named in the task. New behaviour
   without a test does not ship. Every crash fix gets a synthetic regression test that panicked
   before the fix (AGENTS.md §4).
4. **Verify.** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo xtask layers`, and `cargo xtask parity` when a feature's
   status changes. Drive the engine through `printcraft-cli run --script` to check real
   behaviour; for UI work launch with `--control` and look at a screenshot.
5. **Record.** Update the feature's entry in `parity/acrobat-features.toml` (status, evidence,
   and notes that say what is *still* missing). Tick the task in `STATUS.md`. Add a line to the
   `ROADMAP.md` log at the end of a session.
6. **Commit and open a PR.** Green states only, one task id per commit. **Never commit or push
   to `main`** — see §1.1.

**Decide without asking:** library choice within the licence allowlist, file and module layout,
test shape, error wording, how to split a task. **Ask the owner:** anything touching the closed
Adobe-data allowlist (AGENTS.md §1.1), any new copyleft or network dependency, any change to the
layering table, anything that would make a save lossy, and anything that turns a feature on by
default that was off.

### 1.1 Branches and pull requests

Four long-lived branches, promoted in one direction. **None of them is ever committed or pushed
to directly** — every change, including a one-line documentation fix, lands through a pull
request.

```
<type>/<slug>  ──PR──▶  dev  ──PR──▶  main  ──PR──▶  release
  your work            integration    stable        installers
```

| Branch | What it is | What may enter it |
|---|---|---|
| `<type>/<slug>` | Where work happens | Your commits |
| `dev` | Integration. Where features meet each other for the first time | PRs from work branches |
| `main` | Stable. What a clone gets, and what the README describes | Promotion PRs from `dev` |
| `release` | Triggers the signed-installer pipeline (`release.yml`) | Promotion PRs from `main` |

**Why `dev` exists:** a PR is green against the branch it was *written* on, which proves nothing
about it meeting the three PRs that merged while it was in review. `dev` is where that collision
happens, so `main` stays a branch you can clone and trust.

CI runs on every pull request (no branch filter) and on pushes to `main`, `dev` and `release`, so
each merge is verified as *integrated*, not just as proposed.

#### Day-to-day: work branch to `dev`

1. **Branch from an up-to-date `dev`**, named `<type>/<slug>`, or `<type>/<issue>-<slug>` when it
   closes an issue:

   | Type | For | Example |
   |---|---|---|
   | `feat/` | a new feature or task | `feat/f1-font-programs` |
   | `fix/` | a bug, with the issue number | `fix/74-add-text-keeps-boxes` |
   | `docs/` | documentation only | `docs/roadmap-session-16` |
   | `chore/` | tooling, CI, dependencies, housekeeping | `chore/single-skrifa` |
   | `refactor/` | behaviour-preserving change, no new tests expected | `refactor/inspect-on-cos` |

2. **Commit in reviewable steps.** One task id per commit (`F3: CFF charstrings through skrifa`),
   subject in the imperative under ~72 characters, a body that says *why*. Each commit should be
   green on its own; do not commit a red state and fix it in the next one. End the message with
   the environment's attribution trailer, as existing commits do.
3. **Verify before pushing.** The full gate list in step 4 above. CI runs the same gates on the
   PR (`.github/workflows/ci.yml` triggers on `pull_request`), across macOS, Windows, Linux, a
   wasm32 check, `cargo-deny` and the nightly fuzz job — so a red PR means a red branch, not a
   flaky runner. Investigate rather than re-running.
4. **Open the PR against `dev`** — never against `main` — filling in
   `.github/pull_request_template.md`: what changed, why, how it was verified, and which
   `parity/acrobat-features.toml` entries moved. Link the issue with `Closes #N` where there is
   one. Check the base branch before you submit; a repository default can quietly aim it at
   `main`.
5. **Merge only green**, and prefer a merge commit so the branch name survives in the history
   (this is what the existing `Merge pull request #88 from storytold/docs/roadmap-session-15`
   commits are). Delete the branch afterwards.
6. **Rebase, don't merge back.** If `dev` moved under you, `git rebase origin/dev` and re-run the
   gates. Keep work branches short-lived so this stays cheap.

Rules of thumb: keep a PR to one concern; if the diff needs the word "and" twice to describe, it
is two PRs. A PR that changes a feature's parity status must move that entry in the same PR, not
a follow-up.

#### Promotion: `dev` to `main`

A promotion PR is a batch, not a feature. Open one when `dev` holds a coherent set of finished
work — typically at the end of a session, or when a milestone's tasks are all ticked.

- Title it `Promote dev to main: <what it contains>`; the body lists the PRs being promoted and
  anything a user would notice. The template's per-change checklist does not apply — those were
  checked on the way into `dev` — so replace it with the batch summary.
- `main` must be fast-forwardable from `dev`. If it is not, something was pushed to `main`
  directly; find out what before merging.
- Update the `ROADMAP.md` log and progress table in the promotion PR, not in each feature PR, so
  the log reads as one line per session rather than one per commit.
- Merge only when CI is green **on `dev` itself**, not merely on the PRs that fed it.

#### Promotion: `main` to `release`

Only for cutting a release. `release.yml` builds signed installers for every platform on push, so
treat the merge as the release action itself. Version bumps (`cargo xtask version set X.Y.Z`) go
in the promotion PR.

### Definition of done

A feature is done when all of these hold, and `shipped` in the parity file means exactly this:

- it works end to end from the UI **and** headlessly through an `automation` tool (AGENTS.md §3);
- it has a test that would fail if the feature regressed — not a smoke test;
- the parity entry cites that test, and its notes name what is still missing;
- hostile input cannot panic it (`Result` everywhere, bounded allocation and recursion).

---

## 2. Phase order

| Phase | What | State | Why here |
|---|---|---|---|
| **H** | Fidelity harness (`testkit`, `oracle`) | **active** | A regression target must exist *before* the renderer is replaced |
| **D** | Standing debt | **active** | Small, unblocks everything, runs alongside |
| **W** | Pro workflows, the parts needing no network | **available** | Wide but shallow; parallelisable across crates |
| **F** | Font engine (L2 `fonts`) | **parked** | Gated on the standard-14 metrics decision (`STATUS.md` §Decisions) |
| **M2** | Our own renderer | parked behind F | Turns 7 rendering P0 partials into shipped; retires the vendored patches |
| **M7** | Editing existing content | parked behind F | The most visible Pro gap |

H and D are independent and can run in parallel in separate worktrees. M2 needs both H and F;
M7 needs F. While F is parked, the bootstrap renderer stays and work goes to H, D and the parts
of W that need neither fonts nor network.

---

## 3. Phase D — standing debt

Pick these up when they block, or between larger tasks.

- [ ] **D1. Crate READMEs.** 13 crates have none (`engine`, `render`, `edit`, `cos`, `ui-egui`,
      `js`, `preflight`, `compare`, `export`, `ocr`, `organize`, `crypt`, `geom`), yet the session
      protocol says to read them. `cos` and `engine` first. *Accept:* each README states the
      layer, the API sketch, and what is deliberately not done.
- [ ] **D2. Tools for toolless features.** `cargo xtask parity` reports **92 shipped features
      with no automation tool**, 64 with neither tool nor command. That contradicts AGENTS.md §3.
      *Accept:* the parity note shrinks, each new tool has an end-to-end test in
      `crates/automation/tests/automation.rs`.
- [ ] **D3. Control-channel verbs.** `control.rs` has no `scroll`, `hover` or `right_click`, so
      context menus and wheel-zoom paths cannot be driven or regression-tested. *Accept:* new
      verbs plus `crates/ui-egui/tests/control.rs` cases that open a context menu and zoom by
      wheel.
- [ ] **D4. Single `skrifa`.** Three versions are in the tree (0.42.1 via egui, 0.44 via `fonts`,
      0.47 via vendored `hayro-interpret`). *Accept:* one version; `cargo tree -d` shows no
      duplicate.
- [ ] **D5. Evidence depth.** 159 of 404 shipped features rest on exactly one test, and 150
      citations sit in one file. Deepen evidence for P0 features as you touch them; prefer a test
      that cites an external oracle. *Accept:* no P0 feature left with a single generic test.

---

## 4. Phase F — the font engine

> **Parked by the owner, 2026-10-06** (`STATUS.md` §Decisions). F8 needs a decision on where the
> standard-14 metrics come from, and shipping F1–F7 without it would leave the engine unable to
> lay out the commonest fonts in existence. The phase is written up and ready; do not start it
> until it is unparked. M2 and M7 are parked with it.

**Where:** `crates/fonts` (L2). **Why it is the keystone:** `fonts` reads font *dictionaries* but
never a font *program*. `crates/edit/src/text.rs` can therefore only reuse a line's own font when every new
character already has a code and a glyph in it, and otherwise silently substitutes Helvetica.
Glyph outlines today come from inside `hayro`. Both M2 and M7 are blocked on this and nothing
else.

**Tools:** `skrifa` (Apache-2.0 OR MIT, already a dependency) for sfnt/CFF/CFF2 outlines and
metrics; `rustybuzz` (MIT) for shaping in F7. Both are on the `deny.toml` allowlist.

- [ ] **F1. Extract and identify font programs.** Read `/FontFile` (Type 1), `/FontFile2`
      (TrueType), `/FontFile3` (`/Type1C`, `/CIDFontType0C`, `/OpenType`) from the descriptor;
      sniff the real format rather than trusting the key; wrap bare CFF in a minimal OpenType
      shell so one reader handles all of them. *Accept:* `fonts::program::tests` identifies every
      variant from synthetic fixtures, and a truncated or lying `/Length1` yields `Err`, never a
      panic.
- [ ] **F2. Code to glyph.** Simple fonts: base encoding, `/Differences`, and the symbolic
      TrueType rules ((3,0) and (1,0) cmaps, the 0xF000 offset). Composite fonts: CMap to CID to
      GID through `/CIDToGIDMap`. *Accept:* `fonts::glyphs::tests` covers symbolic TrueType,
      Identity-H, an embedded CMap and a `/Differences` override.
- [ ] **F3. Outlines and metrics.** Glyph outlines and real advance widths from the program via
      `skrifa`, with the font dictionary's `/Widths` still winning where it disagrees (the spec
      makes the dictionary authoritative for layout). Type 1 (`eexec`, Type 1 charstrings) has no
      `skrifa` path: convert to CFF or interpret directly. *Accept:* outlines for a TrueType, a
      CFF and a Type 1 fixture; a width disagreement resolves to the dictionary; a malformed
      charstring yields `Err`.
- [ ] **F4. Type 3 fonts.** Glyph procedures are content streams; run them through
      `printcraft-content` with the font matrix, bounded by the existing nesting caps. *Accept:*
      a Type 3 glyph renders and measures; a self-referencing glyph terminates.
- [ ] **F5. Subsetting.** Build a new program containing only the glyphs used, with remapped ids,
      for sfnt and CFF (including CID-keyed). *Accept:* a subset of a fixture is smaller, loads
      in `skrifa`, and renders the same glyphs; the containing document passes `qpdf --check`.
- [ ] **F6. Embedding.** Write a font program back out with a correct descriptor, `/Widths` or
      `/W`, and a generated `/ToUnicode`, for both simple and composite fonts. *Accept:* a
      document with newly embedded text extracts its own text through `printcraft-cli text`, and
      `pdftotext` agrees.
- [ ] **F7. Shaping.** Complex scripts and vertical writing via `rustybuzz`: cluster mapping, RTL
      runs, mark positioning, and `vrt2`/`vert` for vertical CJK. *Accept:* an Arabic and a
      vertical Japanese fixture shape to the expected cluster and advance sequence.
- [ ] **F8. Standard 14 and fallback.** Real metrics for the standard 14 and a deterministic
      fallback chain when a program is missing or broken. **Blocked on an owner decision —
      see [`STATUS.md`](STATUS.md) §Decisions.** *Accept:* standard-14 widths match the spec
      tables; a document whose embedded font fails to parse still lays out, and says so.

---

## 5. Phase H — the fidelity harness

**Why before M2:** the ROADMAP's own honest assessment says Acrobat fidelity is *unmeasured*.
Replacing the renderer without a regression target means swapping one unmeasured renderer for
another.

**Scope, honestly.** Acrobat cannot be an automated oracle: the clean-room rules allow only
black-box observation with synthetic fixtures, and those notes are local-only (`plan/acrobat/`,
AGENTS.md §1.1). So this harness measures against **the specification** and **independent
implementations**, and Acrobat comparison stays a manual, documented step. Copyleft tools
(`poppler`, `mupdf`, `ghostscript`, `veraPDF`) run **only as external oracle processes**, never
linked.

- [ ] **H1. `printcraft-testkit`** (registered, `Class::Testkit`, dev-dependency only). Builders
      for synthetic fixtures: pages, fonts, shadings, transparency groups, forms, annotations —
      deterministic bytes so goldens are stable. *Accept:* the same fixture built twice is
      byte-identical; two existing crates drop hand-rolled fixture code in favour of it.
- [ ] **H2. `printcraft-oracle`** (registered, `Class::Testkit`). Wrappers that shell out to
      `pdftotext`, `qpdf`, `pdfsig`, `mutool` and `veraPDF`, each skipping cleanly when the tool
      is absent so CI stays green on machines without it. *Accept:* every wrapper returns
      `Skipped` rather than failing when the binary is missing.
- [ ] **H3. Render comparison.** `cargo xtask render-oracle`: render synthetic fixtures, compare
      against committed goldens with a perceptual metric and a per-fixture threshold, and report
      the worst offenders. Goldens are our own output of our own fixtures, so they are
      contributor-original (AGENTS.md §1.2) — but every fixture must be built only from allowed
      assets, and each golden needs its `ATTRIBUTION.toml` entry. *Accept:* a deliberate
      one-pixel regression fails the gate; a re-render on another platform does not.
- [ ] **H4. Text fidelity.** Extend the existing `xtask text-oracle` (word-F1 against
      `pdftotext`, median ≥ 0.97) with reading order, RTL and CJK cases, and record a baseline
      per fixture rather than one global median. *Accept:* a reading-order regression in a
      two-column fixture fails the gate.
- [ ] **H5. Behaviour fidelity.** Table-driven cases for form field event order, the AF format /
      keystroke / validate / calculate chain, and action dispatch, written from ISO 32000-2 and
      the public JavaScript for Acrobat API reference. *Accept:* a case per event in the
      documented order, each failing if the order changes.

---

## 6. Phase M2 — our own renderer

Keep `printcraft-render`'s public API frozen (`inspect`, `PageRenderer`/`RenderPool`,
`text::PageText`); swap what is behind it. Land it incrementally with hayro still present and a
switch, so each step is comparable through phase H.

- [ ] **M2.1. Graphics state and the content interpreter** on `printcraft-content`: full state
      stack, CTM, clipping, colour spaces (`color`, L2, reserved), ExtGState.
- [ ] **M2.2. DisplayList device.** A backend-independent list of draw operations, so raster,
      print imposition and export share one producer.
- [ ] **M2.3. Raster device** over the display list. Paths, fills, strokes, clips, images.
- [ ] **M2.4. Text rendering** on phase F: glyph runs, Type 3, CID-to-Unicode.
- [ ] **M2.5. Shadings, patterns, transparency.** Types 1–7, tiling patterns, blend modes, soft
      masks, transparency groups — each with the allocation and nesting caps the vendored hayro
      patches already proved necessary.
- [ ] **M2.6. Image codecs** in `filters`: DCT, JPX, JBIG2, CCITT (`core.image-filters`).
- [ ] **M2.7. Replace `inspect`'s `lopdf`** with `cos` + `model`.
- [ ] **M2.8. Retire the bootstrap.** Remove `hayro`, `hayro-syntax`, `hayro-jbig2`,
      `hayro-interpret` and `lopdf` from `crates/render`'s normal dependencies, keeping them as
      dev-only oracles. Delete the vendored patches that no longer apply, and record the ones
      that should still go upstream.

*Accept, per step:* the H3 render-oracle gate stays green and the H4 text baseline does not
regress; the 983-file corpus sweep keeps 0 crashes; `cargo xtask fuzz --time 300` finds nothing
new.

---

## 7. Phase M7 — editing existing content

Needs F5–F7. The parity notes on `edit.text-edit` describe today's limits precisely; keep them
honest as each falls away.

- [ ] **M7.1. Paragraph model.** Lines to paragraphs with real measured metrics, not
      approximations.
- [ ] **M7.2. Reflow.** Re-wrap a paragraph to a new width and push the following content.
- [ ] **M7.3. Font substitution and embedding on edit.** When a character has no glyph in the
      line's font, subset and embed a face that does (F5/F6) instead of falling back to
      Helvetica.
- [ ] **M7.4. RTL and CJK editing** (`edit.rtl-cjk-editing`) on F7.
- [ ] **M7.5. Keep structure tags on edit** (`edit.keep-tags-on-edit`).
- [ ] **M7.6. Vector and object editing** (`edit.vector-edit`, `edit.arrange-objects`,
      `edit.align-distribute-objects`).

---

## 8. Phase W — Pro workflows

Parallelisable across crates once M2 and M7 are underway. Ordered by user-visible value:

1. **M9 signatures.** Available now, needing no network: FieldMDP, lock-after-signing, OS trust
   stores, the signature-properties dialog, signature appearances, per-certificate trust.
   **Parked:** B-T timestamps, LTV (DSS/VRI) and OCSP/CRL, which need outbound requests — see
   `STATUS.md` §Decisions.
2. **M11 preflight and standards:** preflight profiles, checks, fixups, reports; PDF/X and
   PDF/UA validation. 43 features, the largest single block left.
3. **M10 OCR and Office:** scripts beyond Latin, deskew, editable-text output; `.xlsx`/`.pptx`
   export, Office import.
4. **M12 accessibility and XFA:** autotag, Tags/Order/Content panels, Reading Order tool; static
   then dynamic XFA.
5. **M14 polish:** performance budgets, localization, keyboard-only operation, signed installers.
