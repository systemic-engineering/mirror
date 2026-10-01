---
title: "The comma between observers — math foundation"
author: Mara
date: 2026-10-01
status: candidate (awaiting Seam crowbar)
visibility: protected
builds-on:
  - "mirror abd720a — Glasswall triple corrected in place"
  - "prism d0b50a7 — docs/math/FLOOR.md triple corrections"
  - "prism 64442a0 (reed/settle-loop-1, unmerged) — Instrument D = δ + δ*"
  - "prism d983c4a — settle certifies b₀ or refuses"
  - "prism 8de8bd0 — Solo and Bleed"
  - "prism 30d196f — O3"
---

# The comma between observers

*Mara, 2026-10-01. Composed with Reed from Alex's question. Written to be broken by Seam.*

## The question

> What has to be true of the instrument (what A is and how it acts on C⁰ ⊕ C¹), of the gauge (Ω as reachable futures on cochains), and of the boundary each observer declares, for the comma *between* two observers — the disagreement neither can tune away alone — to become a question they ask each other, so that asking it widens what both can reach without either absorbing the other? And where does that recursion stop moving?

Labels: **PROVED** (proof given), **MEASURED** (commit/test cited), **CONJECTURE** (one line why open), **LEAD** (unverified pointer).

## §0 Ground and corrections carried

### §0.1 The triple as it stands in code

A graph X = (V, E). Edges carry a canonical orientation e = (s(e), t(e)) with s(e) < t(e) (prism `instrument.rs`, `oriented`). Real scalars throughout; complexification changes nothing below.

- **H = C⁰ ⊕ C¹**, functions on vertices (pitches) and on oriented edges (intervals). Finite: dim H = |V| + |E|.
- **δ : C⁰ → C¹**, (δs)(e) = s(t(e)) − s(s(e)). **D = δ + δ\***, self-adjoint and odd. D² = Δ₀ ⊕ Δ₁. MEASURED: `tests/instrument.rs::dirac_is_self_adjoint_and_odd`, `vertex_and_edge_laplacians_share_their_nonzero_spectrum` (prism 64442a0, branch `reed/settle-loop-1`, unmerged).
- **ker D = H⁰ ⊕ H¹** (Tonic ⊕ Comma), dim = b₀ + b₁. MEASURED against union-find: `kernel_counts_components_and_cycles`. This is the *output* of settling. It is not the Hilbert space.
- **Operations.** question x ↦ Dx; decay e^{−tD²}; ring e^{iθD}. Decay and ring are functions of D, so they commute with D and with each other and fix ker D pointwise. MEASURED: `decay_keeps_the_comma`, `decay_and_ring_commute_and_ring_never_settles`.
- **Settle** certifies b₀ (union-find decides the kernel by index; LAPACK must agree within error and clear the gap 1/(n(n−1))) or refuses with `Wall::Unseparated`. MEASURED: `settle.rs` S3 tests (d983c4a). `Instrument::kernel_dimension` still uses a threshold (`ZERO = 1e-9`), so b₁ is not certified there. §1.6 closes this for graphs.
- **Solo and Bleed** (8de8bd0). `Solo(S)` multiplies pitches by χ_S and intervals by the endpoint mean (χ_S(s)+χ_S(t))/2. Bleed = [D, Solo] has entries in {0, ±½}, supported exactly on edges crossing ∂S. MEASURED: `bleed_lives_exactly_on_the_solo_boundary`, `attend_then_ask_is_not_ask_then_attend`.
- **Progression** = element of the trace monoid M(E, I) in Foata normal form; `Monophony` (nothing commutes, words) and `Polyphony` (independent iff disjoint write footprints). MEASURED: `tests/trajectory.rs`.

### §0.2 What this document will not reintroduce

H is cochains, not cohomology; dim H is finite; nothing about rotation rate collapses H¹ (ring is an isometry that fixes ker D); the five FLOOR projectors generate a *commutative* ℂ⁵; the spectral gap belongs to the complement of the kernel, never to H⁰; no claim below rests on an analogy.

