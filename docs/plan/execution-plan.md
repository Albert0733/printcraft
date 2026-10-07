# Execution plan

Ordered work, with the acceptance test each task must bring. Task ids are commit prefixes:
one task per commit, e.g. `F6: Type 1 charstrings without skrifa`.

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
3. **Implement, reviewing each update as you make it.** Write the acceptance test named in the
   task. New behaviour without a test does not ship. Every crash fix gets a synthetic regression
   test that panicked before the fix (AGENTS.md §4). **After each update, review it outward
   through the four zooms of §1.2 and test it at the matching zoom of §1.3** — this is a
   per-update habit, not an end-of-task pass.
4. **Verify the whole task.** `cargo fmt --check`, then
   `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
   `cargo xtask layers`, and `cargo xtask parity` when a feature's status changes.
   Drive the engine through `printcraft-cli run --script` to check real
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
   | `feat/` | a new feature or task | `feat/f1-standard-14-metrics` |
   | `fix/` | a bug, with the issue number | `fix/74-add-text-keeps-boxes` |
   | `docs/` | documentation only | `docs/roadmap-session-16` |
   | `chore/` | tooling, CI, dependencies, housekeeping | `chore/single-skrifa` |
   | `refactor/` | behaviour-preserving change, no new tests expected | `refactor/inspect-on-cos` |

2. **Commit in reviewable steps.** One task id per commit (`F6: Type 1 charstrings without skrifa`),
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

### 1.2 Review at four zooms, after every update

**Review each update as you make it, not when the task is finished.** A defect caught in the
minute it was written costs nothing; the same defect found three commits later has had code built
on top of it, and the fix now risks the thing that was built. This is the practice that keeps
regressions and swallowed errors from compounding.

After each update, read the change outward through four zoom levels. **Scale the effort to the
change:** a comment fix stops at the file level, a parser change goes all the way out. The
judgement of how far to zoom is yours; the habit of zooming is not optional.

**Clause — the expression you just wrote.** AGENTS.md §4 applied line by line:

- every input-derived number: checked or saturating arithmetic, no negative cast to `usize`, no
  division by zero, no NaN or infinity cast;
- every index or slice: `get()` rather than `[i]`, and strings sliced only at char boundaries;
- every `?`: the right error variant, and a message saying what was wrong *and* what is allowed;
- every boundary: `<` against `<=`, inclusive against exclusive, and the 1-based/0-based seam
  (pages and positions are 1-based at the tool and UI boundary, 0-based inside);
- every allocation or loop bound taken from the file: capped.

**Function — the unit you just finished.**

- Is it total? Try to name an input that panics. If you can name one, fix it.
- Walk empty, zero, one, maximum, malformed and cyclic input through it.
- Does it preserve the invariant its callers assume?
- Is the return type honest — `Option`, `Result`, or a lenient fallback that *records* the repair
  rather than hiding it?

**File and module — the neighbours.**

- Do this function and the ones around it now disagree: two sources of truth, two spellings of the
  same check, two ways to express the same state?
- Is anything now dead, or duplicated from another module?
- Is the public surface still coherent, and still the smallest it can be?
- Does the module's doc comment still describe what the module does?

**Project — the blast radius.**

- `cargo xtask layers`, the wasm check, and the never-crash lints.
- Does a shipped feature change behaviour: undo and redo, save fidelity (incremental against
  full), encryption, permissions?
- Does the command registry, the `automation` tool table or the UI control channel need a matching
  entry (AGENTS.md §3)?
- Does any `parity/acrobat-features.toml` status or note become untrue?
- Does the README, or the catalogue's `Availability`, now claim something that is not so?

**If a zoom level turns up a defect, fix it before widening.** Carrying a known file-level problem
out to the project level means reviewing the wrong file.

### 1.3 Test at the same four zooms

Tests mirror the review. A change that merits only a clause-level review merits only a
clause-level test.

| Zoom | What the test pins | Where it lives | Tools |
|---|---|---|---|
| Clause | The hostile-input predicate itself, over generated input | `#[cfg(test)]` in the module | `proptest` |
| Function | Empty, zero, maximum, malformed, cyclic; the contract | `#[cfg(test)]` in the module | — |
| File / module | The module's own API, composed | `crates/<crate>/tests/` | `testkit` (H1) |
| Project | End to end through a tool or the running UI | `crates/automation/tests/`, `crates/ui-egui/tests/` | `egui_kittest`, `oracle` (H2) |

