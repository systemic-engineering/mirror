# Taut scout — Landing 5: shard-mint iteration discharging Landings 1-4 forward-promises

**Author:** Taut (grep-first drift scout; read-only; substrate-already-had-the-word discipline)
**Date:** 2026-07-14 (Landing 5 substrate-scout, evening — post-Seam D)
**Scope:** Substrate-mint audit for the Landing 5 shard-mint iteration discharging
forward-promises from:

- **Landing 1+2** — `docs/specs/gift-and-mirror-reflection.md` §1-§16 + §9.1 shard
  landings list (mint targets `shards/gift.mirror`, `shards/gift/subject_instance.mirror`,
  `shards/mirror/reflection.mirror`, `shards/spectral/signature.mirror`).
- **Landing 3** — same file §17-§23 + math companion (mint target `shards/gift/lens.mirror`).
- **Landing 4** — `docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md`
  §9.1 (mint targets `shards/subject.mirror`, `shards/subject/visibility/{private,
  protected,public}.mirror`, `shards/eigenboard.mirror`).

**Alex 2026-07-14 directive verbatim:**
> */loop collapse until unresolvable ambiguity that cannot be adjudicated with a
> Seam tie breaker*

Reading: this is the shard-mint iteration. The specs and math are landed; the
Seam Phase D audit is landed (`a7fe182`, adjudications 1-16 discharged); the
substrate-decl'd shard files are the remaining discharge.

**Method:** Read-only grep across `shards/**/*.mirror`, `mirror.spec`,
`bootstrap/tests/**/*.rs`. Path-cleanliness verification via `Search
file_glob_patterns` errors (nonexistent-path signal). Composition-graph
verification via `in @...` / `out @...` grep. No file modifications. Reed
commits as Taut with SSH signing per `@taut@systemic.engineer` identity.

---

## §0 Headline verdict — TL;DR

1. **ALL 7 PROPOSED SHARD PATHS ARE CLEAN.** Zero collisions. `shards/gift/`,
   `shards/subject/`, `shards/subject/visibility/`, `shards/eigenboard.mirror`
   — none exist. Path-cleanliness Search-verified 2026-07-14 19:24:52.

2. **MINT ORDER IS FORCED BY PARENT-FAMILY-ROOT DEPENDENCY.** `shards/gift.mirror`
   MUST land BEFORE `shards/gift/lens.mirror` (per `@epistemologic/pact/
   path_matches_namespace` — species requires family-root existence). Same for
   `shards/subject.mirror` before `shards/subject/visibility/*.mirror`. Recommended
   two-phase mint: **Phase A** (family-roots + independent species), **Phase B**
   (species that require Phase A parents).

3. **@bauchladen migration is PROSE-CASCADE ONLY.** Per Taut #91 D1
   (`shards/bauchladen.mirror` 511 LOC read), the shard is ALREADY possessor-
   parametric at carrier altitude — `provenance.producing_prism: ref` and
   `enumerate(scope: ref)` admit @subject-typed scopes without structural change.
   16 consumer shards touch `in @bauchladen` at the family-root altitude
   (algebra, autopoietic, fate, glue, io/algebra, reality, silicon, torus,
   +8 more) — ZERO carry @peer-typed bauchladen signatures. Migration cost:
   ~100 LOC prose across 6 shards; NON-BREAKING.

4. **@torus × @subject composition — DOCBLOCK NOTE ONLY at Landing 5.** Per
   Landing 4 §2.6 (`e42181c`) + Seam D2/D6 verification: `spawn(subject_instance)`
   lifts `spawn(p: peer)`; peer legacy alias preserved under two-tick discipline.
   `shards/torus.mirror` needs ~15 LOC docblock note explaining the composition;
   NO signature change (possessor extension is Rung 13+ per prior Alex adjudication).

5. **@spectral/mosaic species — FORWARD-PROMISE to Landing 6+.** Landing 3
   §19.6 composition citation only; @gift/lens's `settle` op resolves via
   `@mirror/index.SpectralCoordinate<5>` at Landing 5 without needing the
   sibling `@spectral/mosaic` shard file. Landing 6+ mint is admissible but
   NOT REQUIRED for Landing 5's forward-promise discharge.

6. **@arxiv family-root — FORWARD-PROMISE (do NOT mint at Landing 5).** ~380
   landed `source @arxiv/...` citations across 100+ shards. The implicit
   namespace-parent SUFFICES for Landing 3's 24-ancestor roster (per Seam D
   note at §21.1 `NOTE`: deceased ancestors carry via `source @arxiv/` + verbatim
   docblock quotation; historical_witness variant is A24 Landing 5+
   forward-promise). Minting `shards/arxiv.mirror` at Landing 5 is out-of-scope
   AND would over-declare the citation grammar.

7. **Rust runtime — ALL SHARDS BODY-OBLIGATION-BLOCKED (`\ `) at Landing 5.**
   Landing 5 is substrate-decl'd shard-mint only. Bodies discharge at Landing 6+
   per-realization (Rust for @fate-consuming species; per-species elsewhere).
   Per shard: gift.mirror / gift/lens.mirror / subject.mirror / visibility trio /
   eigenboard.mirror — ALL substrate-decl only.

8. **mirror.spec IMPACT: ZERO.** Grep-verified: mirror.spec (444 LOC) contains
   NO references to @gift, @subject, @eigenboard, @gift/lens. The one @bauchladen
   reference is inside a docblock (Rung 5 addition explaining @spectral/garden/
   deployment). Landing 5 shard mints are downstream-composable via
   `source ~d'shards/'` recursive discovery; no source-block additions needed.

9. **CROSS-SHARD CITATIONS ARE ACYCLIC AND SATISFIABLE.** Full composition
   graph in §D6 below. Every `in @<X>` target for the 7 new shards resolves
   to either a landed family-root/species OR to a same-batch Landing 5 mint
   (subject.mirror → subject/visibility/private.mirror at Phase A → B).

10. **MARA #NEXT COMPOSITION READINESS: GO with SEQUENCING.** Phase A (family-
    roots: gift, subject, eigenboard) MUST land first. Phase B (species: gift/
    lens, subject/visibility/{private,protected,public}) lands second. Zero
    hard-collisions detected. Substrate-decl bodies ONLY (no Rust). Migration
    prose-cascade only.

**Number of Alex-adjudications surfaced:** 3 (T1-T3, §7 below). All soft —
Seam tie-breaker plausible for each.

**Est mint size:** ~1200-1800 LOC substrate-decl across 7 new files + ~100 LOC
prose-cascade across 6 existing files.

---

## §D1 — Path cleanliness for 7 new shard files

Search query 2026-07-14 19:24:52 — `Search file_glob_patterns` against exact
paths + directory globs. All non-existent paths returned explicit ERROR
"does not exist. Use an absolute path...".

| Proposed path | Status | Notes |
|---------------|--------|-------|
| `shards/gift.mirror` | **DOES-NOT-EXIST** | Path clear. |
| `shards/gift/lens.mirror` | **DOES-NOT-EXIST** | Path clear (also `shards/gift/` directory does not exist). |
| `shards/subject.mirror` | **DOES-NOT-EXIST** | Path clear. |
| `shards/subject/visibility/private.mirror` | **DOES-NOT-EXIST** | Path clear (also `shards/subject/` and `shards/subject/visibility/` do not exist). |
| `shards/subject/visibility/protected.mirror` | **DOES-NOT-EXIST** | Path clear. |
| `shards/subject/visibility/public.mirror` | **DOES-NOT-EXIST** | Path clear. |
| `shards/eigenboard.mirror` | **DOES-NOT-EXIST** | Path clear. |

