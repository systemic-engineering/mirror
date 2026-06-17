# Sheaf Laplacian

*Cellular sheaves on graphs, the sheaf Laplacian `Δ_F = δ*δ`, the
smallest eigenvalue `λ₀`, and Hodge decomposition. The mathematical
family the substrate uses to measure coherence at multiple altitudes
(peer-cognition collapse, eigenboard, librarian topology).*

## §1 Cellular sheaves

A **cellular sheaf** `F` on a graph `G = (V, E)` assigns:

- to each vertex `v ∈ V`, a vector space `F(v)` (the **stalk**),
- to each edge `e = {u, v} ∈ E`, a vector space `F(e)`,
- to each incident pair `(v, e)` (with `v` an endpoint of `e`), a
  linear **restriction map** `F_{v ⊲ e}: F(v) → F(e)`.

A **global section** is a choice of `x_v ∈ F(v)` for every vertex
such that `F_{u ⊲ e}(x_u) = F_{v ⊲ e}(x_v)` for every edge `e = {u, v}`.
Global sections are exactly the elements of the kernel of the
coboundary map `δ` defined below.

In the constant-stalk case `F(v) = ℝ^k` for all `v`, with restrictions
the identity, the cellular sheaf reduces to the graph itself; the
sheaf Laplacian becomes the ordinary graph Laplacian. Non-constant
stalks and non-trivial restrictions give richer structure.

## §2 The sheaf Laplacian

Let `C⁰(G; F) = ⨁_v F(v)` (vertex cochains) and `C¹(G; F) =
⨁_e F(e)` (edge cochains), each with a chosen inner product. Fix a
reference orientation on each edge.

The **coboundary map** `δ: C⁰ → C¹` acts on `x = (x_v) ∈ C⁰` by:

```
(δ x)_e = F_{v ⊲ e}(x_v) − F_{u ⊲ e}(x_u)    for e = {u, v} oriented u → v
```

The **sheaf Laplacian** is the self-adjoint operator:

```
Δ_F = δ* δ : C⁰ → C⁰
```

where `δ*` is the adjoint of `δ` with respect to the chosen inner
product.

### §2.1 The smallest eigenvalue `λ₀`

The Laplacian `Δ_F` is positive semi-definite. Its smallest
eigenvalue is `λ₀(F) ≥ 0`. The kernel `ker(Δ_F)` is exactly the
space of global sections of `F`:

```
λ₀(F) = 0   ↔   F has a non-trivial global section
λ₀(F) > 0   ↔   no global section exists; the substrate has incoherent regions
```

This is the **sheaf-coherence criterion**. The substrate uses
`λ₀(F)` as a scalar measure of how close `F` is to admitting a
global section.

For the substrate's standard inner products, `λ₀(F)` equals the
**spectral gap** between the trivial subspace and the lowest non-
trivial eigenspace. The Fiedler vector (the eigenvector for `λ₀`)
localizes the obstruction — it identifies the vertices where the
coherence breaks. The substrate uses this localization for
flag-and-suggest at the obstruction site.

## §3 Hodge decomposition

For each `n`, the space of `n`-cochains decomposes orthogonally:

```
Cⁿ(G; F) = ker(Δ_F^n) ⊕ im(δ) ⊕ im(δ*)
```

The summands are:

- **`ker(Δ_F^n)`** — harmonic cochains; representatives of the sheaf
  cohomology `Hⁿ(G; F)`.
- **`im(δ)`** — exact cochains; cohomologically trivial.
- **`im(δ*)`** — co-exact cochains.

In particular, `ker(Δ_F^0) ≅ H⁰(G; F)` — the space of global sections.
The substrate uses this isomorphism to read coherence as a cohomology
statement.

For cellular sheaves on graphs, `H¹(G; F)` is the "first sheaf
cohomology" — the obstruction to extending local sections globally.
Mirror's eigensheaf framework (per `docs/specs/eigensheaf.md` and
`[[architecture-eigenboard-is-sheaf]]`) uses `H¹` as the structural
obstruction reading.

## §4 Substrate carriers

Mirror declares the sheaf-Laplacian machinery in substrate:

- `shards/epistemologic/math/sheaf_laplacian.mirror` declares the
  `restriction` carrier, the operator `Δ_F = δ*δ`, the `eigenvalue`
  carrier, and the `lambda_zero` reader. Mara T8 lifted this from
  Hansen-Ghrist (2019) prior art.
- `shards/epistemologic/math/curvature.mirror` declares
  `balanced_forman` per Topping et al. (2022) — per-edge curvature;
  complementary to `λ₀` for localizing the obstruction.
