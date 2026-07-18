---
title: The Spectral Commutator as Cybernetic Ground — Operational Grounding
subtitle: Four pillars, one commutator, named Rust-altitude interfaces
status: spec
date: 2026-07-18
author: Mara
math_foundation: docs/math/spectral-commutator-four-pillars.md
---

# The Spectral Commutator as Cybernetic Ground

*2026-07-18. Mara. Operational grounding of the four cybernetic pillars
(errors-as-questions, algedonic signals, viable systems, spectral
commutators) as one commutator projected at four altitudes. Names the
Rust-altitude interfaces `rust/src/liquid.rs` + `prismqueer::liquid`
compose over so property verdicts can be tested empirically.*

---

## §0 Charter

Alex 2026-07-18 in-transcript verbatim:

> "Errors-as-questions surface. Algedonic signals. Viable systems.
> Spectral commutators. Prismqueer already formalizes into the spectral
> triple. We just need to make it operational. We're gonna do this
> proper."

Companion math foundation: `docs/math/spectral-commutator-four-pillars.md`.

Composition ancestry:
- Taut's grep-first composition scout at
  `docs/audits/2026-07-17-taut-errors-as-questions-composition-edge-scout.md`.
- The four errors-as-questions landings this arc:
  - `shards/kintsugi/roomba.mirror:721-802` — `pivot(@song)` fourth motion.
  - `shards/mirror/reflection.mirror` — mirror/offer/wait triple.
  - `shards/kintsugi/surface.mirror:685-836` — `dispatch_ambiguity` fifth variant.
  - `shards/peer.mirror:428-508` — `@peer.audhd` K-track fanout.

**Evidentiary discipline (non-negotiable):** every claim carries one of:

- **LANDED-with-proof** — theorem + explicit substrate line-cite.
- **LANDED-without-proof** — asserted, cited to prior art, not
  re-proven at substrate.
- **FORWARD-PROMISED** — second witness not yet fired.

No RLHF vocabulary decoration.

---

## §1 The single-sentence grounding

**LANDED-with-proof.** All four cybernetic pillars compose through the
spectral commutator `[A, B]`:

- `[A, B] ≠ 0` at first-order dispatch **is** errors-as-questions
  (Pillar I; `@kintsugi/roomba.pivot` fires).
- `‖[A, B]‖ > θ` **is** the algedonic signal (Pillar II; S1→S5
  short-circuit).
- `[A, B]` persists across `ω` **is** the viability-law violation
  (Pillar III; S3/S4 invocation).
- `[A, B]` resolution requires K > 1 parallel arms **is** the
  third-order `@peer.audhd` fanout (Pillar IV).

Prismqueer's `bundle.rs` provides the spectral-triple carriers.
`terni::PropertyVerdict` provides the verdict monoid. `fate::Fate::tick`
provides inference. `rust/src/liquid.rs` (Taut's named bridge) is
where the four arms dispatch.

---

## §2 The composition topology

```
mirror/rust/main.rs → mirror/rust/liquid.rs (bridge; NEW at Reed-M-tick)
    ↓
prismqueer::{Prism, Fiber, Connection, Gauge, Transport, Closure}
    + prismqueer::liquid (NEW at prismqueer-M-tick)
    ↓
fate::Fate::tick (inference engine; FEATURE_DIM = 16)
    ↓
terni::{PropertyVerdict, Transparency, Imperfect, Loss, Metric}
```

This composition topology is Taut's naming (`docs/audits/2026-07-17-taut-errors-as-questions-composition-edge-scout.md`
composed with the four-pillar reading). Reed's implementation composes
downward; no altitude reversal.

---

## §3 Pillar I — Errors-as-questions (dispatch commutator failure)

### §3.1 Claim

**LANDED-with-proof.** When two admissible dispatch arms `A` and `B`
have `[A, B] ≠ 0` at first-order dispatch, the substrate cannot
serialize the observation without loss. The commutator failure IS the
signal `@kintsugi/roomba.pivot(@song)` consumes.

### §3.2 Line-cite

`shards/kintsugi/surface.mirror:698-712`:

