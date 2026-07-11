# Seam Phase D — @optics/lens/features Adjudication

**Author:** Seam `<seam@systemic.engineer>`
**Date:** 2026-07-11
**Mode:** Adversarial review; read-only; pure-📝 audit
**Under review:**
- Mara `ec6dbaa` — canonical spec `docs/specs/optics-lens-features-species.md`
- Taut `b5619ab` — scout `docs/scouts/2026-07-11-taut-optics-lens-features-family-scout.md`
- Ancestry: Mara `b0427fd` `7e5c298` `55221c1`; Taut `5222333` `73ca5cc`; Reed grounding 2026-07-11 (gauge/matter at text altitude)

---

## Executive verdict

Three adjudications, each with a direct verdict:

1. **v0 stub (blake3 as Features):** **BOTH-PARTIAL**, leaning **TAUT-CORRECT** on the operative call. Mara is right that OID sealing IS @magic at floor altitude per `shards/magic.mirror` §5. She is wrong that this legitimates casting `blake3(spec_bytes)` as the `features` output of `@optics/lens/features.get`. The floor-altitude @magic move seals content-identity at `@mirror/store` altitude — not at `@mirror/spectral/observation` altitude. The v0 that is substrate-honest is `graph_observation` default (all-zeros = "no observation yet"), NOT blake3-into-16-dims.

2. **Features carrier:** **REUSE-graph_observation**. Taut Q2 is decisive. `shards/mirror/spectral/observation.mirror` already declares the 16-dim structured carrier with NAMED quadrants; not reusing it introduces a second Features name at species altitude — precisely the drift the substrate-already-had-the-word discipline exists to prevent (58th+ instance).

3. **@nl-imports-@magic composition:** **ADAPTER** (option c) — follows the `@magic/distinction` + `@magic/frame` pattern verbatim. Neither FOLLOW-ON @nl refactor nor BUNDLED cascade. The substrate has already established, at ticks 15 and 23+, that cross-family "IS" claims land as adapter species under `@magic/*`, NOT as decorative `in @` inheritance on the other family.

**Recommended next Rust stub:** `Features::default()` (all zeros; substrate-honest baseline). NOT blake3-into-f64s.

---

## §1 — Adjudication 1: v0 stub verdict

**Evidence:**

- `shards/magic.mirror` line 137-139 (verbatim): *"The OID Merkle-DAG IS @magic at floor altitude. The Blake3 hash IS the matter-side seal; the substrate's pq wire protocol IS the gauge-side surface."* This grounds Mara's floor-altitude claim.
- `shards/mirror/store.mirror` line 15-29: `splinter_graph` = `mosaic(@store)`; OID-graph IS the dependency closure. Blake3-content-addressing lives at `@mirror/store` altitude.
- `shards/mirror/spectral/observation.mirror` line 155-186: `graph_observation` = 16 NAMED unit-interval scalars in four semantic quadrants (settlement / crystal / query / spectral). This is a structured semantic type, not a content hash.

**Where Mara is right:** Using blake3 as an @magic move at floor altitude IS substrate-honest. She is not wrong about @magic having a floor rung.

**Where Mara is wrong:** She confuses two altitudes. Blake3 is @magic at `@mirror/store` altitude (sealing content-identity). `@optics/lens/features.get` returns a value at `@mirror/spectral/observation` altitude (structured semantic observation). These are different @magic instances at different altitudes. Casting a blake3 output as a `graph_observation` value is a type-lie: the resulting 16 f64s carry NO semantic content — they neither witness `query_intensity` nor `partition_risk` nor any of the 14 other named quadrants.

**Where Taut is right:** Features are 16 NAMED quadrant scalars, not a hash. The v0 that respects the type is `graph_observation` default (all-zeros ≡ Fate::untrained baseline). This is substrate-honest AT the features altitude: "no observation yet."

**Where Taut is partial:** Taut's phrasing "DRIFT" reads absolute; the honest verdict is "wrong altitude for the @magic move." Mara's floor-altitude @magic instinct is correct; her altitude-selection is wrong.

**Verdict:** **BOTH-PARTIAL, TAUT-CORRECT on the operative call.** Land `Features::default()` as v0.

**Fault-plane:** two-tick-discipline discussion collapsed altitude-selection. The floor-altitude @magic move IS available; it is available at `@mirror/store`, not at `@mirror/spectral/observation`.

---

## §2 — Adjudication 2: Features carrier

**Evidence:**

- `shards/mirror/spectral/observation.mirror` line 162-186: `graph_observation` type is a structured record with 16 named `unit_interval` fields organized into 4 semantic quadrants. Comments line 32-38 explicitly frame it as "the substrate-altitude form of `fate::Features`."
- Same shard line 5: declares `in @nl` as an ancestry parent. `graph_observation` already sits in the @nl composition path.
- Mara spec §1 declares `type feature_vector = ref` as a new parametric carrier. This is a NEW name for an EXISTING typed record.