- Fix a bug at the zoom where the bug lived, and add a test one level out **only** when the bug
  was invisible at its own level.
- A crash fix always gets a synthetic regression test that panicked before the fix (AGENTS.md §4),
  at clause or function zoom.
- Do not write four tests for one change. Write the one at the zoom where the defect would show,
  then ask whether the level out would have caught it — and if the answer is no, that is the gap
  worth a second test.

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
| **F 4.1** | Font engine, dictionary level (F1–F3) | **active, first** | Fixes approximations six shipping crates rely on today; needs no renderer and no new dependency |
| **H** | Fidelity harness (`testkit`, `oracle`) | **active** | A regression target must exist *before* the renderer is replaced |
| **D** | Standing debt | **active** | Small, unblocks everything, runs alongside |
| **W** | Pro workflows, the parts needing no network | **available** | Wide but shallow; parallelisable across crates |
| **F 4.2** | Font engine, program reader (F4–F10) | after H | Payoff lands with M2 and M7, so it queues behind the harness that measures it |
| **M2** | Our own renderer | after F 4.2 | Turns 7 rendering P0 partials into shipped; retires the vendored patches |
| **M7** | Editing existing content | after F 4.2 | The most visible Pro gap |

F 4.1, H and D are independent and can run in parallel in separate worktrees. M2 needs H and
F 4.2; M7 needs F 4.2. Until F 4.2 lands the bootstrap renderer stays.

**Parked by the owner:** outbound network access, which gates only the M9 signature features that
need it (timestamps, LTV, OCSP/CRL). Everything else in W is available. See `STATUS.md`
§Decisions.

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

**Where:** `crates/fonts` (L2). **Why it matters:** `fonts` reads font *dictionaries* but never a
font *program*. Glyph outlines exist nowhere outside `hayro`, and `crates/edit/src/text.rs` can
only reuse a line's own font when every new character already has a code and a glyph in it —
otherwise it silently substitutes Helvetica. M2 and M7 are blocked on this and nothing else.

**Nothing here is gated.** The standard-14 glyphs are already in the repository as the 14 Foxit
`.pfb` faces under `vendor/hayro-interpret/assets/` (BSD-3-Clause, Foxit Software / PDFium
Authors — not Adobe, licence file and SHA-256 in `ATTRIBUTION.toml`). The standard-14 metrics are
already an allowlisted item: AGENTS.md §1.1 permits "the standard-14 font metrics and encoding
tables in the PDF specification… data, not typefaces", and they ship today as
`vendor/hayro-interpret/src/font/generated/metrics.rs` with `adobe_data = true`. Writing our own
tables from ISO 32000-2 Annex D is inside that existing permission; retiring hayro needs the
entry's path reworded, not a new approval.

**Tools:** `skrifa` (Apache-2.0 OR MIT, already a dependency) for sfnt/CFF/CFF2 outlines and
metrics; `rustybuzz` (MIT) for shaping. Both are on the `deny.toml` allowlist.

### 4.1 Tranche one — the dictionary level (active)

No font-program parsing, no renderer, no new dependency. Each of these improves code that ships
today, which is why they come before the harness rather than after it.

- [ ] **F1. Exact standard-14 metrics.** Replace `printcraft_fonts::helvetica_width` — a 15-line
      character-class guess where Times is Helvetica x 0.9, bold is x 1.05 and Courier is a flat
      0.6 — with the real Annex D tables for all 14 faces, plus their encodings. **Six shipping
      crates use that guess right now:** `annot` (text-box line breaking), `forms` (field
      appearance, comb cells, captions), `edit` (added text, watermarks, headers and footers),
      `redact` (overlay text), `sign` (signature appearance), `ocr` (invisible text layer). This
      is the highest value-per-hour task in the phase. *Accept:* every standard-14 width matches
      the specification table; the six call sites move over; a golden appearance stream per crate
      pins the new layout.
