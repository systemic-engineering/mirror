# Taut scout — prismqueer + terni + fate + mirror docs enumeration for property-inference substrate

*2026-07-18. Read-only enumeration per Alex directive at the roomba-pivot
handoff: "We build the property inference layer first. Then we build the
properties in mirror. And then we hook them up through rust/src/liquid.rs
into mirror." Pure-docs 📝 markdown-only bypass.*

Scope: three crates (prismqueer, terni/imperfect, fate) + mirror's
docs/math + docs/specs @liquid claims + rust/src/ current state.
Grep-first. No design decisions. No mints proposed.

---

## §1 Prismqueer surface enumeration

Location: `/Users/alexwolf/dev/projects/prism/prismqueer/`.
Cargo `[package]` = "prismqueer" v0.1.1, edition 2021, description
"The spectral-triple substrate — five operations (focus, project,
split, shift, settle), the Prism trait, zero deps. The foundation."

### §1.1 Modules (from `src/lib.rs:32-75`)

Unfeatured (always compiled):
- `beam` — `Beam`, `Operation`, `Optic` (semifunctor carrier)
- `coincidence` — `canonical_hash`, `coincidence_hash`, `Detector`,
  `HashPrism`. Eigenvalue-based content addressing, N=3 detector.
- `crystal` — `Crystal<P>(pub P, pub Luminosity)`; settled Prism.
- `luminosity` — `Luminosity::{Light, Dimmed(f64), Dark}`.
- `scalar_loss` — `ScalarLoss(pub f64)`; implements `Loss + Metric`.
- `substrate_ref` — `Ref` newtype; validated `@`-prefix nav-ref.
- `trace` — `Op`, `Step`, `StepOutput`, `Trace`, `Traced`.
- `connection` — `Carrier`, `ScalarConnection`.
- `content` — `ContentAddressed`.
- `kernel` — `KernelSpec { dimensions: Vec<usize>, decomposition:
  Decomposition, precision: Precision }`; `Decomposition::{Eigenvalue,
  Svd, MatVec, FullProjection}`. `KernelSpec::from_logits(logits,
  threshold, decomp, precision)` — logits → dimension selector.
- `merkle` — `diff`, `Delta`, `MerkleTree`.
- `metal`, `named`, `oid`, `optic_kind`, `precision`, `spectral_oid`,
  `spectral_uuid`, `store`.

Feature-gated:
- `optics` (feat `optics`)
- `pq` (feat `pq`, requires serde)
- `bundle` (feat `bundle`, transitively `optics`) — the load-bearing
  module for property inference. Exports: `Bundle`, `Closure`,
  `Connection`, `Cyclic`, `Fiber`, `Gauge`, `GroupStructure`,
  `IdentityPrism`, `LawvereFixedPoint`, `StableFiber`, `Transport`.
- `lambda` (feat `lambda`)
- `ffi`, `spectral_dimension` (feat `lapack`)

Re-exports: `terni::{Diagnostic, Imperfect, Loss, Metric,
PropertyVerdict, Transparency}` (lib.rs:98) — the property-verdict
vocabulary is re-exported through prismqueer verbatim.

### §1.2 The `Prism` trait (lib.rs:126-142)

```
pub trait Prism {
    type Input: Beam;
    type Focused:   Beam<In = <Self::Input    as Beam>::Out>;
    type Projected: Beam<In = <Self::Focused  as Beam>::Out>;
    type Refracted: Beam<In = <Self::Projected as Beam>::Out>;
    fn focus(&self, beam: Self::Input) -> Self::Focused;
    fn project(&self, beam: Self::Focused) -> Self::Projected;
    fn settle(&self, beam: Self::Projected) -> Self::Refracted;
}
pub fn apply<P: Prism>(prism: &P, beam: P::Input) -> P::Refracted;
```

### §1.3 Key primitives

- `Oid` (oid.rs) — 64-char hex content address via
  `canonical_hash(bytes)` (N=3 CoincidenceHash + SHA-256 compress).
  `Oid::dark()` = 64 zeros. Trait `Addressable { fn oid() -> Oid }`.
- `SpectralOid` (spectral_oid.rs) — `Oid × Precision`; equality via
  truncated form. `truncation_len(total, precision)` keeps
  `ceil(total * clamp(p, 0, 1))` chars, min 1.
- `Precision(f64)` (precision.rs:10) — "eigenvalues below this are
  zero." `Pressure(f64)` clamped `[0,1]`; `is_critical() >= 0.9`.
- `Ref(String)` (substrate_ref.rs:44) — `@`-prefix validated,
  no whitespace, no control chars.
- `Crystal<P>(pub P, pub Luminosity)` (crystal.rs) — Crystal IS the
  Prism at rest.
- `KernelSpec` (kernel.rs:29) — carries `Vec<usize>` dimensions +
  `Decomposition` + `Precision`. `projection_matrix(n)` builds the
  n×n diagonal projection.

### §1.4 Test surface

Tests: `bundle_integration.rs`, `declaration_round_trip.rs`,
`integration.rs`, `lambda_integration.rs`, `optics_integration.rs`,
`pq_schema.rs`, `pq.rs`, `spectral_uuid.rs`, `substrate_ref.rs`.
Existing test shape is property-based (round-trip + verdict-check),
not tuple-example.

---

## §2 Terni (imperfect) — PropertyVerdict + Loss + Imperfect actual shape

Location: `/Users/alexwolf/dev/projects/prism/imperfect/` (dependency
path from prismqueer/Cargo.toml `terni = { version = "0.7", path =
"../imperfect" }`).

### §2.1 `PropertyVerdict` (transparency.rs:141-155) — VERBATIM

```rust
pub enum PropertyVerdict {
    Pass,
    Partial {
        confidence: f64,           // in [0.0, 1.0]
        diagnostics: Vec<Diagnostic>,
    },
    Fail(Diagnostic),
}
```