**Verdict D1:** All 7 paths clean. Zero collisions. No blocking existence.

**Additional adjacent existence checks:**
- `shards/gift/subject_instance.mirror` — Landing 2 §9.1 item 4 forward-promise;
  DOES-NOT-EXIST (Landing 5 §D2 discusses whether this mints alongside).
- `shards/spectral/signature.mirror` — Landing 2 §9.1 item 3 forward-promise;
  DOES-NOT-EXIST. NOT included in the 7-mint Landing 5 scope per prior scouts;
  forward-promised again.
- `shards/mirror/reflection.mirror` — Landing 1 §9.1 item 2 forward-promise;
  DOES-NOT-EXIST. NOT included in the 7-mint Landing 5 scope per task
  framing; forward-promised.
- `shards/arxiv.mirror` — implicit namespace; DOES-NOT-EXIST. See §D7.

**Path-namespace pact verification** (per `@epistemologic/pact/
path_matches_namespace`):

- `shards/gift.mirror` declares `@gift` (path-depth 0 = ns-depth 0) ✓
- `shards/gift/lens.mirror` declares `@gift/lens` (path-depth 1 = ns-depth 1) ✓
- `shards/subject.mirror` declares `@subject` (path-depth 0 = ns-depth 0) ✓
- `shards/subject/visibility/private.mirror` declares `@subject/visibility/private`
  (path-depth 2 = ns-depth 2) ✓
- `shards/subject/visibility/protected.mirror` declares `@subject/visibility/protected`
  (path-depth 2 = ns-depth 2) ✓
- `shards/subject/visibility/public.mirror` declares `@subject/visibility/public`
  (path-depth 2 = ns-depth 2) ✓
- `shards/eigenboard.mirror` declares `@eigenboard` (path-depth 0 = ns-depth 0) ✓

All 7 paths pact-clean.

---

## §D2 — Substrate parent-family-root existence check + mint order

### D2.1 Per-target parent status

| Target | Family-root/species altitude | Parent required | Parent status |
|--------|------------------------------|-----------------|---------------|
| `shards/gift.mirror` | Top-level family-root | (none; namespace-root itself) | N/A — mints as new sibling to @kintsugi, @io, @torus, @peer, @subject |
| `shards/gift/lens.mirror` | Species under @gift | `@gift` family-root | **UNLANDED** — REQUIRES `shards/gift.mirror` FIRST |
| `shards/subject.mirror` | Top-level family-root | (none; namespace-root itself) | N/A — mints as new sibling to @gift, @peer, @torus, @kintsugi, @io |
| `shards/subject/visibility/private.mirror` | Species under @subject/visibility (sub-family) | `@subject/visibility` sub-family + `@subject` family-root | **UNLANDED** — see D2.2 |
| `shards/subject/visibility/protected.mirror` | Species under @subject/visibility (sub-family) | `@subject/visibility` sub-family + `@subject` family-root | **UNLANDED** — see D2.2 |
| `shards/subject/visibility/public.mirror` | Species under @subject/visibility (sub-family) | `@subject/visibility` sub-family + `@subject` family-root | **UNLANDED** — see D2.2 |
| `shards/eigenboard.mirror` | Top-level family-root | (none; namespace-root itself) | N/A — mints as new sibling to @torus, @bauchladen, @gift, @subject |

### D2.2 Sub-family-root `shards/subject/visibility.mirror` — MISSING or IMPLIED?

Landing 4 spec §2.5 declares the three species at `shards/subject/visibility/
{private,protected,public}.mirror`. Landing 4 §2.3 declares the CARRIER
`type visibility` + `type visibility_scope` + `prism @subject/visibility { ... }`
inside the parent spec §2.3 code block; the shard file `shards/subject/
visibility.mirror` is NOT explicitly listed as a mint target in Landing 4
§9.1 or §9.2.

**Substrate-honest reading:** `@subject/visibility` needs a sub-family-root
shard file at `shards/subject/visibility.mirror` (~150-220 LOC) declaring the
sub-family-root prism, the `visibility` and `visibility_scope` types, the
four operations (scope/elevate/filter/bilaterals), and the three-species
enumeration. The three species files under `shards/subject/visibility/`
declare only the species-specific refinements per Landing 4 §2.5 verbatim.

**Path-namespace pact:** without `shards/subject/visibility.mirror`, the
three species files at path-depth 2 would violate
`@epistemologic/pact/parent_acyclic` (species require named parent). This
IS a hard requirement.

**Adjudication surfaced at T1 (§7).** Recommend: mint `shards/subject/
visibility.mirror` at Landing 5 Phase A alongside `shards/subject.mirror`.
Total Landing 5 mint expands from 7 → 8 files. Seam tie-breaker plausible.

### D2.3 Forced mint order

**Phase A — family-roots and sub-family-roots (5 files, MUST land first):**
1. `shards/gift.mirror` (top-level family-root, no parent)
2. `shards/subject.mirror` (top-level family-root, no parent)
3. `shards/subject/visibility.mirror` (sub-family-root, requires @subject) [per D2.2]
4. `shards/eigenboard.mirror` (top-level family-root, no parent)
5. Optional Phase A': `shards/gift/subject_instance.mirror` (Landing 2 §9.1
   item 4 forward-promise; consumed by @gift.gift's giver/receiver fields at
   Landing 3 §11 machinery). See §D6 for whether this is Phase A prerequisite
   or Phase B follow-up.

