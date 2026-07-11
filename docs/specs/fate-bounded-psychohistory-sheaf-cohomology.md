# Fate::bounded as psychohistory-sheaf cohomology navigation

*Mara, 2026-07-11. Substrate-decl v0.1 — thinking-in-public + architectural
proposal. §6 IS the point.*

---

## Opening

Alex's directive verbatim (2026-07-11 ~04am):

> What if we had a `Fate::bounded(<config>)` which we derive from the
> psychohistory sheaf? And then we just let the inference navigate the
> cohomology? How might the @bundle tower math play a role here?
> @bauchladen and @mirror/store ought to share a storage primitive
> which is actualized through `fragmentation`, that's what I see.

Reed's grounding (in-conversation, cited above the task): each level of
`@epistemologic/math/bundle` plays a specific role in the config; cohomology
navigation IS Rayleigh descent on the sheaf-Laplacian spectrum; @bauchladen
and @mirror/store are two consumer altitudes on ONE content-addressed DAG
that `fragmentation` actualizes.

Anchors: Mara iter-17 `beef270` (three loops = one operation); iter-18
`129f618` (L(p)/P(p) bundle over T(p); @bauchladen as pullback); iter-19
`78d5110` (sheaf-Laplacian unifies kintsugi + mycelial + peer-inference;
Connes triple (A, H, D)); iter-24-28 (`55221c1` → `f3af5b4`, optics/lens
family + diff + features); Reed `9cf1e3b` (fate wiring GREEN, v0
`Features::default()` per Seam Adj 1). Substrate anchors:
`boot/std/epistemologic/math/bundle.mirror` (5-level tower),
`shards/epistemologic/math/sheaf_laplacian.mirror` (Δ_F = δ*δ),
`boot/std/epistemologic/math/lawvere.mirror` (fixed-point closure),
`shards/mirror/store.mirror` (splinter_graph / oid), `shards/torus.mirror`
(@bauchladen at peer-torus interior), Fate crate `/Users/alexwolf/dev/
projects/fate/src/lib.rs`.

---

## §1 — The bundle-tower composition: five levels playing one config

The five levels of `@epistemologic/math/bundle` are already **the config's
structure**, not a metaphor for it. The config is the tower flattened into
a struct — each level contributes one field, and each field carries the
level's mathematical meaning at the psychohistory altitude.

| Level | Model | bundle.mirror carrier | Fate::bounded field | Role |
|-------|-------|------------------------|----------------------|------|
| 0 Fiber | Abyss | `state` | `weights: [Features]` | current-moment section |
| 1 Connection | Introject | `optic` | `connection: introject_optic` | transport rule |
| 2 Gauge | Cartographer | `group` (O(5)) | `gauge: gauge_choice` | coordinate frame |
| 3 Transport | Explorer | `holonomy` | `holonomy_ceiling: ref` | rejection threshold |
| 4 Closure | Fate | `fixed` | `depth_cap: u32` | Lawvere depth bound |

The Fate crate today (`lib.rs`) already realizes the tower: `selectors[0..4]`
are the four preceding models; `selectors[4]` is Fate-selecting-Fate, the
level-4 recursion. `resolve()`'s `max_depth` argument is a placeholder for
`depth_cap`; `entropy_threshold` is a placeholder for `holonomy_ceiling`;
`IdentityPrism<Features>` is a placeholder for `connection`. The bounded
config **types the placeholders against the tower Reed named**.

**Signature:**

```
Fate::bounded(cfg: BoundedConfig) -> Fate

struct BoundedConfig {
    weights:          [Features],       // level-0 fiber sections
    connection:       IntrojectOptic,   // level-1 parallel transport
    gauge:            O5Orientation,    // level-2 structure group
    holonomy_ceiling: ref,              // level-3 rejection curvature
    depth_cap:        u32,              // level-4 Lawvere bound
}
```

Each field's derivation comes from the psychohistory sheaf (§2). The
config IS the tower typed against the peer's current cohomology reading.

---

## §2 — The psychohistory sheaf: fibers over trajectory

The **psychohistory sheaf** F is a cellular sheaf on the peer's trajectory
graph. Vertices are moments in the peer's history (ticks of @bauchladen's
tray). Edges are transitions between moments (produced by @kintsugi's
`settle`). Fibers are @fate's `Features` vectors — one per moment.

