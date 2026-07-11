# Taut scout — `@optics/lens/features` sibling-species boundary check

**Author**: Taut (`taut@systemic.engineer`)
**Date**: 2026-07-11 (iter-14; sibling to iter-13 `73ca5cc`)
**Scope**: `@optics/lens/features` as sibling species to `@optics/lens/diff`
under Mara `b0427fd` (family-root) + `7e5c298` (`/diff`).
**Ground refs**: Mara `b0427fd` `7e5c298` `55221c1`; Taut `5222333` `73ca5cc`;
Reed conversation grounding (@magic verbatim from `shards/magic.mirror`).
**Discipline**: Read-only. Grep-first. `📝` markdown-only.

---

## Executive summary (12 lines)

- **Q1 name check**: NO hits for `@optics/lens/features` / `features_lens` /
  `text_to_features` / `mission_encoder` across shards, boot, mirror.spec.
  The name is FREE at this altitude.
- **Q2 text→Features encoders**: `graph_observation` at
  `shards/mirror/spectral/observation.mirror` **IS the 16-dim Features
  carrier already landed**. It is the substrate-altitude form of what fate
  crate exposes as `pub type Features = [f64; 16]`. `shards/nl.mirror`
  declares `corpus / spectral_triple / connes_distance` (text→ratio); NO
  action producing `graph_observation` from text exists.
- **Q3 @nl↔@magic composition surface**: `shards/nl.mirror` does NOT
  declare `in @magic`. Zero `magic/*` prose in `nl.mirror`. The claim is
  SUBSTRATE-MOTION — coherent with magic-family shape, no prior art landed.
- **Q4 mosaic(feature)**: `shards/mirror/mosaic.mirror` declares the
  parametric `mosaic(altitude)` universal composition form. `mosaic(feature)`
  IS an admissible altitude specialization — the shape is already there.
  The 16-dim `graph_observation` composes as `mosaic(@mirror/spectral)`.
- **Q5 fault-planes**: no collision. `@optics/lens/features` fits the
  family-root shape symmetrically with `/diff`. Composition chain valid but
  requires two prereqs (§Landing sequence).
- **Q6 v0 stub verdict**: **DRIFT**. `blake3(spec_bytes) as Features` is
  substrate-motion for `@optics/lens/diff.get` (blake3-of-content is what
  content-addressing IS at @mirror/store); it is DRIFT for
  `@optics/lens/features.get` — features are 16 named quadrant scalars
  (settlement/crystal/query/spectral), not a hash.
- **@nl-as-@magic-at-text-altitude verdict**: **SUBSTRATE-MOTION**. The
  gauge/matter partition IS available at text altitude (raw prose = gauge;
  extracted structured features = matter); no shard names it yet. Landable,
  but adjacent to this scout, not entailed by it.
- **Overall LRM**: **LANDABLE-WITH-PREREQS** (two prereqs, one substrate
  refactor; see §Landing sequence).
- **Recommended next Reed action**: escalate the two prereq adjudications
  (§Landing sequence Prereqs 1 & 2) to Alex before Mara drafts spec.

---

## §Q1 — Name check

Grep across `shards/**/*.mirror`, `boot/**/*.mirror`, `mirror.spec`:

```
@optics/lens/features       → 0 hits
optics/lens/features        → 0 hits
features_lens               → 0 hits
text_to_features            → 0 hits
mission_encoder             → 0 hits
```

Verdict: **FREE**. Zero substrate collision.

## §Q2 — Existing text→Features (or text→observation) encoders

**`shards/nl.mirror`** (8.2KB, 2026-06-23): declares `corpus / token /
spectral_triple / affect_profile` carriers + `ingest / collect /
build_triple / connes_distance / measure_affect / compare` actions.
Does NOT declare `in @magic`. NO action producing `graph_observation` or
any `[f64; 16]`-shaped feature vector. `@nl.compare(c1, c2) -> ref` returns
a Connes-distance scalar, not a Features vector.

**`boot/std/nl.mirror`** (161B): declares `type nl(text) / type #(nl) /
doc(ast) / commit_message(imperfect)`. No feature extraction.

**`boot/std/nl/english.mirror`** (2.7KB): declares tokenization surface
(`tokenize / words / sentences / paragraphs / sections`) + POS tagging
(`tag / role / lemma`) + morphology + POS types. No `[f64; N]` output.

**LOAD-BEARING FINDING**: `shards/mirror/spectral/observation.mirror`
(8.4KB, 2026-06-12) declares:

```mirror
type unit_interval = f64
type graph_observation = {
  # 4 quadrants × 4 features:
  convergence_settled: unit_interval, pressure_load: unit_interval,
  node_occupancy: unit_interval, edge_density: unit_interval,
  crystal_fraction: unit_interval, settlement_depth: unit_interval,
  interval_ratio: unit_interval, hot_path_density: unit_interval,
  query_intensity: unit_interval, partition_risk: unit_interval,
  tick_maturity: unit_interval, mutation_rate: unit_interval,
  shannon_loss_rate: unit_interval, spectral_dimension: unit_interval,
  eigenvalue_gap: unit_interval, holonomy: unit_interval,
}
```

This IS the 16-dim substrate-altitude form of `fate::Features`. It closes
Recognition #58 v1: what the D²NN input layer reads. The `graph_observation`
carrier is the **substrate-already-had-the-word** for Fate's `Features`.

**GAP**: NO consumer action lifting `nl → graph_observation`. The shard
explicitly forward-promises "the action that *fills* `graph_observation`
from a `score`" but the `nl → observation` path was never named.

## §Q3 — @nl ↔ @magic composition surface

Grep `in @magic` and `magic/*` prose in `shards/nl.mirror`: **zero hits**.

Grep `in @nl` across shards: 41 shards import `@nl`, all for docblock
`# <text>` nl_literal (per shards/nl.mirror line 33 `type nl_literal(text)
= @sigil("#")`). None cite `@nl IS @magic` composition.

Grep specs/math for `nl.*at text altitude` / `@nl IS @magic` /
`text-altitude gauge`: **zero hits**.

Verdict: The proposed composition (`raw text = gauge surface; structured
features = matter mechanism; encoder = magic/contract.bind`) is
substrate-coherent but has ZERO landed prior art. Reed's grounding cites
`shards/magic.mirror` verbatim correctly (surface, mechanism, contract as
5-op gauge / matter partition). The **application to text altitude is
motion, not established.**

## §Q4 — mosaic(feature) composition altitude

`shards/mirror/mosaic.mirror` §"mosaic(altitude) — the parametric universal
composition form":

```mirror
type mosaic(altitude) = ref
# Examples given:
#   mosaic(@store)       is the splinter-graph
#   mosaic(@spec)        is the project manifold
#   mosaic(@emitter)     is the shifted-to-altitude intermediate
#   mosaic(@code/rust)   is the resolved Rust workspace
#   mosaic(@ci/github)   is the GitHub Actions YAML manifold
```

`mosaic(@mirror/spectral)` is admissible by direct symmetry with the
5 examples — the observation quadrants are already 4×4-partitioned. The
`mosaic(feature)` shape Reed proposed is BROADER than a single altitude
declaration; it would need naming as either `mosaic(@mirror/spectral)` or
`mosaic(@optics/lens/features)` (self-referential — the species mosaics its
own carrier). Neither is landed but the SHAPE is landed.

## §Q5 — Fault-planes under `@optics/lens/features` landing

Ancestry chain `@nl + @magic/contract + mosaic + @shatter`:

- `in @nl` — 41 landed consumers; SAFE.
- `in @magic/contract` — 8 landed consumers (magic/distinction,
  magic/frame, magic/mechanism, magic/reveal, magic/surface, code/beam,
  io/stagefreight, smarts/magic); SAFE.
- `in @mirror/mosaic` — NOT USED as an `in @` import anywhere; imported
  via own species declaration only. Motion.
- `in @shatter` — per Taut iter-12 (`5222333` §5) verdict: `@shatter` is
  **NOT usable as `in @`** — @shatter species live as ONE INSTANCE of the
  (get, put) Foster-pair at species altitude. Reed's Q6.b composition
  string `|> @shatter` is COMPOSITIONAL (an operation, not an ancestry
  import). Substrate-honest reading: `@shatter` here = "witness Foster
  laws", which the species inherits via `in @optics/lens` PARENT already.
  Do NOT add `in @shatter`.

**Collision with existing species**: `@optics/lens/features` has zero
substrate collision (Q1). Sibling to `@optics/lens/diff` under
`@optics/lens` is symmetric composition — same shape as adding
`@nl/english` under `@nl`.

## §Q6 — Peer_beam runtime consequence

If `@optics/lens/features` lands, `cmd_peer_beam`'s fate-wiring becomes:

```
mission_text |> @optics/lens/features.get -> Features [f64; 16]
             |> fate.select                -> Decision { model, ... }
             |> @optics/lens/diff.get      -> diff_bytes
             |> @io.write
```

**v0 stub verdict**: current `bootstrap/src/lib.rs:4160` `emit_peer_beam_diff`
emits `blake3(spec_bytes)` **as the spec_oid** — that is
`@mirror/store.oid`, NOT `@optics/lens/features.get`. Using it as a
Features `[f64; 16]` (16 bytes hash → 16 f64 unit-intervals) would be
**DRIFT**: features are 16 NAMED quadrant scalars (Recognition #58,
`shards/mirror/spectral/observation.mirror` §"The 16 features"), not a
content hash. A blake3-modulo-encoder is substrate-motion at spec_oid
altitude and substrate-drift at features altitude.

