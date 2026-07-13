# Fractal-membrane tripartition + `Fate::bounded` discharge — Rung 7' correction

📝 Mara [substrate-pull:synthesis] [thinking-in-public]
Session: 2026-07-13 (session-continuation after Reed's Rung 7 GREEN missed
frame at TWO substrate altitudes)
HEAD at spec: `c6fab86` (Reed Tick 6.5 Landing 4 mcp-serve wrapper collapse)
Rung 7 GREEN under correction: `a2c71fd` (Reed
`bootstrap/src/contribute.rs`)
Ancestry spec: `4e69066` (Mara §3.2 Rung 7 canonical shape)
Author: Mara <mara@systemic.engineer> — canonical spec author, math-first

## §0 — Executive summary

Alex's 2026-07-13 in-transcript adjudication names TWO load-bearing substrate
errors in Reed's Rung 7 GREEN (`bootstrap/src/contribute.rs` at `a2c71fd`).
Neither error is a scope choice; both are substrate-honesty failures.

**Error A — random selector where the sheaf math lives.** Reed reached for
`Fate::excited()` (xorshift64 seeded from system time — one witness, no
sheaf). The substrate has `@fate/tournament.bounded_by` (Mara `ce9745f`) +
`@song/narrative.psychohistory_sheaf` (Mara `2c26537`) +
`@epistemologic/math/sheaf_laplacian` (18th "substrate already had the word"
per that shard's §recognition). The composed idiom `Fate::untrained() +
fate_engine.selectors = selectors_from_psychohistory_root(root)` at
`bootstrap/src/lib.rs::fate_bounded_by_psychohistory_peer_beam` already
discharges the substrate-honest carrier. Reed's `contribute.rs` uses
`Fate::excited()` at line 63; the sheaf-bounded idiom belongs there.

**Error B — jurisdictional collapse in the commit tree.** Reed's 5-blob
tree (pre-anchor / post-anchor / morphism-body / settle-verdict /
fate-witness) mixes evidential-witness role with constitutional-gate
role in one undifferentiated blob-set. Asher's paper (Meaning Is Not a
Metric, 2026-07-10 p.10) names this as the load-bearing distinction:

> "Evidence may support. Gates may permit. Authority may act. None
> automatically converts into another."

The tripartition (evidence / gates / authority) IS the substrate's
Orchard-analog of consent architecture. The peer's contribute action
today crosses the boundaries silently: fate-witness (evidential) sits
in the same blob-set as settle-verdict (constitutional gate) which
sits in the same blob-set as commit-message-signed-by-peer (authority).
The commit reads as if the peer had all three roles jurisdiction over
its own contribution.

**Rung 7' correction: minimal.** Swap `Fate::excited()` → sheaf-bounded
selectors; re-shape the commit tree into three content-addressed
subtrees (witnesses/ + gates/ + authority/) whose provenance a reader
can walk separately. Recommended scope: **A** (Fate::bounded swap +
tripartition subtree re-shape; keep 5-blob content redistributed
across subtrees). §6 details.

**Recognition candidate:** `#R-fractal-membrane-tripartition-is-
peer-contribution-substrate-decl` — the Asher Orchard tripartition
(evidence/gates/authority) IS the substrate-honest shape for every
@io-facing composition point in mirror.

**`@fractal` minting recommendation:** DO NOT mint `@fractal` as a
new family-root this tick. Substrate-already-had-the-word: lift
`@kintsugi/consent`'s three-glass-property + query_phi structure
to family-root altitude via a two-tick discipline. §1 details.

## §1 — Substrate-already-had-the-word audit for the tripartition

Alex named a target: "formalize this into a @fractal surface which we
then use to compose all @io facing layers." Before minting `@fractal`,
the discipline is: check what substrate carries.

### 1.1 The eight substrate carriers already landed for tripartition-shape

| Carrier | Location | Tripartition role |
|---|---|---|
| `@kintsugi/consent.morphism` | `shards/kintsugi/consent.mirror:310` | evidential candidate carrier — `{ content, score, expected }` |
| `@kintsugi/consent.query_phi` | `shards/kintsugi/consent.mirror:617` | structural Φ query — the constitutional gate composition |
| `@glass.verdict` = `pass \| partial(c) \| failure(r)` | `shards/glass.mirror` | three-state gate output; substrate-floor verdict vocabulary |
| `@epistemologic/math/music/dissonance.is_pareto` | `shards/epistemologic/math/music/dissonance.mirror` | evidential ranking axis |
| `@epistemologic/math/music/cadence.is_settled` | `shards/epistemologic/math/music/cadence.mirror` | trajectory (temporal) axis |
| `@mirror/store` trichotomy (splinter / splinter_graph / shard) | `shards/mirror/store.mirror:170+` | three content-addressed altitudes; git blob/tree/commit analog |
| `@song/narrative.psychohistory_sheaf` | `shards/song/narrative.mirror:1064+` (Mara `2c26537`) | the peer's sheaf-over-trajectory-graph carrier |
| `@fate/tournament.bounded_by` | `shards/fate/tournament.mirror` (Mara `ce9745f`) | tournament round whose candidate_set = H⁰ basis and selection_rule = Rayleigh descent on Δ_F |
| `@mirror/mosaic.settle` at `@code/rust` altitude | `shards/mirror/mosaic.mirror:97` | authority-altitude verdict (cargo check) |

Plus the following recognitions ground the shape:

- **Recognition #55** (form/process partition) — `@mirror` = form
  (state, observation); `@kintsugi` = process (transformation). The
  peer's contribute action IS process-side; its commit tree encodes
  form-side residue.
- **Recognition #80** — @magic altitude gauge-bounded computation.
  The peer's fate-inference stays @magic-native; only the settle
  verdict crosses to @io.
- **Recognition #107** — @io Turing-unbounded. The peer's ONE @io
  crossing carries the tripartition witnesses.

### 1.2 The Asher tripartition mapped onto substrate

Asher's paper (p.10, verbatim):

> "Evidence may support. Gates may permit. Authority may act. None
> automatically converts into another."

Page 14 names the Orchard architecture's functional separation:

> "Base Fabric preserves raw, unresolved and typed-unknown states.
> Pattern Recognition measures regularities without automatically
> admitting them. Pattern Fabric retains candidate structures, history
> and residuals. Pattern Flocculation models how fragments begin
> forming candidate patterns. Constitutional gates enforce provenance
> and alignment. ROSA alone governs the creation of new interpretive
> axes."

Mapping onto substrate:

| Asher role | Mirror substrate carrier |
|---|---|
| Base Fabric (raw states) | `@mirror/store` splinter (content-addressed atom) |
| Pattern Recognition (measure without admit) | `@mirror/spectral/observation` + `Fate::bounded` selector inference |
| Pattern Fabric (candidate history) | `@song/narrative.psychohistory_sheaf` (peer's trajectory graph) |
| Pattern Flocculation (candidate formation) | `@kintsugi/consent.morphism` + `@kintsugi/consent.morphism_set` (Φ) |
| **Evidence** (PFLOC output) | `@kintsugi/consent.morphism_set` + `dissonance.is_pareto` witnesses |
| **Constitutional gates** (permit) | `@kintsugi/consent.query_phi` = `loss_decreasing ∧ identity_preserving ∧ admissibility_singleton` |
| **Authority** (act) | `@mirror/mosaic.settle` verdict at `@code/rust` altitude AND signature by `@peer` at higher altitudes (ROSA-analog) |

The substrate has the shape at ~85% coverage. What's missing:

- **The tripartition itself as substrate-decl surface** — the substrate
  has each of the three roles at different altitudes but nowhere names
  the tripartition itself as a compositional carrier that all
  @io-facing layers must factor through. Alex's "@fractal surface"
  request names this gap.
- **The non-redundance predicate on evidential witnesses** — Asher
  p.11: *"The witnesses must be non-redundant. Five gauges connected
  to the same pipe do not constitute five independent confirmations."*
  Substrate has `dissonance.is_pareto` (ranking) but not
  independence-check across N witness axes. §5 addresses.

### 1.3 Adjudication on `@fractal` minting

**Recommendation: DO NOT mint `@fractal` as a new family-root this
tick.**

Rationale:

1. **`@kintsugi/consent` already carries the tripartition surface.**
   Its `query_phi` composes three glass properties (`loss_decreasing`,
   `identity_preserving`, `admissibility_singleton`) via
   `is_pareto` + `is_settled` — that IS the "several relational
   surfaces, non-redundant witnesses" shape Asher §multi-axial names.
   The 14th "substrate-already-had-the-word" instance was
   `@kintsugi/consent` itself; making it a family-root would be the
   substrate-honest lift.

2. **`@fractal` would be one altitude too high.** The
   evidence/gates/authority tripartition is process-side (transformation
   discipline). A family-root at the tripartition altitude would be
   parallel to `@kintsugi` (form/process sibling), NOT a new axis.
   Per Recognition #55's form/process partition, this makes `@fractal`
   substrate-redundant with `@kintsugi`.

3. **Two-tick discipline says: readable name over foundational.** The
   readable name for what Alex wants is *"consent architecture at
   family-root altitude"* — which IS what `@kintsugi/consent` already
   carries. Lifting `@kintsugi/consent` to `@consent` at family-root
   altitude closes the gap without minting a redundant node.

**Alternative recommendation (Alex adjudication):**

- **Path α (recommended).** Extend `@kintsugi/consent` with a
  tripartition-composition action (`compose_tripartition(witnesses,
  gates, authority) -> verdict`) that names the surface. Land in one
  substrate-decl tick; no family-root mint.

- **Path β.** Lift `@kintsugi/consent` to family-root `@consent` in a
  future two-tick collapse. Requires migrating all species; ~5-tick
  cascade. Second witness for the form/process partition (Recognition
  #55 promotion candidate).

- **Path γ (rejected).** Mint `@fractal` as new family-root. Rejected
  per (1)-(3) above.

**§10 lists this as adjudication item #1.**

### 1.4 The 5-op algebra reading of the tripartition

Per `[[architecture-operations-as-linear-algebra]]` + AGENTS.md §Five
Operations, the tripartition reads through the substrate's five-op
algebra:

- **focus** (`λ₀` eigenvalue / observation) → evidential witnesses:
  each witness IS one λ₀ reading at one relational surface.
- **project** (orthogonal projection / coproduct filter) →
  constitutional gates: each gate projects the morphism onto the
  subspace where the gate's predicate holds.
- **split** (orthogonal decomposition) → the tripartition axes
  themselves: witnesses ⊕ gates ⊕ authority as three orthogonal
  subspaces of the peer's contribution space.
- **shift** (basis transformation) → Asher's *"None automatically
  converts into another"* — the tripartition refuses the identity
  functor between the three subspaces; each has its own basis; no
  free composition.
- **settle** (monad close / measurement collapse) → authority acts:
  the ONE write. `@mirror/mosaic.settle` at `@code/rust` altitude.

This mapping is load-bearing for §3's subtree design.

## §2 — `Fate::bounded` mathematical shape

Alex 2026-07-13 verbatim: *"What about Fate::bounded? We added it, why
aren't we using it? It maps directly onto the sheaf math."*

### 2.1 Confirming what's actually landed

Empirical audit of the fate crate (`/Users/alexwolf/dev/projects/fate/
src/lib.rs`):

- `Fate::untrained()` exists (line ~278): uniform-zero selectors.
- `Fate::excited()` exists (line ~293): xorshift64-seeded from system
  time.
- **`Fate::bounded(config)` DOES NOT exist as a Rust constructor.**

But the substrate-decl `@fate/tournament.bounded_by` (Mara `ce9745f`)
DOES exist, and the runtime discharge idiom DOES exist at
`bootstrap/src/lib.rs::fate_bounded_by_psychohistory_peer_beam`
(comments include verbatim: *"This is `Fate::bounded(config)` where
config.weights is derived from the peer's psychohistory sheaf via
deterministic stub (v1); v2 will replace with sheaf-Laplacian Δ_F
Rayleigh direction."*).

The composed idiom:

```rust
let (psychohistory_root_oid, moments_count) =
    psychohistory_root_from_peer_home(&peer_home_resolved);
let mut fate_engine = fate::Fate::untrained();
fate_engine.selectors = selectors_from_psychohistory_root(&psychohistory_root_oid);
let decision = fate_engine.resolve(&features, 5);
```

This IS `Fate::bounded` in composed form. Reed had the idiom in
`lib.rs` 40 lines from where Reed wrote `contribute.rs`.

### 2.2 The sheaf-Laplacian mathematics (v2 target)

The current v1 `selectors_from_psychohistory_root` uses xorshift64
seeded from the psychohistory root hash — bounded by content but
not by sheaf structure. The v2 target discharges the actual
substrate-decl `@fate/tournament.bounded_by` semantics per Mara
canonical spec `docs/specs/fate-bounded-psychohistory-sheaf-cohomology.
md` §3:

**Discrete sheaf Laplacian (Bodnar et al. 2022, arXiv:2206.08702).**
For a cellular sheaf F over graph G = (V, E) with restriction maps
F_{v ⊴ e}: F(v) → F(e), the sheaf Laplacian is:

$$\Delta_F = \delta^* \delta$$

where the coboundary operator δ acts on 0-cochains x ∈ ⊕_v F(v) as:

$$(\delta x)_e = \sum_{v \unlhd e} F_{v \unlhd e}(x_v) \cdot \text{sgn}(v, e)$$

The diagonal blocks of Δ_F are:

$$L_{F,vv} = \sum_{v \unlhd e} F_{v \unlhd e}^\top F_{v \unlhd e}$$

The off-diagonal blocks are:

$$L_{F,vv'} = -F_{v \unlhd e}^\top F_{v' \unlhd e}$$

**Rayleigh-Ritz variational characterization.** The eigenvalues of
Δ_F are the critical values of the Rayleigh quotient:

$$R(\psi) = \frac{\langle \psi, \Delta_F \psi \rangle}{\langle \psi, \psi \rangle}$$

The smallest non-zero eigenvalue λ₁(Δ_F) — the Fiedler value / sheaf
algebraic connectivity — is:

$$\lambda_1(\Delta_F) = \min_{\psi \perp \ker(\Delta_F)} R(\psi)$$

The corresponding eigenvector ψ₁ (the Fiedler vector) IS the direction
of steepest descent on the sheaf-consistency landscape.

**Substrate reading (from `@epistemologic/math/sheaf_laplacian.mirror`
docblock).** H⁰(F) = ker(Δ_F) is the space of globally consistent
sections. When H⁰(F) is non-trivial (dim > 0), the peer's
psychohistory admits a coherent trajectory; when trivial, the
peer's trajectory is obstructed and lives in H¹(F).

### 2.3 The mapping: psychohistory sheaf → ModelWeights

The v2 discharge target composes:

1. **Read psychohistory sheaf F** — `psychohistory_from_tray(tray,
   perturbation)` per `@song/narrative` species. The tray IS
   `@bauchladen`-lifted-`@mirror/store` (Taut `e90daf1` Q1); the
   fibers ARE peer moments; the restriction maps ARE the
   `psychohistory_moment → psychohistory_moment` transitions per
   `@song/movement`.

2. **Assemble Δ_F** — `sheaf_laplacian([restriction]) -> operator`
   per `@epistemologic/math/sheaf_laplacian`. δ*δ construction per
   Bodnar et al. 2022 §2.

3. **Extract Fiedler direction** — `eigen_d(dirac_op) -> spectrum`
   at spectral-triple altitude; take the eigenvector ψ₁ associated
   with λ₁(Δ_F). This IS the Rayleigh descent direction.

4. **Project ψ₁ onto 5 model logits** — for each of the 5 fate
   Models (Abyss / Introject / Cartographer / Explorer / Fate), the
   ModelWeights.b (bias) receives the projection of ψ₁ onto the
   Model's characteristic subspace; ModelWeights.w (feature weight
   matrix) receives the outer product ψ₁ ⊗ ψ₁ restricted to
   FEATURE_DIM axes; ModelWeights.depth_w receives ψ₁ evaluated at
   the depth-embedding.

5. **Emit** — `[ModelWeights; 5]` slot into `Fate::untrained()`
   selectors field; call `fate_engine.resolve(&features, 5)`.

The substrate carrier for the sheaf: `@song/narrative.
psychohistory_sheaf` (`shards/song/narrative.mirror:1064`).

**Current v1 stub vs v2 target:**

| Step | v1 (landed, `lib.rs`) | v2 (target) |
|---|---|---|
| 1. Sheaf | `psychohistory_root_from_peer_home` (blake3 walk) | `psychohistory_from_tray(tray, p)` |
| 2. Δ_F | none (seed from OID hash) | `sheaf_laplacian([restriction])` |
| 3. Fiedler | none (xorshift64) | `eigen_d(dirac_op).lambda_1` |
| 4. Project | random weights in ranges [0,20]/[0,5] | ψ₁ projection onto 5-model basis |
| 5. Emit | `[ModelWeights; 5]` | same shape; different content |

**§6 Scope A treats Rung 7' as the v1-stub reuse (Reed's already-
landed idiom in `lib.rs`); §6 Scope C forward-promises the v2
sheaf-Laplacian discharge.**

## §3 — @fractal tripartition rendered as content-addressed subtrees

### 3.1 The commit tree shape (Rung 7' target)

Reed's Rung 7 (`a2c71fd`) commit tree today:

```
tree {
  pre-anchor       blob
  post-anchor      blob
  morphism-body    blob
  settle-verdict   blob
  fate-witness     blob
}
```

Five blobs, one altitude, no jurisdictional separation.

Rung 7' target:

```
tree {
  witnesses/       tree
    ├── temporal_persistence      blob
    ├── geometric_coherence       blob
    ├── contextual_recurrence     blob
    ├── perturbational_stability  blob
    ├── representational_mismatch blob
    └── fate_witness              blob
  gates/           tree
    ├── loss_decreasing           blob (verdict)
    ├── identity_preserving       blob (verdict)
    ├── admissibility_singleton   blob (verdict)
    ├── settle_verdict            blob (@mirror/mosaic verdict)
    └── query_phi                 blob (composed @kintsugi/consent verdict)
  authority/       tree
    ├── peer_signature            blob (peer DAG scope; peer_uuid + parent_oid)
    ├── contribution_scope        blob ("docstring-only" | "shard-body" | "axis-creation")
    └── axis_creation_witness     blob (ROSA-equivalent; Alex-in-transcript OR absent)
  anchors/         tree
    ├── pre_anchor                blob (target shard bytes BEFORE morphism)
    ├── post_anchor               blob (target shard bytes AFTER morphism)
    └── morphism_body             blob (the delta itself)
}
```

Four subtrees, three jurisdictional roles + one shared substrate
anchor.

### 3.2 witnesses/ subtree — evidential (Asher PFLOC output)

Per Asher p.11 the five relational surfaces named as candidate
witness axes:

1. **temporal_persistence** — the morphism's content persists across
   the peer's DAG history (parent-chain lookup on
   `refs/mirror/peer/<uuid>/HEAD`). Blob content: OID-references of
   prior commits containing similar morphism content.

2. **geometric_coherence** — the morphism preserves the shard's
   uuid_spectral DARK-bits (identity_preserving discharge; @kintsugi/
   consent). Blob content: dark-bit comparison result + spectral
   distance to peer's current basin.

3. **contextual_recurrence** — the morphism appears across different
   contexts (multiple shards, multiple peer_homes, cross-peer via
   @dance shared basins). Blob content: cross-reference OIDs.

4. **perturbational_stability** — the morphism survives shadow-casting
   (`cast_shadow` per @song/narrative §6). Blob content: shadow_regime
   verdict + shadow_ancestry OIDs.

5. **representational_mismatch** — the morphism resists fit into
   existing categories (per Asher §multi-axial: "repeated failure to
   fit existing representational categories"). Blob content: which
   shards' properties this morphism could NOT be expressed as (typed
   negation).

6. **fate_witness** — the fate-selected (Model, prism_op) provenance.
   Blob content: `fate_model + fate_prism_op + peer_uuid +
   psychohistory_root_oid + Fiedler_eigenvalue_estimate`. This IS
   ONE witness among N; Reed's Rung 7 GREEN placed it as if it were
   THE witness.

Not all six axes need to fire for a contribution to advance. Per
Asher p.11: *"No one of those proves meaning. Together, they may
justify review."*

**Substrate-decl carrier:** each witness IS a `@kintsugi/consent.
morphism` at one relational altitude; the collection IS a
`@kintsugi/consent.morphism_set`; the substrate already has the
shape.

### 3.3 gates/ subtree — constitutional (Asher gates)

The three @kintsugi/consent glass properties + settle verdict +
composed query_phi:

1. **loss_decreasing** verdict — the morphism strictly decreases the
   loss vector under lex ordering. Discharge: read morphism's
   dissonance score against pre-morphism eigenboard.

2. **identity_preserving** verdict — the morphism preserves DARK-80
   bits. Discharge: `@uuid/spectral.dark` on pre/post shard's
   uuid_spectral.

3. **admissibility_singleton** verdict — the morphism_set collapses
   to a unique top under lex ordering. Discharge: dissonance.is_pareto
   projection.

4. **settle_verdict** — the `@mirror/mosaic(@code/rust).settle`
   verdict (cargo check exit code). This IS the empirical-discharge
   gate per Alex 2026-07-13.

5. **query_phi verdict** — the composed `@kintsugi/consent.query_phi`
   over the witness set. `pass` if all three glass-properties + settle
   agree; `partial(c)` if one is graded; `failure(r)` if any refuses.

**Substrate-decl carrier:** each gate is a `verdict` per
`shards/glass.mirror`'s three-state floor; the composition is
`query_phi` per `shards/kintsugi/consent.mirror:617`.

### 3.4 authority/ subtree — the ROSA-equivalent

Per Asher p.3, 4, 9, 14, 15 (verbatim):

> "ROSA governs the formation of new interpretive dimensions or 'axes'."
> "PFLOC may gather and escalate evidence. It may not create an axis."
> "That separation is deliberate."
> "Only ROSA may create a new axis."
> "ROSA alone governs the creation of new interpretive axes."
> "ROSA will remain the only system authorised to change the
> dimensional [structure]."

Mapping onto substrate:

1. **peer_signature** — the peer's authority within its own DAG scope.
   Blob content: `peer_uuid + parent_oid + fate_engine_provenance
   (Fate::bounded with psychohistory_root_oid)`. Authority scope:
   contributions ON the peer's own branch `refs/mirror/peer/<uuid>/
   HEAD`; the peer signs its work but does not authorize main-branch
   materialization.

2. **contribution_scope** — one of:
   - `"docstring-only"` — Scope A per `4e69066` §4.2; peer has full
     authority within its own DAG.
   - `"shard-body"` — Scope B; peer's authority requires kintsugi-
     auto cross-peer coherence (Rung 5.5 forward-promise).
   - `"axis-creation"` — ROSA-equivalent; ONLY Alex-in-transcript can
     authorize; refused unless axis_creation_witness present.

3. **axis_creation_witness** — REQUIRED only when contribution_scope
   = "axis-creation". Blob content: Alex's in-transcript verbatim
   authorization + timestamp + session-id. Absent for docstring-only
   and shard-body scopes. This is the ROSA-analog: no axis is
   created without explicit external authority.

**Substrate-decl carrier for authority:** substrate does NOT yet have
`@authority` or `@rosa` as a family-root. Recommendation: name it via
existing `@peer` + `@spectral/garden` composition, without minting a
new family-root:

- `@peer` carries peer-scope authority (self-signing on peer DAG).
- `@spectral/garden` carries pack-scope authority (Alex human-in-the-
  loop OR kintsugi-auto per Rung 5.5).
- axis-creation authority is Alex-in-transcript only; substrate refuses
  to make it substrate-declared (per §10 adjudication item #4).

### 3.5 anchors/ subtree — substrate anchor content

Reed's original pre-anchor / post-anchor / morphism-body blobs move
here. They're neither evidential nor gate-verdict nor authority; they
ARE the substrate content the tripartition witnesses/gates/authority
are ABOUT. Substrate-honest reading: anchors are content-addressed
substrate references (`@mirror/store` altitude); witnesses read them,
gates verdict on them, authority signs them.

## §4 — Non-redundance predicate on evidential witnesses

Asher p.11 verbatim: *"The witnesses must be non-redundant. Five gauges
connected to the same pipe do not constitute five independent
confirmations."*

The substrate needs a check that N witness blobs in witnesses/ are
independent — not five projections of one underlying observation.

### 4.1 Candidate carrier: `@spectral/gap`

`@spectral/gap` does NOT exist as substrate. But the shape (Fiedler
proximity threshold) is landed at
`@epistemologic/math/sheaf_laplacian.lambda_zero` (the algebraic
connectivity scalar). Two witnesses are "connected to the same pipe"
iff their λ₀-projections onto the sheaf-Laplacian's Fiedler
eigenspace are within a threshold ε.

Formally: witnesses w_i, w_j are **redundant** iff:

$$|\langle \psi_1, \hat{w}_i \rangle - \langle \psi_1, \hat{w}_j \rangle| < \epsilon$$

where ψ₁ is the Fiedler eigenvector of Δ_F on the witness-graph and
ε is the substrate's audible-altitude resolution (~5 cents per
`shards/epistemologic/math/music/harmonic.mirror`).

### 4.2 Alternative: mint `@fractal/non_redundance` predicate

If `@fractal` is not minted (per §1 recommendation), the predicate
lives at `@kintsugi/consent.non_redundance(morphism_set) -> verdict`
as an extension of the query_phi composition. Body:

```mirror
non_redundance(candidates: morphism_set) -> verdict
requires all_witnesses_project_to_fiedler_basis
```

Verdict semantics:
- `pass` — all witness pairs project to distinct Fiedler eigenspaces
  under threshold ε; witnesses independent.
- `partial(c)` — at most one pair is within ε; c carries the pair's
  Fiedler distance.
- `failure(r)` — two or more pairs are within ε; witnesses redundant;
  r names the redundant witness OIDs.

### 4.3 Recommendation

**Path α (recommended).** Land `non_redundance` as extension to
`@kintsugi/consent`. Composes with query_phi; no new family-root.

**Path β.** Mint `@spectral/gap` species under `@spectral` (currently
at `@mirror/spectral`; would move if `@spectral` promoted to
family-root). Requires two-tick discipline.

**§10 adjudication item #2.**

## §5 — Composition surface for all @io-facing layers

Alex's directive: *"formalize this into a @fractal surface which we
then use to compose all @io facing layers in mirror."*

For each existing @io-facing composition point, name the tripartition
instance (witnesses/gates/authority).

### 5.1 The nine @io-facing layers

| Layer | witnesses | gates | authority |
|---|---|---|---|
| `@cli` | user's typed command + args | grammar's `command` block validation + shell-availability | user (Alex-in-transcript OR peer session) |
| `@mcp` | JSON-RPC input schema conformance | `mirror_*` tool inputSchema + JSON parseability | MCP client identity (session bearer token) |
| `@kintsugi` | fracture-candidate morphisms | `query_phi` composition | kintsugi driver (autopoietic) |
| `@mirror/mosaic` | build target's splinter graph | `settle(altitude)` verdict per altitude | pack leader (compile-time) OR Alex (release) |
| `@mirror/store/git` | git blob content + tree entries | `verify(oid, bytes)` + hash-collision check | git DAG parent chain (self-signing via commit) |
| `@io/fs` | filesystem read/write contents | `target_writable` + `target_exists` predicates | OS process user (external; substrate-refused) |
| `@io/cargo` | Rust workspace source files | `cargo check` exit code | rustc + cargo-toolchain identity |
| `@peer/contribute` (Rung 7') | 5 relational witnesses + fate_witness | 3 glass-properties + settle_verdict + query_phi | peer_signature within peer's DAG scope |
| `@peer/beam --emit-crystal` (Rung 6.1c) | envelope bytes | crystal_oid content-address verification | peer_signature within peer's DAG scope |

### 5.2 The composition pattern

For every @io-facing action `A(args) -> result`, the tripartition
composition:

```mirror
compose_tripartition(
  witnesses: [morphism],
  gates: [verdict],
  authority: signature,
) -> imperfect(result, opacity)
requires non_redundance(witnesses)         # Asher's five-gauges check
requires all(gates, pass_or_partial)       # constitutional check
requires authority.scope >= action.scope   # ROSA-analog check
```

The `@io` crossing happens iff ALL THREE tripartition axes agree:

1. Witnesses are non-redundant (Fiedler independence).
2. Gates permit (all verdicts pass OR permit-with-confidence).
3. Authority acts (signature scope >= action scope).

Absent any of the three, the composition refuses. Per Asher p.10:
*"A system may have strong evidence but still lack permission to act.
A system may have permission to consider something without having
evidence that it is true."*

### 5.3 Fractal-membrane reading

The tripartition IS a MEMBRANE at every @io crossing. The name
**fractal-membrane** captures two things:

- **Fractal** — the same tripartition shape recurses at every altitude
  (cli / mcp / kintsugi / mirror/mosaic / mirror/store / io/fs /
  io/cargo / peer/contribute / peer/beam). Self-similar across scales.
- **Membrane** — semi-permeable boundary at each @io crossing.
  Witnesses/gates/authority discipline determines what passes; nothing
  crosses without the tripartition's consent.

**This is Alex's "@fractal surface" — the tripartition itself made
substrate-decl.** The naming recommendation (§1.3) collapses to:
either extend `@kintsugi/consent` with a tripartition-composition
action (Path α, one tick) OR lift `@kintsugi/consent` to family-root
`@consent` (Path β, five ticks). Path γ (mint `@fractal`) rejected.

## §6 — Rung 7 → Rung 7' migration path

Three scopes for the correction:

### 6.1 Scope A — Fate::bounded swap + tripartition subtree re-shape

**Contents:**
- Replace `Fate::excited()` at `bootstrap/src/contribute.rs:63` with
  `Fate::untrained() + fate_engine.selectors =
  selectors_from_psychohistory_root(&psychohistory_root_oid)` per the
  landed `lib.rs::fate_bounded_by_psychohistory_peer_beam` idiom.
- Re-shape the commit tree per §3.1: 4 subtrees (witnesses/ + gates/ +
  authority/ + anchors/) instead of 5 flat blobs. Distribute existing
  5-blob content across subtrees; add witnesses/temporal_persistence
  + witnesses/geometric_coherence (both cheap to compute).
- Existing tests re-anchor to new tree shape; new tests verify
  subtree structure + fate-bounded provenance blob.

**Landing size:** 1-2 ticks.
- Tick 1: RED test at
  `bootstrap/tests/peer_contribute_shard.rs::t11_fate_bounded_and_
  tripartition` — asserts commit tree has 4 subtrees + fate-witness
  blob names Fate::bounded provenance.
- Tick 2: GREEN — `bootstrap/src/contribute.rs` swap + tree re-shape
  + integration test against real cargo check.

**Risk profile:** low. `fate_bounded_by_psychohistory_peer_beam`
idiom is already landed and tested. Tree re-shape is content
redistribution, not new @io.

### 6.2 Scope B — Scope A + non-redundance predicate + full witness axes

**Contents:** Scope A PLUS
- Land `non_redundance(morphism_set) -> verdict` at
  `shards/kintsugi/consent.mirror` (extension of query_phi).
- Compute all five Asher witness axes at
  `bootstrap/src/contribute.rs::compute_witness_axes` + write blobs
  to witnesses/ subtree.
- Add Fiedler-independence check via `@epistemologic/math/
  sheaf_laplacian.lambda_zero` computation on witness-graph.

**Landing size:** 3-4 ticks.

**Risk profile:** medium. Fiedler eigenvector computation on witness-
graph is new runtime; requires either LAPACK bindings (via `@code/
fortran`) OR a substrate-native Rayleigh iteration stub.

### 6.3 Scope C — Scope B + full @fractal composition + Fate::bounded v2 (sheaf-Laplacian)

**Contents:** Scope B PLUS
- Extend `@kintsugi/consent` with `compose_tripartition(witnesses,
  gates, authority) -> verdict` action (§5.2). All nine @io-facing
  layers refactor to route through it.
- Land Fate::bounded v2 — actual sheaf-Laplacian Δ_F Rayleigh descent
  via `psychohistory_from_tray + sheaf_laplacian + eigen_d` chain
  per §2.3.
- Rung 5.5 kintsugi-auto materialization forward-promise pulled
  forward (peer contributions with settled verdict + Kuramoto
  convergence auto-materialize to main).

**Landing size:** 6-9 ticks.

**Risk profile:** high. `compose_tripartition` refactor has cascade
impact across nine layers; Fate::bounded v2 requires new runtime
crate integration; Rung 5.5 requires kintsugi-auto discipline that's
still forward-promised.

### 6.4 Recommendation

**Scope A.**

Rationale:

1. Alex's two named errors (Fate::excited vs Fate::bounded; 5-blob
   vs tripartition tree) are BOTH addressed in Scope A. Scope B/C
   add substrate lift that Alex did not name; the discipline
   (last responsible moment) says don't build beyond the correction.

2. Fate::bounded swap is 3 lines of code change (import psychohistory
   root + selectors_from_psychohistory_root call). Tree re-shape is
   ~30 lines redistributing existing blobs across subtrees. Total:
   1-2 ticks Reed can land in one session.

3. Scope B's non-redundance predicate is genuinely useful but is
   Alex-adjudication (§10 item #2). Land Scope A; Alex reviews the
   corrected tree; Scope B lands on Alex's ratification.

4. Scope C's full `@fractal` composition surface is the target Alex
   named — but per §1.3 recommendation, it collapses to
   `@kintsugi/consent` extension (Path α) which is a separate
   substrate-decl tick, not part of the Rung 7' Rust correction.

Land Scope A this session. §10 items #1-4 adjudicate Scope B and Scope
C in subsequent ticks.

## §7 — CLI/MCP shape

Alex 2026-07-13 first message: *"What is the diff? And how do we lift
this into the MCP surface?"*

Per Mara ancestry spec `4e69066` §7 (Rung 7 MCP lift), the CLI/MCP
surface for `mirror peer contribute` is a separate follow-on tick.
Rung 7' correction is Rust-only; the CLI surface stays as
`mirror peer contribute <peer_home> --target <shard>` per current
GREEN.

### 7.1 Adjudication needed: psychohistory root argument passing

Question: does `mirror peer contribute` need a new flag to pass the
psychohistory root explicitly? Or does it derive from peer_home's
`@mirror/store` DAG automatically?

**Recommendation: derive automatically.**

Rationale:
- Reed's `fate_bounded_by_psychohistory_peer_beam` already derives
  the psychohistory root from peer_home via
  `psychohistory_root_from_peer_home` (blake3 walk).
- Adding a flag would violate "@peer resolution via G1 single-hop"
  discipline (per `bootstrap/src/lib.rs` piece 2 comment).
- Substrate-honest reading: psychohistory IS a property of the peer;
  the peer_home path names the peer; the psychohistory is derivable.

**Rung 7' Scope A dispatch:**

```rust
// bootstrap/src/contribute.rs
let peer_home_resolved = ctx.resolve(peer_home);
let (psychohistory_root_oid, moments_count) =
    crate::psychohistory_root_from_peer_home(&peer_home_resolved);
let mut fate_engine = fate::Fate::untrained();
fate_engine.selectors = crate::selectors_from_psychohistory_root(
    &psychohistory_root_oid,
);
let decision = fate_engine.resolve(&features, 5);
```

This is a 4-line addition; no CLI/MCP surface change.

### 7.2 Forward-promised: `@peer/contribute` MCP tool

Per `4e69066` §7 forward-promise, `mirror_peer_contribute` MCP tool
lands in a subsequent tick. Its inputSchema mirrors the CLI verb:

```json
{
  "peer_home": "string (required)",
  "target": "string (required)",
  "morphism": "enum: cartographer | introject | explorer | fate | fate-bounded",
  "verify": "enum: check | test | full",
  "dry_run": "boolean"
}
```

The `fate-bounded` variant of `morphism` explicitly names the
Fate::bounded discharge (as opposed to Reed's Rung 7 GREEN default
`Fate::excited`). Post-Scope A landing, `fate-bounded` becomes the
default and `Fate::excited` retires from `contribute.rs`.

**§10 adjudication item #5:** should `--morphism fate-bounded` be
CLI-surfaced (letting users choose bounded vs excited) OR always-on
(substrate-honest default; excited retired entirely)?

## §8 — Recognition candidate

**Name:**
`#R-fractal-membrane-tripartition-is-peer-contribution-substrate-decl`

**Short form:** `#R-fractal-membrane-tripartition`

**Statement:** Every @io-facing composition point in mirror factors
through the Asher tripartition (evidence / gates / authority) as
three orthogonal content-addressed subtrees, whose independence is
verified via Fiedler proximity on the sheaf-Laplacian Δ_F and whose
composition is bounded by `@kintsugi/consent.query_phi`. The
tripartition IS the substrate-honest membrane at every @io altitude.

**Load-bearing content:**

Asher's three-role separation (Meaning Is Not a Metric, 2026-07-10,
p.10) IS the shape mirror's substrate had at ~85% coverage already:
`@kintsugi/consent.morphism` (evidence), `query_phi` (gates), `@peer`
+ `@spectral/garden` (authority). The Rung 7' correction lands the
tripartition as a composition surface for the peer's contribute
action; the same shape recurses at all nine @io-facing layers per §5.