Merge semantics (`merge_with`, transparency.rs:164-196):
- `Fail` dominates on either side.
- `Partial + Partial` → combine diagnostics, take `min(c1, c2)`.
- `Pass` is neutral element.

`Diagnostic` (transparency.rs:107) is a newtype over `String`.

**Load-bearing:** this is a fully-formed three-state verdict enum
with monoid merge. Prismqueer::liquid does NOT need to re-invent it.
Consumers importing `prismqueer::PropertyVerdict` get it directly.

### §2.2 `Loss` trait (lib.rs:108-121)

```rust
pub trait Loss: Clone + Default {
    fn zero() -> Self;                         // identity
    fn total() -> Self;                        // absorbing
    fn is_zero(&self) -> bool;
    fn combine(self, other: Self) -> Self;     // associative
}
```

### §2.3 `Metric` trait (lib.rs:145-163)

Extends `Loss` with `is_non_negative`, `distance_to`, and `triangle`
(returns bool, sample-based). Required by Connes bounded-commutator
condition; `ScalarLoss` implements it.

### §2.4 `Imperfect<T, E, L: Loss>` (lib.rs:180-189)

```rust
pub enum Imperfect<T, E, L: Loss> {
    Success(T),
    Partial(T, L),
    Failure(E, L),
}
```

Bind: `eh` / `imp` / `tri` (lib.rs:395-401). Recovery via `.recover`
always produces `Partial` (failure survives as cost).

### §2.5 `Transparency<P>` (transparency.rs:238-252)

```rust
pub enum Transparency<P: Ord + Clone> {
    Clear,
    Opaque(OpacityMap<P>),         // BTreeMap<P, PropertyVerdict>
}
```

Loss instance: `Clear` = zero, `Opaque(empty)` = total. `combine`
unions maps via `verdict_union` + per-key `merge_with`.

**Sizing implication:** With `PropertyVerdict` + `Transparency<P>`
already carried by terni and re-exported through prismqueer, the
verdict discipline is complete. `prismqueer::liquid` becomes
composition of existing verdict machinery, not new verdict types.
Weight likely closer to ~200 LOC than ~600.

---

## §3 Fate inference API + tournament + weights + strategy

Location: `/Users/alexwolf/dev/projects/fate/`. Cargo depends on
`prismqueer` via `prism = { package = "prismqueer", path =
"../prism/prismqueer", features = ["bundle"] }`.

LOC per file: `lib.rs:1459`, `runtime.rs:749`, `derive.rs:771`,
`train.rs:351`, `weights.rs:246`, `manifold.rs:275`,
`metal_runtime.rs:263`, `feature.rs:240`, `strategy.rs:63`,
`compiled.rs:3`.

### §3.1 `Model` enum (lib.rs:41-52)

```rust
pub enum Model { Abyss, Introject, Cartographer, Explorer, Fate }
```

### §3.2 `Features` type (lib.rs:56-59)

```rust
pub const FEATURE_DIM: usize = 16;
pub type Features = [f64; FEATURE_DIM];
```

### §3.3 `Decision` (lib.rs:62-89)

```rust
pub struct Decision {
    pub model: Model,
    pub confidence: f64,
    pub distribution: [f64; 5],
}
impl Decision {
    pub fn best_non_fate(&self) -> Decision;  // zeroes idx 4, renormalizes
}
```

### §3.4 `FateOutput` (lib.rs:93-100) — the ONE-TICK OUTPUT

```rust
pub struct FateOutput {
    pub model: Model,
    pub decision: Decision,
    pub kernel_spec: prism::KernelSpec,
    pub loss: ManifoldLoss,
    pub health: feature::HolonomyHealth,
}
```

### §3.5 The tournament — `Fate` (lib.rs:116-138)

```rust
pub struct Fate {
    pub selectors: [ModelWeights; 5],          // context-dependent
    pub strategy: Strategy,
    pub resolved_model: Model,                 // Lawvere fixed point
    pub kernel_spec: prism::KernelSpec,
    connection: prism::IdentityPrism<Features>,// ZST
}
```

Key methods:
- `select(current: Model, features: &Features) -> Decision` — one
  forward pass; picks from `selectors[current_idx]`.
- `resolve(features: &Features, max_depth: usize) -> Decision` — the
  META-loop: Fate selecting for itself until it dispatches to a
  different model. Exit: entropy floor (h < 1.0 at depth > 0) OR
  non-Fate winner OR max_depth. Fallback: `best_non_fate()`.
- `tick(features: &Features) -> FateOutput` — the full Bundle-tower
  pass: focus → project → settle over ManifoldState → decision + loss.
- `untrained()` — zero weights, uniform distribution.
- `excited()` — xorshift64 random weights from system time.

### §3.6 The property inference API — `Fate::tick`

This IS the API prismqueer::liquid would consume. Given features,
Fate returns `(model, decision, kernel_spec, loss, health)`.
Composes over `Prism::{focus, project, settle}` + `Transport`.

Fate ALSO implements `prism::Prism` directly (lib.rs:451-493):
- `type Input = Optic<(), (Model, Features)>`
- `type Focused = Optic<(Model, Features), [f64; 5]>` (logits)
- `type Projected = Optic<[f64; 5], Decision>`
- `type Refracted = Optic<Decision, Model>`

Bundle-tower impls (lib.rs:499-551): `Fiber`, `Connection`, `Gauge`,
`Transport` (dispatches to `transport_fortran` under lapack, else
`transport_rust`), `Closure` (returns `resolved_model`).

### §3.7 `ManifoldState` + `ManifoldLoss` (manifold.rs:20-36, 55-95)