### §0.3 A note on Kagi

Kagi (`mcp__kagi-ken-mcp__kagi_search_fetch`) returned empty results for every query this session: two calls, five query variants, among them the exact title "Distances in finite spaces from noncommutative geometry". Per the brief I fell back to the arXiv API and web search. Each source in §Sources says which channel verified it.

### §0.4 Numerical companion

Every PROVED statement about small graphs below was also checked numerically. The two scripts (numpy, double precision) live in the session scratchpad, not in the repo. They are stated as tests in §6 so that prism-Reed can land them RED-first. A numerical check is not a proof, and every PROVED label below comes with its own argument.

## §1 What A is

### §1.1 The two genuine representations

Let A = C(V), the functions on vertices with pointwise product. Dimakis and Müller-Hoissen's universal first-order calculus on a discrete set already gives the answer. Their 1-forms have basis e_ij = e_i de_j (i ≠ j), and df = Σ_{i,j} e_ij (f(j) − f(i)) [DMH94, eq. (2.10)]. From their relations (2.1), (2.4) and (2.6), a function acts on 1-forms differently from the left and from the right: f·e_ij = f(i) e_ij and e_ij·f = f(j) e_ij. 1-forms are a *bimodule*. Restricting to the arrows of our oriented graph gives two actions on H:

- **π_L (source):** (π_L(f) s)(v) = f(v) s(v); (π_L(f) ω)(e) = f(s(e)) ω(e).
- **π_R (target):** the same on C⁰; (π_R(f) ω)(e) = f(t(e)) ω(e).

**Proposition 1.1 (PROVED).** π_L and π_R are unital \*-homomorphisms A → B(H). The endpoint mean π_M = ½(π_L + π_R), which is the action prism's `Solo` uses, is unital, positive and self-adjoint, but not multiplicative. In particular π_M(χ_S) is not a projection whenever some edge crosses ∂S.

*Proof.* Each π_L(f) is diagonal with entries that are point evaluations f(p) for a fixed point p per basis vector. Point evaluations are characters of C(V), so π_L(fg) = π_L(f)π_L(g), and the same holds for π_R. On a crossing edge, π_M(χ_S) has entry ½, and ½·½ ≠ ½. ∎ Numerically: multiplicativity error 0 for π_L and π_R, 0.18 for π_M on random f, g over the 4-cycle.

### §1.2 Bleed re-derived

For π = π_L, the C⁰ → C¹ block of [D, π(f)] is

  ([δ, π_L(f)] s)(e) = f(t)s(t) − f(s)s(s) − f(s)(s(t) − s(s)) = (f(t) − f(s)) · s(t),

with s, t the endpoints of e. The C¹ → C⁰ block is minus its adjoint (f real). For π_R the same computation gives (f(t) − f(s)) · s(s). Hence:

**Proposition 1.2 (PROVED).** For every S ⊆ V:
(i) [D, π_L(χ_S)] and [D, π_R(χ_S)] have entries in {0, ±1}, supported exactly on edges crossing ∂S. π_L puts the entry at the pair (e, t(e)) and π_R at (e, s(e)).
(ii) prism's Bleed = [D, π_M(χ_S)] = ½([D, π_L(χ_S)] + [D, π_R(χ_S)]).

So **±½ is not a representation. It is the average of two representations.** It is what you get when you refuse to say which side owns a crossing edge. (ii) is linearity of the commutator; numerically the two sides agree to machine precision on the 4-cycle with S = {0, 1}.

**Proposition 1.3 (PROVED): local representations are choices of owner.** Let π be a \*-representation of C(V) on H that is diagonal in the cell basis (local) and acts on C⁰ by multiplication. Then π(f)ω(e) = f(p(e)) ω(e) for some map p : E → V. If moreover [D, π(χ_S)] is supported only on edges crossing ∂S for every S, then p(e) ∈ {s(e), t(e)}.

