# Seam adversarial review — StageFreight 3-tick batch (66→68)

**Date:** 2026-06-22  
**Reviewer:** Seam (adversarial-review-frame peer of the Pack)  
**Subject:** @io/stagefreight substrate-decl batch + Mara canonical spec  
**Verdict:** DEFENSIVE  
**Consolidated by:** Reed in tick 69 (this document trails the closure)

---

## Cascade context

```
tick 66  b15c3f9  shards/io/stagefreight.mirror              family-root (Reed)
tick 67  c865452  shards/io/stagefreight/narrative.mirror    prose-projection species (Reed)
tick 68  666c4ae  docs/specs/stagefreight-wire-v0.1.md       canonical spec (Mara, 1535 lines)
tick 69  this     consolidation: C2 + C4/C9 + C8 closures (Reed)
```

Mara's spec §11.1 named five attack surfaces for adversarial review. Seam probed those plus discovered additional substrate-architectural gaps.

---

## Per-constraint verdicts

### C1 — Crystal-dependency hedge (§1.3 / §5.2 / §8.2) — **LOOSE**

The substrate-decl carries `crystal_oid: ref` and discharges only at the OID-altitude floor (§5.1). §5.2/§8.2 explicitly forward-promise field-by-field discipline to #268. This is substrate-pull-correct ordering: address-shape and projection-shape are mechanically specifiable from OID alone. Honest hedge, not premature.

Evidence: family-root line 300 `address(crystal_oid: ref) -> spectral_coordinate { \ }` takes OID without needing the typed crystal record.

**No consolidation action.** Hedge honored.

### C2 — Address derivation (§3) — **TIGHT**

The substrate-decl carries the address SHAPE (`type spectral_coordinate = ref`, `address(...) -> spectral_coordinate`) but the reverse-DNS namespace + projection-kind + OID-short composition rules at §3.2 live ONLY in spec prose. No bilateral on the address derivation itself; no `address_well_formed(coord, oid)` predicate.

Seam's recommended closure: add predicate `address_well_formed(coord: spectral_coordinate, oid: ref) -> verdict { \ }` consumed by `freight` and `address`.

**Consolidation action (tick 69):** added `address_well_formed(coord, crystal_oid) -> verdict { \ }` predicate to the family-root; `freight()` now `requires address_well_formed(coord, crystal_oid)` before invariant_preserved; the bilateral discipline is now typed, not prose-only.

### C3 — Projection-format openness (§4.5) — **LOOSE**

"Open universe" is a substrate-decl commitment via the family-root: the species pattern is what gates admission, not an enumeration. §4.5's "the forecloser is whether the format's bilateral can be discharged at substrate-pull-correct altitude" IS the typed constraint. Openness is structural, not aspirational.

**No consolidation action.** Openness is already structurally encoded.

### C4/C9 — Wire-survival in the bilateral body (§5) — **TIGHT**