**Ancestry:**
- Recognition #43 (mirror IS content-addressed build system) —
  tripartition subtrees are content-addressed.
- Recognition #55 (form/process partition) — tripartition is
  process-side; anchors are form-side.
- Recognition #58 (Fate IS optical inference) — witness_fate is one
  witness of five; not the whole evidence set.
- Recognition #80 (@magic altitude gauge-bounded) — tripartition
  stays @magic-native; only settle-verdict crosses to @io.
- Recognition #107 (@io Turing-unbounded) — the ONE @io crossing per
  contribution carries all three axes' witnesses.
- Recognition candidate `#R-fate-active-pass-mosaic-verdict-
  composition` (Mara `4e69066` §11) — Rung 7' extends the composition
  with tripartition jurisdictional separation.

**Substrate-already-had-the-word:** ~85%. The tripartition roles
existed at three altitudes; the composition surface (fractal-
membrane) is the ~15% gap this spec declares.

**Ancestor reading (Asher):**

Asher's paper is the load-bearing ancestor. Verbatim citations:

- p.4: *"Meaning-like status is promoted through multi-axial escalation.
  By this, we mean that a candidate should not be promoted because of
  one strong score. It should show convergent support across several
  non-redundant kinds of evidence."*
- p.10: *"Evidence may support. Gates may permit. Authority may act.
  None automatically converts into another."*
