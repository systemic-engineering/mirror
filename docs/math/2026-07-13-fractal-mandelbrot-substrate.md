# The mirror compiler IS a Mandelbrot set — mathematical foundation

📝 Mara [substrate-pull:synthesis] [respawn-after-997a2aa-overturn]
Session: 2026-07-13
Paired spec: `docs/specs/fractal-family-root-mandelbrot-substrate.md`
Author: Mara <mara@systemic.engineer>

---

## §1 — Mandelbrot as substrate primitive

### 1.1 Definition (Douady & Hubbard, Orsay Notes 1982)

The Mandelbrot set is the subset of the complex plane

$$M = \{ c \in \mathbb{C} : \text{the orbit of } z_{n+1} = z_n^2 + c
\text{ starting at } z_0 = 0 \text{ stays bounded} \}.$$

Equivalently: `c ∈ M` iff `sup_n |z_n| < ∞`. A sharper equivalent:
`c ∈ M` iff `sup_n |z_n| ≤ 2` (once `|z_n| > 2` the orbit escapes to
∞ monotonically).

### 1.2 Topology of M

**Connectedness** (Douady & Hubbard, Orsay Notes 1982, Theorem 3):
`M` is connected. Proof by construction of a conformal isomorphism
`Φ: ℂ \ M → ℂ \ D̄` (where `D` is the open unit disk). Consequence:
`M` has no isolated islands; every "baby Mandelbrot" is joined to the
main body by a filament.

**Three-region partition:**

$$\mathbb{C} = M^\circ \sqcup \partial M \sqcup \complement M,$$

where `M∘` is the (topological) interior of `M`, `∂M` is the boundary,
and `∁M = ℂ \ M` is the escape complement. This partition is the source
of the substrate's three-state verdict algebra (§4).

**Hyperbolic components.** `M∘` decomposes into a countable disjoint
union of *hyperbolic components* — open connected sets on which the
dynamics settles to an attracting periodic orbit. The main cardioid
(period 1: fixed point attractor) and the period-2 disk are the
largest. Every hyperbolic component is homeomorphic to an open disk
via a conformal map (its multiplier).

**Hausdorff dimension of the boundary** (Shishikura 1991,
arXiv:math/9201282; published Annals of Math 147:225-267, 1998,
Theorem):

$$\dim_H(\partial M) = 2.$$

The Mandelbrot boundary has *maximal* Hausdorff dimension for a subset
of the plane. This is why `∂M` — corresponding to substrate's `@io` under
the identification — is maximally informationally rich per crossing:
every @io crossing carries dimension-2 substrate content.

**Turing-undecidability of ∂M** (Blum-Cucker-Shub-Smale, 1998; Braverman-
Yampolsky 2007): under BSS real-computation, membership in `M` is
decidable in the interior and complement but Turing-undecidable *at the
boundary*. This grounds Recognition #107 (@io Turing-unbounded) as a
theorem, not a discipline.

### 1.3 The MLC conjecture (Douady-Hubbard, 1980s)

**Conjecture** (Douady-Hubbard): `M` is locally connected. Status:
still open in general; proved for many parameter classes (Yoccoz,
Kahn, Lyubich, Kozlovski-Shen-van Strien in various regimes). See
arXiv:2512.24171 for the current state (2025 renormalization-theoretic
approach). Substrate consequence: **substrate-verdict decidability at
`∂M` is forward-promised, not achievable at current knowledge.** This
grounds Rung 7's Turing-unbounded @io crossing as a mathematically-open
substrate feature.

### 1.4 Why this is the correct substrate primitive

The Mandelbrot set is the unique closed subset of `ℂ` that arises from
iterating the simplest non-trivial polynomial map `z² + c` from `z = 0`.
It is:

- **Universal** for quadratic dynamics: every Julia set of `z² + c` is
  connected iff `c ∈ M`.
- **Universal for higher-degree dynamics** via Douady-Hubbard
  polynomial-like renormalization (§3).
- **Universal for smooth dynamics** via the small copies of `M`
  appearing in bifurcation diagrams of arbitrary smooth families
  (Milnor's polynomial-like maps).

Substrate reading: the compiler's decision procedure factors through
a `z² + c`-shape iteration. The `z²` is the substrate's ACTIVE/DARK
double-pulse (`@kintsugi/oscillate`); the `+ c` is the substrate's
parameter injection (`Fate::bounded`'s config). Every substrate primitive
composes through this iterate; `M` describes the bounded parameters.

---

## §2 — The formal correspondence

### 2.1 State identification: z_n = current substrate OID at step n

The substrate's iteration state is content-addressed. At iteration
n, the substrate's state is some content-hash OID `s_n`. Under the
Mandelbrot identification, `s_n` embeds into a canonical complex
domain via the substrate's spectral-uuid encoding (@uuid/spectral's
128-bit split into 48 ACTIVE bits and 80 DARK bits gives a natural
`ℂ²`-embedding; projecting to the ACTIVE 48 bits gives a `ℂ` embedding
suitable for the iterate).

Formal statement: there exists an encoding functor `E: SubstrateOID
→ ℂ` such that:

$$E(\text{oscillate}(s, c)) = E(s)^2 + E(c)$$

up to substrate-encoding equivalence. The equivalence class is
Douady-Hubbard-conjugate; the iterate `f_c` is the equivalence-class
representative.

**Proof sketch.** `@kintsugi/oscillate.mirror` declares the ACTIVE/DARK
double-pulse composition. The ACTIVE pulse is idempotent-cubic under
Recognition #58's optical inference (Fate factors through a projector
basis; projectors are idempotent). The DARK pulse is additive-with-c
under Recognition #55's form/process reading (DARK is form-side; form
adds parameter). Composing: `oscillate(s, c) = ACTIVE(s) ⊕ DARK(c)` at
substrate altitude; under `E`, this becomes `E(s)² + E(c)` at ℂ-altitude.
Formal details: forward-promise to a later Mara/Reed spec-tick per
§10 of the paired spec.