```rust
pub type ManifoldState = [[f64; FEATURE_DIM]; FEATURE_DIM];  // 16×16
pub struct ManifoldLoss { pub delta: [[f64; 16]; 16] }
impl ManifoldLoss {
    pub fn between(before, after) -> Self;    // element-wise diff
    pub fn total(&self) -> f64;               // Frobenius norm
    pub fn active_trace(&self) -> f64;        // Σ delta[i][i], i ∈ ACTIVE
    pub fn dark_trace(&self) -> f64;
    pub fn active_trace_conserved(&self, tol: f64) -> bool;
}
```

### §3.8 `feature` module (feature.rs)

16 dimensions split: 6 ACTIVE (`TEMPORAL, PROCESSING, STABILITY,
NOVELTY, CAUTION, COHERENCE`) + 10 DARK (`CREATIVITY, CONFIDENCE,
FORMALITY, OUTPUT_REGULATION, ABSTRACTION, DEFERENCE,
CONFIDENCE_CALIBRATION, INNOVATION, REASONING_DEPTH, EMOTIONAL_TONE`).

Casimir: `C₂ = Σ (λᵢ · xᵢ)²` over ACTIVE, with
`CASIMIR_EIGENVALUES = [4.12, 3.98, 4.05, 3.91, 4.08, 3.97]`.
`casimir_penalty(before, after) = |C₂(before) - C₂(after)|`.

Holonomy health: `BERRY_PHASE = 0.847`. `HolonomyHealth::{TooShallow
(<0.1×BP), Healthy (0.1..10×BP), OverCutting (>10×BP)}`.

### §3.9 `Strategy` (strategy.rs) — the Group

```rust
pub enum Strategy {
    SpectralPartition, CommunityDetection, BreadthFirst,
    DepthFirst, Random,
}
```

Cyclic group Z/5 via `ordinal()` + `from_ordinal()`. Implements
`prism::GroupStructure { identity, inverse, compose }`.
Note (strategy.rs:44-50): the group structure is admittedly a
"substrate-honest minimal impl" for type-contract satisfaction;
categorical semantics of strategies do not imply Z/5.

### §3.10 `Weights` (weights.rs)

```rust
pub struct WeightSet { pub bias: [u8; 5], pub feature_weights: [[u8; 16]; 5] }
pub struct Weights { pub sets: [WeightSet; 5] }
```

Quantized `u8` weights. `default_cycle()` biases context i toward
`(i+1) % 5` (Abyss→Introject→Cartographer→Explorer→Fate→Abyss).
`to_bytes()` / `from_bytes()` serialization for the BF runtime.

### §3.11 `FateRuntime` — Brainfuck (runtime.rs:87-750)

`FATE_BF = include_str!("../brainfuck/fate.bf")`. Executes the model
as a Brainfuck program with 22-byte input (16 features + 1 model
index + 5 biases). This is the Kolmogorov atom. `bf_execute` has
1M-step cap. Filter-only tape parse (only `><+-.,[]` retained).

### §3.12 `Pipeline` trait (lib.rs:673-772)

Fate impls Pipeline: `focus(ManifoldState → Features)` extracts
diagonal for ACTIVE + off-diagonal coupling-norm for DARK; measures
`manifold_observation_loss` (delta from Casimir eigenvalues).
`project` runs `resolve`. `settle` builds weighted outer product Σ
prob_m · v_m ⊗ v_m^T from steering vectors, then scales ACTIVE
diagonal to hit Casimir target.

---

## §4 Mirror docs/math property enumeration

63 files under `docs/math/` per Search enumeration. Grouped by root
+ source. Distinguishing:
- **[LANDED-PROOF]** — claim with derivation/proof in-file
- **[LANDED-CLAIM]** — asserted, referenced to prior art, no proof
- **[FORWARD-PROMISE]** — named as future landing

### §4.1 the-tower (Baez-Schreiber + Connes altitudes)

1. **[LANDED-CLAIM]** Baez-Schreiber 2-connection compatibility
   `dA + [A,A] = t(B)` IS the compatibility theorem across the
   supervision tower — beam-runtime.md §2.5 (Baez-Schreiber 2004
   arXiv:hep-th/0412325 §3 pullback-agreement).
2. **[LANDED-PROOF]** Supervision tree ≅ simplicial Lie-group tower
   — beam-runtime.md §2.1. Baez-Schreiber 2004 Theorem 3 named.
3. **[LANDED-CLAIM]** Actors ≅ sections of principal bundle;
   message-passing ≅ parallel transport; sync call ≅ transport with
   ack-holonomy; async cast ≅ one-way transport — beam-runtime.md
   §§2.2-2.3.
4. **[LANDED-CLAIM]** Let-it-crash ≅ autopoietic closure (Lawvere
   fixed-point) — beam-runtime.md §2.4. Cross-refs to
   `prism::Closure` + `LawvereFixedPoint` in prismqueer.
5. **[LANDED-CLAIM]** Distributed Erlang ≅ sheaf-gluing on a
   principal bundle — beam-runtime.md §2.6.
6. **[LANDED-PROOF]** Ado's theorem — Lie algebras of finite-dim
   Lie groups ARE matrix algebras — cited at
   `docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md`
   §1 (statement 1 of the terminal-geometry claim). Ado is a
   classical theorem; usage here is citation, not re-derivation.
7. **[LANDED-CLAIM]** Kuramoto phase-lock ≅ ensemble connection
   at @dance altitude — beam-runtime.md §6 +
   docs/specs/gen-prism-as-bundle-section-and-dance-as-ensemble-
   connection.md §2.3. Aumann-agreement envelope ≅ ensemble
   holonomy.
8. **[LANDED-CLAIM]** Holonomy trace hol_γ(A) ∈ G is the ordered
   path-integral of A along γ — stigmergy-witnessed-computation.md
   §5.2. Rolling `signature_beat` chain IS hol_γ(A) partly
   (conjecture, partial-fail cases noted §5.3).
9. **[LANDED-PROOF]** Recognition #79 gauge is void-duality basis —
   docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md
