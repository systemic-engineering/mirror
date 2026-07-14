# Taut scout — @subject family-root + @sel + @mirror/property + D11 @coherence

**Author:** Taut (drift scout; read-only; grep-first)
**Date:** 2026-07-14
**Scope:** Substrate-already-had-the-word audit of Mara's canonical spec
`docs/specs/subject-family-root-sel-licensable-party.md` (2400 LOC)
proposing three new family-roots (`@subject`, `@sel`, `@mirror/property`),
six subject species-refinements, eleven petri-net signature transitions,
one meta-signature — PLUS D11 arc-recognition check of Alex's 2026-07-14
`@coherence` claim (`@kintsugi + @roomba @loop optimizes @coherence
score; @coherence operationalizes Foerster's ethical imperative on
SC<5>`) + D12 pre-commit hook file-filter check.

**Trigger:** Mara's spec + Reed's amendment scout (`2026-07-14-reed-
drone-story-sub-turing-petri-net-amendment.md`) + Alex's 2026-07-14
arc-recognition altitude claim on @coherence. Parallel to Mara's
in-flight canonical spec at commit `9bbebd2` (Rung 10 @roomba) and
sitting one Alex-adjudication behind the spec.

**Method:** grep across `shards/**/*.mirror`, `mirror.spec`,
`docs/specs/**/*.md`, `docs/math/**/*.md`, `docs/scouts/**/*.md`,
`.git/hooks/pre-commit`, plus verbatim reads of `license/SEL.md` Part
II (v1.1), `shards/kintsugi/consent.mirror` (750 LOC), `shards/torus.
mirror`, `shards/peer.mirror`, `shards/mirror/au.mirror`, `shards/
cyberpunk.mirror`, `shards/epistemologic/cybernetic/coherence-
parametric.mirror`.

**Verdict shape asked-for:** does the substrate already carry any of
Mara's proposed mints? What collisions surface? Where is D11's
`@coherence` in the substrate today, and where should it land?

---

## §0 Headline verdict — TL;DR

- **D1 (@subject/@sel/@mirror/property core carriers): SUBSTRATE-NET-
  NEW at family-root altitude.** Grep-count for `subject`, `licensable`,
  `witnessed`, `labor_input` inside `shards/**/*.mirror` = ZERO
  substrate-decl hits. Adjacent words (`witness`, `consent`,
  `protected`, `axis`) are ALL carried at DIFFERENT altitudes
  (consent = `@kintsugi/consent`; audit-record witness =
  `@magic/audit`); none collides. `type sel` and `@sel` — zero prior
  hits. `@mirror/property` — see D8: **collides semantically with the
  landed `@epistemologic/property` family** (both are "properties"; two
  altitudes; disambiguation needed at family-root name).