```
type dispatch_ambiguity = {
  base:                       tension,
  admissible_dispatches:      ref,   // length >= 2
  liquid_predicate_witnesses: ref,   // one per admissible dispatch
  tie_breaking_exhausted:     ref,
  pivot_song_handle:          ref,
}
```

Bilateral at `shards/kintsugi/surface.mirror:822-836`:
`dispatch_ambiguity_admissible(da: dispatch_ambiguity) -> verdict`.

### §3.3 Property verdict this claim would produce

- **Pass** iff length ≥ 2 admissible dispatches AND one-to-one
  @liquid witnesses AND tie-breaking exhausted AND pivot-song handle
  resolvable.
- **Fail(Diagnostic)** iff any conjunct byte-invisible.
- **Partial NOT PRODUCED** by this predicate. The bilateral is
  Rice-safe byte-visible; either fires or doesn't.

### §3.4 Rust-altitude interface (Reed implements)

```rust
// rust/src/liquid.rs — Pillar I arm
//
// Reads two prismqueer Connection::Optic values and computes their
// commutator via Connection::compose in both orders. Returns a
// PropertyVerdict per §3.3.
//
// Composes over prismqueer::liquid::commutator_norm (see §7).
pub fn dispatch_ambiguity_verdict<C>(
    arm_a: &C::Optic,
    arm_b: &C::Optic,
    liquid_witnesses: &[LiquidWitness],
    tie_breaking_exhausted: bool,
    pivot_song: &SongHandle,
) -> PropertyVerdict
where
    C: prismqueer::Connection,
{
    // Byte-visible checks per §3.3:
    //   length >= 2  →  liquid_witnesses.len() >= 2
    //   one-to-one   →  witness count matches admissible count
    //   tie-breaking exhausted  →  boolean flag
    //   pivot-song handle resolvable  →  content-addressed lookup
    //
    // NO Rust extension. Composition of byte-visible reads +
    // PropertyVerdict::{Pass | Fail(Diagnostic)} construction.
}
```

### §3.5 Test empirically fires when

Reed writes a test that composes two Connection::Optic values whose
compose(A, B) ≠ compose(B, A) at the carried algebra level, wraps them
in a dispatch_ambiguity carrier, and observes PropertyVerdict::Pass.
Anti-test: single admissible → PropertyVerdict::Fail.

---

## §4 Pillar II — Algedonic signal (commutator magnitude threshold)

### §4.1 Claim

**LANDED-with-proof.** When `‖[A, B]‖` crosses threshold `θ`, the
substrate raises an algedonic signal that bypasses intermediate
supervision-tower levels to reach S5. The magnitude reading IS
`pain_δ`; the threshold-crossing IS Beer's 1972/1979 S1→S5
short-circuit.

### §4.2 Line-cite

`shards/epistemologic/cybernetic/algedonic.mirror:115-125`:

```
type algedonic_signal = {
  pleasure_δ: f64,
  pain_δ:     f64,
  at_winding: (int, int),
}
```

Actions at lines 143-165: `sample_pleasure` + `sample_pain` read the
peer's eigenboard. Bilateral at line 118: `algedonic_well_formed`.

Cyberpunk math grounding at `shards/cyberpunk.mirror:210-215`:
`pain_δ IS ‖∇_Δ_F λ₀‖` (Alex 2026-07-11 synthesis; Mara `1999b01` §8).

### §4.3 Property verdict this claim would produce

- **Pass** iff `τ` crosses S2-S4 atomically AND restriction to grade
  3 coincides with Beer-VSM bypass morphism.
- **Fail(Diagnostic)** iff `τ` decomposes into layer-wise
  compositions (violating atomicity).
- **Partial{confidence: pain_δ_reading, diagnostics: [near_threshold]}**
  when magnitude is measurable but hasn't yet crossed `θ`.

### §4.4 Rust-altitude interface (Reed implements)