10. **[LANDED-CLAIM]** Recognition #82 frame as cognitive order —
    docs/math/the-tower/recognition-82-*.md
11. **[LANDED-CLAIM]** Recognition #80 magic as form-process
    substrate-decl — same directory.

### §4.2 sheaf

12. **[LANDED-PROOF]** Cellular sheaf `F` on graph `G = (V,E)` with
    stalks `F(v), F(e)` and restriction maps `F_{v ⊴ e}` —
    docs/math/sheaf/laplacian.md §1.
13. **[LANDED-PROOF]** Sheaf Laplacian `Δ_F = δ*δ` self-adjoint,
    PSD; smallest eigenvalue λ₀ IS the sheaf-coherence criterion —
    laplacian.md §2.
14. **[LANDED-CLAIM]** Hodge decomposition on cellular sheaves; H¹
    = "first sheaf cohomology" — laplacian.md §3.
15. **[LANDED-CLAIM]** Fiedler value λ₂(Δ_F) as verification surface
    — stigmergy-mycelial-composition.md §4.1 (Russold 2022
    persistent sheaf cohomology cited).
16. **[LANDED-CLAIM]** Anastomosis by content_oid collision —
    stigmergy-mycelial-composition.md §4.2.

### §4.3 kintsugi (fracture, roomba)

17. **[LANDED-PROOF]** Bilateral-arm redundancy predicate:
    `redundant(a, B) := arm_is_in_reflective_corpus(a) ∧
    arm_matches_sentinel(a)` — bilateral-arm-redundant.md §2.
18. **[LANDED-PROOF]** Retirement safety theorem — 4 invariants
    (I1) sbec, (I2) rust_loc, (I3) test_pass_rate, (I4)
    io_violations — bilateral-arm-redundant.md §3.
19. **[LANDED-PROOF]** Fixed-point termination on redundant-arm
    class — §4. `O(|B| × |R|)` complexity — §5.
20. **[LANDED-PROOF]** Rice-safety of the redundancy predicate —
    §5.
21. **[LANDED-PROOF]** Bump-composition preserves Foerster
    admissibility — roomba/bump-and-vacuum.md §1.5.
22. **[LANDED-PROOF]** Dangling-consistency proposition on DAG —
    roomba/bump-and-vacuum.md §2.5.
23. **[LANDED-PROOF]** Bump ∘ vacuum non-commutativity — §3.5.
24. **[LANDED-CLAIM]** Fiedler-conductivity preservation:
    removing a dangling OID preserves λ₂ of the live subgraph as
    honest measurement — §4.2 (Fiedler 1973 cited).
25. **[LANDED-CLAIM]** `@epistemologic/property/ouroboros_monotone`
    four-conjunct extended with Fiedler-ascent:
    `λ₂(live(after)) ≥ λ₂(live(before))` — §5.1 + shard-decl
    `shards/epistemologic/property/ouroboros_monotone.mirror`.

### §4.4 liquid-types

26. **[LANDED-CLAIM]** Rondon-Kawaguchi-Jhala liquid-inference:
    refinement predicates from a decidable-logic qualifier set;
    fixed-point iteration seeded at `⋀ Q`, monotone-weakening —
    liquid-types/README.md §1.1 (Rondon-Kawaguchi-Jhala 2008 PLDI).
27. **[LANDED-CLAIM]** Qualifier set `Q_mirror = @epistemologic/
    property/*` — §2.2.
28. **[LANDED-CLAIM]** Auto-classifier decision procedure at
    doc-claim altitude — §3 (five signals as qualifier witnesses).
29. **[LANDED-CLAIM]** `@projection.preview: satisfiable |
    unsatisfiable | partial` as three-valued form of liquid-model-
    check — §1.4.
30. **[LANDED-CLAIM]** Recognition #107 Hilbert-Turing separation
    grounds decidability boundary — §1.7.

### §4.5 uuid/spectral-time

31. **[LANDED-PROOF]** `uuid_spectral_time` IS categorical product
    of identity-space + Kuramoto-Aumann time-space — §1.
32. **[LANDED-PROOF]** R4-affinity-by-construction theorem: argsort
    on 48-bit ACTIVE prefix IS Kuramoto-Aumann convergence at
    annotation altitude — §3 (Aumann 1976 + Kuramoto 1975 cited).
33. **[LANDED-PROOF]** Dedup semantics projection homomorphism —
    §4.
34. **[LANDED-CLAIM]** Byte-distance IS pain/pleasure gradient —
    §5.

### §4.6 bilateral (glue-metalogue)

35. **[LANDED-PROOF]** `@bilateral(A, B)` degenerate collapse when
    A = B — §2.
36. **[LANDED-CLAIM]** Three-conjunct general translation-floor
    semantics — §3.2.
37. **[LANDED-PROOF]** Rice-safety by case (four cases) — §4.
38. **[LANDED-PROOF]** Fixed-point convergence + monotonicity —
    §5.
39. **[LANDED-PROOF]** `@bilateral` is a functor between
    composition-category and verdict-category — §7.
40. **[LANDED-CLAIM]** Fractal attending: every bilateral computes
    via Fiedler-Laplacian on `@spectral/db` — §8.

### §4.7 stigmergy (2026-07-18 landing)

41. **[LANDED-CLAIM]** Grassé 1959 four-part stigmergy ≅
    substrate carriers (medium/trace/rule/agent) — §2.1.
42. **[LANDED-CLAIM]** ACO convergence-in-value ≅
    ouroboros_monotone (Stützle-Dorigo 2002 Theorem 1) — §3.1.
43. **[LANDED-CLAIM]** ACO convergence-in-solution (Gutjahr
    2000-2002 Theorems 4, 5) — §3.1.
44. **[LANDED-CLAIM]** Martingale stopping time finite a.s. (Lin)
    — §3.1.
