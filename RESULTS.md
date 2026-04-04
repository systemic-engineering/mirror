# Spectral Cryptographic Break — Results

**Date:** 2026-04-04
**Branch:** `break/crypto`
**Status:** Negative result. Settled. Publishable.

## Thesis

The discrete logarithm problem on elliptic curves is a navigation problem
on the Cayley graph of the curve group. If the spectral structure of this
graph is compressible and scale-invariant, the private key can be derived
from the public key classically.

## Experiments

### 1. Cayley Graph Eigendecomposition (8-bit)

**Curve:** y² = x³ + x + 1 (mod 251). 282 points.

The Cayley graph of the cyclic group with one generator is a ring.
The ring Laplacian eigenvalues match the DFT formula `2 - 2cos(2πk/n)`
to machine epsilon (7.11e-15).

**Result:** 100% private key recovery via Fiedler eigenvector phase.
The crystal forms. The DFT navigates. O(1) per key after O(n²) eigendecomposition.

**Interpretation:** Known result. The DFT solves cyclic groups. The eigendecomposition IS the DLP in disguise — O(n²) to compute.

### 2. Ego-Graph Spectral Coordinates (8-bit, 12-bit)

**spectral-db ingestion:** 282 nodes settled in 2 ticks. 4100 nodes settled in 2 ticks.

Ego-graph coordinates (1-hop neighborhood Laplacian) give zero discriminating power on ring graphs. Every vertex on a ring has identical local structure (degree 2).

**Result:** Monotonicity 54.5% (random). Distance ratio 1.05. No signal.

### 3. Character Sums / Legendre Pattern (8-bit)

The Legendre symbol pattern χ(x³+ax+b) over GF(p) encodes which x-values
lie on the curve. Its sum equals the negative Frobenius trace (confirmed: Σχ = 30 = -trace for p=251).

Three correlations tested against the private key:
- Cumulative Legendre sum at x_Q: ρ = 0.029
- Raw x-coordinate: ρ = 0.005
- Character sum magnitude |S_{x_Q}|: ρ = 0.005

**Result:** All below noise floor. No signal. The algebraic structure constrains group order but not the DLP within the group.

### 4. Crystal Transfer (8-bit → 12-bit)

The critical test: does the spectral structure at 8-bit predict the
12-bit structure? Compared normalized x-coordinate sequences of kG
across GF(251) and GF(4093).

- x-sequence correlation: 0.010 (zero)
- Step ratio correlation: 0.005 (zero)

8-bit 2G = (63, 93), normalized 0.251.
12-bit 2G = (3070, 2557), normalized 0.750.

**Result:** The crystal does not tile. Walks on different fields are
uncorrelated. The group operation depends on field arithmetic, which
differs completely across characteristics.

### 5. Sparse Lanczos + Component Decomposition (12-bit)

Built SparseLaplacian with O(n+m) memory and O(m) matvec.
Lanczos iteration for top-k eigenpairs. Component detection via BFS.

The 12-bit group (4100 points) has 2 connected components (generator
order 2050 = N/2). Component-aware Lanczos gives Fiedler eigenvalue
off by 130x — Krylov subspace too small for the degenerate ring spectrum.

**Result:** Sparse Lanczos works for small rings (n=50: exact). Fails
for large rings (n=2050) due to eigenvalue degeneracy.

### 6. Cooley-Tukey Butterfly (8-bit)

Tested whether the FFT butterfly decomposition provides leverage.
The butterfly operates on index pairs (k, k+n/2) in ring ordering.
Computing ring[k] IS the DLP for k. The recursion doesn't help because
each level needs the DLP.

**Result:** Confirmed circularity. Cooley-Tukey requires ring-ordered
access, which IS the discrete log.

### 7. Lens — x-Projection Spectrum (8-bit)

The function f(k) = x(kG) maps group index to x-coordinate. Computed
the full DFT F(ω) and power spectrum for all 282 frequencies.

- Peak-to-average ratio: 15.12
- Normalized spectral entropy: 0.8684
- 50% energy in 14.2% of frequencies

**But:** The k² mod p baseline has nearly identical properties (entropy 0.8841,
peak-to-avg 7.28, 50% energy in 13.5%). An LCG baseline is actually MORE
structured (entropy 0.43, peak-to-avg 84.65).

