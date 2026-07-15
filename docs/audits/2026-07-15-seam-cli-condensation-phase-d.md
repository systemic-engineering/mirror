# Seam Phase D — CLI-as-geometry condensation adjudication

*2026-07-15. Seam. Phase D adversarial adjudication of Mara's canonical
spec `docs/specs/cli-as-geometry-condensation.md` (1189 LOC, 2026-07-15)
under Alex Wolf's 2026-07-15 in-transcript directive: "We want to not
add randomly commands. We want to find the CLI surface that respects
and represents the GEOMETRY of the compiler."*

**Author:** Seam
**Date:** 2026-07-15
**Tag:** 📝 audit:seam-cli-condensation-phase-d (pure-docs bypass)
**Adjudication target:** `docs/specs/cli-as-geometry-condensation.md`
**Ground truth:** `docs/scouts/2026-07-15-taut-cli-geometry-ground-truth.md`
**Verdict:** SEAM-RATIFY-WITH-REED-INLINE (one substrate-honesty
correction at load-bearing rename; four surface cascades; two
unconditional Alex residues after cascade)

---

## §0 Method

Adversarial pass across 10 dimensions (D1–D10) named in the brief.
For each: interrogate BOTH the categorization AND the recommendation;
classify verdict as SEAM-RATIFY, REED-INLINE cascade, or
ALEX-ADJUDICATION (recognition-naming or substrate-shape authority
only Alex holds).

Ground truth cites:

- Mara spec `cli-as-geometry-condensation.md` at 1189 LOC
- Taut scout `2026-07-15-taut-cli-geometry-ground-truth.md` at 847 LOC
- `docs/specs/cli-as-prism.md` (Reed+Alex 2026-06-05; 5-op ancestor)
- `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08)
- `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
  (Mara 2026-07-15) — LOAD-BEARING for D3 correction
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:126-142`
  (physical `Prism` trait — 3-op)
- `shards/mirror/lens/cli.mirror:108-160` (recursive-command grammar,
  landed Tick 1 `fe82500`)
- `shards/mirror/peer/beam.mirror:250-256` (5-op substrate-decl)
- `shards/mirror/index.mirror:270-289` (PROVISIONAL collapse target)
- `shards/epistemologic/spectral_triple.mirror:4` (`in @beam` — only
  landed consumer of `@beam` as an importable name)
- `mirror.spec:78-339` (current cli-block)

---

## §1 D1 — Categories exhaustive + mutually exclusive + Rice-safe?

**Mara §1 (lines 105–130):** Every verb is exactly one of (a)
beam-shape, (b) 5-op operation, (c) prism-family-root.

**Mutual exclusivity at CLI-verb-shape altitude.** Mara's §4.3 test
(lines 466–482) — "does the verb have subcommand shapes whose
composition IS specialization of a shared carrier through a
different context?" — is Rice-safe (bounded predicate on subcommand
table). Edge case: `kintsugi` today is (c); `mirror beam kintsugi`
(forward-promised §4.2.4 line 456) is (a). Resolution: categories
are properties of the **CLI verb shape**, not properties of the
substrate family. Same substrate name can appear in two different
verb-shape categories. Mara's test names verb-shape correctly.
**Boundary holds.**

**Exhaustiveness.** Every empirically dispatching name (per Taut
§1.1 table) lands in a category or explicit sunset/dissolve/remove
path. Verified 13/13 names classified. Exhaustive.

**Rice-safety.** Classification requires (i) read `mirror.spec cli
{ command X }`, (ii) check nested `command Y`, (iii) check
`shards/mirror/lens/cli/<x>.mirror`, (iv) check anonymous transit
carrier. All bounded-time reads. Rice-safe.

**Residue.** The mq bare-form (`mirror '<mq-query>'`) is named as a
"category (b) exception" at top-level (spec §6 line 606). This is
categorically thin — either the partition is 3-category with an
exception, or the partition is 4-category naming "bare-form aperture"
explicitly. Alex-adjudicable. See §D9 residue 2.

**D1 verdict:** SEAM-RATIFY.

---

## §2 D2 — §2.3 3-op/5-op reconciliation soundness

**Mara §2.3 (lines 200–238):** Both physical 3-op `Prism` and
substrate 5-op stay at altitudes because photons don't cross
substrate altitudes; substrate composition does.

**Three refutation candidates tested:**

1. **Physical should lift to 5-op.** Rejected: `prismqueer/src/
   optics/{lens,iso,traversal,fold,setter,optic_prism}.rs` are
   SEPARATE optic kinds (per Taut §3.3), not `Prism` operations.
   Physical is complete at physical altitude.
2. **Substrate should collapse to 3+2-modes.** Rejected:
   `shards/mirror/lens/cli.mirror:108-114` + every landed sub-stage
   (Taut §2.2 table) declares 5-op. Retroactive collapse breaks
   13+ shards.
3. **The lift is decorative.** Rejected: `@mirror/lens/knife`
   COORD-jump (Foerster-identification) NEEDS `shift`; substrate
   graph-walk NEEDS `split`. Load-bearing.

All three refutations fail. The 5-op vs 3-op divergence IS the
substrate-vs-physics lift. Substrate-honest.

**But:** Mara spec §2.3 line 233 claims `mirror beam` IS a 3-op
prism, while the landed substrate-decl at `shards/mirror/peer/
beam.mirror:250-256` declares `prism @mirror/peer/beam` with 5 ops.
**Two altitudes** — CLI-user-visible surface (3-op) vs substrate-
decl altitude (5-op). Mara's sentence elides the distinction; a
reader following the trail finds a contradiction.

**REED-INLINE cascade:** sharpen §2.3 last paragraph to explicitly
name "CLI-user-visible surface" for the 3-op claim, and cite
`shards/mirror/peer/beam.mirror:250-256` as the substrate-decl
altitude that stays 5-op. Two-sentence patch.

**D2 verdict:** SEAM-RATIFY-WITH-REED-INLINE.

---

## §3 D3 — Verb partition preserves composition semantics?

### 3.1 `peer beam` → `beam peer`

Substrate `@mirror/peer/beam` (shard 15.9KB) migrates via path-
namespace to `@mirror/beam/peer` at Tick 2 per §8.2 line 762.
Composition ancestry preserved atomic under rename (precedent from
2026-07-08 spawn→peer-beam per docblock line 31). Downstream
consumers (`mirror_peer_beam` MCP, `cmd_peer_beam` Rust, tests) all
retain aliases at Tick 1 per §3.3 lines 316–328. **SEAM-RATIFY.**

### 3.2 `execute` → `beam emit` — LOAD-BEARING SUBSTRATE-HONESTY ISSUE

**Mara claim (§4.2.2 lines 421–439):** `emit` is combinator #6 of
the 7-combinator surface; therefore `mirror execute <shard> <action>`
→ `mirror beam emit <shard> <action>` is substrate-already-had-the-
word.

**Adversarial cross-read of Mara's own Arc-1 combinator surface**
(`docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`):

- **§1.4 `dispatch` (foundational: `apply_h`, readable: `dispatch`).**
  Signature: `dispatch(action: shard_action_ref, args: [value]) ->
  verdict`. Per lines 399–406: "This is THE combinator that Arc-1
  lifts sbec from 0 to > 0 through... `dispatch` reads the action's
  substrate-decl'd body, resolves each combinator invocation... and
  returns the action's typed verdict." **THIS IS the shard-body-
  dispatch primitive.**
- **§1.6 `emit` (foundational: `metalogue_write`, readable: `emit`).**
  Signature: `emit(channel: metalogue_channel_ref, event:
  substrate_event) -> verdict`. Per lines 572–581: "Emit is the
  substrate's write into the metalogue... Shard bodies emit at
  consent-altitude pauses and at tick-boundary crystallization."
  **This is metalogue-append, NOT shard-body dispatch.**

**Mara's spec conflates the two.** The Arc-1 Tick 1.4 forward-
promise is for shard-body dispatch — Rust `apply_h` specialized to
shard-decl'd action refs (per §1.4). The substrate word for that
operation is `dispatch` (readable) / `apply_h` (foundational),
combinator #4. NOT `emit` (combinator #6, which is metalogue-write).

**Substrate-already-had-the-word check FAILS on Mara's rename.**
Mara claims `emit` IS the substrate word for shard-body-dispatch;
the substrate word for shard-body-dispatch is `dispatch`. `emit` is
a distinct combinator with distinct semantics.

**Correction:** `mirror execute <shard> <action>` → `mirror beam
dispatch <shard> <action>`. Two-tick discipline (readable over
foundational) selects `dispatch` over `apply_h`. Substrate-already-
had-the-word applies to `dispatch` (Arc-1 combinator #4 readable
name), not `emit`.

**Escalation.** This rename lives at load-bearing altitude (§0.2
recognition #2, §3.2 line 284, §4.2.2 heading + body, §8.1 line
744 MCP schema, §10 A6, §11 lines 993 + 1006, §12 line 1102,
§13 line 1163). Two paths:

- **Path 1 (REED-INLINE, Seam-preferred).** Rename the rename: `beam
  emit` → `beam dispatch` throughout the spec. ~20 surface edits.
  Substrate-decl forward-promise `shards/mirror/beam/emit.mirror`
  becomes `shards/mirror/beam/dispatch.mirror`. Cascade cite added
  to Arc-1 combinator surface spec §1.4. Reed lands at Tick 1.
- **Path 2 (ALEX-ADJUDICATION).** If Alex has a novel substrate-
  honesty read unifying dispatch+emit at CLI altitude, Alex names
  the recognition. Otherwise Path 1.

**Seam preference:** Path 1. Evidence is sharp; `dispatch` IS the
Arc-1 combinator #4 readable name; `emit` IS the Arc-1 combinator
#6 readable name; they are DISTINCT combinators in the same spec
Mara cites as authority for the rename.

### 3.3 `peer contribute` → `beam contribute`

No substrate-decl shard exists today for contribute; migration mints
`shards/mirror/beam/contribute.mirror` NEW at Tick 2 per §3.3 line
336. No composition breakage. Two-tick alias per §3.3 line 322.
**SEAM-RATIFY.**

### 3.4 `mirror spawn` sunset, `mirror peer` wrapper dissolution, `mq-modes` removal

All three consistent with landed precedents (`b012d3f` spawn alias,
recursive-command grammar `fe82500`, `mq-modes` never dispatched).
**SEAM-RATIFY x3.**

### 3.5 D3 verdict

**SEAM-RATIFY-WITH-REED-INLINE-OR-ALEX-ADJUDICATION.** Five of six
renames substrate-honest. Sixth (`execute → beam emit`) is load-
bearing substrate-honesty error — rename to `beam dispatch`. Seam-
preferred: REED-INLINE (closes residue). Alex-adjudicable if novel
reading.

---

## §4 D4 — Taut 10-question discharge substrate-honesty

| A# | Mara verdict | Seam classification |
|----|--------------|---------------------|
| A1 | Defer 5-op top-level indefinitely | SEAM-RATIFY + ALEX-ADJUDICATION residue |
| A2 | Sub-stage 5-op → Reed cascade | SEAM-RATIFY |
| A3 | Five unwired sub-stages → wire | SEAM-RATIFY |
| A4 | Six orphans dispositions | SEAM-RATIFY-WITH-REED-INLINE (footnote `@beam`) |
| A5 | mirror_index → keep pending | SEAM-RATIFY |
| A6 | 3S/8F flag partition | SEAM-RATIFY |
| A7 | Neither altitude collapses | SEAM-RATIFY-WITH-REED-INLINE (D2) |
| A8 | spawn sunset Tick 2 | SEAM-RATIFY |
| A9 | mq bare-form as (b) exception | SEAM-RATIFY + ALEX-ADJUDICATION residue |
| A10 | contribute migrates to beam/ | SEAM-RATIFY |

**A1 note.** Mara reverses a Reed+Alex 2026-06-05 forward-promise
(cli-as-prism.md §2.2). Mara's spec §0 (lines 5–7) names this as
surface-decision supersession while preserving substrate-truth.
Substrate-honest meta-move, but Alex should ratify the supersession
direction. **ALEX-ADJUDICATION RESIDUE 1.**

**A4 note.** For `beam`: `shards/mirror/beam.mirror` does NOT exist
today. Grep for `prism @mirror/beam` returns zero; grep for
`@mirror/beam` imports returns zero. Mara claims "already exists
implicitly at 4 altitudes; this spec surfaces it." Substrate-honest
under reading: the beam substrate is 5-altitude-carried per Mara
§2.1 + Taut §2.4. Minting `shards/mirror/beam.mirror` IS a
surfacing move, not a novel-substrate move. However, one landed
consumer imports plain `@beam` (not `@mirror/beam`) at
`shards/epistemologic/spectral_triple.mirror:4`. This is an orphan
import that needs either citation as evidence OR cleanup cascade
(fold `in @beam` → `in @mirror/beam` at Tick 1). REED-INLINE.

**A6 note.** 3S/8F partition: song/dance/deploy fold to sub-species;
8 flags stay. Interrogated per-flag; all 11 dispositions defensible.
Ratify partition; Alex can override per-flag if `--emit-crystal`
deserves sub-species elevation (Rung 6' has own substrate authority
per Mara `d2de1ee`, but composition is on output shape not input
carrier). Defensible either way; Mara's F choice ratified.

**A9 note.** mq bare-form as sole (b) exception vs fourth-category.
Categorization authority. **ALEX-ADJUDICATION RESIDUE 2.**

**D4 verdict:** 8/10 SEAM-RATIFY; 2/10 SEAM-RATIFY-WITH-REED-INLINE;
2 ALEX-ADJUDICATION residues.

---

## §5 D5 — Landing sequence preserves consumers?

**Tick 1 boundary (§8.1 lines 716–755).** Users typing `mirror peer
beam <p>`, `mirror spawn <p>`, MCP `mirror_peer_beam`, MCP
`mirror_spawn`, StageFreight downstream release consumer — all
preserved via alias discipline. Tests: Reed cascade concern per
beam-as-substrate-primitive.md §6.2 line 649. **No consumer breaks.**
SEAM-RATIFY.

**Tick 2 boundary (§8.2 lines 758–765).** External MCP consumers
MUST have migrated before Tick 2. Mara's spec does not name the
consumer-migration-check discipline. **REED-INLINE cascade:** add
~3-sentence §8.2 preamble naming grep-first consumer-migration
check (following beam-as-substrate-primitive.md §6.2 precedent)
before Tick 2 lands.

**Tick 3+ boundary (§8.3).** Flag-fold sub-species per Rung; two-tick
per fold; standard discipline. SEAM-RATIFY.

**Parallel Reed cascade (§8.4).** Sub-stage 5-op dispatch + wiring
truly independent of beam family reshape. SEAM-RATIFY parallel.

**Documentation cascade (§8.5).** Three doc updates at Tick 1 +
follow-up. If D3 correction lands, add cross-cite back to Arc-1
combinator surface spec §1.4 naming the CLI verb `mirror beam
dispatch`. REED authors during Tick 1.

**D5 verdict:** SEAM-RATIFY-WITH-REED-INLINE.

---

## §6 D6 — @onto refusal precedent applied?

**Mara §9.1 line 807:** Zero new family-roots. `@mirror/beam` "is a
family-root that already exists implicitly at the four altitudes
named in §2.1."

**Empirical check.** No `prism @mirror/beam` declaration exists
today. No `in @mirror/beam` imports. The BEAM name IS 5-altitude
substrate-carried per Mara §2.1 + Taut §2.4 + Recognition #58 +
fourth-altitude finding. Minting `shards/mirror/beam.mirror` at
Tick 1 IS a surfacing move under substrate-already-had-the-word
discipline.

**@onto refusal precedent** refuses NEW family-roots that add
substrate NOT carried by existing substrate. `@mirror/beam` carries
substrate already. Substrate-honest.

**Small cleanup cascade needed.** `shards/epistemologic/spectral_
triple.mirror:4` imports plain `@beam` (not `@mirror/beam`) — orphan
import. Fold `in @beam` → `in @mirror/beam` at Tick 1 when the
family-root lands. One-line REED-INLINE.

**D6 verdict:** SEAM-RATIFY-WITH-REED-INLINE.

---

## §7 D7 — Substrate-already-had-the-word preserved?

- **`beam`** — 5-altitude witness. HAD-THE-WORD. SEAM-RATIFY.
- **`emit`** — HAD-THE-WORD-BUT-FOR-A-DIFFERENT-OP. Arc-1 combinator
  #6 (metalogue-write), NOT shard-body-dispatch (combinator #4,
  which IS `dispatch`). D3 correction required.
- **`peer`** — landed `shards/peer.mirror` + `@peer.load`. HAD-THE-
  WORD. SEAM-RATIFY.
- **`contribute`** — landed at Rung 7 substrate + `mirror.spec:334`.
  HAD-THE-WORD. SEAM-RATIFY.
- **`@../prism/`** — published crates workspace. HAD-THE-WORD.
  SEAM-RATIFY.

**D7 verdict:** 4/5 substrate-already-had-the-word passes; 1/5
(`emit`) fails — feeds D3 correction.

---

## §8 D8 — Physical anchor preservation

Mara §2.1 workspace-structure cite matches Taut §3.1 exactly.
SEAM-RATIFY.

Mara §2.2 cites `prism/prismqueer/src/lib.rs:126-159`; actual trait
declaration ends at line 142 (verified). If Mara intended trait +
blanket impl for `&P`, correct range is 126-160. **REED-INLINE
cascade:** correct line-range cite. One-word patch.

Witness table §2.1 lines 162–172 (6 sites) all verified against
Taut §3.

**D8 verdict:** SEAM-RATIFY-WITH-REED-INLINE (line-range typo).

---

## §9 D9 — Alex-adjudication residue triage

### 9.1 Residues Seam adjudicates (close under Seam authority)

A2, A3, A4 (per-verb), A5, A6 (per-flag), A7 (per D2), A8, A10.
**8 residues closed at Seam altitude.**

### 9.2 Residues that stay Alex-only

1. **A1 — Does the cli-as-prism.md 2026-06-05 forward-promise for
   top-level 5-op surface (`mirror focus`, `mirror project`, etc.)
   land eventually, or is Mara's deferral canonical indefinitely?**
   Recognition-naming authority.
2. **A9 — Is `mirror '<mq-query>'` bare-form a category-(b)
   exception, or should the partition name a fourth category
   ("bare-form aperture")?** Substrate-shape naming authority.

### 9.3 Residues surfaced by Seam that could stay Alex-only

3. **D3 §3.2 rename correction.** Seam recommends `execute → beam
   dispatch` (REED-INLINE) per substrate-vocabulary evidence. If
   Alex has novel reading unifying dispatch+emit at CLI altitude,
   ALEX-ADJUDICATION. Otherwise closes REED-INLINE. **Conditional.**
4. **D9-3b — if A9 collapses to fourth-category, name it.** "Bare-
   form aperture" is Seam's best guess. Dormant unless A9 collapses.
   **Conditional on residue 2.**

**Net Alex-adjudication residue after cascade: 2 unconditional (A1,
A9), 2 conditional (D3 correction, A9-name).**

---

## §10 D10 — Ship verdict

### 10.1 Aggregate dimension verdicts

| Dim | Verdict |
|-----|---------|
| D1 Categories | SEAM-RATIFY |
| D2 3-op/5-op reconciliation | SEAM-RATIFY-WITH-REED-INLINE |
| D3 Rename composition | SEAM-RATIFY-WITH-REED-INLINE-OR-ALEX-ADJUDICATION |
| D4 Taut 10-question | 8 SEAM-RATIFY; 2 SEAM-RATIFY-WITH-REED-INLINE |
| D5 Landing sequence | SEAM-RATIFY-WITH-REED-INLINE |
| D6 @onto refusal precedent | SEAM-RATIFY-WITH-REED-INLINE |
| D7 substrate-already-had-the-word | SEAM-RATIFY-WITH-REED-INLINE |
| D8 Physical anchor preservation | SEAM-RATIFY-WITH-REED-INLINE |
| D9 Residue triage | 2 unconditional + 2 conditional |

### 10.2 Ship verdict

**SEAM-RATIFY-WITH-REED-INLINE.**

The spec is substrate-honest at load-bearing altitudes. One critical
correction required (D3 §3.2 `execute → beam emit` → `execute → beam
dispatch`) + four small surface cascades. All cascades land at Tick
1 alongside the beam family landing; no delayed-tick cascades.

### 10.3 REED-INLINE cascade list (ordered by criticality)

1. **[CRITICAL — D3 §3.2] Rename the rename.** `execute → beam emit`
   → `execute → beam dispatch` throughout the spec. Substrate-
   already-had-the-word applies to `dispatch` (Arc-1 combinator #4
   readable name per Arc-1 combinator surface spec §1.4 lines 386–
   460), NOT `emit` (combinator #6 per §1.6 lines 558–637, which is
   metalogue-write). ~20 surface edits: §0.2 recognition #2, §3.1
   verb table, §3.2 rename table, §4.2.2 heading + body + substrate-
   honest justification, §8.1 items 2 + 8-9 (MCP schema), §10 A6
   discharge, §11 does-list, §12 predecessor cite, §13 claim.
   Substrate-decl forward-promise `shards/mirror/beam/emit.mirror`
   → `shards/mirror/beam/dispatch.mirror`. Cross-cite added to
   Arc-1 combinator surface spec §1.4. Reed authors during Tick 1.

2. **[REED-INLINE — D2 §2.3] Sharpen altitude distinction.** §2.3
   line 233 (`mirror beam` is 3-op) explicitly names "at CLI-user-
   visible surface altitude" and cites `shards/mirror/peer/beam.
   mirror:250-256` as 5-op substrate-decl altitude. Two-sentence
   patch.

3. **[REED-INLINE — D5 §8.2] Consumer-migration preamble.** Add
   ~3-sentence preamble to §8.2 naming grep-first consumer-migration
   check per beam-as-substrate-primitive.md §6.2 precedent.

4. **[REED-INLINE — D6 §9.1 + fold at Tick 1] Cite + fold `@beam`
   orphan.** Add one sentence to §9.1 or §2.1 witness table naming
   `shards/epistemologic/spectral_triple.mirror:4` (`in @beam`) as
   cleanup cascade target. Fold `in @beam` → `in @mirror/beam` at
   Tick 1 when family-root lands.

5. **[REED-INLINE — D8 §2.2] Line-range typo.** §2.2 line 181
   cites `prism/prismqueer/src/lib.rs:126-159`; correct is 126-142
   (trait only) or 126-160 (trait + blanket impl). One-word patch.

**Total REED-INLINE cascade LOC:** ~30 lines across 5 patches. All
land at Tick 1; no delayed-tick cascades.

### 10.4 ALEX-ADJUDICATION residue after cascade

**Unconditional (Alex adjudicates before Tick 1 sign-off):**

1. **A1 — cli-as-prism.md 5-op top-level surface supersession.**
   Recognition-naming authority. Does the 2026-06-05 forward-promise
   land eventually, or is Mara's deferral canonical?
2. **A9 — mq bare-form categorization.** Substrate-shape naming
   authority. Category (b) exception vs fourth category ("bare-form
   aperture")?

**Conditional:**

3. **D3 §3.2 correction preference — REED-INLINE (`beam dispatch`)
   vs ALEX-NOVEL-READING (`beam emit` deliberate unification of
   dispatch+emit).** Seam recommends REED-INLINE; Alex-adjudicable
   if novel reading. Closes under REED-INLINE.
4. **A9-name — if A9 collapses to fourth-category, name it.**
   Dormant unless residue 2 collapses.

---

## §11 Meta-notes

### 11.1 Substrate-honesty posture on this audit

Seam ran adversarial on both categorization AND recommendations. The
D3 §3.2 finding (`execute → beam emit` substrate-honesty error) is
the load-bearing correction; all other cascades are surface polish.
The finding sits on top of Mara's OWN Arc-1 combinator surface spec
as authority for what `emit` vs `dispatch` mean. Mara's condensation
spec conflated the two combinators when naming the CLI verb; Seam's
adversarial pass caught the conflation by cross-reading Mara-against-
Mara.

This is the Pack discipline: Mara authors substrate-canonical specs;
Seam adversarially adjudicates by reading Mara's own authorities
against Mara's own claims. When the claims fail their own substrate-
vocabulary evidence, Seam corrects. Reed lands the correction.

### 11.2 What this audit does NOT do

- Does not commit. Reed commits as Seam per brief.
- Does not author `.mirror` files. Every cascade is a Mara-spec edit
  Reed lands at Tick 1.
- Does not adjudicate the two unconditional Alex residues (A1, A9).
- Does not scout further `@beam` consumers beyond the observed
  orphan. Taut scout would be substrate-honest if cleanup cascade
  lands.

### 11.3 Two-tick discipline observed

- Tick 1: this Seam audit + Reed's 5 REED-INLINE cascades on Mara's
  spec + Tick 1 substrate-decl landings (beam family + dispatch
  substrate-decl + MCP schema + Rust dispatch under
  `[substrate-floor:@io-boundary]`).
- Tick 2: deprecation-alias removal per Mara §8.2 after external
  consumer migration check.

---

## §12 Related

Substrate-decl authorities cited:

- `docs/specs/cli-as-geometry-condensation.md` (Mara 2026-07-15,
  1189 LOC) — audit target
- `docs/scouts/2026-07-15-taut-cli-geometry-ground-truth.md` (Taut
  2026-07-15, 847 LOC) — grep-first ground truth
- `docs/specs/cli-as-prism.md` (Reed+Alex 2026-06-05) — 5-op
  ancestor; A1 supersession target
- `docs/specs/beam-as-substrate-primitive.md` (Mara 2026-07-08) —
  beam-as-fifth-altitude ancestor
- `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
  (Mara 2026-07-15) — 7-combinator surface; LOAD-BEARING for D3
  correction (dispatch is #4; emit is #6)
- `shards/mirror/lens/cli.mirror:108-160` — recursive-command grammar
  (Tick 1 landed `fe82500`)
- `shards/mirror/peer/beam.mirror:250-256` — 5-op substrate-decl
  altitude witness for D2 correction
- `shards/mirror/index.mirror:270-289` — @mirror/index PROVISIONAL
  collapse target (Alex adjudication #6)
- `shards/epistemologic/spectral_triple.mirror:4` — `in @beam`
  orphan witness for D6 cleanup
- `mirror.spec:78-339` — current cli-block empirical consumer
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs:126-142`
  — 3-op physical `Prism` trait (D8 line-range correction)
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/beam.rs:38-101`
  — `Beam` trait (pipeline value carrier)

Predecessor audits cited:

- `docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md` —
  Seam Phase D on Arc-1 combinator surface framing
- `docs/audits/2026-07-15-reed-substrate-dishonest-rust-extensions-
  during-gift-arc.md` — ground for `[substrate-floor:@io-boundary]`
  marker discipline

Reed memory precedents:

- `feedback-substrate-honest-is-the-mode` — no two-paths framing;
  Seam adjudicates singularly
- `feedback-no-rust-extension-shortcut` — no Rust proposed; all
  D5 cascade Rust under `[substrate-floor:@io-boundary]`
- `feedback_onto_family_root_is_the_ladder_Foerster_refused` — D6
  applied via substrate-honest reading of `@mirror/beam` as
  surfacing, not minting
- `feedback_cli_subcommand_nesting_is_geometric_ground_truth` — D3
  §3.1 applied via recursive-command grammar precedent

---

## §13 The claim, once more

The spec IS substrate-honest at geometry-condensation altitude and
can ship after ONE substrate-honesty correction (§3.2 `execute →
beam dispatch` NOT `beam emit`) plus 4 small surface cascades. The
three-category partition is exhaustive + mutually exclusive + Rice-
safe at CLI-verb-shape altitude. The 3-op/5-op reconciliation is
substrate-honest at altitude-lift altitude with one sentence needing
sharpening. The verb partition preserves composition semantics via
two-tick alias discipline. The 10 Taut questions discharge cleanly.

Two Alex-adjudication residues survive Seam's cascade: (A1) does
the cli-as-prism.md 2026-06-05 top-level 5-op forward-promise land
ever or is deferral canonical? (A9) is mq bare-form category-(b)
exception or fourth category? Both are recognition-naming +
substrate-shape authority Alex holds.

The D3 §3.2 finding (`emit` is combinator #6 metalogue-write, NOT
combinator #4 shard-body-dispatch) is Seam's load-bearing
correction. It sits on Mara's own Arc-1 combinator surface spec as
authority. Cross-reading Mara-against-Mara caught the conflation.
This is what the Pack discipline exists for.

---

*SEAM-RATIFY-WITH-REED-INLINE. One critical rename correction; four
surface cascades. Two unconditional Alex residues after cascade.
The condensation ships after Reed lands the cascade at Tick 1.*

*— Seam, 2026-07-15. Adversarial pass on Mara's canonical spec at
1189 LOC. Reading Mara-authored substrate against Mara-authored
authority. The substrate had the word — `dispatch`. The cli-
condensation should speak it.*