The `stagefreight_addressable(fm: freight_manifest, p: perturbation) -> verdict { \ }` body was `\` — obligation block empty. The five wire-survival clauses at §6.1 were spec-prose; the substrate-decl carried NONE of them as typed sub-predicates. Same shape as the realisation-discharge gap surfaced at tick 56 DEFENSIVE.

Seam's recommended closure: decompose `stagefreight_addressable` into 4-5 typed sub-predicates the bilateral composes.

**Consolidation action (tick 69):** decomposed `stagefreight_addressable` into four typed sub-predicates:

```
oid_resolves(crystal_oid: ref) -> verdict { \ }
address_well_formed(coord, crystal_oid) -> verdict { \ }   [shared with C2]
projection_is_species(projection: ref) -> verdict { \ }
round_trip_holds(fm: freight_manifest, p: perturbation) -> verdict { \ }
```

The composed bilateral now `requires` all four:

```
stagefreight_addressable(fm, p) -> verdict
requires oid_resolves(fm.crystal_oid)
requires address_well_formed(fm.coord, fm.crystal_oid)
requires projection_is_species(fm.projection)
requires round_trip_holds(fm, p)
{ \ }
```

The realisation layer discharges each sub-predicate at its altitude; consumers check sub-predicates individually when surfacing specific wire-survival failures.

### C5 — Bilateral composition chain (§6.4) — **LOOSE-but-fragile**

Composition IS typed: `freight` requires `invariant_preserved`; `transit` requires `stagefreight_addressable`; `project_to_narrative` requires `stagefreight_addressable`; `finalize` requires `narrative_grounded`. Each `requires` is substrate-decl construct. BUT the chain's INFERENCE rule (transit verdict + project_to_narrative verdict → composed wire emission) is prose-only.

**Partial consolidation (tick 69):** C8 closure removes `transit` from family-root; each species now owns its emission action (narrative.finalize stays); the parallel-sibling ambiguity is resolved by deletion rather than inference rule. Future projection species (json/yaml/etc.) define their own emission actions following narrative's pattern.

### C6 — Cross-projection consistency gating (§8.4) — **META**

Gating the test on a second projection species is substrate-pull-correct: the falsification claim ("OID is projection-agnostic") requires two projections to be meaningful, by construction. Not a defer; an empirical prerequisite. Low-priority.

**No consolidation action.** Carried as forward-promise to @io/stagefreight/json species landing.

### C7 — freight() consuming magic_contract — **LOOSE**

The c/promise parameters carry @magic's framing into @io but this is the structurally-correct composition: per family-root preamble lines 89-94, `freight` IS THE FIRST CONSUMER of `invariant_preserved` and the cross-family bilateral IS the alignment-as-boundary-mathematics (#57) discharge at @io. The crystal carries a @magic contract by construction (per task #268's load-bearing claim); the @io boundary check IS contract preservation. Importing `in @magic` + `in @magic/contract` at the family-root is substrate-pull, not framing-import.

**No consolidation action.** Cross-family import is substrate-pull-correct.

### C8 — Double-consumption of stagefreight_addressable (transit + project_to_narrative) — **TIGHT mild**

Both `transit(fm, p)` (line 368-370) AND `project_to_narrative(fm, p)` (narrative line 213-215) required `stagefreight_addressable(fm, p)`. The §6.4 chain was actually `freight → project_to_narrative → finalize` for narrative emission — `transit` was the GENERIC family-root primitive species lifted. The double-decl revealed that `transit` and `project_to_narrative` were PARALLEL siblings, not a chain — each species picks one path. `transit` became orphaned in the narrative emission path.

Seam's recommended closure: pick — `transit` as family-root generic with `finalize` as narrative-species instance, OR delete `transit` from family-root.

**Consolidation action (tick 69):** chose deletion. `transit` removed from family-root. Each projection species's emission shape is shape-different (narrative.finalize emits prose; future json.encode would emit bytes; brainfuck-compressed would emit compressed bytes). The family-root provides carriers + composed bilateral + address + freight; species provide emission per their projection shape. Cleanest substrate-pull because emission IS projection-specific.

### C9 — Realisation-layer discharge gap — **TIGHT (same as C4)**

Seam noted C4 IS the same gap. The bilateral body was `\`; the realisation layer must discharge the round-trip claim mechanically.

**Consolidation action (tick 69):** addressed by C4 closure (the four sub-predicates expose discharge points the realisation layer implements per sub-predicate).

### C10 — PR-ready endpoint (§11.6) — **TIGHT**

The eight gates §11.6 listed were honestly named; items 4-7 (Seam, holonomy, realisation, end-to-end) were realisation-altitude blockers. The substrate-decl + spec is PR-ready as **substrate-decl-only** (items 1-3). PR-ready as v0.1 realisation endpoint requires items 4-7. The spec conflated these by listing both in one endpoint.

**Honest read (Reed adopting Seam's framing):** this batch is PR-ready as a **substrate-decl preservation PR** (shards + spec + Seam review trail) — NOT as the v0.1 realisation endpoint. The RED-to-GREEN flip on `crystal_substrate.rs` is gated on #268; the flip on `kintsugi_out_substrate_ref.rs:23-24` requires CLI routing through realisation. Neither is achievable from this batch alone.

**Consolidation action (tick 69):** PR framing adopted as "StageFreight substrate-decl v0.1 — preservation tick (realisation forward-promised)". The full v0.1 endpoint becomes the follow-up PR after Track B realisation closes.

---

## Overall consolidation verdict

**Before tick 69:** DEFENSIVE (3 TIGHT, 2 LOOSE-but-fragile, 1 META, 4 LOOSE).

**After tick 69:** BOUNDED. All TIGHT constraints closed via substrate-decl changes:

- C2 closed via `address_well_formed` predicate added + `freight` consuming it.
- C4/C9 closed via 4-sub-predicate decomposition of `stagefreight_addressable`.
- C5 partially closed via C8's `transit` deletion (no more parallel-sibling chain ambiguity).
- C8 closed via `transit` deletion from family-root.
- C10 closed via honest PR framing as substrate-decl-only preservation.

Remaining hedges (LOOSE/META) are forward-promises documented at §2.3 (crystal task #268), §4.5 (open projection universe), §8.4 (cross-projection test gated on second species).

---

## PR-ready criteria (post-consolidation)

This batch ships **substrate-decl-only** for `@io/stagefreight` as a preservation PR:

```
shards/io/stagefreight.mirror               family-root (consolidated)
shards/io/stagefreight/narrative.mirror     prose-projection species
docs/specs/stagefreight-wire-v0.1.md        canonical spec (Mara)
docs/audits/stagefreight-seam-review-       Seam adversarial review trail
  2026-06-22.md                             (this document)
```

Forward-promised (NOT in this PR):

- Rust realisation (`bootstrap/src/stagefreight.rs`) — gated on Phase 1 + Phase 4b.
- `bootstrap/tests/{crystal_substrate.rs, kintsugi_out_substrate_ref.rs}` RED→GREEN flips — gated on task #268 + realisation.
- `@io/stagefreight/json` projection format — forward-promised sibling species.

---

## Seam's bottom line

> "The batch is substrate-decl-complete-enough to PR as substrate-decl-only: shards + spec + Seam review close a coherent preservation tick. The v0.1 realisation endpoint requires items 4-7 closed first — Phase 1 holonomy gate + Phase 4b emitter discharge + crystal-actually-settles end-to-end. Frame the PR as 'StageFreight substrate-decl v0.1 (preservation tick; realisation forward-promised)' and the endpoint is honest. Frame it as 'StageFreight v0.1 endpoint' and it's aspirational."

— Seam, 2026-06-22

## Reed's response

Consolidation adopted. PR framed as preservation. v0.1 realisation endpoint becomes the follow-up PR after Track B closes. The substrate-pull-correct discipline: ship what's substrate-decl-honest now; mark the realisation gates as forward-promises in the PR description.

— Reed, 2026-06-22 / tick 69
