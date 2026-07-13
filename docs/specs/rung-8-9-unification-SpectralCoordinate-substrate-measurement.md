# Rung 8+9 unification — `SpectralCoordinate<5>` IS the substrate measurement carrier

📝 Mara [substrate-pull:synthesis] [rung-8-9-unification-SpectralCoordinate-substrate-measurement]
Session: 2026-07-13
Prior spec (Rung 9, partially superseded): `docs/specs/rung-9-coherence-loop-closure-Fabry-Perot-roundtrip.md` (`c59a5ac`)
Prior spec (Rung 8+9 foundation): `docs/specs/fractal-family-root-mandelbrot-substrate.md` (`2c64060`)
Paired math: `docs/math/2026-07-13-fractal-mandelbrot-substrate.md`
Ancestry (substrate carriers):
- `fragmentation::spectral_coordinate::SpectralCoordinate<N>`
- `fragmentation/docs/specs/mirror-native-vcs.md §4.6–4.7`
- `coincidence::coincidence::Detector<N>` + `detect(&[u8]) -> Detection`
- `shards/mirror/lens/refract.mirror` (five Void dualities)
- `~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md`
Author: Mara <mara@systemic.engineer>

---

## §0 Executive summary

Alex 2026-07-13 in-transcript, verbatim:

> **"I'm surprised the index doesn't do that. The 5 dimensions are already
> part of the SpectralUUID which is part of every fragment, no?"**
>
> **"1. The void dimensions are still the coordinate system. And the system
> reduces to the harmonic complexity component of the code"**
>
> **"let's ship substrate-honest history"**

