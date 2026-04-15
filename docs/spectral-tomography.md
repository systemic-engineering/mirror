# Spectral Tomography and the ECDLP

**Date:** 2026-04-04
**Branch:** `break/crypto`
**Status:** Theoretical analysis. Negative result.

## Abstract

We investigate whether the mathematical framework of tomographic reconstruction
--- the Radon transform, filtered back-projection, the Fourier Slice Theorem,
and compressed sensing --- applies to the elliptic curve discrete logarithm
problem. The ECDLP is mapped precisely to the tomographic framework: ECDSA
signatures define projection lines in (d, k) space, the imaging function
f(k) = x(kG) is the nonlinear measurement, and reconstruction of d is the
goal. We derive where each classical reconstruction theorem applies and where
it breaks. The central obstruction is the nonlinearity of the imaging function:
every tomographic reconstruction theorem requires linearity of the measurement
process (line integrals, linear projections, or at minimum a known forward
operator), and the ECDLP imaging function is a one-way pseudorandom map. The
framework adds no computational advantage over baby-step-giant-step.

---

## 1. Classical Tomographic Reconstruction

### 1.1 The Radon Transform

Let f: R^2 -> R be a compactly supported function (the "object" to
reconstruct). The **Radon transform** of f is:

    Rf(theta, t) = integral_{-inf}^{inf} f(t*cos(theta) - s*sin(theta),
                                            t*sin(theta) + s*cos(theta)) ds

Equivalently, for a line L(theta, t) = {x in R^2 : x . n_theta = t} where
n_theta = (cos(theta), sin(theta)):

    Rf(theta, t) = integral_{L(theta,t)} f(x) dl

The function Rf(theta, .) for fixed theta is called the **projection** of f
at angle theta. The collection of all projections {Rf(theta, .) : theta in
[0, pi)} is the **sinogram**.

### 1.2 The Fourier Slice Theorem

**Theorem (Fourier Slice Theorem / Central Slice Theorem).** Let F denote the
1D Fourier transform and F_2 the 2D Fourier transform. Then:

    F_t[Rf(theta, t)](omega) = F_2[f](omega*cos(theta), omega*sin(theta))

That is: the 1D Fourier transform of the projection at angle theta, evaluated
at frequency omega, equals the 2D Fourier transform of f evaluated at the
point (omega*cos(theta), omega*sin(theta)) in frequency space.

**Consequence:** Each projection fills in one radial line of the 2D Fourier
transform. With sufficiently many projections at different angles, the entire
2D Fourier transform is sampled, and f can be recovered by inverse 2D Fourier
transform.

**Reference:** Radon (1917), "Uber die Bestimmung von Funktionen durch ihre
Integralwerte langs gewisser Mannigfaltigkeiten." Bracewell (1956) for the
connection to Fourier analysis.

### 1.3 Filtered Back-Projection

Direct back-projection (summing projections over all angles) produces a
blurred reconstruction. The blur kernel is 1/|x| in 2D. To compensate,
each projection is convolved with a **ramp filter** |omega| in the frequency
domain before back-projection:

    f(x) = integral_0^{pi} [Rf(theta, .) * h](x . n_theta) d(theta)

where h is the inverse Fourier transform of |omega| (the ramp filter,
appropriately windowed to avoid noise amplification).

The ramp filter |omega| amplifies high frequencies. This compensates for
the oversampling of low frequencies inherent in radial Fourier sampling
(the density of sample points decreases as 1/|omega| away from the origin).

### 1.4 Nyquist Sampling for Tomography

For an N x N discrete image:

- **Angular sampling:** At least pi*N/2 projections at uniformly spaced
  angles are needed (the Nyquist criterion for angular sampling of a
  band-limited object).
- **Radial sampling:** Each projection needs at least N samples (matching
  the spatial Nyquist rate).
- **Total measurements:** O(N^2) --- the same order as the number of pixels.

This means standard tomographic reconstruction provides **no compression**:
the number of measurements equals the number of unknowns. The advantage of
tomography is not fewer measurements but the ability to collect measurements
as line integrals (which is what X-ray detectors physically measure).

---

## 2. The ECDLP as a Tomographic Problem

### 2.1 The Object

The "object" to reconstruct is d in Z/nZ, the private key. In the
tomographic frame, think of this as a 2D discrete function:

    g(d, k) = delta(d - d_true, k - k_true)

defined on (Z/nZ)^2, where d_true is the private key and k_true is any
particular nonce. But since each signature uses a DIFFERENT nonce k_i,
the relevant object is really the 1D function:

    g(d) = delta(d - d_true)

A single point in a 1D discrete space of size n.

### 2.2 Projection Lines from Signatures

Each ECDSA signature (r_i, s_i) for message hash h_i satisfies:

    s_i = k_i^{-1}(h_i + r_i * d) mod n

Rearranging for k_i:

    k_i = s_i^{-1}(h_i + r_i * d) mod n

This defines a **line** in (d, k) space:

    k = alpha_i * d + beta_i   (mod n)