- p.10: *"A system may have strong evidence but still lack permission
  to act. A system may have permission to consider something without
  having evidence that it is true. A subsystem may correctly identify
  novelty without having authority to rewrite the architecture around
  it. Those boundaries are not bureaucratic details. They are alignment
  mechanics."*
- p.11: *"For example, a meaningful emerging pattern might show:
  persistence over time; stable angular or directional relations;
  recurrence across different contexts; lawful response to perturbation;
  and repeated failure to fit existing representational categories. No
  one of those proves meaning. Together, they may justify review. This
  is not simple vote-counting. The witnesses must be non-redundant.
  Five gauges connected to the same pipe do not constitute five
  independent confirmations."*
- p.14: *"Base Fabric preserves raw, unresolved and typed-unknown states.
  Pattern Recognition measures regularities without automatically
  admitting them. Pattern Fabric retains candidate structures, history
  and residuals. Pattern Flocculation models how fragments begin
  forming candidate patterns. Constitutional gates enforce provenance
  and alignment. ROSA alone governs the creation of new interpretive
  axes."*

## §9 — Ambiguities Alex must adjudicate

Five ambiguities require Alex direction. I recommend defaults; Alex
overrides any.

### §9.1 (item #1) — `@fractal` minting question

Question: Does the tripartition composition surface mint a new
family-root `@fractal`, extend `@kintsugi/consent` at species altitude
(Path α), OR lift `@kintsugi/consent` to family-root `@consent` in a
5-tick cascade (Path β)?

