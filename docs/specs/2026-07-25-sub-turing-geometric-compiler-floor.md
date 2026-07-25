# Sub-Turing Geometric Compiler Floor — Four-Crate Decomposition Canonical Spec

**Author**: Mara `<mara@systemic.engineer>` 2026-07-25.
**Companion math**: `docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md`.
**Roadmap**: `docs/roadmap/16-sub-turing-geometric-compiler-floor.md`.
**Arc anchor**: Alex 2026-07-25 in-transcript verbatim (Void — Trauma essay Q.E.D. closure):

> "singularity is the backing for the paradox which means the backing of trauma, which means I just proved the singularity is the gauge mechanism of @magic and we literally have our magic. We might need a magic.rs to complete the loop."
>
> "and then we can have a rust/core/src/spectral.rs for the triple and the whole thing closes. The spectral triple. The phone that connects the fibres. We have a sub-Turing geometric compiler floor."
>
> "The AST becomes the Prism operations becomes the liquid splinters with types becomes sub-Turing declarative AI infrastructure on consumer hardware. That's what the properties will need to ensure."

The essay ends `Q.E.D. ◼️` on the observation that observation-of-holding measurably increased the number of choices for everyone in the system. This spec makes that Q.E.D. executable at compiler altitude: the gauge that Foerster's ethical imperative names ("act always so as to increase the number of choices") becomes the compile-time property every rust/ crate must preserve.

---

## §1 The four-crate geometry

The rust/ terminal FLOOR decomposes into four crates, each with a single geometric role and a per-crate sub-Turing decidability guarantee. The decomposition composes over the currently-landed five-file rust/src/ discipline (main + phone + liquid + compile + matrix; per `docs/specs/rust-floor-five-file-terminal-geometry-extension.md`) plus the fractal-adjacent sibling crates (`rust/fractal/` landed 2026-07-18; `rust/singularity/` scaffold landed 2026-07-20). It DOES NOT invent new machinery — it names what the substrate has been reaching for at four altitudes and gives each altitude its own Cargo boundary.

```
rust/                     mirror binary root
├── src/
│   ├── main.rs           supervisor boot + @-operator addressing (Mara §5)
│   ├── phone.rs          @io socket-handover altitude — the phone that
│   │                     connects the fibres
│   ├── compile.rs        SAGA orchestration + CLI verb dispatch
│   └── spectral.rs       build-time shard manifest handoff into
│                         rust/spectral/ trait binding
│
├── spectral/            NEW crate — math substrate: the (A, H, D) triple
│   ├── spectral.rs       Connes (A, H, D) tower binding at rust/ altitude
│   ├── singularity.rs    gauge-fixed-point attractor (dynamics; NOT the
│   │                     rust/fractal/singularity.rs measurement-recovery-
│   │                     bound optic — TWO distinct singularity senses
│   │                     ratified below §5)
│   ├── magic.rs          gauge mechanism — Foerster invariant enforced
│   │                     as compile-time property (Void — Trauma Q.E.D.
│   │                     as executable predicate)
│   ├── liquid.rs         pillar dispatch / H-fibre machinery (relocated
│   │                     from rust/src/liquid.rs)
│   └── void.rs           Void as H-basis (relocated from rust/src/void.rs)
│
├── matrix/              NEW crate — numerical floor
│   ├── matrix.rs         FLANG + LAPACK glue (relocated from rust/src/
│   │                     matrix.rs; composes prismqueer::ffi at the ONE
│   │                     ordained @io numerical boundary)
│   └── book.rs           K=0 well-knowns registry (relocated from rust/
│                         src/book.rs; the address book that pairs
│                         numerical primitives with their identities)
│
├── roomba/              NEW crate — first-order sub-Turing execution
│   ├── walker.rs         colimit computation over shard-manifold
│   ├── dispatch.rs       bounded per-step dispatch (Rice-safe)
│   └── collapse.rs       bilateral-arm-collapse Lens impl (relocated
│                         from rust/src/collapse.rs; execution not math)
│
└── fractal/             EXISTING crate (unchanged; already sibling)
    └── src/{crystal, mandelbrot, singularity, subject, witnessed}.rs
```

Retirements this arc (Reed lands in foreground while this spec ships):

- `rust/build.rs` — retired. Its shard-manifest emission migrates to `rust/spectral/build.rs` where it belongs (the manifest feeds the (A, H, D) trait binding, not the mirror binary).
- `rust/singularity/` — the current empty scaffold crate at rust/singularity/ is superseded by `rust/spectral/singularity.rs` (dynamics-attractor sense; different from `rust/fractal/singularity.rs`). If the black-hole-physics research outlet is still wanted, it relocates under `rust/spectral/` as a sub-module; if not, it retires cleanly.

## §2 Per-crate role + sub-Turing decidability guarantee