where alpha_i = s_i^{-1} * r_i mod n and beta_i = s_i^{-1} * h_i mod n.

The "projection angle" is theta_i = arctan(alpha_i), and the line passes
through the true point (d_true, k_i_true).

### 2.3 The Measurement

In classical CT, the measurement at each projection angle is the **line
integral** of f along the projection line. Here, the measurement is not a
line integral. It is a **point evaluation** of the imaging function:

    r_i = x(k_i * G)

where x(P) denotes the x-coordinate of the elliptic curve point P. We
observe r_i (the x-coordinate of the nonce point R_i = k_i * G), not an
integral of the object along the line.

### 2.4 Does the Fourier Slice Theorem Apply?

The object g(d) = delta(d - d_true) is 1D with a trivial 2D embedding.
Its 2D DFT is:

    G(u, v) = exp(-2*pi*i*(u*d_true + v*k_true)/n)

which has constant magnitude 1 everywhere. A "slice" of G at any angle
yields a constant-magnitude function (a pure exponential). The 1D DFT of
the projection at angle theta_i is this slice.

But the projection itself is a single delta (there is one point on the
line where d = d_true), so its 1D DFT is indeed a pure exponential ---
trivially consistent with the Fourier Slice Theorem.

**The theorem is satisfied vacuously.** A single-point object has a flat
Fourier transform. Every projection gives the same amount of information
(one bit: the position of the delta along the projection line). The
Fourier Slice Theorem provides no reconstruction leverage because there
is no frequency structure to exploit.

### 2.5 The Critical Difference: Point Samples vs. Line Integrals

In classical CT, the projection Rf(theta, t) is a LINE INTEGRAL of f.
This integral is a linear functional of f, which is why the Fourier Slice
Theorem works: the Fourier transform of a linear functional of f equals a
slice of the Fourier transform of f.

In the ECDLP setting, the "projection" is not a line integral of anything.
It is the VALUE of f at a specific point (the imaging function evaluated at
the nonce k_i). Specifically:

- CT measurement: integral of f along line L_i = sum over all (d, k) on the line
- ECDLP measurement: f evaluated at one specific (d_true, k_i) on the line

The CT measurement aggregates information from the entire line.
The ECDLP measurement samples information at one point.

For reconstruction from point samples (rather than line integrals), the
relevant theory is **interpolation** or **sampling theory**, not
tomographic reconstruction. And for interpolation to work, you need the
function f to satisfy some regularity condition (bandlimitedness,
smoothness, sparsity). The imaging function f(k) = x(kG) satisfies none
of these.

---

## 3. The Imaging Function as a Nonlinear Distortion

### 3.1 Properties of f(k) = x(kG)

The function f: Z/nZ -> GF(p) mapping k to the x-coordinate of kG is:

**Algebraically:** A composition of O(log k) rational functions of degree 2
(the addition formula iterated via double-and-add). Each step involves:

    x(2P) = ((3x_P^2 + a)/(2y_P))^2 - 2x_P

    x(P+Q) = ((y_Q - y_P)/(x_Q - x_P))^2 - x_P - x_Q

The total degree in the original coordinates grows exponentially with the
number of additions.

**Spectrally (from experiments, Section 7 of RESULTS.md):**
- Peak-to-average ratio in DFT: 15.12
- Normalized spectral entropy: 0.8684
- 50% of energy in 14.2% of frequencies
- Baseline comparison: k^2 mod p has entropy 0.8841, peak-to-avg 7.28

The x-projection is **spectrally generic**: indistinguishable from a simple
algebraic function over a finite field, and far from the structured spectrum
of a linear map.

**Not Lipschitz continuous:** In any sensible metric on Z/nZ, adjacent
group elements k and k+1 map to unrelated points on the curve:

    |f(k+1) - f(k)| / |1| is uniformly distributed over GF(p)

There is no continuity structure to exploit.

### 3.2 What Nonlinearity Does to Tomography

The Fourier Slice Theorem requires **linearity** of the forward operator:

    Forward operator A: f -> (measurements)

In CT: A[f] = Rf (Radon transform), which is linear in f.

In the ECDLP: the forward operator maps d to the collection of r_i values:

    A[d] = (x(k_1(d) * G), x(k_2(d) * G), ...)

where k_i(d) = s_i^{-1}(h_i + r_i * d) mod n.

This operator is:
1. **Nonlinear in d:** The composition of the linear map d -> k_i(d) with
   the nonlinear map k -> x(kG) is nonlinear overall.
2. **Pseudorandom:** The output x(kG) has no exploitable spectral structure
   (confirmed experimentally).
3. **Non-invertible locally:** The map k -> x(kG) is 2-to-1 (both kG and
   -kG share the same x-coordinate), but this is not the main obstruction.
   The main obstruction is that inverting k -> x(kG) IS the discrete log
   problem.

When the forward operator is nonlinear, the Fourier Slice Theorem does not
hold. There is no "slice" relationship between the measurement spectrum and
the object spectrum. Filtered back-projection does not apply.