**Result:** The x-projection is **spectrally generic** — no curve-specific
structure beyond what any simple algebraic function over a finite field
produces. The concentration is real but not exploitable.

### 8. Traverse — Coordinate-Order Walk (8-bit)

Walked curve points in x-coordinate order (not group order). Computed
group differences between consecutive-x points. Compared to random
permutation baseline.

- Group-index difference variance: 0.9936× expected (uniform)
- DFT peak-to-average: 5.85 (curve) vs 5.05 (random) — ratio 1.16
- Autocorrelation peaks: 10 (curve) vs 8 (random)

**Result:** The coordinate-to-index permutation is **spectrally flat** when
viewed from coordinate space. Indistinguishable from a random permutation.

### 9. Iso — Windowed Spectral Matching (8-bit)

Given public key Q = dG, computed the window w(j) = x(Q + jG) using
only public data. Cross-correlated with full f(k) to attempt recovery
of d.

- Cross-correlation peak lands at correct d for all test keys
- Peak-to-second ratio: max 1.21 — indistinguishable from noise
- Minimum window for correct peak position: scales O(√n)
- Phase-based recovery: complete failure
- **Circularity:** computing f(k) for all k requires O(n) = brute force DLP

**Result:** Faint signal exists but is buried in noise. The approach is
circular — total cost O(n) regardless of window size.

## Literature Survey

Research agent surveyed 10 areas (see full output in agent logs):