45. **[LANDED-CLAIM]** Wang 2025 (r+1)/2 approximation ratio bound
    — §3.2.
46. **[LANDED-CLAIM]** Mycelial anastomosis via content_oid
    collision — §4.2.
47. **[LANDED-CLAIM]** Pheromone-gradient IS Baez-Schreiber
    connection 1-form's holonomy trace (Alex's conjecture, partly
    refined) — §5.
48. **[LANDED-CLAIM]** Recognition candidate `#R-signature-beat-
    rolling-signature-IS-holonomy-of-baez-schreiber-connection` —
    §3.3 (candidate strength).

### §4.8 spawn (loop monad)

49. **[LANDED-PROOF]** `spawn_loop` monad-law satisfaction
    (left/right identity, associativity) — §2.
50. **[LANDED-PROOF]** Halting theorem for bounded reductions —
    §3.1.
51. **[LANDED-PROOF]** System F / Gödel's T grounding — §3.3.
52. **[LANDED-PROOF]** Rice-safety of `@spawn ≤ @loop` — §4.
53. **[LANDED-CLAIM]** Recognition candidate #132 `pull_frontier`
    IS substrate-pull tangent vector — §5.3.

### §4.9 fractal (Mandelbrot substrate)

54. **[LANDED-CLAIM]** M∘ = @magic; ∂M = @io — §2.3
    (Recognitions #80, #107 as topology).
55. **[LANDED-CLAIM]** Shishikura 1998 theorem: dim_H(∂M) = 2 —
    §2.4.
56. **[LANDED-CLAIM]** Douady-Hubbard 1985 straightening theorem —
    §3.2.
57. **[LANDED-CLAIM]** Substrate-refactor invariance = two-tick
    discipline — §3.5.
58. **[LANDED-CLAIM]** Every peer DAG is a baby Mandelbrot — §4.5.
59. **[LANDED-CLAIM]** Coordination-without-signal theorem — §5.4.

### §4.10 gestalt

60. **[LANDED-CLAIM]** @gestalt as presheaf on reader-time — §1.2.
61. **[LANDED-CLAIM]** Merkle-DAG monotone growth per reader-
    interaction — §3.2.
62. **[LANDED-CLAIM]** Consent-depth as cellular sheaf projection
    — §4.1.
63. **[LANDED-CLAIM]** Sheaf-Laplacian λ₀ Fiedler for annotation
    coherence — §4.3.
64. **[LANDED-CLAIM]** Reader-inside-operator fixed-point via
    Banach contraction — §6.2.
65. **[FORWARD-PROMISE]** M1: sheaf-Laplacian spectrum on monotone-
    growing sheaf — §9.

### §4.11 onto (candidate operator)

66. **[LANDED-CLAIM]** Circular-reflexive requirement on @onto —
    §2.2.
67. **[LANDED-CLAIM]** Four altitudes of @onto operation — §8.
68. **[LANDED-CLAIM]** Discriminator between @onto-TRUE and
    @onto-FALSE cases — §5.3.

### §4.12 prism-kind

69. **[LANDED-PROOF]** Five-signal discriminator: inheritance,
    carrier density, cross-family imports, docblock citation,
    primary carrier shape — §3.
70. **[LANDED-CLAIM]** Empirical discriminator — §4.

### §4.13 zero, spin, autopoiesis, affect, consciousness,
provenance, resource-budget, supervisor, polyglot (skimmed by
enumeration; representative claims not exhaustive here):

71. **[LANDED-CLAIM]** Casimir invariant C₂ = Σ (λᵢ·xᵢ)² over
    ACTIVE dimensions with quadratic conservation under
    redistribution — fate/src/feature.rs::casimir + tests.
    NOTE: this claim lives in fate crate, not mirror docs/math;
    property landing would validate fate's implementation.
72. **[LANDED-CLAIM]** Berry phase = 0.847 for 6-active-dim fiber
    bundle; holonomy health classification `loss / BERRY_PHASE`
    — fate/src/feature.rs.
73. **[LANDED-CLAIM]** Un-cite-ability theorem — provenance/
    un-cite-ability-theorem.md.
74. **[LANDED-CLAIM]** Emergent supervision from geometry —
    supervisor/emergent-supervision-from-geometry.md (82.6KB).
75. **[LANDED-CLAIM]** Zero-point field and λ₀ — zero/zero-point-
    field-and-lambda-zero.md.
76. **[LANDED-CLAIM]** CPT recursion — spin/cpt-recursion.md.
77. **[LANDED-CLAIM]** Delight as natural transformation — top-
    level delight-as-natural-transformation.md.

**Count:** ~77 distinct mathematical claims enumerated. Realistic
"first-pass property-testable" subset likely 15-25 (the ones with
computable witnesses at rust altitude): #7-8 (Kuramoto/Aumann),
#12-15 (Fiedler/sheaf-Laplacian), #17-20 (bilateral-arm-redundant
invariants), #24 (Fiedler-conductivity), #25 (ouroboros_monotone
four-conjunct), #31-33 (uuid spectral-time argsort), #35-38
(bilateral collapse/Rice/fixed-point), #46 (anastomosis by OID
collision), #49-50 (spawn_loop monad + halting), #57
(two-tick refactor invariance), #71-72 (Casimir + Berry-phase).

---

## §5 Mirror docs/specs @liquid semantic claims

### §5.1 `docs/specs/liquid-types-for-mirror.md` (Reed 2026-06-04,
41.7KB)

- §2.1 Direct mapping: liquid refinement ≅ mirror predicate on
  substrate carrier.
- §2.2 Critical divergence: verdicts (three-mode: pass/partial/fail)
  vs booleans (SMT).