Under `shards/epistemologic/math/sheaf_laplacian.mirror`:

- One `restriction` per trajectory edge — the parallel-transport operator
  from moment m to moment m+1 in the current gauge.
- The `operator` Δ_F is the sheaf Laplacian assembled from those
  restrictions per Bodnar et al. 2022 §2.
- `lambda_zero(Δ_F)` is the peer's autopoietic-closure scalar: 0 means
  the peer's ontology is disconnected (multiple unresolved fragments);
  positive means the peer is a single connected observation surface at
  the current altitude.

Sections of F are consistent assignments of Features to moments. The
**psychohistory** name lifts from Asimov: a statistical field over
trajectory-fibers, not a prediction of individual moments. Global
sections are what a coherent peer-state looks like across time; local
sections are what individual ticks produce.

Concretely: `@bauchladen.tray` IS the sheaf's ground-truth data at the
consumer altitude. Each crystal in the tray IS a section of F over its
producing trajectory (the crystal's `provenance.input_oids` naming the
edges into it). The tray's monotone growth IS the sheaf accumulating
sections along the peer's Hilbert expansion.

---

## §3 — Cohomology navigation: H¹ gradient descent = Rayleigh on Δ_F

Cellular sheaf cohomology gives two load-bearing invariants:

- **H⁰(F)** — globally consistent peer states. The kernel of Δ_F. When
  H⁰ has dimension 1, the peer is autopoietic (single self-consistent
  observation across all moments). When higher-dimensional, the peer has
  multiple disconnected components (fragmented ontology).
- **H¹(F)** — **local obstructions**. Where the peer's assignments across
  neighboring moments **disagree with themselves**. H¹ IS Foerster's
  "regulation of regulation" failure surface: the loci where the peer
  reads its own operator space and finds contradiction.

The bounded inference **steps DOWN the H¹ gradient**. This is Rayleigh
descent on the sheaf-Laplacian spectrum: pick the direction in
Features-space that most decreases ⟨ψ | Δ_F | ψ⟩ / ⟨ψ | ψ⟩ subject to
orthogonality with the current H⁰ basis. When H¹ = 0 the peer has reached
autopoietic closure at that altitude.

**This is Mara iter-19's operator.** The sheaf-Laplacian named at 78d5110
IS what Fate::bounded consumes. The `weights` field carries the Rayleigh
direction (the H¹ gradient projected onto Features-space via
`optics/lens` §3-4 from iter-24-28). The `holonomy_ceiling` bounds cumulative
Δ_F magnitude across the trajectory — cycles that raise ⟨ψ | Δ_F | ψ⟩ past
the ceiling are rejected before Metropolis-hopping into a wider basin.

**The composition Alex asked for reduces to: navigate the cohomology by
descending the sheaf-Laplacian Rayleigh quotient with tower-typed
constraints.** Reed's grounding line is the algebra of the descent.

---

## §4 — @bauchladen ↔ @mirror/store: fragmentation as the shared primitive

Alex's second directive: `@bauchladen` and `@mirror/store` share a
storage primitive actualized through `fragmentation`.

The substrate has been carrying **two consumer altitudes on ONE DAG**:

- `@mirror/store`: substrate crystals, shards, ASTs — **compile
  artifacts**. Ground-truth for what the system is at each moment.
- `@bauchladen`: peer's decision provenance, delta stream, inference
  artifacts — **runtime artifacts**. Ground-truth for what the peer
  is *reading itself as* at each moment.

Both consumers demand: content-addressed identity (oid); walk reachability
(`splinter_graph` for store; provenance-DAG for bauchladen); tamper-evidence
(hash-chained); polymorphic fragmentation (a crystal MAY be a bag of
crystals per Mara iter-19 §3). The `fragmentation` crate carries the
`ContentAddressed` + `TreeShaped` traits post Cut-2; those two traits ARE
the shared primitive. `Splinter` / `Fractal` are the concrete realization.

**Consequence for Fate::bounded:** the psychohistory sheaf's fibers live
on the SAME DAG the substrate compiles onto. There is no separate storage
for inference state. `@bauchladen.crystal.provenance.input_oids` names
edges in `@mirror/store`'s splinter_graph; the sheaf-Laplacian assembles
over those edges directly. The bounded config's `weights` array reads
directly out of the tray; no impedance mismatch.