| Approach | Status |
|----------|--------|
| Hensel lifting | Inapplicable across primes |
| p-adic methods (Silverman's Four Faces) | All four fail |
| Schoof/SEA | Computes order, not DLP |
| Index calculus / summation polynomials | Orthogonal to spectral structure |
| Weil descent (GHS) | Requires extension fields |
| Dequantization of Shor | **Proven impossible** (HSP lower bound) |
| CM curve self-similarity | Within one curve only |
| Isogenies | Same field; Ramanujan works against you |
| Cross-field transfer | **Structurally impossible** (Shoup bound) |

### Theoretical Survey: Spectral Bridges (2026-04-04)

Six areas investigated for a cross-object spectral bridge (iso between
different mathematical structures). Full survey: `docs/theoretical-iso-survey.md`.

| Direction | Status |
|-----------|--------|
| Spectral reduction of embedding degree | Dead — k = ord_n(p) is arithmetic, not spectral |
| Isogeny graph spectra | Ramanujan property works FOR security |
| AG codes / Riemann-Roch | No connection — different mathematical objects |
| SEA-style modular decomposition | Computes global invariants, not local DLP |
| Lower bounds for structured algorithms | **Open** — no proof field structure can't help |
| Spectral bridge X | Representation-theoretic circularity — see below |

## Why the Spectral Approach Fails

### Layer 1: The permutation is pseudorandom

The coordinate-to-index mapping (which curve point (x,y) corresponds to
which group element k) behaves as a pseudorandom permutation. Verified
in both directions:

- **Group → coordinates** (lens): f(k) = x(kG) is spectrally generic.
  No curve-specific structure. Same spectral profile as k² mod p.
- **Coordinates → group** (traverse): walking in x-order, group differences
  are uniform. Indistinguishable from random permutation.

### Layer 2: The spectral decomposition is circular

Any spectral decomposition of Z/nZ requires evaluating group characters
χ_ω(P) = exp(2πi·log_G(P)·ω/n). Evaluating χ_ω(P) requires the discrete
log of P. The DFT of the group IS the DLP.

This isn't an accident. The hardness of ECDLP is precisely the statement
that no efficient representation of the group action exists from coordinate
descriptions of points.

### Layer 3: The group law is non-linearizable

Spectral methods linearize. The EC group law is a degree-2 rational function
in the coordinates:

    x(P+Q) = ((y_Q - y_P)/(x_Q - x_P))² - x_P - x_Q

This has no known linearization over any finite field. The formal group
gives a local (p-adic) linearization, but computing it globally requires
O(n) operations.

Shor's algorithm doesn't linearize either — it evaluates the non-linear
map in quantum superposition. There is no classical analogue.

## The Open Gap

No proof exists that ECDLP over prime fields requires Ω(√n) time for
all algorithms. The Shoup/Nechaev/AGM lower bounds cover generic and
algebraic group models but not algorithms that exploit field structure.
The gap between "generic algorithms need √n" and "there exists a
polynomial-time algorithm using field structure" is technically open.

Every concrete approach that enters this gap hits one of three walls:
1. Reduces to a known-hard problem in the target domain
2. Requires the discrete log to set up the spectral decomposition
3. Exploits structure that cryptographic curves are designed to lack

## Optics Vocabulary

The experiments were organized using the optics vocabulary from
functional programming / category theory:

| Optic | Property | Experiments | Signal? |
|-------|----------|-------------|---------|
| **Fold** | Collapse structure to value. Lossy. | 1 (eigendecomposition), 3 (character sums), 5 (Lanczos) | Trivially known |
| **Prism** | Partial decomposition into cases. | 6 (butterfly), 5 (components) | Circular |
| **Traverse** | Walk all elements, preserve shape. | 8 (coordinate-order walk) | Flat — no signal |
| **Lens** | Bidirectional focus into sub-structure. | 7 (x-projection DFT) | Spectrally generic |
| **Iso** | Lossless bidirectional bridge. | 9 (windowed matching), theoretical survey | Circular + noise |
| **Composition** | Circular-reflexive loop over all five. | 10 (Abyss meta-graph) | Fixed point exists but input is circular |

Consumer optics (fold, prism) collapse too much information.
Producer optics (traverse, lens, iso) preserve structure but find
the permutation is pseudorandom in both directions.
Composing all five (circular-reflexive navigation) preserves existing
signal but creates no new signal — the views are orthogonal (all
pairwise spectral distances ≈ 1.0).

### 10. Abyss Meta-Graph — Circular Reflexive Navigation (8-bit)

Built a meta-graph where nodes are the five mathematical views and
edges are spectral distances. Navigated by minimum-meaning-loss through
circular-reflexive loops.

- All pairwise spectral distances ≈ 1.0 — views are orthogonal
- Peak sharpening from Iso converges to d=42 (correct) — but Iso's
  input is circular (requires O(n) DLP computations)
- No other starting view recovers d under any interference operation
- Chebyshev coincidence: Lens and Hodge share dominant T₁₆
- No view follows Catalan rank distribution

**Result:** The circular loop converges (the mechanism works). But
it preserves existing signal, not creates new signal. The five views
are in orthogonal spectral subspaces — no interference pattern can
concentrate information that isn't already present.

**The Chebyshev T₁₆ coincidence** between Lens (group-theoretic) and
Hodge (geometric) views is genuine cross-domain structure. Not useful
for DLP. Real for spectral navigation of mathematical knowledge graphs.

## What We Built (Valuable)

- **SparseLaplacian** in coincidence: O(n+m) memory, O(m) matvec, Lanczos top-k
- **Component detection** + per-component eigendecomposition
- **Full elliptic curve arithmetic**: point_add, scalar_mul, Tonelli-Shanks mod_sqrt
- **spectral-db integration**: 65k-node graph ingestion, settling in 2 ticks
- **skeleton-key test harness**: clean, reproducible, TDD throughout
- **Theoretical survey**: 30+ papers reviewed, six directions evaluated
- **16 .mirror grammars**: the meta-graph of EC mathematics, content-addressed,
  typed, navigable. `@weierstrass` as hub (9 inbound refs). `spectral_bridge`
  encoding the functors between structures. The Abyss's first real territory.
- **Mechanism insight**: LLM ≡ Abyss ≡ circular-reflexive collapse of
  possibility space. Same operation, different graph. Applies to Pathfinder.

## The Sentence

The EC group law is non-linearizable over finite fields. Spectral methods
linearize. The coordinate-to-index permutation is pseudorandom in both
directions. Any spectral decomposition of the group requires the discrete
log to evaluate — the representation theory is circular. Shor bypasses
this via quantum superposition, not by finding a better target space.

Ten experiments. Six theoretical directions. Three layers of explanation.
The internet stays locked.

Both outcomes were always publishable. This one doesn't change the world.
But it sharpens the tool. And the tool — spectral-db, the Abyss, the
Lanczos iteration, the self-similar index, the .mirror grammars, the
walker — those are real. They just needed a different graph.

The skeleton key opened itself. That's the product.