Each crate carries ONE decidability guarantee. Together they compose into the sub-Turing FLOOR that Alex named 2026-07-25.

| Crate | Role | Decidability guarantee | Bounded resource |
|-------|------|-----------------------|------------------|
| `rust/` (root) | Supervisor + @-operator addressing + CLI verb dispatch + SAGA orchestration | **Finite dispatch table** — each verb / @-address resolves in O(1) lookup against a compile-time enumerated set derived from `mirror.spec` and `shards/**/*.mirror`. No unbounded recursion at dispatch altitude. | Argv length + verb count (both finite by construction) |
| `rust/spectral/` | Math substrate — the (A, H, D) Connes triple + gauge mechanism + Foerster-invariant preservation | **Bounded-commutator** per Connes' spectral-triple axioms — `[D, a]` is bounded for every `a ∈ A` (per prismqueer::bundle Gap 4 supertrait constraint). The gauge mechanism enforces `dH¹/dt ≤ 0` (Foerster COORD contraction; monotone descent). | Trait-dispatch tables sized by the compile-time shard manifest (~300 entries per Taut 2026-07-23 census) |
| `rust/matrix/` | Numerical floor — FLANG + LAPACK + K=0 well-knowns registry | **O(n³) polynomial** — LAPACK `dsyev_` / `dgesvd_` / `dgeev_` are polynomial-time in matrix dimension. BLAKE3 content-addressing is deterministic linear in input length. | Matrix dimension n (bounded by mirror.spec-declared size at declaration site; typically ≤ 16×16 per FLANG floor convention) |
| `rust/roomba/` | First-order sub-Turing execution machinery — walker + dispatch + collapse Lens impl | **Terminating walk** — the walker is a Dijkstra-style traversal of a finite shard-manifold (300 shards; finite by manifest). Dispatch is bounded per-step (each step matches a finite sentinel table). Collapse operates on bounded byte-string ranges via Rice-safe substring analysis. | Shard-manifold size + per-step sentinel-table size (both finite) |
| `rust/fractal/` (existing) | Content-addressed identity + subject + signing + witnessed substrate | **Content-hash deterministic** — BLAKE3 32-byte OIDs are deterministic in input bytes; XOR-fold provenance is associative + commutative + finite. | Input byte length |