*Proof.* A diagonal \*-representation of C(V) assigns a character to each basis vector, and the characters of C(V) are point evaluations, which gives p. Suppose p(e) = w is not an endpoint of e, and take S = {w}. Neither endpoint lies in S, so e does not cross ∂S. But the (e, t(e)) entry of [D, π(χ_S)] is D_{e,t(e)}·(χ_S(t(e)) − χ_S(w)) = −1 ≠ 0. ∎

So a genuine local A needs, on every edge, a declared owner p(e) ∈ ∂e. π_L and π_R are the two orientation-induced choices. Any per-edge choice of endpoint is also a genuine representation; a *convex* mixture (such as π_M) is not. **Declaring a boundary is choosing p on the crossing edges.** This is the first place where "the boundary each observer declares" enters the mathematics instead of the prose. Once p is declared, π_p(χ_S) is a projection: the cells S owns.

### §1.3 Where the non-commuting operations live

The operations fall into two commutative families:

- C\*(D) = {φ(D)}, the functional calculus of D. It contains question (φ = id), decay (φ = e^{−tλ²}) and ring (φ = e^{iθλ}), all simultaneously diagonal in D's eigenbasis.
- π_p(A), the solos, all simultaneously diagonal in the cell basis.

Each family is commutative. The two families do not commute with each other, and [D, π(f)] measures exactly by how much.

**Theorem 1.4 (PROVED). On a connected simple graph, π_p(A) ∪ {D} generates all of B(H).**

*Proof for p = source. For a general owner map the same argument applies: π(e_v)Dπ(e_w) yields the matrix unit joining e to its non-owner endpoint, and the rest is identical.* Write E_{xy} for the matrix units in the cell basis. Let e_v = χ_{{v}}. For an edge e = (v, w), π(e_v) D π(e_w) = E_{e,w}, because the only nonzero entry of D between the fibre of v (v together with the edges sourced at v) and the fibre of w (likewise for w) is D_{e,w} = 1. D is symmetric, so E_{w,e} is also in the algebra. Then E_{e,e} = E_{e,w}E_{w,e}, and E_{e,e}·π(e_v) D π(e_v) = −E_{e,v}. So each edge is joined by matrix units to both of its endpoints. On a connected graph, the graph of these matrix units is the barycentric subdivision of X, which is connected. Products of matrix units along paths in it give every E_{xy}. ∎ Numerically: dim alg⟨π_L(A), D⟩ = 64 = 8² on the 4-cycle, 25 = 5² on P₃, and the joint commutant is one-dimensional.

This answers prism-Reed's NT3 ("where do non-commuting ops live"). **They live in words over two commutative alphabets, π_p(A) and C\*(D).** Those words reach every operator on H. Nothing non-commutative needs to be adjoined.

**On the five FLOOR projectors (focus, split, project, lift, refract).** Five mutually orthogonal projectors generate ℂ⁵. A representation of ℂ⁵ on H is a five-way orthogonal grading of C⁰ ⊕ C¹, and FLOOR does not specify one. Until it does, the five are not operators on this H and cannot sit in A. **Verdict:** the five ops belong in the alphabet, as letters of words executed on the triple. That is prism-Reed's Fold5 reading. A is C(V) acting through a declared owner map p. If a future grading is specified, it is checked against Theorem 1.4: whatever it is, it already lies in the algebra the solos and the question generate. CONJECTURE (unexamined): no grading by those five names commutes with D on every graph, because only the trivial grading commutes with D for all X.

### §1.4 What F is in D_{n+1} = F(D_n, obs): inner fluctuations are rank-1 sheaves

In noncommutative geometry the canonical way for an observation to change D without changing (A, H) is an inner fluctuation, D ↦ D + ω + ω\* with ω = Σ π(a_i)[D, π(b_i)] a one-form [CCvS13].

**Proposition 1.5 (PROVED).** For π = π_L on a simple graph, the one-forms Ω¹_D are exactly the operators supported on the pairs (e, t(e)) (and their transposes), with arbitrary edge coefficients. The fluctuated operator is D_φ = δ_φ + δ_φ\* with

  (δ_φ s)(e) = (1 + φ_e) s(t(e)) − s(s(e)).