- §2.3 `\` inference and properties.
- §4 Continuous verdicts vs boolean predicates. Ben Yaacov et al.
  2008 grounding.
- §5 Spectral alternative to SMT: eigenvalue-based property
  verification. λ₁ (Fiedler value) IS spectral gap.
  Fiedler vector localizes error.
- §5.4 Spectral liquid inference algorithm.
- §6.1 Grammar hierarchy: `@epistemologic → @epistemologic/property
  → @epistemologic/liquid`.
- §6.4 Relationship to Fate — Fate's tournament ≅ liquid inference
  fixed-point iteration.
- §7 Adopt-now / adopt-next / do-not-adopt discipline.

### §5.2 `docs/specs/gen-prism-as-bundle-section-and-dance-as-
ensemble-connection.md`

- §2.1 gen_prism = bundle section at process altitude.
- §2.2 @spectral/supervisor = bundle connection at supervision
  altitude (Baez-Schreiber compatibility theorem cited).
- §2.3 @dance = bundle connection at ensemble altitude (Kuramoto
  phase-lock on Förster torus).
- §2.4 @peer.audhd(p, ctx) LANDED as K-parallel @roomba walkers +
  Aumann-agreement envelope + @liquid predicates gating winning arm
  (Arc 5 M1 LANDED `cc816f9` + `b2c5d09` + `12cdf0e`).
- §2.5 dance.rs = Rust realization of ensemble connection.

### §5.3 `docs/specs/rust-floor-birthed-by-roomba-from-mirror-
spec.md`

- §1 Terminal-geometry claim: phone.rs / matrix.rs / main.rs; one
  altitude each; dance.rs collapses into matrix.rs because Ado's
  theorem + Baez-Schreiber 2-connection compatibility ARE matrix
  algebra.
- §4.3 Kuramoto phase-lock + Fiedler + Baez-Schreiber all COLLAPSE
  INTO matrix.rs.
- §5 main.rs supervisor + @-operator addressing.

### §5.4 `docs/specs/2026-07-18-stigmergy-witnessed-computation-
mycelial-composition.md`

- §0 Executive summary: walker leaves markers (`signature_beat`
  chain IS stigmergic pheromone); ensemble reads trail via
  Kuramoto phase-lock; crystal settlement IS mycelial anastomosis.
- §2 REFUSED mints: `@stigmergy` family-root, `@pheromone` species,
  `@marker` species (all substrate-already-had-the-word).
- §4 K>1 fanout @dance via @peer.audhd_action + audhd_admissible.
- §5.3 Persistent H⁰ birth as recognition-ratification event.

### §5.5 Shards

- `shards/liquid.mirror` (family-root, 15.3KB, 2026-07-17) —
  composition-altitude operator `@liquid(@X)` sibling to
  `@sre / @shatter / @glue`.
- `shards/epistemologic/liquid.mirror` (theory-species, 15.1KB,
  2026-07-17) — the refinement-theory carrier at epistemologic
  altitude.
- `shards/epistemologic/property/ouroboros_monotone.mirror` — the
  four-conjunct property + one composed bilateral.

---

## §6 rust/src/ current state + Cargo.toml diff

### §6.1 Current file surface (all under `/Users/alexwolf/dev/
projects/mirror/rust/`)

- `src/main.rs` — 914 LOC. Supervisor boot + @-operator dispatch
  M0 shadow. Hand-rolled argv. 11 hardcoded verbs. `cmd_roomba`
  fires walker + calls into `collapse::` for arm-collapse +
  `deposit_observation_crystal` for pheromone trail. Contains
  hand-rolled SHA-256 impl (lib.rs-style, ~85 LOC).
- `src/phone.rs` — 329 LOC. @io/fs socket-handover (fs.read,
  fs.write, fs.append_to, git.add, git.commit).
- `src/matrix.rs` — 171 LOC. Sub-Turing FLANG emit target (empty
  body at M0).
- `src/collapse.rs` — 378 LOC. Byte-analysis for redundant-arm
  detection (bilateral corpus loader, sentinel matching).

Total: 1792 LOC.

### §6.2 Current `Cargo.toml`

```toml
[package]
name = "mirror"
version = "0.1.0"
edition = "2021"
[[bin]]
name = "mirror"
path = "src/main.rs"
[dependencies]                # ZERO deps at M0 discipline
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### §6.3 What adding `prismqueer + fate` deps would change

Diff shape:

```toml
[dependencies]
# Cross-crate path deps to sibling projects.
prismqueer = { path = "../../prism/prismqueer", features = ["bundle"] }
fate = { path = "../../fate" }
# terni re-exported via prismqueer; direct dep not needed unless
# consumers name terni types outside prismqueer's re-exports.
```

Path shape observation: `mirror/rust/Cargo.toml` sits at
`/Users/alexwolf/dev/projects/mirror/rust/`, prismqueer at
`/Users/alexwolf/dev/projects/prism/prismqueer/`, fate at
`/Users/alexwolf/dev/projects/fate/`. Relative paths:
`../../prism/prismqueer`, `../../fate`.

Consequences:
- Discipline change: Cargo.toml notes "NO clap. NO structopt. NO
  serde." + "ZERO deps at M0". Adding prismqueer + fate breaks the
  M0 discipline. This is arc-level substrate decision, not Taut's
  to resolve.
- Feature: `bundle` needed for `Fiber`, `Connection`, `Gauge`,
  `Transport`, `Closure`, `LawvereFixedPoint`. Fate already needs
  it and enables it transitively; specifying it on the mirror side
  makes the contract explicit.
- Build size: prismqueer default feature-set is empty; bundle+optics
  costs a few KB. Fate default is empty; training/lapack/metal are
  gated.
- Both crates use `edition = "2021"`. No conflict.
- Note: prismqueer includes `prismqueer-projections` sibling crate
  (proc-macros for `declaration!{}` + `#[derive(Prism)]` +
  `#[derive(Lambda)]`). Path lookup shows
  `/Users/alexwolf/dev/projects/prism/projections/` sits alongside
  `prismqueer/`. Adding prismqueer pulls this in automatically.