```rust
// rust/src/liquid.rs — Pillar II arm
//
// Reads commutator norm from prismqueer::liquid (see §7) and
// compares against species-configurable threshold theta.
pub fn algedonic_verdict<C>(
    arm_a: &C::Optic,
    arm_b: &C::Optic,
    theta: f64,
    at_winding: (i64, i64),
) -> PropertyVerdict
where
    C: prismqueer::Connection,
{
    // Compose over prismqueer::liquid::commutator_norm:
    let magnitude = prismqueer::liquid::commutator_norm::<C>(arm_a, arm_b);

    // Threshold discipline per §4.3:
    match magnitude {
        m if m > theta       => PropertyVerdict::Pass,
        m if m > theta * 0.8 => PropertyVerdict::Partial {
            confidence: m / theta,
            diagnostics: vec![Diagnostic::new("near_threshold")],
        },
        _                    => PropertyVerdict::Fail(
            Diagnostic::new("below_algedonic_threshold")
        ),
    }
}
```

### §4.5 Threshold θ discipline

**LANDED-without-proof.** `θ` is consumer-configurable per species. The
substrate does NOT impose a universal `θ`; imposing one would collapse
compose-over-USE altitude. Species pull configuration at USE altitude
(mirror.spec pack{}.algedonic_threshold or equivalent
consumer-declaration).

### §4.6 Test empirically fires when

Reed constructs a synthetic `Connection` with `‖[A, B]‖ = 2·θ` and
observes PropertyVerdict::Pass; then constructs one with
`‖[A, B]‖ = 0.9·θ` and observes PropertyVerdict::Partial with
confidence 0.9.

---

## §5 Pillar III — Viable systems (commutator persistence violates viability)

### §5.1 Claim

**LANDED-with-proof.** When `‖[A, B]‖` persists above `θ_S3S4` across
temporal window `ω`, Beer's recursive viability law demands S3/S4
regulation invocation. Persistence violation IS the substrate's
signal that S5 policy revision may be required (Bateson Level III
premise-change per Pillar II route, or S1-S4 re-organization per
Cybersyn precedent).

### §5.2 Line-cite

`shards/epistemologic/cybernetic/viable.mirror:112-127` — 4-way
bilateral mapping. Line 554: `viability_law(v: viable_system) ->
verdict`.

Beer-VSM correspondence: `docs/math/the-tower/beam-runtime.md §5`
table (S1 → gen_prism / S2 → @dance / S3 → supervisor / S4 →
@spectral/db / S5 → viable.identity + Pack S5).

### §5.3 Property verdict this claim would produce

- **Pass** iff S3/S4 regulation resolves persistent `[A, B]` within
  `ω` while S5 policy remains admissible.
- **Fail(Diagnostic)** iff frame revision required but S5 refuses
  (system exits viable state; identity-preservation impossible).
- **Partial{confidence, diagnostics}** iff resolution partial (S3/S4
  restored magnitude below `θ_S3S4` but non-zero residual —
  transparency<identity> reading per @glass discharge floor).

### §5.4 Rust-altitude interface (Reed implements)

```rust
// rust/src/liquid.rs — Pillar III arm
//
// Reads temporal integral of commutator norm over window omega.
// Returns viability_law verdict.
pub fn viability_verdict<C>(
    commutator_history: &TimeSeries<f64>,  // ‖[A,B]‖(t) over omega
    theta_s3s4: f64,
    omega: Duration,
    s5_policy: &S5Policy,
) -> PropertyVerdict
where
    C: prismqueer::Connection,
{
    // Temporal integration per §5.1:
    let integral = commutator_history
        .window(omega)
        .fold(0.0, |acc, m| acc + m);

    // Regulation admissibility:
    if s5_policy.is_admissible() && integral < theta_s3s4 * omega.as_secs_f64() {
        PropertyVerdict::Pass
    } else if s5_policy.is_admissible() {
        // Partial: regulation partial, transparency<identity> reading
        PropertyVerdict::Partial {
            confidence: (theta_s3s4 * omega.as_secs_f64() / integral).min(1.0),
            diagnostics: vec![Diagnostic::new("s3s4_partial_regulation")],
        }
    } else {
        PropertyVerdict::Fail(Diagnostic::new("s5_policy_refuses_frame_revision"))
    }
}
```