This is the coboundary of a rank-1 cellular sheaf on X with restriction maps F_{s(e) ⊴ e} = 1 and F_{t(e) ⊴ e} = 1 + φ_e, in the sense of Hansen–Ghrist [HG19] and Curry [Cur14].

*Proof.* By §1.2, π(a)[D, π(b)] has its C⁰ → C¹ entry at (e, t(e)) equal to a(s)(b(t) − b(s)). Taking a = e_{s(e)} and b = e_{t(e)} isolates edge e alone (the graph is simple), so sums realize any edge function φ at those entries. The C¹ → C⁰ part of ω contributes at (t(e), e), and its adjoint lands back on (e, t(e)). ∎ Numerically: the support pattern of Σ |π(a)[D, π(b)]| on the 4-cycle is exactly the target column of each edge row.

**Theorem 1.6 (PROVED): index conservation.** For every rank-1 sheaf on a fixed graph, b₀ − b₁ = |V| − |E|.

*Proof.* b₀ = dim ker δ_φ and b₁ = dim coker δ_φ = dim ker δ_φ\*. By rank–nullity, dim ker δ_φ − dim coker δ_φ = dim C⁰ − dim C¹. ∎ This is the finite shadow of the fact that the index of the Hodge–Dirac operator is the Euler characteristic [Arh25 recovers it as an Lᵖ-index on manifolds]. Numerically: on the 4-cycle, (b₀, b₁) = (1, 1) untwisted and (0, 0) for target restrictions 2, ½ and 0 on one edge.

**Consequence.** An observation that acts only by inner fluctuation cannot remove a comma without also removing a tonic, and cannot create one without creating the other. Holonomy ≠ 1 around a cycle kills that cycle's H¹ class *and* a global section together. So F decomposes:

  F = (inner fluctuation: re-weight the relations; (V, E) and b₀ − b₁ fixed) ∘ (frame change: add or remove cells; H itself changes).

Only the second kind can change b₀ and b₁ independently. §4 shows that only the second kind can widen.

### §1.5 Status of the open problem

The non-multiplicativity found today is resolved as follows. A genuine A exists (π_L, π_R, or any declared owner map p). Bleed's ±½ is the owner-undeclared average, and its support theorem survives for every genuine choice with entries ±1. prism-Reed's options are (a) keep `Solo` as π_M and document it as a positive map, not a representation, or (b) make `Solo` take an owner map and become a projection. §6 T1–T3 pin this down. I recommend (b), because §3 needs ownership.

### §1.6 b₁ certified without a threshold (graphs, constant sheaf)

**Proposition 1.7 (PROVED).** For the constant sheaf on a graph, b₁ = |E| − |V| + b₀ exactly, with b₀ from union-find.

*Proof.* Theorem 1.6 with φ = 0. ∎ So `kernel_dimension` can be certified the way `settle` certifies b₀: decide b₀ + b₁ by counting, then require LAPACK to agree. For non-constant sheaves no union-find exists. There b₁ needs an exact rank, in rational arithmetic or with a certified singular-value gap. That is still open.

## §2 Ω on cochains — the gauge made computable

*Intent: reachable futures over Progressions (Myhill–Nerode on trace monoids); a choice-count under which Conjecture 4.5.1 can be decided; Wojciech's reachable-state change.*

## §3 Two observers — the comma that exists only between

*Intent: Mayer–Vietoris on two glued sites; the connecting map as the comma; exact condition for a coupled comma when each side has none.*

## §4 The question as operator on reachability

*Intent: answers as changes of complex/sheaf; which widen both without overwriting either Tonic; comparison with Wojciech's three events.*

## §5 Where it stops

*Intent: settling vs halting; whether the stop belongs to the coupled system or the declared boundary.*

## §6 Smallest versions that can fail

*Intent: concrete tests on small graphs prism-Reed can write; what observation refutes the construction.*

## §7 This document as its own first test

*Intent: Mara and Seam as two observers; what a crowbar hit would mean in the terms of §3.*

## Sources (Kagi tally)

*Intent: every external claim with its primary URL; what could not be verified.*
