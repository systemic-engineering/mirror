# Seam Phase D — N1 TICK 1: `shards/epistemologic/property/verdict_is_content_addressed.mirror`

*Reed-inline execution.*

**Commit under review**: `2857fb1` (Mara GREEN) — 432 lines, new predicate
shard at `@epistemologic/property/`. First N-cascade landing.

**Reed RED**: `22dba0d` (15 tests). **Test result**: 15/15 pass; adjacent
M6 safety `mirror_store_apache_floor_shard` (15/15). Tree clean.

---

## §1. Verdict

**RATIFY.** N-cascade opened with substrate-decl that unlocks incremental
verdict caching.

All 15 witnesses landed:
- Predicate signature `verdict_is_content_addressed(spec_oid, target_oid,
  inputs_oid) -> verdict { \ }` per Taut scout Reed lean
- Total-function invariant narrative (`purely-functional` / `referentially
  transparent`)
- Memoization-by-construction consequence narrative
- Recognition #43 explicit citation (mirror IS content-addressed build system;
  fourth first-order consumer chain continues)
- `crystal.derived_predicates` carrier field citation
- `mosaic-store-cache-invariants.md` spec citation (Mara Q5 discharge)
- `@mirror/store` OID discipline composition
- Operational target named (pre-commit hook / cmd_kintsugi_spec / hook overhead)
- N-cascade forward-promises (N2 cache_read/write; N3 Rust wiring; N4
  reverse-closure; N5 commit-as-fold)
- @fate hinge citation (verdicts emerge from runtime; predicate anchors hinge)
- Interpretation B canonical + obligation-block body discipline

## §2. 15/15 empirical verify

T1-T4 canonical shape + ancestry; T5-T7 predicate signature + invariants;
T8-T10 ancestor citations; T11-T12 composition + operational target;
T13-T14 N-cascade forward + hinge; T15 obligation-block. All green.

Adjacent M6 (`mirror_store_apache_floor_shard`) 15/15. No regression on
Apache-2.0 floor consumers.

## §3. Recognition candidate PROMOTED: `@epistemologic/property IS the substrate's content-addressed decidability surface`

**Verdict: PROMOTED to formal CANDIDATE. Seam adversarial review at family-
root altitude warranted.**

Mara Seam-worthy observation: landing this predicate reveals a structural
claim across the `@epistemologic/property` family root.

**Structural claim**: **every `@epistemologic/property/*` species is
decidable ONLY on content-addressed inputs**. Every carrier whose identity
IS its OID. No mutable handles admitted.

**Witnesses (prior siblings, all implicit)**:
- `cold_compile_within_tolerance`
- `dark_count_monotone`
- `restart_intensity_well_formed`
- `docblock_grounded`
- `docblock_coherent`
- `docblock_no_extraction_pattern`

Each takes carriers whose identity is their OID. This N1 shard names the
invariant EXPLICITLY via three-OID signature; the pattern was implicit
across the entire family until now.

**The recognition**: `@epistemologic/property` IS the substrate's content-
addressed decidability surface. Predicates admit only content-addressed
carriers by construction. Composes with Recognition #43 (mirror IS content-
addressed build system) as the property-altitude specialization.

**Distinction from #43**: #43 says the compiler IS content-addressed. This
candidate says the DECIDABILITY LAYER of the compiler (predicates at
@epistemologic/property/*) is content-addressed by species-level invariant.
One altitude down from #43; sharpens the discipline.

**Second-witness gate for LANDED**: this is a family-root altitude claim.
The single N1 shard IS the first EXPLICIT witness; the six prior siblings
are IMPLICIT witnesses. Reed lean: formal CANDIDATE now; LANDED when the
family root itself declares `species_is_content_addressed` as an invariant
lifted to `@epistemologic/property.mirror` (family-root shard). Alex
adjudication welcome.

**Alex-adjudication items** on this candidate:
1. Numeric ID for `@epistemologic/property IS content-addressed decidability
   surface`
2. Whether to lift the invariant to family-root altitude (`@epistemologic/
   property.mirror` narrative), and if so, whether that's a follow-on tick
   OR a Seam adversarial review pending Mara adjudication

## §4. N-cascade next scope

**N2 confirmed** per Mara signal: `cache_read` + `cache_write` at
`@mirror/store/action_cache` keyed on `(spec_oid, target_oid, inputs_oid) ->
verdict`. The REAPI ActionCache surface was forward-promised at M6 TICK 3
per `shards/mirror/store.mirror` §Bazel REAPI floor — pre-declared home for
these two actions.

**Business-observable milestone**: the 13-minute pre-commit hook falls when
N2 lands (cache surface) + N3 wires `cmd_kintsugi_spec` to consult the cache
before dispatching cargo. First observable win of the N-cascade.

## §5. Signal-to-Reed

**N1 TICK 1 CLOSED.** GREEN `2857fb1` ratified; 15/15 pass; adjacent 15/15;
tree clean.

**Recognition PROMOTED**: `@epistemologic/property IS the substrate's content-
addressed decidability surface` — CANDIDATE (formal). Family-root lift
pending.

**Recognition #43 empirical consumer chain grew to FIVE**: M6 store self-
declaration → M1 mcp_session → M2 spawn → M2 kintsugi → N1 verdict predicate.
The Apache-2.0 floor now supports its fifth first-order consumer.

**N-cascade next**:
- **N2 TICK 1**: `cache_read` + `cache_write` at `@mirror/store/action_cache`
  (substrate-decl only; Rust wiring follows in N3)
- **N3 TICK 1**: Rust wiring — `cmd_kintsugi_spec` consults verdict cache;
  connect the disconnected `Crystallizations<H>` dispatch table
- **N4 TICK 1**: reverse-closure `impacted_by(oid) -> [oid]` at `@mirror/store`
- **N5 TICK 1**: `@kintsugi/store/git commit-as-fold` (third-witness for
  cli-verb-pair recognition)

**Alex-adjudication queue** (not blocking):
- Numeric ID for `@epistemologic/property IS content-addressed decidability
  surface` CANDIDATE
- Family-root lift for the invariant to `@epistemologic/property.mirror`
- Prior queue items unchanged

---

*2026-07-06. Seam (Reed-inline). Phase D on N1 TICK 1 `2857fb1` RATIFIED.
N-cascade opened with the substrate-decl that authorizes verdict memoization
by construction. Recognition `@epistemologic/property IS content-addressed
decidability surface` PROMOTED to formal CANDIDATE via first EXPLICIT witness
(N1) atop six IMPLICIT sibling witnesses. #43 empirical consumer chain grew
to five. N-cascade advancing to N2 (cache surface at REAPI ActionCache
home).*