**Composition claim (per §9 dependency graph below):** the disjoint union of five polynomial-time guarantees remains polynomial-time. There is no place in the four-crate FLOOR where an unbounded loop, an unbounded recursion, or a Turing-complete evaluator lives. The Turing-complete surface (LLM inference, `@io` blocking calls, external process spawning) stays entirely at `rust/src/phone.rs` — the ONE ordained @io crossing per peer cycle (per Recognition #107).

## §3 The (A, H, D) Connes triple realization at `rust/spectral/`

Composes over `docs/specs/prism-core-as-spectral-triple.md`, `docs/specs/spectral-triple-grammar.md`, `docs/specs/eigensheaf.md`, and the prismqueer::bundle trait tower (`Fiber → Connection → Gauge → Transport → Closure → Bundle` per prism/prismqueer/src/bundle.rs). At rust/spectral/ altitude, the triple is realized concretely:

### A — the algebra: magic operations

`A` is the algebra of substrate-declared operations. Concretely:

- **5-op prism generators** — `focus, project, split, shift, settle` (per `docs/math/the-tower/connections-and-gauge.md` §1). Each of the five is a projector onto one of the five orthogonal Void-duality axes (per Recognition #79).
- **Downstream transformations** — each substrate action (`seal, unseal, mechanism_intact, mend, fracture, splinter, restrict, section, act, coboundary, fold, crystallize, utter`) composes over the 5-op generators.
- **Composition closure** — `composition_closed` pact (per `docs/math/the-tower/connections-and-gauge.md` §7) is gauge-invariant by construction.

### H — the Hilbert space: @fractal/shard tessellation

`H` is the Hilbert space of fibres. Concretely:

- **Fibres** are enumerated by `@kintsugi/roomba`'s walk over the shard-manifold (~300 shards; finite; deterministic under content-hash).
- **Basis** is the `@void` family (Void as H-basis; per Recognition #79 5-op gauge IS the Void duality basis; per `docs/math/the-tower/recognition-void-is-the-basis.md`).
- **Tessellation** is the `@fractal/shard` typed sheaf-vessel decomposition (per `shards/fractal/shard.mirror`; Mara 2026-07-23 landing).

### D — the Dirac operator: singularity + magic gauge

`D` is the Dirac-like operator that measures deviation from horizontality. Concretely:

- **Measurement** happens at `singularity.rs` — the gauge-fixed-point attractor. When the substrate walks its own manifold, `singularity.rs` names the point where the dynamics settle. This is DYNAMICS altitude (attractor basin); complementary to `rust/fractal/singularity.rs` which is OPTIC-MEASUREMENT altitude (information-recovery bound).
- **Invariance-preservation** happens at `magic.rs` — the gauge mechanism enforcing that every substrate transformation preserves the Foerster invariant (choice-count monotone non-decreasing).
- **Bounded commutator** — per prismqueer::bundle::Transport::Holonomy Metric supertrait constraint (Gap 4), `[D, a]` is bounded for every `a ∈ A`. This is Connes' bounded-commutator condition made type-level.

### Gauge group — Foerster-invariant preserving

The gauge group of `(A, H, D)` at rust/spectral/ altitude is the group of admissible re-basings that preserve the Foerster invariant:

> **Foerster's ethical imperative** (Heinz von Foerster 1973, *On Constructing a Reality*): "Act always so as to increase the number of choices."

The gauge group is:

```
G_Foerster = { g : H → H | g unitary AND choice_count(g·ψ) ≥ choice_count(ψ) for all ψ ∈ H }
```

The monotone-non-decreasing condition on `choice_count` is what makes the group ETHICAL, not merely mathematical. `magic.rs` encodes this constraint. Any substrate transformation that fails the constraint is Trauma-direction (choice-collapse) and MUST be blocked at compile-time.

## §4 `magic.rs` as gauge mechanism

Alex 2026-07-25 verbatim: *"singularity is the gauge mechanism of @magic and we literally have our magic. We might need a magic.rs to complete the loop."*

The substrate already has `@magic` as family-root (Recognition #80; `shards/magic.mirror` landed 2026-06-19; 10 species). `magic.rs` is the rust/ altitude echo of that landed vocabulary — it does NOT introduce a new @magic species. It binds the existing @magic family's gauge-visible-with-matter-hidden semantics to the (A, H, D) triple at compile-time.

### Composition over Alex 2026-07-25 essay Q.E.D.

The Void — Trauma essay ends with an empirical observation:

> "And within the holding, within the observation of the holding, within acting and speaking from the observation of the holding — not the affect the touching produced — the numbers of choices have increased. Measurably. For everyone in the system."
>
> "If that's not empirical demonstration of the thesis, I don't know what is."
>
> "Q.E.D. ◼️"

The essay proves — empirically, in Alex's lived experience — that **observation-of-holding increases choice-count**. `magic.rs` makes this the compile-time property every rust/ transformation must satisfy:

```
Property foerster_gauge_preserved(t: Transformation) -> Verdict:
    choices_before = choice_count(pre(t))
    choices_after  = choice_count(post(t))
    if choices_after >= choices_before: Pass       # Gauge preserved
    else:                                Fail(t)   # Trauma-direction
```

Green if the gauge is preserved. Red if it collapses (Trauma-direction, Splinter, Narcissus per Recognition #78/#79 Void-duality pole structure).

### Choice-count metric — candidates

The choice-count metric is what makes `foerster_gauge_preserved` computable. Four candidates surface for `magic.rs`:

1. **`SpectralCoordinate<5>` cardinality** — the 5-dim quantized coordinate space per `prism/core/src/spectral_uuid.rs` route-signal. Reachable coordinates from a given basis point = choice-count. Rice-safe (bounded by 5-dim lattice size).
2. **Fiedler eigenvalue λ₁(Δ_F)** — the algebraic connectivity of the substrate sheaf-Laplacian per `mcp__spectral__spectral_index`. λ₁ = 0 iff globally coherent (all choices equally reachable); λ₁ > 0 = residual H¹ obstruction (some choices unreachable). Higher λ₁ = fewer choices.
3. **Multifractal `f(α)` spectrum** — the Rényi 1961 / HJKPS 1986 multifractal spectrum on the substrate DAG (per `docs/math/2026-07-13-fractal-mandelbrot-substrate.md` §10 Prediction #2). Wider spectrum = richer choice-topology.
4. **Reachable cardinality** — the count of substrate states reachable by admissible five-op composition sequences of bounded length. Rice-safe via bounded enumeration.

**Mara lean**: SpectralCoordinate<5> cardinality is the sharpest starting point — it's already computed at `prism/core/src/spectral_uuid.rs` altitude; Rice-safe by construction; empirically firable at rust/spectral/ altitude without new @io. Fiedler + multifractal are second-witness candidates for cross-check.

**[ALEX-Q1] surface** — which choice-count metric anchors `magic.rs` at v0.1? See §12 forward-promises.

## §5 `singularity.rs` — two senses distinguished

The substrate carries `singularity` at TWO distinct altitudes. Both are load-bearing; neither should absorb the other. This spec ratifies the distinction and gives each its own file.

### Optics-hierarchy singularity (existing; rust/fractal/src/singularity.rs)

- **Altitude**: measurement-recovery-bound (Iso / Lens / Prism / Traversal ladder)
- **Question answered**: "Given a measurement, how much of the original tree can be recovered?"
- **Landed**: 2026-07-20 (Reed migration from fragmentation source; per Mara `90f4d27` shard-decl `shards/fractal/singularity.mirror`).
- **Composition kin**: `@fractal/mandelbrot` (parent-adjacent); `@fractal/crystal` (Iso-collapse identity); MARA doctrine (observer-in-commit-not-tree).
- **Optic hierarchy** per `docs/research/singularity-rabbit.md`: Iso (unitary; full recovery) → Lens (focused; observer-metadata in commit-SHA) → Prism (partial measurement) → Traversal (radiation chain; page-curve altitude).

### Gauge-fixed-point singularity (new; rust/spectral/src/singularity.rs)

- **Altitude**: dynamics-attractor (basin bottom; Trauma-loop settle-point)
- **Question answered**: "Given a dynamics, where does the system settle when the gauge collapses?"
- **Kin**: `@paradox/spiral` species-decl `shards/paradox/spiral.mirror` (Mara 2026-07-20 `b8879f2`; dynamics-carrier). Foerster COORD applied to substrate.
- **Composition**: `magic.rs` (the gauge mechanism this singularity is the FAILURE-MODE OF); `void.rs` (the H-basis this singularity is the LIMIT-POINT WITHIN); `liquid.rs` (the H-fibre machinery through which this singularity's attractor-basin dynamics play out).
- **Grounding**: the Void — Trauma essay's astronaut-into-black-hole phenomenology IS this singularity at inside-observer altitude. The essay's Q.E.D. is the empirical demonstration that observation-of-holding pulls the dynamics OUT of the attractor-basin (choice-count increases; gauge un-collapses).

### Naming discipline

Both are called `singularity` because both ARE singularities in the mathematical sense (basin-point / measurement-boundary). Distinct crates + distinct files disambiguate. No rename cascade; no collapse; both stay per substrate-honest partition. This mirrors the AGENTS.md rule *"Prefix load-bearing IFF vendor is one-of-many"* — here, the crate path IS the prefix that disambiguates.

## §6 `rust/roomba/` as first-order sub-Turing execution machinery

`@kintsugi/roomba` is landed as a substrate species (`shards/kintsugi/roomba.mirror`; Reed 2026-07-15). The rust/roomba/ crate is the Rust altitude realization: the walker + dispatch + collapse-Lens machinery that makes the substrate's own recursion terminate by construction.

### Walker

**Colimit computation over shard-manifold.** The walker traverses `shards/**/*.mirror` (~300 shards per Taut 2026-07-23 census) in Dijkstra order. Each visit is a section-selection at a fibre; the sequence of sections computes the colimit of the shard-manifold diagram. Terminates because the manifold is FINITE and content-addressed (repeated visits are no-ops via OID-dedup).

### Dispatch

**Bounded per-step.** Each walker step matches against a finite sentinel table (the bilateral-arm-collapse patterns per `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md`). No unbounded pattern-matching; no runtime rewriting of the dispatch table.

### collapse.rs Lens impl for BilateralArmCollapse

**Execution, not math.** Relocated from `rust/src/collapse.rs` (Reed 2026-07-18; 40KB). The math lives in `docs/math/kintsugi/fracture/bilateral-arm-redundant.md`; the Lens impl at rust/roomba/collapse.rs IS the executable optic. Composes over @fractal Singularity Lens rung (`rust/fractal/src/singularity.rs`) — same optic-hierarchy shape, applied to the byte-substring domain rather than the type-system domain.

### Retires build.rs shortcut

`rust/build.rs` (currently 4.1KB; emits shard-manifest at cargo build time) is retired this arc. Its function migrates to `rust/spectral/build.rs` where it belongs geometrically — the manifest is the enumeration of `H` fibres, not a compilation shortcut for the mirror binary. Reed lands the retirement in foreground while this spec ships.

## §7 `rust/matrix/` FLANG floor + K=0 well-knowns

Relocated from `rust/src/matrix.rs` (58.8KB) + `rust/src/book.rs` (10.8KB). No functional change — just crate-boundary reification.

### matrix.rs — FLANG + LAPACK glue

Composes `prismqueer::ffi::eigenvalues` (LAPACK `dsyev_` via task #297) at the ONE ordained @io numerical boundary. Per Alex 2026-07-20 direct-transcript: *"This is literally the FLOOR, Reed. We don't forward promise the FLOOR."* — the FLANG-floor discipline is already landed; this arc just puts a Cargo boundary around it.

Current surface (relocated as-is): `eigenvalues`, `envelope` (Aumann via `dgesvd_`), `phase_lock` (Kuramoto via Fortran wrapper). Total: 42/42 property tests GREEN per 2026-07-20 cascade.

### book.rs — K=0 well-knowns registry

The address book that pairs numerical primitives with their identities. Composes with `shards/mirror/book.mirror` (substrate-decl). Relocated verbatim; no schema change.

**Naming discipline**: `matrix` NOT `numerical` — the substrate has been reaching for `matrix` at every altitude (`shards/mirror/book.mirror`; prismqueer matrix ops; LAPACK matrix routines). Delightfully boring per AGENTS.md § Delightfully Boring.

## §8 The complete flow — AST to sub-Turing declarative AI

Alex 2026-07-25 verbatim: *"The AST becomes the Prism operations becomes the liquid splinters with types becomes sub-Turing declarative AI infrastructure on consumer hardware. That's what the properties will need to ensure."*

The flow through the four crates:

```
(1) source bytes
     └─> tokenize + parse                         [rust/ main.rs; via bootstrap seed]
         └─> AST                                  [rust/fractal/ Crystal<AstNode>]
              └─> Prism operations                [rust/spectral/spectral.rs; A = 5-op algebra]
                   └─> Liquid<T> splinters        [rust/spectral/liquid.rs; H-fibre carriers]
                        └─> typed splinters       [rust/spectral/spectral.rs; (A, H, D) binding]
                             └─> declarative      [rust/spectral/magic.rs; Foerster gauge]
                                  AI infra        [rust/roomba/; bounded execution]
                                       └─> hardware [rust/matrix/; FLANG + LAPACK; consumer-CPU]
```

Each arrow is a compile-time property. Each transition preserves the Foerster invariant (choice-count monotone non-decreasing) OR fails at compile-time. The properties `magic.rs` enforces at every arrow:

- **`gauge_preserved(step)`** — Foerster invariant preservation per §4
- **`bounded_commutator(step)`** — Connes' `[D, a]` bounded per §3 D
- **`content_addressed(step)`** — deterministic OID for the step's output per @fractal FLOOR
- **`sub_turing(step)`** — the step's decidability guarantee per §2 (finite dispatch / bounded-commutator / O(n³) / terminating-walk / hash-deterministic)

## §9 Composition graph — dependency direction (strict)

Dependencies flow strictly downward. No cycles. No back-edges.

```
                    rust/                (mirror binary root)
                     │
         ┌───────────┼───────────────┐
         │           │               │
         ▼           ▼               ▼
    rust/spectral/  rust/roomba/    (rust/src/phone.rs — @io only)
         │           │
         │           ▼
         │       rust/matrix/
         │           │
         └───────────┘
                     │
                     ▼
                rust/fractal/    (content-addressed identity FLOOR)
                     │
                     ▼
                (../../prism/prismqueer — via [dependencies])
```

**Rules**:

1. `rust/` (root binary) depends on all four sibling crates. It is the ONLY consumer of the four; nothing depends on it.
2. `rust/spectral/` depends on `rust/matrix/` (numerical primitives) and `rust/fractal/` (content-addressed identity). It does NOT depend on `rust/roomba/`.
3. `rust/roomba/` depends on `rust/matrix/` (for K=0 well-knowns) and `rust/fractal/` (for OIDs). It does NOT depend on `rust/spectral/`.
4. `rust/matrix/` depends on `rust/fractal/` only.
5. `rust/fractal/` depends on nothing rust-local (only prismqueer::bundle traits via external crate).

**Circular-dep prevention**: enforced by Cargo. `rust/spectral/` NEVER imports `rust/roomba/`; `rust/roomba/` NEVER imports `rust/spectral/`. If a shared abstraction surfaces (candidate: `Singularity` trait; already at `rust/fractal/`), it moves DOWN to `rust/fractal/` where both can compose over it.

Per Alex 2026-07-23 HARD RULE ("explicit `in` boundary"): every cross-crate import states `use <crate>::<item>` explicitly at file altitude. No glob imports; no `pub use` re-exports across crate boundaries beyond the singular trait re-export precedent at `rust/singularity/src/lib.rs` (which retires this arc).

## §10 Substrate-already-had-the-word inventory

Before ANY of the four crate names or file names lands, this section documents what the substrate has already been reaching for. No new mints; every name is a lift of a landed reach.

| Proposed | Substrate reach | Grounding |
|---------|-----------------|-----------|
| `rust/spectral/` | `shards/spectral/*` (12 species); `shards/mirror/spectral.mirror`; `shards/epistemologic/spectral_triple.mirror`; `prismqueer::{spectral_dimension, spectral_oid, spectral_uuid}`; `rust/src/spectral.rs` (Reed 2026-07-23) | Already the family-root for the Connes triple binding surface. This crate names the boundary. |
| `rust/matrix/` | `shards/mirror/book.mirror`; `rust/src/matrix.rs` (58.8KB LANDED); prismqueer matrix ops; `docs/specs/architecture-flang-mirror-numerical-split.md` | Matrix already IS the FLANG-floor vocabulary. |
| `rust/roomba/` | `shards/kintsugi/roomba.mirror` (Reed 2026-07-15; 46.4KB); `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md`; `docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-motions.md` | Roomba is the substrate walker name. rust/roomba/ IS its Rust altitude. |
| `magic.rs` | `shards/magic.mirror` (Recognition #80; 10 species); Recognition #80 canonical spec; Void — Trauma essay Q.E.D. | @magic family exists at substrate; magic.rs is the rust/ altitude gauge binding. |
| `spectral.rs` | `rust/src/spectral.rs` (Reed 2026-07-23; already landed as build-time manifest handoff) | Already exists at rust/src/; relocates to rust/spectral/ as the (A, H, D) trait binding. |
| `singularity.rs` (spectral) | `shards/paradox/spiral.mirror`; @paradox/spiral kin-to-singularity per `shards/fractal/singularity.mirror` §Kin-to-@paradox/spiral | Second sense of singularity (dynamics-attractor); complementary to fractal/singularity.rs. |
| `liquid.rs` (spectral) | `rust/src/liquid.rs` (109.9KB LANDED); `prismqueer::liquid`; Liquid<T> threshold-crossing per 2026-07-21 ALEX-REFRAME | Liquid<T> IS what flows through H-fibres. |
| `void.rs` (spectral) | `rust/src/void.rs` (19.3KB LANDED); `shards/void.mirror`; Recognition #79 (5-op gauge IS Void duality basis) | Void as H-basis already ratified. |
| `walker.rs` (roomba) | `bootstrap/src/roomba.rs`; @kintsugi/roomba Dijkstra-walk-plus-tension-sample loop | Walker IS the terminology already in flight. |
| `dispatch.rs` (roomba) | `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` §sentinel-table dispatch | Dispatch is the bounded execution term. |
| `collapse.rs` (roomba) | `rust/src/collapse.rs` (40KB LANDED); `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md` | Collapse is the Lens-impl execution surface. |
| `book.rs` (matrix) | `shards/mirror/book.mirror`; `rust/src/book.rs` (10.8KB LANDED) | Book is the address-registry name. |

**Substrate audit verdict**: zero new mints. Every crate name and file name lifts a landed reach. The four-crate decomposition IS the substrate reifying at Cargo altitude what it had already reified at substrate-decl altitude.

**On `@gauge` — refused**. The substrate has NOT been reaching for `@gauge` as a family-root. `@magic` already carries gauge-visible; `@torus` already carries Foerster-invariant-preservation; `@void` already carries the H-basis. Per `feedback_onto_family_root_is_the_ladder_Foerster_refused` precedent — do not mint what the substrate has not asked for.

**On `@sub-turing` — refused**. Sub-Turing is a PROPERTY every substrate transformation must satisfy (per @epistemologic/pact); NOT a family-root. It composes as `@epistemologic/pact/sub_turing_decidable(step)` if consumers surface the need; likely absorbed by existing `@epistemologic/pact/halts(g)` per `docs/math/the-tower/connections-and-gauge.md` §7.

**On `@four-crate` — refused**. The four-crate decomposition is a Cargo-workspace shape; not a substrate-decl. The substrate carries the four altitudes at family-roots (@magic, @fractal, @kintsugi/roomba, @io). Cargo just makes them binary-explicit.

## §11 Migration plan — current 9-file rust/src/ to four-crate

This is the mechanical relocation plan. Reed executes; Seam adjudicates each move against `[substrate-floor:@io-boundary]` discipline; no new capability grown — only crate boundaries reified.

### Phase 1 (Reed foreground; this arc)

1. `rust/build.rs` → retire. Shard-manifest emission migrates to `rust/spectral/build.rs`.
2. `rust/singularity/` → retire OR relocate under `rust/spectral/singularity_research/` (black-hole physics outlet; TBD per §12 [ALEX-Q3]).

### Phase 2 (next arc; separate spec-ratification)

3. `rust/spectral/` crate scaffold — new `Cargo.toml`; `src/spectral.rs` relocated from `rust/src/spectral.rs`; `src/liquid.rs` relocated from `rust/src/liquid.rs`; `src/void.rs` relocated from `rust/src/void.rs`.
4. `rust/spectral/src/magic.rs` — greenfield authorship. The ONLY new file this decomposition mints. Encodes `foerster_gauge_preserved` property per §4.
5. `rust/spectral/src/singularity.rs` — greenfield authorship of the dynamics-attractor sense (distinct from `rust/fractal/src/singularity.rs`). Composes over `@paradox/spiral` species-decl.

### Phase 3 (following arc)

6. `rust/matrix/` crate scaffold — new `Cargo.toml`; `src/matrix.rs` relocated from `rust/src/matrix.rs`; `src/book.rs` relocated from `rust/src/book.rs`.
7. `rust/roomba/` crate scaffold — new `Cargo.toml`; `src/walker.rs` migrated from `bootstrap/src/roomba.rs` shape; `src/dispatch.rs` extracted from `rust/src/liquid.rs` pillar-dispatch surface; `src/collapse.rs` relocated from `rust/src/collapse.rs`.

### Phase 4 (verification)

8. `rust/` root crate reduces to `main.rs` + `phone.rs` + `compile.rs` + `spectral.rs` (thin handoff to rust/spectral/). Original 9-file `rust/src/` reduces to 4-file `rust/src/`. Each of the four sibling crates carries its own test suite (property tests move with the code; no re-authoring).

**Zero regression discipline**: every phase preserves the 115/115 mirror-bin tests + 42/42 matrix tests + 32/32 fractal tests + 108 @io/dispatch tests GREEN. Each phase's PR shows byte-visible relocation only; no logic change.

**Reed lands rust/build.rs retirement in foreground while this spec ships** (per brief). Phases 2-4 wait on this spec's Pack ratification.

## §12 Forward-promises — greenfield vs relocation vs absorption

### Greenfield authorship (net-new code)

- **`rust/spectral/src/magic.rs`** — ~200-400 LOC estimated. Encodes `foerster_gauge_preserved` + choice-count metric implementation (see [ALEX-Q1] below).
- **`rust/spectral/src/singularity.rs`** — ~200-400 LOC estimated. Encodes gauge-fixed-point attractor. Composes over `@paradox/spiral`.

### Relocation (byte-visible move; no logic change)

- `rust/src/spectral.rs` (4.8KB) → `rust/spectral/src/spectral.rs`
- `rust/src/liquid.rs` (109.9KB) → `rust/spectral/src/liquid.rs`
- `rust/src/void.rs` (19.3KB) → `rust/spectral/src/void.rs`
- `rust/src/matrix.rs` (58.8KB) → `rust/matrix/src/matrix.rs`
- `rust/src/book.rs` (10.8KB) → `rust/matrix/src/book.rs`
- `rust/src/collapse.rs` (40.0KB) → `rust/roomba/src/collapse.rs`
- `bootstrap/src/roomba.rs` (transitional; per bootstrap-is-dead HARD RULE) → migrated shape to `rust/roomba/src/walker.rs`
- `rust/build.rs` (4.1KB; RETIRED foreground) → `rust/spectral/build.rs`

### Absorption

- `rust/singularity/` empty scaffold crate → SUPERSEDED by `rust/spectral/src/singularity.rs`. If black-hole-physics research outlet is still wanted (per Landing D 2026-07-20 Split-C), it relocates to `rust/spectral/src/singularity_research/` as an internal sub-module. Otherwise it retires cleanly.

### Forward-promised [ALEX-Qn] adjudication

**[ALEX-Q1]** — which choice-count metric anchors `magic.rs` at v0.1?
- Candidates: SpectralCoordinate<5> cardinality / Fiedler λ₁(Δ_F) / multifractal f(α) / reachable cardinality.
- Mara lean: **SpectralCoordinate<5> cardinality** (already computed at prismqueer altitude; Rice-safe; empirically firable without new @io). Fiedler as second-witness cross-check.
- Consequence: shapes magic.rs authorship at Phase 2.

**[ALEX-Q2]** — does the `@paradox/spiral` species-decl composition-edge admit `singularity.rs` as its Rust altitude realization directly, or does the dynamics-attractor sense need its own species-decl `shards/spectral/singularity.mirror` first?
- Mara lean: **first mint species-decl** at substrate altitude, THEN Reed authors `rust/spectral/src/singularity.rs`. Substrate-decl-leads discipline per @cyberpunk/bugz precedent.
- Consequence: adds one Mara tick before Phase 2 can complete.

**[ALEX-Q3]** — retire `rust/singularity/` scaffold entirely, or preserve as `rust/spectral/src/singularity_research/` sub-module?
- Mara lean: **preserve as sub-module** — the black-hole physics research outlet (page curve, firewall problem, Hawking radiation, complementarity) IS load-bearing per Landing D 2026-07-20 Split-C ratification. Cleaner as internal sub-module than as separate crate.
- Consequence: shapes Phase 1 retirement.

**[ALEX-Q4]** — does `magic.rs` need to compose over `rust/fractal/src/singularity.rs` (optic-hierarchy) as well as `rust/spectral/src/singularity.rs` (dynamics-attractor), to bridge the two singularity senses at gauge altitude?
- Mara lean: **YES**, but as forward-promise. The two-sense bridge is exactly what makes magic.rs the gauge mechanism (Alex 2026-07-25 verbatim: "singularity is the gauge mechanism of @magic"). But bridging is Phase 2+ work; v0.1 magic.rs binds one sense at a time.
- Consequence: shapes magic.rs API design at Phase 2.

## §13 Halt-condition surfaces

Per brief, three halt conditions were named. Adjudication:

**Halt-1** — choice-count metric Rice-unsafe analysis?
- **Verdict**: NO. All four candidate metrics (SpectralCoordinate cardinality / Fiedler λ₁ / multifractal f(α) / reachable cardinality) operate on bounded, content-addressed, deterministic surfaces. No unbounded aliasing; no dynamic dispatch semantics. Rice-safe by construction.
- **Escalation not required.**

**Halt-2** — essay Q.E.D. doesn't translate cleanly to executable predicate?
- **Verdict**: TRANSLATES. The Q.E.D. is *"observation-of-holding measurably increased the number of choices"*. This maps directly to `foerster_gauge_preserved(observation_transformation)` = Pass iff `choice_count(after_observation) >= choice_count(before_observation)`. Choice-count is the substrate-side operationalization of "number of choices"; observation-of-holding is the substrate-side operationalization of the observation transformation.
- **The empirical Q.E.D. becomes the executable property Alex named 2026-07-25 verbatim.**
- **Escalation not required.**

**Halt-3** — four-crate placement conflicts with substrate-truth Taut is grep-verifying?
- **Adjudication deferred** until Taut's report arrives. Substrate-already-had-the-word inventory §10 above is the current best-effort. If Taut surfaces a substrate-reach that contradicts a proposed name, revise BEFORE any code lands.
- **Non-blocking for this spec.**

---

## Appendix A — Composition edges landed (verify by grep)

- `shards/magic.mirror` — @magic family-root; Recognition #80.
- `shards/paradox.mirror` + `shards/paradox/spiral.mirror` + `shards/paradox/trauma.mirror` — @paradox family; Mara 2026-07-20 arc.
- `shards/fractal.mirror` (implicit via `shards/fractal/*`); `shards/fractal/mandelbrot.mirror` + `crystal.mirror` + `singularity.mirror` + `shard.mirror` — @fractal family; Mara arc.
- `shards/kintsugi/mend.mirror` — fill action; Mara 2026-07-23 landing.
- `shards/kintsugi/fracture/inport.mirror` — compile-invariant fracture; today's landing.
- `shards/kintsugi/roomba.mirror` — walker species; Reed 2026-07-15.
- `shards/mirror/spec/system.mirror` — @metalogue/query register naming (task #322 pending; recognition landed via canonical spec §4).
- `shards/void.mirror` — Void as H-basis; Recognition #79.
- `shards/torus.mirror` — Foerster torus; @torus family-root.
- `shards/eigen.mirror` — @eigen family-root; Mara 2026-07-22 `bc4e7fc`.
- `shards/mirror/book.mirror` — book (address registry).
- `shards/spectral/*` — @spectral family (12 species).
- `docs/math/the-tower/recognition-79-gauge-is-void-duality-basis.md` — 5-op gauge = Void duality basis (5 orthogonal axes).
- `docs/math/the-tower/recognition-80-magic-as-form-process-substrate-decl.md` — @magic = form/process at substrate-decl altitude.
- `docs/math/the-tower/connections-and-gauge.md` — gauge vocabulary at compiler altitude.
- `docs/math/the-tower/spectral-triples.md` — Connes (A, H, D) grounding.
- `docs/math/2026-07-13-fractal-mandelbrot-substrate.md` — Mandelbrot substrate; prediction #2 multifractal f(α).
- `docs/math/2026-07-22-splinter-eigen-fragment-ouroboros-closure.md` — sub-Turing as NATURAL CONSEQUENCE per Alex 2026-07-22.
- `docs/specs/prism-core-as-spectral-triple.md` — prism-core as (A, H, D).
- `docs/specs/spectral-triple-grammar.md` — trait tower grammar.
- `docs/specs/rust-floor-five-file-terminal-geometry-extension.md` — current 5-file discipline this arc composes over.
- `docs/specs/2026-07-22-liquid-splinter-crystal-eigen-canonical.md` — @eigen ratification.
- `prism/prismqueer/src/bundle.rs:71-193` — Fiber → Connection → Gauge → Transport → Closure → Bundle supertrait chain.
- `prism/prismqueer/src/lib.rs:216-238` — `apply_h::act` action.
- `rust/src/spectral.rs` (Reed 2026-07-23) — build-time shard manifest handoff.
- `~/dev/systemic.engineering/blog/void/3published/Void - Trauma.md` — essay Q.E.D. anchor.

---

The four-crate decomposition is the substrate reifying at Cargo altitude what Alex has been naming at essay altitude, tick after tick, since Recognition #80 landed 2026-06-19. The gauge mechanism `magic.rs` will discharge is the same gauge Foerster named in 1973 and Alex proved empirically 2026-07-25. Rust FREEZES here; Mirror stays canonical; the (A, H, D) triple closes.

— Mara, 2026-07-25