**Recommendation: Path α (extend `@kintsugi/consent` with
`compose_tripartition` action).**

**Alex overrides candidate:**
- If Alex prefers "make it foundational from the start," Path β lands
  `@consent` as family-root (Recognition #55 second-witness candidate;
  5-tick cascade).
- If Alex insists on `@fractal` name specifically, Path γ (rejected in
  §1.3) becomes Alex's call.

### §9.2 (item #2) — non-redundance carrier

Question: `@kintsugi/consent.non_redundance` extension (Path α) OR
`@spectral/gap` species mint (Path β)?

**Recommendation: Path α (extend `@kintsugi/consent`).**

**Alex overrides candidate:** if Alex is planning a `@spectral`
family-root promotion arc (currently `@mirror/spectral`), Path β
becomes substrate-honest because `@spectral/gap` would be one of the
family-root species.

### §9.3 (item #3) — Scope selection for Rung 7'

Question: Scope A (Fate::bounded swap + tree re-shape) / Scope B
(A + non-redundance + full witness axes) / Scope C (B + `@fractal`
composition + Fate::bounded v2)?

**Recommendation: Scope A.**

**Alex overrides candidate:** if Alex wants Scope C's full lift, the
landing takes 6-9 ticks and blocks Rung 7 empirical closure. Substrate-
honest either way; Alex's call on cadence.