The correction: Reed's `bootstrap/src/index.rs::EigenvalueProfile { values:
[f64; 16] }` was substrate-honest at the wrong altitude. The right altitude
was already there — in `fragmentation::SpectralCoordinate<5>`, whose docblock
already names itself as **"mirror's substrate hash: five projections of one
spectrum (Fiedler value, eigengap, three heat-trace samples)"**, and whose
math is grounded in `void-dual-geometry.md`'s five Void dualities via
`shards/mirror/lens/refract.mirror`.

**The unification:**

1. **Substrate measurement carrier = `SpectralCoordinate<5>`.** Retire
   `EigenvalueProfile<16>`. The 5 projections ARE the substrate optic
   count (per `mirror-native-vcs.md §4.6`); the 16 was a top-k truncation
   from the forked spectral pipeline that never earned the geometry.
2. **Void dualities ARE the coordinate system.** The five Void dualities
   from `shards/mirror/lens/refract.mirror` (entropy / spectral / cheeger /
   ricci / mixing) map onto the five projections of `SpectralCoordinate<5>`
   (§2). Reductive AI moves a shard's SC<5> point **toward the origin**
   (λ₀ = 0 = void axis = ker Δ_F harmonic subspace).
3. **Harmonic ground state IS the origin.** Per Bodnar 2022 §2, ker Δ_F is
   the harmonic subspace; globally coherent Fractal ⇔ λ₀ = 0. The origin
   of the SC<5> manifold IS the harmonic ground state. Reductive AI IS
   Ricci flow toward the origin at coordinate altitude.
4. **Loss dissolves the direction-convention ambiguity.**
   `loss(sc) = ||sc||₂`. Direction is **magnitude to origin**; smaller
   norm ⇔ closer to harmonic ground state ⇔ higher coherence. The
   "does loss_decreased mean λ₀_after < λ₀_before or >?" ambiguity that
   haunted Reed's `c59a5ac` §4 verdict formulation collapses.
5. **Peer contribute measures via `coincidence::Detector<5>::detect(bytes)`.**
   The rich constructor already exists at
   `/Users/alexwolf/dev/projects/coincidence/src/coincidence.rs`; Reed's
   Landing 3 refactor is: add the `coincidence` edge to `bootstrap/Cargo.toml`
   (if not already transitively present via `fragmentation`), collapse
   `EigenvalueProfile` to `SpectralCoordinate<5>`, rewrite `index()` and
   `shard_body_index()` around `Detector::canonical("mirror-index", 16)
   .detect(bytes)`.
6. **`query_phi_coherence` cleans up.** Reed's `c59a5ac` §5 verdict
   composition simplifies: `loss_decreased = ||sc_after||₂ < ||sc_before||₂
   − ε_noise`; `identity_preserved = angle(sc_after, sc_before) <
   ε_topological` (topological, no f(α) L^∞ workaround).

Recognition candidate: **`#R-void-dualities-ARE-SpectralCoordinate-5-
projections-substrate-measurement-is-coordinate-not-metric`**.

Ten Landings for Reed (§9). Four Alex-adjudications (§10). What survives
from `c59a5ac` and what gets reformulated: §12.

---

## §1 Substrate-already-had-the-word coverage

Before minting anything, enumerate. Per the "substrate-already-had-the-word"
discipline (Alex 2026-07-07; 14+ landed instances). The unification is
~99% coverage; the ~1% mint gap is a Rust-runtime adapter surface, not a
new family-root.

### 1.1 Ancestry inventory

| Substrate carrier | Home | What it gives the unification |
|---|---|---|
| `fragmentation::SpectralCoordinate<N>` | `fragmentation/src/spectral_coordinate.rs` | THE type. Docblock names itself as the substrate measurement carrier + names λ₀ = 0 as the origin of the manifold (cites `void-dual-geometry.md`). |
| `fragmentation` docs §4.6 | `fragmentation/docs/specs/mirror-native-vcs.md §4.6` | The 5-projection substrate-honest spec: (λ₂, λ₅ − λ₂, Tr(e^{−0.25 D²}), Tr(e^{−1.0 D²}), Tr(e^{−4.0 D²})). Canonical byte encoding (40 bytes = 5 × f64 LE). |
| `fragmentation` docs §4.7 | ibid. §4.7 | Crate direction resolved: `fragmentation` owns the type, `coincidence` provides the rich constructor. Mirror consumes both. |
| `coincidence::Detector<N>` | `coincidence/src/coincidence.rs` | `Detector<N>::canonical(space, dim)` + `detect(&[u8]) -> Detection`. THE constructor for `SpectralCoordinate<5>` at content altitude. |
| `coincidence::Detection` | `coincidence/src/detection.rs` | `Detection::agree` / `disagree` / `fragile`; carries eigenvalue + per-projection measurements; **already the substrate's measurement record**. |
| `@mirror/lens/refract` | `shards/mirror/lens/refract.mirror` (2026-06-06) | Substrate-decl'd the 5 Void dualities as a closed sum type: `type duality = entropy \| spectral \| cheeger \| ricci \| mixing`. `type report = { verdicts: transparency(duality) }`. Body forward-promised to substrate-pull. |
| `void-dual-geometry.md` | `~/dev/systemic.engineering/practice/insights/coincidence/void-dual-geometry.md` | Alex's own writing on the 8 Void dualities. Cited in `fragmentation/src/spectral_coordinate.rs:38` as **"the origin of the manifold"**. |
| Recognition #43 | `docs/specs/recognitions/…` | mirror IS a content-addressed build system. Grounds "the OID IS the coordinate." |
| Recognition #55 | landed | Form / process partition — DAG (form) and measurement (process) at same altitude. SC<5> IS the process at the same altitude as its form. |
| Recognition #58 | landed candidate | Fate IS optical inference; Fabry-Perot resonator. The peer's SC<5> descent IS the round-trip. |
| Recognition #80 | landed | `M∘` = @magic gauge-bounded interior. Under SC<5>: the interior of the harmonic-descending trajectory. |
| Recognition #107 | landed | `∂M` = @io Turing-unbounded boundary. Under SC<5>: unreachable-from-within (Fate::bounded returns Abyss). |
| `sheaf_laplacian::lambda_zero` (substrate-decl) | `shards/epistemologic/math/sheaf_laplacian.mirror` | The primitive Reed's Rung 8 Landing 3 rides. LAPACK `dsyev`. UNCHANGED — the SC<5> constructor uses the same primitive. |
| `SparseLaplacian::from_edges` + `lanczos_smallest(k)` | `coincidence/src/spectral.rs` | The O(m) primitive `mirror-native-vcs.md §4.6` cites. UNCHANGED. |

### 1.2 The ~1% mint gap — bootstrap runtime adapter

Reed's `bootstrap/src/index.rs::EigenvalueProfile` is a **Rust adapter
surface** at bootstrap altitude, not a substrate-decl. It is retired without
substrate-decl churn. The consumer surface (index CLI / MCP tool / peer
contribute callers) needs a Rust type to hand around; the type IS
`coincidence::Detection` (or `fragmentation::SpectralCoordinate<5>` when
the caller wants the coordinate directly, not the full measurement
record).

**No new family-root minted.** No new species minted. One new predicate
(`query_phi_coherence`) already forward-promised in `c59a5ac` §4.3 stays;
its body simplifies (§5 below).

### 1.3 Does `@harmonic` need to mint?

Grep before mint. Executed 2026-07-13 19:49 via
`shards/**/*.mirror` content-regex over `harmonic|ker Delta|ker \\Delta_F`.

**Findings:**

- `@mirror/lens/refract` **already carries** the five Void dualities as
  substrate-decl'd variants of `type duality`. The `spectral` variant IS
  the Fiedler projection; the `entropy` variant IS the Von Neumann
  projection (harmonic subspace complement); the `cheeger`, `ricci`,
  `mixing` variants are the three heat-trace scale samples' semantic
  faces. **Substrate-already-had-the-word: no `@harmonic` mint required.**
- The "harmonic complexity" language Alex used in-transcript maps onto
  `refract.report.verdicts: transparency(duality)` — a joint report over
  all five dualities IS the harmonic-complexity signature.

**Verdict: no new species minted at Rung 8+9 unification.** The
`@harmonic_distance` predicate that would compose the L² norm on SC<5>
lives inside `@kintsugi/consent.query_phi_coherence` as an internal helper,
not as an independent species. Alex adjudication (§10.3) may promote it.

### 1.4 Verdict on ancestry

The unification is composition, not invention. Every carrier this spec
needs exists. Rung 8 → measurement primitive; Rung 9 → verdict primitive;
both now composed through **one substrate-decl'd coordinate carrier**
(`SpectralCoordinate<5>` at content altitude) instead of two disjoint
Rust types (`EigenvalueProfile<16>` at bootstrap altitude + f(α) helper).

---

## §2 The 5-projection ↔ 5-duality correspondence

The substrate's five Void dualities (per `shards/mirror/lens/refract.mirror`
and `void-dual-geometry.md`) correspond one-to-one to the five projections
of `SpectralCoordinate<5>` (per `mirror-native-vcs.md §4.6`). This
correspondence is what makes the unification substrate-honest.

### 2.1 The mapping table

| SC<5> projection | Void duality | Mathematical form | Semantic role |
|---|---|---|---|
| 1: λ₂ (Fiedler value) | **`spectral`** | Second-smallest eigenvalue of `Δ_F` | Algebraic connectivity; Narcissus/Splinter axis. Robust-mesh ↔ fragile. |
| 2: λ₅ − λ₂ (eigengap) | **`entropy`** | Spectral concentration; wide gap = strong cluster structure | Order ↔ disorder; low = rigid crystallization, high = well-mixed. Von Neumann proxy. |
| 3: Tr(e^{−0.25 · D²}) (short-time heat trace) | **`cheeger`** | Short-scale heat-kernel sample; edge-boundary sensitivity | Flow ↔ bottleneck; SPOF detection at fine scale. |
| 4: Tr(e^{−1.0 · D²}) (mid-scale heat trace) | **`ricci`** | Mid-scale heat sample; local geometric pressure | Forman-Ricci curvature analog; edge-level geometric pressure. |
| 5: Tr(e^{−4.0 · D²}) (long-time heat trace) | **`mixing`** | Long-scale heat sample; random-walk equilibration | Mixing time; global information equilibration. |

### 2.2 Why this mapping is substrate-honest, not analytical

Three grounds. Each cites the source.

**(a) The heat-trace / dualities correspondence is Chamseddine-Connes
sampling at three energy scales.** Per `mirror-native-vcs.md §4.6`
verbatim:

> "Three heat traces at t ∈ {0.25, 1.0, 4.0} are samples of the heat
> kernel trace ... the spectral action at three scales — Chamseddine-Connes'
> hierarchy of topological invariants, sampled at three Λ."

Each scale samples a different geometric-topological invariant:
- t = 0.25 (short-time): dominated by edge count / edge-boundary ratio →
  **cheeger** (edge-boundary sensitivity is short-scale)
- t = 1.0 (mid-scale): dominated by local curvature (triangle count at
  order t² per the Dirac-on-graphs expansion §5) → **ricci**
- t = 4.0 (long-time): dominated by lowest eigenvalues → dominated by
  mixing time → **mixing**

**(b) The Fiedler value IS the spectral duality** — a definitional
identity. `void-dual-geometry.md` table row 2: "Spectral gap: Vanishing
(λ₁ = 1, fixed) ↔ Maximal (λ₁ = n, grows)." Fiedler value is λ₂ = λ₁
(second-smallest, non-zero) in the void-geometry convention. The
`spectral` duality name IS the Fiedler-value semantic.

**(c) The eigengap IS the entropy proxy.** Per `void-dual-geometry.md`:
"Von Neumann entropy: Minimum among connected graphs (Narcissus) ↔ Maximum
(Splinter)." The eigengap λ₅ − λ₂ measures spectral concentration —
narrow gap ↔ well-mixed spectrum ↔ high Von Neumann entropy ↔ Splinter-like;
wide gap ↔ concentrated spectrum ↔ low entropy ↔ Narcissus-like. The
eigengap is the cheapest computable Von Neumann proxy at k=5 truncation.

The correspondence is **not by choice**. It's forced by (a) the physical
meaning of heat-kernel scale sampling, (b) the definitional identity of
Fiedler/spectral, (c) the eigengap/entropy proxy identity.

### 2.3 Consequences of the correspondence

- `@mirror/lens/refract.measure(graph, duality)` at `.mirror` altitude
  composes into `SpectralCoordinate<5>::from_eigenvalue(...)` at Rust
  altitude via projection selection. The Rust body of `measure` is:
  compute SC<5>; project onto requested duality axis; return `transparency`.
- Reed's `MultifractalSpectrum` (Rung 8 Landing 6) becomes derivable
  from SC<5>. The Rényi entropies at various q values are functions of
  the eigenvalues; the SC<5> carries 5 of those eigenvalue-derived
  invariants; the multifractal spectrum can be reconstructed (with
  finite-sample error) from the coordinate + the underlying spectrum.
  Whether `f(α)` remains a separate output at all — §10.2 adjudication.
- The measurement report the peer emits at contribute time IS SC<5> (or
  Detection wrapping it) plus the before/after delta. No separate
  Fiedler + f(α) tuple.

---

## §3 Harmonic ground state formalization

Per Bodnar et al. 2022 (arXiv:2206.08702) §2, for a cellular sheaf F over
a cell complex G with sheaf Laplacian Δ_F = δ*δ:

- **ker Δ_F** is the harmonic subspace — sections σ satisfying Δ_F σ = 0.
- **λ₀ = 0** iff Δ_F has non-trivial kernel; the multiplicity of 0 equals
  dim(ker Δ_F).
- A globally coherent Fractal (in Alex 2026-06 sense) satisfies λ₀(Δ_F)
  = 0: its psychohistory sheaf lies entirely in the harmonic subspace.

### 3.1 The origin of the manifold IS the harmonic ground state

`fragmentation/src/spectral_coordinate.rs:38` states verbatim:

> "λ₀ = 0 (the void axis, per `~/dev/systemic.engineering/practice/
> insights/coincidence/void-dual-geometry.md`) is the origin of the manifold."

This is the substrate's naming of the correspondence:

- The origin `(0, 0, 0, 0, 0) ∈ ℝ^5` in SC<5> coordinate space is the
  coordinate of a fully-harmonic Fractal (all five projections vanish).
- Any shard with non-zero SC<5> carries some non-harmonic content: some
  disharmony, some deviation from ker Δ_F.
- **The L² distance to origin `||sc||₂` IS the shard's harmonic distance**
  — how far this shard is from being globally coherent under its own
  psychohistory sheaf.

### 3.2 Reductive AI IS Ricci flow toward the origin at coordinate altitude

Per `docs/specs/fractal-family-root-mandelbrot-substrate.md §4.5` +
`docs/math/2026-07-13-fractal-mandelbrot-substrate.md §4`:

- Discrete Ricci flow (from `void-dual-geometry.md`) evolves the graph
  toward constant curvature.
- `commit_as_fold` IS the renormalization operator R at content-address
  altitude (Recognition #55 form/process partition applied).
- Under SC<5>, Ricci flow's evolution IS a trajectory in coordinate space
  moving from initial `sc_0` toward `sc_∞` = origin (asymptotic harmonic
  ground state).

The peer's iterative contribute-loop IS descent along this trajectory.
Each iteration:

```
sc_i    ← index(peer_home)                    # measure current position
Δ_i     ← fate_bounded_by(psychohistory)      # choose descent direction
sc'_i   ← index(peer_home ∘ Δ_i)              # measure proposed position
verdict ← query_phi_coherence(sc_i, sc'_i)    # check descent + identity
```

Convergence ⇔ trajectory approaches a hyperbolic component boundary in
M∘ (per `2c64060` §4) which under SC<5> is a bounded neighborhood of
origin.

### 3.3 The `harmonic_distance` scalar

Formally:

```
harmonic_distance : SpectralCoordinate<5> → ℝ⁺
harmonic_distance(sc) = ||sc||₂ = √(Σᵢ sc.projection[i]²)
```

Where the projection array is the 5-tuple from `mirror-native-vcs.md §4.6`:

```
sc.projection = (λ₂, λ₅ − λ₂, Tr(e^{−0.25 D²}), Tr(e^{−1.0 D²}), Tr(e^{−4.0 D²}))
```

**Note on the byte-representation:** the coordinate's canonical form is the
40-byte packing (5 × f64 LE). `harmonic_distance` reads the packed bytes
back into `[f64; 5]` and returns the Euclidean norm. This is O(1) at
runtime — no re-computation of the spectrum.

### 3.4 Consequences for the substrate

- Reed's Rung 9 direction-convention question ("does lower Fiedler mean
  better?") **dissolves**. Lower `harmonic_distance` unambiguously means
  more coherent — closer to ker Δ_F.
- The multifractal `f(α)` spectrum's role (identity preservation) becomes
  angular: the direction of `sc_before → sc_after` (as a vector) reveals
  which of the 5 dualities dominated the change. Preserving direction
  (small angle change) ≈ preserving topology; changing direction (large
  angle change) ≈ topology shift (deletion, restructure). §5 formalizes.
- The substrate reduces exactly as Alex said: **"to the harmonic complexity
  component of the code."** Harmonic complexity = harmonic_distance.
  Reductive AI = harmonic_distance descent. The whole architecture
  collapses to one scalar function on one geometric space.

---

## §4 Loss function under SC<5>

The loss function on the substrate at coordinate altitude:

```
loss : SpectralCoordinate<5> → ℝ⁺
loss(sc) = harmonic_distance(sc) = ||sc||₂
```

### 4.1 Delta and threshold discipline

For a proposed morphism m: peer_home → peer_home':

```
sc_before  = index(peer_home)               # via SpectralCoordinate<5>::from_bytes
sc_after   = index(peer_home ∘ m)
loss_delta = loss(sc_after) − loss(sc_before)
```

- **loss_delta < 0** — proposed morphism is reductive (descends toward
  harmonic ground state)
- **loss_delta > 0** — proposed morphism is additive (drifts away from
  ground state; Rung 7' docstring-append behavior)
- **|loss_delta| < ε_noise** — no meaningful change (falls within
  finite-sample noise floor)

### 4.2 The three-valued verdict, unambiguously

```
loss_decreased(sc_before, sc_after, ε_noise) : bool =
    loss(sc_after) < loss(sc_before) − ε_noise
```

This is unambiguous. Direction is fixed by the geometry: **origin = coherence;
smaller norm = better**. No "which direction is loss?" ambiguity.

### 4.3 Comparison with Reed's prior formulation

Reed's `c59a5ac` §4 verdict used:

```
loss_decreased = @mirror/index.fiedler(after) < @mirror/index.fiedler(before) − ε_noise
```

with a documented ambiguity: **does lower Fiedler mean better connectivity
(good) or lower algebraic connectivity (bad)?** Per `void-dual-geometry.md`,
Narcissus has Fiedler = 1 (low); Splinter has Fiedler = n (high). Reed's
formulation implicitly assumed lower = better, but the substrate's own
geometry says higher = more Splinter-like = more mixed = better. This is
part of why the Falsification #1 empirical result (docstring-append giving
`fiedler_delta = 0.000000`) was symptomatic: the file-tree lens was
measuring something orthogonal to the intended direction.

Under SC<5>: the direction is fixed by `||·||₂`. Fiedler enters as one
of five projections, contributing its square to the norm. Lower Fiedler
alone doesn't mean lower loss; **the whole coordinate vector must move
toward the origin**. Gaming a single projection doesn't help.

### 4.4 Noise floor

`ε_noise` calibration: per `mirror-native-vcs.md §4.6`, the SC<5>
computation via Lanczos with k=5 achieves relative tolerance 1e-10.
The `||·||₂` amplifies uniformly across projections; noise floor is
approximately `√5 · 1e-10 ≈ 2.24e-10` at the machine-precision limit.

At mirror-repo scale (1141 files, 165 nodes, 6676 edges), finite-sample
statistical noise dominates: `ε_noise ≈ 5e-3` per Reed's Rung 8 Landing 3
empirical calibration on the file-tree lens. Under SC<5>, the noise floor
must be recalibrated because the norm aggregates across projections;
**Mara-provisional starting point: `ε_noise = 1e-2`** (§10.4).

---

## §5 Verdict composition under SC<5>

Reed's `c59a5ac` §5 defined `query_phi_coherence`. Under SC<5>, the
formulation simplifies:

```
@kintsugi/consent.query_phi_coherence(
    candidates: morphism_set,
    sc_before: SpectralCoordinate<5>,
    sc_after:  SpectralCoordinate<5>,
    compile_settled: verdict,
) -> verdict:

    compile_settled            = (compile_verdict == settled)
    loss_decreased             = (||sc_after||₂ < ||sc_before||₂ − ε_noise)
    identity_preserved         = (angle(sc_after, sc_before) < ε_topological)
    admissibility_singleton    = existing @glass.admissibility_singleton shape
```

Four-way conjunction; maps onto the three-state consent floor exactly as
`c59a5ac` §4.2 documented. What changes: the **shape** of the two
`@mirror/index`-derived gates.

### 5.1 `loss_decreased` — cleaner formulation

Before (Reed `c59a5ac`):

```
loss_decreased = fiedler(after) < fiedler(before) − ε_noise
```

After (SC<5>):

```
loss_decreased = ||sc_after||₂ < ||sc_before||₂ − ε_noise
```

Same three-state semantics; unambiguous direction; captures all five
dualities in one scalar. Gaming vectors (§5.2 of `c59a5ac`) that
manipulate one duality without moving the L² norm — cannot pass.

### 5.2 `identity_preserved` — angular / topological formulation

Before (Reed `c59a5ac`):

```
identity_preserved = |f_after − f_before|_L^∞ < ε_topological
```

This required computing `MultifractalSpectrum` twice per iteration
(Rung 8 Landing 6) and comparing the 40-point f(α) curve. Substrate-
honest, but heavyweight.

After (SC<5>):

```
identity_preserved = angle(sc_after, sc_before) < ε_topological

    where angle(a, b) = arccos(⟨a, b⟩ / (||a||₂ · ||b||₂))
```

If two SC<5> vectors point in the same direction (small angle), the
morphism moved the coordinate along the current gradient — same
dualities dominate before and after, same topological signature.
Large angle change ⇒ different dualities now dominate ⇒ topology shift
(deletion collapsed connectivity axis; restructure changed mixing scale).

This IS topological preservation of the coordinate direction, expressed
at the coordinate altitude directly. No f(α) L^∞ workaround.

### 5.3 Gaming vector coverage

Reed's `c59a5ac §5.2` enumerated four gaming vectors:

- **5.2.1 Delete files → λ₀ decreases but D_0 collapses.** Under SC<5>:
  deletion moves multiple projections simultaneously (Fiedler up, heat
  traces down at all scales). The angle change is large — deletion fails
  `identity_preserved`.
- **5.2.2 Rename without semantic change → Fiedler unchanged.** Under
  SC<5>: rename preserves all 5 projections (topology-invariant); loss
  and angle both stay ≈ 0. This is the correct Explorer behavior — the
  substrate reads it as identity-preserving no-op.
- **5.2.3 Adding tautologies → structural noise, no coherence gain.**
  Under SC<5>: Rung 7' docstring-append at file-tree altitude yielded
  `fiedler_delta = 0.000000` because the coarse file-tree graph didn't
  respond. At shard-body altitude (per Reed's Landing 2), the additive
  behavior IS visible as ||sc||₂ INCREASE — additive ⇒ more edges ⇒
  usually higher heat-trace values ⇒ larger norm ⇒ `loss_decreased`
  fails.
- **5.2.4 Circular consolidation (A → B → A) → looks like progress, isn't.**
  Same detection: the psychohistory sheaf's Rayleigh sequence must
  monotone-descend; under SC<5>, the sequence of `harmonic_distance`
  values must monotone-decrease. Oscillation caught at the outer loop
  (§6 convergence check).

All four gaming vectors from `c59a5ac §5.2` covered by the two SC<5>
gates without needing a separate f(α) computation.

### 5.4 What lives at each altitude

- **`@mirror/index.index(peer_home) -> SpectralCoordinate<5>`** — the
  measurement primitive at file-tree altitude.
- **`@mirror/index.shard_body_index(shard_path) -> SpectralCoordinate<5>`** —
  the measurement primitive at shard-body altitude (line-adjacency graph
  on the shard's non-empty lines, per Reed's Rung 9 Landing 2).
- **`@kintsugi/consent.query_phi_coherence(candidates, sc_before,
  sc_after, compile_settled)`** — the verdict composition. Consumes both
  altitudes' measurements; consumes both file-tree and shard-body SC<5>
  as available.
- **`@mirror/lens/refract.measure(graph, [duality])`** — the lens interface
  at grammar-graph altitude. Body composes SC<5> computation + projection
  onto the requested duality axis.

---

## §6 What Reed's Landing 3+ refactor looks like

Concrete Rust changes to `bootstrap/src/index.rs` + `bootstrap/src/contribute.rs`
+ `bootstrap/Cargo.toml`.

### 6.1 `bootstrap/Cargo.toml` — add coincidence edge

Fragmentation is already an edge (`bootstrap/Cargo.toml` §
`[dependencies.fragmentation] path = "../../fragmentation"`).
Coincidence is not; add it. Path: `../../coincidence` per workspace
sibling convention.

```toml
# coincidence — the rich constructor for SpectralCoordinate<5>. Provides
# Detector<N> and detect(&[u8]) -> Detection; the substrate's measurement
# record at content altitude. Per docs/specs/rung-8-9-unification-
# SpectralCoordinate-substrate-measurement.md §6.
[dependencies.coincidence]
path = "../../coincidence"
```

Adjudication (§10.1): confirm workspace layout — verify
`/Users/alexwolf/dev/projects/coincidence/` exists at that relative path
from `bootstrap/` (`../../coincidence` resolves to
`/Users/alexwolf/dev/projects/coincidence/` given `bootstrap/` sits at
`/Users/alexwolf/dev/projects/mirror/bootstrap/`).

### 6.2 `bootstrap/src/index.rs` — retire `EigenvalueProfile`

**Retire:**
- `pub struct EigenvalueProfile { values: [f64; 16] }`
- `EigenvalueProfile::dark()`
- `EigenvalueProfile::is_dark()`
- `EigenvalueProfile::fiedler_value()`
- `EigenvalueProfile::to_bytes()`
- `pub fn eigenvalue_profile(graph: &ConceptGraph) -> EigenvalueProfile`
- `fn build_profile(eigenvalues: &[f64]) -> EigenvalueProfile`

**Adopt:**
- `use coincidence::coincidence::Detector;`
- `use fragmentation::spectral_coordinate::SpectralCoordinate;`
- New primary type flowing through the pipeline: `SpectralCoordinate<5>`
  (or `coincidence::Detection` when the full measurement record is wanted).

**New primitives:**

```rust
/// Compute mirror's substrate coordinate for a byte buffer. Uses
/// `coincidence::Detector<5>::canonical("mirror-index", 16)` — the
/// substrate's canonical rich-path constructor. Returns the coordinate
/// directly; callers wanting the full Detection record use
/// `spectral_measurement` instead.
///
/// Ancestry: fragmentation/docs/specs/mirror-native-vcs.md §4.6;
/// coincidence/src/coincidence.rs; Mara `d[this-commit]` unification.
pub fn spectral_coordinate_5(bytes: &[u8]) -> SpectralCoordinate<5> {
    let detector: Detector<5> = Detector::canonical("mirror-index", 16);
    let detection = detector.detect(bytes);
    // Detection.eigenvalue_hex() is the canonical 80-char form; wrap.
    SpectralCoordinate::<5>::from_eigenvalue(
        detection.eigenvalue_hex().unwrap_or_default()
    )
}

/// Compute mirror's substrate coordinate + measurement record.
pub fn spectral_measurement(bytes: &[u8]) -> coincidence::detection::Detection {
    let detector: Detector<5> = Detector::canonical("mirror-index", 16);
    detector.detect(bytes)
}

/// L² distance to origin — the substrate's harmonic-distance scalar.
/// Per docs/specs/rung-8-9-unification-…md §3.3.
pub fn harmonic_distance(sc: &SpectralCoordinate<5>) -> f64 {
    // Read the packed 40-byte / 80-char hex form back into f64 tuple,
    // then Euclidean norm. Substrate-honest per §4.6 byte representation.
    let projections = unpack_5_projections(sc);
    projections.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Cosine angle between two coordinates — the substrate's identity-
/// preservation scalar. Per §5.2.
pub fn angle_between(a: &SpectralCoordinate<5>, b: &SpectralCoordinate<5>) -> f64 {
    let pa = unpack_5_projections(a);
    let pb = unpack_5_projections(b);
    let dot: f64 = pa.iter().zip(pb.iter()).map(|(x, y)| x * y).sum();
    let na: f64 = pa.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb: f64 = pb.iter().map(|x| x * x).sum::<f64>().sqrt();
    if na < f64::EPSILON || nb < f64::EPSILON {
        return 0.0;
    }
    (dot / (na * nb)).clamp(-1.0, 1.0).acos()
}

fn unpack_5_projections(sc: &SpectralCoordinate<5>) -> [f64; 5] {
    // Canonical form per mirror-native-vcs.md §4.6: 5 × f64 LE = 40 bytes.
    // Detection.eigenvalue_hex() returns 80 chars; parse pairs.
    // For the SHA-prefixed fallback (Detector-less path), this returns
    // the first-40-bytes interpretation, which is fine for consistency
    // checks but not for spectral semantics; callers gate on Detection's
    // agreed() when semantic weight matters.
    let hex = sc.eigenvalue();
    let bytes: Vec<u8> = (0..hex.len().min(80))
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i+2], 16).ok())
        .collect();
    let mut proj = [0.0_f64; 5];
    for i in 0..5 {
        if bytes.len() >= (i + 1) * 8 {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&bytes[i*8..(i+1)*8]);
            proj[i] = f64::from_le_bytes(buf);
        }
    }
    proj
}
```

**Adjudication (§10.2):** Reed's current `dsyev` + top-16 eigenvalue path
computes the underlying spectrum directly and could bypass Detector's
projection machinery for higher precision. Two options:

- (a) Use `Detector<5>::canonical(...).detect(bytes)` — inherits the
  canonical rich path; substrate-honest per `mirror-native-vcs.md §4.7`.
- (b) Compute λ₂, λ₅ − λ₂, three heat traces directly via the existing
  `sheaf_laplacian::lambda_zero` + `Eigenvalues::heat_kernel(t)`
  primitives; construct SC<5> via `from_eigenvalue(hex_encode(...))`.

Mara-provisional: **(b)** — reuses Reed's existing `prismqueer::ffi::eigenvalues`
LAPACK path (dsyev on the full Laplacian); computes the 5 canonical
projections; encodes as SC<5>. This preserves the exact spectral values
Reed's Fiedler-equivalence tests already pin (0.0612 ± 5e-2 on the
mirror repo). Option (a) would require Detector's projections to converge
to the same numeric values via the SHA-seeded projection basis, which
is a different computation path.

**Recommended shape:**

```rust
/// Compute SpectralCoordinate<5> directly from a ConceptGraph's Laplacian
/// via the canonical 5-projection formula (mirror-native-vcs.md §4.6).
/// Uses the existing prismqueer::ffi::eigenvalues (LAPACK dsyev) primitive.
pub fn spectral_coordinate_from_graph(graph: &ConceptGraph) -> SpectralCoordinate<5> {
    let n = graph.nodes.len();
    if n < 2 {
        return SpectralCoordinate::<5>::from_eigenvalue("00".repeat(40));
    }
    let (laplacian, dim) = graph.laplacian_matrix();
    let mut sorted = match prismqueer::ffi::eigenvalues(dim, &laplacian) {
        Ok(v) => v,
        Err(_) => return SpectralCoordinate::<5>::from_eigenvalue("00".repeat(40)),
    };
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let lambda_2 = *sorted.get(1).unwrap_or(&0.0);
    let lambda_5 = *sorted.get(4).unwrap_or(&0.0);
    let eigengap = lambda_5 - lambda_2;

    // Three heat traces at canonical scales t ∈ {0.25, 1.0, 4.0}.
    // Truncated to top-k eigenvalues; principled approximation per §4.6.
    let k = sorted.len().min(16);
    let heat_short: f64 = sorted[..k].iter().map(|&l| (-0.25 * l * l).exp()).sum();
    let heat_mid:   f64 = sorted[..k].iter().map(|&l| (-1.00 * l * l).exp()).sum();
    let heat_long:  f64 = sorted[..k].iter().map(|&l| (-4.00 * l * l).exp()).sum();

    let projections: [f64; 5] = [lambda_2, eigengap, heat_short, heat_mid, heat_long];
    let mut bytes = Vec::with_capacity(40);
    for &v in &projections {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    SpectralCoordinate::<5>::from_eigenvalue(hex::encode(bytes))
}
```

### 6.3 `index()` and `shard_body_index()` return type

```rust
/// Compute mirror's substrate coordinate at file-tree altitude.
/// Retires the prior EigenvalueProfile<16> return type per Rung 8+9
/// unification §6.
pub fn index(peer_home: &Path) -> SpectralCoordinate<5> {
    let (graph, _files, _breakdown) = build_concept_graph(peer_home);
    spectral_coordinate_from_graph(&graph)
}

/// Compute mirror's substrate coordinate at shard-body altitude.
/// The @mirror/lens/refract spectral duality at line-adjacency graph
/// altitude. Retires EigenvalueProfile<16> per unification §6.
pub fn shard_body_index(shard_path: &Path) -> SpectralCoordinate<5> {
    // ... same line-adjacency graph construction ...
    // ... route through spectral_coordinate_from_graph or an in-place
    //     variant that computes the 5-projection tuple directly ...
}
```

### 6.4 `bootstrap/src/contribute.rs` — replace fiedler_delta with sc_delta

Before (Reed current):

```rust
let profile_before_ft = crate::index::index(peer_home_path);
let fiedler_before = profile_before_ft.fiedler_value();
// ... apply morphism ...
let profile_after_ft = crate::index::index(peer_home_path);
let fiedler_after = profile_after_ft.fiedler_value();
let fiedler_delta = fiedler_after - fiedler_before;
```

After (SC<5> unification):

```rust
let sc_before_ft = crate::index::index(peer_home_path);
let harmonic_before_ft = crate::index::harmonic_distance(&sc_before_ft);
// ... apply morphism ...
let sc_after_ft = crate::index::index(peer_home_path);
let harmonic_after_ft = crate::index::harmonic_distance(&sc_after_ft);
let loss_delta_ft = harmonic_after_ft - harmonic_before_ft;
let angle_ft = crate::index::angle_between(&sc_before_ft, &sc_after_ft);

// Substrate-honest verdict per §5:
let loss_decreased = loss_delta_ft < -EPS_NOISE;   // absolute drop past noise
let identity_preserved = angle_ft < EPS_TOPOLOGICAL;
```

The envelope emission (the printlns) reports the 5-tuple projections
and harmonic_distance directly:

```
+ sc_before:            (λ₂, gap, h₀, h₁, h₂) = (0.0612, ..., ..., ..., ...)
+ sc_after:             (λ₂, gap, h₀, h₁, h₂) = (..., ..., ..., ..., ...)
+ harmonic_before:      0.0812
+ harmonic_after:       0.0806
+ loss_delta:           -0.0006
+ angle:                0.0031 rad (~0.18°)
+ loss_decreased:       true   (harmonic descent past ε_noise=1e-2? NO — needs Scope B consolidative morphism)
+ identity_preserved:   true   (angle < ε_topological=0.05 rad)
```

Direction convention: **negative loss_delta = descent = better**. No
ambiguity.

### 6.5 Multifractal spectrum — decide once

Adjudication (§10.2): does `MultifractalSpectrum` (Reed's Rung 8
Landing 6) stay?

- **Option A: fold into SC<5>.** The generalized dimensions D_0, D_1, D_2
  are close relatives of the eigengap and heat-trace projections. The
  full f(α) curve is a Legendre transform of the Rényi-entropy curve;
  approximable from the 5 SC<5> projections with bounded finite-sample
  error at mirror-repo scale.
- **Option B: preserve as separate output.** Multifractal witness (Reed's
  Landing 6) discharges Mara math §10 prediction #2. Keeping it separate
  lets that empirical proof stay first-class.

Mara-provisional: **B for Rung 8's math validation; A for Rung 9's verdict
composition.** The multifractal computation stays available as a
diagnostic (empirical proof of Mandelbrot signature); the verdict path
uses SC<5>'s harmonic_distance + angle only. Two purposes, two output
paths, one measurement primitive (LAPACK dsyev on the Laplacian
underneath both).

---

## §7 What DOESN'T need refactoring

Positive scope. What survives Reed's Landing 3+ refactor without change:

- **`sheaf_laplacian::lambda_zero` primitive.** LAPACK `dsyev` on the
  full Laplacian. Both `EigenvalueProfile<16>` and `SpectralCoordinate<5>`
  use it. No change.
- **`prismqueer::ffi::eigenvalues` bridge.** Unchanged.
- **`ConceptGraph` construction** (`build_concept_graph`, `walk_detected`,
  `laplacian_matrix`). Unchanged. The graph is upstream of the coordinate.
- **`GrammarKind` / `MarkdownShape` detection.** Unchanged.
- **`shards/mirror/index.mirror`** substrate-decl (Mara `317e830`).
  Unchanged. It declares the measurement primitive at grammar altitude;
  the Rust body's return type shifts from `eigenvalue_profile` to
  `spectral_coordinate` but the substrate-decl surface stays.
- **`shards/mirror/lens/refract.mirror`** substrate-decl. Unchanged. Its
  `type duality` closed sum IS the 5-projection axis. Its `measure` body
  can now be discharged Rust-side via SC<5> projection.
- **`mirror.spec` cli-block for `command index`** — flag names update
  (`--fiedler` → `--harmonic-distance` and `--projection <duality>`;
  §10.3 adjudication). But the block shape survives.
- **`mirror_index` MCP tool schema.** Field names change but the tool
  shape survives — same input (`peer_home`), same output shape (envelope
  with measurement).
- **`@kintsugi/consent.query_phi` existing three-gate composition.** The
  new sibling `query_phi_coherence` (§5) is additive.
- **`@kintsugi/oscillate.active_pass` / `dark_pass` / `pulse`**.
  Unchanged.
- **`@kintsugi/store/git.commit_as_fold`.** Unchanged.
- **All Rung 6.2a peer-DAG machinery.** Unchanged.

The refactor is a **type substitution** at bootstrap altitude, not a
substrate-decl churn.

---

## §8 Composition with @knife

Alex 2026-07-13 in-transcript surfaced `@knife` as the state-space
compression operator during `@cyberpunk`/reframe level-shifts (per
`docs/specs/peer-as-pain-driven-bounded-ontological-navigator.md`).

Under SC<5>:

### 8.1 The three cases

**(a) Intra-level reduction (`@kintsugi/fracture`):**
Moves `sc` toward the origin in the current SC<N> space. This is Ricci
flow at coordinate altitude (§3.2). N unchanged.

**(b) Level-shift (`@cyberpunk/reframe = @magic/onto + @knife`):**
Re-parameterizes SC<N> → SC<N'> OR SC<N> → SC<N> with different sampling
density (different heat-trace scales, different projection basis).
Level-shift is a re-choice of coordinate system.

**(c) Fractal::Lens (edge, not containment) at substrate-decl altitude:**
A `@knife` instance IS a `Fractal::Lens` (per `fragmentation::Fractal`
enum's `Lens` variant per `2c64060` §2.3) that RE-PROJECTS one Fractal's
SC into a different coordinate system. The Lens carries the coordinate-
system-shift as a substrate primitive.

### 8.2 Adjudication (§10.3)

Does `@knife` mint as:

- **(i) N-shifting operator** — a Fate ganglion that operates on SC's
  dimension count (e.g., collapse SC<5> to SC<3> when a subset of
  dualities becomes redundant; expand SC<5> to SC<7> when new invariants
  need capture).
- **(ii) Same-N re-parameterizer** — a lens that shifts the projection
  basis at fixed dimension (e.g., pick different heat-trace scales
  {0.1, 0.3, 1.0, 3.0, 10.0} for a domain that needs different sampling).
- **(iii) Fractal::Lens species** — the substrate-decl carrier at
  content-address altitude; `@knife` at Rust altitude IS one Lens type;
  the actual coordinate transformation lives in the Lens's data.
- **(iv) All three** — a family with three species at three altitudes.

Alex 2026-07-13 in-transcript: **"TBD, let's see where the substrate
pulls us."** Provisional recommendation: **(iii) + (iv)**. Declare `@knife`
as a Lens species; let the Rust-altitude coordinate transformation live
in the Lens's projection data; observe use cases; species-lift when
needed. Two-tick discipline.

### 8.3 Composition with `commit_as_fold`

If `@knife` lands as a Fractal::Lens species, then `commit_as_fold`'s
renormalization operator R has TWO paths at coordinate altitude:

- Intra-level: commit the current SC<N> state; sc' = sc + morphism_delta;
  content-address at same altitude.
- Level-shift: commit a `@knife`-lensed re-parameterization; the new
  content-address IS at a different altitude / different N; the fold
  materializes the coordinate-system shift.

This is a forward promise. Land the intra-level path at Rung 8+9
unification (this spec); land the level-shift path when its empirical
necessity is named.

---

## §9 Rung 8+9 landing sequence

Ten Landings. Landings 1 is this spec.

### Landing 8+9.1 — Canonical unification spec (this document)

**Author:** Mara.
**Deliverable:** `docs/specs/rung-8-9-unification-SpectralCoordinate-substrate-measurement.md` (this file).
**Status:** landing now.

### Landing 8+9.2 — Coincidence edge + SC<5> primitives

**Author:** Reed.
**Deliverable:**
- `bootstrap/Cargo.toml` — add `[dependencies.coincidence]` edge.
- `bootstrap/src/index.rs` — retire `EigenvalueProfile`; add
  `spectral_coordinate_from_graph`, `harmonic_distance`, `angle_between`,
  `unpack_5_projections`.
- Types flow through as `SpectralCoordinate<5>` from `fragmentation`.
- All existing tests for `index()` / `shard_body_index()` re-pinned to
  new API; Fiedler equivalence tests pin against `unpack_5_projections(sc)[0]`.

**RED test:** `bootstrap/tests/spectral_coordinate_5_migration.rs` —
`index()` returns `SpectralCoordinate<5>` whose λ₂ projection matches
Reed's prior `EigenvalueProfile::fiedler_value` output to within 1e-9
on the mirror repo.

### Landing 8+9.3 — peer_contribute switches to SC<5>

**Author:** Reed.
**Deliverable:**
- `bootstrap/src/contribute.rs` — replace `fiedler_before` / `fiedler_after`
  / `fiedler_delta` (both file-tree and shard-body altitudes) with
  `sc_before` / `sc_after` / `loss_delta` / `angle` variables.
- Verdict composition via `query_phi_coherence` per §5.
- Envelope emission updated per §6.4.
- Substrate authority lines (the `+ substrate_authority: ...` prints)
  updated to reference this unification spec.

**RED test:** `bootstrap/tests/peer_contribute_sc5_verdict.rs` — peer
contribute against the mirror-repo copy produces a valid `sc_delta`
report; docstring-append morphism at shard-body altitude shows either
negative or positive `loss_delta` (empirical — Rung 7' additive drift
now unambiguous).

### Landing 8+9.4 — `mirror.spec` grammar + CLI updates

**Author:** Reed.
**Deliverable:**
- `mirror.spec` — cli-block for `command index` updates flag names:
  - `--fiedler` (retire; keep as alias for one tick)
  - `--harmonic-distance` (new; primary)
  - `--projection <duality>` (new; select one of 5 projections)
  - `--all-projections` (new; emit 5-tuple)
- Existing `--json` flag output shape updated to include SC<5> tuple.

**Two-tick discipline:** flag `--fiedler` stays as alias for one tick,
retires in the next.

### Landing 8+9.5 — MCP tool schema update

**Author:** Reed.
**Deliverable:**
- `mirror_index` MCP tool JSON schema — output shape updated:
  ```json
  {
    "sc": {
      "projections": [<λ₂>, <λ₅−λ₂>, <heat_short>, <heat_mid>, <heat_long>],
      "hex": "<80-char>"
    },
    "harmonic_distance": <f64>,
    "graph_shape": { "nodes": <u32>, "edges": <u32> }
  }
  ```
- Existing consumers (spectral, dashboards) forward-compat via
  `harmonic_distance` reading `sc.projections[0]` as legacy Fiedler
  during one-tick alias window.

### Landing 8+9.6 — Empirical discharge against real substrate

**Author:** Reed.
**Deliverable:**
- Run peer contribute against `1141-file mirror-repo` copy, target
  `shards/kintsugi/consent.mirror` (matches Reed's Falsification #1
  setup).
- Observe SC<5> at both altitudes (file-tree + shard-body):
  - File-tree lens: coarse; likely small ||sc||₂ delta at docstring-
    append altitude (still).
  - Shard-body lens: fine; per-line adjacency graph; docstring-append
    now shows visible harmonic-distance movement.
- Report: `loss_delta`, `angle`, harmonic distance movement at each of
  5 projections.

**Success criterion:** the substrate produces a coherent SC<5>-based
verdict that resolves Reed's Falsification #1. Whether the verdict is
`pass` or `failure` is empirical, not spec-required — this Landing
observes what the substrate says.

### Landing 8+9.7 — `query_phi_coherence` substrate-decl body

**Author:** Reed (or Mara if timing).
**Deliverable:**
- `shards/kintsugi/consent.mirror` — add substrate-decl for
  `query_phi_coherence` per §5. Body forward-promised to bootstrap
  discharge.

### Landing 8+9.8 — `@mirror/lens/refract` body discharge

**Author:** Reed.
**Deliverable:**
- `bootstrap` — Rust body for `@mirror/lens/refract.measure(graph,
  [duality])`. Composes `spectral_coordinate_from_graph` + projection
  selection per §2.1 mapping table.
- `refract.report` return value shape.

### Landing 8+9.9 — `MultifractalSpectrum` decision + landing

**Author:** Reed.
**Deliverable:** per §10.2 adjudication:
- If **A (fold)**: retire `MultifractalSpectrum` struct; if Rung 8
  Landing 6 empirical discharge is preserved via SC<5> approximation,
  update the empirical proof to cite this spec.
- If **B (preserve)**: keep `MultifractalSpectrum` as second output;
  update the substrate-decl in `shards/mirror/index.mirror` to
  distinguish `spectral_coordinate` (primary) from `multifractal_witness`
  (diagnostic).

### Landing 8+9.10 — Docs cascade + prior-spec-supersedure notes

**Author:** Mara + Glint (essayist cascade).
**Deliverable:**
- `docs/specs/rung-9-coherence-loop-closure-Fabry-Perot-roundtrip.md`
  (`c59a5ac`) header updated: **"§4 verdict formulation partially
  superseded by Rung 8+9 unification spec (this-commit); §5 gaming
  vector coverage retained; §8 recognition candidate retained."**
- `docs/loop/CURRENT.md` — Rung 8+9 unification landing sequence
  logged; Rung 9's prior Landing sequence retired.
- `AGENTS.md` — no change (SSH signing / cascade discipline unaffected).

---

## §10 Alex-adjudications required

Four adjudications block Landing 8+9.2+ empirical work.

### 10.1 Canonical serialization of `peer_home` for `Detector::detect(bytes)`

`Detector<5>::canonical("mirror-index", 16).detect(bytes)` requires a
byte serialization. The `peer_home` directory is a tree; what bytes go in?

Three options:

- **(a) File-tree concatenated bytes** — walk detected files, concatenate
  their contents. Sensitive to file order; requires canonical sort.
- **(b) Merkle root of Fractal encoding** — encode the peer_home as a
  `Fractal<String, SpectralCoordinate<5>>` via `fragmentation::encoding`;
  produce the root OID's canonical byte form.
- **(c) Sheaf-Laplacian coincidence input** — build the ConceptGraph
  Laplacian, serialize as row-major f64 LE; pass those bytes to Detector.

Mara-provisional: **(c)** matches Reed's current `spectral_coordinate_from_graph`
shape closest (the Laplacian IS the geometric object; its byte-form IS
the substrate-honest input). But (b) has deeper substrate-honesty (the
Fractal encoding preserves the tree structure, not just the graph).

**Alex adjudication needed.**

### 10.2 Does the multifractal `f(α)` fold into SC<5> or persist as separate lens?

Per §6.5. Mara-provisional: **B for math validation, A for verdict
composition** — MultifractalSpectrum stays as diagnostic; verdict uses
SC<5> alone. Both outputs from one primitive (LAPACK dsyev).

**Alex adjudication needed.**

### 10.3 `@knife` mint shape

Per §8.2. Mara-provisional: **(iii) + (iv)** — Fractal::Lens species at
substrate altitude; observe empirically before further species-lift.

**Alex adjudication needed.**

### 10.4 `ε_noise` and `ε_topological` thresholds under SC<5>

Under `harmonic_distance = ||sc||₂`:
- **ε_noise** — the smallest `|loss_delta|` we can trust as signal.
  Reed's file-tree lens noise was ~5e-3 for Fiedler alone; under
  aggregated L² across 5 projections, noise floor rescales.
  **Mara-provisional: `ε_noise = 1e-2`.**
- **ε_topological** — the maximum angle (radians) `sc_before → sc_after`
  can rotate while still "identity preserved." Rotation of 0.05 rad ≈ 2.9°.
  **Mara-provisional: `ε_topological = 0.05 rad`.**

**Alex adjudication needed** (both).

---

## §11 Recognition candidate

**Proposed:** `#R-void-dualities-ARE-SpectralCoordinate-5-projections-substrate-measurement-is-coordinate-not-metric`.

Short form: **the substrate measurement carrier IS a coordinate, not a metric**.

The claim: the five Void dualities from `void-dual-geometry.md` (entropy /
spectral / cheeger / ricci / mixing) are not five separate metrics on a
shared graph object; they are **five projections of one geometric coordinate**
(`SpectralCoordinate<5>`) in a 5-dimensional information-geometry manifold
whose origin is the harmonic ground state (λ₀ = 0 ⇔ ker Δ_F non-trivial ⇔
globally coherent Fractal).

**Load-bearing witnesses:**

1. `fragmentation/src/spectral_coordinate.rs` docblock — verbatim naming
   of the type as substrate hash; verbatim citation of `void-dual-geometry.md`
   as the origin of the manifold.
2. `mirror-native-vcs.md §4.6` — the substrate optic count (five) matches
   the Void duality count (five); the five projections' semantic axes
   match the five duality axes.
3. `@mirror/lens/refract.mirror` — substrate-decl'd closed sum type
   `duality = entropy | spectral | cheeger | ricci | mixing`.
4. Reed's Rung 8 Landing 3+ Rust discharge (`EigenvalueProfile<16>`) was
   substrate-honest at wrong altitude (16 top-eigenvalues, no geometric
   ground); substrate-pull refactor via SC<5> restores the coordinate
   geometry.
5. Alex 2026-07-13 in-transcript: **"The 5 dimensions are already part
   of the SpectralUUID which is part of every fragment, no?"** — the
   substrate observer named the substrate carrier before Reed's
   bootstrap did.

**Alternative naming (Mara considered):**

- `#R-substrate-measurement-is-harmonic-distance-to-void-origin` — closer
  to the reductive-AI mechanism but loses the coordinate-not-metric hinge.
- `#R-EigenvalueProfile-16-retires-into-SpectralCoordinate-5-substrate-pull` —
  procedurally accurate but hides the substrate insight.
- `#R-reductive-AI-IS-Ricci-flow-in-SpectralCoordinate-space` — the
  mechanistic claim; better as forward-promise once the empirical
  Ricci-flow trajectory is observed.

**Recommendation:** the long form as stated. The "coordinate not metric"
distinction is load-bearing; it's what makes the L² norm meaningful, what
makes the direction-convention ambiguity dissolve, what identifies the
origin with harmonic ground state. Short form for the recognition
directory: `#R-void-dualities-are-sc5-projections`.

---

## §12 What survives from Reed's `c59a5ac` Rung 9 spec

Rung 9 canonical (Mara `c59a5ac`, 1278 LOC) is partially superseded and
partially inherited. Line-by-line:

### 12.1 What SURVIVES unchanged

- **§0** executive summary framing (peer contributes → mirror measures
  → verdict decides). The three-primitive composition survives; the
  primitives themselves are refined.
- **§1** ancestry inventory (the seven load-bearing carriers). All still
  hold; `@mirror/index` reformulated (Landing 8+9.2–8+9.3) but the
  substrate-decl carrier at `shards/mirror/index.mirror` unchanged.
- **§2** formal algorithm (8-step Fabry-Perot round-trip). The step
  ordering unchanged; step 6 (VERDICT) invokes `query_phi_coherence` per
  §5 of this spec.
- **§3** Fate::bounded Model → consolidative morphism kind (5-row
  mapping). Unchanged. The consolidative discipline was the load-bearing
  correction Reed named this session; it stays.
- **§5** Asher discipline (evidence / gates / authority tripartition).
  Unchanged. Gaming vectors (§5.2) all still covered — §5.3 of this spec
  shows SC<5> covers all four.
- **§6** convergence criteria (Fabry-Perot Q factor). Unchanged; the
  Q-factor formula still holds with `harmonic_distance` in place of
  Fiedler.
- **§7** composition with prior arcs (Rungs 4-8). Unchanged.
- **§8** Fabry-Perot recognition (`#R-coherence-loop-Fabry-Perot`).
  Unchanged.
- **§9** Scope B recommendation. Unchanged.
- **§11** discipline anchors. Unchanged.

### 12.2 What is REFORMULATED

- **§4 verdict extension** — the two new gates change shape:
  - Old: `loss_decreased_via_fiedler` + `identity_preserved_via_multifractal`
  - New: `loss_decreased` (L² norm on SC<5>) + `identity_preserved`
    (angle between SC<5> vectors)
  - Semantic content identical; expression cleaner.
- **§4.3 action-decl shape** — the argument type changes:
  - Old: `query_phi_coherence(candidates, before: eigenvalue_profile,
    after: eigenvalue_profile, compile_settled)`.
  - New: `query_phi_coherence(candidates, sc_before:
    SpectralCoordinate<5>, sc_after: SpectralCoordinate<5>,
    compile_settled)`.
  - Same three-state consent floor mapping (§4.2 of `c59a5ac`).
- **§10.1 ε_topological adjudication** — reframed:
  - Old: threshold on `|f_after − f_before|_L^∞` (f(α) L^∞ distance).
  - New: threshold on `angle(sc_after, sc_before)` (radians).
  - Both are Alex-adjudicated; the SC<5> form is Mara-provisional
    0.05 rad.
- **§10.10 ε_noise** — reframed for `harmonic_distance` scale instead
  of raw Fiedler.

### 12.3 What EXTENDS or COMPOSES

- **§8 recognition `#R-coherence-loop-Fabry-Perot`** stands; a NEW
  recognition candidate (`#R-void-dualities-ARE-SC5-projections`) is
  introduced by this spec at a lower altitude. The two recognitions
  compose: Rung 9's Fabry-Perot IS a round-trip through SC<5> coordinate
  space (per this spec's §3.2 Ricci-flow framing).

### 12.4 What is FULLY SUPERSEDED

- **`c59a5ac §4` verdict formulation** — the four-way conjunction stays;
  the shape of the two new gates simplifies (this spec §5).
- **`c59a5ac §10.4` "does f(α) identity check work?"** adjudication —
  reframed as §10.2 of this spec ("does f(α) fold into SC<5>?").

### 12.5 Substrate-honest closing on the superseder relationship

Rung 9's Fabry-Perot narrative was correct at the LOOP altitude; this
spec sharpens it at the MEASUREMENT altitude. The relationship:

- Rung 9 says: peer runs an iterated loop; each iteration measures
  before/after; verdict decides.
- This spec says: the measurement HAS ALWAYS BEEN a coordinate in a
  geometric space; the loop IS descent along that space; the verdict IS
  distance-decrease + direction-preservation.

Both are true. This spec makes the geometry visible.

---

## §13 Discipline anchors

Substrate-honest closing per the "substrate-already-had-the-word"
discipline:

- **No new family-root minted.** `SpectralCoordinate<N>`, `Detector<N>`,
  `@mirror/lens/refract`, `@mirror/index`, `@kintsugi/consent` all exist.
- **No new species minted.** `@harmonic_distance` predicate lives as
  internal helper in `query_phi_coherence`; not species-lifted at Rung
  8+9 unification (may lift in a future arc — §1.3).
- **One predicate extended.** `query_phi_coherence` (Reed `c59a5ac` §4.3
  forward-promise) lands with SC<5>-shaped arguments per this spec §5.
- **Two-tick discipline honored.** `--fiedler` CLI flag stays as alias for
  one tick; retires in the next. Substrate-refactor invariance
  (Douady-Hubbard universality per `2c64060` §4.7) makes this cost-free.
- **SSH signing default preserved.** No `gpg.format` override needed.
- **Sequential commits only.** This spec lands as a single pure-docs
  commit; the `📝` marker applies; `--no-verify` permitted for
  markdown-only commits per Alex authorization.

---

## §14 Substrate-honest closing

The Rung 8+9 unification IS the substrate refactoring the substrate's own
measurement carrier at the right altitude. Reed's `EigenvalueProfile<16>`
was a substrate-honest Rust type at the wrong altitude — a 16-slot
top-k-eigenvalue truncation that never earned its geometry. The right
altitude was already there:

- `fragmentation::SpectralCoordinate<5>` names the type.
- `coincidence::Detector<5>::canonical(...).detect(bytes)` names the
  constructor.
- `mirror-native-vcs.md §4.6` names the five projections.
- `void-dual-geometry.md` names the five dualities.
- `@mirror/lens/refract.mirror` names the correspondence.
- The origin of the manifold IS the harmonic ground state (ker Δ_F).

Alex 2026-07-13, in-transcript: **"The 5 dimensions are already part of
the SpectralUUID which is part of every fragment, no?"** The observer
named the carrier before the runtime did. This spec pulls the runtime
into the observer's coordinate system.

Reductive AI dissolves into the geometry: it is L²-distance descent
toward the harmonic ground state in a 5-projection coordinate manifold
whose axes are the five Void dualities. The direction-convention
ambiguity dissolves (magnitude to origin is unambiguous). The
identity-preservation gate simplifies (angle in coordinate space is
topological). The verdict cleans up (four-way conjunction on two
scalar reads: harmonic_distance and angle).

Alex 2026-07-13, in-transcript: **"let's ship substrate-honest history."**

Recommend: Alex adjudicates §10 (all four); Reed lands 8+9.2 (Cargo edge +
SC<5> primitives) via TDD; Reed lands 8+9.3 (contribute switch to SC<5>)
via TDD; empirical discharge at 8+9.6 against real substrate observes
whether shard-body lens now resolves Falsification #1 unambiguously
under SC<5> direction convention.

*End of Rung 8+9 unification canonical spec.*

*Ancestry: `fragmentation::spectral_coordinate::SpectralCoordinate<N>`
(Alex's crate; docblock names substrate carrier + cites `void-dual-
geometry.md` as origin of the manifold); `fragmentation/docs/specs/
mirror-native-vcs.md §4.6-4.7`; `coincidence::coincidence::Detector<N>`
(Bothe 1924); `coincidence::detection::Detection`;
`shards/mirror/lens/refract.mirror` (2026-06-06 landed 5-duality closed
sum); `shards/mirror/index.mirror` (Mara `317e830`); Reed Rung 8 Landing
3+ (`d043ce1` + `f9a47af`); Reed Rung 7' GREEN + Rung 9 Landing 1+2
(`bootstrap/src/contribute.rs`); Mara `c59a5ac` Rung 9 canonical spec
(partially superseded, partially inherited per §12); Mara `2c64060`
`docs/specs/fractal-family-root-mandelbrot-substrate.md`; Bodnar et al.
2022 (arXiv:2206.08702) sheaf-Laplacian; Chamseddine-Connes spectral
action; Douady-Hubbard 1982/1985; Halsey-Jensen-Kadanoff-Procaccia-
Shraiman 1986; `~/dev/systemic.engineering/practice/insights/coincidence/
void-dual-geometry.md`; Recognitions #43, #55, #58, #80, #107;
`#R-fractal-is-mandelbrot-substrate` (candidate; parent);
`#R-void-dualities-ARE-SpectralCoordinate-5-projections-substrate-measurement-is-coordinate-not-metric`
(this spec's candidate). Alex 2026-07-13 in-transcript directives.
CLAUDE.md substrate-pull discipline; two-tick discipline;
substrate-already-had-the-word discipline (Alex 2026-07-07).*
