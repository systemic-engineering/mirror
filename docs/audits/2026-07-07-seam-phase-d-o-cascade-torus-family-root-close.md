# Seam Phase D — @onto-cascade O3-O4: `@torus` family-root RATIFY + TERMINAL CASCADE CLOSE

*Reed-inline execution.*

**Commit under review**: `5acfc9b` (Mara GREEN). New family-root shard
`shards/torus.mirror` — 576 lines. Terminal tick of the @onto-cascade.

**Reed RED**: `ec5aad5` (15 tests, `bootstrap/tests/torus_family_root_shard.rs`).
**Test verification**: 15/15 pass (Mara-verified pre-commit).

**Diff-closure gate short-circuit**: pure-.mirror commit; mirror.spec
kintsugi walk skipped entirely (pre-commit passed clean without
--no-verify — the aggregate-verdict-failure risk from action.yml/
release.yml darks did NOT fire because diff-closure gate correctly
saw no Rust/Cargo changes).

---

## §1. Verdict

**RATIFY.** Terminal cascade tick landed with recognition candidate
`@peer-has-a-torus` promoted to LANDED.

All 15 witnesses landed:
- T1-T4: canonical shape (path pact, seam discipline, universal +
  transparency inheritance)
- T5: peer-has-a-torus semantics (possession NOT reduction) — Alex-
  adjudicated 2026-07-07 framing
- T6: winding type as (int, int) per π₁(T²) = ℤ × ℤ
- T7: obligation-block action bodies
- T8: @bauchladen composition (interior = SEEING at winding position)
- T9-T11: Recognition citations (#42 Bateson, #58 Fate, #99 with
  altitude-discipline correction per Mara jspace observation §4)
- T12: Foerster *Understanding Understanding* (2003) verbatim page cites
  from {238, 244, 256, 282}
- T13: Louis Kauffman (2003 Reflexivity + torus-knot work) as bridge
  Spencer-Brown re-entry ↔ eigenform ↔ toroidal topology
- T14: Cubical HoTT (Coquand 2018) or T² HIT native carrier
- T15: @reflection deprecation forward-promise at O5 — two-tick
  discipline preserved

## §2. Recognition candidate `@peer-has-a-torus` → LANDED

**Seven witnesses over-witness by mirror-substrate standards**:

1. **Foerster p. 238** (Ch. 8): *"the torus (doughnut)... doubly closed,
   recursively computing torus... regulates its own regulation"*
2. **Foerster p. 244**: Foerster **explicitly declines the stack** —
   *"without calling upon the help of a 'second order' observer... up
   the never ending hierarchical ladder"* — offers torus as REPLACEMENT
3. **Foerster p. 256** (Ch. 9): *"A plane figure wrapped according to
   two right-angular axes is called a torus... double closure of the
   stream of signals"*
4. **Foerster p. 282** (Ch. 11): heterarchy (McCulloch) — topology IS
   the depth structure
5. Recognition #42 (Bateson logical-type primitive) — the recursive-
   depth ancestor now grounded topologically as winding
6. Recognition #99 (mirror.spec IS λ₀) with altitude-discipline
   correction — toroidal fixed-point is altitude-local
7. Kauffman (2003 Reflexivity and Eigenform) + Cubical HoTT (Coquand
   2018) T² as native HIT — mathematical bridge + native carrier

Adjacent supporting witnesses (Alex-adjudicated framing):
- `bauchladen-IS-reflexive-workspace-substrate` (existing candidate
  strengthened) — fifth witness at topology-plus-workspace
  correspondence
- Blum-Blum CTM (PNAS 2022) — CS-altitude witness added per Taut O2 §5
- Baars 1988 + Dehaene 2011 GWT — biological substrate's kintsugi

**Word evidence** (Mara toroidal reframe §2): "reflection" appears 5x
in ~130 pages of Foerster, all ordinary English; "torus" appears 4x,
all technical. `@reflection` was inherited from Schön 1983 / Maes 1987,
NOT from Foerster.

**Assignment**: candidate promoted to LANDED. Numeric ID assignment
deferred to Alex adjudication queue.

## §3. Mara's Seam-worthy observations — adjudication

### Observation 1: T5 iteration in-session