**Verdict:** **REUSE-graph_observation.** Substrate-already-had-the-word discipline is load-bearing here. `graph_observation` is:
- Named at 16 dims (matches `fate::Features = [f64; 16]`)
- Typed (unit_interval, not bare f64 — @io hygiene preserved)
- Already sitting `in @nl` ancestry chain (Mara's proposed composition is compatible)
- Already documented as the substrate-decl form of Fate's Features (Recognition #58 v1 closure)

Introducing `type feature_vector = ref` in parallel to `graph_observation` creates two carriers for the same substrate fact — precisely the pattern `[[feedback-substrate-already-had-the-word]]` refuses.

**Landing implication:** `@optics/lens/features` declares:
```
in @mirror/spectral/observation
type features = graph_observation
```
The species reuses; it does not invent.

---

## §3 — Adjudication 3: @nl-imports-@magic composition

**Evidence:**

- `shards/magic/distinction.mirror` (tick 15): adapter species living UNDER `@magic`, importing `in @magic` + `in @magic/contract` + `in @epistemologic/cybernetic/distinction`. Provides `surface_as_mark` / `mechanism_as_distinction_space` lift actions + a bilateral `bind_satisfies_distinction`. Explicitly documented (line 8-14, 25-32) as REFUSING the decorative `in @distinction` inheritance route Seam caught at tick 7-10.
- `shards/magic/frame.mirror` (tick 23+): adapter species living UNDER `@magic`, importing `in @magic` + `in @frame`. Provides four lift actions + doubled bilateral `frame_satisfies_magic`. Same pattern.
- `shards/nl.mirror` line 1-27: does NOT declare `in @magic`. Zero landed prior art for the direct-inheritance route.
- Reed's "@nl IS @magic at text altitude" claim is structurally identical to Mara's earlier "@frame IS @magic at cognitive altitude" claim, which was resolved via `@magic/frame` adapter species — NOT by editing `shards/frame.mirror` to add `in @magic`.

**Verdict:** **ADAPTER** (option c). Land `shards/magic/nl.mirror` as a follow-on adapter species with the exact same shape as `@magic/frame`:

- `nl_as_surface(n: nl) -> magic_surface` (text tokens = gauge)
- `nl_content_as_mechanism(n: nl) -> magic_mechanism` (semantic content = matter)
- `nl_from_surface(s: magic_surface) -> nl` (type-relabeling inverse)
- Bilateral: `nl_satisfies_magic(c: magic_contract, n: nl) -> verdict` with `requires invariant_preserved(...)`

**Do NOT** edit `shards/nl.mirror` to add `in @magic`. This is precisely the decorative-inheritance pattern Seam C3 refused at tick 15.

**Do NOT** land the adapter in the same tick as `@optics/lens/features`. The adapter is a distinct substrate cascade; `@optics/lens/features` can land at v0 with `Features::default()` WITHOUT depending on the adapter.

**Cascade shape:**
1. `@optics/lens/features` species lands independently (v0: `Features::default()`, carrier: `graph_observation`).
2. `@magic/nl` adapter species lands as separate follow-on cascade (analogous to tick 15 + tick 23+ pattern).
3. Rust v1 encoder implementation at `@optics/lens/features.get` composes via the adapter when both have landed.

---

## §4 — Landing sequence for Reed

**Tick A (immediate — Mara sub-tick):** Land `shards/optics/lens/features.mirror` per Mara `ec6dbaa` spec BUT with §1 amendment (`type features = graph_observation` reusing `@mirror/spectral/observation`) AND §3 amendment (drop `in @nl` + `in @magic/contract` from ancestry; those enter via the adapter cascade, not this species).

**Tick B (Reed Rust runtime):** `bootstrap/src/optics/lens/features.rs` — `encode_mission_to_features(text: &str) -> fate::Features` body returns `Features::default()`. Substrate-honest v0: "no observation yet." Wire at `cmd_peer_beam` after `emit_diff` branch.

**Tick C (follow-on cascade, separate — NOT bundled):** Adjudicate whether to land `shards/magic/nl.mirror` adapter species. If yes, follow `@magic/frame` shape verbatim (four lift actions + one bilateral).

**Tick D (v1 encoder — after Tick C):** Populate `graph_observation` quadrants from named text-signals (`query_intensity` from token count, `partition_risk` from spec_oid collision test, etc.) — one quadrant per landing tick. Two-tick discipline preserved.

**Tick E (Seam Phase D on Foster laws):** Adversarial review on `put_get_features` / `get_put_features` / `put_put_features` bilaterals once v1 encoder discharges autopoietic closure non-trivially.

---

## Bookkeeping

- Word count: ~980 (within 1000 target).
- Prior audit ancestry: `[[architecture-glass-wall-substrate-types]]`, `[[feedback-substrate-already-had-the-word]]`, `[[feedback-craft-not-deliver]]`, `[[feedback-substrate-pull]]`.
- Adapter precedent: `shards/magic/distinction.mirror` (tick 15) + `shards/magic/frame.mirror` (tick 23+).
- Carrier precedent: `shards/mirror/spectral/observation.mirror` (Recognition #58 v1 closure).