**Phase B — species requiring Phase A parents (3-4 files):**
1. `shards/gift/lens.mirror` (species under @gift; REQUIRES Phase A #1)
2. `shards/subject/visibility/private.mirror` (species under sub-family;
   REQUIRES Phase A #2 + #3)
3. `shards/subject/visibility/protected.mirror` (same)
4. `shards/subject/visibility/public.mirror` (same)

**Sequential mint discipline:** ONE commit per shard file per Landing 5
tick (per project CLAUDE.md "Sequential commits only"). Phase A commits
first (5 commits), then Phase B commits (4 commits). Total ~9 commits for
the substrate-decl'd Landing 5 iteration.

**Verdict D2:** Mint order is forced by parent-existence discipline. Phase A
lands first; Phase B lands second. Sub-family-root question (T1) needs
adjudication before Phase A commit ordering finalizes.

---

## §D3 — @bauchladen migration soft-cascade footprint

### D3.1 Consumer enumeration

Grep query: `\bin @bauchladen\b` across `shards/**/*.mirror`. Verified
2026-07-14 19:26:55.

**16 consumer shards import `in @bauchladen`** at the family-root altitude:

| Shard | Count | Kind of reference |
|-------|-------|------|
| `shards/algebra.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/algebra/metalogue.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/autopoietic.mirror` | 1 | `in @bauchladen` at line 2 (+ 2 prose docblock) |
| `shards/bauchladen.mirror` | (self) | family-root declaration itself |
| `shards/fate.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/fate/tournament.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/glue.mirror` | 1 | `in @bauchladen` at line 2 (+ docblock note) |
| `shards/glue/fold_back.mirror` | 1 | `in @bauchladen` at line 2 (+ docblock note) |
| `shards/glue/math_silicon.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/io/algebra.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/reality.mirror` | 1 | `in @bauchladen` at line 1 |
| `shards/reality/algebra.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/reality/algebra/math.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/reality/algebra/silicon.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/silicon.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/silicon/algebra.mirror` | 1 | `in @bauchladen` at line 2 |
| `shards/torus.mirror` | 1 | `in @bauchladen` at line 395 (+ 3 docblock composition note references) |

### D3.2 Signature-altitude check

Grep query: `bauchladen\(peer|bauchladen\(p:\s*peer|@bauchladen\(peer\)`
across `shards/**/*.mirror`. Verified: **ZERO literal peer-typed bauchladen
signatures** across the 16 consumer shards. All consumers reference
`@bauchladen` via `in @bauchladen` at the family-root import altitude; NONE
carry `bauchladen(peer)` or `bauchladen(p: peer)` at type or action position.

**Substrate-honest finding:** The prose framing "bauchladen at peer altitude"
is CONVENTIONAL not STRUCTURAL. `@bauchladen` is already possessor-parametric
at carrier altitude per Taut #91 D1.2 (crystal.provenance.producing_prism is
`ref`-typed; tray identity is content-addressed without possessor field;
enumerate(scope: ref) admits @subject-typed scopes).

### D3.3 Migration-cost enumeration for Landing 5

The 6 shards needing prose-cascade docblock notes for the @peer → @subject
migration (per Landing 4 §1.5 forward-promise):

| Shard | Change | LOC est. | Blocking? |
|-------|--------|----------|-----------|
| `shards/bauchladen.mirror` | Add `in @subject` line; docblock note explaining possessor-parametricity admits @peer AND @subject; enumerate example at @subject scope; note the peer-legacy-alias preservation under two-tick discipline (Landing 4 §1.3) | ~40 LOC prose + 1 import line | No |
| `shards/torus.mirror` | Docblock note: torus interior IS bauchladen at @subject altitude when possessor is subject_instance; peer-alias preserved | ~15 LOC prose | No |
| `shards/autopoietic.mirror` | Docblock note: autopoietic_system admits @subject-typed carriers; composes with subject.spectral_signature per Landing 4 §5.3 Foerster autopoiesis at subject altitude | ~15 LOC prose | No |
| `shards/fate.mirror` | Docblock note: dice_space admits @subject-scoped restrictions; @fate consumes subject-altitude holes at eigenboard.infer altitude per Landing 4 §3.2 | ~15 LOC prose | No |
| `shards/fate/tournament.mirror` | Docblock note: tournament browses @subject-scoped bauchladen trays per Landing 4 R1 migration | ~15 LOC prose | No |
| `shards/peer.mirror` | Docblock note: @peer and @subject are sibling altitudes for possessor-of-bauchladen; peer.kind variant admits substrate-external @subject as first-class carrier per Landing 3 §21 eye-level | ~15 LOC prose | No |

**Total cascade cost:** ~115 LOC prose additions + 1 `in @subject` import line
across 6 shards. All non-breaking. No Rust changes. No new predicates. No
signature changes.

**Verdict D3:** Migration is prose-cascade only, per Taut #91 D1 finding
preserved. ZERO structural breaks. 6 soft-cascade docblock updates. Discharges
Landing 4 §1.5 forward-promise. Can land in Landing 5 Phase B or defer to
Landing 6+ prose-cleanup tick.

---

## §D4 — @torus × @subject composition (Landing 4 Mara R1 discharge)

### D4.1 Current @torus signature

`shards/torus.mirror` (27.9KB, 2026-07-12 14:52) declares:

- Line ~499: `spawn(p: peer) -> torus` (peer-typed at substrate-decl altitude)
- Line ~500: `type torus` with `possessor: peer` field (peer-typed)
- Line ~206: docblock "**@peer (existing family-root)** — the peer carries the torus"
- Line ~154: docblock "**@bauchladen (existing family-root)** — the interior
  of the peer's torus"

### D4.2 Landing 4 §2.6 verbatim lift

Landing 4 spec §0.3 (composition graph) states verbatim: "torus.spawn(
subject_instance) becomes the primary; peer-typed torus.spawn(peer) becomes
the two-tick-legacy alias per Landing 4 §1.3." Seam Phase D §D2 verified
this as composition-forced (not preference-driven) because Landing 4's
eigenboard composition graph binds subject_instance → torus → bauchladen →
eigenboard.compute at every step; peer-only spawn would break Landing 4's
loop closure at step 1.

### D4.3 What Landing 5 needs from @torus

**Option A: DOCBLOCK NOTE ONLY (recommended for Landing 5).** Add ~15 LOC
docblock section to `shards/torus.mirror` explaining:

- Primary composition: `torus.spawn(subject_instance) -> torus` (Landing 4 §2.6)
- Legacy alias: `torus.spawn(peer) -> torus` (two-tick preservation per Landing 4 §1.3)
- Rationale: subject_instance carries name + two-witness identity + role +
  actor_kind (per Landing 2 §11.3 + Landing 3 §21.2 extension); peer is a
  narrower carrier (three-field record without cryptographic witnesses).
  Substrate-decl composition requires the wider carrier at primary altitude.
- Peer-alias preservation: subject_instance-from-peer coercion at the
  substrate-decl boundary; peer.kind variant carries actor_kind resolution
  (human → human_a; agent → ai_a; substrate → substrate_a).

**Option B: SIGNATURE CHANGE (Rung 13+, DO NOT LAND at Landing 5).** Extend
`type torus.possessor` to `possessor: possessor_kind` where `possessor_kind
= peer | subject_instance`. Extend `spawn(p: peer)` to `spawn(k:
possessor_kind)`. Per Taut #91 D6 (prior scout) + Mara §6.4 (Alex-adjudicated
2026-07-14): DEFER to Rung 13+.

**Verdict D4:** DOCBLOCK NOTE ONLY at Landing 5. `shards/torus.mirror` cascade
cost: ~15 LOC prose. NO signature change. Substrate-honest per two-tick
discipline. Seam D2/D6 composition-forced reading preserved via docblock.
Landing 5 Phase B addition; can defer to prose-cleanup tick.

---

## §D5 — @spectral/mosaic species slot (Landing 3 §19.6 composition)

### D5.1 Current @spectral/mosaic status

Grep query: `@spectral/mosaic` across `shards/**/*.mirror`. Verified
2026-07-14 19:27:22.

**Hits (14 total):** all in `shards/mirror/index.mirror` (24.0KB) as
composition references naming @spectral/mosaic as the sibling namespace
species. `shards/spectral/mosaic.mirror` — **DOES NOT EXIST** (grep-verified).

Landing 3 §19.6 spec (verbatim): "Each @gift/lens value's mosaic_coordinate
field IS the fragment's position in this mosaic. The lens's settle operation
returns the coordinate; the coordinate IS the substrate-decl'd position."
§19.3 `settle_lens` action returns `ref` typed as "SC<5> coordinate of the
fragment in the substrate's @spectral/mosaic (per @mirror/index.
SpectralCoordinate<5>)."