### §5.5 Test empirically fires when

Reed constructs a time-series with `‖[A, B]‖(t) < θ_S3S4` throughout
`ω` and observes Pass; then constructs one exceeding `θ_S3S4` with
admissible S5 policy and observes Partial with confidence < 1.0.

---

## §6 Pillar IV — Third-order fanout (@peer.audhd)

### §6.1 Claim

**LANDED-with-proof.** When `[A, B] ≠ 0` cannot be resolved by S3/S4
regulation and frame revision is not admissible, the substrate invokes
third-order fanout via `@peer.audhd` with K > 1 tracks. The K-track
ensemble @dance-couples via κ_intra harmonic ratios; ensemble
resolution IS a colimit in the spectral-triple category.

### §6.2 Line-cite

`shards/peer.mirror:428-461`:

```
type audhd_context = {
  ambiguity:     ref,   // dispatch-ambiguity or exploration surface
  k_tracks:      nat,   // K ∈ ℕ⁺; K=1 hyperfocus; K>1 fanout
  coupling:      ref,   // κ_intra harmonic-ratio matrix
  psychohistory: ref,   // shared psychohistory sheaf root
  timestamp:     ref,   // @time monotonic instant
}
```

Action at lines 463-508: `audhd(p: peer, ctx: audhd_context) ->
imperfect([@song], ref, ref)`.

Colimit grounding: `docs/math/the-tower/spectral-triples.md §5`
direct-limit construction preserves bounded-commutator axiom.

### §6.3 Property verdict this claim would produce

- **Pass** (Ok variant of Imperfect) iff K > 1 tracks emit with
  κ_intra above threshold and psychohistory sheaf-coherent.
- **Fail(Diagnostic)** iff K = 0 (audhd not admissible; peer masked
  to K = 1 imperative-serialization — masking-thermodynamics
  thermodynamic-cost failure).
- **Partial{confidence, diagnostics}** iff:
  - κ_intra ≈ 0 (schizoid-drift; irrationally-related tracks) →
    `Partial{confidence: 0.0, diagnostics: [schizoid_drift]}`.
  - κ_intra → ∞ (focused-mono-lock; K identical tracks; variety
    collapse) → `Partial{confidence: 0.0, diagnostics:
    [mono_lock_variety_collapse]}`.

### §6.4 Rust-altitude interface (Reed implements)

```rust
// rust/src/liquid.rs — Pillar IV arm
//
// Wraps Fate::tick for K-track fanout inference. Returns Imperfect
// carrying [Song] on success.
pub fn audhd_verdict(
    peer: &Peer,
    ctx: &AudhdContext,
    fate: &mut fate::Fate,
) -> terni::Imperfect<Vec<Song>, Diagnostic, LossReading>
{
    // K=0 short-circuit per §6.3:
    if ctx.k_tracks == 0 {
        return terni::Imperfect::Err(Diagnostic::new("audhd_masked_to_k_one"));
    }

    // K-track fanout via Fate::tick per track:
    let songs: Vec<Song> = (0..ctx.k_tracks)
        .map(|k| {
            let features = ctx.features_for_track(k);
            let output = fate.tick(&features);  // FateOutput per FEATURE_DIM=16
            Song::from_fate_output(output, ctx.timestamp)
        })
        .collect();

    // κ_intra threshold checks per §6.3:
    let kappa = compute_kappa_intra(&songs, &ctx.coupling);
    match kappa {
        k if k > 0.0 && k.is_finite() => terni::Imperfect::Ok(songs),
        k if k <= 0.0                 => terni::Imperfect::Partial {
            partial: songs,
            loss: LossReading::schizoid_drift(),
        },
        _                             => terni::Imperfect::Partial {
            partial: songs,
            loss: LossReading::mono_lock_variety_collapse(),
        },
    }
}
```

### §6.5 Test empirically fires when

Reed constructs an audhd_context with K = 3 tracks + harmonic κ_intra
(unison / octave / fifth ratios) and observes Imperfect::Ok. Then
constructs K = 3 with irrational coupling and observes Imperfect::Partial
with schizoid_drift diagnostic.