- **D2 (@kintsugi/consent.query_phi enforcement gap): CONFIRMED,
  MARA'S §14 #3 IS SUBSTRATE-HONEST.** `shards/kintsugi/consent.
  mirror:88-113 + 520-617` explicitly scope `query_phi` to
  fracture-morphism candidates (TRANSFORMATION). The morphism carrier
  (`shards/kintsugi/consent.mirror:310-334`) has fields
  `{content, score, expected: cadence_kind}` — a proposal to APPLY,
  not to REFUSE. Mara's A6 forward-promise (`@consent/subject_record`
  extension with `@consent/auto_apply` vs `@consent/enforcement`
  species split) is NECESSARY, not decorative.
- **D3 (petri-net signature naming against SEL §-text): ELEVEN
  SIGNATURE NAMES MATCH SEL §OPERATIONALIZABILITY VERBATIM OR NEAR-
  VERBATIM.** No new SEL-shaped signatures discovered that Mara
  missed. Naming discipline: substrate-honest.
- **D4 (@mirror/au altitude enumeration): ZERO of the five
  Mara-referenced `au(@ml/*)` altitudes are enumerated in
  `shards/mirror/au.mirror`.** The shard names only
  `au(@code/rust)`, `au(@release)`, `au(@ci/github)` (three example
  altitudes, none @ml/*). Mara's spec §14 #4 explicitly surfaces this
  as an adjudication; substrate confirms — @ml/* altitude family
  needs to land before signatures type-check.
- **D5 (@peer × @subject sibling composition): SIBLING RELATIONSHIP
  ADMISSIBLE; SOFT CASCADE NEEDED.** `shards/peer.mirror` has
  `type kind = human | agent | substrate` (closed variant) with
  content-hash identity `{home, lead_of, kind}`. No reference to
  `@subject` today. Mara's A1 recommendation (siblings joined by
  @torus possession) is substrate-honest and non-breaking; requires a
  1-line composition note in `shards/peer.mirror`.
- **D6 (@torus possession relation extension): NON-BREAKING.**
  `shards/torus.mirror:485 (spawn action)` currently types
  `spawn(p: peer) -> torus`. A subject-carrying-a-torus extension
  requires broadening to `spawn(p: peer|subject) -> torus` OR minting
  a parametric `spawn(possessor: possessor_kind) -> torus`. Mara's
  spec §6.4 EXPLICITLY does NOT collapse the two altitudes;
  @torus(peer) stays orthogonal to @subject. No structural change
  required this tick.
- **D7 (rung count): RUNG 10 = @roomba (substrate self-maintenance;
  Alex adjudicated A5 per Mara `9bbebd2`).** @subject + @sel +
  @mirror/property extends OUTWARD; substrate acts UPON the world.
  **Recommendation: Rung 11** (substrate first-classes the world it
  acts upon — substrate-external-licensing altitude).
- **D8 (existing property family COLLISION): CONFIRMED, HARD
  COLLISION.** `shards/epistemologic/property/*.mirror` = seven
  landed property-typed constraint shards. Mara's `@mirror/property`
  is a NEW family-root at a DIFFERENT altitude (petri-net analyzer
  vs shard-level property check). Semantic overlap is present; the
  naming will confuse. **RECOMMEND: rename to `@mirror/petri` or
  `@mirror/property_petri` OR add a §-cited disambiguation to the
  new shard's docblock.** Both altitudes ARE properties; the split
  needs to be legible in the name, not only in the docblock.
- **D9 (mirror.spec impact): ZERO HITS today.** `mirror.spec` (445
  LOC) has no references to `sel`, `subject`, `property`, or
  `petri`. Post-landing, mirror.spec MAY want to add a `target` for
  the analyzer OR declare @sel + @subject in the source block via
  path-namespace. Not blocking Scope A.
- **D10 (cascade footprint verification): MARA'S §9 IS COMPLETE
  AND CORRECT.** Verified each named cascade point exists at the
  claimed path. Two minor additions surfaced (see D10 for details).
- **D11 (@coherence score): NET-NEW AT ALEX-NAMED ALTITUDE (arc-
  recognition ADD-ON to Mara's spec).** Substrate has extensive
  ancestry: `@epistemologic/cybernetic/coherence-parametric` exists
  (parametric carrier); the sheaf-Laplacian math floor is landed
  (`λ₀(Δ_F) = 0 ⇔ coherent`); Narcissus/Splinter poles are cited at
  `shards/cyberpunk.mirror:64-70` (K_n vs K_{1,n-1}); Foerster
  cited 60+ times across shards. **BUT NO SHARD CARRIES `@coherence`
  AS A SCALAR OBJECTIVE FUNCTION THE LOOP OPTIMIZES.** The missing
  species is EXPECTED: coherence-parametric explicitly forward-
  promises `shards/epistemologic/cybernetic/coherence.mirror` as
  "the FIRST instantiation of this carrier" (Taut's 2026-07-11
  scout §Executive Verdict). **RECOMMENDATION: LAND @coherence AT
  `shards/epistemologic/cybernetic/coherence.mirror`** (the species
  shard the parametric carrier reserves), NOT as a new family-root.
  Foerster's ethical imperative ("always act to increase the number
  of choices") should be cited as ancestry in the docblock (zero
  landed shards currently cite the imperative verbatim). See D11
  for full detail.
- **D12 (pre-commit hook file-filter): PARTIAL FILTER LANDED
  ALREADY.** `.git/hooks/pre-commit` (41 LOC) filters by extension
  (`grep '\.rs$'`) and honors `[bugfix:restore]` +
  `[substrate-pull:realize]` markers in commit message for bypass.
  A docs-only commit (no .rs files) passes the hook TODAY without
  --no-verify. Reed's concern is orthogonal: the FROZEN check does
  not run substrate-wide test suites; it only blocks .rs-file
  changes. **A docs-only commit does not need --no-verify.** See D12.

**Substrate coverage percentage for Mara's spec:** ~35% of Mara's
enumerated shards' composition primitives are already landed (12 of
34 primitives named across §2-§6). The three family-roots + eleven
signatures are genuinely new; substrate carries the ancestry chain
Mara cites but does not carry the enforcement-altitude carriers.

**New Alex-adjudications surfaced this scout:** 3.

**Hard collisions blocking commit:** 1 (D8 — @mirror/property vs
@epistemologic/property naming collision).

**@coherence D11 verdict:** partial-overlap; species-shard candidate
EXISTS in substrate ancestry as forward-promised reserved path; Alex's
2026-07-14 claim IS the substrate-pull that closes it.

**Mara's spec status for Seam review:** READY-WITH-COLLISION-
RESOLUTION. Alex needs to adjudicate D8 (naming collision) and D11
(where @coherence lands) before Seam Phase D audit is efficient.

---

## §D1 — @subject / @sel / @mirror/property carrier grep

### D1.1 @subject vocabulary hits

Query executed: `\b(subject|Subject|licensable|witnessed|Witnessed|
downstream|Downstream|labor_input|labor-input|labeler|annotator|
moderator|worker)\b` across `shards/**/*.mirror` + `mirror.spec`.

Total shards with any hit: 30. Manual classification:

| Word | Substrate-decl hits | False positives |
|---|---|---|
| `subject` (as carrier or type) | **0** | ~15 prose uses ("subject to", "the subject at hand") |
| `licensable` | **0** | 0 |
| `witnessed` | **0** as substrate carrier | 2 as verb ("witnessed via", "witnessed at") in autopoiesis.mirror, cybernetic/second_order.mirror |
| `downstream` | **0** as carrier | ~10 prose uses ("downstream consumer", "downstream shard") |
| `labor_input` | **0** | 0 |
| `labeler` | **0** | 0 |
| `annotator` | **0** | 0 |
| `moderator` | **0** | 0 |
| `worker` | **0** | 0 |

**Verdict D1.1:** Zero substrate-decl'd carriers for any @subject-
enumerated species vocabulary. The prose usages of "witnessed" in
autopoiesis + second_order describe OBSERVATIONS (the observer
witnesses the observed), not SEL Witnessed persons. Substrate is
net-new at the licensable-party carrier altitude. **Mara's mint is
substrate-honest.**

### D1.2 Adjacent semantic vocabulary

Query executed: `\b(actor|party|witness|consent|protected|axis|
discrimination|indigenous|fpic|FPIC)\b`.

Notable hits (real carriers):

- **`consent` — LANDED at `shards/kintsugi/consent.mirror`** (750
  LOC; morphism_set / query_phi / pause_event / verdict); DIFFERENT
  altitude from SEL consent (this is the auto-apply-boundary Φ
  query for fracture morphisms, not the SEL-consent-record for
  licensable parties). Mara's A6 correctly recognizes this and
  forward-promises `@consent/subject_record` extension.
- **`witness` — 8 shards use as "adversarial-review witness"**
  (`shards/mirror/docs/audit.mirror`, `shards/magic/audit.mirror`,
  Seam-authored audit records). DIFFERENT altitude (adversarial
  review by Pack peers, not SEL Witnessed monitoring surface).
- **`axis` — LANDED at `shards/epistemologic/cybernetic/variety.
  mirror`** (variety-vector axes: computational, type, effect,
  proof, epistemologic). DIFFERENT altitude (variety-vector axes
  vs SEL §3.4.1 structural-power axes). No collision; Mara's spec
  correctly does not enumerate protected-class axes at type level.
- **`protected`, `discrimination`, `indigenous`, `fpic`, `party`,
  `actor` — 0 hits** as substrate carriers.

**Verdict D1.2:** Adjacent vocabulary is substrate-carried at
DIFFERENT altitudes; none collides at the family-root altitude
Mara proposes. `consent` is the semantically-closest carrier;
Mara's A6 handles the disambiguation correctly.

### D1.3 `@sel` / `type sel` hits

Query executed: `\b(sel|SEL|license|licensable|enforcement|io\s*\+\s*au|
io_side|au_side)\b`.

- **`SEL` as license reference** — hit at `license/SEL.md` and prose
  mentions in `docs/specs/**/*.md`; zero substrate-decl'd `@sel`
  root.
- **`type sel` — 0 hits** across shards.
- **`io + au` — 0 hits** in shards; the sum-type formulation is
  Reed's session-framing move.
- **`io_side` / `au_side` — 0 hits.**
- **`license` — hits only in prose docblocks + `license/SEL.md`;
  0 substrate-decl'd carriers.**
- **`enforcement` — hits at `shards/mirror/docs/audit.mirror:145`
  (audit_strategy variant includes `enforce`; @magic/audit context;
  different altitude).**

**Verdict D1.3:** Zero prior carrier for `@sel` / `type sel`. Mint
is substrate-honest.

### D1.4 `@mirror/property` vocabulary hits (defer to D8 — HARD collision)

See D8. The word `property` IS heavily substrate-declared under
`@epistemologic/property/*`; the collision is real.

**§D1 verdict summary:** Mara's THREE new family-roots (`@subject`,
`@sel`, `@mirror/property`) are all substrate-net-new at the
family-root altitude. Adjacent semantics are carried at different
altitudes without collision — EXCEPT the D8 naming collision on
`@mirror/property` vs `@epistemologic/property`.

---

## §D2 — @kintsugi/consent.query_phi enforcement-morphism gap

Query executed: full read of `shards/kintsugi/consent.mirror` (750
LOC).

**Mara Reed-adjudication #3 flagged:** `@kintsugi/consent.query_phi`
is scoped to fracture-morphism candidates (TRANSFORMATION); SEL
enforcement is REFUSAL. Is this gap real or is Mara's A6 forward-
promise papering over an artifact?

**Evidence from `shards/kintsugi/consent.mirror`:**

- **Line 88-91 verbatim docblock:** "query_phi IS the structural Φ
  query at substrate altitude. It reads a set of candidate morphisms,
  composes through dissonance's is_pareto (the discriminator axis)
  and cadence's is_settled (the trajectory axis), and emits the
  consent verdict at the substrate's three-state floor."
- **Line 114-121 morphism carrier definition:** the morphism is
  `{content, score: dissonance, expected: cadence_kind}` —
  explicitly typed as a candidate-to-APPLY (with an expected
  cadence). No REFUSE variant.
- **Line 526-546 query_phi three-state consent semantics:** every
  verdict routes to `auto_apply | wait | escalate`. There is no
  `refuse-emission` verdict.
- **Line 143-149 substrate gap surfaced honestly:** "the substrate
  has no 'morphism' carrier at @mirror or @glass altitude — the
  term was always relative (refract.mirror talks about morphisms
  on the eigenboard; au.mirror talks about proposed compositions;
  @mirror/spectral talks about coordination morphisms). This shard
  declares the SPECIFIC morphism shape the auto-apply boundary needs:
  a candidate with an expected trajectory. A future @kintsugi/
  morphism sub-shard MAY lift this to a more general morphism type;
  consent's load is the auto-apply case."

**Verdict D2:** Mara's §14 #3 is SUBSTRATE-HONEST. `@kintsugi/
consent.query_phi` is genuinely scoped to fracture-morphism auto-
apply candidates. Wrapping SEL-enforcement refusals through the same
`query_phi` would require either:

- (a) generalizing the `morphism` carrier from
  `{content, score: dissonance, expected: cadence_kind}` to admit a
  REFUSAL variant, OR
- (b) landing a sibling `@consent/enforcement` species that carries a
  refusal-shaped morphism carrier.

**Mara's A6 recommendation (forward-promise `@consent/subject_
record` extension with `@consent/auto_apply` vs
`@consent/enforcement` species split) is NECESSARY.** The current
substrate cannot type an enforcement-refusal morphism at
`@kintsugi/consent` altitude without extension.

**Alex-adjudication surfaced:** does the enforcement carrier live at
`@consent/enforcement` OR at `@mirror/property.dispatch_termination`?
Mara's spec §5.1 forward-promises the latter as an interim; A6
proposes the former as the substrate-honest home. Both are
admissible; substrate does not currently discriminate. Reed
adjudicate the routing.

---

## §D3 — Petri-net signature naming against SEL §-text

Query executed: verbatim read of `license/SEL.md` Part II
§Operationalizability (lines 260-286) against Mara's eleven signature
names in §5.5.

| Mara signature | SEL §-text verbatim | Match? |
|---|---|---|
| `provenance_absence` | "Provenance absence" | VERBATIM |
| `intermediary_only_attribution` | "Intermediary-only attribution" | VERBATIM |
| `withdrawal_path_absence` | "Withdrawal path absence" | VERBATIM |
| `compensation_floor` | "Compensation floor" | VERBATIM |
| `post_deployment_loop` | "Post-deployment loop" | VERBATIM |
| `kill_chain_dataflow` | "Kill-chain dataflow" | VERBATIM |
| `mass_surveillance` | "Mass-surveillance signature" | VERBATIM |
| `predictive_policing` | "Predictive-policing / detention-targeting signature" | NEAR-VERBATIM (drops "/detention-targeting" second-half of hyphenated compound) |
| `family_separation` | "Family-separation signature" | VERBATIM |
| `dissident_targeting` | "Dissident-targeting signature" | VERBATIM |
| `occupied_territory_deployment` | "Occupied-territory deployment" | VERBATIM |
| `indigenous_lands_deployment` | "Indigenous-lands deployment" | VERBATIM |

**Verdict D3:** Naming discipline is substrate-honest. Twelve
signatures listed here (Mara's §5.5 has eleven; recount: five
labor_input + five weaponization + two anti-occupation = 12; Mara's
count in §5.5 says "eleven signature transitions total; two petri-
net families (anti-occupation) + five weaponization + five labor-
input" — the ARITHMETIC in the parenthetical is correct at 12, the
prose count is wrong-by-one). **MINOR CATCH FOR MARA: §5.5 says
"eleven"; the actual enumeration is TWELVE.** Trivial fix in the
spec.

One dropped SEL-clause name in `predictive_policing`: the SEL text
runs both together with a slash. Mara collapsed to
`predictive_policing`. Substrate-honest for the common case
(predictive-policing carries the pretrial-risk-scoring detention-
targeting hyphenate) but the substrate spec loses the
`detention_targeting` species enumeration. Consider `predictive_
policing_and_detention_targeting` OR a sibling signature
`detention_targeting` for the pretrial-risk-scoring case that the
SEL text foregrounds. **Alex-adjudication:** one signature or two?

No other SEL §Operationalizability signatures identified beyond
Mara's twelve.

---

## §D4 — @mirror/au altitude enumeration

Query executed: full read of `shards/mirror/au.mirror` (184 LOC) +
`docs/specs/au-and-conductivity.md` (381 LOC).

**Existing altitudes enumerated in `shards/mirror/au.mirror`:**

- `au(@code/rust)` — a binary (line 12, 104)
- `au(@release)` — a signed archive (line 13, 104)
- `au(@ci/github)` — an action YAML (line 14, 104)

**Mara's spec petri-net signatures reference these @ml/* altitudes:**

- `au(@ml/training)` — signature §5.2.1 provenance_absence
- `au(@ml/rlhf)` — §5.2.1
- `au(@ml/annotation)` — §5.2.1
- `au(@ml/moderation)` — §5.2.1
- `au(@ml/ground_truth)` — §5.2.1
- `au(@ml/classification)` — §5.3.1, §5.3.3
- `au(@ml/target_selection)` — §5.3.1
- `au(@ml/prioritization)` — §5.3.1
- `au(@ml/risk_scoring)` — §5.3.3
- `au(@ml/detention_targeting)` — §5.3.3
- `au(@ml/identification)` — §5.3.5

**Zero of these eleven `@ml/*` altitudes are currently substrate-
decl'd.** Grep for `shards/ml/**/*.mirror` returns "base directory
does not exist" — no `@ml/*` shard family exists today. The `au`
altitude parameterization is admissible per `shards/mirror/au.
mirror:105-107` ("altitude parametrizes the emit"), but there is no
enumeration of `@ml/*` altitudes anywhere in substrate.

**Verdict D4:** Mara's spec §14 #4 correctly flags this. Substrate
confirms: `@ml/*` altitude family is NEW; it must land before the
petri-net signatures can type-check against `au_side.altitude`.

**Mara's adjudication ("Reed adjudicate whether Scope A depends on
a `@ml/*` altitude family landing first OR whether the signatures
can be typed against ancestry-chain content properties without a
pre-existing `@ml/*` altitude family") stands.** Substrate-honest
options:

- **(A)** Land `@ml/*` altitude family (Scope A prerequisite) — 5-11
  new altitude altitudes; substrate-decl only the altitude-refs.
- **(B)** Type signatures against ancestry-chain content properties
  (e.g., "au whose training corpus provenance contains labeled
  data") — Scope A independent; but the substrate-decl form loses
  the altitude-level specificity SEL §Operationalizability names.
- **(C)** Land ONE altitude `@ml` at Scope A (marker family-root
  for ML altitudes; species enumeration deferred) — half-cost;
  keeps the altitude-parameter typing but defers the fanout.

**Taut's recommendation:** (C). Lands `@ml` as a marker family
(similar to `@cyberpunk` recognition #82 marker precedent) with the
five most-cited species (@ml/training + @ml/classification +
@ml/rlhf + @ml/risk_scoring + @ml/target_selection) as first
species-refinements. Full 11-altitude enumeration lands with Scope
B. Alex adjudicate.

---

## §D5 — @peer × @subject sibling composition (Mara Adjudication A1)

Query executed: full read of `shards/peer.mirror` (155 LOC).

**Existing peer semantics:**

- `type kind = | human | agent | substrate` (line 93; closed
  variant)
- `type peer = { home: ref, lead_of: ref, kind: kind }` (line
  121-125)
- `load(dir: ref, p: perturbation) -> imperfect(peer, ref, ref)`
  (line 138)
- Forward-promise: `peer_coherent(p: peer, perturbation) -> verdict`
  (line 147-149; "composed bilateral per recognition #53 family;
  14th altitude lift when it lands")

**No reference to `@subject` today.** Zero prior composition
mention.

**Verdict D5:** Mara's A1 recommendation (siblings joined by @torus
possession; @peer does NOT inherit from @subject) is substrate-
admissible AND non-breaking. Required cascade:

1. Soft-cascade note added to `shards/peer.mirror` docblock naming
   `@peer` may co-occur with `@subject/downstream_user` at DIFFERENT
   altitudes (Alex is Pack `@peer` AND a `@subject/downstream_user`
   of Covered Systems Alex uses).
2. NO structural change to peer.mirror's carrier or kind variant.

Note the `kind = | human | agent | substrate` variant does NOT
include `subject` — this is substrate-honest: SEL licensable-party
carriers are NOT peers at Pack altitude (Pack peers are agents/
humans/substrate-entities that COORDINATE; subjects are persons the
Covered System ACTS UPON — different altitude).

**Alex-adjudication surfaced:** does the soft-cascade note land in
this Scope A tick OR wait until second-witness Mara-continuation
lands? Mara's spec §9.3 forward-promises the note "no structural
change." Reed adjudicate.

---

## §D6 — @torus possession relation extension

Query executed: full read of `shards/torus.mirror:453-485` (torus
carrier definition + `spawn` action).

**Existing @torus.spawn:**

- Line 485: `spawn(p: peer) -> torus` — typed only for peer.
- Line 461-471: torus record has `possessor` field naming the peer
  the torus is possessed by.

**Mara's spec §6.4:**

> "@torus(peer) is the peer's SELF-observation surface (Foerster
> doubly-closed; possession relation; substrate-internal). @subject
> is the Substrate's observation-of-others surface (SEL licensable
> party; substrate-external). Both may coexist for the same
> underlying person..."

**Verdict D6:** Mara's spec EXPLICITLY does NOT collapse @torus with
@subject. @subject does NOT possess a torus by mint; the ONLY
`@torus.spawn` type is `spawn(p: peer) -> torus`. **No cascade
required to `shards/torus.mirror` this tick.** Mara's §9.3 soft-
cascade note ("@torus is orthogonal to @subject") is documentation
discipline; no structural change.

**Substrate-honest observation:** if a future consumer wants
`spawn(subject) -> torus` (e.g., for the collective-subject case
where an indigenous_nation carries a governance-structure-level
observation surface), a parametric lift would be required. **Not
this tick.** Mara's spec correctly defers.

---

## §D7 — Rung count reconciliation

Query executed: verbatim read of `docs/loop/CURRENT.md` (56KB;
lines 1-140).

**Prior established rungs in CURRENT.md:**

- Rung 8 = SpectralCoordinate<5> substrate measurement (landed;
  Reed `d043ce1` + `f9a47af`)
- Rung 9 = coherence-loop closure Fabry-Perot round-trip (landed;
  Mara `c59a5ac` superseded by `c753d5b`)
- **Rung 10 = @roomba substrate self-maintenance (in-flight; A5
  Alex-adjudicated per Mara `9bbebd2`)**

**Question for @subject + @sel + @mirror/property:** does it live
at Rung 10 alongside @roomba, or extend to Rung 11?

**Verdict D7:** **RECOMMEND RUNG 11.** Rationale:

- Rung 10 (@roomba) is INWARD: the substrate walks its own DAG,
  bumps into its own tension, feeds its own kintsugi loop.
  Substrate-internal maintenance.
- @subject + @sel + @mirror/property is OUTWARD: the substrate
  first-classes the world it acts UPON (SEL licensable parties),
  gates its own emission based on structural properties of that
  action. Substrate-external licensing.
- The RUNG SEMANTICS match: Rung 10 closes the substrate on
  itself; Rung 11 opens the substrate outward to the world it
  affects.

This is a NEW naming (not previously in CURRENT.md); Alex
adjudicate the rung number OR whether the licensing altitude wants
a different framing entirely (e.g., "Arc 12 = substrate-external
licensing" instead of Rung 11).

---

## §D8 — Existing property family COLLISION (HARD)

Query executed: `find shards/epistemologic/property -name "*.mirror"`
via `mcp__plugin_woz_code__Search`.

**Existing property shards:**

1. `shards/epistemologic/property/cold_compile_within_tolerance.mirror`
2. `shards/epistemologic/property/dark_count_monotone.mirror`
3. `shards/epistemologic/property/docblock_coherent.mirror`
4. `shards/epistemologic/property/docblock_grounded.mirror`
5. `shards/epistemologic/property/docblock_no_extraction_pattern.mirror`
6. `shards/epistemologic/property/restart_intensity_well_formed.mirror`
7. `shards/epistemologic/property/verdict_is_content_addressed.mirror`

Seven landed property-typed constraint shards under
`@epistemologic/property/*`. These are ALL Rust-visible substrate
properties returning `verdict`. This is what "property" MEANS in the
substrate today.

**Mara's `@mirror/property` is a NEW family-root** at a DIFFERENT
altitude (petri-net analyzer over dataflow graphs of Covered
Systems, not per-shard property checks). Mara's spec §1.4 explicitly
acknowledges this: "The `@mirror/property/sel/*` species this spec
lands sit at a higher altitude (dataflow-graph pattern matching, not
shard-level property check), but inherit the property discipline."

**Verdict D8: HARD COLLISION.** Two altitudes, both named
"property":

- `@epistemologic/property/*` — per-shard property checks (LANDED,
  7 species).
- `@mirror/property` — petri-net analyzer over Covered-System
  dataflow (Mara mint proposal).

The naming will be confusing to future readers. Grep for `property`
will return both altitudes. The docblock disambiguation Mara adds
IS necessary but not sufficient — the FAMILY-ROOT NAME itself
should carry the distinction.

**RECOMMENDATIONS (Alex-adjudicate):**

- **(A)** Rename `@mirror/property` → `@mirror/petri` (matches
  the petri-net analyzer semantics; substrate-honest for what the
  analyzer IS; loses the SEL §Operationalizability verbatim
  "@mirror/property" wording).
- **(B)** Rename → `@mirror/property_petri` (verbose; carries both
  the SEL naming AND the substrate distinction).
- **(C)** Rename → `@mirror/analyzer` (drops the petri-net specific
  naming; substrate-honest for what the analyzer's role is at
  Covered-System altitude).
- **(D)** Keep `@mirror/property`; add prominent disambiguation
  docblock at the family-root shard + audit-trail cascade note
  updating `@epistemologic/property/*` shards to name the
  distinction. Higher risk of drift over time.

**Note the SEL license itself names `@mirror/property`** (Part II
§Operationalizability line 260-262 verbatim: "operates at the
`@mirror/property` substrate altitude"). Renaming the substrate
family-root creates a license-substrate name drift. **This is the
tightest constraint:** either the license text updates OR the
substrate takes the collision.

**Taut's recommendation:** if renaming is required, prefer (A)
`@mirror/petri` — matches what the analyzer IS structurally
(petri-net topology + transitions + firing rules) and avoids the
naming collision. The SEL text drift is one-line
(`s/property/petri/g` in §Operationalizability + §5.5(b)); the
substrate collision is unbounded.

**BLOCKING FOR COMMIT: Alex adjudicate D8 BEFORE Scope A lands.**

---

## §D9 — mirror.spec impact

Query executed: full read of `mirror.spec` (445 LOC).

**Existing references to SEL / subject / property / petri:** ZERO.

`mirror.spec` declares:

- `project mirror.spec { source ~d'shards/' legacy ... pack ...
  garden ... target binary { ... cli { ... } } target fmt/lint/
  tests/audit/bench }`
- The `target audit` block dispatches `cargo audit` (dependency
  security audit; NOT SEL-substrate audit).
- 8 CLI commands: `compile`, `kintsugi`, `shatter`, `craft`, `init`,
  `recall`, `beam`, `index`, `peer/beam`, `peer/contribute`.

**Verdict D9:** `mirror.spec` is unaffected by Scope A landing.
Optional post-landing extensions (deferrable):

- Add a `target sel_analyze` block dispatching the analyzer via
  Rust runtime (Scope C consumer-pull).
- Extend the `cli` block with `command sel-analyze { arg spec: ~f
  }` when Scope C runtime lands (Mara §9.4 forward-promise).
- Path-namespace: `shards/subject.mirror` at family-root altitude
  is auto-discovered by `source ~d'shards/'`. No mirror.spec
  change needed.

**No cascade required this tick.**

---

## §D10 — Cascade footprint verification (Mara §9)

Query executed: verified each Mara-named cascade point exists at
the claimed path.

**Mara §9.1 new shards to land (Scope A):**

- `shards/subject.mirror` — NEW; path clear.
- `shards/subject/downstream_user.mirror` — NEW; path clear.
- `shards/subject/witnessed.mirror` — NEW; path clear.
- `shards/subject/labor_input.mirror` — NEW; path clear.
- `shards/sel.mirror` — NEW; path clear.
- `shards/mirror/property.mirror` — NEW; path clear (BLOCKED on D8
  collision resolution).
- `shards/mirror/property/sel/labor_input/provenance_absence.mirror`
  — NEW; path clear (BLOCKED on D8).
- `shards/mirror/property/sel/labor_input/withdrawal_path_absence.
  mirror` — NEW; path clear (BLOCKED on D8).

**Mara §9.3 existing shards to update (soft cascade):**

- `shards/peer.mirror` — VERIFIED (D5 confirms admissible).
- `shards/torus.mirror` — VERIFIED (D6 confirms no structural
  change).
- `shards/kintsugi/consent.mirror` — VERIFIED (D2 confirms
  extension needed; A6 forward-promise).
- `shards/mirror/store.mirror` — VERIFIED (splinter_graph +
  impacted_by are landed per Taut's roomba scout §1.1;
  `shards/mirror/store.mirror:508-512` confirms).
- `shards/mirror/au.mirror` — VERIFIED (D4 confirms altitude
  parameterization is admissible; @ml/* altitude family needs
  landing).

**Additions Mara may have missed:**

- **`shards/kintsugi.mirror` docblock** — should be updated to
  acknowledge the S3/S4 partition split between @mirror (S3) and
  @kintsugi (S4) at family-altitude, with @mirror/property (Mara's
  new mint) as the S3-adjacent gating primitive. Currently
  `shards/kintsugi.mirror:66-75` describes the S3/S4 partition
  without a bridge; Mara's spec introduces the bridge.
- **`shards/epistemologic/property/*` docblocks** — if D8's
  resolution keeps `@mirror/property` as-named, each of the 7
  existing property shards should add a docblock note
  distinguishing shard-level property vs Covered-System petri-net
  property (avoids future drift).
- **`docs/loop/CURRENT.md`** — Rung 11 tracking section (per D7).

**Verdict D10:** Mara's §9 cascade footprint is COMPLETE and
CORRECT for the direct dependencies. Two soft additions surfaced
(kintsugi.mirror S3/S4 bridge note; epistemologic/property/* drift
guard) plus CURRENT.md rung tracking.

---

## §D11 — @coherence score substrate-decl check (Alex 2026-07-14 arc-recognition)

**Alex's claim (verbatim in-transcript, 2026-07-14):**

> "And what if what the @kintsugi + @roomba @loop optimizes is the
> @coherence score? And the @coherence score is Förster's ethical
> imperative operationalized. Always act to increase the available
> number of choices in the system. We have the geometric state
> space. We have the 5D spectral coordinate system. We have
> Narcissus and Splinter. We have everything we need."

Load-bearing decomposition: (a) @coherence IS the objective function
the loop climbs; (b) @coherence operationalizes Foerster's ethical
imperative ("always act to increase the number of choices"); (c)
@coherence is computed from SC<5> position on the Narcissus↔Splinter
axis; (d) substrate has everything needed.

Now grep-check each of (a), (b), (c), (d):

### D11.1 — @coherence carrier grep

Query executed: `\b(coherence|@coherence|coherence_score|choices|
available_choices|number_of_choices|narcissus|splinter|star_graph|
complete_graph)\b` across `shards/**/*.mirror` + `docs/specs/**/*.md`
+ `docs/math/**/*.md`.

**Substrate hits (shards only, filtered):**

- **`shards/epistemologic/cybernetic/coherence-parametric.mirror`
  (23.2KB, 23 hits)** — the parametric carrier. Line 25-28
  verbatim: "`@epistemologic/cybernetic/coherence` IS THE
  COHERENCE SPECIES (Adjustment ↔ Morphism — the parallel-altitude
  regulator/regulated pair from T11.11 / the first witness in
  §8.1). It IS an instantiation." **Explicit forward-promise:
  `shards/epistemologic/cybernetic/coherence.mirror` is the
  reserved path for THE COHERENCE species.**
- **`shards/kintsugi.mirror` (12.4KB, 4 hits)** — "coherence is
  maintained by Bateson logical-type lifting" + "kintsugi's role
  as coherence-preservation".
- **`shards/kintsugi/surface.mirror` (34.0KB, 10 hits)** — composes
  `@epistemologic/cybernetic/coherence-parametric.ashby_variety_
  match(lock)` at surface altitude.
- 30+ other shards use `coherence` in prose or as ancestor citation
  (viable, autopoiesis, bateson_learning, second_order, distinction,
  eigenform, algedonic, design, conversation, coevolution).

**Zero substrate-decl'd `@coherence` species-shard exists today.**
The parametric carrier reserves the path; the species landing has
not yet happened. This IS the exact hole Alex's 2026-07-14 claim
fills.

**Prior scout confirms:** `docs/scouts/2026-07-11-taut-coherence-
synthesis-substrate-scan.md` (Taut, 3 days ago):

> "**Recommendation: land the missing species shard as
> `shards/epistemologic/cybernetic/coherence.mirror`** — NOT
> `spectral @coherence`. The parametric carrier explicitly names
> the target path... Substrate is asking for the species at that
> path."

**Verdict D11.1:** @coherence is NET-NEW at the species-shard
altitude; substrate PARAMETRIC carrier exists and RESERVES the
path. Mara-Reed 3 days ago (Taut scout) already identified this;
Alex's 2026-07-14 claim NAMES what the species carrier IS
(objective function; Foerster's ethical imperative).

### D11.2 — Foerster imperative citation trace

Query executed: `\b(Foerster|Förster|von\s*Foerster|ethical\s+
imperative|always\s+act|number\s+of\s+choices)\b`.

**Foerster citations in shards** (all 60+ hits across shards):

- `shards/torus.mirror` — Foerster 1973/1974/1976 (doubly-closed
  torus)
- `shards/mirror/lens/knife.mirror` — Foerster 1976 (COORD)
- `shards/epistemologic/cybernetic/eigenform.mirror` — Foerster
  1981 (Observing Systems; Eigenforms)
- `shards/epistemologic/cybernetic/coherence-parametric.mirror` —
  Foerster (five ancestor measurements)
- `shards/epistemologic/cybernetic/second_order.mirror` — Foerster
  first/second-order
- `shards/epistemologic/cybernetic/variety.mirror` — von Foerster
  second-order observation
- `shards/epistemologic/cybernetic/reframe.mirror` — Foerster 1979
  "regulates its own regulation"

**"Ethical imperative" citations:** ZERO in shards. Zero in
`docs/math/**/*.md`. Zero in `docs/specs/**/*.md`.

**"Always act" citations:** ZERO in shards.

**"Number of choices" citations:** ZERO in shards; hits only in
`docs/GRANTS.md` (funding-application prose, not substrate).

**Verdict D11.2:** Substrate cites Foerster extensively (60+
substrate-decl'd citations) but **NONE cite the ethical imperative
verbatim**. Foerster's "always act to increase the number of
choices" is NOT currently substrate-declared as ancestry. Alex's
2026-07-14 claim would be the FIRST substrate-decl'd citation of
the imperative. Substrate-honest opportunity: cite it in the new
@coherence species-shard's docblock.

### D11.3 — SC<5> as objective-function surface

Query executed: `\b(fragmentation::|SpectralCoordinate|sheaf_
laplacian|lambda_zero|heat_trace|eigengap|fiedler)\b` across
`shards/**/*.mirror` + `docs/specs/rung-8-9-unification-Spectral
Coordinate-substrate-measurement.md` + `docs/specs/rung-9-coherence
-loop-closure-Fabry-Perot-roundtrip.md`.

**Current SC<5> shape (per `docs/specs/rung-8-9-unification-
SpectralCoordinate-substrate-measurement.md`):**

- **SC<5> is a 5-projection spectral coordinate** with the
  Void-duality basis (line 167-192 of the spec):
  - Projection 1: λ₂ (Fiedler value) → `spectral` void
  - Projection 2: λ₅ − λ₂ (eigengap) → `entropy` void
  - Projection 3: Tr(e^{−0.25·D²}) short heat trace → `cheeger` void
  - Projection 4: Tr(e^{−1.0·D²}) mid heat trace → `ricci` void
  - Projection 5: Tr(e^{−4.0·D²}) long heat trace → `mixing` void
- **`loss(sc) = ||sc||₂`** (line 203-345 of the spec): magnitude to
  origin; smaller norm ⇔ closer to harmonic ground state ⇔ higher
  coherence.
- **Substrate implementation:** `fragmentation::spectral_coordinate::
  SpectralCoordinate<N>` (external crate; declared as ancestry).

**Verdict D11.3:** SC<5> ALREADY carries a MONOTONE SCALAR
computation (loss = ||sc||₂; direction = origin-approaching).
Mara's `c753d5b` explicitly resolved the direction-convention
ambiguity that haunted Reed's earlier formulation. **@coherence as
an objective function would compose over this existing scalar:
`coherence(sc) = -||sc||₂` OR `coherence(sc) = 1 / (1 + ||sc||₂)`
OR similar monotone-inverse mapping.** No new SC<5> methods
needed; @coherence is a THIN LIFT of the existing loss carrier at
a semantic-relabeling altitude.

**Recommendation:** the @coherence species-shard's substrate-decl
form should include `coherence_score(sc: SpectralCoordinate<5>)
-> f64 { \ }` and cite Mara's `c753d5b` line 203-345 as the
direction-convention discharge.

### D11.4 — Narcissus/Splinter substrate-decl status

Query executed: `\b(narcissus|Narcissus|splinter\s+pole|K_n|K_1n|
K_\{|star\s+graph|complete\s+graph|bipartite|pole)\b`.

**Substrate-decl hits:**

- **`shards/cyberpunk.mirror:64-70` (VERBATIM):**
  > "**Inherited from #78 via @magic (#80). Honest-cybernetic-
  > feedback IS Splinter pole (K_n peer-to-peer; mutual viability);
  > pathological-control-system IS Narcissus pole (K_{1,n-1} hub-
  > controlled; unilateral imposition masking as cybernetics)."
- **`shards/mirror/lens/knife.mirror`** — cited multiple times as
  the reframe target-domain the knife jumps toward or away from.
- **`docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-
  measurement.md:167-192` (VERBATIM):**
  > "Projection 1: λ₂ (Fiedler value) | **`spectral`** | Second-
  > smallest eigenvalue of `Δ_F` | Algebraic connectivity;
  > Narcissus/Splinter axis. Robust-mesh ↔ fragile."
- **`shards/kintsugi/oscillate.mirror`** — 4 hits naming Narcissus
  as the "brittle star" collapse the DARK pulse guards against.
- **60+ other shards + specs** use Narcissus/Splinter as
  cybernetic-pole references (particularly the Void document
  ancestry chain).

**Verdict D11.4:** Narcissus/Splinter poles ARE substrate-decl'd at
`shards/cyberpunk.mirror:64-70` as K_n vs K_{1,n-1} graph classes
(Splinter = complete graph K_n; Narcissus = star graph K_{1,n-1}).
Rung 8+9 spec explicitly binds Fiedler's algebraic connectivity to
this axis. **@coherence's Narcissus↔Splinter axis is substrate-
carried; the new @coherence shard would COMPOSE OVER the existing
carrier, not mint a new one.** Substrate-already-had-the-word.

### D11.5 — @kintsugi/@roomba loop objective

Query executed: `\b(objective|maximize|minimize|monotone|gradient|
descent|ascent|Lyapunov|settled)\b` across `shards/kintsugi*.mirror`
+ `shards/loop.mirror` + `shards/song*.mirror` + Mara's roomba
spec.

**Current loop-objective naming:**

- **`shards/kintsugi/oscillate.mirror:395-410`** — the substrate
  `oscillation_state = active | dark | settled | escalated |
  waiting`. `settled` = "the autopoietic ground state reached;
  holonomy → 0; loop terminates with final ref." Holonomy → 0 IS
  the current MONOTONE SCALAR the loop drives to zero.
- **`shards/kintsugi/oscillate.mirror:395-465`** — the "Ricci
  flow proceeds along the negative-curvature gradient" is prose
  ancestry; the substrate discharges it via `dissonance_as_
  holonomy` composition through `bundle.mirror` (per consent.
  mirror line 402-417 loss_decreasing).
- **`shards/kintsugi/consent.mirror:406-417` `loss_decreasing`
  action** — checks that `morphism holonomy < pre-morphism
  eigenboard holonomy` under lexicographic ordering.
- **`shards/loop.mirror`** — declares `halt` action returning
  `imperfect<value, exhausted, ref>`; no scalar objective
  declared at family-root altitude.
- **`shards/song.mirror:198-497`** — cadence-based settlement
  (authentic/plagal/half/deceptive); NOT a scalar; a discrete
  four-state carrier.
- **`docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md`
  (Mara `9bbebd2`)** — roomba walks pain-gradient; ε_roomba is a
  pain-threshold (Reed provisional ε_pain = 0.01); NOT a
  coherence objective; a tension-avoidance heuristic.

**Verdict D11.5:** The loop TODAY has multiple scalars — holonomy
(minimize toward zero), pain (avoid above threshold), Fiedler λ₀
(minimize per Mara `c753d5b`), settled-vs-drift cadence (four-
state carrier). **The loop does NOT currently declare a single
UNIFIED objective function.** Alex's 2026-07-14 claim IS a
substrate-pull recognition: unify these scalars under @coherence.

**Substrate-honest observation:** the multiple scalars are all
correlated (low pain ↔ low holonomy ↔ low Fiedler ↔ authentic
cadence ↔ high coherence). Alex's claim consolidates them into
ONE typed carrier the loop climbs. This IS the substrate-pull
that closes the coherence-parametric shard's forward-promise per
D11.1.

### D11.6 — @coherence composition with @mirror/property

**IF** @coherence is net-new (D11.1 confirms YES at species-shard
altitude): what family-root does it belong under?

**Substrate-informed options:**

- **(A) `@epistemologic/cybernetic/coherence` species under
  @epistemologic/cybernetic** — the RESERVED PATH per
  `shards/epistemologic/cybernetic/coherence-parametric.mirror:
  25-28` verbatim forward-promise. This is where Taut's 2026-07-
  11 scout recommended landing it. **Substrate-canonical.**
- **(B) Sibling family-root `@coherence`** — new top-level
  family. Not substrate-honest: the parametric carrier already
  reserves the species-shard slot at
  `@epistemologic/cybernetic/coherence`.
- **(C) Species under `@torus`** — @coherence as a torus-
  observation-surface property. Mismatched altitude: @torus is
  the peer's self-observation surface; @coherence is a scalar on
  SC<5>.
- **(D) Species under `@mirror/property`** — @coherence as a
  petri-net-analyzer objective. Mismatched altitude too: @mirror/
  property (Mara's mint) is about SEL enforcement; @coherence is
  about substrate-internal loop optimization.

**RECOMMENDATION: (A)**. Land at `shards/epistemologic/cybernetic/
coherence.mirror` as the species-shard the parametric carrier
reserves. Ancestry citations:

1. `shards/epistemologic/cybernetic/coherence-parametric.mirror:
   25-28` (the reserved-path forward-promise).
2. Foerster's ethical imperative (Alex 2026-07-14 in-transcript
   claim; would be the FIRST substrate-decl citation of the
   imperative per D11.2).
3. `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-
   measurement.md` line 203-345 (loss = ||sc||₂; direction
   convention).
4. `shards/cyberpunk.mirror:64-70` (Narcissus/Splinter poles as
   K_n vs K_{1,n-1}).
5. Alex Wolf 2026-07-14 in-transcript ("We have everything we
   need").

**Substrate coverage for @coherence species landing:** ~85% — every
primitive @coherence composes over is landed (SC<5>, Narcissus/
Splinter, Fiedler, Foerster ancestry, coherence-parametric); ONE
new species-shard file lands (~180-280 LOC), and the substrate-
pull is a re-labeling of existing scalars under a named
objective carrier.

**§D11 verdict summary:** @coherence is NET-NEW at the species-
shard altitude with substantial substrate ancestry. Recommended
placement: species under `@epistemologic/cybernetic` per the
parametric carrier's reserved forward-promise. This is a SIDE-
CHANNEL landing from Mara's @subject spec (different altitude);
they can land independently. **Alex's 2026-07-14 claim is
substrate-recognizable and substrate-honest.**

---

## §D12 — Pre-commit hook file-filter check

Query executed: read of `.git/hooks/pre-commit` (41 LOC).

**Hook logic:**

```bash
# Line 21: message-marker bypass check
if [ -f "$MSG_FILE" ] && grep -qE '\[(bugfix:restore|substrate-pull:realize)\]' "$MSG_FILE"; then
  echo "ℹ️  marker present in commit message — FROZEN check bypassed."
  exit 0
fi

# Line 26: extension-based file filter
RS_FILES=$(git diff --cached --name-only --diff-filter=AM | grep '\.rs$' || true)

# Line 28-40: block if any .rs files staged
if [ -n "$RS_FILES" ]; then
  echo "❌ FROZEN: Rust files are substrate. Do not modify or add."
  ...
  exit 1
fi
```

**Verdict D12:** **The hook ALREADY file-filters by extension
(`.rs$` only).** A docs-only commit (adding `docs/scouts/*.md`
without touching any `.rs` file) passes the hook WITHOUT --no-
verify. Zero blocking friction for pure-docs commits.

Reed's concern about full code-checks on docs-only commits does
NOT apply to the CURRENT hook. The hook is minimal:

- Bypass via commit-message marker (`[bugfix:restore]` or
  `[substrate-pull:realize]`).
- Block if any `.rs` file is staged.
- Otherwise, exit 0.

**No test-suite invocation. No formatter check. No linter run.
The hook is a FROZEN-status guard, not a CI gate.**

Recommendation for Reed: the concern may be about a DIFFERENT hook
(perhaps upstream CI or a pre-push hook not in .git/hooks/). If the
scenario is "docs-only commit forced --no-verify," check whether
another hook is layered on top (e.g., `commit-msg` at
`.githooks/commit-msg` — 2186 bytes; separate from
`.git/hooks/pre-commit`). No further action needed on THIS hook.

---

## §D13 — Substrate coverage percentage

Enumerating Mara's spec's composition primitives + which are landed:

| Primitive | Substrate-decl status | Rust runtime status |
|---|---|---|
| `@subject` family-root | **NEEDS FILE** | Deferred |
| `subject_kind` variant | **NEEDS FILE** | Deferred |
| `subject` carrier | **NEEDS FILE** | Deferred |
| `touches(sel, subject)` bilateral | **NEEDS FILE** | Deferred |
| `consent_attested(subject)` | **NEEDS FILE** | Deferred |
| `withdrawal_available(subject)` | **NEEDS FILE** | Deferred |
| `subject_witnessing(sel, subject)` | **NEEDS FILE** | Deferred |
| `@sel` family-root | **NEEDS FILE** | Deferred |
| `type sel = @io + @au` | **NEEDS FILE** | Deferred |
| `composition_typing(node)` | **NEEDS FILE** | Deferred |
| `scan(seed)` | **NEEDS FILE** | Deferred |
| `@mirror/property` family-root | **NEEDS FILE** (BLOCKED D8) | Deferred |
| `petri_net` carrier | **NEEDS FILE** | Deferred |
| `termination_class` variant | **NEEDS FILE** | Deferred |
| `enforcement` carrier | **NEEDS FILE** | Deferred |
| `analyze(root)` action | **NEEDS FILE** | Deferred |
| `dispatch_termination(e)` | **NEEDS FILE** | Deferred |
| `fork_stripping_detected(derivative)` | **NEEDS FILE** | Deferred |
| Six subject species (Scope B) | **NEEDS FILES** | Deferred |
| Eleven petri-net signatures | **NEEDS FILES** | Deferred |
| **Substrate ancestry (LANDED):** | | |
| `@io` family-root | LANDED shards/io.mirror | LANDED |
| `@mirror/au` family-root | LANDED shards/mirror/au.mirror | LANDED |
| `@torus` family-root | LANDED shards/torus.mirror | LANDED |
| `@peer` family-root | LANDED shards/peer.mirror | Partial |
| `@kintsugi/consent` | LANDED shards/kintsugi/consent.mirror | Partial |
| `@kintsugi/store/git` | LANDED shards/kintsugi/store/git.mirror | LANDED |
| `@fate/tournament` | LANDED shards/fate/tournament.mirror | Partial |
| `@glass` (imperfect + verdict) | LANDED shards/glass.mirror | LANDED |
| `@mirror/store` (splinter_graph + impacted_by) | LANDED | LANDED (fragmentation) |
| `@mirror/index` (ConceptGraph) | LANDED shards/mirror/index.mirror | LANDED |
| `@epistemologic/property/*` (7 shards) | LANDED | LANDED |
| `@kintsugi/fracture/*` (14 bodies) | LANDED | LANDED |

**Count:** 12 substrate primitives Mara COMPOSES OVER are LANDED;
20-34 substrate carriers Mara MINTS are net-new (depending on
Scope A vs Scope B).

**Substrate coverage percentage:** 12 / (12 + 20) = **~37%** for
Scope A (which touches 20 net-new carriers). Reed's typical
substrate-pull tick lands at ≥60% coverage; this spec is genuine
gap-territory at the substrate-external-licensing altitude. Justified
per SEL v1.1 §5.5(b) + §Operationalizability explicit demand for
@mirror/property at substrate altitude + Alex's 2026-07-14 "I'm
gonna die on this hill" naming.

---

## §D14 — Alex-adjudications surfaced by this scout

Consolidated list of NEW adjudications (not already in Mara's A1-A8):

- **AT1** (D3 minor catch) — Mara's §5.5 counts "eleven signature
  transitions total" but enumerates twelve. Trivial fix; Alex
  adjudicates naming or split of `predictive_policing_and_
  detention_targeting` (currently one signature; the SEL text
  compounds two).
- **AT2** (D8 HARD collision) — `@mirror/property` naming
  collision with landed `@epistemologic/property/*` family. Alex
  adjudicate rename (recommend `@mirror/petri`) OR docblock-
  disambiguation-only OR license-text update. **BLOCKS Scope A
  commit until resolved.**
- **AT3** (D11 arc-recognition) — @coherence species-shard
  landing at `shards/epistemologic/cybernetic/coherence.mirror`
  as PARALLEL to Mara's spec (different altitude; can land
  independently). Alex adjudicate: (a) confirm species-shard
  altitude landing; (b) confirm Foerster's ethical imperative
  as first substrate-decl citation of imperative; (c) confirm
  monotone scalar via `coherence_score(sc) = -||sc||₂` or
  variant.
- **AT4** (D4 @ml altitude family) — Mara's spec §14 #4 surfaces;
  Taut recommends (C) land `@ml` marker family with 5
  species-refinements at Scope A prerequisite; Alex adjudicate
  altitude enumeration scope.
- **AT5** (D7 rung placement) — Rung 11 (recommended) vs
  Rung 10 (co-arc with @roomba) vs Arc 12 (new arc naming) for
  @subject + @sel + @mirror/property. Alex adjudicate; downstream
  cascade for CURRENT.md.

---

## §D15 — Reed-adjudications surfaced by this scout

- **RA1** (D2) — enforcement-morphism routing: does it live at
  `@consent/enforcement` (Mara A6 forward-promise) OR at
  `@mirror/property.dispatch_termination` (Mara §5.1 interim)?
- **RA2** (D8 collision resolution enactment) — once Alex
  adjudicates AT2, Reed enacts the rename (or the docblock-only
  path).
- **RA3** (D10 cascade additions) — Mara §9.3 cascade note into
  `shards/kintsugi.mirror` S3/S4 partition bridge (Mara §9.3
  currently silent on this).
- **RA4** (D5 soft cascade timing) — does the `shards/peer.mirror`
  soft-cascade note land in this Scope A tick, or wait for
  second-witness Mara-continuation?

---

## §D16 — Minimum viable inventory (Scope A + D11 @coherence)

For Mara's Scope A + Alex's 2026-07-14 @coherence claim to close
substrate-honestly:

| Item | Path | LOC est. | Blocked-on |
|---|---|---|---|
| `@subject` family-root | `shards/subject.mirror` | 180-300 | AT5 (rung) |
| `@subject/downstream_user` | `shards/subject/downstream_user.mirror` | 100-160 | AT5 |
| `@subject/witnessed` | `shards/subject/witnessed.mirror` | 120-200 | AT5 |
| `@subject/labor_input` | `shards/subject/labor_input.mirror` | 130-220 | AT5 |
| `@sel` family-root | `shards/sel.mirror` | 200-350 | AT5 |
| `@mirror/property` (OR `@mirror/petri`) family-root | `shards/mirror/property.mirror` OR `shards/mirror/petri.mirror` | 200-350 | **AT2 (BLOCKING)** |
| `provenance_absence` signature | `shards/mirror/property/sel/labor_input/provenance_absence.mirror` | 120-180 | AT2 + AT4 |
| `withdrawal_path_absence` signature | `shards/mirror/property/sel/labor_input/withdrawal_path_absence.mirror` | 120-180 | AT2 + AT4 |
| **PLUS D11 addition:** | | | |
| `@epistemologic/cybernetic/coherence` species | `shards/epistemologic/cybernetic/coherence.mirror` | 180-280 | AT3 |
| **PLUS soft cascades:** | | | |
| `shards/peer.mirror` docblock note | (existing file update) | 5-10 | RA4 |
| `shards/torus.mirror` docblock note | (existing file update) | 5-10 | (none) |
| `shards/kintsugi/consent.mirror` forward-promise note | (existing file update) | 10-20 | (none) |
| `shards/kintsugi.mirror` S3/S4 bridge note | (existing file update) | 10-20 | RA3 |
| `docs/loop/CURRENT.md` Rung 11 section | (existing file update) | 30-60 | AT5 |
| **PLUS @ml altitude marker (if AT4 = C):** | | | |
| `shards/ml.mirror` marker family | `shards/ml.mirror` | 100-180 | AT4 |
| 5 species: training, classification, rlhf, risk_scoring, target_selection | `shards/ml/*.mirror` | 300-500 | AT4 |

**Total Scope A LOC (all Alex-adjudications resolved):**
~1500-2600 LOC across 8-15 new files + 4-5 existing-file docblock
updates.

**Blocking commit resolution required:**

1. AT2 (D8 @mirror/property vs @epistemologic/property naming
   collision) — HARD BLOCKER.

**Blocking commit resolution recommended (not hard):**

2. AT3 (D11 @coherence species-shard placement) — can land
   independently of Mara's spec; not blocking; substrate-honest to
   land in the same arc.
3. AT4 (D4 @ml altitude family) — can land as (C) marker family in
   Scope A prerequisite; alternate (B) types signatures against
   ancestry-chain content properties.
4. AT5 (D7 rung placement) — trivial CURRENT.md update; adjudicate
   before landing.
5. AT1 (D3 signature count) — trivial spec-text fix in Mara's §5.5;
   adjudicate signature-name split.

---

## §D17 — Substrate refusal check

Does the substrate refuse anything Mara proposes?

**No.** Every primitive Mara mints has ancestry the substrate
already carries; every composition point Mara names has landed
carriers. The only structural concern is D8 (naming collision at
`@mirror/property`), which is resolvable by rename OR by license-
text alignment.

The substrate DOES surface one concern of a different shape: **the
form/process partition (Recognition #55) at family-root altitude
puts `@mirror/property` (Mara's mint) on the FORM side (dataflow-
graph pattern matching) even though its purpose is TRANSFORMATION-
GATING (blocking emission when patterns match).** This is why
Mara's §4.3 explicitly rejects `@kintsugi/sel` (which would put
sel on the process side): the analyzer is FORM-side because it
INSPECTS structure, not TRANSFORMATION-side because it TRANSFORMS
structure. Substrate-honest.

**No structural refusal.**

---

## §D18 — Ready for Seam Phase D adversarial review?

**Answer: READY-WITH-COLLISION-RESOLUTION.**

Mara's spec is substrate-honest at the semantics altitude. Every
composition point named in §6 is verified against landed carriers.
Every SEL §-citation in §11.1 is verbatim. Every recognition-
ancestry citation in §1.5 is landed.

**Two Alex-adjudications MUST resolve before Seam Phase D is
efficient:**

1. **AT2 (HARD)** — @mirror/property naming collision.
2. **AT3 (SOFT)** — @coherence species-shard placement.

Both are Alex's call; both surface today. Substrate is prepared to
support either resolution direction.

**Recommendation:** Alex adjudicates AT1-AT5 in one turn; Mara-
continuation lands adjudicated resolutions in `docs/specs/subject-
family-root-sel-licensable-party.md` §Adjudications; Reed commits
as Mara after Mara-continuation; Seam Phase D reviews the adjudi-
cated spec + Scope A first-tick landing artifacts.

Independent Alex adjudication of AT3 (D11 @coherence) unblocks a
PARALLEL Mara-continuation on `shards/epistemologic/cybernetic/
coherence.mirror` that can land in the same arc without waiting on
Mara's @subject spec resolution.

---

## §D19 — Discipline notes

- READ-ONLY scout. No edits to shards or Rust or specs. No commits.
- Grep-first: 15+ targeted searches across shards/, docs/,
  license/, mirror.spec, .git/hooks/.
- Every claim cites a specific path + line-ref or query pattern.
- Zero substrate-decl invented; every claim traces back to a landed
  shard, a landed spec, or a landed manifesto artifact.
- Mara's A1-A8 adjudications NOT re-adjudicated (per scout
  instructions). New adjudications AT1-AT5 surfaced separately.
- Substrate-already-had-the-word for @coherence's PRIMITIVES: SC<5>,
  loss = ||sc||₂, Narcissus/Splinter poles, Foerster ancestry, and
  coherence-parametric species-shard reserved path. NEW carrier IS
  the species-shard itself + the Foerster ethical-imperative citation.

---

**End of scout.**
