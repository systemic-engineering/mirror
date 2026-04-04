# Theoretical Survey: Spectral Bridges for ECDLP

**Date:** 2026-04-04
**Branch:** `break/crypto`
**Status:** Literature/theory survey. No new experiments.

## Summary

Every known efficiently computable map from an elliptic curve group to a
simpler algebraic structure either (a) lands in a domain where the DLP is
equally hard, (b) requires the embedding degree to be small (which
cryptographic curves are chosen to prevent), or (c) was broken not by spectral
methods but by algebraic structure that doesn't generalize. No impossibility
proof exists for all spectral approaches -- the gap between Shoup's generic
lower bound and what structured algorithms can achieve remains open -- but
every concrete avenue surveyed here is either dead or requires a breakthrough
with no known path.

---

## 1. Spectral Reduction of Embedding Degree

### What's known

The **Weil pairing** `e_n: E[n] x E[n] -> mu_n` maps pairs of n-torsion
points to n-th roots of unity in GF(p^k)*, where k is the embedding degree
(smallest integer with n | p^k - 1). The **MOV attack** (Menezes-Okamoto-Vanstone,
1993) and **Frey-Ruck attack** (1994) reduce ECDLP to DLP in GF(p^k)* when
k is small.

For cryptographic curves, k is chosen to be large -- typically k ~ n for
random curves over prime fields, making GF(p^k) astronomical. Specifically:

- **Supersingular curves** over GF(p) have k | 6, so MOV applies. These are
  deliberately avoided for ECDLP-based crypto (but used in pairing-based
  crypto with carefully chosen parameters).
- **Random curves** over GF(p) with p prime and #E(GF(p)) = n prime:
  k is the multiplicative order of p mod n. For random p, n, this is
  generically n-1 or (n-1)/2. The extension field GF(p^k) has ~n*log(p)
  bits -- DLP there is harder, not easier.
- **Pairing-friendly curves** (BN, BLS, KSS families) are constructed with
  specific small k (k = 12, 24, 48). NFS-based attacks on GF(p^k)* for these
  k values have driven parameter sizes up significantly (Kim-Barbulescu 2016,
  Guillevic et al. 2020+).

### Can spectral methods reduce k?

No known mechanism. The embedding degree is an arithmetic invariant:
k = ord_n(p), the multiplicative order of the field characteristic modulo the
group order. This is determined by the number-theoretic relationship between
p and n. A "spectral" recharacterization of the pairing would need to compute
the Weil pairing value without working in GF(p^k), which would require
evaluating division polynomials of degree O(n^2) -- itself harder than the DLP.

**The spectrum of the pairing.** The Weil pairing is a bilinear map.
Viewed as a matrix (indexing E[n] x E[n] -> mu_n), it is the Gram matrix of
the Weil form, which is alternating and non-degenerate. Its "spectrum" in any
meaningful sense is determined by the group structure of E[n] = Z/nZ x Z/nZ
(for the full n-torsion over the algebraic closure). Over GF(p), only one
copy of Z/nZ is rational; the other lives in GF(p^k). The spectral structure
of this matrix doesn't bypass the extension field -- it encodes it.

**Key paper:** Galbraith-Paterson-Smart (2008) systematically characterized
pairing-friendly curves and showed the embedding degree is constrained by
CM discriminant. No spectral shortcut exists within their framework.

### Verdict: Dead for cryptographic curves.

The embedding degree is not a spectral quantity that can be "reduced" -- it
is a hard number-theoretic constraint. Curves used in practice have k chosen
to make MOV/Frey-Ruck infeasible. No spectral reformulation changes this.

---

## 2. Isogeny Graphs as Spectral Objects

### What's known

The **supersingular isogeny graph** G_l(p) has vertices = supersingular
j-invariants over GF(p^2) (approximately p/12 vertices) and edges =
isogenies of degree l. These graphs are **(l+1)-regular Ramanujan graphs**
(Pizer 1990, confirmed by the Ramanujan-Petersson conjecture, proved by
Deligne 1974 for modular forms).