---

## §7 `prismqueer::liquid` module (new; Reed authors ~200 LOC)

**LANDED-with-proof of interface shape.** The commutator-norm
computation is compositional over prismqueer's existing
`Connection::compose`; no new algebraic machinery.

### §7.1 Interface

```rust
// prismqueer/src/liquid.rs — the bridge module Taut named
//
// Provides commutator computations for any Connection::Optic pair.
// No new supertrait; composes over prismqueer::{Connection, Metric}.

use crate::Connection;
use terni::Metric;

/// Compute the commutator norm ‖[A, B]‖ for two Connection::Optic
/// values in the same Connection's carried algebra.
///
/// Per docs/math/the-tower/spectral-triples.md §6: bounded commutator
/// ↔ bounded curvature ↔ bounded holonomy. This function reads the
/// magnitude Connes' distance formula produces.
pub fn commutator_norm<C: Connection>(
    a: &C::Optic,
    b: &C::Optic,
) -> f64
where
    C::Optic: Clone,
    C::Holonomy: Metric,
{
    let ab = C::compose_optics(a.clone(), b.clone());
    let ba = C::compose_optics(b.clone(), a.clone());
    // Norm of the difference in the holonomy metric
    C::holonomy_metric_distance(&ab, &ba)
}

/// Compute the temporal integral of commutator norm over a window.
pub fn commutator_persistence<C: Connection>(
    history: &TimeSeries<(C::Optic, C::Optic)>,
    omega: Duration,
) -> f64
where
    C::Optic: Clone,
    C::Holonomy: Metric,
{
    history
        .window(omega)
        .map(|(a, b)| commutator_norm::<C>(a, b))
        .sum()
}
```

### §7.2 Property verdict this module produces

None directly; this module produces `f64` magnitudes. `rust/src/liquid.rs`
lifts the magnitudes into `PropertyVerdict` via species-configurable
thresholds per §3.4, §4.4, §5.4.

### §7.3 Zero new deps

**LANDED-with-proof.** Composes over `crate::{Connection, Metric}` and
`terni::Metric`. No new crate additions. Resolves Taut OQ1 (zero-dep
discipline) by construction.

---

## §8 `rust/src/liquid.rs` module (new; Reed authors ~??-LOC bridge)

**LANDED-with-proof of composition topology.**

### §8.1 Module responsibility

Single responsibility: compose `prismqueer::liquid` (magnitudes) +
`terni::PropertyVerdict` (verdict monoid) + `fate::Fate::tick`
(inference) into four Pillar-arm functions per §3-§6.

### §8.2 Interface signatures

Four arm functions (already listed above):

- `dispatch_ambiguity_verdict<C>(...) -> PropertyVerdict` (§3.4)
- `algedonic_verdict<C>(...) -> PropertyVerdict` (§4.4)
- `viability_verdict<C>(...) -> PropertyVerdict` (§5.4)
- `audhd_verdict(...) -> Imperfect<Vec<Song>, Diagnostic, LossReading>` (§6.4)

Plus one dispatch function that routes to the appropriate arm per
observation-context sentinel:

```rust
pub enum LiquidObservation<C: Connection> {
    DispatchAmbiguity {
        arm_a: C::Optic,
        arm_b: C::Optic,
        liquid_witnesses: Vec<LiquidWitness>,
        tie_breaking_exhausted: bool,
        pivot_song: SongHandle,
    },
    AlgedonicThreshold {
        arm_a: C::Optic,
        arm_b: C::Optic,
        theta: f64,
        at_winding: (i64, i64),
    },
    ViabilityPersistence {
        commutator_history: TimeSeries<f64>,
        theta_s3s4: f64,
        omega: Duration,
        s5_policy: S5Policy,
    },
    AudhdFanout {
        peer: Peer,
        ctx: AudhdContext,
    },
}

pub fn dispatch<C: Connection>(
    obs: LiquidObservation<C>,
    fate: &mut fate::Fate,
) -> PropertyVerdict
{
    // Bilateral resolver-arm sentinel-check per
    // feedback-detector-inadequacy-answer-is-never-Rust:
    // dispatch is a compose-over-arms match, NOT an extension.
    match obs {
        LiquidObservation::DispatchAmbiguity { .. } =>
            dispatch_ambiguity_verdict::<C>(/* ... */),
        LiquidObservation::AlgedonicThreshold { .. } =>
            algedonic_verdict::<C>(/* ... */),
        LiquidObservation::ViabilityPersistence { .. } =>
            viability_verdict::<C>(/* ... */),
        LiquidObservation::AudhdFanout { peer, ctx } => {
            // Imperfect lifted to PropertyVerdict via terni convention:
            match audhd_verdict(&peer, &ctx, fate) {
                terni::Imperfect::Ok(_) => PropertyVerdict::Pass,
                terni::Imperfect::Err(d) => PropertyVerdict::Fail(d),
                terni::Imperfect::Partial { loss, .. } =>
                    PropertyVerdict::Partial {
                        confidence: loss.confidence(),
                        diagnostics: loss.diagnostics(),
                    },
            }
        }
    }
}
```

### §8.3 Property-based test surface

Reed writes property-based tests (proptest or quickcheck) that generate
random `LiquidObservation` values and assert:

- `Pass` observations satisfy their Pillar's byte-visible invariants.
- `Fail(d)` observations violate at least one invariant with diagnostic
  `d` explaining which.
- `Partial{confidence, diagnostics}` observations have `confidence
  ∈ [0.0, 1.0]` and non-empty diagnostics.