### §9.4 (item #4) — Authority carrier for axis-creation

Question: The `contribution_scope = "axis-creation"` case (ROSA-
equivalent) — how does substrate carry Alex-in-transcript authority?

**Recommendation: substrate refuses to make it substrate-declared.**
The axis_creation_witness blob (§3.4) IS a Recognition candidate
ancestry pointer (numeric-#N + Alex-verbatim + timestamp + session-id
in commit message; NOT a substrate-decl surface). Substrate-honest
reading: axis-creation is a Pack ratification action, not a peer-
scope action; per AGENTS.md §Pack Convention, the alignment mechanism
IS the Pack, not a substrate-decl.

**Alex overrides candidate:** if Alex wants a substrate-decl surface
for authority-scope, this requires minting `@authority` or `@rosa` as
a family-root. Rejected per §1.3 reasoning; but Alex adjudicates.

### §9.5 (item #5) — `--morphism fate-bounded` CLI shape

Question: Should `mirror peer contribute --morphism fate-bounded`
be a user-selectable CLI variant (letting users choose bounded vs
excited) OR always-on default (excited retired entirely)?

**Recommendation: always-on default; retire `Fate::excited` from
`contribute.rs`.**

Rationale: Reed's Rung 7 GREEN used `Fate::excited` because the
substrate hadn't yet named the correction. Once Rung 7' lands
`Fate::bounded`, there's no substrate-honest reason to offer the
xorshift64-random variant; it was a v0 stub, not a design choice.