**Ramanujan property.** For an (l+1)-regular graph on N vertices, the
non-trivial eigenvalues lambda satisfy |lambda| <= 2*sqrt(l). This is the
optimal spectral gap. It means:

- Random walks mix in O(log N) steps (rapid mixing)
- The graph is an optimal expander
- Short paths between any two vertices exist (diameter O(log N))

**Cryptographic use.** SIDH (Jao-De Feo 2011) and CSIDH (Castryck-Lange-
Martindale-Panny-Renes 2018) built key exchange on the hardness of finding
isogeny paths in these graphs.

**SIDH was broken** (Castryck-Decru 2022, Maino-Martindale 2022, Robert
2023). The attack exploited the auxiliary torsion point information that
SIDH provided, not the spectral structure. It used the Kani-Frey theorem to
translate the problem to genus-2 Jacobians. The spectral gap was irrelevant
to the break.

**CSIDH remains standing** (as of early 2025). CSIDH uses the *ordinary*
isogeny graph with class group action, not auxiliary points. Best attacks
are quantum subexponential (Kuperberg/Regev-style algorithms for abelian
hidden shift).

### Spectral attacks on isogeny problems

The Ramanujan property *works against the attacker*:

1. **Optimal expansion = no bottlenecks.** A spectral attack would look for
   community structure, bottlenecks, or low-conductance cuts. Ramanujan graphs
   have none. The Cheeger inequality gives conductance >= (l+1 - 2*sqrt(l))/2,
   which is Theta(l) -- no sparse cuts exist.

2. **Eigenvalue multiplicity.** The spectral gap is uniform. There is no
   "Fiedler structure" to exploit because the second eigenvalue is already
   near-maximal. All non-trivial eigenvalues cluster in [-2*sqrt(l), 2*sqrt(l)],
   giving no geometric information about specific paths.

3. **Path-finding vs. spectral structure.** Finding a specific isogeny path
   (the hard problem) requires local information. The spectrum gives global
   mixing properties. Knowing the graph is Ramanujan tells you paths exist
   and are short, but not how to find them without enumeration.

4. **Spectral hash functions.** Charles-Goren-Lauter (2009) constructed hash
   functions from walks on supersingular isogeny graphs precisely because the
   Ramanujan property guarantees pseudo-random behavior -- the spectrum works
   FOR security, not against it.

**Relevant work:** Eisentrager-Hallgren-Lauter-Morrison-Petit (2018)
connected the endomorphism ring computation problem to isogeny path-finding.
Wesolowski (2022) proved equivalence between computing endomorphism rings
and computing isogenies for supersingular curves. Neither used spectral
methods -- both are algebraic.

### Could spectral methods help anyway?

One speculative direction: the **Brandt matrix** / **Hecke operator** T_l
on the space of modular forms of weight 2 and level p has the same
eigenvalues as the adjacency matrix of the supersingular isogeny graph
(Mestre 1986, Pizer 1990). If one could extract DLP information from
Hecke eigenvalues... but Hecke eigenvalues are Fourier coefficients of
newforms, which encode L-function data. Computing them is itself hard
(polynomial in p for fixed level, but the connection to path-finding is
unclear).

### Verdict: Working against you. The Ramanujan property is the enemy.

---

## 3. Algebraic Geometry Codes and Spectral Structure

### What's known

**Goppa codes** from elliptic curves: Let E/GF(q) be an elliptic curve with
rational points P_1,...,P_n and a divisor G. The algebraic geometry code
C(D,G) is the image of the Riemann-Roch space L(G) under evaluation at the
P_i.

**Parameters:** For an elliptic curve (genus 1), Riemann-Roch gives
dim L(G) = deg(G) for deg(G) >= 1. The code has length n = #E(GF(q)) - 1,
dimension k = deg(G), and minimum distance d >= n - deg(G).