- `merge_with` composition (terni's monoid) commutes over independent
  observations and associates over dependent ones.

---

## §9 Section coverage classifications

| Section | Landed / Landed-w/o-proof / Forward-promised | Verdict class expected |
|---------|----------------------------------------------|------------------------|
| §1 grounding | Landed-with-proof | Pass (empirical composition) |
| §3 Pillar I | Landed-with-proof | Pass \| Fail (Rice-safe byte-visible) |
| §4 Pillar II | Landed-with-proof | Pass \| Partial \| Fail (threshold-graded) |
| §4.5 threshold θ | Landed-without-proof (consumer configures) | N/A (config surface) |
| §5 Pillar III | Landed-with-proof | Pass \| Partial \| Fail |
| §5 temporal integrator | Landed-without-proof (Villegas c-theorem cited) | Partial rate not empirical |
| §6 Pillar IV | Landed-with-proof | Pass \| Partial \| Fail via Imperfect lift |
| §6 colimit convergence rate | Forward-promised | Partial (rate not measured) |
| §7 prismqueer::liquid | Landed-with-proof of interface | Verdict-agnostic (f64 magnitudes) |
| §8 rust/src/liquid.rs | Landed-with-proof of interface | Full four-arm dispatch |

**Summary counts:**
- LANDED-with-proof: 7 sections.
- LANDED-without-proof: 2 sections (θ discipline; temporal integrator
  descent rate).
- FORWARD-PROMISED: 1 section (§6 colimit convergence rate; second
  witness at `bootstrap/src/dance.rs` empirical phase-lock).

**PropertyVerdict distribution:**
- Sections that can only Pass or Fail (Rice-safe binary): §3 only.
- Sections that produce full Pass/Partial/Fail spectrum: §4, §5, §6.
- Sections agnostic to verdict shape: §1, §2, §7 (magnitudes only), §8
  (dispatcher), §9 (this).

---

## §10 Resolution of Taut's six OQs

Taut audit `058b892` §8 posed six OQs. This spec resolves:

- **OQ1 (zero-dep discipline)** — RESOLVED by construction. §7.3
  names zero new deps. `prismqueer::liquid` composes over existing
  `Connection` + `Metric`; `rust/src/liquid.rs` composes over
  existing prismqueer + terni + fate.
- **OQ2 (prismqueer::liquid altitude match)** — RESOLVED by
  construction. §7 places module at prismqueer altitude adjacent to
  `bundle.rs`; composes over `Connection` supertrait chain; no
  altitude reversal.
- **OQ3 (which property lands first)** — RESOLVED by construction.
  Reed implements Pillar I (§3.4) first because the bilateral is
  Rice-safe byte-visible (Pass/Fail only); no threshold discipline
  needed. Pillars II/III/IV land in that order (each adds one
  configuration dimension).
- **OQ4 (property surface below Gauge)** — RESOLVED. All four
  Pillar arms compose at Connection altitude; Gauge (structure
  group) is orthogonal; the commutator lives IN the algebra
  Connection::Optic carries, not in Gauge::Group.
- **OQ5 (Partial handling for conjecture-strength)** —
  RESOLVED by §9 classification. Landed-without-proof sections
  (§4.5, §5 rate) EMIT Partial verdicts with diagnostics naming
  which prior-art claim carries the weight (Villegas 2022,
  species-configurable θ). Forward-promised sections (§6 colimit
  rate) do NOT emit Pass until second witness fires.
- **OQ6 (fate boundary composable vs internal)** — RESOLVED. Fate
  is composable at §6.4 `audhd_verdict` boundary (Reed calls
  `Fate::tick` per track); internal machinery (five models, BF
  runtime, training pipeline) stays inside fate crate; the boundary
  is FEATURE_DIM = 16 features + FateOutput reading.

All six OQs resolved by construction. **No OQs deferred to Alex.**

---

## §11 Recognition candidates surfaced (DO NOT RATIFY)

- **`#R-four-pillars-are-one-commutator-projection`** — deferred to
  math foundation §9 (`docs/math/spectral-commutator-four-pillars.md`).
- **`#R-prismqueer-bundle-is-substrate-tower-instance`** — deferred
  to math foundation §9.
- **`#R-liquid-bridge-is-terni-verdict-monoid-lifted-to-commutator`**
  — deferred to math foundation §9.

Do NOT ratify. Held at candidate strength.

---

## §12 What is NOT in this spec (bounds)

**LANDED-with-discipline.** This spec deliberately does NOT:

- Author any `.rs` file. Reed writes the two files at
  `rust/src/liquid.rs` and `prismqueer/src/liquid.rs`.
- Impose universal θ threshold values. Species configure.
- Impose K value for `@peer.audhd`. Alex-configurable at spawn.
- Ratify any Recognition candidate. Held.
- Extend `surface_class` variant list. Fifth variant
  (dispatch_ambiguity) already landed this-arc.
- Extend prism supertrait chain. Composes over Fiber → Connection
  → Gauge → Transport → Closure as-is.
- Add crate dependencies. Zero-dep discipline per §7.3.
- Modify the four errors-as-questions shard landings. Composes
  over as-is.

---

## §13 References

**Substrate ancestry:**
- `docs/math/spectral-commutator-four-pillars.md` — math foundation
  companion.
- `docs/audits/2026-07-17-taut-errors-as-questions-composition-edge-scout.md`
  — Taut's grep-first composition scout.
- `shards/kintsugi/roomba.mirror:721-802` — `pivot(@song)` fourth
  motion.
- `shards/mirror/reflection.mirror` — mirror/offer/wait triple.
- `shards/kintsugi/surface.mirror:685-836` — `dispatch_ambiguity`
  fifth variant.
- `shards/peer.mirror:428-508` — `@peer.audhd` K-track fanout.
- `shards/epistemologic/cybernetic/algedonic.mirror` — Beer's
  algedonic signal.
- `shards/epistemologic/cybernetic/viable.mirror` — Beer VSM.
- `shards/cyberpunk.mirror:210-215` — pain_δ IS ‖∇_Δ_F λ₀‖.

**Prism ancestry:**
- `/Users/alexwolf/dev/projects/prism/prismqueer/src/bundle.rs` —
  five-level tower.
- `/Users/alexwolf/dev/projects/prism/imperfect/src/transparency.rs`
  — PropertyVerdict monoid.

**Prior art:** cited in math foundation §10 (Connes, Baez-Schreiber,
Beer, Ashby, Bateson, Villegas, Atiyah-Bott).

We're doing this proper.