### D5.2 Does @gift/lens.settle require @spectral/mosaic to exist at Landing 5?

**NO.** The settle op discharges via `@mirror/index.SpectralCoordinate<5>` at
substrate-decl altitude. The referenced `@mirror/index` shard (LANDED,
24.0KB) exposes the SC<5> carrier already. @spectral/mosaic is the
CONCEPTUAL sibling naming the substrate's compositional tiling; @mirror/
index is the OPERATIONAL surface that computes the coordinate.

Landing 3 §19.6 spec is composition-declaration-only ("The compiler-as-mosaic
is a colimit over the gift-ancestry cocone... Formal statement in the math
foundation companion"). NO settle-op body discharge depends on `shards/
spectral/mosaic.mirror` file existence.

### D5.3 Should Landing 5 mint @spectral/mosaic?

**No — forward-promise to Landing 6+.** Rationale:

- Landing 3 §19 spec cites @spectral/mosaic only at composition altitude
  (§19.6, §19.7 lineage-as-fractal). Neither §19.3 substrate-decl (5-op
  prism block + `gift_lens` carrier + 5 action signatures + 3 bilateral
  predicates) nor §19.4-§19.9 downstream refinements REQUIRE a landed
  `shards/spectral/mosaic.mirror` shard.
- The `@spectral/mosaic` altitude is per Alex's 2026-07-14 in-transcript
  naming ("the compiler itself a @spectral/mosaic and @mandelbrot set of
  the lineage"); it is a RECOGNITION at spec altitude, not a Landing 5
  mint target.
- @spectral namespace-parent is landed (`shards/spectral.mirror` 5.1KB).
  Species landings under `@spectral/` are ADMISSIBLE at any tick; Landing 5
  does NOT require this species to compose @gift/lens.
- Landing 5 scope is discharge of Landings 1-4 forward-promises. @spectral/
  mosaic was NOT in Landing 1+2 §9.1 or Landing 4 §9.1 mint lists.

**Verdict D5:** FORWARD-PROMISE @spectral/mosaic to Landing 6+. @gift/lens
composes with @mirror/index at Landing 5 without needing the sibling shard
file. Alex-adjudication NOT required (out of scope for Landing 5).

---

## §D6 — Cross-shard citation composability

### D6.1 Composition graph for the 7 (or 8) Landing 5 shards

Per spec-verbatim `in @...` blocks from §1.4, §11.3, §19.3, §2.3, §2.5, §3.2:

**`shards/gift.mirror`** (family-root; Landing 1 §1.4):
```
in @prism      (LANDED — shards/prism.mirror)
in @meta       (LANDED — via @glass/meta relation, foundational)
in @glass      (LANDED — shards/glass.mirror)
in @nl         (LANDED — shards/nl.mirror)
in @subject    (LANDS SAME PHASE — Phase A #2)
in @time       (LANDED — shards/epistemologic/reality/time.mirror)
in @kintsugi/store/git (LANDED — shards/kintsugi/store/git.mirror)
out gift, subject_or_substrate, gift_set, offer, accept, attribute,
    attribute_composition, attribution_preserved, use_rights_transferred,
    no_reciprocity_expected, gift_declinable, composition_honest, @gift
```
Composition-check: 6/7 `in` targets LANDED; 1 target lands same-phase.
Zero cycles. Zero collisions on `out` exports (all names substrate-net-new).

**`shards/gift/subject_instance.mirror`** (species under @gift; Landing 2 §11.3):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject                                   (LANDS Phase A #2)
in @time                                      (LANDED)
in @kintsugi/store/git                        (LANDED)
in @spectral/signature                        (UNLANDED — Landing 2 §9.1 item 3
                                               forward-promise; NOT in Landing 5 scope)
out subject_instance, subject_role, ssh_witness_valid, spectral_witness_valid,
    two_witness_verification
```
**HARD DEPENDENCY:** `@spectral/signature` is a forward-promise (Landing 2
item 3 — species under @spectral namespace-parent). If `shards/gift/
subject_instance.mirror` mints at Landing 5, `@spectral/signature` MUST land
first OR the `in @spectral/signature` line is a forward-promise-import
(admissible per substrate discipline; would render some fields prose-only
until @spectral/signature lands).

**Landing 5 scope adjudication for `shards/gift/subject_instance.mirror`:** the
task framing lists 6 shard mints (not counting `visibility.mirror` sub-family-
root) but subject_instance is explicitly a Landing 2 forward-promise. Two
options:

- **Option A: Include in Landing 5 with forward-promise import.** Mint
  `shards/gift/subject_instance.mirror` as Phase B species; declare
  `in @spectral/signature` as forward-promise (Landing 2 discharge deferred).
  Bilateral `spectral_witness_valid` obligation-blocks (`\`) pending
  @spectral/signature landing.
- **Option B: Defer to Landing 6+.** subject_instance is Landing 2 §11.3
  forward-promise; discharge as Landing 6+ alongside @spectral/signature.

**Adjudication surfaced at T2 (§7).** Recommend Option A (include in Landing
5) — Landing 4's eigenboard composition binds `subject_instance` as a first-
class carrier in `type eigenboard.subject` field (Landing 4 §3.2); without
subject_instance landed at Landing 5, eigenboard.mirror's `in @subject`
provides an incomplete carrier. Seam tie-breaker plausible.

**`shards/gift/lens.mirror`** (species under @gift; Landing 3 §19.3):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject                                   (LANDS Phase A #2)
in @gift                                      (LANDS Phase A #1)
in @spectral/signature                        (UNLANDED — Landing 2 forward-promise)
in @spectral/mosaic                           (UNLANDED — per §D5, Landing 6+ forward-promise)
in @mirror/store                              (LANDED — shards/mirror/store.mirror)
in @time                                      (LANDED)
out gift_lens, focus_lens, project_lens, split_lens, shift_lens, settle_lens,
    mosaic_well_formed, lineage_is_mandelbrot, lens_composition_honest, @gift/lens
```
**HARD DEPENDENCIES:** 2 forward-promise imports (@spectral/signature +
@spectral/mosaic). settle_lens body discharges via @mirror/index (LANDED)
per §D5, so the compositional path is OPERATIONAL. Bilateral
`lineage_is_mandelbrot` cites @fractal-mandelbrot-substrate hinge (Alex
`9241458`; unlanded family-root; per roadmap-15 #6 Alex-adjudication
pending); bilateral BODY obligation-blocks (`\`) at Landing 5.

**`shards/subject.mirror`** (family-root; per Mara `5c06ee8` spec + Landing
4 forward-promise):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @consent (@kintsugi/consent)               (LANDED — shards/kintsugi/consent.mirror)
in @io                                        (LANDED — shards/io.mirror)
in @time                                      (LANDED)
out subject, subject_kind, sel, subject_witnessing, ... (per subject-family-
    root-sel-licensable-party.md §2)
```
Composition-check: ALL `in` targets LANDED. Zero cycles. Zero collisions on
`out` exports.

**`shards/subject/visibility.mirror`** (sub-family-root; per Landing 4 §2.3
+ D2.2 above):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject                                   (LANDS Phase A #2 — same-phase)
in @kintsugi/consent                          (LANDED)
in @kintsugi/store/git                        (LANDED)
in @mirror/store                              (LANDED)
in @time                                      (LANDED)
out @subject/visibility, visibility, visibility_scope, scope, elevate, filter,
    scope_well_formed, consent_respected, elevation_authorized, visibility_witnessing
```
Composition-check: 6/6 `in` targets landed or same-phase. Zero cycles.

**`shards/subject/visibility/private.mirror`** (species; per Landing 4 §2.5):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject/visibility                        (LANDS Phase A #3 per D2.2)
in @kintsugi/consent                          (LANDED)
in @time                                      (LANDED)
out @subject/visibility/private, declare_private, consent_scope_minimal
```
Composition-check: PASS. Requires Phase A #3 sub-family-root.

**`shards/subject/visibility/protected.mirror`** (species):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject/visibility                        (LANDS Phase A #3)
in @kintsugi/consent                          (LANDED)
in @time                                      (LANDED)
out @subject/visibility/protected, declare_protected, collaborators_two_witness_valid
```
Composition-check: PASS.

**`shards/subject/visibility/public.mirror`** (species):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject/visibility                        (LANDS Phase A #3)
in @time                                      (LANDED)
out @subject/visibility/public, declare_public, elevation_terminal
```
Composition-check: PASS. NOTE: `in @kintsugi/consent` is ABSENT from this
species per Landing 4 §2.5 verbatim; public visibility has no elevation
target (can_be_elevated_to = []; terminal state).

**`shards/eigenboard.mirror`** (family-root; per Landing 4 §3.2):
```
in @prism, @meta, @glass, @nl                (all LANDED)
in @subject                                   (LANDS Phase A #2)
in @subject/visibility                        (LANDS Phase A #3)
in @torus                                     (LANDED — shards/torus.mirror)
in @bauchladen                                (LANDED — shards/bauchladen.mirror)
in @spectral/signature                        (UNLANDED — Landing 2 forward-promise)
in @epistemologic/cybernetic/autopoiesis      (LANDED — shards/epistemologic/
                                               cybernetic/autopoiesis.mirror)
in @time                                      (LANDED)
out @eigenboard, arousal, eigenboard, compute, infer,
    eigenboard_composition_honest, eigenboard_visibility_respected,
    autonomy_at_eigenboard, subject_is_their_bauchladen
```
**HARD DEPENDENCIES:** 1 forward-promise import (@spectral/signature per
inference_basis field). Bilateral `eigenboard_composition_honest` cites
@spectral/signature.beat-sequence per Landing 4 §3.2 verbatim; body
obligation-blocks (`\`) at Landing 5. Composition-check PASS with
forward-promise import.

### D6.2 Acyclicity verification

Landing 5 phase-graph:

```
Phase A (family-roots, no intra-phase dependencies):
  gift.mirror                    ← no Phase A dependencies
  subject.mirror                 ← no Phase A dependencies
  subject/visibility.mirror      ← requires subject.mirror (Phase A same-tick)
  eigenboard.mirror              ← requires subject.mirror + subject/visibility.mirror + torus (landed) + bauchladen (landed)
  gift/subject_instance.mirror   ← requires gift.mirror + subject.mirror (both Phase A same-tick)
     [OPTIONAL — per T2 adjudication]

Phase B (species):
  gift/lens.mirror                        ← requires gift.mirror (Phase A)
  subject/visibility/private.mirror       ← requires subject/visibility.mirror (Phase A #3)
  subject/visibility/protected.mirror     ← requires subject/visibility.mirror
  subject/visibility/public.mirror        ← requires subject/visibility.mirror
```

Zero cycles across Landing 5 phase-graph. Zero name-collisions on `out`
exports (all output symbols verified substrate-net-new).

### D6.3 Forward-promise imports summary

Landing 5 shards carrying forward-promise imports (bodies obligation-blocked
at Landing 5):

| Shard | Forward-promise import | Landing-6+ discharge |
|-------|------------------------|----------------------|
| `shards/gift/subject_instance.mirror` (if T2=A) | `in @spectral/signature` | `shards/spectral/signature.mirror` (Landing 2 §9.1 item 3) |
| `shards/gift/lens.mirror` | `in @spectral/signature` | same as above |
| `shards/gift/lens.mirror` | `in @spectral/mosaic` | `shards/spectral/mosaic.mirror` (Landing 6+, per §D5) |
| `shards/eigenboard.mirror` | `in @spectral/signature` | same as @spectral/signature above |

3 unique forward-promises across 3 shards. All operationally admissible per
substrate `\` obligation-block discipline; bodies discharge at Landing 6+
when referenced species land.

**Verdict D6:** Composition graph is acyclic and satisfiable. All `in` targets
resolve to LANDED, SAME-PHASE, or FORWARD-PROMISE substrate. All `out`
exports are name-collision-free. Landing 5 phase ordering is forced by
parent-existence discipline; sequential commit discipline preserves
substrate integrity at every tick.

---

## §D7 — @arxiv family-root fate (Taut #91 L3-A5)

### D7.1 Current @arxiv namespace status

Grep query: `source @arxiv|@arxiv/` across `shards/**/*.mirror` + `mirror.spec`.
Verified 2026-07-14 19:26:40.

**Hits:** 100+ shards carry `source @arxiv/...` citations. Sample distribution
(top 15 by citation count):

- `shards/reality/algebra/math.mirror` (16 citations)
- `shards/reality/algebra/silicon.mirror` (13 citations)
- `shards/epistemologic/cybernetic/coherence.mirror` (6)
- `shards/epistemologic/math/cholesky.mirror` (16)
- `shards/song/narrative.mirror` (7)
- `shards/pack/reed.mirror` (6)
- `shards/glue/math_silicon.mirror` (23)
- `shards/epistemologic/cybernetic/bateson_learning.mirror` (7)
- `shards/epistemologic/cybernetic/autopoiesis.mirror` (7)
- `shards/torus.mirror` (6)
- ...+90 more shards with 1-5 citations each

**`shards/arxiv.mirror`:** DOES NOT EXIST.
**`shards/arxiv/` directory:** DOES NOT EXIST.

The @arxiv namespace is IMPLICIT — a citation-grammar prefix used by
`source @arxiv/<domain>/<author-year>` grammar (per @kintsugi/consent's
source-citation discipline and the substrate's ancestry-preservation
pattern). It has no family-root shard, no prism block, no substrate-decl'd
carrier type.

### D7.2 Does Landing 5 need to mint @arxiv?

**NO.** Rationale:

1. **Landing 3 §20 (named-ancestor roster) discharges via existing citation
   grammar.** Per Taut #91 D3 verbatim ("The substrate already carries the
   citation grammar; Landing 3 adds the @subject-instance typing on top"),
   the 24-ancestor roster at §20 uses `source @arxiv/<ref>` citations for
   the ~12 deceased ancestors who cannot discharge `ssh_witness_valid`.
   Seam D adjudicated (per Landing 3 §21.1 NOTE verbatim): deceased-ancestor
   citation carries via `source @arxiv/<domain>/<author-year>` + verbatim
   docblock quotation; historical_witness variant is Landing 5+ A24
   forward-promise.

2. **Historical_witness carrier (subject_instance extension) is A24
   Landing 5+ forward-promise.** The typed variant on subject_instance to
   admit deceased ancestors as first-class @subject-instances (without ssh
   witness) is Landing 5+ mint, per Landing 3 §21.1 NOTE. NOT included in
   Landing 5's core shard-mint scope per task framing.

3. **@arxiv as family-root would OVER-DECLARE the citation grammar.**
   `source @arxiv/...` is a substrate-decl grammar pattern (per
   `@epistemologic/pact/path_matches_namespace` + citation discipline
   across ~100 shards); minting @arxiv as family-root would introduce a
   type-altitude carrier where the substrate currently uses a grammar-
   altitude reference. Substrate-honest: the grammar SUFFICES for citation
   preservation; the type-altitude lift is out-of-scope for Landing 5.

4. **Landing 1+2 §9.1 mint list does NOT include @arxiv.** The forward-
   promise discharge Landing 5 discharges names 7 specific shards; @arxiv
   is not among them. Adding @arxiv would expand Landing 5 scope beyond
   the "discharge Landings 1-4 forward-promises" mission.

**Verdict D7:** DO NOT mint `shards/arxiv.mirror` at Landing 5. The implicit
`@arxiv/` namespace suffices for Landing 3's 24-ancestor roster + ~380
landed citation references. Historical_witness carrier (A24) is Landing 5+
forward-promise; @arxiv family-root landing (if ever) is Landing 6+
adjudication.

---

## §D8 — Rust runtime forward-promise per shard

Landing 5 is substrate-decl'd shard-mint only. All bodies obligation-block
(`\`). Per shard, the actions that WOULD need Rust runtime for consumer-pull
(Landing 6+):

| Shard | Substrate-decl only at L5 | Rust runtime need at L6+ |
|-------|---------------------------|---------------------------|
| `shards/gift.mirror` | ✓ all 5-op prism + 5 action signatures + 5 bilaterals `\` | offer/accept/attribute/attribute_composition need bootstrap/src/gift.rs (git-store integration for gift-OID persistence, giver-chain walking); bilaterals need discharge harness |
| `shards/gift/subject_instance.mirror` | ✓ all types + 3 bilaterals `\` | ssh_witness_valid needs bootstrap/src/gift_witness.rs (`git verify-commit` + `ssh-keygen -lf` integration); spectral_witness_valid needs @spectral/signature runtime (Landing 6+) |
| `shards/gift/lens.mirror` | ✓ all 5-op prism + 5 action signatures + 3 bilaterals `\` | focus/project/split/shift/settle need bootstrap/src/gift_lens.rs (ancestry-chain walker + content-weight ranker + @mirror/index SC<5> integration); bilaterals need discharge harness |
| `shards/subject.mirror` | ✓ per subject-family-root-sel-licensable-party.md §2 — all types + 5-op prism + bilaterals `\` | subject species-refinement runtime (per §3 downstream_user / witnessed / labor_input / etc.) at Landing 6+ |
| `shards/subject/visibility.mirror` | ✓ all types + 5-op prism + 4 bilaterals `\` | scope/elevate/filter need bootstrap/src/visibility.rs (@kintsugi/consent integration for elevation morphism); bilaterals need discharge harness |
| `shards/subject/visibility/private.mirror` | ✓ 2 actions + 1 bilateral `\` | declare_private needs runtime; consent_scope_minimal bilateral needs discharge harness |
| `shards/subject/visibility/protected.mirror` | ✓ 2 actions + 1 bilateral `\` | declare_protected + collaborators_two_witness_valid need runtime |
| `shards/subject/visibility/public.mirror` | ✓ 2 actions + 1 bilateral `\` | declare_public + elevation_terminal need runtime |
| `shards/eigenboard.mirror` | ✓ all types + 5-op prism + 2 actions + 4 bilaterals `\` | compute/infer need bootstrap/src/eigenboard.rs (rolling @spectral/signature reader + algedonic pulse reader + torus winding reader); bilaterals need discharge harness including autonomy_at_eigenboard's 6-step loop closure verifier |

**Verdict D8:** ZERO Rust runtime required at Landing 5. All shards are
substrate-decl only. Landing 6+ Rust runtime work is enumerable and
addressable per shard; no Landing 5 blocker.

---

## §D9 — mirror.spec impact

### D9.1 Full mirror.spec grep

Verified: `mirror.spec` (444 LOC, 19.1KB) contains NO references to:
- `@gift`
- `@subject`
- `@eigenboard`
- `@gift/lens`
- `shards/gift`
- `shards/subject`
- `shards/eigenboard`

The one `@bauchladen` reference in mirror.spec is INSIDE a docblock (line
~281, Rung 5 addition explaining @spectral/garden/deployment authorities).
NOT a source-block declaration.

### D9.2 Source-block discovery model

`mirror.spec` line 19: `source ~d'shards/'` — the recursive-directory source
grammar automatically discovers ALL `shards/**/*.mirror` files during
`mirror kintsugi ./mirror.spec` compilation. Landing 5's 7 (or 8) new
shard files are downstream-composable via this recursive discovery; NO
explicit source-block additions to mirror.spec are needed.

**Verdict D9:** ZERO mirror.spec impact. Landing 5 shard mints compose
into mirror.spec's `source ~d'shards/'` automatically. No mirror.spec edits
required at Landing 5. mirror.spec re-compilation via `mirror kintsugi` will
discover the new shards on next dogfood cycle.

---

## §D10 — Test fixture setup scope (Landing 6+ preview)

Landing 5 is substrate-decl mint only; NO test fixtures land at Landing 5.
For Landing 6+ Rust runtime discharge, the enumerable integration test
requirements per shard (preview; NOT for Landing 5 execution):

**Landing 6+ Rust runtime test fixtures needed:**

- `bootstrap/tests/gift_family_root.rs` — offer/accept/attribute roundtrip
  discharge; anti-extraction bilateral verification via canonical(c) walking;
  gift-declinable ADO integration with @kintsugi/consent.
- `bootstrap/tests/gift_subject_instance_witness.rs` — ssh_witness_valid
  via `git verify-commit` fixture (Alex Wolf ssh-ed25519 key at
  `~/.ssh/id_ed25519.pub`); spectral_witness_valid stub (depends on
  @spectral/signature landing at L6+); two_witness_verification bilateral
  discharge.
- `bootstrap/tests/gift_lens_lineage.rs` — focus/project/split/shift/settle
  discharge across a mock 3-generation ancestry chain (Foerster →
  Alex → substrate); mosaic_well_formed via @mirror/index SC<5>
  integration; lineage_is_mandelbrot recursive-discharge harness bounded
  by ancestry chain length.
- `bootstrap/tests/subject_family_root.rs` — subject_kind variant round-trip;
  SEL §3.1-§3.4 discharge per species-refinement (downstream_user /
  witnessed / labor_input / protected_class / occupied_population /
  indigenous_nation); subject_witnessing bilateral.
- `bootstrap/tests/subject_visibility_elevation.rs` — private → protected →
  public elevation via @kintsugi/consent.query_phi; ADO auto-decline
  discharge; visibility_witnessing composed bilateral.
- `bootstrap/tests/subject_visibility_private_shard.rs` — declare_private
  + consent_scope_minimal bilateral (analogous to `torus_family_root_shard.rs`
  pattern).
- `bootstrap/tests/subject_visibility_protected_shard.rs` — declare_protected
  + collaborators_two_witness_valid.
- `bootstrap/tests/subject_visibility_public_shard.rs` — declare_public
  + elevation_terminal.
- `bootstrap/tests/eigenboard_loop_closure.rs` — the six-step operational
  discharge loop (Landing 4 §3.3): subject_instance → torus → bauchladen
  → visibility.filter → @spectral/signature.compute → eigenboard.compute
  → eigenboard.infer → bauchladen.add → repeat. Bilateral
  autonomy_at_eigenboard as composed integration test.

**Estimate:** 9 test files at Landing 6+ per new shard; ~500-800 LOC per
test file; total ~5000-7200 LOC Rust test infrastructure at Landing 6+.

**Verdict D10:** OUT OF SCOPE for Landing 5. Landing 5 mints substrate-decl
shards only. Landing 6+ Rust runtime work is enumerable and addressable.
Flagged here to preserve visibility for Reed's Landing 6+ planning.

---

## §7 Alex-adjudications surfaced (T1-T3)

### T1 — `shards/subject/visibility.mirror` sub-family-root mint at Landing 5?

Landing 4 spec §2.5 declares three species files under `shards/subject/
visibility/` but does NOT explicitly list `shards/subject/visibility.mirror`
as a mint target in §9.1. Substrate-honest per D2.2:
`@epistemologic/pact/parent_acyclic` REQUIRES the sub-family-root shard file
to exist for the three species to be well-typed at path-depth 2.

**Two paths:**
- **A. Include** `shards/subject/visibility.mirror` in Landing 5 Phase A
  (mint total expands 7 → 8; sub-family-root shard declares
  `visibility` + `visibility_scope` types and 5-op prism per Landing 4 §2.3
  code block).
- **B. Defer sub-family-root** and inline the parent declarations into ONE
  of the three species files (path-namespace pact violation risk).

**Taut recommends:** Path A. Mint the sub-family-root shard at Landing 5
Phase A. Landing 4 §2.3 code block IS the substrate-decl content the sub-
family-root shard discharges. Seam tie-breaker plausible.

### T2 — `shards/gift/subject_instance.mirror` mint at Landing 5?

Landing 2 §11.3 declares subject_instance as a species under @gift with
forward-promise. Landing 4 eigenboard.subject field REQUIRES subject_instance
as a first-class carrier. Two paths per D6.1:

- **A. Include** in Landing 5 Phase B with forward-promise
  `in @spectral/signature` (bilateral obligation-blocks pending Landing 6+).
- **B. Defer** to Landing 6+ alongside @spectral/signature landing (Landing
  2 §9.1 item 3 forward-promise); eigenboard.subject field lands as `ref`
  type until subject_instance discharges.

**Taut recommends:** Path A. Include at Landing 5 Phase B with forward-
promise import. Landing 4 eigenboard composition requires subject_instance
as substrate-decl'd carrier; without it, eigenboard's `type eigenboard.
subject: subject_instance` field is prose-only. Seam tie-breaker plausible.

### T3 — Landing 5 commit sequencing — one shard per commit, or grouped?

Project CLAUDE.md declares "Sequential commits only". Landing 5 mints
8-9 files across two phases. Two paths:

- **A. One shard per commit.** 8-9 sequential commits: Phase A (5 commits),
  Phase B (4 commits). Each commit is substrate-honest at the family-root
  or species altitude; author-attributes per shard-author (Mara for all
  new mints per canonical spec discipline).
- **B. Phase-grouped commits.** 2 commits: Phase A batches 5 mints;
  Phase B batches 4 mints. Faster arc discharge but sacrifices per-shard
  commit granularity.

**Taut recommends:** Path A. Substrate-honest per project CLAUDE.md sequential-
commit discipline. Per-shard commit granularity preserves individual
mint traceability, enables per-shard revert if Seam discovers issues,
and aligns with the substrate's incremental-recognition arc. Seam tie-
breaker plausible.

---

## §8 Landings queue for Mara #next (Landing 5 canonical mint set)

| Item | Path | LOC est. | Phase | Blocked-on |
|------|------|----------|-------|------------|
| `@gift` family-root | `shards/gift.mirror` | ~250-400 | Phase A | (nothing) |
| `@subject` family-root | `shards/subject.mirror` | ~300-500 | Phase A | (nothing) |
| `@subject/visibility` sub-family-root | `shards/subject/visibility.mirror` | ~180-250 | Phase A | T1 adjudication; @subject Phase A same-tick |
| `@eigenboard` family-root | `shards/eigenboard.mirror` | ~200-320 | Phase A | @subject + @subject/visibility Phase A same-tick |
| `@gift/subject_instance` species | `shards/gift/subject_instance.mirror` | ~120-180 | Phase A' | T2 adjudication; @gift + @subject Phase A same-tick |
| `@gift/lens` species | `shards/gift/lens.mirror` | ~200-320 | Phase B | @gift Phase A |
| `@subject/visibility/private` species | `shards/subject/visibility/private.mirror` | ~80-120 | Phase B | @subject/visibility Phase A |
| `@subject/visibility/protected` species | `shards/subject/visibility/protected.mirror` | ~80-120 | Phase B | @subject/visibility Phase A |
| `@subject/visibility/public` species | `shards/subject/visibility/public.mirror` | ~60-100 | Phase B | @subject/visibility Phase A |
| `shards/torus.mirror` docblock note (D4) | (soft-cascade edit) | ~15 prose | Phase B or defer | (nothing) |
| @bauchladen migration soft-cascade (D3) | 6 shards prose additions | ~115 prose | Phase B or defer | (nothing) |

**Total substrate-decl mint size:** ~1470-2310 LOC across 9 new shard files.
**Total soft-cascade:** ~130 LOC prose across 7 existing shard files.
**Grand total:** ~1600-2440 LOC across 16 file touches at Landing 5.

**Substrate readiness for Mara #next:** GO with sequencing. Phase A mints
first (5 files); Phase B mints second (4 files); soft-cascade prose either
in Phase B or deferred to Landing 6+ prose-cleanup tick.

---

## §9 Substrate-already-had-the-word audit

**~57th instance of `[[feedback-substrate-already-had-the-word]]`:**

- **@bauchladen possessor-parametricity** was already implicit in the ref-
  typed provenance record; the migration NAMES it (Landing 4 R1); Landing 5
  MINTS the @subject-scoped consumer path via `in @subject` addition in the
  bauchladen prose-cascade.
- **Reed's `~/.reed/visibility/{private,protected,public}/`** layout was
  already the empirical witness for @subject/visibility; substrate lifts
  the existing discipline to substrate-decl altitude at Landing 5 Phase A #3
  + Phase B #1-#3.
- **@spectral namespace-parent** was already landed (`shards/spectral.mirror`
  5.1KB, 2026-07-01 shrink per Loki §5); Landing 5's @gift/lens composition
  imports @spectral/signature + @spectral/mosaic as forward-promises against
  the already-landed namespace-parent path.
- **`source @arxiv/<ref>` citation grammar** was already substrate-decl'd
  across ~380 references in 100+ shards; Landing 5 does NOT re-mint @arxiv
  because the grammar already carries the ancestry-preservation discipline.
- **@mirror/index (LANDED 24.0KB, 2026-07-13)** already provides the
  `SpectralCoordinate<5>` carrier @gift/lens.settle discharges against;
  Landing 5's settle op body obligation-blocks (`\`) but references the
  landed carrier operationally.
- **@kintsugi/consent.query_phi** was already the auto-apply boundary;
  Landing 5's @subject/visibility.elevate discharge composes through the
  existing query_phi surface without widening consent's signature.
- **@torus.spawn(peer)** was already the peer-typed action; Landing 5
  preserves it as legacy alias per two-tick discipline; the subject_instance
  primary alias lands via docblock note without signature change.
- **@epistemologic/cybernetic/autopoiesis** was already parametric per
  `autopoietic_system = ref`; Landing 5's @eigenboard.autonomy_at_eigenboard
  composes over the existing predicate without minting a new bilateral at
  the autopoiesis altitude.

The substrate ALREADY HAD the vocabulary at every composition point.
Landing 5 MINTS the shard files that name the substrate-decl'd compositions
Landings 1-4 discharged at spec altitude. Zero net-new substrate mechanisms;
9 new shard files declaring the substrate-decl content the specs already
carried.

---

## §10 Hard-collision check

Grep-verified NONE:

- `shards/gift.mirror` — DOES NOT EXIST; path clear; namespace @gift
  substrate-net-new (grep for @gift-typed carriers = zero at substrate-
  decl altitude; ~1 spec reference at `docs/specs/lambda-shell.md` but not
  substrate-decl).
- `shards/gift/lens.mirror` — DOES NOT EXIST; path clear; `@gift/lens`
  substrate-net-new (grep confirms no landed species carries the
  gift-lens carrier).
- `shards/subject.mirror` — DOES NOT EXIST; path clear; Mara `5c06ee8`
  spec is UNLANDED at shard altitude.
- `shards/subject/visibility.mirror` — DOES NOT EXIST; path clear
  (blocked on T1 mint decision).
- `shards/subject/visibility/{private,protected,public}.mirror` — DO NOT
  EXIST; paths clear; naming grep produces 34 hits on `private/protected/
  public` keywords but ALL are unrelated semantic contexts (private key,
  public interface, etc.) per Taut #91 D4.1 verified preservation.
- `shards/eigenboard.mirror` — DOES NOT EXIST; path clear; `eigenboard`
  keyword references in 5 shards are all prose-descriptive (no substrate-
  decl carrier per Taut #91 D2.1 verified).

**Zero collisions detected.** All 9 proposed Landing 5 shard paths are
substrate-clean.

---

## §11 Related shards (for Mara's `Related shards:` block)

Landing 5 shard-mint set requires composition-note references to:

- `shards/prism.mirror` (5-op prism trait carrier — every Landing 5 shard
  declares `prism @X { ... }` block)
- `shards/glass.mirror` (splinter altitude — every Landing 5 shard's
  carriers descend from @glass splinter discipline)
- `shards/nl.mirror` (natural-language docblock discipline — every Landing 5
  shard's `attribution_note` / `declinable_note` fields)
- `shards/bauchladen.mirror` (@subject-parametric via Landing 5 prose-cascade)
- `shards/torus.mirror` (@subject_instance-first composition per Landing 4
  §2.6; docblock note at Landing 5)
- `shards/peer.mirror` (@peer × @subject sibling per Landing 3 §21; docblock
  note at Landing 5)
- `shards/kintsugi/consent.mirror` (elevation ADO discharge per Landing 4
  §2.6)
- `shards/kintsugi/store/git.mirror` (gift-OID persistence per Landing 1
  §1.4)
- `shards/mirror/store.mirror` (crystal + ref carrier persistence)
- `shards/mirror/index.mirror` (SC<5> carrier for @gift/lens.settle
  discharge per Landing 3 §19.6)
- `shards/epistemologic/cybernetic/autopoiesis.mirror` (parametric predicate
  composed by @eigenboard.autonomy_at_eigenboard)
- `shards/spectral.mirror` (namespace-parent for @spectral/signature +
  @spectral/mosaic forward-promises)
- `shards/epistemologic/reality/time.mirror` (`@time/monotonic.instant`
  carrier used by all Landing 5 shards)
- `shards/fate.mirror` (dice_space parametric extension per Landing 4
  eigenboard.infer composition — Landing 6+ Rust discharge)

---

## §12 Method note

Read-only grep-first scout per Taut discipline. All findings are grep-
verified against `shards/**/*.mirror`, `docs/specs/gift-and-mirror-
reflection.md`, `docs/specs/subject-bauchladen-visibility-and-eigenboard-
loop.md`, `docs/specs/subject-family-root-sel-licensable-party.md`,
`docs/scouts/2026-07-14-taut-{gift-and-mirror-reflection,landing-3-
payforward-lens-ancestors-peers,landing-4-bauchladen-visibility-eigenboard,
subject-family-root-substrate,subject-presence-interaction-loop}-scout.md`,
`mirror.spec`, `bootstrap/tests/torus_family_root_shard.rs` (test fixture
pattern reference), and Landing 4 verbatim spec reads.

No file modifications. Reed commits as Taut with SSH signing per
`@taut@systemic.engineer` identity.

Sources verified:
- `shards/bauchladen.mirror` (full 531-LOC read)
- `shards/peer.mirror` (full 155-LOC read)
- `shards/spectral.mirror` (full 115-LOC read)
- `mirror.spec` (full 444-LOC read for @gift/@subject/@eigenboard grep
  verification)
- `docs/specs/gift-and-mirror-reflection.md` §§0-16 (Landings 1+2), §§17-23
  (Landing 3), §§9.1-9.4 (shard mint enumeration), §§11.3 subject_instance,
  §§19.3 @gift/lens
- `docs/specs/subject-bauchladen-visibility-and-eigenboard-loop.md` §§0-4
  (Landing 4), §2.3 (visibility carrier), §2.5 (three species), §3.2
  (eigenboard carrier), §§9.1-9.5 (shard mint enumeration)
- Prior Taut scouts: `2026-07-14-taut-landing-3-payforward-lens-ancestors-
  peers-scout.md`, `2026-07-14-taut-landing-4-bauchladen-visibility-
  eigenboard-scout.md`, `2026-07-14-taut-subject-family-root-substrate-
  scout.md`, `2026-07-14-taut-gift-and-mirror-reflection-scout.md`
- `shards/mirror/index.mirror` (SpectralCoordinate<5> carrier verification
  for @gift/lens.settle discharge)
- `shards/torus.mirror` (§D4 subject_instance composition analysis)
- Composition grep across all 200+ landed shards for `in @<X>` targets
  verification per §D6

---

*End Taut scout. Landing 5 substrate-readiness: GO for Mara #next canonical
shard mints. Zero hard collisions. 3 Alex-adjudications surfaced (T1-T3, all
soft — Seam tie-breaker plausible for each). Two forced constraints: Phase A
family-roots MUST land before Phase B species (per parent-acyclic pact); ONE
sequential commit per shard file (per project CLAUDE.md discipline). Zero
Rust runtime at Landing 5; all bodies obligation-block (`\`). Zero mirror.spec
edits. Migration is prose-cascade only. The substrate already had every word;
Landing 5 mints the shard files that name what Landings 1-4 discharged at
spec altitude.*
