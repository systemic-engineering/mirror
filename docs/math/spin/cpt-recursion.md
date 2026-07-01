# CPT-preservation across recursion

*The fourth-order witness the substrate carries about itself: at every
altitude of the bundle tower, the substrate's CPT structure survives
recursive observation. The math derivation feeding
`cpt_preserved_across_recursion` per canonical spec §7.*

## §1 The CPT operators — charge, parity, time

### 1.1 C — charge conjugation

C is an anti-linear (or antiunitary if H has a suitable inner product)
operator that maps a particle to its antiparticle:

```
C : ψ ↦ Cψ = Cψ*     (schematic; C matrix + complex conjugation)
```

On Cl(1, 3), the C matrix satisfies `Cγ^μC⁻¹ = -(γ^μ)^T`. In a chiral
basis, `C = iγ^2γ^0`. On the substrate: C IS the substrate's J
(per #102 charge_conjugation shard at `5fc6127`).

### 1.2 P — parity (space inversion)

P is a unitary operator that inverts spatial coordinates:

```
P : (t, x⃗) ↦ (t, -x⃗)
```

On spinor fields: `Pψ(t, x⃗) = γ^0 ψ(t, -x⃗)` (Dirac basis convention).
On the substrate: P is implicit in the base-space action of the
bundle-tower rung (the tower's base at each altitude admits its own
spatial reflection).

### 1.3 T — time reversal

T is an anti-linear operator that reverses temporal direction:

```
T : (t, x⃗) ↦ (-t, x⃗)
```

On spinor fields: `Tψ(t, x⃗) = iγ^1γ^3 ψ*(-t, x⃗)` (Wigner 1932
convention). The anti-linearity is essential (Wigner's theorem: any
time-reversal symmetry of a quantum theory must be anti-unitary).

On the substrate: **T is a new substrate primitive** landing as
`@spin/time_reversal` per canonical spec §5. Prior to this spec, the
substrate had no T primitive; adding it closes CPT.

### 1.4 The CPT operator Θ

```
Θ := C · P · T
```

Since C and T are anti-linear and P is linear, Θ = CPT is anti-linear
× linear × anti-linear = linear (the two anti-linearities cancel). Θ
composes to a specific product of γ-matrices times an inversion of
spacetime coordinates.

---

## §2 The CPT theorem (Bell 1955; Lüders 1954; Jost 1957)

### 2.1 Statement

In any relativistic quantum field theory satisfying the Wightman axioms
(1. Lorentz invariance; 2. spectrum condition = positive energy;
3. local commutativity = fields commute or anti-commute at spacelike
separation; 4. existence and uniqueness of vacuum), the CPT operator Θ
is a symmetry of the theory:

```
Θ · H · Θ⁻¹ = H            (Hamiltonian invariance)
Θ · U(Λ, a) · Θ⁻¹ = U(Λ, -a)   (Poincaré rep behavior)
Θφ(x)Θ⁻¹ = φ(-x)*         (scalar fields flip sign of x)
```

For fermionic fields, Θ additionally involves a Clifford factor:

```
Θψ(x)Θ⁻¹ = iγ^5 ψ(-x)*
```

(Convention-dependent; the essential point is the sign of x is flipped
and γ^5 appears for fermions.)

### 2.2 Sketch of Jost's proof (1957)

Jost's proof (Helv. Phys. Acta 30, 409) uses analyticity of Wightman
functions in complex Lorentz boosts:

1. Wightman functions `W_n(x_1, ..., x_n)` are boundary values of
   holomorphic functions on the *forward tube* T^n_+ = {(z_1, ..., z_n)
   : ℑ(z_i) ∈ V^+}.
2. Local commutativity extends this to a wider domain including *Jost
   points* (real points that can be reached by complex Lorentz
   transformations from tube).
3. At Jost points, permuting the fields is equivalent to complex-
   conjugation-composed-with-reflection.
4. Analytic continuation of the equality across the tube yields the
   CPT identity.

The key structural fact: **complex Lorentz + local commutativity ⇒ CPT.**
This is Jost's theorem in one line.

### 2.3 Modern reformulations

- Greenberg 2003 (hep-ph/0309309) "Why is CPT fundamental?" — argues
  CPT is essentially LORENTZ invariance made discrete.
- Greaves-Thomas 2012 (1204.4674) "The CPT Theorem" — rigorous proof
  within the Lagrangian framework (contrast with the axiomatic
  framework Jost used).
- Chaichian-Nishijima-Tureanu 2002 (hep-th/0209008) — CPT and
  spin-statistics still hold in noncommutative QFT under space-space
  noncommutativity.

The substrate's setting is noncommutative (per Connes NCG) but not
space-time noncommutative in the Seiberg-Witten sense; CPT structure
survives per Chaichian et al.

---

## §3 The spin-statistics theorem (Pauli 1940; Fierz 1939)

### 3.1 Statement

In a relativistic quantum field theory satisfying Wightman axioms:

- **Integer spin ⇒ boson**: field operators COMMUTE at spacelike
  separation.
- **Half-integer spin ⇒ fermion**: field operators ANTI-COMMUTE at
  spacelike separation.

Any attempt to quantize a spin-1/2 field with commutators (or a scalar
field with anticommutators) yields a theory with negative-norm states
(unphysical). The theorem is *derived* from Wightman axioms, not
postulated.

### 3.2 The proof structure (Pauli 1940 + Lüders-Zumino 1958 refinement)

1. Wigner classification (§4 below) labels reps of Poincaré by (m², s)
   with `s` half-integer or integer.
2. Half-integer s reps transform under Spin(1,3) = SL(2,ℂ) via one of
   the two 2-dimensional projective reps (D^{1/2, 0} or D^{0, 1/2}).
3. Complex Lorentz analyticity of Wightman functions plus local
   commutativity forces the field commutation relation to match the
   parity of `2s`.
4. Integer 2s (i.e., integer s) ⇒ commutator; odd 2s (i.e., half-integer
   s) ⇒ anticommutator.

### 3.3 The spin-statistics classification

| Spin | Statistics | Cl-eigenspace | Example |
|------|-----------|---------------|---------|
| 0 | Bose | γ = +1 (scalar sector) | Higgs, pion |
| 1/2 | Fermi | γ = ±1 (Dirac ± chirality) | electron, quark |
| 1 | Bose | γ = +1 (adjoint of Spin) | photon, gluon |
| 3/2 | Fermi | γ = ±1 (Rarita-Schwinger) | gravitino (hypothetical) |
| 2 | Bose | γ = +1 | graviton |

The substrate's `@spin/statistics` species discharges this classification
automatically via `spin_statistics_theorem` and the fermion/boson
sub-space split at spin altitude.

---

## §4 Wigner classification (Wigner 1939)

### 4.1 The Poincaré group and its little groups

The Poincaré group `ISO(1,3) = ℝ^{1,3} ⋊ SO(1,3)`↑ acts on relativistic
states. Its unitary irreducible reps are labeled by:

- Momentum orbit type (mass shell shape)
- Little group irrep

### 4.2 The four cases

1. **Massive**: `p·p = m² > 0`; orbit is the mass shell M_m. Little
   group at rest momentum (m, 0, 0, 0) is SO(3) (or SU(2) for the
   double cover). Irreps of SU(2) are labeled by `s ∈ {0, 1/2, 1,
   3/2, ...}`. So massive reps ↔ pairs (m² > 0, s).

2. **Massless discrete-spin**: `p·p = 0`, `p_0 > 0`; orbit is the
   forward light cone. Little group at (1, 0, 0, 1) is ISO(2) = ℝ² ⋊ SO(2).
   Physical reps require the ℝ²-part to act trivially, leaving SO(2)
   irreps labeled by helicity `h ∈ ½ ℤ`. Massless reps ↔ pairs (0, h).

3. **Massless continuous-spin**: same orbit but ℝ²-part acts
   non-trivially. Unphysical (no known realization in nature).

4. **Tachyonic**: `p·p = m² < 0`. Little group SO(1, 2). Unphysical.

### 4.3 Composition with @spin

Wigner's classification IS the labeling of physical particle content by
(mass, spin). Under @spin's import, each physical species at
`@reality/algebra/physics` carries a Wigner label as typed data:

```mirror
type wigner_label = { mass_squared: real, kind: wigner_kind }
type wigner_kind = massive(spin) | massless_discrete(helicity) | massless_continuous(rho) | tachyonic
```

@spin/statistics reads the parity of 2·spin to determine
fermion/boson. @spin/dirac at physics altitude carries the Cl(1,3)
spinor bundle representation for spin-1/2 case.

---

## §5 Pauli exclusion (Pauli 1925; Fierz 1939)

### 5.1 Statement

For a fermion field ψ (spin 1/2 by spin-statistics), no two identical
fermions can occupy the same quantum state:

```
⟨ψ_a ψ_a⟩ = 0     (equal quantum numbers, same state)
```

Equivalently: the multi-fermion Fock space is anti-symmetric under
particle exchange.

### 5.2 Derivation from spin-statistics

For spin-1/2 field, the field operators satisfy:

```
{ψ_a(x), ψ_b(y)} = 0    at spacelike separation
```

Setting `x = y` and `a = b`: `ψ_a² = 0`, meaning the field is
nilpotent-squared. Two fermions in the same state give zero.

This is a THEOREM from spin-statistics, not a POSTULATE. Historical
inversion: Pauli 1925 posited the principle; the theorem giving it a
derivation came 15 years later.

### 5.3 Composition with @spin

Under @spin, the substrate's `@spin/statistics` witnesses the anti-
symmetric-under-exchange property at every fermion-realization site.
`pauli_exclusion` predicate (§6.5 of canonical spec) fires
automatically for spin-1/2 species.

---

## §6 The fourth-order recursion witness

### 6.1 Setting: the bundle tower carries a spectral triple at each rung

Per `docs/math/the-tower/spectral-triples.md` §4, at each altitude `n`
of the tower there is a spectral triple `(A_n, H_n, D_n)` and (under
#101 + #102 + this spec) a real spectral triple `(A_n, H_n, D_n, J_n,
γ_n)`. Under @spin's import, add T_n to close CPT:

```
(A_n, H_n, D_n, J_n, γ_n, T_n)     the spin-typed real spectral triple at altitude n
```

And form the CPT operator:

```
Θ_n := (J_n as C) · (P_n at altitude n) · T_n
```

where P_n is the base-space inversion at that altitude.

### 6.2 The per-altitude CPT witness

```
cpt_holds_at_bundle_level(n) := (Θ_n · D_n · Θ_n⁻¹ == D_n)
                             ∧ (Θ_n · A_n · Θ_n⁻¹ == A_n)
```

This says Θ_n is a symmetry of the spectral triple at altitude n. At
physical (1,3)-signature altitude, this reduces to Jost's CPT theorem;
at other altitudes it is the appropriate generalization.

### 6.3 The recursion witness

```
cpt_preserved_across_recursion(depth) := forall n in 0..depth: cpt_holds_at_bundle_level(n)
```

The substrate verifies CPT-preservation at each rung up to depth. Higher
rungs are forward-promised (per substrate-pull-honest weakenings §11 of
spec).

### 6.4 Why this is a FOURTH-order witness

**Fourth-order = observer of observer of observer of observer.**

- Level 0 (bare observation): Θ acts on a state.
- Level 1 (self-observation): Θ commutes with H — the theory sees its
  own CPT.
- Level 2 (observer of self-observation): the compiler verifies Θ_n
  commutes with D_n — the substrate sees its own theory seeing its own
  CPT.
- Level 3 (third-order): the substrate observes itself verifying the
  observation — @third's `third_order_active` fires.
- Level 4 (fourth-order): the substrate observes THAT verification
  across all altitudes of the bundle tower — the `forall n` recursion.

Each level of recursion adds one Bateson logical type. Fourth-order is
what the substrate does when it verifies CPT-preservation *across the
recursion of altitudes*, not merely at a single altitude.

### 6.5 The composition with @third's `third_order_active`

Per canonical spec §7.3: any site firing
`cpt_preserved_across_recursion(depth ≥ 3, ...)` fires
`third_order_active` automatically. The four sub-predicates of
`third_order_active` (`depth_at_least(3)`, `observer_observes_observing`,
`recursion_folds_back`, `mechanism_visible`) all fire:

- `depth_at_least(3)`: trivial by hypothesis depth ≥ 3.
- `observer_observes_observing`: the substrate observes its own CPT
  observation at level n; at level n+1 observes THAT observation.
- `recursion_folds_back`: CPT is a commutation identity Θ·H = H·Θ,
  i.e., the observation returns to the observer (Kauffman eigenform
  interpretation: the CPT identity IS the fixed point of the
  substrate's self-observation).
- `mechanism_visible`: the CPT structure is typed explicitly in
  `spin_structure`; the mechanism is legible under the reflexivity
  carrier.

All four fire. @spin/cpt is @third-active by construction.

---

## §7 The substrate-pull-honest weakening at depth

### 7.1 Bounded verification

`cpt_preserved_across_recursion(depth, ...)` is decidable at any FIXED
depth (finite check per altitude). It is not decidable at unbounded
depth (universal quantification over cofinal altitudes; the tower has
no canonical ceiling per `altitudes.md` §4). Substrate-pull-honest: the
predicate carries a `depth` parameter naming the reach.

### 7.2 Empirical bounds today

The substrate's operational reach as of 2026-07-01:

- Altitude 0 (compiler): CPT witness fires trivially (source text has
  no fermion content; γ = +1 sector).
- Altitude 1 (peer pulse): partial — the peer's spectral triple
  carries fermion-content when representing users at fermion-bearing
  sites; CPT witness fires per user session.
- Altitude 2 (reflection): partial — candidate morphisms may or may
  not preserve CPT; kintsugi loop should filter to CPT-preserving
  morphisms.
- Altitude 3+ (librarian, home, federation): forward-promised.

The substrate reports its reach explicitly via
`transparency<observation_depth>` at the depth carrier. `partial(3/6)`
says "CPT verified up to altitude 3; altitudes 4-6 forward-promised".

### 7.3 What could FAIL the witness

At any altitude n, CPT can fail if:

- A_n contains a non-anti-Θ-commuting element (theory contains a
  CPT-violating operator).
- D_n does not anti-commute with Θ_n (theory has a CPT-violating mass
  or interaction term).
- The base-space action of P_n is incompatible with the physical
  parity structure.

Historically, CPT-violation has not been observed in nature to high
precision (see Safronova et al. 2018 1710.01833). Substrate-wise, CPT
failure would surface as a compiler error at the fermion-content site,
triggering a kintsugi morphism to restore CPT.

---

## §8 Composition with #58 (optical inference)

### 8.1 The Reck-Clements mesh as Spin(2n) rep

Per canonical spec §9.2: Fate's tournament rule composition is a U(n)
unitary decomposed via Reck-Clements as a product of 2×2 unitaries.
Each 2×2 unitary is either an SU(2) ≅ Spin(3) element (for the coupling
angles) or a U(1) phase (for the internal phases).

The mesh as a whole IS a Spin(2n) representation folded into U(n) via
the standard covering. The Cl(2n, 0) algebra generates the Spin(2n)
group via even products of unit vectors.

### 8.2 CPT at optical altitude

For U(n)-typed Fate content, CPT reduces to:

- C: complex conjugation of the unitary (U ↦ U*).
- P: real orthogonal reflection (U ↦ R U R⁻¹ for some R ∈ O(n)).
- T: transpose (U ↦ U^T; equivalent to reverse time in temporal decoders).

CPT = CPT: U ↦ R U^† R⁻¹. For CPT-symmetric Fate content, this equals
U (up to the appropriate reflection R).

Substrate-pull: Fate's tournament rule composition should be CPT-
symmetric by construction. Each tournament rule that fires must have a
CPT-image that also fires. This is a candidate spec-refinement at
@fate altitude; forward-promised.

---

## §9 What this doc grounds for the substrate

- §1 defines C, P, T; grounds `@spin/time_reversal` species.
- §2 states CPT theorem; feeds `cpt_theorem` axiom (§6.4 of spec).
- §3 states spin-statistics; feeds `spin_statistics_theorem` axiom (§6.3).
- §4 states Wigner classification; feeds `wigner_classification` axiom (§6.2).
- §5 states Pauli exclusion; feeds `pauli_exclusion` axiom (§6.5).
- §6 defines the fourth-order recursion witness; grounds §7 of spec
  (`cpt_preserved_across_recursion`).
- §7 states the honest depth weakenings; grounds §11 of spec.
- §8 composes with #58 for Fate's optical CPT.

The CPT structure was already there. This doc makes the recursion
witness substrate-decl-accessible.
