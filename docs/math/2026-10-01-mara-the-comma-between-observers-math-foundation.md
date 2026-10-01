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

### §2.1 Subjects, sites, letters

A **state** is σ = (X, F, x): a complex, a sheaf on it (constant unless stated otherwise), and a cochain x ∈ H = C⁰ ⊕ C¹. Write P for the orthogonal projection onto ker D.

A **subject** S declares a **site**: a closed subcomplex K_S ⊆ X (vertices together with edges whose endpoints it contains), and an owner p(e) for each edge that crosses out of it (§1.2). The site is the boundary the subject declares. Two sites may share cells. A shared cell is one both subjects may write.

Letters come in two kinds:
- **subject letters**, write(c) with c ∈ C(K_S) := C⁰(K_S) ⊕ C¹(K_S): S changes values on its own cells;
- **system letters**: question (x ↦ Dx), decay, ring, solo, and frame changes (§1.4, §4).

A history is a Progression u ∈ M(letters, I). Under `Polyphony`, two letters are independent iff neither writes a cell the other touches. Writes to a shared cell are therefore always dependent, so the trace keeps their order.

**Proposition 2.1 (PROVED).** If every independent pair commutes on every reachable state (the soundness contract in `time/independence.rs`), the action of words on states factors through M(E, I): trace-equivalent histories reach the same state.