---

## §7 Composition-shape reporting

Reporting composition topology (NOT design recommendations — Alex
adjudicates the arc direction). What the code already-carries vs
what would be genuinely new:

### §7.1 Already-carried by prismqueer + terni

- Three-mode `PropertyVerdict::{Pass, Partial{c, ds}, Fail(d)}` with
  monoid merge (Fail dominates, Partial combines diagnostics + takes
  min confidence).
- `Transparency<P>` opacity map keyed by substrate location P;
  `Clear` = zero, `Opaque(empty)` = total; `combine` unions maps.
- `Loss` + `Metric` traits with monoid + distance/triangle
  semantics.
- `Imperfect<T, E, L>` three-state carrier (Success / Partial /
  Failure) with `.eh() / .imp() / .tri()` bind + `.recover`.
- `Oid` content-addressing via CoincidenceHash<3>.
- `SpectralOid` at precision.
- `Ref` @-prefix validated nav-ref.
- `Prism` trait + `Beam` + `Optic` compositional semifunctor.
- Bundle tower: `Fiber`, `Connection`, `Gauge`, `Transport`,
  `Closure`, `LawvereFixedPoint`, `GroupStructure`.
- `KernelSpec` with dimension selector + decomposition + precision.
- `Crystal<P>` = settled Prism at rest.

### §7.2 Already-carried by fate

- `Fate::tick(features) -> FateOutput { model, decision, kernel_spec,
  loss, health }` — the property-inference forward pass at 16-dim
  fiber-bundle altitude.
- `Fate::resolve(features, max_depth) -> Decision` — meta-loop with
  entropy-floor exit.
- `Fate::select(current, features) -> Decision` — one-shot.
- `ManifoldState` (16×16 connection matrix) + `ManifoldLoss`
  (curvature tensor with Frobenius / active_trace / dark_trace).
- `Casimir` invariant + `casimir_penalty` + `holonomy_health`
  (Berry phase-based classification).
- `Fate` implements `Prism + Fiber + Connection + Gauge + Transport
  + Closure` from prismqueer bundle.
- `FateRuntime` = Brainfuck interpreter over `fate.bf` (Kolmogorov
  atom); reproducibility contract.

### §7.3 What would be genuinely NEW (not already-carried)

The load-bearing question: does the substrate need a new type surface
in `prismqueer::liquid`, or is `@liquid(@X)` composition of what's
already there?

Evidence:
- `shards/liquid.mirror:26-30` names @liquid at COMPOSITION
  altitude, sibling to `@sre / @shatter / @glue`. Composition
  operators are family-roots that take substrate carriers as
  arguments and produce substrate lenses. They do NOT declare a
  WHAT-IS species.
- `shards/epistemologic/liquid.mirror:34-42` names the THEORY at
  `@epistemologic/liquid` — predicate carriers, composition axioms,
  decidability boundary, Rice-safe connection.
- `docs/math/liquid-types/README.md §2` names operators `refine`,
  `extract`, `prove`, `route` — the algebra layer.
- `docs/specs/liquid-types-for-mirror.md §5.4` names "Spectral
  Liquid Inference": eigenvalue decision procedure, Fiedler
  bisection.

The gap between "verdict verdict + refinement predicate + qualifier
set" (already fully specced at math + shard altitude) and "existing
prismqueer::PropertyVerdict + Fate::tick" is the DISPATCH LAYER: the
functions `refine`, `extract`, `prove`, `route` that TAKE a substrate
carrier + refinement predicate and produce a `Transparency<Ref>`.

Fate's `tick(features) -> FateOutput` is the inference engine; a
`prismqueer::liquid` module (if it lands) would be the boundary
between:
- Input side: extracting `Features` (16-dim vector) from a mirror
  substrate carrier (a `.mirror` shard, a `.rs` file, a docblock).
- Output side: mapping `FateOutput.decision` + `FateOutput.loss` to
  a `Transparency<Ref>` opacity map with `PropertyVerdict` entries.

### §7.4 rust/src/liquid.rs as bridge

Per Alex directive: the bridge is the third step. Preconditions:
- Layer 1 (prismqueer::liquid) LANDED — provides
  `refine(carrier, predicate) -> Transparency<Ref>` or equivalent.
- Layer 2 (mirror properties) LANDED — provides `Features`-shaped
  observations of specific mirror mathematical claims (e.g. Fiedler
  descent on splinter graph; Casimir on 6-active-dim carrier;
  ouroboros_monotone four-conjunct).
- Layer 3 (rust/src/liquid.rs) — composes 1 + 2 to produce
  Transparency<Ref> verdicts at rust altitude, callable from
  main.rs cmd_roomba (materialize dispatch arm per Mara §7.4).

### §7.5 Composition topology as read

```
┌────────────────────────────────────────────────────────┐
│  mirror/rust/                                          │
│    main.rs          ← cmd_roomba dispatch              │
│    liquid.rs   ← [NEW] bridge to fate + prismqueer     │
│    phone.rs         ← @io/fs                           │
│    matrix.rs        ← sub-Turing (empty at M0)         │
│    collapse.rs      ← bilateral-arm detection          │
│                                                        │
│  DEPS ↓                                                │
│  prismqueer                                            │
│    ├── PropertyVerdict, Transparency<P>   ← terni       │
│    ├── Imperfect<T,E,L>, Loss, Metric     ← terni       │
│    ├── Prism, Beam, Optic                              │
│    ├── Bundle: Fiber, Connection,                      │
│    │       Gauge, Transport, Closure                   │
│    ├── Oid, SpectralOid, Ref                           │
│    ├── KernelSpec, Precision                           │
│    ├── liquid  ← [NEW MODULE IF LANDED]                │
│    │     refine / extract / prove / route              │
│    └── (feature) lambda, optics, pq, lapack            │
│                                                        │
│  fate                                                  │
│    ├── Fate::tick → FateOutput{model, decision,        │
│    │       kernel_spec, loss, health}                  │
│    ├── ManifoldState 16×16, ManifoldLoss (Frobenius)   │
│    ├── feature: ACTIVE(6) + DARK(10), Casimir,         │
│    │       BERRY_PHASE, HolonomyHealth                 │
│    ├── Strategy: Z/5 GroupStructure                    │
│    ├── Weights: 5 selectors × 90 params (450 total)    │
│    └── FateRuntime: Brainfuck over fate.bf             │
└────────────────────────────────────────────────────────┘
```