- [ ] **F2. Encoding and code-to-Unicode completeness.** The symbolic TrueType rules ((3,0) and
      (1,0) cmaps, the 0xF000 offset), `/Differences` with glyph names, and embedded CMap
      codespace and CID ranges — all readable from the dictionary alone. Improves text
      extraction, find and copy on fonts with no `/ToUnicode` (`view.render-cid-fonts`, a P0
      partial). *Accept:* `fonts::encodings` tests cover each rule; the H4 text baseline improves
      on a no-`/ToUnicode` fixture and regresses nowhere.
- [ ] **F3. Deterministic fallback.** One documented chain when a font is missing, unembedded or
      broken, with the choice *recorded* rather than silent, so callers can report it. *Accept:* a
      document whose embedded font fails to parse still lays out, and `doc_info` says which face
      was substituted and why.

### 4.2 Tranche two — the program reader (after phase H)

Real font-program parsing. The payoff lands with M2 and M7, so these queue behind the harness that
will measure them.

- [ ] **F4. Extract and identify font programs.** `/FontFile` (Type 1), `/FontFile2` (TrueType),
      `/FontFile3` (`/Type1C`, `/CIDFontType0C`, `/OpenType`); sniff the real format rather than
      trusting the key; wrap bare CFF in a minimal OpenType shell so one reader handles all of
      them. *Accept:* every variant identified from synthetic fixtures; a truncated or lying
      `/Length1` yields `Err`, never a panic.
- [ ] **F5. Code to glyph id.** Through the program's own cmap for simple fonts, and
      CMap to CID to GID through `/CIDToGIDMap` for composite ones. Builds on F2. *Accept:*
      symbolic TrueType, Identity-H, an embedded CMap and a `/Differences` override each resolve
      to the expected glyph id.
- [ ] **F6. Outlines and program metrics.** Outlines and advance widths via `skrifa`, with the
      dictionary's `/Widths` still winning where the two disagree (the specification makes the
      dictionary authoritative for layout). Type 1 (`eexec`, Type 1 charstrings) has no `skrifa`
      path: convert to CFF or interpret directly. *Accept:* outlines for TrueType, CFF and Type 1
      fixtures; a width disagreement resolves to the dictionary; a malformed charstring yields
      `Err`.
- [ ] **F7. Type 3 fonts.** Glyph procedures are content streams; run them through
      `printcraft-content` with the font matrix, bounded by the existing nesting caps. *Accept:* a
      Type 3 glyph renders and measures; a self-referencing glyph terminates.
- [ ] **F8. Subsetting.** A new program carrying only the glyphs used, ids remapped, for sfnt and
      CFF including CID-keyed. *Accept:* the subset is smaller, loads in `skrifa`, renders the
      same glyphs, and its document passes `qpdf --check`.
- [ ] **F9. Embedding.** Write a program back out with a correct descriptor, `/Widths` or `/W`,
      and a generated `/ToUnicode`, for simple and composite fonts. **This is what lets
      `edit.text-edit` stop silently substituting Helvetica** (M7.3). *Accept:* a document with
      newly embedded text extracts its own text through `printcraft-cli text`, and `pdftotext`
      agrees.
- [ ] **F10. Shaping.** Complex scripts and vertical writing via `rustybuzz`: cluster mapping, RTL
      runs, mark positioning, and `vrt2`/`vert` for vertical CJK. *Accept:* an Arabic and a
      vertical Japanese fixture shape to the expected cluster and advance sequence.

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

Needs F8–F10 (subsetting, embedding, shaping). The parity notes on `edit.text-edit` describe
today's limits precisely; keep them honest as each falls away.

- [ ] **M7.1. Paragraph model.** Lines to paragraphs with real measured metrics, not
      approximations.
- [ ] **M7.2. Reflow.** Re-wrap a paragraph to a new width and push the following content.
- [ ] **M7.3. Font substitution and embedding on edit.** When a character has no glyph in the
      line's font, subset and embed a face that does (F8/F9) instead of falling back to
      Helvetica.
- [ ] **M7.4. RTL and CJK editing** (`edit.rtl-cjk-editing`) on F10.
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