*Proof.* Trace equivalence is the congruence generated by swapping adjacent independent letters [Maz77; CF69]. Each swap preserves the reached state by the contract. ∎ The contract is declared, not measured. `Polyphony` is sound only as far as the footprints are honest (prism's own doc comment). For write(c) and solo, the honest footprint of every letter in C\*(D) is *every cell*, because D couples a vertex to all its incident edges.

### §2.2 Reachable settled futures

The recursion stops moving where decay converges. P = lim_{t→∞} e^{−tD²} (PROVED: spectral theorem in finite dimension; every nonzero eigenvalue of D² is positive). What S can still bring about is therefore read at the settled level.

**Definition 2.2.** The reachable settled futures of S at σ are

  Ω_S(σ) := { P(x + c) : c ∈ C(K_S) } = Px + R_S,  where R_S := P(C(K_S)).

The **choice-count** is c_S(σ) := dim R_S. The **residue** is n_S := dim ker D − c_S: the settled directions S cannot steer.

This is a choice of definition, made in the open. Its claim to being the right Ω has three parts. (i) It is a function of (X, F, K_S) and the cochain, so Conjecture 4.5.1 now has content. (ii) It measures what S can *make settle*, which is Wojciech's "what subjects can generate afterward" read at the level where generation stops moving. (iii) It is computable with the tools prism already has (§2.3). It does not count the transient states S can visit on the way. A Seam objection on exactly that point would be a real one.

**Proposition 2.3 (PROVED).** Decay and ring never change Ω_S. The question collapses the offset: Ω_S(Dx) = R_S.

*Proof.* Pφ(D) = φ(0)P for every φ in the functional calculus, because P projects onto the 0-eigenspace of D. Decay and ring have φ(0) = 1 and the question has φ(0) = 0. ∎

### §2.3 The choice-count in closed form

**Theorem 2.4 (PROVED).** For the constant sheaf on a graph X and a site K_S with vertex set V_S and edge set E_S,

  c_S = [ b₀(X) − #{components of X containing no vertex of V_S} ] + [ b₁(X) − b₁(X ∖ E_S) ],

where X ∖ E_S is X with S's edges deleted and all vertices kept.

*Proof.* D² = Δ₀ ⊕ Δ₁ is block diagonal, so P = P₀ ⊕ P₁ with P₀ onto H⁰ (locally constant functions) and P₁ onto the harmonic 1-cochains. With no 2-cells, these are ker δ\* = Z, the cycle (flow) space. For a coordinate subspace W and a subspace Z, dim P_Z(W) = dim Z − dim(Z ∩ W^⊥), since (P_Z W)^⊥ ∩ Z = Z ∩ W^⊥. For W = C⁰(V_S): the locally constant functions vanishing on V_S are those supported on components that miss V_S. For W = C¹(E_S): the flows vanishing on E_S are exactly the flows of X ∖ E_S, a space of dimension b₁(X ∖ E_S). ∎

Every term is a union-find count (Proposition 1.7), so c_S is certifiable exactly, with no threshold. MEASURED (numerically, not in prism): the closed form equals the numerical rank of P restricted to C(K_S) on all 300 random graphs (2–7 vertices, random sites), 0 mismatches.

### §2.4 Myhill–Nerode at the subject's resolution

For a language L in a monoid M, the residual of a history u is u⁻¹L = {w : uw ∈ L}. Myhill–Nerode identifies histories with equal residuals, and L is recognizable iff there are finitely many residuals. For trace monoids, the recognizable languages are exactly those accepted by Zielonka's finite asynchronous automata [Zie87; DM11]. Here the futures are vector-valued, so the residual is replaced by what the history leaves reachable:

**Definition 2.5.** u ≡_S v iff Ω_S(u·σ₀) = Ω_S(v·σ₀). On a common frame, this holds iff P(x_u − x_v) ∈ R_S. The classes form an affine space of dimension n_S, so **the more S can steer, the less of its past it has to live with.**

**Proposition 2.6 (PROVED).** ≡_S is preserved by appending any letter in C\*(D) and any subject write. It is *not* preserved by solo.

*Proof.* For appended writes, differences are unchanged. For φ(D), Pφ(D)(a − b) = φ(0)P(a − b). Counterexample for solo: let X be an edge {0,1} disjoint from a triangle {2,3,4}, with K_S = {0,1} and its edge. R_S spans χ_{{0,1}}, and S cannot reach the triangle's comma h = (2,3) + (3,4) − (2,4). Let y be a vertex function on the triangle with mean 0 and y₃ ≠ y₄. Then Py = 0 ∈ R_S, so histories differing by y are ≡_S. Append the system word "question, then solo π_L(χ_{{2}})": ⟨π_L(χ_{{2}}) δy, h⟩ = (y₃ − y₂) − (y₄ − y₂) = y₃ − y₄ ≠ 0. That is a comma component outside R_S, so the two histories are no longer ≡_S. ∎

This is Wojciech's crowbar at the subject's resolution. Two histories with identical subject-visible reachability (the "auditor-visible state", with S as auditor) have divergent future reachability. The divergence happens exactly where attending and asking fail to commute, which is Bleed. It is *not* "same full state, different futures": the model is Markov in the full state, and the residue n_S is where the hidden difference lives. The future half of the crowbar (futures that depend on the record itself) still needs letters that read the Progression. CONJECTURE: frame-change letters whose admissibility reads the Foata form (for example "a cell becomes shared only if both subjects' writes to it sit in one Foata step") give ≡ classes that no state-only model reproduces. It is open because no such letter has been specified.

Counting choices on traces rather than words matters for any discrete variant. Two independent subject moves in either order are one choice, not two. LEAD: for a finite alphabet of discrete subject moves, the choice-count is the number of residual classes reachable in M(Σ_S, I). It is finite iff the language is recognizable, so Zielonka's theorem gives a finite distributed automaton to count with.

### §2.5 The Förster gauge, two readings

Alex 2026-10-01: *"The system may derive anything that does not alter the future statespace of subjects interacting with the system, which includes agentic workers."* Förster: *"Act always so as to increase the number of choices"* [vF, p. 6 and p. 12].

- **G₌ (Alex's literal wording: does not alter):** a system letter t is admissible iff Ω_S(tσ) = Ω_S(σ) for every subject S.
- **G⊇ (the brief's reading: does not narrow):** Ω_S(tσ) ⊇ Ω_S(σ) for every S.
- **Förster's own imperative (widen):** c_S strictly increases for some S and decreases for none.

**Proposition 2.7 (PROVED).** On a fixed frame (X, F, sites unchanged), G⊇ ⇔ G₌ ⇔ P(tσ − σ) ∈ R_S for all S, and c_S cannot change. *Only a frame change can widen.*

*Proof.* Both affine sets have direction R_S, and an affine subspace contained in a translate of itself with the same direction equals it. c_S = dim R_S depends only on the frame. ∎

### §2.6 Conjecture 4.5.1, decided

The 2026-09-09 document states: for every assertion s and its question Ds, the choice-space after is at least the choice-space before. With Ω_S defined, it splits:

**Theorem 2.8 (PROVED).**
(a) *Question as observation* (the record keeps s; Ds is exposed, not substituted): the state is unchanged and G₌ holds trivially.
(b) *Question as update* (x ↦ Dx): c_S is unchanged, so the count form holds and is vacuous. The inclusion form holds iff Ps ∈ R_S. For the constant sheaf it holds for every s iff K_S meets every component of X.
(c) *Counterexample to the unrestricted inclusion form:* X has two components, K_S lies in one, and s = χ_{second component}. Then Ps = s ∉ R_S and Ω_S(Ds) = R_S ≠ s + R_S. The question erased a tonic that S could not have set and cannot restore.
(d) *In no reading does the question widen.*

*Proof.* Proposition 2.3 and Proposition 2.7. ∎

So Conjecture 4.5.1 is replaced by: **the question is gauge-neutral for subjects whose site meets every component carrying the assertion's tonic, and gauge-violating (as an update) for the rest. Widening is never the question's work. It is the work of the answer, which changes the frame.** §4 is about answers.

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