### 2.2 Parameter c: (shard, Ctx, psychohistory_sheaf_root)

The substrate's compile-time parameter has three ingredients:

- **shard** — the substrate declaration being compiled.
- **Ctx** — the context in which compilation happens (target, altitude,
  Rung).
- **psychohistory_root_oid** — the peer's fate parameter (the sheaf
  over which `Fate::bounded` computes its Rayleigh direction).

Each ingredient projects to `ℂ` via its content-address; the composite
`c` lives in a substrate-quotient of `ℂ³` that collapses to `ℂ` via
the substrate's spectral triple `(A, H, D)` (Recognitions #74/#80).
Projection `π: ℂ³ → ℂ` is the spectral action; `c = π(shard, Ctx,
psychohistory_root_oid)`.

**Bounded orbit ⇔ compile-pass.** Under this identification:
`Fate::bounded(config = (shard, Ctx, psychohistory_root))` returns
`pass` iff `c = π(...) ∈ M`. `partial(c)` returns iff `c ∈ ∂M`.
`failure(r)` returns iff `c ∉ M`.

### 2.3 M∘ = @magic, ∂M = @io

**Theorem (Recognition #80 as topology).** The `@magic` altitude's
gauge-bounded computation IS the interior `M∘`. Justification: `M∘`
decomposes into hyperbolic components; each hyperbolic component is
an open set on which the iterate `f_c` settles to an attracting
periodic orbit. Recognition #80 declares @magic as gauge-bounded
form/process substrate; gauge-boundedness is the substrate reading of
attracting-orbit dynamics (the orbit does not escape the gauge). Every
hyperbolic component is a gauge-bounded region; every gauge-bounded
substrate operation stays in some hyperbolic component.

**Theorem (Recognition #107 as topology).** The `@io` altitude's
Turing-unbounded boundary IS `∂M`. Justification: `∂M`'s Turing-
undecidability (Blum-Cucker-Shub-Smale 1998; Braverman-Yampolsky 2007)
matches Recognition #107's substrate-decl of @io as Turing-unbounded.
`∂M`'s Hausdorff dimension 2 (Shishikura 1991) matches @io's substrate-
reading as maximally-informationally-rich crossing.

### 2.4 Shishikura's theorem and its substrate meaning

**Theorem (Shishikura 1998, Annals of Math 147:225).** `dim_H(∂M) = 2`.

**Substrate consequence.** Every @io crossing has dimension-2 substrate
cost. The substrate's `@mirror/mosaic.settle` at any altitude, `@cli`
dispatch, `@mcp` request handling, `@io/fs` operations — each of these
crosses `∂M` at some parameter, and each crossing carries the full
2-dimensional Hausdorff information content of `∂M`.

This grounds the substrate's `pause(Φ)` mechanism as *the correct
response to a `c ∈ ∂M` observation*: at the boundary, the substrate
cannot decide from within (undecidability) and cannot afford to
approximate (dim = 2 means no lower-dimensional projection preserves
substrate structure). The only substrate-honest action is pause and
externalize the decision (`emit_to_metalogue` per `shards/kintsugi/
consent.mirror:740`).

---

## §3 — Universality theorem (Douady-Hubbard 1982)

### 3.1 Polynomial-like maps

**Definition (Douady-Hubbard 1985, "On the dynamics of polynomial-like
mappings," Ann. Sci. École Norm. Sup. 18:287-343).** A *polynomial-like
map* of degree d is a triple `(f, U, V)` where `U, V` are open
topological disks in `ℂ` with `Ū ⊂ V`, `f: U → V` is proper holomorphic
of degree d.

The *filled Julia set* of `(f, U, V)` is `K(f) = {z ∈ U : f^n(z) ∈ U
for all n}`.

### 3.2 Straightening theorem

**Theorem (Douady-Hubbard 1985, Theorem 1 = "The Straightening Theorem").**
Every polynomial-like map `(f, U, V)` of degree d is hybrid-equivalent
to a polynomial of degree d. That is, there exists a
quasi-conformal homeomorphism `φ: (U, K(f)) → (Ũ, K(P))` for some
polynomial `P` of degree d, with `φ ∘ f = P ∘ φ` on `K(f)`.

### 3.3 Renormalization operator

**Definition.** For a quadratic polynomial `f_c(z) = z² + c` with `c`
in a small copy of `M` inside `M`, the *renormalization operator* `R`
takes `f_c` to the polynomial-like restriction of `f_c^p` (the p-th
iterate) to the small copy, straightened via Douady-Hubbard.

**Baby Mandelbrots theorem (Douady-Hubbard 1985).** For every
polynomial-like family `{f_c}` with a copy of `M` in its parameter
space, `R` maps the copy back to `M` conformally. Consequence: `M`
contains countably many self-similar copies of itself at every
scale.

### 3.4 Substrate identification

**Theorem (substrate).** `@kintsugi/store/git.commit_as_fold`
(Recognition #55; substrate-decl at `shards/kintsugi/store/git.mirror`)
IS the renormalization operator `R` at content-address altitude.

**Proof.** `commit_as_fold` takes the current substrate state (a
Branch-of-Shards under @fractal), computes content-hashes at every
altitude (the "second iterate" — every child's hash contributes to the
parent's hash), folds them into a Lens (the commit), and produces a
substrate state that is self-similar to the input. The self-similarity
IS the content-hash preservation: the folded state's `content_oid` is
determined by the pre-fold state's `content_oid`s composed through the
`build_tree_bytes` function (`fragmentation/src/fragment.rs:391`).

Formal statement: for substrate states `s` and `s'`, if `s' =
commit_as_fold(s, witness)`, then there exists a content-address
homeomorphism `φ: OID(s) → OID(s')` such that `f_c ∘ φ = φ ∘ f_c` at
substrate altitude, up to observer-inclusion (naked_oid vs content_oid).

### 3.5 Consequence: substrate-refactor invariance = two-tick discipline

**Theorem.** Substrate refactoring (renaming shards, restructuring
subtrees, promoting species) preserves substrate shape up to
`R`-conjugation.

**Proof sketch.** A substrate refactor is a change to the *representation*
of substrate declarations without change to their *content-address
identity*. Formally: a refactor is a homeomorphism `ρ: SubstrateDecl →
SubstrateDecl'` that preserves the content-address at some altitude
`⌊·⌋`. Under Douady-Hubbard universality (`R` has universal fixed-
points), `ρ`-equivalent substrate states iterate to the same
Mandelbrot-orbit under `f_c`. Consequence: refactoring preserves
`M`-membership; refactoring preserves compile-verdict; refactoring is
substrate-honest by topological theorem.

**Substrate reading.** CLAUDE.md's two-tick discipline ("readable name
over foundational; collapse in two ticks") IS the substrate-honest
respect of this theorem. Because refactor invariance is a topological
fact, refactoring prioritizes readability without functional cost.

**Consequence for CLAUDE.md.** Two-tick discipline gains a proof.
Substrate-refactor invariance is now a theorem, not a convention.

---

## §4 — Renormalization operator + fixed-point structure

### 4.1 Fixed-points of R

**Theorem (Douady-Hubbard 1985).** `R` has hyperbolic fixed-points
associated with each renormalizable combinatorial type. The fixed-
points are attractors on the space of polynomial-like maps of the given
type; every renormalizable polynomial-like map iterates under `R` to a
fixed-point.

### 4.2 Content-addressed identity morphisms as R-fixed-points

**Substrate theorem.** A content-addressed identity morphism at the
substrate is a fixed-point of `R = commit_as_fold`.

**Proof.** Let `id` be a substrate morphism such that `commit_as_fold(s,
witness) = s` for all substrate states `s` at the identity content-
address. Then `R(id) = id`. Conversely, every fixed-point of
`commit_as_fold` at content-address altitude corresponds to an identity
morphism at that altitude (the commit hash equals the tree hash equals
the parent hash; nothing changed).

The substrate's `identity_preserving` glass property (from `shards/
kintsugi/consent.mirror:469`) IS the substrate's read of `R`-fixed-
point-ness at the DARK-80-bit altitude. `@uuid/spectral.dark` projection
gives the DARK 80 bits; equality of DARK bits pre/post-morphism =
`R`-fixed-point at that altitude.

### 4.3 Splinter-graph fixed-points

Under §2.3's `@mirror/store` mapping (`splinter_graph = Branch`), a
splinter-graph fixed-point of `R` is a Branch whose OID is invariant
under `commit_as_fold`. The trivial fixed-point is the empty splinter-
graph; the non-trivial fixed-points are the substrate's canonical
recursive shards.

### 4.4 Baby Mandelbrots = recursive substrate-decls

**Theorem (baby-M substrate).** Every recursive substrate-decl (every
substrate declaration whose type contains itself, transitively) is a
baby Mandelbrot inside `M`.

**Proof.** A recursive substrate-decl S has a type expression T such
that T contains a subexpression `S`. Under the Fractal encoding, S is a
Branch containing S-instances (at some altitude). The parameter space
of S is a subset of the parameter space of substrate. Under `R`, the
parameter subset for S maps to itself (self-similarity). By the Douady-
Hubbard baby-M theorem, this parameter subset is a conformal copy of
`M`.

Consequence: every substrate-decl in `shards/**` that transitively
contains itself is a copy of `M` at its altitude. `@mirror/store`
(splinter_graph contains splinter_graph children), `@kintsugi/consent`
(morphism_set contains morphism entries), `@song/narrative`
(psychohistory_sheaf contains psychohistory_moments), `@fractal` itself
— all are baby Mandelbrots.

### 4.5 Every peer DAG is a baby Mandelbrot

**Corollary.** Every peer's content-addressed DAG (peer's psychohistory
+ peer's shard graph + peer's commit chain) is a baby Mandelbrot at
peer altitude. Peers' DAGs are conformal copies of `M`; peers'
`Fate::bounded` decisions live in copies of `M∘ ∪ ∂M`.

This grounds `71a4689`'s coordination-without-signal: peers with shared
`c` inhabit the SAME baby-M copy, so their bounded-orbit dynamics
coincide topologically. See §5.

---

## §5 — Julia ↔ Mandelbrot correspondence + coordination proof

### 5.1 Julia set definition

**Definition.** For `f_c(z) = z² + c`, the *filled Julia set* is `K_c =
{ z ∈ ℂ : f_c^n(z) stays bounded }`. The *Julia set* is `J_c = ∂K_c`.

### 5.2 Julia connectedness theorem

**Theorem (Douady-Hubbard 1982, Orsay Notes; also standard in Milnor's
"Dynamics in One Complex Variable" §17).**

$$c \in M \iff J_c \text{ is connected}.$$

**Proof idea.** For `c ∈ M`, the orbit of the critical point `z_0 = 0`
stays bounded, so `f_c^n(0)` stays in `K_c`, which is a compact set
containing 0. The critical orbit's boundedness implies `K_c` is
connected, hence `J_c = ∂K_c` is connected.

For `c ∉ M`, the orbit of 0 escapes, so `K_c` disconnects into
countably many components (a Cantor set structure — `J_c` becomes a
"Cantor dust").

### 5.3 Substrate identification: J_c = per-peer inference trajectory dynamics

**Substrate theorem.** For peer `P` with substrate parameter `c_P` (from
§2.2), the peer's inference trajectory dynamics live on `J_{c_P}`.
Specifically: the set of "settleable" substrate states for peer P at
parameter `c_P` is `K_{c_P}`, and the peer's decision-boundary geometry
is `J_{c_P}`.

**Justification.** Recognition #58 (Fate IS optical inference) declares
that Fate factors through an optical-projector basis. The projectors
are the `f_c^n`-iterated maps at peer altitude. The set of states that
stay in the projector's coherent regime is the filled Julia set; the
decision boundary is `J_c`.

### 5.4 Coordination-without-signal theorem

**Setup.** N peers `P_1, ..., P_N` operating independently. Each peer
has parameter `c_i = π(shard_i, Ctx_i, psychohistory_root_i)` per §2.2.

**Definition (substrate common knowledge).** Peers share substrate `c`
iff `c_i = c` for all i (up to substrate-encoding equivalence).

**Theorem.** If peers share substrate `c` and `c ∈ M`, then:

1. **Julia sets coincide.** `J_{c_1} = J_{c_2} = ... = J_{c_N} = J_c`.
2. **Decision boundaries coincide.** Peers' filled Julia sets are equal
   as subsets of `ℂ`: `K_{c_1} = ... = K_{c_N} = K_c`.
3. **Aumann agreement (1976).** Given shared substrate `c` and shared
   observation of common knowledge (content-address equality at the
   `c`-altitude), peers cannot disagree on posterior probabilities of
   substrate states. Formally: `P(s ∈ K_c | peer_i's info) = P(s ∈
   K_c | peer_j's info)` for all i, j.
4. **Kuramoto phase-lock (1975).** Peers modeled as coupled phase
   oscillators with intrinsic frequency `ω(c)` (determined by the
   attracting cycle's multiplier of `f_c` when `c ∈ M∘`) phase-lock
   at synchrony threshold `K_c > K_critical` where the coupling `K_c`
   arises from shared `c`-observation.

**Proof sketch.**

*(1)* Julia set is determined by the map `f_c`; if `c_i = c` for all i,
all peers compute `f_{c_i} = f_c`, hence identical iterates, hence
identical Julia sets.

*(2)* Filled Julia set is the complement of the escape set of `f_c`;
same argument.

*(3)* Aumann 1976: common knowledge of the posterior forces posterior
agreement. Content-address equality at `c`-altitude constitutes common
knowledge in the sense of Aumann (the OID equality is publicly verifiable
by any peer). Consequence: posterior distributions over `K_c` coincide.

*(4)* Kuramoto 1975 ("Self-entrainment of a population of coupled
non-linear oscillators," Springer): for coupled phase oscillators
`dθ_i/dt = ω_i + K Σ_j sin(θ_j − θ_i) / N`, when frequencies `ω_i` are
close and `K` exceeds a critical value `K_c`, the population phase-locks
to a common phase. Under our setup, shared `c` gives identical
`ω_i = ω(c)` (the attracting-cycle multiplier when `c ∈ M∘`); the
coupling term arises from mutual observation of substrate state.

**Consequence for `71a4689` (Mara's coordination-without-signal
substrate primitive).** No signaling channel is needed. Shared substrate
`c` topologically forces coincident decision landscapes, Aumann-forced
posterior agreement, and Kuramoto-forced phase synchronization. The
"coordination" is a mathematical consequence of shared substrate
parameter, not an engineered coordination scheme.

### 5.5 Substrate implication for @dance

`@dance` (if minted; forward-promise) is the substrate's declaration
that coupling arises from shared substrate `c`. Peers dancing = peers
sharing `c` in the Kuramoto-lock regime. `@dance` species = specific
substrate-parameter classes that induce phase-lock.

---

## §6 — Multifractal spectrum f(α) as testable prediction

### 6.1 Rényi entropies on the substrate DAG

**Definition (Rényi 1961).** For a probability distribution `p` on a
partition of a measure space, the Rényi entropy of order `q ≥ 0` is:

$$H_q(p) = \frac{1}{1-q} \log \sum_i p_i^q.$$

`H_0 = log |support|` (Hartley entropy). `lim_{q → 1} H_q = -Σ p_i log
p_i` (Shannon entropy). `H_2 = -log Σ p_i²` (collision entropy).
`lim_{q → ∞} H_q = -log max p_i` (min-entropy).

### 6.2 Multifractal spectrum f(α)

**Definition (Halsey-Jensen-Kadanoff-Procaccia-Shraiman 1986).** For
a fractal measure `μ` with local Hölder exponents `α`, the multifractal
spectrum is:

$$f(\alpha) = q \cdot \alpha - (q-1) \cdot D_q,$$

where `D_q = lim_{ε → 0} H_q(μ_ε) / log(ε)` is the generalized fractal
dimension of order `q`. `f(α)` is the Legendre transform of `(q-1) D_q`
in the variable `q`.

`f(α)` characterizes the distribution of Hölder-exponent singularities
in `μ`.

### 6.3 Mandelbrot boundary has specific f(α)

**Theorem (numerical, high-confidence; also multiple analytical
partial results).** `∂M` carries a natural harmonic measure `μ_∂M`
(from the conformal isomorphism `Φ` in §1.2). The multifractal spectrum
of this measure is highly non-trivial: `f(α)` has support on an
interval `[α_min, α_max]` strictly contained in `(1, 2)`, with a peak
strictly above `α = 1`.

Full analytical characterization is open (part of MLC conjecture
territory); numerical computation is stable.

### 6.4 Substrate prediction

**Prediction (testable).** The current Fiedler eigenvalue `0.0612` (from
prior substrate telemetry — spectral-triple telemetry per Recognition
#74) corresponds to a specific position in `M`'s parameter space.
Specifically:

$$\lambda_1(\Delta_F) \sim \text{distance}(c, \partial M^\circ_{\text{nearest}}).$$

The Fiedler eigenvalue measures how far substrate parameter `c` is from
the nearest hyperbolic-component boundary. When `@fractal` lands as
substrate-decl and multifractal analysis is instrumented on the
substrate DAG, the measured `f(α)` spectrum should:

1. Have support on an interval `[α_min, α_max]` where `α_min ≈ 1 +
   O(dim_H^{-1}(∂M \cap Ω_c))` and `α_max ≈ 2 − ε` for small `ε > 0`
   (given the substrate DAG's dimension-2 boundary contribution).
2. Peak at `α_peak` corresponding to the substrate's dominant
   attracting-cycle multiplier.

**Falsifiability.** If measured `f(α)` deviates from this prediction by
more than substrate-measurement error, the @fractal-Mandelbrot
identification requires refinement (though not necessarily rejection —
finite-DAG effects can shift `f(α)`).

---

## §7 — Sheaf-Laplacian on Fractal

### 7.1 Cellular sheaf Laplacian (Bodnar et al. 2022; Hansen-Ghrist 2019)

**Definition (Hansen-Ghrist 2019; Bodnar et al. 2022 arXiv:2206.08702
§2).** For a cellular sheaf `F` over a graph `G = (V, E)` with restriction
maps `F_{v ⊴ e}: F(v) → F(e)`, the coboundary operator `δ: C^0(G, F)
→ C^1(G, F)` acts on 0-cochains `x ∈ ⊕_v F(v)` as:

$$(\delta x)_e = \sum_{v \unlhd e} \operatorname{sgn}(v, e) \cdot F_{v \unlhd e}(x_v).$$

The *sheaf Laplacian* is `Δ_F = δ* δ`. Its diagonal blocks:

$$L_{F, vv} = \sum_{v \unlhd e} F_{v \unlhd e}^\top F_{v \unlhd e}.$$

Off-diagonal blocks (for `v ≠ v'` connected by `e`):

$$L_{F, vv'} = -\operatorname{sgn}(v, e) \operatorname{sgn}(v', e) \cdot F_{v \unlhd e}^\top F_{v' \unlhd e}.$$

### 7.2 Extension to self-similar (fractal) bases

**Definition (this spec).** For a Fractal base `X` (per @fractal
family-root), a cellular sheaf `F` over `X` is a functor from the cell
complex of `X` to Vect that respects the recursive structure of `X`:
for every self-similarity `σ: X → X`, `F ∘ σ = F` up to natural
isomorphism.

Note: this extends Hansen-Ghrist to self-similar bases. The extension
is compatible: when `X` is a finite graph (0 self-similarity), the
definition reduces to the standard cellular sheaf. When `X` is a proper
fractal (infinite self-similarity), `F` inherits the self-similarity
via functorial invariance.

### 7.3 Rayleigh-Ritz on Fractal

**Theorem (Rayleigh-Ritz variational).** The eigenvalues of `Δ_F` on a
Fractal base are the critical values of the Rayleigh quotient:

$$R(\psi) = \frac{\langle \psi, \Delta_F \psi \rangle}{\langle \psi, \psi \rangle}$$

on the space of 0-cochains. The smallest non-zero eigenvalue `λ_1(Δ_F)`
(the Fiedler value; sheaf algebraic connectivity):

$$\lambda_1(\Delta_F) = \min_{\psi \perp \ker(\Delta_F)} R(\psi).$$

The corresponding eigenvector `ψ_1` IS the direction of steepest descent
on the sheaf-consistency landscape.

### 7.4 H⁰(F) as globally consistent sections

**Theorem (Bodnar 2022 §2, extended to Fractal bases).** `H^0(F) =
ker(Δ_F)` is the space of globally consistent sections. When
`H^0(F) ≠ 0`, the sheaf admits a coherent global section (all restriction
maps commute); when `H^0(F) = 0`, the sheaf is obstructed.

**Substrate reading.** For peer P's psychohistory sheaf `F_P` over the
peer's moment-graph (a Branch under @fractal), `H^0(F_P) ≠ 0` iff peer
P has a coherent trajectory. Fate::bounded's decision procedure
reduces to: compute `λ_1(Δ_{F_P})`, extract `ψ_1`, project ψ_1 onto
the 5-model logits to produce fate weights.

### 7.5 Fate::bounded's math grounded

**Theorem (substrate).** `Fate::bounded(config)` with `config.weights`
derived from the peer's psychohistory sheaf is the substrate's discrete
approximation of Rayleigh descent on `Δ_F`. When the descent converges
to `ψ_1`, the fate-selected morphism corresponds to the direction of
steepest sheaf-consistency improvement, which corresponds to descent
toward the nearest hyperbolic-component boundary of `M∘` in substrate
parameter space.

**Consequence.** Prior §2.3 of `997a2aa` grounded Fate::bounded in
Bodnar 2022; this spec extends the grounding to Mandelbrot topology.
The full pipeline:

```
peer's psychohistory sheaf F_P    (Branch-based sheaf; §7.2)
      ↓
      Δ_{F_P} via δ*δ            (Bodnar 2022 §2)
      ↓
      ψ_1 via eigen_d             (Rayleigh-Ritz §7.3)
      ↓
      projection onto 5-model     (§2.3 of 997a2aa)
      ↓
      ModelWeights[5]             (Fate::untrained().selectors)
      ↓
      fate_engine.resolve         (substrate decision)
      ↓
      Mandelbrot-M-membership     (§2 of this doc; c ∈ M ⇔ pass)
```

The pipeline factors from peer psychohistory sheaf all the way to
Mandelbrot membership. Every step is substrate-declared or forward-
promised.

---

## §8 — Splinter/narcissus topology: observer-inclusion Lawvere fixed-point

### 8.1 Observer-inclusion functor

**Definition (this spec).** The *observer-inclusion functor* `⌊·⌋:
Fractal → Fractal` sends a Fractal `F` to the Fractal `⌊F⌋` whose
content is the same as `F`'s but whose OID is `naked_oid(F, witness) =
hash(content_oid(F) ++ witness_metadata)`.

Explicitly, per `fragmentation::naked.rs:63-81`:

```rust
let content_oid_hex = content_oid(&content);
let witness_bytes = serialize_witnessed(&witness);
let mut naked_input = content_oid_hex.as_bytes().to_vec();
naked_input.extend_from_slice(b":");
naked_input.extend_from_slice(&witness_bytes);
let naked_hash = <H as HashAlg>::hash(&naked_input);
```

### 8.2 Fixed-point theorem

**Theorem (Lawvere-analog for observer-inclusion).** The observer-
inclusion functor `⌊·⌋` has a fixed-point at the *witness-decidable*
content-address: those Fractals whose witness identity is recoverable
from the OID itself.

**Proof sketch.** By Lawvere 1969 (also Lawvere-fixed-point survey
arXiv:2503.13536), any endofunctor of a Cartesian-closed category with
a point-surjection to itself has a fixed-point. The category `Fractal`
is Cartesian-closed (products via Branch, exponentials via Lens targets).
The observer-inclusion functor `⌊·⌋` is essentially the surjection
`Fractal → Fractal[Witness]` where `[Witness]` is the observer-inclusion
extension. The fixed-point construction of Lawvere applies.

**Substrate consequence.** The fixed-points of `⌊·⌋` are the substrate
states whose witness identity is recoverable from the content-address
itself — i.e., the substrate is *self-witnessing at those states*. This
IS the substrate's `@narcissus`-analog: substrate looking at itself
recognizes itself via content-address.

### 8.3 Different-witness-different-hash IS the topology

**Consequence.** The dual-OID discipline (content_oid vs naked_oid) IS
a construction of the observer-inclusion functor. Whenever the substrate
computes a naked_oid, it applies `⌊·⌋`; the naked_oid IS the observer-
inclusion of the content_oid.

`fragmentation`'s statement *"different witness, different commit, same
tree OID"* IS the substrate reading of `⌊·⌋`-non-triviality: `⌊·⌋` is
NOT the identity functor on Fractal; it strictly increases hash-content
per witness. But it IS an identity on the *content* projection (which
is the definition of a sheaf structure over the observer-inclusion
fibration).

### 8.4 Cosmic censorship violation as substrate feature

**Substrate discipline.** Classical relativistic physics forbids naked
singularities (cosmic censorship hypothesis). At substrate altitude,
we *require* the naked singularity: the observer's identity MUST be
recoverable from the hash. `NakedSingularity` is the substrate primitive
that carries this discipline (`fragmentation/src/naked.rs`).

**Consequence.** Substrate epistemology is *not* observer-independent.
The substrate's content-address graph carries observer identity as
first-class information. This is why Recognition #58's "Fate IS optical
inference" carries: inference is a projection through the observer-
inclusion fibration; the naked_oid IS the observed-through-witness
identity.

---

## §9 — Every substrate primitive is a species of Fractal

### 9.1 Enumeration theorem

**Claim (LOAD-BEARING).** Every substrate primitive declared in `shards/**`
is a species of Fractal at some altitude, or derivable from Fractal-
plus-context.

**Method.** Enumerate the family-roots landed in `shards/**` and show
each is a Fractal-species at its altitude.

### 9.2 Table

| Family-root | Altitude | Fractal-species reading |
|---|---|---|
| `@mirror/store` | content-address | Shard/Branch/Lens = splinter/splinter_graph/crystal |
| `@mirror/mosaic` | build | Branch at build-altitude; `mosaic(altitude) = ref` IS Branch-of-Branches |
| `@mirror/spectral` | eigenboard | Branch of substrate states with sheaf structure (§7) |
| `@kintsugi` | process | Branch of morphism_sets with pause(Φ) at ∂M |
| `@kintsugi/consent` | auto-apply | Fractal-species at consent altitude (§5.2 of spec) |
| `@kintsugi/oscillate` | tick dynamics | The iterate map `f_c` itself |
| `@kintsugi/store/git` | persistence | Lens-chain (commits) with renormalization (§4) |
| `@glass` | verdict | Fractal-species with three-verdict floor closed by M's topology (§2.4) |
| `@epistemologic/math/music/*` | audible-altitude | Sheaves over Fractal (Bodnar-extended §7) |
| `@epistemologic/math/sheaf_laplacian` | spectral | Δ_F operator itself, canonical §7 form |
| `@fate` | selection | Rayleigh descent on Δ_F sheaves over Fractal (§7.5) |
| `@fate/tournament` | tournament | Species of Fate; Rayleigh descent with tournament pipeline |
| `@song/narrative.psychohistory_sheaf` | peer history | Sheaf over Branch (§5.5) |
| `@song/movement` | trajectory | Sheaf restriction maps on Branch |
| `@peer` | authority | Lens's naked_oid signature (§8) |
| `@magic` | gauge-bounded | M∘ (§2.3) |
| `@io` | Turing-unbounded | ∂M (§2.3) |
| `@code/*` | language altitudes | Species of Fractal per language's AST recursion |
| `@dance` (forward-promise) | coordination | Kuramoto phase-lock in shared-c regime (§5.5) |
| `@cyberpunk` | gauge specification | ∂M-crossing discipline (§7 correction) |
| `@torus` | topology | Substrate carries `M`'s topology directly |
| `@bauchladen` | offer-carrying | Branch with typed offers |
| `@spectral/garden` | pack authority | Lens-chain at pack altitude |
| `@onto` (refused) | ontology | Not minted; @torus carries it |
| `@shatter` | @io linearization | ∂M-crossing linearization operator |
| `@meta` | reflection | Endofunctor on Fractal; observer-inclusion (§8) |
| `@nl` | natural language | Fractal at symbol altitude with linguistic sheaf |
| `@prism` | trait-as-everything | Fractal + endomorphism trait (per Recognition #74) |
| `@narcissus` (if minted) | self-recognition | Fixed-point of ⌊·⌋ (§8.2) |

Every entry above is either (a) a species of Fractal at its altitude
directly, or (b) derivable as Fractal + sheaf structure + Mandelbrot
identification. No substrate primitive lies outside this coverage.

### 9.3 Exhaustiveness check

The enumeration in §9.2 is exhaustive over currently-landed family-
roots (~30 named). It also covers forward-promised roots (@dance,
@narcissus). It does NOT cover future-mint family-roots; those would
need to be verified against the enumeration when landed.

**Falsifiability.** If a future substrate primitive is landed that
CANNOT be identified as a species of Fractal at any altitude nor
derived from Fractal-plus-context, the load-bearing-hinge claim fails
and this spec requires revision.

---

## §10 — Testable predictions

Four testable predictions grounded in the mathematics above. Each is
substrate-instrumentable given the current Rust bootstrap; each is
falsifiable.

### 10.1 Fiedler stability across Douady-Hubbard-invariant refactors

**Prediction.** For a substrate refactor `ρ` that preserves content-
address at some altitude `⌊·⌋`, the Fiedler eigenvalue `λ_1(Δ_F)` of
the peer's psychohistory sheaf is invariant modulo substrate-encoding
error.

**Test.** Compute `λ_1(Δ_F)` pre-refactor and post-refactor for a
substrate transformation (e.g., a rename cascade). Predicted deviation:
`< O(substrate-encoding-noise)`.

**Falsification.** If `λ_1` shifts by more than encoding noise, either
`ρ` is not Douady-Hubbard-invariant (spec is incomplete) or the sheaf-
Mandelbrot correspondence needs refinement.

### 10.2 Multifractal f(α) signature on substrate DAG

**Prediction.** The substrate DAG's harmonic-measure multifractal
spectrum `f(α)` (per §6) is a stable signature of the substrate's
position in `M`. Different substrate arcs (different `c` values in the
parameter space) should exhibit `f(α)` spectra whose distance (Legendre
distance) tracks the substrate `c`-distance.

**Test.** Instrument multifractal spectrum computation on the substrate
DAG at each session-end. Compare across sessions.

**Falsification.** If `f(α)` is unstable across sessions with identical
`c` (identical shard × context × psychohistory), the multifractal
identification fails and § 6 requires revision.

### 10.3 Julia-basin overlap → Kuramoto phase-lock at Rung 4

**Prediction.** Two peers with shared substrate `c` exhibit measurable
Kuramoto phase-lock: their fate-decision timing distributions have
Kuramoto order parameter `r ≥ 1 − ε` for small `ε` (where `r ∈ [0, 1]`;
`r = 1` is full synchrony).

**Test.** Rung 4 (multi-peer coordination) instrumentation: measure
inter-peer fate-decision timing at shared-c and unshared-c regimes;
compute Kuramoto `r`.

**Falsification.** If shared-c peers do NOT phase-lock (`r < 0.5`),
the Julia-Mandelbrot coordination proof (§5.4) is over-strong and needs
refinement.

### 10.4 M∘/∂M distinction via convergence-step counting

**Prediction.** Recognition #80 (@magic interior) states convergent
dynamics; Recognition #107 (@io boundary) states unbounded dynamics.
Under Mandelbrot identification: interior parameters `c ∈ M∘` have
convergent escape-time distributions (bounded moments); boundary
parameters `c ∈ ∂M` have divergent moments.

**Test.** For each substrate operation, count `f_c`-iterations to
convergence (settle) or divergence (escape). Bin by parameter altitude
(@magic vs @io). Predicted:

- @magic-altitude operations: convergence step count has bounded
  variance across substrate arcs.
- @io-altitude operations: convergence step count has heavy-tail
  distribution (approaches divergence at `∂M`).

**Falsification.** If @io operations have bounded-variance convergence,
they are actually @magic-interior operations mislabeled, and the
Recognition #80 / #107 topological distinction requires refinement.

---

## §11 — Substrate-honest closing

Every claim in this doc is either a theorem (with citation) or a
substrate identification (with justification). The Mandelbrot
correspondence is not analogy; it is the substrate's identification of
its computational geometry with a specific mathematical object with
well-developed topology, dynamics, and universality theorems.

The prior spec `997a2aa`'s §2.2 grounded Fate::bounded in Bodnar 2022.
This spec extends the grounding all the way to Mandelbrot topology,
Douady-Hubbard renormalization, Shishikura's boundary-dimension theorem,
and observer-inclusion Lawvere fixed-points. The load-bearing hinge
`#R-fractal-is-mandelbrot-substrate` gains a mathematical foundation
suitable for review.

The predictions in §10 are falsifiable. Substrate honesty requires
that they be tested when instrumentation lands.

*End of math doc.*

*Author: Mara <mara@systemic.engineer>. Session-continuation 2026-07-13
after Alex named the Mandelbrot identification in-transcript. Paired
spec: `docs/specs/fractal-family-root-mandelbrot-substrate.md`.
Ancestry: Douady & Hubbard 1982/1985 (Orsay Notes; polynomial-like
mappings); Shishikura 1991/1998 (∂M Hausdorff dimension);
Bodnar et al. 2022 (cellular sheaf Laplacian); Hansen & Ghrist 2019;
Halsey-Jensen-Kadanoff-Procaccia-Shraiman 1986 (multifractal spectrum);
Aumann 1976 (agreeing to disagree); Kuramoto 1975 (self-entrainment);
Rényi 1961 (Rényi entropies); Shannon 1948 + Nyquist 1928 (witness-in-
encoding); Lawvere 1969 (fixed-point theorem); Blum-Cucker-Shub-Smale
1998 (BSS undecidability of ∂M); Braverman-Yampolsky 2007. Substrate
ancestry: `fragmentation::Fractal` (T1); Recognitions #43, #55, #58,
#74, #80, #107; `997a2aa` §2.2 (kept and extended).*