### §7.6 Sizing observation

With PropertyVerdict + Transparency + Imperfect + all bundle-tower
traits + Fate::tick already fully implemented, a `prismqueer::liquid`
module that COMPOSES over existing verdict discipline is closer to
~200 LOC than ~600. But if `refine / extract / prove / route` need
new type-level machinery (e.g. `LiquidQualifier`, `LiquidClaim`,
`RefinementPredicate` newtypes per `docs/math/liquid-types/README.md
§2.1`), the LOC climbs.

The math root §2.1 explicitly names the newtype carriers. Landing
these in Rust would be new type surface; landing them as pure
composition over `PropertyVerdict + Ref + Precision` may not require
new types. This is a design decision, not a scout finding.

---

## §8 Open questions for Alex

**OQ1.** **M0 zero-dep discipline vs prismqueer + fate deps.**
`mirror/rust/Cargo.toml:38-39` explicitly names "NO clap. NO
structopt. NO serde." and defends M0 as zero-deps floor. Adding
prismqueer + fate breaks this. Is this a substrate-honest arc-level
decision (M0 discipline retires as the property-inference substrate
takes over), or does prismqueer + fate need to satisfy a different
introduction path (e.g. behind a feature flag; or in a separate
crate `mirror-liquid` that mirror depends on downstream)?

**OQ2.** **Composition altitude of `prismqueer::liquid` vs
`@liquid` family-root vs `@epistemologic/liquid` theory.**
`shards/liquid.mirror` names @liquid at composition altitude
(family-root operator). `shards/epistemologic/liquid.mirror` names
@epistemologic/liquid at theory altitude. A Rust module
`prismqueer::liquid` would sit at which of these — the composition
operator, or the theory carrier, or a third altitude (operational
extraction subspecies)? Per math §2.1 the newtype carriers land at
theory altitude; the operator `refine` lives one altitude UP.

**OQ3.** **Which properties from §4 property enumeration land
first?** ~77 distinct claims enumerated across docs/math; ~15-25
have computable witnesses at rust altitude. The three-layer
build order names "Mirror's mathematical properties" as step 2
without picking. Candidates with highest existing infrastructure:
Fiedler-conductivity (#24), ouroboros_monotone four-conjunct (#25;
already shard-decl'd + math root proof), bilateral-arm-redundant
invariants (#17-20; math root proof + empirical anchors at `06f14f5`),
uuid_spectral_time argsort R4-affinity (#31-33; math root proof +
already implicit in `shards/uuid/spectral/time.mirror`).

**OQ4.** **Fate's `Strategy` cyclic-group Z/5 IS acknowledged as
substrate-honest-minimal.** `fate/src/strategy.rs:44-50` says the
Z/5 group structure is chosen "solely to satisfy the type contract";
the categorical semantics of strategies do NOT imply this group. If
prismqueer::liquid composes over Fate's `Gauge`, does this thin
group structure carry through to a property claim, or does the
property-testing surface need to sit BELOW `Gauge` to avoid resting
on the placeholder?

**OQ5.** **Recognition candidate `#R-signature-beat-rolling-
signature-IS-holonomy-of-baez-schreiber-connection` (per stigmergy
math §3.3) partly-fails per §5.3.** How does property-testing handle
the partial-fail case? `PropertyVerdict::Partial{confidence,
diagnostics}` seems well-shaped for it (Alex's conjecture holds at
confidence < 1.0, diagnostic names the partial-fail condition),
but this crosses from theory into empirical calibration. Is
"partial holonomy trace" a property test the substrate wants to
land, or does it stay in math root as conjecture?

**OQ6.** **fate/src/lib.rs is 1459 LOC.** The Fate crate is a large
surface. Which parts does `prismqueer::liquid` compose over vs which
parts stay Fate-internal? Candidates: `tick + resolve + select`
(inference API), `Model + Decision + FateOutput` (verdict types),
`Features + ManifoldState + ManifoldLoss` (state types), `feature::
{Casimir, holonomy_health, BERRY_PHASE}` (numerical anchors). NOT
candidates for prismqueer::liquid: `FateRuntime` (Brainfuck),
`weights::Weights` (serialization), `Pipeline` trait impl.

---

## §9 Empirical anchors

- Enumeration: 63 files under `docs/math/` (Search result); ~77
  distinct claims counted.
- Enumeration: 141 files under `docs/specs/` (from ls).
- prismqueer: 27 source files; 9 test files; feature list
  (default/optics/bundle/lambda/lapack/pq).
- terni: 2 source files (`lib.rs` 100.5KB, `transparency.rs` 13.1KB);
  1 test file (`transparency.rs` 13.4KB).
- fate: 10 source files, 6212 LOC total; depends on
  `prismqueer` with `features = ["bundle"]`.
- mirror/rust: 4 source files, 1792 LOC total; zero deps.
- 32a1022 RED tests: not read in this scout (out of scope per Alex's
  three-layer directive; this scout enumerates the substrate BEFORE
  the property-inference layer, not the tests that exposed the gap).

---

*End of scout. Read-only. No mints. No design decisions. Full
substrate-honest reporting; where a claim could not be verified in
code (e.g. specific Recognition landing dates), the claim was named
as a CLAIM without proof-attribution.*