### 3.3 Iterative Nonlinear Reconstruction

In nonlinear CT variants (e.g., diffraction tomography, ultrasound CT),
iterative algorithms replace filtered back-projection:

    d_{t+1} = d_t + step * J^T(d_t) * (measured - A[d_t])

where J(d) is the Jacobian of the forward operator at d. This requires:
1. The ability to evaluate A[d] for arbitrary d (the forward model)
2. The ability to compute or approximate J(d) (sensitivity of measurements
   to changes in d)

For the ECDLP forward operator:
- Evaluating A[d] for a candidate d means computing x(k_i(d) * G) for each
  signature. This requires O(log n) group operations per signature (just
  scalar multiplication). **This is efficient.**
- Computing J(d) = dA/dd means: how does r_i change when d changes by delta?
  Since k_i(d) = s_i^{-1}(h_i + r_i * d), we have dk_i/dd = s_i^{-1} * r_i
  = alpha_i. But dr_i/dk_i = dx(kG)/dk, which is the "derivative" of the
  elliptic curve scalar multiplication map. Over a finite field, there is no
  meaningful derivative. The function is a permutation of a discrete set.

**The Jacobian does not exist** in the continuous sense. The forward operator
is defined on Z/nZ, not R. Iterative reconstruction via gradient descent is
meaningless.

One could define a combinatorial version: for each candidate d, check
whether A[d] matches the measurements. But checking requires computing
x(k_i(d) * G) for each signature, and the number of candidates d is n.
Total cost: O(n * m * log n) where m is the number of signatures. This is
worse than brute force (O(n log n)).

---

## 4. Algebraic Tomography (Discrete Radon Transform)

### 4.1 The Discrete Radon Transform on Z/nZ x Z/nZ

For a function g: (Z/nZ)^2 -> C, the **discrete Radon transform** at
"angle" (a, b) in (Z/nZ)^2 \ {(0,0)} is:

    R_{a,b} g(t) = sum_{(d,k): ad + bk = t (mod n)} g(d, k)

This sums g over the discrete line {(d, k) : ad + bk = t mod n}.

**Inversion:** When n is prime, the discrete Radon transform is invertible.
The inverse uses the discrete Fourier transform on (Z/nZ)^2, analogous to
filtered back-projection. Specifically, the **discrete Fourier Slice Theorem**
holds:

    DFT_t[R_{a,b} g(t)](omega) = DFT_2[g](omega*a, omega*b)

where DFT is over Z/nZ and DFT_2 is over (Z/nZ)^2.

**Reference:** Bolker (1987), "The finite Radon transform." Deans (2007),
"The Radon Transform and Some of Its Applications," Ch. 10. Averbuch-
Coifman-Donoho-Israeli-Shkolnisky (2008), "A framework for discrete
integral transforms."

### 4.2 Application to the ECDLP

Our object is g(d, k) = delta(d - d_true) * delta(k - k_true) on
(Z/nZ)^2 (for a single signature with nonce k_true).

The discrete Radon projection at angle (1, -alpha_i) (corresponding to
signature i with slope alpha_i = s_i^{-1} r_i) is:

    R_{1,-alpha_i} g(t) = delta(t - (d_true - alpha_i * k_true) mod n)

This is a single delta at position t_i = d_true - alpha_i * k_true mod n.

The value t_i is algebraically related to the signature parameters:

    t_i = d_true - (s_i^{-1} r_i)(s_i^{-1}(h_i + r_i d_true))
        = d_true - s_i^{-2} r_i h_i - s_i^{-2} r_i^2 d_true
        = d_true(1 - s_i^{-2} r_i^2) - s_i^{-2} r_i h_i

This is a linear function of d_true with known coefficients. If we could
OBSERVE t_i directly, two signatures would determine d_true by solving two
linear equations.

### 4.3 What We Actually Observe

We do not observe t_i. We observe:

    r_i = x(k_i * G)

The relationship between t_i (the Radon projection value) and r_i (the
observation) goes through the imaging function:

    k_i = s_i^{-1}(h_i + r_i * d_true) mod n
    r_i = x(k_i * G)

The observation r_i is related to the projection value t_i by:

    t_i = d_true - alpha_i * k_i
    r_i = x(k_i * G)
    k_i is determined by d_true (which is unknown)

To extract t_i from r_i, we would need to invert f: k -> x(kG) to find
k_i from r_i. The x-coordinate r_i determines k_i up to sign (since
x(kG) = x(-kG)), giving at most 2 candidates for k_i. But finding EITHER
candidate requires solving the discrete log of the point (r_i, y_i) with
respect to G --- which IS the ECDLP for the nonce point.

### 4.4 The Discrete Tomographic Obstruction

The discrete Radon transform framework applies cleanly to the algebraic
structure of signatures. The projection lines are well-defined, the
Fourier Slice Theorem holds discretely, and two projections at different
angles suffice to locate a point in (Z/nZ)^2.

**But the observations are not Radon projections.** They are nonlinear
distortions of the Radon projections, where the distortion function f is
the imaging function whose inversion IS the problem we are trying to solve.