**Spectral connection:** The "spectrum" of an AG code can mean:
- The **weight enumerator** and its MacWilliams dual
- The **zeta function** of the curve (which determines #E(GF(q^m)) for all m)
- The eigenvalues of the **code's Tanner graph**

The zeta function Z(E/GF(q), T) = (1 - alpha*T)(1 - alpha_bar*T) / ((1-T)(1-qT))
where alpha + alpha_bar = q + 1 - #E(GF(q)) (the Frobenius trace). This is
exactly what SEA computes.

### Connection to DLP?

**There is none that helps.** The AG code encodes the *linear* structure of
function spaces on the curve. The DLP is about the *group* structure of
rational points. These are different: L(G) is a vector space over GF(q),
while E(GF(q)) is a finite abelian group under the chord-tangent law.

The weight enumerator of C(D,G) depends on the intersection of L(G) with
hyperplanes in GF(q)^n -- this is determined by the divisor class group
geometry, not the point group. You can compute the full weight enumerator
without knowing any discrete logs, and knowing it doesn't help compute them.

**Interesting near-miss:** The **Duursma zeta function** (2001) unifies
the code zeta function with the curve zeta function for AG codes. But
the unification is at the level of counting (intersection numbers, not
group navigation).

### Verdict: No connection. Different mathematical structures.

---

## 4. SEA-Style Modular Decomposition for DLP

### What's known

The **Schoof-Elkies-Atkin (SEA) algorithm** computes #E(GF(p)) in
O(log^5 p) time (or O(log^4 p) with fast arithmetic). It works by:

1. For small primes l, compute the Frobenius trace t mod l
2. Use CRT to reconstruct t mod (product of small l's) > 4*sqrt(p)
3. Then #E = p + 1 - t

The key insight: the Frobenius endomorphism phi satisfies
phi^2 - t*phi + p = 0 on E[l] (the l-torsion), so t mod l is determined
by the action of Frobenius on E[l], which can be computed using division
polynomials of degree O(l^2).

### Can we do the same for DLP?

**The critical difference:** SEA computes a *global invariant* (the group
order n = #E(GF(p))). The DLP asks for a *specific element* (the integer k
such that Q = kG). Global invariants can be determined modulo small primes
independently. Individual discrete logs cannot -- at least, not without
structure that nobody has found.

**What would be needed:** A family of efficiently computable functions
f_l: E(GF(p)) -> Z/lZ such that f_l(kG) = k mod l, for small primes l.
Then CRT would give k. But f_l(kG) = k mod l means f_l is a group
homomorphism from <G> to Z/lZ (up to the constant f_l(G)). Such a
homomorphism exists abstractly (project onto the l-part) but computing it
is equivalent to the DLP modulo l.

**Pohlig-Hellman** does exactly this when n has small factors: it computes
k mod l for each prime l | n using O(sqrt(l)) work. For n prime (as in
cryptographic curves), there are no small factors.

**Summation polynomials** (Semaev 2004) attempted a decomposition approach:
express the DLP as a system of polynomial equations over factor bases.
For curves over extension fields GF(p^n), Gaudry, Diem, and others showed
sub-exponential algorithms. For curves over prime fields, the polynomial
systems have degree that grows exponentially -- no speedup.

**Relevant work:** Petit-Quisquater (2012) connected summation polynomials
to Serre's algebraic geometry bounds. Huang-Raskind (2020) studied the
Brauer group obstruction to index calculus on elliptic curves.

### Verdict: Structurally different problem. SEA computes a global invariant; DLP asks for a local one. The decomposition doesn't transfer.

---

## 5. Lower Bounds and Impossibility Results

### Generic group model (Shoup 1997, Nechaev 1994)

In the **generic group model (GGM)**, group elements are represented by
random labels, and the only operations are group law and equality testing.
Any algorithm solving DLP in a generic group of order n requires
Omega(sqrt(n)) group operations. This matches Pollard's rho.

**What the GGM does NOT cover:** Algorithms that exploit the representation
of group elements. For elliptic curves, points have coordinates (x,y) in
GF(p), and the field arithmetic is available. The GGM says nothing about
algorithms that use x-coordinates, field structure, etc.

### Auxiliary input model

**Corrigan-Gibbs and Kogan (2018)** studied DLP in groups with auxiliary
input (e.g., a table of some discrete logs). They showed that even with
polynomial auxiliary input, generic algorithms still need Omega(sqrt(n))
operations. But again, this is generic.

### Algebraic group model (AGM, Fuchsbauer-Kiltz-Loss 2018)

The **AGM** strengthens the GGM by requiring algorithms to output group
elements as linear combinations of previously seen elements. DLP lower
bounds in the AGM are Omega(sqrt(n)), matching the GGM. The AGM captures
more structure than the GGM but still doesn't model field arithmetic.

### Index calculus barriers for prime field EC

**Diem (2011)** proved that for elliptic curves over GF(p) with p prime,
any index calculus algorithm using factor bases of size B requires
exp(Omega(sqrt(log n))) relations, giving no sub-exponential algorithm.
This is specific to prime field curves.

**The Semaev-Wenger barrier:** Summation polynomials of degree m over
GF(p) have degree growing as m!, which kills Weil descent/Grobner basis
approaches for prime fields. Over extension fields GF(p^n), the situation
is different -- Gaudry (2009), Diem (2011), and Joux-Vitse (2012) achieved
sub-exponential for specific extension degrees.

### What's truly open

**No proof exists that ECDLP over prime fields requires Omega(sqrt(n))
time for ALL algorithms.** The known lower bounds are:

1. Omega(sqrt(n)) in the generic group model -- but real algorithms aren't generic
2. Omega(sqrt(n)) in the algebraic group model -- but this doesn't model field structure
3. No sub-exponential index calculus for prime fields -- but this only rules out one technique
4. No proof that field structure can't help -- this is wide open

**The honest gap:** There is a possibility space between "generic algorithms
need sqrt(n)" and "there exists a polynomial-time algorithm using field
structure." This gap is where Shor's algorithm lives (quantum), where
hypothetical spectral bridges would live (classical), and where the
actual hardness assumption for elliptic curve cryptography sits.

**Relation to P vs NP:** ECDLP is in NP (and coNP, since it has unique
solutions). If P != NP, ECDLP could still be in P (it's not known to be
NP-complete). A proof that ECDLP requires super-polynomial time would
separate P from NP under standard assumptions, which nobody expects to
achieve soon.

### Verdict: The gap is open. No impossibility proof exists for structured (non-generic) algorithms. But no concrete approach has made progress either.

---

## 6. The Big Question: Does a Spectral Bridge X Exist?

### Formal constraints

We seek an algebraic structure X with:
- An efficiently computable map phi: E(GF(p)) -> X
- DLP in X is easy (polynomial time)
- phi preserves group structure: phi(kG) is easily related to k

### What phi can and cannot be

**If phi is a group homomorphism** to a group where DLP is easy (e.g.,
(Z/nZ, +) itself), then phi directly solves ECDLP. Such a phi would give
a polynomial-time algorithm, contradicting widespread belief. No such phi
is known.

**If phi is a group homomorphism to GF(p^k)***, this is the Weil/Tate
pairing. DLP in GF(p^k)* is sub-exponential via NFS, but for cryptographic
curves k ~ n, so GF(p^k)* is too large.

**If phi maps to a graph/matrix with spectral structure**, the question is
whether spectral invariants of X encode k. Our experiments (1-6) showed:
the Cayley graph spectrum is trivially known and doesn't encode k. Any
other graph derived from the curve would need a non-trivial construction.

### Known candidates for X and why they fail

| Candidate X | Map phi | Why it fails |
|---|---|---|
| GF(p^k)* | Weil/Tate pairing | k too large for crypto curves |
| Jacobian of higher-genus curve | Weil descent | Only works over extension fields |
| Isogeny graph path | Isogeny walk | Finding the path IS the hard problem |
| Z/nZ (directly) | ??? | Would break ECDLP; none known |
| Matrix group GL(m, GF(q)) | Representation | DLP in matrix groups is also hard |
| Class group of number field | CM theory | Class group DLP is also hard |
| Lattice | ??? | No known group-preserving embedding; lattice DLP is also hard |

### The representation-theoretic obstruction

A spectral bridge is essentially a *representation* of the group E(GF(p))
in some space where the group action is diagonalizable and the eigenvalues
are computable. For a cyclic group Z/nZ, the irreducible representations
are the characters chi_w(k) = exp(2*pi*i*k*w/n). The DFT matrix IS the
change-of-basis to the eigenbasis. But evaluating chi_w(P) for a curve
point P requires knowing k = log_G(P) -- the representation is circular.

Any representation of Z/nZ in a space where we can efficiently compute the
matrix entries would immediately give us the discrete log. The hardness of
ECDLP is precisely the statement that no such efficient representation exists
(when "representation" means "from the coordinate description of points").

### What would a breakthrough look like?

It would need to be a map phi: E(GF(p)) -> X that:
1. Can be computed from (x,y) coordinates without knowing the discrete log
2. Has a target X where the image phi(kG) reveals k (or k mod l for CRT)
3. Is not equivalent to computing the pairing, Frobenius, or any known map

Such a map would essentially need to "linearize" the group law -- turn the
non-linear (rational function) addition on E into a linear operation. The
group law on E is:

    x(P+Q) = ((y_Q - y_P)/(x_Q - x_P))^2 - x_P - x_Q

This rational function of degree 2 in the coordinates has no known
linearization over any base. Formal group theory gives a local
linearization (the formal logarithm) that converges p-adically, but
computing it globally requires O(n) operations (Satoh's algorithm for
point counting uses this idea, but only for the Frobenius trace, not
individual logs).

### Impossibility evidence (short of proof)

1. **40+ years of cryptanalytic effort** on ECDLP by the world's best
   mathematicians and cryptographers have produced no sub-exponential
   classical algorithm for curves over prime fields.

2. **The pairing is the best known bridge**, and it fails for generic curves
   because k ~ n. This is not accidental -- Balasubramanian-Koblitz (1998)
   proved that for random curves, the embedding degree is large with
   overwhelming probability.

3. **Representation-theoretic circularity** (above): any spectral
   decomposition of the group action requires the discrete log to evaluate.

4. **Shor's algorithm works** because quantum mechanics provides a physical
   mechanism (superposition + interference) to evaluate all characters
   simultaneously. Classically, you'd need to evaluate exp(2*pi*i*k*w/n)
   for all k, which requires knowing k for each point. The quantum speedup
   is not about finding a better X -- it's about evaluating the map to
   the SAME X (the character group) in superposition.

5. **Dequantization attempts fail** (as documented in RESULTS.md):
   the hidden subgroup problem approach that Shor uses has a proven
   Omega(sqrt(n)) classical lower bound for abelian HSP (Ettinger-Hoyer-
   Knill 2004).

---

## 7. Honest Assessment

### Is the "spectral bridge" direction theoretically possible?

**There is no proof it's impossible.** The absence of a proof that ECDLP
requires super-polynomial time means we cannot rule out any approach. This
is a consequence of the broader P vs NP situation, not specific to spectral
methods.

### Is it promising?

**No.** Every concrete spectral approach either:
- Reduces to a known-hard problem in the target domain
- Requires information (the discrete log) to set up the spectral decomposition
- Exploits structure that cryptographic curves are designed to lack

The five experiments in this project confirmed these barriers concretely for
a small curve. The literature confirms them theoretically for all curves.

### What would change this assessment?

1. A new algebraic structure X not on the list above, with a non-obvious
   connection to elliptic curves
2. A way to compute Weil pairing values in a smaller extension field
   (would break pairing-based crypto too)
3. A classical simulation of quantum Fourier sampling (would break much
   more than ECDLP)
4. A proof that P = NP (would break everything)

None of these has any known path forward.

### The structural reason

The deepest reason the spectral bridge fails is this: the group E(GF(p))
is "accidentally cyclic." It is cyclic (or nearly so) not because of any
intrinsic spectral structure, but because of an arithmetic coincidence
(the group order happens to be prime or nearly prime). The coordinates
(x,y) are related to the group index k by a highly non-linear map (iterated
rational function composition). Spectral methods linearize; the EC group
law is essentially non-linearizable over finite fields.

Shor's algorithm doesn't linearize either -- it evaluates the non-linear
map in superposition. There is no classical analogue of this.

---

## References

### Foundational
- Menezes-Okamoto-Vanstone (1993). Reducing elliptic curve logarithms to logarithms in a finite field. IEEE Trans. Inf. Theory.
- Frey-Ruck (1994). A remark concerning m-divisibility and the discrete logarithm in the divisor class group of curves. Math. Comp.
- Shoup (1997). Lower bounds for discrete logarithms and related problems. EUROCRYPT.
- Nechaev (1994). Complexity of a determinate algorithm for the discrete logarithm. Math. Notes.
- Balasubramanian-Koblitz (1998). The improbability that an elliptic curve has subexponential discrete log problem under the Menezes-Okamoto-Vanstone algorithm. J. Cryptology.

### Isogeny graphs
- Pizer (1990). Ramanujan graphs and Hecke operators. Bull. AMS.
- Charles-Goren-Lauter (2009). Cryptographic hash functions from expander graphs. J. Cryptology.
- Jao-De Feo (2011). Towards quantum-resistant cryptosystems from supersingular elliptic curve isogenies. PQCrypto.
- Castryck-Lange-Martindale-Panny-Renes (2018). CSIDH: An efficient post-quantum commutative group action. ASIACRYPT.
- Castryck-Decru (2022). An efficient key recovery attack on SIDH. EUROCRYPT 2023.
- Maino-Martindale (2022). An attack on SIDH with arbitrary starting curve.
- Robert (2023). Breaking SIDH in polynomial time. EUROCRYPT 2023.
- Wesolowski (2022). The supersingular isogeny path and endomorphism ring problems are equivalent. FOCS.
- Eisentrager-Hallgren-Lauter-Morrison-Petit (2018). Supersingular isogeny graphs and endomorphism rings. EUROCRYPT.

### Lower bounds and models
- Fuchsbauer-Kiltz-Loss (2018). The algebraic group model and its applications. CRYPTO.
- Corrigan-Gibbs-Kogan (2018). The discrete-logarithm problem with preprocessing. EUROCRYPT.
- Diem (2011). On the discrete logarithm problem in class groups of curves. Math. Comp.
- Ettinger-Hoyer-Knill (2004). The quantum query complexity of the hidden subgroup problem is polynomial. Inf. Proc. Lett.

### Index calculus and summation polynomials
- Semaev (2004). Summation polynomials and the discrete logarithm problem on elliptic curves.
- Gaudry (2009). Index calculus for abelian varieties of small dimension and the elliptic curve discrete logarithm problem. J. Symb. Comp.
- Joux-Vitse (2012). Cover and decomposition index calculus on elliptic curves. PKC.
- Petit-Quisquater (2012). On polynomial systems arising from a Weil descent. ASIACRYPT.
- Huang-Raskind (2020). Brauer group obstruction for index calculus on elliptic curves (preprint).

### Pairing-based
- Galbraith-Paterson-Smart (2008). Pairings for cryptographers. Discrete Applied Math.
- Kim-Barbulescu (2016). Extended tower number field sieve. CRYPTO.

### SEA and point counting
- Schoof (1985). Elliptic curves over finite fields and the computation of square roots mod p. Math. Comp.
- Elkies (1998). Elliptic and modular curves over finite fields and related computational issues.
- Satoh (2000). The canonical lift of an ordinary elliptic curve over a finite field and its point counting.

### Representation theory
- Serre (1977). Linear representations of finite groups. Springer.
- Silverman (1986/2009). The arithmetic of elliptic curves. Springer.

---

## Note on web search

Web search was unavailable during this survey. The analysis is based on the
standard cryptographic literature through early 2025. Key results that may
have appeared in 2025-2026 and could be relevant:
- Any progress on CSIDH cryptanalysis via spectral/algebraic methods
- New lower bounds in the algebraic group model
- Advances in lattice-based approaches to ECDLP
- Quantum algorithm improvements for elliptic curve HSP

If web search becomes available, these four areas would be the priority
for updating this survey.