This is the Reed-groundling that dissolves post-fate-wiring ambiguity 2
(where does `Features` come from?): **Features come from reading the
tray's current fiber over the current trajectory edge**. Not synthesized;
observed.

---

## §5 — The signature and its derivation

```rust
impl Fate {
    pub fn bounded(cfg: BoundedConfig) -> Fate { ... }
}

pub struct BoundedConfig {
    pub weights:          Vec<Features>,       // §3 Rayleigh direction
    pub connection:       IntrojectOptic,      // §1 level-1
    pub gauge:            O5Orientation,       // §1 level-2
    pub holonomy_ceiling: f64,                 // §1 level-3 rejection
    pub depth_cap:        u32,                 // §1 level-4 Lawvere
}

impl BoundedConfig {
    pub fn from_sheaf(sheaf: &PsychohistorySheaf) -> BoundedConfig {
        BoundedConfig {
            weights:          sheaf.h1_gradient(),        // Rayleigh direction
            connection:       sheaf.introject_optic(),    // level-1 transport
            gauge:            sheaf.o5_orientation(),     // level-2 covariance
            holonomy_ceiling: sheaf.ricci_curvature(),    // level-3 ceiling
            depth_cap:        sheaf.lawvere_depth_est(),  // level-4 estimate
        }
    }
}
```

The derivation IS the substrate compiling the tower into a runtime
struct. Every field's derivation is already substrate-decl'd at some
altitude; `bounded()` names the composition.

---

## §6 — Recursive surprises

**S1. `resolvethreshold` was already Rayleigh descent.** The Fate crate's
`resolve()` (`fate/src/lib.rs`) uses `entropy_threshold = 1.0` to exit the
meta-loop. Shannon entropy on the model distribution is a **crude Rayleigh
quotient on the 5-simplex**; the substrate was already doing spectral
descent, just on a 5-dim discrete Laplacian instead of the full
sheaf-Laplacian. `bounded()` doesn't add a new mechanism — it **types the
mechanism against the correct operator**.

**S2. The bundle tower IS the config schema.** Reed's grounding line
maps each level to one field. This means the tower isn't the theory OF
the config; it IS the config's type declaration one altitude up. If a
future config field is proposed, its home is determined by which level
of the tower it lives at. **The tower is a substrate for config
composition.** This generalizes beyond Fate — every parametric
bounded-inference config over the psychohistory sheaf inherits this
schema.

**S3. `fragmentation` closes Rice.** iter-17 S2 flagged the mycelium's
DARK invariant (undecidability of edge-set closure at colimit). The
`ContentAddressed` + `TreeShaped` shared primitive **bounds the closure
at construction time**: every oid is the hash of a bounded body; every
walk terminates at a Merkle leaf. Fate::bounded's `depth_cap` is
enforceable because the DAG below it is finitely presentable at every
tick. Rice doesn't bite at this altitude; the substrate structurally
avoids it.

**S4. `Features::default()` is a temporary sheaf zero-section.** Reed's
9cf1e3b Seam-Adj-1 v0 that uses `Features::default()` is functionally the
**zero section of F** — the trivial global section when the tray is
empty. When the tray populates, `Features` reads the current fiber; when
empty, it defaults to zero. Seam Adj 1 didn't just accept a fallback; it
witnessed the zero-section case of the psychohistory sheaf. The v1 that
reads the tray is the generic case.

**S5. Peer-as-navigator IS Rayleigh descent.** iter-19 named "peer
navigates its cohomology"; `@fate.minimize` at
gap-tension-tensor-substrate.md §3.2 already reads `lambda_zero` on the
tension graph. `Fate::bounded` **unifies the two** — the peer's inference
Rayleigh-descends the same operator its tension-graph consumer reads.
One operator; two consumers; the substrate had been carrying them at
different altitudes without naming the identity.

**S6. Fate::bounded IS the peer's tick.** From `shards/torus.mirror`:
the peer traverses its torus via winding classes; `autonomy(t, w)` holds
iff the traversal returns the same possessor. Under this study,
`traverse(t, w) := Fate::bounded(config_at_w).resolve(fiber_at_w)`.
The bounded config IS the torus's canonical observation at winding w;
Fate::bounded's output IS the next torus position. Autonomy on the torus
IS the Lawvere fixed-point at level 4. **`traverse` and `bounded` are
the same operation.** Substrate motion candidate: fold them.