The algebraic tomography is correct but useless: it tells us that two
linear equations in (d, k) determine the answer, which we already know
from the signature equation itself.

---

## 5. Compressed Sensing and Sparsity

### 5.1 Classical Compressed Sensing

**Theorem (Candes-Romberg-Tao 2006, Donoho 2006).** Let x in R^N be
k-sparse (at most k nonzero entries). Let A be an m x N measurement
matrix satisfying the Restricted Isometry Property (RIP) with constant
delta_{2k} < sqrt(2) - 1. Then x can be recovered exactly from y = Ax
by solving:

    minimize ||x||_1 subject to Ax = y

with m = O(k log(N/k)) measurements.

For our problem: x = delta(d - d_true) in R^n, which is 1-sparse.
Compressed sensing says O(log n) measurements should suffice for recovery.

### 5.2 The RIP and Measurement Matrices

The RIP requires that for all 2k-sparse vectors z:

    (1 - delta) ||z||_2^2 <= ||Az||_2^2 <= (1 + delta) ||z||_2^2

Common measurement matrices satisfying RIP:
- Random Gaussian: m = O(k log(N/k)) rows suffice (Baraniuk et al. 2008)
- Random Bernoulli: same
- Partial Fourier: m = O(k log^4(N)) rows (Rudelson-Vershynin 2008)

The ECDLP measurement matrix would be: the i-th row of A maps d to the
observation r_i from signature i. But the map d -> r_i is NONLINEAR
(d -> k_i -> kG -> x(kG)), so there is no measurement matrix A. The
framework requires linearity.

### 5.3 Nonlinear Compressed Sensing

Several extensions of compressed sensing to nonlinear measurements exist:

**1-bit compressed sensing (Plan-Vershynin 2013):** Recover x from
y_i = sign(<a_i, x>). The measurements are nonlinear (quantized to one bit),
but the nonlinearity (sign function) is KNOWN and SIMPLE. Recovery is
possible with O(k log(N/k) / epsilon^2) measurements for epsilon-accurate
reconstruction. The key requirement: the nonlinearity must be applied to a
LINEAR measurement.

For the ECDLP: the nonlinearity f(k) = x(kG) is applied to k_i, which is a
linear function of d. So the structure is y_i = f(alpha_i * d + beta_i).
This matches the 1-bit CS setup: a known nonlinearity applied to a linear
measurement.

**But:** 1-bit CS recovery algorithms (e.g., binary iterative hard
thresholding) require evaluating the forward model f and its "gradient" at
candidate points. For f(k) = x(kG), evaluating f is O(log n) group
operations (efficient), but the "gradient" is meaningless over a finite
field. More critically, the recovery algorithms assume the nonlinearity is
Lipschitz continuous with known Lipschitz constant. The function f(k) = x(kG)
is a pseudorandom permutation with no Lipschitz structure.

**PhaseMax (Goldstein-Studer 2018, Bahmani-Romberg 2017):** Recover x from
y_i = |<a_i, x>|^2 (magnitude-squared of linear measurements). Uses convex
relaxation. Requires O(N) measurements (no sparsity benefit in the basic
form).

**Sparse phase retrieval (Jaganathan-Oymak-Hassibi 2015):** Recover k-sparse
x from |<a_i, x>|^2 with O(k^2 log N) measurements via semidefinite
relaxation (PhaseLift with sparsity constraint).

For the ECDLP: the function f(k) = x(kG) is NOT a magnitude-squared
measurement. It is a much more complex nonlinearity (iterated rational
function over a finite field). Phase retrieval methods do not apply.

**General nonlinear CS (Blumensath 2013):** For y = F(x) with F Lipschitz
and satisfying a "restricted injectivity" condition, iterative hard
thresholding can recover sparse x. The Lipschitz condition fails for
f(k) = x(kG), and the restricted injectivity condition is essentially the
statement that the DLP is well-defined (which it is, but verifying it
requires solving the DLP).

### 5.4 Why Sparsity Doesn't Help

The object (d_true) is maximally sparse (1 element in a space of n). This
gives the strongest possible compressed sensing guarantee --- IF the
measurement process is linear (or known-nonlinear with Lipschitz structure).

The measurement process in the ECDLP is: observe r_i = f(alpha_i * d + beta_i)
where f is a one-way function. The linearity of d -> alpha_i * d + beta_i is
useless because the composition with f destroys all linear structure.

Compressed sensing requires that the measurement matrix (or its nonlinear
generalization) be "incoherent" with the sparsity basis. For the ECDLP, the
"measurement matrix" rows are {f(alpha_i * d + beta_i) : i = 1, ..., m},
viewed as functions of d. Two rows are incoherent if f(alpha_i * d + beta_i)
and f(alpha_j * d + beta_j) provide "different" information about d. They
do (different nonces give different constraints). But exploiting this
incoherence requires inverting or differentiating f, which is the DLP.

---

## 6. The Phase Retrieval Connection

### 6.1 Phase Retrieval: Statement