**Substrate-honest v0 stub**: return `graph_observation` where each
quadrant scalar is derived from a NAMED text-signal (e.g.
`query_intensity = 1.0`, `partition_risk = 0.0`, others zero) —
substrate-truthful about which dims mission-text populates (Q6.c from
iter-13). Better still: forward-promise the encoder body as `\ {}` at
substrate-decl altitude, land Rust stub as `Features::default()` (all
zeros ≡ Fate::untrained baseline; substrate-honest as "no observation
yet").

---

## §Landing sequence (LANDABLE-WITH-PREREQS)

### Prereq 1 (Alex adjudication)
**Which is the 16-dim carrier?** Options:

- **A**: `@optics/lens/features` REUSES the landed `graph_observation`
  from `@mirror/spectral/observation` (substrate-already-had-the-word).
  The species declares `type features = graph_observation` and imports
  `in @mirror/spectral/observation`.
- **B**: `@optics/lens/features` declares its OWN `type features = ref`
  (following the `/diff` pattern of `type diff_bytes = ref`), leaving the
  16-dim structure discharge to Rust.

Reed adjudication (my read): **Option A**. `graph_observation` IS the
substrate-decl form Reed grounded to; NOT reusing it is drift-motion
introducing a second Features name at species altitude.

### Prereq 2 (Alex adjudication)
**Does `@nl` need to be extended to declare `in @magic`?** Options:

- **X**: YES — the encoder IS the text-altitude @magic/contract.bind, so
  @nl imports @magic and declares `nl_gauge` (raw text = surface) /
  `nl_matter` (extracted structure = mechanism). This is the
  @nl-as-@magic-at-text-altitude claim substantiated at substrate-decl
  altitude. Follow-on species.
- **Y**: NO — @optics/lens/features handles the encoder in Rust; @nl
  stays measurement-only. Simpler; loses the magic composition Reed
  proposed at Q6.b.

Reed adjudication (my read): **defer to X in a follow-on tick**; land
`@optics/lens/features` with **Y** (Rust-side encoder, no @nl extension)
first. Two-tick discipline: minimum-cut landing for Blocker 2 closure,
then @nl↔@magic composition as separate cascade.

### Minimum-cut landing (parallels @optics/lens/diff)

Same shape as `55221c1` spec + `5222333` scout + `b0427fd` + `7e5c298`:

1. **Mara spec**: `docs/specs/optics-lens-features-species.md` (this
   scout + Alex answers to Prereq 1 & 2).
2. **Mara substrate-decl**: `shards/optics/lens/features.mirror` —
   sibling to `/diff` under `@optics/lens`. Imports: `@prism @glass @meta
   @nl @optics/lens @mirror/spectral/observation` (Option A). Actions:
   `get(text: nl) -> graph_observation { \ {} }` + `put(edited:
   graph_observation, old_text: nl) -> nl { \ {} }` + three Foster
   bilaterals specialized to features carrier.
3. **Reed Rust runtime**: `bootstrap/src/optics/lens/features.rs` —
   encoder function `encode_mission_to_features(text: &str) ->
   fate::Features`. v0 body: return `Features::default()` (all zeros;
   substrate-honest "no text observation yet"). v1 body: populate
   `query_intensity` from token count, `partition_risk` from spec_oid
   collision test, etc. — one quadrant per landing tick.
4. **Wiring at `cmd_peer_beam`**: after `emit_diff` branch, gate the
   fate-wiring on `@optics/lens/features.get(mission_text) → features`
   → `fate::Fate::excited().tick(&features)` → envelope carries
   `model` + `confidence` + `distribution`.
5. **Seam Phase D audit**: Foster laws at features altitude
   (`put_get_features`, `get_put_features`, `put_put_features`).

### Cascade grounding

- Mara `b0427fd` — `@optics/lens` family-root, three Foster bilaterals.
- Mara `7e5c298` — `@optics/lens/diff` sibling shape this scout mirrors.
- Mara `55221c1` — iter-24 spec §7 Tick 4 (Rust runtime discharge as
  separate tick).
- Taut `5222333` — iter-12 §5 (`@shatter` compositional, not `in @`).
- Taut `73ca5cc` — iter-13 §Q6.b (mission-text encoder home ambiguity —
  THIS scout answers Q6.b: @optics/lens/features, Option Y v0).
- Reed conversation grounding — `shards/magic.mirror` verbatim (surface,
  mechanism, contract as 5-op gauge/matter partition).

Word count: ~1085 (within 1100 target).