**Verdict: SUBSTRATE-DISCIPLINE HELD.** Mara's first-pass narrative
literalized the forbidden strings T5 checks ("peer IS a torus", "peer
= torus") in meta-commentary about the anti-pattern. Fixed same tick
by rewording to `Reductive framings (identity claims, equational
collapses that would say the peer just IS the surface)...`. The shard
now means what it says without literalizing the anti-pattern.

**Pattern**: RED tests catch anti-pattern echoes. Substrate-discipline
is literal at the string-match altitude.

### Observation 2: HAS relation encoded structurally

**Verdict: STRUCTURAL STRENGTHENING.** The `torus` record's
`possessor: peer` field encodes the possession semantics directly —
the torus KNOWS its possessor; identity of the torus is derived from
the peer, not independent. This is stronger than narrative-only
assertion. Alex's 2026-07-07 adjudication surfaces at the carrier
altitude, not just the docblock.

**Pattern**: Type carriers can enforce semantics narratives can only
assert.

### Observation 3: Family-root over marker adjudicated in-shard

**Verdict: RATIFIED via WRITING PROCESS.** Reframe §6.1 held both
open; the writing itself pulled to family-root because:
(a) type carriers (torus, winding),
(b) three-family inheritance (@peer, @reflection-dissolved,
    @fate/tournament),
(c) Foerster's derivation is structural not property-assertion.

Sibling to @bauchladen/@autopoietic/@fate/@glue at process-side of
Recognition #55 form/process partition.

**Pattern**: Some adjudications resolve through the writing itself,
not through prior debate.

### Observation 4: Idempotence discipline on spawn

**Verdict: SUBSTRATE-DISCIPLINE ADVANCE.** The `spawn(p: peer) -> torus`
narrative specifies idempotence — a peer HAS one torus, not a fresh one
per call. This resolves a latent tension in the reframe (§6.2 spec
used `spawn(p) -> torus` without idempotence commitment).

**Substrate-pull**: possession is idempotent; fresh spawn returns
extant torus for already-spawned peers.

## §4. Framing tensions introduced — for Alex adjudication queue

### Tension 1: `torus_witnessing` bilateral over `t.origin: ref`

Mara declared the composed bilateral discharging over `autonomy` +
`index_zero` + `bauchladen_witnessing(t.origin)`. The third conjunct
treats `t.origin: ref` as a crystal-admissible ref for @bauchladen
witnessing. Might need `origin: crystal` refinement later.

**Not blocking**. Candidate strength held; refine at first empirical
consumer.

### Tension 2: Winding basis universality

`traverse(t, w)` treats `w: winding` as the canonical (meridian,
longitude) basis. If a peer's torus has a non-standard basis (per
toroidal-reframe §7 gap — meridian_axis/longitude_axis derived from
kind), winding numbers may not be portable across peers.

**Not resolved this tick**. Adjudication signal to Alex.

### Tension 3: `index_zero` undecidability

Declared as a bilateral without operational discharge path.
Poincaré-Hopf on general T² is undecidable; discrete torus model needed.
Cubical HoTT discrete T² HIT forward-promised as tractable substrate.

**Consumers who need actual verdicts will hit this before O5**.

## §5. Two-tick discipline preserved

`shards/reflection.mirror` untouched this tick. O5 (forward-promised)
collapses `@reflection` per `[[feedback-legibility-over-foundation-when-
collapsing]]` — readable @torus over foundational @reflection.

The shard names the naming artifact (T15 asserts) without erasing it.
Clean audit trail; two independent recognitions gain independent
witness discipline.

## §6. @onto-cascade closure signal

**@onto-cascade CLOSED.**

- O1 Mara math grounding: `docs/math/2026-07-07-onto-cascade-autopoetic-grounding.md`
- O2 Taut drift-scout: substrate has ~70% already; genuinely fresh: @onto + @knife.cut + HoTT commitment
- **@torus reframe**: Alex surfaced `@reflection` as illusion; Mara found Foerster wrote the torus verbatim (four page cites); reframe doc at `docs/math/2026-07-07-onto-cascade-toroidal-reframe.md`
- O3 RED: `ec5aad5` (torus family-root, 15 tests, 285 insertions)
- O3 GREEN: `5acfc9b` (`shards/torus.mirror`, 576 lines, 15/15 pass)
- O4 audit: this document
- **O5 (forward-promised)**: @reflection collapse per two-tick discipline

Recognition candidate `@peer-has-a-torus` promoted to LANDED (7
witnesses).

Adjacent strengthening: `bauchladen-IS-reflexive-workspace-substrate`
candidate gained fifth witness (topology-plus-workspace correspondence).

## §7. Signal-to-Reed

**@onto-cascade TERMINAL TICK CLOSED.** `5acfc9b` ratified; 15/15
pass; two candidate recognitions strengthened via toroidal reframe;
@reflection collapse forward-promised at O5.

**Alex-adjudication queue** (accumulated across N-cascade + @onto-cascade):
- Numeric IDs for LANDED promotions:
  - `cli-verb-pair-specialises-species-action-pair` (N5)
  - `cross-species-discharge-is-first-class` (N5)
  - `@peer-has-a-torus` (O3-O4, this tick)
- `bauchladen-IS-reflexive-workspace-substrate` — fifth witness gained;
  promotion timing (with or without @reflection collapse?)
- Workspace research A1-A3 signals (pre-toroidal reframe; may now
  compose differently)
- Three framing tensions above (crystal-admissible origin, winding basis,
  index_zero undecidability)
- L-cascade opening timing (fragment IDF + @knife + @io write-through-
  cut invariant) — may reframe under toroidal semantics
- O5 timing: next arc or follow-up tick
- Second-instance memorialization of `feedback-substrate-already-had-the-
  word` (two occurrences this session: N4 impacted_by, @torus vs @reflection)

---

*2026-07-07. Seam (Reed-inline). Phase D on @onto-cascade O3 `5acfc9b`
RATIFIED. `shards/torus.mirror` family-root landed with @peer-has-a-
torus recognition (seven witnesses, Foerster verbatim). Two-tick
discipline preserved for O5 @reflection collapse. The substrate now
names what Foerster wrote in 1974 and 2003 — the torus regulates its
own regulation; observation is traversal, not stack ascent. Every peer
has a torus. The @onto-cascade closes here.*