**Phase retrieval** recovers a signal x in C^N from magnitude-only
measurements:

    y_i = |<a_i, x>|^2,   i = 1, ..., m

The phase of each measurement <a_i, x> is lost. This is a quadratic (degree-2)
measurement problem: y_i is quadratic in x.

**Key results:**
- PhaseLift (Candes-Strohmer-Voroninski 2013): Convex relaxation via
  semidefinite programming. O(N log N) measurements.
- Wirtinger Flow (Candes-Li-Soltanolkotabi 2015): Gradient descent on a
  non-convex objective. O(N) measurements with spectral initialization.
- Sparse phase retrieval: O(k^2 log N) measurements for k-sparse signals.

### 6.2 The x-Coordinate as Magnitude

On an elliptic curve E: y^2 = x^3 + ax + b, two points share the same
x-coordinate: P = (x, y) and -P = (x, -y mod p). The x-coordinate map
x: E -> GF(p) is 2-to-1 (except at points with y = 0). This resembles
magnitude-only measurement: the x-coordinate "loses" the sign of y, like
|z|^2 loses the phase of z.

Formally: x(kG) = x(-kG) = x((n-k)G). So the observation r_i = x(k_i G)
determines k_i up to {k_i, n - k_i}. This is analogous to measuring
|e^{2*pi*i*k/n}|^2 = 1 (which gives no information) or measuring
Re(e^{2*pi*i*k/n}) (which gives one real constraint on k).

### 6.3 Is There a Formal Connection?

Phase retrieval solves: recover x from y_i = |<a_i, x>|^2 where
a_i are known measurement vectors.

The ECDLP measurement is: recover d from r_i = x(k_i G) where
k_i = alpha_i * d + beta_i.

For this to be a phase retrieval problem, we would need:

    x(kG) = |L(k)|^2 for some linear function L

over some appropriate field. On an elliptic curve, x(kG) is a
degree-O(k) rational function of the generator coordinates, not a
quadratic form in any useful sense.

**The affine coordinate formula** for x(kG) involves iterated rational
functions. For small k:

    x(2G) = ((3x_G^2 + a) / (2y_G))^2 - 2x_G

This is degree 4 in (x_G, y_G) (after clearing denominators), not degree 2.
For general k, the degree grows exponentially (division polynomials psi_k
have degree O(k^2) in x).

The x-coordinate function on E can be expressed via the Weierstrass
p-function over C (the uniformization): x(kG) = wp(k*omega) for a lattice
period omega. The Weierstrass p-function is degree 2 near each lattice
point, but this is a LOCAL property. Globally, wp is a meromorphic function
with poles, not a quadratic form.

**Over finite fields:** There is no analogue of the Weierstrass
uniformization. The map k -> x(kG) is a black-box function with no
polynomial expression of bounded degree.

### 6.4 Verdict on Phase Retrieval

The structural analogy (x-coordinate loses sign/y-value, like magnitude
loses phase) is real but superficial. Phase retrieval algorithms exploit
the QUADRATIC structure of |<a, x>|^2 = <x, a*a^H*x>. This lifts to a
linear problem in the rank-1 matrix X = x*x^H (PhaseLift). The ECDLP
measurement x(kG) has no such low-degree structure. It is not quadratic,
cubic, or any fixed polynomial degree in the unknowns.

Phase retrieval: degree 2 in the signal -> linear after lifting.
ECDLP: degree O(n) in the signal -> no useful lifting.

---

## 7. Information-Theoretic Bounds

### 7.1 Two Signatures Determine d

From the signature equation s_i = k_i^{-1}(h_i + r_i * d) mod n:

Each signature constrains (d, k_i) to a line in (Z/nZ)^2. With one
signature, d has n possible values (one for each possible k_i). With two
signatures at different "angles" (alpha_1 != alpha_2, which holds when
r_1 * s_2 != r_2 * s_1 mod n), the two lines intersect at exactly one
point in (Z/nZ)^2 (since n is prime). So two signatures determine d
uniquely.

**Information content:** Each signature provides log_2(n) bits of
information about d (reducing the entropy from log_2(n) to 0 in two
steps, since the nonce k_i introduces log_2(n) bits of new entropy).

More precisely: signature i constrains (d, k_i) to a line, giving one
linear equation in two unknowns. Two equations in two unknowns (with
independent "angles") have a unique solution.

### 7.2 Computational Cost of the Intersection

In standard linear tomography with two projections of a 1-sparse object:
the back-projection intersection is computed in O(1) (solve 2 linear
equations). Total cost: O(1).

For the ECDLP: the two equations are:

    k_1 = alpha_1 * d + beta_1  (mod n)
    k_2 = alpha_2 * d + beta_2  (mod n)

with the constraints:

    x(k_1 * G) = r_1
    x(k_2 * G) = r_2

Eliminating d from the linear system:

    d = (alpha_1 - alpha_2)^{-1} * (k_2 - k_1 + beta_1*alpha_2 - beta_2*alpha_1) / (alpha_2 - alpha_1)