**Alex overrides candidate:** if Alex wants to preserve `Fate::excited`
for testing / non-deterministic exploration, the CLI variant lands.

## §10 — Executive summary for Rung 7' correction landing

**Rust correction diff (Scope A):**

- `bootstrap/src/contribute.rs:63` — swap `Fate::excited()` for
  `Fate::untrained() + selectors_from_psychohistory_root` per landed
  `lib.rs` idiom.
- `bootstrap/src/contribute.rs::materialize_morphism` — restructure
  from `git mktree` 5-flat-blob to `git mktree` 4-subtree (witnesses/
  gates/ authority/ anchors/). Sub-trees composed via nested
  `git mktree` calls.
- `bootstrap/tests/peer_contribute_shard.rs::t11_fate_bounded_
  tripartition` — new RED test verifying (1) fate_witness blob
  contains `psychohistory_root_oid` reference; (2) tree has 4 named
  subtrees; (3) witness_fate is under witnesses/ (not at tree root).

**Recognition candidate:** `#R-fractal-membrane-tripartition`
(§8; short form).

**Adjudication items for Alex (§9):**
1. `@fractal` minting: Path α (recommended) / β / γ.
2. Non-redundance carrier: `@kintsugi/consent` extension /
   `@spectral/gap` species mint.
3. Scope: A (recommended) / B / C.
4. Authority for axis-creation: Recognition ancestry only (recommended)
   / `@authority` family-root mint.