- `shards/spectral/entanglement.mirror` IS a sheaf restriction map at
  substrate altitude (per recognition #55 landed 2026-06-11).

These are the substrate's reference declarations; specs cite them.

## §5 Three altitudes that use `λ₀`

The substrate uses sheaf-Laplacian `λ₀` at multiple altitudes; each
altitude gives a different sheaf, but the same math.

### §5.1 Peer cognition (peer-altitude coherence)

Per `[[architecture-mirror-cogito-glass-over-fate]]` and
`docs/specs/peer-cognition.md` §3: the dependency-chain graph at the
peer altitude carries a sheaf `F_peer` whose stalks are the peer's
cognitive frames at each chain step, and whose restrictions are the
frame-compatibility constraints. `λ₀(F_peer) = 0` means the peer's
chain composes coherently; `λ₀(F_peer) > 0` means the peer is
holding incompatible frames — the substrate's collapse-measurement.

The standalone-use heuristic (per `peer-cognition.md` §3.2) uses
`λ₀(F)` to decide whether a candidate sub-glass should be a root
prism (composed standalone, low `λ₀`) or stay a glass (composed only
in-context, requires the parent frame).

### §5.2 Librarian topology (cross-repo coherence)

Per `[[architecture-spectral-db-autopoietic-memory]]` and
`docs/specs/spectral-db-as-autopoietic-memory.md` §3–§5: the topology
graph (vertices = repos, edges = cross-repo references) carries a
sheaf `F_topology` whose stalks are the per-repo crystal indices and
whose restrictions are the cross-repo reference compatibility maps.
`λ₀(F_topology) = 0` means the topology admits a consistent global
index; the librarian's topology perturbations preserve this property
(per `consolidation_preserves_sheaf_coherence`).

### §5.3 Eigenboard (per-peer sheaf)

Per `[[architecture-eigenboard-is-sheaf]]` and
`docs/specs/eigensheaf.md`: the eigenboard IS a cellular sheaf on the
five-operation graph; restriction maps ARE the conductivity tensor.
The eigenboard's `λ₀` is the peer's instantaneous structural
coherence; the Fiedler vector localizes attention to the smallest
obstruction. Reflection writes morphisms by reading the Fiedler
vector.

## §6 The Polyak-Łojasiewicz contraction

For the substrate's kintsugi loop (per
`[[reference-mirror-spectral-spec]]`), each tick contracts the
`λ₀(F)` measurement toward zero with ratio `ρ < 1`:

```
λ₀(F_{t+1}) ≤ ρ · λ₀(F_t)
```

This is the Polyak-Łojasiewicz form of the substrate's monotone
descent. The fixed point is `λ₀ = 0` — the substrate-coherent state.

The contraction is achieved by the substrate-pull alignment: each
fracture body proposes a morphism that minimizes `λ₀`. Per
`[[feedback-manual-closure-is-training-pull]]`, manual patching
breaks this guarantee — the loop's contraction depends on the
fracture bodies' substrate-pull discipline.

## §7 Per-edge curvature (complementary to `λ₀`)

The **Balanced Forman curvature** `Ric(i, j)` (per Topping et al.
2022) is a per-edge scalar that complements `λ₀`'s global reading:

- `λ₀(F)` says **how much** coherence is missing (global scalar).
- `Ric(i, j)` says **where** coherence is missing (per-edge).

The substrate's `shards/epistemologic/math/curvature.mirror`
declares this carrier. The drone story (per
`docs/specs/drone-narrative-mapping.md` and
`[[project-drone-as-documentation]]`) uses per-edge curvature when
it "looked at relationships, at the geometry of attention" (L29);
the drone reads per-edge curvature to identify which sub-graph
edge is the locus of tension.

For the librarian's perturbation choice, the combination is:

- `λ₀(F_topology)` says the topology has a coherence problem.
- `Ric(repo_a, repo_b)` says the problem is on the
  `(repo_a, repo_b)` edge.
- The perturbation operates on that edge specifically.

## §8 Connection to the principal-bundle tower

The sheaf-Laplacian framework is the **sheaf-theoretic shadow** of
the bundle's holonomy structure (see
`docs/math/the-tower/holonomy.md`):

- A cellular sheaf `F` on `G` is the section sheaf of an associated
  vector bundle.
- `δ` is the coboundary; `δ*` its adjoint; `Δ_F = δ*δ`.
- The sheaf's global sections correspond to the bundle's flat
  sections (holonomy-trivial transports).
- `λ₀(F)` is the energy of the lowest non-flat section — the
  minimum holonomy norm.

This is why the substrate uses the same `λ₀` reading at every
altitude: each altitude has its own bundle (per
`docs/math/the-tower/altitudes.md`); each bundle has its own sheaf
of sections; each sheaf has its own `Δ_F` and `λ₀`. The math is
altitude-portable.

## §9 Prior art

- **Hansen, J. and Ghrist, R.** (2019). *Toward a spectral theory of
  cellular sheaves*. J. Appl. Comput. Topol. 3, 315–358. **Load-
  bearing reference for the substrate's sheaf-Laplacian declaration.**
- **Topping, J. et al.** (2022). *Understanding over-squashing and
  bottlenecks on graphs via curvature*. ICLR 2022. The per-edge
  Balanced Forman curvature.
- **Hodge** (1941) original; **Eckmann** (1944) for the discrete
  case. The Hodge decomposition.
- **Curry, J.** (2014) *Sheaves, Cosheaves and Applications*. PhD
  thesis. The applied-topology grounding the substrate inherits.
- **mirror's eigensheaf framework**:
  `docs/specs/eigensheaf.md` (Mara) — the substrate-altitude
  framework using these tools.
- **`shards/epistemologic/math/sheaf_laplacian.mirror`** — the
  substrate declaration; cited by the specs that use the math.