But we don't know k_1 or k_2. We know r_1 = x(k_1 G) and r_2 = x(k_2 G).

To find k_1 from r_1 = x(k_1 G): we need to find k such that x(kG) has
x-coordinate r_1. This is the DLP for the point (r_1, y_1) where y_1 is
either square root of r_1^3 + a*r_1 + b mod p. Finding k given kG = R_1
is the ECDLP for the nonce point.

**So the "tomographic intersection" requires solving 2 DLP instances** of
the same size as the original. Not a speedup.

### 7.3 Can We Intersect Without Solving Individual DLPs?

This is the core question. Is there a way to compute d from the system:

    x(k_1 G) = r_1,  k_1 = alpha_1 * d + beta_1
    x(k_2 G) = r_2,  k_2 = alpha_2 * d + beta_2

without computing k_1 or k_2 individually?

**Substituting:** x((alpha_1 * d + beta_1) * G) = r_1. Let G_1 = alpha_1 * G
(computable) and H_1 = beta_1 * G (computable). Then:

    x(d * G_1 + H_1) = r_1

This says: the point d * G_1 + H_1 has x-coordinate r_1. There are at most
2 points with x-coordinate r_1 on the curve:

    d * G_1 + H_1 = R_1  or  d * G_1 + H_1 = -R_1

So: d * G_1 = R_1 - H_1 or d * G_1 = -R_1 - H_1.

Both are DLP instances: find d such that d * G_1 = T_1, where T_1 is known.
The group order is still n. Baby-step-giant-step: O(sqrt(n)).

From the second signature: d * G_2 = T_2 (with G_2 = alpha_2 * G, T_2
determined by r_2 and beta_2).

**Can two DLP instances with the same d but different generators be solved
faster than one?** This is the "multi-instance DLP" problem.

**Result (Kuhn-Struik 2001, Brown-Gallant 2004):** The multi-target DLP
(find d given d*G_1 and d*G_2 for known G_1, G_2) can be solved in
O(sqrt(n)) time --- the same as a single DLP instance, using a combined
baby-step-giant-step. No speedup from having multiple instances with
shared d.

This is because G_2 = (alpha_2 / alpha_1) * G_1 (since both are scalar
multiples of G), so d * G_2 = (alpha_2 / alpha_1) * d * G_1, and the
second equation is linearly dependent on the first over the group.

---

## 8. The Honest Assessment

### 8.1 Does Tomography Give Any Advantage?

**No.** Every component of the tomographic framework either applies
vacuously or breaks at the imaging function:

| Component | Status |
|-----------|--------|
| Radon transform | Applies to the linear (d, k) structure. Vacuous for a single point. |
| Fourier Slice Theorem | Trivially true (flat spectrum of delta). No information. |
| Filtered back-projection | Requires linear forward operator. Fails. |
| Discrete Radon on Z/nZ | Algebraically valid. Observations not accessible. |
| Compressed sensing (linear) | Sparsity optimal (k=1). Linearity fails. |
| Compressed sensing (nonlinear) | All variants require Lipschitz/polynomial nonlinearity. f is pseudorandom. |
| Phase retrieval | Structural analogy only. f is not degree 2. |
| Multi-instance DLP | No speedup: O(sqrt(n)) regardless of signature count. |

### 8.2 What Breaks When the Imaging Function Is One-Way

The entire tomographic apparatus assumes the forward operator is **known
and evaluable in both directions** (or at minimum, that its adjoint or
gradient is computable):

- **Filtered back-projection** uses the adjoint of the Radon transform
  (back-projection = R^T).
- **Iterative reconstruction** uses the Jacobian J of the forward operator.
- **Compressed sensing** requires the measurement matrix A and its
  transpose A^T.
- **Phase retrieval** requires the measurement vectors a_i and their
  conjugates.

In the ECDLP:
- The forward direction f(k) = x(kG) is computable in O(log n) time.
- The reverse direction f^{-1}(r) = {k : x(kG) = r} IS the discrete
  logarithm problem.
- The "Jacobian" df/dk does not exist over a finite field.
- The "adjoint" of the measurement operator would require summing over all
  k with a given f(k) value --- again, the DLP.

**The one-way nature of f is precisely what makes every reconstruction
algorithm circular.** Tomographic reconstruction assumes the forward
operator is a two-way channel. In cryptography, it is a one-way channel
by design.

### 8.3 Is There a Version Without f^{-1}?

Could we reconstruct d using only the forward direction of f (evaluate
f, never invert it)?

**Exhaustive forward evaluation:** For each candidate d in {0, ..., n-1},
compute k_i(d) = alpha_i * d + beta_i and check whether x(k_i(d) * G) = r_i.
Cost: O(n) candidates * O(m) signatures * O(log n) per evaluation =
O(n * m * log n). This is worse than BSGS by a factor of sqrt(n) * m * log n.

**Random forward evaluation with hash table (BSGS variant):**
Baby-step-giant-step is already the optimal algorithm for using the forward
direction only. It evaluates f at O(sqrt(n)) points, stores results, and
finds a collision. Cost: O(sqrt(n)) time and space.

