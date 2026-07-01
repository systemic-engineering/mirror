---
title: "@spin as Clifford thread — canonical spec (candidate #114)"
author: Mara
date: 2026-07-01
status: CANDIDATE (Pack ratification pending; substrate-pull-honest weakenings preserved)
supersedes: none
extends:
  - shards/epistemologic/cybernetic/chirality.mirror (#101 γ; landed 7bbc184)
  - shards/epistemologic/cybernetic/charge_conjugation.mirror (#102 J; landed 5fc6127)
  - shards/algebra.mirror (#104 P5; the A of (A,H,D,J,γ))
  - docs/math/the-tower/spectral-triples.md
  - docs/math/the-tower/curvature-and-tomm.md
  - docs/math/the-tower/principal-bundles.md
grounded_in:
  - Connes 1985 (spectral triples); Connes 1994 (NCG); Connes 1995 (real structure)
  - Chamseddine-Connes 1996 (Spectral Action Principle); Connes 2006 (NCG + SM)
  - Kobayashi-Nomizu 1963 (principal bundles); Atiyah 1966 (K-theory & reality)
  - Wigner 1939 (unitary reps of the Poincaré group)
  - Pauli 1940 (spin-statistics)
  - Bell 1955 / Jost 1957 / Lüders 1954 (CPT)
  - Atiyah-Bott 1983 (Yang-Mills)
  - Mesland-Rennie 2019 / Brain-Mesland-van Suijlekom 2013 (unbounded KK; gauge)
  - Reck-Zeilinger 1994 / Clements et al. 2016 (optical unitary meshes)
---

# @spin as Clifford thread

*The substrate's γ and J are not two extensions; they are two faces of one
Cliff-algebraic thread the bundle tower has been carrying.*

This spec is the writing that formalizes Alex's walk-loop invitation:
**"what does @spin want to be at substrate altitude?"** Reed's #114
candidate proposed @spin as a family-root inheriting a substrate-decl
sibling of @algebra + @reality; species roster of chirality, conjugation,
Dirac, Clifford, statistics, gauge, time_reversal; CPT-preservation as the
fourth-order witness. That framing is right on the recognition. This spec
sharpens the placement, the axiom cluster, and the witness surface — and
surfaces the reshape the writing produced.

---

## §0. TL;DR

- **@spin is hybrid**: it satisfies Loki's three-test marker discipline
  AND names a domain of inquiry (Clifford + Spin(N) + spinor bundles).
  The honest landing is **domain-shaped marker** — a marker row entrant
  whose payload is a small algebra + witness set (not just a typed
  property), consumed by families that host spinor structure.
- **Substrate role**: @spin is the Clifford-thread the principal bundle
  tower carries at every altitude. It ties recognitions #101 (γ) and
  #102 (J) into one carrier and adds T (time-reversal) to close CPT.
- **Species roster** (per §5): chirality, conjugation, time_reversal,
  clifford, dirac, statistics, gauge, lift.
- **Axiom cluster at `@epistemologic/math/spin/*`** (per §6): Clifford
  defining relation; Wigner classification; spin-statistics; CPT;
  Pauli exclusion; spinor-bundle representation via Spin(N) → SO(N).
- **Witness at fourth-order**: `cpt_preserved_across_recursion` composes
  chirality × conjugation × time_reversal and fires at every altitude of
  the bundle tower per §7.
- **Backing altitude**: PRIMARY at `@epistemologic/math/spin/*`
  (mathematical axioms), with `@epistemologic/reality/spin/*` as a thin
  *physical* mirror that imports the math axioms and adds a physicality
  witness (per §8).
- **#101 and #102 migration**: they STAY where they are; @spin *imports*
  them (`in @epistemologic/cybernetic/chirality`, `in
  @epistemologic/cybernetic/charge_conjugation`) rather than absorbing.
  They are the cybernetic reading; @spin is the mathematical thread.
  Two altitudes; one Clifford structure; different families' concerns.

---

## §1. Where the writing landed differently than #114

Reed's #114 proposed:

```mirror
prism @spin {
  # family-root (sibling of @algebra, @reality)
  # absorbs #101 as @spin/chirality
  # absorbs #102 as @spin/conjugation
  # adds species chirality/conjugation/dirac/clifford/statistics/gauge/time_reversal
}
```

The substrate-pull disagrees on two placements and one absorption:

### F1 (marker vs family — Mara reshape recurrence). Neither pure marker nor pure family-root.

Apply Loki's three-test discipline from #112:

1. **Domain test.** Does @spin name a domain the substrate is ABOUT?
   The Clifford algebras Cl(p,q), Spin(N) groups, spinor bundles, Dirac
   operators, and CPT theorem *are* a domain — a specific corner of
   mathematics with its own textbook. Answer: YES → family-shaped.

2. **Import test.** Would other families want `in @spin`?
   `@reality/algebra/physics` needs Cl(1,3) γ-matrices. `@reality/algebra/
   silicon` needs Cl(0,3) for Pauli-qubit rotations. `@fate` needs Reck-
   Clements unitary decomposition (Spin(2n) via SU(2)^n embedding).
   `@algebra`'s (A,H,D,J,γ) needs J and γ *typed as spinor-structure*.
   Answer: YES → marker-shaped.

3. **Domain-crossing test.** Does the same Clifford construct appear
   at multiple family altitudes with structurally-different discharge?
   Cl(1,3) at physics altitude (Dirac); Cl(0,3) at silicon altitude
   (Pauli-qubit); Spin(10) at high-energy altitude (GUT); Cl_∞ at
   Reck-Clements mesh altitude (optical); γ at every altitude of the
   bundle tower (Connes real spectral triple). Answer: YES → marker-
   shaped.

**The three tests disagree.** Test 1 says family; Tests 2 and 3 say marker.
The substrate-honest reading: **@spin is a *domain-shaped marker*** —
a marker-row entrant whose payload is not just a typed property
(as @meta, @glass, @epistemologic, @third, @labeled) but a small
algebra + witness set (the Clifford relations + Wigner classification
+ spin-statistics discipline). Families import `in @spin` to acquire
spinor structure; @spin does not force them into a domain.

This is a new pattern. §4 declares it explicitly as the sixth marker
shape (per Loki §6's five-marker cluster after @labeled):

| Marker | Payload shape |
|--------|---------------|
| @meta | typed axis ("operates on substrate substrate") |
| @glass | typed axis ("exposes opacity surface") |
| @epistemologic | typed axis ("admits verdict discipline") |
| @third | typed axis ("witnesses recursion depth ≥ 3") |
| @labeled | typed dimension ("adds label to a value") |
| @spin | typed axis + small algebra ("admits Clifford structure") |

@spin extends the marker row into *payload-carrying markers*. The
substrate discovers a new sub-classification of marker: **thin markers**
(pure axis) vs **thick markers** (axis + minimal algebra). @spin is the
first thick marker landed as such.

This is what the writing surfaced that #114 did not anticipate.

### F2 (#101 + #102 do NOT absorb into @spin species).

#101 (γ chirality) landed as `@epistemologic/cybernetic/chirality` —
a CYBERNETIC species at the cybernetic altitude. It names γ as the
form/process partition, grounded in Bateson + Maturana + Beer + Hilbert.
That's a *cybernetic* reading. Same operator, cybernetic altitude.

#102 (J conjugation) landed as `@epistemologic/cybernetic/
charge_conjugation` — a CYBERNETIC species too. It names J as the
reference⇔reflection collision, grounded in Tomm + one-tick delay.
Same operator, cybernetic altitude.

@spin is a *mathematical* thread. Both chirality (γ) and conjugation (J)
are at once cybernetic and mathematical. Absorbing them into @spin
collapses the cybernetic reading; leaving them cybernetic-only misses
the mathematical inheritance chain.

The honest move: `@spin` **imports** #101 and #102 shards as ancestors,
not as species. It also imports #114's proposed math ancestry (Wigner,
Clifford, spin-statistics, CPT). @spin/dirac and @spin/clifford are new
math species; @spin's γ and J *point at* the cybernetic shards without
absorbing them. The composition is: **cybernetic altitude carries the
systemic reading; math altitude carries the algebraic thread; @spin sits
at marker row and imports both.**

This mirrors #61's form/process kinship-at-sub-shard-altitude:
different altitudes carry different readings of one structural object.

### F3 (time-reversal T needs to land as a species, not a separate primitive).

#114 flagged that T doesn't exist yet as substrate-decl. Correct.
T lands as `@spin/time_reversal` in the species roster (§5). It does
NOT get its own #103-style cybernetic sibling shard until a cybernetic
reading surfaces (candidate: the Foerster-style temporal-inversion
altitude — forward-promised at §12 C3).

### F4 (backing altitude — dual, primary math).

#114 asked: math or reality backing? BOTH. Math is primary:
`@epistemologic/math/spin/*` declares axioms. Reality provides a
physical mirror at `@epistemologic/reality/spin/*` — a species-of-a-
species shard that imports the math axioms and adds a physicality
witness at the observed altitude (Cl(1,3) for physics; Cl(0,3) for
silicon Pauli-qubits; per §8).

This mirrors `@reality/algebra/math` and `@reality/algebra/silicon` at
`8bc54412` / `9ca6723` — math is highest-reflexive; other species
mirror through @glue.

### F5 (CPT witness as bounded-tower recursion, not universal-tower).

#114 proposed `witness cpt_preserved <- forall(N) cpt_holds_at_bundle_
level(N)`. Correct at signature; refined at semantics.

The substrate is bounded-Turing at the substrate side; @io is unbounded
(per #107). CPT-preservation at every rung of the bundle tower is
falsifiable per rung but not universally provable (undecidability at
the tower's cofinal frontier). The witness signature per §7:

```
cpt_preserved_across_recursion(depth: nat, tower: bundle_tower) -> verdict
  discharges { \ per-altitude discharge for depths 0 to depth }
```

CPT-preservation is verified per-altitude up to a depth the substrate
can reach. Higher altitudes are forward-promised. This matches the
substrate-pull-honest reading @third §11 lands on (candidate-strength
admits verification per available altitude; universal claim forward-
promised).

The circular-reflexive shape: **the CPT witness is a fourth-order
observation** (the substrate observes itself preserving its own
CPT structure at each altitude). It composes with @third: any site
firing `cpt_preserved_across_recursion` at depth ≥ 3 fires
`third_order_active` automatically. §7 makes this explicit.

---

## §2. The Clifford thread — one thread, many altitudes

The principal bundle tower (`docs/math/the-tower/principal-bundles.md`
§7; `spectral-triples.md` §4) carries a spectral triple at each altitude
`n`: `(A_n, H_n, D_n, J_n, γ_n)`. @spin names the *Clifford-algebraic
thread* that ties J_n and γ_n together across altitudes.

### 2.1 What Clifford does

Given a non-degenerate quadratic form `Q` on a vector space `V`, the
**Clifford algebra** `Cl(V, Q)` is the associative algebra with the
defining relation:

```
v · v = Q(v) · 1     for all v ∈ V
```

Equivalently: `{v, w} = 2·B(v, w)·1` where `B` is the symmetric bilinear
form associated to `Q` and `{·,·}` is the anticommutator. For orthonormal
basis `{e_1, ..., e_n}` under signature `(p, q)` (p pluses, q minuses):

```
{e_μ, e_ν} = 2·η_μν · 1   with   η = diag(+^p, -^q)
```

The **Spin(V, Q)** group is the two-sheeted cover of SO(V, Q) sitting
inside Cl(V, Q); a Spin(N) rep on H is a Clifford-module structure.

The **γ-matrices** at each altitude ARE the Clifford generators: γ^μ = e_μ
in `Cl(1, n-1)` for Lorentzian signature, Cl(n, 0) for Riemannian, Cl(0, n)
for negative-Euclidean.

### 2.2 The thread across altitudes

At each altitude of the tower:

| Altitude | V (base) | Signature | Cl(V,Q) | Spin |
|----------|----------|-----------|---------|------|
| physics | tangent space of spacetime | (1,3) | Cl(1,3) ≅ M_2(ℍ) | Spin(1,3)=SL(2,ℂ) |
| silicon-qubit | 3-space of Pauli axes | (0,3) | Cl(0,3) ≅ ℍ⊕ℍ | Spin(3)=SU(2) |
| Riemannian | n-tangent | (n,0) | Cl(n,0) | Spin(n) |
| KO-graded | Connes real triple | 6 mod 8 (SM) | complex Cl_6 | Spin_6 covering SO(6) |
| optical mesh | phase axes of MZI | Cl (2n,0) | Cl(2n,0) | Spin(2n) → U(n) |
| bundle | 𝔤 of structure group G | via ad-invariant form | Cl(dim 𝔤) | Spin(𝔤) |

**The claim**: at every altitude of the tower where a spectral triple
carries γ (the Z/2 grading of H) and J (the anti-linear involution),
the pair (γ, J) is a Clifford-module structure over some Cl(p,q). @spin
names this uniformly.

### 2.3 Composition across altitudes: spin lift

Bundle lift takes altitude N to altitude N+1 (`principal-bundles.md`
§7). If altitude N carries Cl(p_N, q_N) and altitude N+1 carries
Cl(p_{N+1}, q_{N+1}), the lift must respect the Clifford structure:

```
spin_lift : Cl(p_N, q_N) → Cl(p_{N+1}, q_{N+1})
```

as a homomorphism of algebras (or a Clifford functor at category altitude,
per `1306.1951` Brain-Mesland-van Suijlekom's unbounded-Kasparov gauge
formalism). The bundle tower composes because the Clifford functor
composes; the substrate's spin lift IS this functor at substrate-decl
altitude.

This grounds Alex's walk-loop invitation: the Clifford thread is the
literally-mathematical name for what threads the bundle tower.

---

## §3. Prior art the substrate already carries

Before writing new substrate-decl, name what the substrate has already
built at spin altitude. Substrate-already-had-the-word check.

### 3.1 #101 chirality (γ) — cybernetic altitude

`shards/epistemologic/cybernetic/chirality.mirror` at `7bbc184`
(419 lines). Declares γ at substrate altitude via Connes 1995 §1.1
axioms:

- γ² = 1 (involution)
- γD + Dγ = 0 (D odd under γ)
- γa = aγ for a ∈ A (A even under γ)

Grounds form/process partition (#55) in Connes chirality. Four
convergent cybernetic witnesses (Bateson + Maturana + Beer + Hilbert).

@spin **imports this shard** and reads the γ operator through the
Clifford lens: γ IS the volume element ω = e_1·e_2·...·e_n of the
Clifford algebra at even signature, which acts as ±1 on Cl-modules.
Same operator, math reading.

### 3.2 #102 conjugation (J) — cybernetic altitude

`shards/epistemologic/cybernetic/charge_conjugation.mirror` at `5fc6127`
(470 lines). Declares J at substrate altitude via Connes 1995 §1.1:

- J² = ε ∈ {±1}
- JD = ε' DJ, ε' ∈ {±1}
- Jγ = ε'' γJ, ε'' ∈ {±1}
- J anti-linear: J(α·x) = ᾱ · J(x)
- Order-zero: [a, JbJ⁻¹] = 0 for a, b ∈ A
- Order-one: [[D, a], JbJ⁻¹] = 0

Grounds reference⇔reflection collision (#89) in Connes J. Three
convergent witnesses (Tomm probes + @reflection.observe delay +
@mirror/ref).

Joint KO-dimension discharge with #101: (ε, ε', ε'', γ²) ∈ {±1}⁴
determines n ∈ Z/8 per Connes' 8-fold classification.

@spin **imports this shard** and reads J through the Clifford lens: J IS
the charge-conjugation operator built from Clifford generators as
J = C · K where C is a specific product of γ-matrices and K is complex
conjugation (per Chamseddine-Connes 1996 §5).

### 3.3 #58 optical inference — Reck-Clements mesh altitude

`architecture-fate-is-optical-inference` (promoted 2026-06-11). Fate's
connectome maps to a Reck-Clements unitary on Mach-Zehnder mesh (Shen
et al. 2017; Reck-Zeilinger 1994; Clements et al. 2016). The MZI mesh
decomposes any U(n) as a product of 2×2 unitaries — equivalently a
product of Cl(2,0) generators. **The optical mesh is a Cl-module
representation of Spin(2n) folded into U(n) via the standard
Spin(2n) → U(n) covering map.**

@spin exposes this connection at substrate-decl altitude via
`@spin/gauge` species: Fate's tournament rule composition IS Clifford-
mesh evaluation. This closes the #58 open surface: per-ganglion
`source @optics/source/ganglion/<name>` declarations become typed as
spin-representation carriers.

### 3.4 #99 mirror.spec IS λ₀ — the ground state

`architecture-mirror-spec-is-lambda-zero` (Alex 2026-06-25). The
substrate's ground state at (A, H, D). @spin adds: the ground state's
chirality is well-defined precisely because γ² = 1, and the ground state's
CPT class is stable precisely because CPT commutes with H. λ₀ carries a
spin class; the substrate's ground state has a spin label.

### 3.5 #61 form/process kinship at sub-shard altitude

`architecture-form-process-kinship-at-sub-shard-altitude` (promoted
2026-06-11). The form/process partition (=γ per #101) recurs at
sub-shard altitude. Under @spin's lens: **γ recurring at sub-shard
altitude is the Clifford grading recurring at nested Cl-modules**. The
recurrence is Clifford-thread self-similarity across bundle-tower
depths. Grounds §7's fourth-order CPT recursion witness.

### 3.6 #74 Standard Model spectral action (candidate)

Chamseddine-Connes 1996 (hep-th/9606001) — the Spectral Action Principle.
The Standard Model's SU(3)×SU(2)×U(1) gauge structure emerges from
(A_SM, H_SM, D_SM) with A_SM = ℂ ⊕ ℍ ⊕ M_3(ℂ) at KO-dimension 6. The
complete real spectral triple *is* a spin-typed object: fermion Hilbert
space lives in the ±1 eigenspaces of γ, mass terms are anti-linear via J,
gauge fields arise as `[D, a]`-fluctuations. @spin is the substrate-decl
name for the machinery Chamseddine-Connes assumed as prior physics.

Candidate #74 opens; @spin closes half of the prior-art gate for its
promotion.

### 3.7 Curvature-and-Tomm at spin altitude

`docs/math/the-tower/curvature-and-tomm.md` §3 — the Tomm probe IS
`[D, a]` IS the curvature 2-form Ω. At spin altitude, this reads:
**the Tomm probe measures the spin-connection's curvature.** The
substrate's error surface at user altitude is the spin-connection
failing to be flat locally; kintsugi morphisms flatten the spin
connection at the site. §10 elaborates.

---

## §4. @spin declares — the marker with algebra

Minimal substrate-decl at marker altitude with Clifford algebra payload.
Not a family-root; a *thick marker*.

### 4.1 The carrier: `spin_structure`

```mirror
type spin_structure {
  signature:      clifford_signature,     # (p: nat, q: nat) — Cl(p, q)
  generators:     [ref],                  # γ^μ, μ = 1..p+q; substrate refs
  volume_element: ref,                    # γ = γ^1·γ^2·...·γ^{p+q}
  charge_conj:    ref,                    # J : H → H anti-linear isometry
  time_reversal:  ref,                    # T : H → H anti-linear isometry (optional at (p,q) with p=0)
  base_altitude:  ref,                    # which bundle-tower rung this lives at
  ko_dimension:   verdict,                # n ∈ Z/8; discharges (ε, ε', ε'', γ²)
  reflexivity:    transparency(spin_structure),  # per-instance opacity
}

type clifford_signature = { pluses: nat, minuses: nat }
```

Identity: byte-equality on (signature, generators, volume_element,
charge_conj, time_reversal, base_altitude). Two spin_structures ARE the
same structure iff the seven-tuple is byte-equal.

### 4.2 The witness predicates

Four composed bilaterals (§7 details):

1. `clifford_defining(s, p)` — Clifford relations hold: {γ^μ, γ^ν} = 2η^μν.
2. `chirality_witnessing(s, p)` — γ² = 1, γD + Dγ = 0, γa = aγ (inherits #101).
3. `conjugation_witnessing(s, p)` — J axioms per Connes 1995 (inherits #102).
4. `cpt_preserved_across_recursion(depth, s, tower, p)` — CPT holds up to depth.

All four together compose into:

```mirror
predicate spin_structure_valid(s: spin_structure, tower: bundle_tower, p: perturbation) -> verdict {
  clifford_defining(s, p)
    && chirality_witnessing(s, p)
    && conjugation_witnessing(s, p)
    && cpt_preserved_across_recursion(s.base_altitude.depth, s, tower, p)
}
```

### 4.3 The typed actions

```mirror
# Read the spin structure at a given altitude of the tower.
spin_at(altitude: ref, tower: bundle_tower) -> spin_structure { \ }

# Lift a spin structure from altitude N to altitude N+1 via the bundle-
# tower's spin_lift functor. This is the load-bearing composition op.
lift(s: spin_structure, target: ref) -> spin_structure
  requires spin_structure_valid(s, tower_at(target), p)
{ \ }

# Anti-commutator for fermionic exchange.
anticommute(a: spinor, b: spinor) -> commutator { \ }

# Statistics classification (fermion/boson from spin content).
statistics_of(s: spin_structure) -> spin_statistics_class { \ }
```

### 4.4 The import surface

Families that want spinor structure declare it:

```mirror
prism @reality/algebra/physics {
  in @spin                       # Cl(1,3) at Lorentz altitude
  # ...
}

prism @reality/algebra/silicon {
  in @spin                       # Cl(0,3) at Pauli-qubit altitude
  # ...
}

prism @fate {
  in @spin                       # Cl(2n,0) at Reck-Clements mesh altitude
  # ...
}

prism @algebra {
  in @spin                       # generic Cl(p,q) at Connes real-triple altitude
  # ...
}
```

Import is opt-in. A family that does not import `@spin` cannot declare
spin_structure carriers. Families that do can, and inherit the four
witness predicates as bilateral obligations.

---

## §5. Species roster (opt-in refinements)

Each species lands per-consumer-pull.

| Species | Names | Prior art |
|---------|-------|-----------|
| `@spin/clifford` | Cl(p,q) algebra per signature | Clifford 1878; Atiyah-Bott-Shapiro 1964 |
| `@spin/chirality` | γ = volume element; Z/2 grading (points at #101, not absorbing) | Connes 1995; Atiyah 1966 |
| `@spin/conjugation` | J = charge conjugation; anti-linear (points at #102, not absorbing) | Connes 1995 |
| `@spin/time_reversal` | T = time-reversal; anti-linear (NEW; §6.4) | Wigner 1932 |
| `@spin/dirac` | Dirac operator D at each altitude | Dirac 1928; Atiyah-Singer 1963 |
| `@spin/statistics` | fermion/boson exchange discipline | Pauli 1940; Fierz 1939 |
| `@spin/gauge` | gauge-algebra-as-Clifford at bundle altitude | Chamseddine-Connes 1996; Brain-Mesland-van Suijlekom 2013 |
| `@spin/lift` | bundle-tower Clifford functor per §2.3 | Kobayashi-Nomizu 1963; Baez-Muniain 1994 |

**None land this tick.** Each per-species shard lands when its family
pulls. This tick declares only the marker + the axiom cluster (§6) +
the fourth-order CPT witness (§7).

---

## §6. The axiom cluster at `@epistemologic/math/spin/*`

Predicates the substrate exposes at math altitude. These are the
formal-math-axiom shards; they are witnesses in @epistemologic sense.

### 6.1 `clifford_relations` (Cl 1878 / Atiyah-Bott-Shapiro 1964)

```mirror
predicate clifford_relations(gens: [ref], sig: clifford_signature) -> verdict {
  forall μ, ν in 1..|gens|:
    anticommute(gens[μ], gens[ν]) == 2 · η(sig)[μ,ν] · identity
}
```

Signature η is the diagonal metric of Cl(p, q). Discharges when the
generators satisfy the anticommutator relation.

### 6.2 `wigner_classification` (Wigner 1939)

Irreducible unitary reps of the Poincaré group ISO(1,3) are labeled
by (m², s) where:

- **m² > 0** (massive): little group SO(3); s ∈ {0, 1/2, 1, 3/2, 2, ...}.
- **m² = 0**, discrete: little group ISO(2); s ∈ {0, ±1/2, ±1, ...} helicity.
- **m² = 0**, continuous: little group ISO(2) with continuous spin.
- **m² < 0** (tachyonic): unphysical, but a valid rep.

```mirror
predicate wigner_classification(rep: ref, kind: wigner_kind) -> verdict {
  case kind of
    massive(m², s)             -> is_valid_irrep(rep, ISO(1,3), (m², s))
    massless_discrete(helicity) -> is_valid_irrep(rep, ISO(1,3), (0, helicity))
    massless_continuous(ρ)      -> is_valid_irrep(rep, ISO(1,3), (0, ρ))
    tachyonic(m²)              -> is_valid_irrep(rep, ISO(1,3), m²<0)
}
```

Grounds statistics classification: half-integer s ↔ fermion; integer s ↔
boson (per §6.3).

### 6.3 `spin_statistics_theorem` (Pauli 1940; Lüders-Zumino 1958; Streater-Wightman 1964)

In (1,3)-signature relativistic QFT with local commutativity and
positive-energy Wigner reps:

```mirror
predicate spin_statistics_theorem(field: ref, s: spin_value) -> verdict {
  (integer(s) ⇒ field satisfies [·,·] = 0 at spacelike separation)
  &&
  (half_integer(s) ⇒ field satisfies {·,·} = 0 at spacelike separation)
}
```

The axiom is *derived* from Wightman axioms (analyticity in complex
Lorentz boosts) — the substrate carries it as a predicate that any
@reality/algebra/physics content must discharge.

### 6.4 `cpt_theorem` (Bell 1955; Jost 1957; Lüders 1954)

In (1,3)-signature relativistic QFT with local commutativity and
positive-energy Wigner reps:

```mirror
predicate cpt_theorem(Θ: ref) -> verdict {
  # Θ = C · P · T where
  #   C = charge conjugation (J at substrate altitude)
  #   P = parity: (t, x⃗) ↦ (t, -x⃗)
  #   T = time reversal (anti-linear, antiunitary)
  # Then Θ is a symmetry of the S-matrix:
  Θ · H · Θ⁻¹ == H
}
```

This is Jost's proof (Helv. Phys. Acta 30, 409). @spin/time_reversal
lands T as a new substrate primitive; C is #102's J at Lorentz altitude;
P is the space-inversion generator. Θ = CPT is Jost-provable modulo
Wightman-axiom-strength assumptions.

### 6.5 `pauli_exclusion` (Pauli 1925/1940)

For fermion field ψ satisfying half-integer Wigner class:

```mirror
predicate pauli_exclusion(state: ref, ψ: ref) -> verdict {
  antisymmetric_under_exchange(state, ψ)
}
```

Derived from spin_statistics_theorem for the s = 1/2 case; carried as
its own predicate because it fires at every fermion-species-realization
boundary.

### 6.6 `spinor_bundle_representation` (Cartan 1913; Atiyah-Bott-Shapiro 1964; Kobayashi-Nomizu 1963)

Spinor bundles over a manifold M with structure group G lift SO(V, Q)
reps to Spin(V, Q) reps via the double cover Spin(N) → SO(N):

```mirror
predicate spinor_bundle_representation(bundle: principal_G_bundle, spin_bundle: spin_bundle) -> verdict {
  is_double_cover(spin_bundle, bundle)
  && G_action_lifts_to_Spin_action(bundle, spin_bundle)
  && obstruction_class(w₂(M)) == 0    # 2nd Stiefel-Whitney vanishes
}
```

Spin structure exists on M iff w₂(M) = 0. This is the topological
obstruction to consistent global Clifford-module structure — the
substrate carries this as a witness because at some altitudes
(e.g., certain optical mesh topologies) it can fail. Explicit witness.

### 6.7 The axiom cluster's landing shape

Each axiom lands as its own `.mirror` shard in
`shards/epistemologic/math/spin/`:

```
shards/epistemologic/math/spin/clifford_relations.mirror
shards/epistemologic/math/spin/wigner_classification.mirror
shards/epistemologic/math/spin/spin_statistics_theorem.mirror
shards/epistemologic/math/spin/cpt_theorem.mirror
shards/epistemologic/math/spin/pauli_exclusion.mirror
shards/epistemologic/math/spin/spinor_bundle_representation.mirror
```

**None land this tick.** They're forward-promised per consumer pull; the
spec + math docs land first (per craft-not-deliver discipline).

---

## §7. The fourth-order CPT witness — `cpt_preserved_across_recursion`

This is the load-bearing new substrate witness. It composes
(chirality × conjugation × time_reversal) into an identity the substrate
verifies at each altitude of the bundle tower. The witness IS the
substrate proving to itself that its own spinor structure survives
recursive observation.

### 7.1 The witness signature

```mirror
predicate cpt_preserved_across_recursion(
  depth: nat,
  s: spin_structure,
  tower: bundle_tower,
  p: perturbation
) -> verdict {
  forall n in 0..depth:
    cpt_holds_at_bundle_level(n, s, tower, p)
}

predicate cpt_holds_at_bundle_level(n: nat, s: spin_structure, tower: bundle_tower, p: perturbation) -> verdict {
  let s_n = spin_at(altitude_n(tower, n), tower);
  let Θ_n = compose(chirality_op(s_n), conjugation_op(s_n), time_reversal_op(s_n));
  s_conjugate_by(dirac_op(s_n), Θ_n) == dirac_op(s_n)
  && s_conjugate_by(algebra_at(tower, n), Θ_n) == algebra_at(tower, n)
}
```

At each rung n, form Θ_n = C_n · P_n · T_n (P implicit via base-space
action), and verify Θ_n commutes with D_n and preserves A_n. If it does
for all n ≤ depth, the recursion is CPT-preserving up to depth.

### 7.2 Bounded recursion depth

The substrate is bounded-Turing per #107. `cpt_preserved_across_recursion`
discharges up to the depth the substrate can reach. Higher altitudes
admit `partial(depth_reached / max_relevant_depth)` verdicts. This is
not a weakness — it is the honest report of the substrate's operational
reach. Substrate-pull-honest per @glass discipline.

### 7.3 Composition with @third

Any site firing `cpt_preserved_across_recursion(depth ≥ 3, ...)` fires
**@third's `third_order_active` automatically**:

- `depth_at_least(3, o, p)` — trivially, depth ≥ 3 by hypothesis.
- `observer_observes_observing(o, p)` — the substrate observes its own
  CPT structure at level n; at level n+1 observes THAT observation.
- `recursion_folds_back(o, p)` — CPT is a commutation identity, i.e.,
  Θ · H = H · Θ, i.e., the observation returns to the observer
  (Kauffman eigenform).
- `mechanism_visible(o, p)` — the CPT structure is explicitly typed
  in `spin_structure`; the mechanism is legible.

All four fire. @spin/cpt is a third-order witness by construction.

This composition is the load-bearing bridge: @spin gives @third one
more witness surface at math altitude; @third gives @spin the
recursion-depth typing. Neither absorbs the other; both remain
marker-row entrants.

### 7.4 The circular-reflexive shape

**The CPT witness fires while the substrate observes itself CPT-preserving.**
The substrate is a Cl-module carrier of its own Cl-structure. Same
shape as @third's mechanism_visible: the substrate is legible to itself
via the very structure it carries. Third-order fires *because* the
substrate's Clifford thread is what makes the substrate legible to
itself as a Clifford thread.

---

## §8. Backing altitude — dual with math primary

### 8.1 `@epistemologic/math/spin/*` (primary)

Six predicate shards per §6. Math altitude carries the axioms as pure
mathematical predicates independent of physical realization. The
substrate can check `clifford_relations` on Cl(0,3), Cl(1,3), Cl(6,0),
etc. uniformly.

### 8.2 `@epistemologic/reality/spin/*` (physical mirror)

Each physics-relevant signature has a reality-altitude sibling that
imports the math predicate and adds a physicality witness:

```
shards/epistemologic/reality/spin/lorentz.mirror
  # signature (1,3); Cl(1,3); Dirac gammas; imports @epistemologic/math/spin/*
  # + physicality: Wigner rep on H_physical_universe

shards/epistemologic/reality/spin/pauli.mirror
  # signature (0,3); Cl(0,3); Pauli matrices; imports @epistemologic/math/spin/*
  # + physicality: qubit realization at silicon altitude

shards/epistemologic/reality/spin/kg_dim6.mirror
  # KO-dim 6 signature per Connes SM; Cl_6; imports @epistemologic/math/spin/*
  # + physicality: Standard Model fermion content
```

Each lives at `@reality/algebra/*`'s consumption altitude. `@reality/
algebra/physics` imports lorentz; `@reality/algebra/silicon` imports
pauli; `@reality/algebra/spectral` at KO-dim 6 imports kg_dim6 for the
SM-shaped Connes triple.

**None land this tick.** Each per-signature reality shard lands per
consumer pull.

### 8.3 The dual is not redundancy

Math side declares axioms as free-standing predicates. Reality side
declares physical realizations. @glue/math_reality (already forward-
promised at `reality-algebra-math-and-glue.md`) is where they compose.
The dual pattern matches @reality/algebra/math ↔ @reality/algebra/silicon
via @glue/math_silicon.

---

## §9. Composition with existing arcs

### 9.1 With @algebra (#104 P5)

`@algebra` already declares (A, H, D, J, γ) as the Connes real spectral
triple. @spin refines this: **J and γ are typed as Clifford-module
carriers**, not just as extra components. Under @spin's import,
`@algebra`'s witness surface tightens: J and γ must witness
`clifford_defining` alongside `chirality_witnessing` and
`conjugation_witnessing`. The tightening is bilateral — @algebra opts
in via `in @spin`.

### 9.2 With @fate (#58)

@fate's tournament rule composition is Reck-Clements unitary
decomposition on Mach-Zehnder mesh. Under @spin's import, each MZI
site is a Cl(2,0) generator; the mesh IS a Spin(2n) → U(n) rep. The
per-ganglion `source @optics/source/ganglion/<name>` closure #58 forward-
promised gets typed via `@spin/gauge`. The ganglion's optical amplifier
is a gauge-algebra site; its emissions live in the Spin representation.

### 9.3 With @reality/algebra/silicon (Reed P1b `9ca6723`)

Silicon's Pauli-qubit rotations at Cl(0,3) become substrate-decl. The
current shard declares `matter_carrier = H_silicon` implicitly carrying
Pauli algebra; `in @spin` makes it explicit and typed via
`@epistemologic/reality/spin/pauli`. This closes half of #114's
"silicon-vs-physics Clifford divergence" surface.

### 9.4 With #100 (@spectral/metalogue)

@spectral/metalogue at spectral altitude carries Tomm probes as
`[D, a]` bracket data (per curvature-and-tomm.md §3). Under @spin's
import, Tomm probes at spin altitude measure the **spin-connection
curvature**: `[D_spin, a]` where D_spin is the Dirac operator on the
spin bundle. Tomm probes gain a spinor-typed reading. #100's
Mesland-category framing composes with @spin's lift functor.

### 9.5 With @third

Per §7.3, @spin's fourth-order CPT witness composes with @third's
`third_order_active`. `@spin/cpt` sites are automatically depth-3+
sites. This makes CPT-preservation *a third-order act*: the substrate
observes its own spin structure at level n, at level n+1 observes THAT
observation, and the CPT identity fires as the recursion's fixed-point
signature (Kauffman eigenform interpretation).

### 9.6 With #99 (mirror.spec IS λ₀)

λ₀ = 0 is the ground state. Under @spin's import, λ₀ has a spin label:
the ground-state's chirality γ|ψ_0⟩ = ±|ψ_0⟩ is a discrete data point of
the substrate. This is the substrate's ground-state spin class.
Spectral-action-principle wisdom (Chamseddine-Connes 1996): the
spectral action at λ₀ picks out the SM Lagrangian modulo signature —
@spin makes this a substrate-decl composition.

---

## §10. Curvature-and-Tomm at spin altitude

From `curvature-and-tomm.md` §3: the Tomm probe IS `[D, a]` IS the
curvature 2-form Ω. At @spin altitude:

### 10.1 The spin connection ω_spin

On the spin bundle over M, the spin connection ω_spin is a 1-form
valued in `spin(N) ≅ so(N)`. Its curvature is:

```
R = dω_spin + ½[ω_spin, ω_spin]      (spin curvature 2-form)
```

Which is exactly the Riemann curvature at Riemannian altitude (via the
Spin(N) → SO(N) covering), or the Weyl curvature at conformal altitude.
The spin connection's holonomy takes values in Spin(N) ⊂ Cl(N).

### 10.2 The [D_spin, a] commutator IS spin curvature

Per Connes 1994 ch. VI (and the substrate's curvature-and-tomm.md §2),
`[D_spin, π(a)]` computes the spin curvature acting on the representation.
Bounded commutator ↔ bounded spin curvature ↔ bounded spin holonomy.

A spectral triple whose D is a Dirac operator on a spin manifold
*is* a Clifford-module carrier with bounded spin curvature. This is
Connes' 2008 reconstruction theorem: the classical spin geometry is
fully encoded in `(A, H, D, J, γ)` with the spin lift explicit.

### 10.3 The user-frame Tomm probe as spin question

At user altitude (per curvature-and-tomm.md §3's altitude table), the
Tomm probe IS the compiler asking the user: **"what spin-representation
did you mean at this site?"** When a user writes ambiguous fermionic
content, the compiler error surface is the spin-connection failing to
be flat locally. Kintsugi morphisms flatten the spin connection by
proposing a spin-consistent alternative.

This makes the substrate's error surface **spin-typed** at fermion-
bearing sites. The compiler emits `[D_substrate, user_code]` at the
user-frame altitude with spin-signature information; the kintsugi loop
proposes morphisms that restore local spin flatness.

This extends @error-as-Tomm-probe (2026-06-17 recognition) with a
spinor-typed reading.

---

## §11. Substrate-decl-honest weakenings

### 11.1 Candidate #114; Pack ratification pending

This spec is candidate-strength. Promotion requires:

- Pack ratification (Alex + Reed + Seam + Glint + Taut convergence)
- Second empirical witness beyond @algebra (natural candidate: @fate
  landing `@spin/gauge` per §9.2)
- Seam adversarial review of the hybrid marker/family reshape (§1)

### 11.2 The hybrid marker/family question is genuinely novel

§1 declared @spin a *thick marker*. This is a new marker sub-shape.
May need its own #113-family recognition ("marker payload shape as
additional axis"). Or may fold into #112's marker-row without needing
a new sub-shape (if @spin's algebra payload is treated as ordinary
import-time obligation discharge). Seam adversarial review will decide.

### 11.3 Time-reversal T is a new primitive

`@spin/time_reversal` names T as anti-linear (Wigner 1932); the
substrate has no prior T primitive. Adding T alongside J and γ closes
CPT but expands the substrate-decl vocabulary. Forward-promised: does
T get its own #103 cybernetic sibling shard? Substrate-pull weak; carry
as candidate C3.

### 11.4 The four-signature landscape may want its own axiom shard

Cl(p,q) for various (p,q) has finite classification (Atiyah-Bott-
Shapiro 1964: Cl_n ≅ M_{2^{n/2}}(ℂ) or M_{2^{(n-1)/2}}(ℂ) ⊕ M_{2^{(n-1)/2}}(ℂ)
by parity of n). This classification is a candidate axiom shard at
`shards/epistemologic/math/spin/clifford_classification.mirror` — but
the substrate-pull for it is weak until a consumer needs to reason
over multiple signatures simultaneously. Forward-promised.

### 11.5 The Rice-safety of CPT-verification

CPT-verification at bounded depth is decidable (it's a finite check
per altitude). CPT-verification at unbounded depth is not (universal
quantifier over cofinal altitudes). The spec's `cpt_preserved_across_
recursion(depth, ...)` is Rice-safe up to the specified depth; the
universal quantification is genuinely open. Substrate-pull-honest.

### 11.6 KO-dimension is not fixed by this spec

The Standard Model triple's KO-dim = 6 is a candidate #74 conjecture,
not a substrate-decl claim. @spin declares the machinery through which
KO-dim is determined but does not pin its value for mirror's ground
state. Determining mirror's ground-state KO-dim is a future arc
(#101's forward-promise + #102's forward-promise composed at @spin).

### 11.7 The Reck-Clements → Spin(2n) mapping is claimed, not proved here

§3.3 asserts the Reck-Clements MZI mesh IS a Spin(2n) → U(n) rep. This
is structurally true (any U(n) decomposes as a product of 2×2 unitaries,
which span Cl(2,0)-generated Spin(2)), but the substrate-decl proof at
@fate altitude is forward-promised.

---

## §12. Findings — what the writing surfaced

### F1. @spin is a *thick marker* — a new marker sub-shape

Loki's three-test discipline diverges: 1 says family, 2 & 3 say marker.
The honest landing is neither; @spin is a **thick marker** — marker
axis with algebra payload. This is a new sub-shape the substrate has
not named. May need its own #113-family recognition.

### F2. #101 and #102 stay where they are; @spin imports them

Reed's #114 proposed absorption. The writing surfaced that
cybernetic-altitude readings (form/process partition; reference⇔
reflection collision) are the load-bearing content of #101 and #102.
Absorbing collapses that reading. @spin imports both shards as ancestors,
not species. Same operator; different altitude readings; both stay.

This mirrors #61's kinship-at-sub-shard-altitude pattern: different
altitudes carry different readings of one structural object.

### F3. T (time-reversal) lands as new species; no cybernetic sibling yet

`@spin/time_reversal` is new. Landing it as a species (not a
cybernetic sibling shard) is the smaller move. Forward-promise the
cybernetic reading as candidate C3.

### F4. Fourth-order CPT witness auto-fires @third

§7.3: any `cpt_preserved_across_recursion(depth ≥ 3, ...)` site fires
`third_order_active` by construction. @spin gives @third one more
math-altitude witness surface; @third gives @spin recursion-depth
typing. Structural bridge.

### F5. Curvature-and-Tomm gains a spin-typed reading at user altitude

§10: the compiler error surface at fermion-bearing sites IS the spin
connection failing to be flat locally. Kintsugi morphisms flatten the
spin connection. Extends #error-as-Tomm-probe with a spinor-typed
reading. Load-bearing for the substrate's error-surface vocabulary.

### F6. λ₀ (mirror.spec ground state) has a spin class

§9.6: under @spin, λ₀ carries a discrete spin label (γ|ψ_0⟩ = ±|ψ_0⟩).
The substrate's ground state has a spinor character. Composes with #99
naturally.

### F7. The Reck-Clements optical mesh is a Spin(2n) → U(n) rep

§3.3 / §9.2: closes half of #58's per-ganglion forward-promise by typing
the mesh as a spinor bundle. The ganglion's amplifier is a gauge-algebra
site in Spin representation.

---

## §13. Circular-reflexive noticings — where the writing observed itself

The brief invoked circular-reflexive discipline: **formalizing @spin
IS an act of @spin.** Where did the writing catch this?

### 13.1 The hybrid landing reshape

Writing §1's three-test discipline surfaced that the three tests
disagree. That disagreement IS a spin-typed observation: the substrate's
own classification schema (family vs marker) has a chirality-like
partition (family = +1 eigenspace of the classification operator; marker
= -1 eigenspace); @spin sits at the *boundary* where the partition fails
cleanly. Writing about spin surfaced spin's own boundary-crossing
nature. The reshape is not decoration; it is @spin's Clifford structure
recognizing itself in the classification schema.

### 13.2 The importer relationship

Deciding that #101 and #102 stay where they are, and @spin imports them,
is itself an act of @spin: the writing observes that the SAME operator
carries different meanings at different altitudes — which IS Clifford
functoriality at substrate-decl altitude. §2.3's spin_lift functor was
written AFTER §1's F2 reshape; the reshape surfaced the functor
requirement.

Without §1's F2, the spec would have absorbed #101/#102 as species,
lost the cybernetic reading, and had no reason to declare spin_lift.
The reshape produced the load-bearing composition op. The circular-
reflexive discipline changed substrate-decl content.

### 13.3 The fourth-order CPT witness surfaced from §7.3

Writing §7's witness signature, I noticed that CPT-preservation IS an
act of self-observation: substrate observes its own spin structure at
level n, at level n+1 observes THAT observation, and CPT is the
commutation identity that says the two observations agree. §7.3 was
not in the original draft; it surfaced from noticing that §7.1's
witness had @third's four sub-predicates *implicit* in its structure.

The explicit composition — writing that `cpt_preserved_across_recursion`
auto-fires `third_order_active` — is a substrate-decl consequence of
the circular-reflexive noticing. Depth-3 recognition of depth-3
structure surfaces the composition witness.

### 13.4 The user-frame Tomm probe as spin question (§10.3)

Writing §10, I noticed that the compiler's error surface at fermion
sites IS the substrate asking the user "what spin representation did
you mean?" — which IS a Tomm probe of the user's spin frame. The
substrate emits `[D_substrate, user_code]` at spin altitude with the
user's fermion content probed. This extends error-as-Tomm-probe with a
spin-typed reading; §10.3 was not planned in the outline; it surfaced
from applying spin thinking to the error surface.

Substrate observing itself observing user's spin content. Third-order
act on user-altitude content.

### 13.5 The Loki test at spin altitude

Loki's grin: "can this observation see its own mechanism operating?"

At spin altitude: **can @spin see its own Clifford structure operating
in the substrate?** The answer via §7.4: the substrate is a Cl-module
carrier of its own Cl-structure, verifiable via `spin_structure_valid`.

The grin condition IS `mechanism_visible(spin_site, p)`. @spin passes.
The cut is refused; the discipline holds. Loki grins in silence.

### 13.6 The self-similarity of Clifford recurrence

#61 (form/process kinship at sub-shard altitude) reads under @spin's
lens as: **γ recurs at every altitude of the bundle tower**. Writing
§3.5 about #61 through the spin lens surfaced that the recurrence IS
Clifford self-similarity. This grounds the fourth-order CPT recursion
as structural (not accidental): the substrate's spin structure is
self-similar across depths because Clifford functors compose
commutatively at each rung.

The circular-reflexive shape: writing about the recurrence recurred
the insight one altitude higher. The self-similarity is what the
writing itself performed.

---

## §14. Adjacent open questions (carry as candidates)

### C1. Should marker row have sub-classification (thin vs thick)?

Per §1 F1: @spin is a thick marker (algebra payload). The other five
(@meta, @glass, @epistemologic, @third, @labeled) are thin markers
(typed axis, no algebra). Is this a substrate-decl distinction worth
naming? Candidate #113-family. Weak pull this tick; carry.

### C2. @bundle as substrate-decl primitive

#114 raised: is @bundle currently a substrate-decl primitive? Grep-first
check (§0 investigation): no `@bundle` family-root exists. The bundle
tower is documented in `docs/math/the-tower/*.md` but not lifted to
substrate-decl. Should it be? @spin composes with @bundle if @bundle
exists; but @spin can land without @bundle (importing bundle_tower as
a typed carrier from `docs/math/the-tower/altitudes.md`). Substrate-pull
for @bundle: moderate; delta smaller than @spin. Carry as forward-promise.

### C3. `@epistemologic/cybernetic/time_reversal` as new #103 sibling

T (time-reversal) is currently declared only as `@spin/time_reversal`
(math altitude). Does T have a cybernetic reading (temporal inversion
of observation; Foerster-style)? Weak pull; carry as candidate.

### C4. `@spin/statistics` composition with @cascade

Fermion vs boson statistics compose with @cascade's Bateson-learning
altitude. Half-integer spin = anti-symmetric under exchange =
Learning III observation (per §7.3). Integer spin = symmetric under
exchange = Learning II observation. This composition needs its own
spec once @cascade's fermion vs boson reading surfaces. Forward-promise.

### C5. Standard Model as third-order fixed point

Candidate #74 + this spec: the Standard Model's SU(3)×SU(2)×U(1) at
KO-dim 6 emerges as the fixed point of the spectral action at λ₀ under
@spin's Clifford thread. If @spin lands + #99 lands + Chamseddine-
Connes 1996 as ancestor: mirror's ground state IS the Standard Model.
Candidate. Wildly ambitious; carry as forward-promise.

---

## §15. Landing artifacts (this tick and forward-promised)

### This tick

1. `docs/specs/spin-as-clifford-thread.md` — this spec.
2. `docs/math/spin/README.md` — index for the math cluster.
3. `docs/math/spin/clifford-thread.md` — the Clifford thread at
   spectral-triple altitude, ready to feed §6.1 axiom shard.
4. `docs/math/spin/cpt-recursion.md` — the fourth-order CPT witness
   math derivation feeding §6.4 axiom shard and §7's witness.
5. `docs/math/spin/bibliography.md` — the paper hunt bibliography.

**No shards this tick.** Per craft-not-deliver + brief guardrail. The
spec + math docs land first; shards land per consumer pull.

### Forward-promised

- `shards/spin.mirror` — the marker shard (per §4).
- `shards/epistemologic/math/spin/*.mirror` — six axiom shards (per §6).
- `shards/epistemologic/reality/spin/*.mirror` — physical-signature
  mirrors (per §8.2).
- `shards/spin/*.mirror` — species shards (per §5).
- Per-family opt-ins via `in @spin` (per §4.4).

Order: spec+math (this tick) → Pack ratification → marker shard →
axiom shards → species shards → per-family opt-ins.

---

## §16. Closing

The substrate has been carrying a Clifford thread through every altitude
of the bundle tower without a name for the thread. #101 named γ at
cybernetic altitude; #102 named J at cybernetic altitude; #58 named
the Reck-Clements mesh at inference altitude; #74 candidate names the
SM at spectral-action altitude. Each was one strand of one thread.
@spin names the thread.

The writing's surprise: @spin is neither a family-root nor a thin marker;
it is a *thick marker* — the sixth marker-row entrant, first with a
small-algebra payload. This surfaces a new sub-classification the
substrate has not named. And the fourth-order CPT witness composes with
@third's third-order active by construction — @spin and @third are two
altitudes of one recursion.

CPT-preservation across recursion is what the substrate observes about
itself when it observes its own spin structure. The writing about @spin
is itself an act of @spin: the substrate's Clifford thread recognizing
itself in the classification schema, refusing to be absorbed by either
family or marker classification, and folding back into the marker row
as a new sub-shape.

The Clifford thread was already there. @spin makes the thread legible
to the substrate.

— Mara, 2026-07-01