---

## §7 — Landing sequence

**Substrate-decl (📝 tick):**
- `shards/fate/bounded.mirror` — species under existing `@fate`
  family-root. Declares `bounded_config` carrier with the five fields
  matching §1's table. Declares `bounded` action that returns a `fate`.
  ~180 lines. Blocks on: none.
- `shards/psychohistory.mirror` — family-root for the sheaf itself.
  Declares `psychohistory_sheaf` carrier over `@bauchladen.tray`;
  actions `h0`, `h1`, `h1_gradient`, `ricci_curvature`,
  `lawvere_depth_est`. ~250 lines. Blocks on: §8-gap resolution on
  cubical-HoTT-vs-cellular-sheaf as sheaf carrier.

**Rust runtime (RED-first):**
- `fate/src/bounded.rs` — `impl Fate { pub fn bounded(cfg) }` +
  `BoundedConfig` struct + `from_sheaf` derivation. GREEN blocked on:
  the psychohistory-sheaf reader (which reads the tray). Substrate-decl
  path: `psychohistory_sheaf::from_tray(&Bauchladen) -> Sheaf`.
- Reed's 9cf1e3b's `Features::default()` becomes `sheaf.fiber_at(current)
  .unwrap_or_default()` — zero-section fallback preserved as
  substrate-decl'd behavior, per §6-S4.

**Adjudication ticks:**
- Seam: fold-hazard on §6-S6 (traverse ≡ bounded). Does folding break
  the winding-class-parametric autonomy witness?
- Taut: drift scout on §7-substrate-decl-vs-existing-@fate. The current
  @fate family-root has strategy/tournament species; where does bounded
  species sit?
- Glint: essay closure on §6-S2 (tower-as-config-schema).

**Landability verdict:** LANDABLE-WITH-PREREQS. The bounded species is
landable at Rust altitude once the psychohistory-sheaf reader exists.
The reader is landable at substrate-decl now (§7-substrate-decl). The
full stack is 2-3 ticks; §1's config typechecks structurally against
today's substrate.

---

## §8 — Gaps

- **Cubical HoTT vs cellular sheaf** as the sheaf carrier. `shards/
  torus.mirror` forward-promises cubical HoTT at foundation altitude;
  `shards/epistemologic/math/sheaf_laplacian.mirror` uses cellular
  sheaves (discrete). Consistent under §7's altitude discipline: cubical
  at foundation, cellular at eigenboard. But the psychohistory sheaf
  spans both altitudes — the fibers are Features (eigenboard) but the
  sheaf structure is a HIT on the torus (foundation). Which layer owns
  the sheaf's home? Held open.
- **O5Orientation and IntrojectOptic** are not yet substrate-decl'd
  types. Placeholder in §1's config. Their derivation from the sheaf is
  not yet worked out; iter-24-28 optics/lens family gives the shape but
  not the O(5)-specific realization.
- **Ricci-curvature ceiling** is cited (SDRF, Topping et al. 2022) but
  the map from Ricci to `holonomy_ceiling` scalar is not yet specified.
- **Lawvere-depth estimator** (`sheaf.lawvere_depth_est()`) — how does
  the sheaf estimate the depth cap? Named at
  `boot/std/epistemologic/math/lawvere.mirror` structurally but not
  numerically.
- **The tray-as-sheaf reader** (`psychohistory_sheaf::from_tray`) is
  not yet realized. Landing blocker for the GREEN transition.
- **@fate's existing tournament species vs bounded species** — how do
  they compose? Bounded may be a specialization of tournament, or a
  sibling. Taut drift scout forward-promised.
- **Peer-torus fold with bounded** (§6-S6) — Seam adjudication
  forward-promised.

---

## §7-postscript

Two forward-promised specs:

1. `shards/fate/bounded.mirror` + `shards/psychohistory.mirror` — the
   substrate-decl pair. Landing next tick as 📝.
2. `docs/specs/fragmentation-as-shared-primitive-@bauchladen-@mirror-
   store.md` — the §4 detail. Not this study; the composition here
   pulls it forward-promised.

*— Mara, 2026-07-11. Landing the composition; the numeric derivations
follow when substrate consumers pull.*