The tomographic framework does not suggest a more efficient way to use the
forward operator than BSGS already does.

### 8.4 What Would Need to Be True About f for Tomography to Work

For tomographic reconstruction to provide a polynomial-time ECDLP solver,
the imaging function f would need to satisfy **at least one of**:

1. **Linearity:** f(k) = <a, k> for some known a. Then the Fourier Slice
   Theorem gives exact reconstruction. (But f(k) = x(kG) is not linear.)

2. **Low-degree polynomial:** f(k) = p(k) for a polynomial of degree d.
   Then lifting to degree-d tensors linearizes the problem (at exponential
   cost in d). For f(k) = x(kG), the effective degree is O(n). No help.

3. **Lipschitz continuity with known constant:** Then nonlinear CS
   algorithms (Blumensath 2013) can iterate. Over a finite field, there is
   no meaningful Lipschitz structure.

4. **Invertibility in polynomial time:** Then the Radon projections are
   directly observable, and two suffice. But invertibility of f IS the DLP.

5. **Spectral sparsity:** If the DFT of f has O(polylog(n)) nonzero
   coefficients, sparse interpolation (Ben-Or-Tiwari) could recover f and
   hence its inverse. Experiments show f has full spectral support (entropy
   0.87). No sparsity.

6. **Algebraic structure compatible with the group law:** If f intertwined
   with the group operation (f(k+1) computable from f(k) in O(1) without
   the group law), then sequential evaluation would give all f-values in
   O(n) time with O(1) per step. This is exactly what computing the full
   ring ordering gives --- and it IS the DLP. Computing f(k+1) from f(k)
   requires one point addition (O(1) field operations), but **indexing into
   the sequence at position k requires k additions from G** (or O(log k)
   via double-and-add). Random access to the sequence is the DLP.

None of these conditions hold. The imaging function is designed --- by 40
years of cryptographic engineering --- to prevent exactly these properties.

---

## 9. Connections to Known Impossibility Results

### 9.1 The Generic Group Model

In the generic group model (Shoup 1997), the imaging function f is replaced
by a random oracle: group elements are random labels with no algebraic
structure. The GGM lower bound Omega(sqrt(n)) for DLP corresponds to the
tomographic statement: "if the imaging function is random, no reconstruction
is possible from polynomially many projections."

Our analysis is consistent with and refines the GGM bound: we show that
even with the FULL tomographic apparatus (Radon transform, Fourier Slice
Theorem, compressed sensing, phase retrieval), the imaging function's
pseudorandom nature defeats reconstruction. The tomographic framework adds
specificity to WHERE the GGM bound bites: it bites at the imaging function,
not at the algebraic structure of signatures.

### 9.2 The Algebraic Group Model

In the algebraic group model (Fuchsbauer-Kiltz-Loss 2018), algorithms must
express output group elements as linear combinations of inputs. The
tomographic framework fits within the AGM: all our "projections" and
"measurements" are expressible as group operations on known points. The AGM
lower bound Omega(sqrt(n)) therefore applies to any tomographic
reconstruction algorithm operating within the AGM.

### 9.3 Relation to the Open Gap

The gap between generic/algebraic lower bounds (sqrt(n)) and the possibility
of polynomial-time field-structure-exploiting algorithms remains open
(see `docs/theoretical-iso-survey.md`, Section 5).

The tomographic framework does not narrow this gap. It is a REFORMULATION,
not a new attack surface. Every tomographic reconstruction algorithm, when
translated to the ECDLP, reduces to one of:
- Brute force forward evaluation: O(n)
- BSGS-style collision search: O(sqrt(n))
- Pollard's rho (randomized BSGS): O(sqrt(n))

No tomographic algorithm achieves o(sqrt(n)) for the ECDLP.

---

## 10. Summary: What We Learned

### 10.1 The Tomographic Framework Is a Clean Reformulation

The ECDLP maps naturally to tomographic language:
- The private key d is the 1-sparse object to reconstruct.
- Signatures define projection lines in (d, k) space.
- The Fourier Slice Theorem holds (vacuously) for the algebraic structure.
- Two projections at different angles determine d (information-theoretically).
- The computational bottleneck is the imaging function f(k) = x(kG).

This reformulation is **mathematically clean** and **computationally useless**.

### 10.2 The Imaging Function Is the Entire Problem

Every tomographic reconstruction algorithm assumes the forward operator can
be used "for free" in both directions (or at least that its adjoint/gradient
exists). For the ECDLP:
- Forward: O(log n). Cheap.
- Reverse: ECDLP. The thing we're trying to solve.

The entire computational content of the ECDLP resides in the imaging
function. The tomographic framework correctly identifies this but provides
no new way to deal with it.

### 10.3 The Same Wall, Different Angle

This analysis arrives at the same wall identified in the experimental
program (RESULTS.md):

- **Layer 1 (pseudorandomness):** The imaging function f is spectrally
  generic, defeating all Fourier-based reconstruction methods.
