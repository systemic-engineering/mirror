# Spectral Cryptographic Break — Results

**Date:** 2026-04-04
**Branch:** `break/crypto`
**Status:** Negative result. Clean. Publishable.

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

## Why the Spectral Approach Fails

The "crystal" at any scale is the DFT matrix of Z/nZ.
This matrix is **trivially known** for any cyclic group of any size.
It's always `cos(2πjk/n)`.

The DLP hardness is NOT in the spectral structure.
It's in the **coordinate-to-index mapping**: which curve point (x,y)
corresponds to which group element k. This mapping depends on:
1. The curve equation y² = x³ + ax + b
2. The field arithmetic in GF(p)
3. The choice of generator G

All three are field-specific. Different fields produce unrelated mappings.
Any cross-field correlation would violate Shoup's Ω(√n) lower bound
for generic DLP algorithms.

## What We Built (Valuable)

- **SparseLaplacian** in coincidence: O(n+m) memory, O(m) matvec, Lanczos top-k
- **Component detection** + per-component eigendecomposition
- **Full elliptic curve arithmetic**: point_add, scalar_mul, Tonelli-Shanks mod_sqrt
- **spectral-db integration**: 65k-node graph ingestion, settling in 2 ticks
- **skeleton-key test harness**: clean, reproducible, TDD throughout

## The Sentence

The spectral structure of elliptic curve Cayley graphs is trivially known
(it's the DFT of Z/nZ). The hardness of the discrete log lives in the
coordinate-to-index mapping, which is field-specific and does not transfer
across characteristics. The crystal doesn't tile because there is no
crystal — only a permutation that differs per field.

Both outcomes were always publishable. This one doesn't change the world.
But it sharpens the tool. And the tool — spectral-db, the Abyss, the
Lanczos iteration, the self-similar index — those are real. They just
needed a different graph.