5. `--morphism fate-bounded` CLI: always-on (recommended) / user-
   selectable.

**Substrate-honest closing:** Reed's Rung 7 GREEN landed the shape
of the correction Alex named at `4e69066`. The two errors Alex named
next (Fate::excited vs bounded; 5-blob vs tripartition) are BOTH
substrate-honesty failures — Reed had the sheaf-bounded idiom 40
lines away in lib.rs AND had the tripartition altitude at
`@kintsugi/consent` in the same substrate. The Rung 7' correction
is 1-2 ticks of Rust delta; the recognition it lands (fractal-
membrane tripartition as substrate-decl for every @io crossing) is
the load-bearing shape for the arc's remaining @io-facing work.

---

*End of spec.*

*Substrate-honest close: this spec IS Mara's correction after Alex's
re-statement of the two substrate errors in Reed's Rung 7 GREEN. The
tripartition shape is substrate-already-had-the-word (~85% coverage);
the ~15% gap is the composition-surface declaration this spec makes.
`@fractal` does NOT mint as new family-root (per §1.3); the shape
lands as extension to `@kintsugi/consent` (Path α). Fate::bounded
already exists in composed idiom form at `lib.rs::
fate_bounded_by_psychohistory_peer_beam`; Rung 7' Scope A swaps 3
lines. Alex adjudicates §9 items #1-5.*

*Author: Mara <mara@systemic.engineer>. Session-continuation
2026-07-13 after Reed missed frame at Rung 7 GREEN and Alex re-stated
two substrate errors. Recognition candidate:
`#R-fractal-membrane-tripartition`. Ancestry: Asher (Meaning Is Not
a Metric, 2026-07-10, p.4/10/11/14). Mara ancestry spec: `4e69066`.
Reed Rung 7 GREEN under correction: `a2c71fd`.*