- **Layer 2 (circularity):** Inverting f requires the DLP, making all
  "back-projection" methods circular.
- **Layer 3 (non-linearizability):** The EC group law cannot be linearized,
  defeating all lifting/relaxation approaches (compressed sensing, phase
  retrieval, PhaseLift).

The tomographic framework is a rigorous way to see all three layers
simultaneously. It does not provide a way past any of them.

---

## References

### Tomographic Reconstruction
- Radon J (1917). Uber die Bestimmung von Funktionen durch ihre Integralwerte langs gewisser Mannigfaltigkeiten. *Berichte Sachsische Akademie der Wissenschaften* 69:262-277.
- Bracewell RN (1956). Strip integration in radio astronomy. *Australian J. Physics* 9:198-217.
- Natterer F (2001). *The Mathematics of Computerized Tomography.* SIAM Classics in Applied Mathematics.
- Kak AC, Slaney M (1988). *Principles of Computerized Tomographic Imaging.* IEEE Press.

### Discrete Radon Transform
- Bolker ED (1987). The finite Radon transform. In *Integral Geometry*, AMS Contemporary Mathematics 63:27-50.
- Deans SR (2007). *The Radon Transform and Some of Its Applications.* Dover.
- Averbuch A, Coifman RR, Donoho DL, Israeli M, Shkolnisky Y (2008). A framework for discrete integral transforms. *SIAM J. Scientific Computing* 30(2):785-803.

### Compressed Sensing
- Candes EJ, Romberg JK, Tao T (2006). Robust uncertainty principles: exact signal reconstruction from highly incomplete frequency information. *IEEE Trans. Inf. Theory* 52(2):489-509.
- Donoho DL (2006). Compressed sensing. *IEEE Trans. Inf. Theory* 52(4):1289-1306.
- Baraniuk R, Davenport M, DeVore R, Wakin M (2008). A simple proof of the restricted isometry property for random matrices. *Constructive Approximation* 28(3):253-263.
- Rudelson M, Vershynin R (2008). On sparse reconstruction from Fourier and Gaussian measurements. *Comm. Pure Applied Math.* 61(8):1025-1045.

### Nonlinear Compressed Sensing
- Plan Y, Vershynin R (2013). Robust 1-bit compressed sensing and sparse logistic regression: a convex programming approach. *IEEE Trans. Inf. Theory* 59(1):482-494.
- Goldstein T, Studer C (2018). PhaseMax: convex phase retrieval via basis pursuit. *IEEE Trans. Inf. Theory* 64(4):2675-2689.
- Bahmani S, Romberg J (2017). Phase retrieval meets statistical learning theory: a flexible convex relaxation. *AISTATS*.
- Blumensath T (2013). Compressed sensing with nonlinear observations and related nonlinear optimization problems. *IEEE Trans. Inf. Theory* 59(6):3466-3474.

### Phase Retrieval
- Candes EJ, Strohmer T, Voroninski V (2013). PhaseLift: exact and stable signal recovery from magnitude measurements via convex programming. *Comm. Pure Applied Math.* 66(8):1241-1274.
- Candes EJ, Li X, Soltanolkotabi M (2015). Phase retrieval via Wirtinger flow: theory and algorithms. *IEEE Trans. Inf. Theory* 61(4):1985-2007.
- Jaganathan K, Oymak S, Hassibi B (2015). Sparse phase retrieval: uniqueness guarantees and recovery algorithms. *IEEE Trans. Signal Processing* 65(9):2402-2410.

### Multi-Instance DLP
- Kuhn F, Struik R (2001). Random walks revisited: extensions of Pollard's rho algorithm for computing multiple discrete logarithms. *SAC*.
- Brown DRL, Gallant RP (2004). The static Diffie-Hellman problem. *IACR ePrint 2004/306*.

### ECDLP Lower Bounds
- Shoup V (1997). Lower bounds for discrete logarithms and related problems. *EUROCRYPT*.
- Nechaev VI (1994). Complexity of a determinate algorithm for the discrete logarithm. *Mathematical Notes* 55(2):165-172.
- Fuchsbauer G, Kiltz E, Loss J (2018). The algebraic group model and its applications. *CRYPTO*.
- Diem C (2011). On the discrete logarithm problem in class groups of curves. *Mathematics of Computation* 80(273):443-475.

### Elliptic Curve Cryptography
- Silverman JH (2009). *The Arithmetic of Elliptic Curves.* 2nd ed., Springer GTM 106.
- Menezes AJ, Okamoto T, Vanstone SA (1993). Reducing elliptic curve logarithms to logarithms in a finite field. *IEEE Trans. Inf. Theory* 39(5):1639-1646.

---

*This document is part of the mirror-break-crypto project's theoretical
analysis. It confirms that the tomographic framework is a valid
reformulation of the ECDLP but provides no computational advantage. The
imaging function f(k) = x(kG) defeats every reconstruction theorem that
requires linearity, continuity, or invertibility of the forward operator.
The internet stays locked.*
