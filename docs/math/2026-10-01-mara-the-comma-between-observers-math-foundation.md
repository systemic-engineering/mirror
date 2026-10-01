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

Only the second kind can change b₀ and b₁ independently. Both kinds can widen: an inner fluctuation that restores holonomy 1 around a cycle adds a tonic and a comma together (§4.3). *(Corrected during drafting. An earlier version of this sentence said only frame changes widen. That is false.)*

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

### §3.1 Gluing two triples

Two subjects A and B declare sites K_A, K_B with K_A ∪ K_B = X. Write I = K_A ∩ K_B for the shared boundary, a closed subcomplex. Each site has its own triple (C(K_A), D_A) and (C(K_B), D_B), built from the sheaf restricted to the site. The coupled triple is (C(X), D_X). It is not C(K_A) ⊕ C(K_B), which counts shared cells twice. It is the equalizer: the pairs that agree on I.

**Lemma 3.1 (PROVED).** The sequence of cochain complexes

  0 → C•(X) → C•(K_A) ⊕ C•(K_B) → C•(I) → 0,  x ↦ (x|_A, x|_B),  (a, b) ↦ a|_I − b|_I

is exact and commutes with δ. *Proof.* Every cell of X lies in K_A or K_B, which gives injectivity. A pair agreeing on I glues to a cochain on X, which gives exactness in the middle. A cochain on I extends by zero to K_A, which gives surjectivity. Restriction commutes with δ because the sites are closed. ∎

The zig-zag lemma turns this into the Mayer–Vietoris sequence. On a graph, H² = 0, so

  0 → H⁰(X) → H⁰(A) ⊕ H⁰(B) → H⁰(I) —∂→ H¹(X) → H¹(A) ⊕ H¹(B) → H¹(I) → 0.

### §3.2 The comma between

**Definition 3.2.** The *comma between A and B* is im ∂ ⊆ H¹(X). By exactness this equals ker(H¹(X) → H¹(A) ⊕ H¹(B)): the disagreements of the coupled system that restrict to zero on each site.

**Theorem 3.3 (PROVED).**

  dim im ∂ = dim H⁰(I) − dim H⁰(A) − dim H⁰(B) + dim H⁰(X).

For the constant sheaf with K_A, K_B and X connected, this is b₀(I) − 1.

*Proof.* Alternating dimensions along the exact segment 0 → H⁰(X) → H⁰(A) ⊕ H⁰(B) → H⁰(I) → im ∂ → 0. ∎

**Corollary 3.4 (PROVED): when the coupled comma exists though neither side has one.** Suppose H¹(A) = H¹(B) = 0. Then im ∂ = H¹(X), and the coupled system has a comma iff H⁰(A) ⊕ H⁰(B) → H⁰(I) is not surjective. For the constant sheaf and connected sites, **a comma exists between two observers, though neither has one alone, iff their shared boundary is disconnected**. The number of such commas is the number of components of the boundary minus one.

**What "neither can tune it away alone" means precisely (PROVED).** Take s ∈ H⁰(I) outside the image. Extend it to any a ∈ C⁰(K_A) with a|_I = s, and set b = 0 on K_B. The 1-cochain z on X that equals δa on A's edges and 0 on B's edges is well defined, because δa = 0 = δb on edges of I (s is a section there). Its class is ∂s. On K_A, z = δa is a question A's own assertion a answers. On K_B, z = 0. **Each side sees only a question it can answer alone. The sum is not the question of any assertion on X.** Each can tune away its own part. Neither tuning, nor both tunings, removes the class, because the two tunings disagree on I.

### §3.3 The smallest instance, and its twisted control

**Two paths glued at two points.** K_A = path 0–1–2–3, K_B = path 3–4–5–0, I = {0, 3}. Then (b₀, b₁) is (1, 0) for each side, (2, 0) for I and (1, 1) for X, so dim im ∂ = 2 − 1 − 1 + 1 = 1. **One comma, and it exists only between them.** With three contact points (A = path 0…–4, B a star at a new vertex 5 touching 0, 2 and 4), dim im ∂ = 2. MEASURED numerically: both match Theorem 3.3.

**Twisted control (PROVED).** Give the cycle a rank-1 sheaf with nonzero restrictions (§1.4) and holonomy h around the loop. Each path still has H⁰ = 1 and H¹ = 0, since a rank-1 sheaf on a tree with invertible restrictions has a unique section up to scale. H⁰(I) = 2, and H⁰(X) = 1 iff h = 1. So dim im ∂ = [h = 1]. When the two observers' transports around the shared loop disagree (h ≠ 1), there is no comma between them, and also no global tonic. Every edge disagreement is then the question of some assertion, but nothing settles globally (Theorem 1.6 trades one for the other). **The comma between two observers exists only over a shared tonic.** It is the residue of an agreement, not its absence. Numerically: twisting edge (1, 2) by 2 gives (b₀, b₁)(X) = (0, 0) and dim im ∂ = 0.

### §3.4 How this meets §1

Under a declared owner map p, every crossing edge belongs to exactly one site, so I consists of vertices only: the far endpoints of crossing edges. Then dim im ∂ = |I| − b₀(A) − b₀(B) + b₀(X), and each extra contact point closes a loop through both sites. Bleed under π_p (±1) sits on the crossing edges, and the comma between is assembled from those same edges by the construction above. The undeclared π_M (±½) splits each crossing edge between the two sites, and no site decomposition corresponds to it. LEAD: that would make "undeclared boundary" and "no Mayer–Vietoris decomposition" the same failure; not checked beyond this observation.

## §4 The question as operator on reachability

### §4.1 Answers, authorship, sovereignty

An **answer** is a letter that changes the frame, (X, F, K_A, K_B) → (X', F', K_A', K_B'). It may add or remove cells, or fluctuate restriction maps. Like every letter, it has a footprint (the cells it writes) and authors (the subjects who sign it).

**Definition 4.1 (sovereignty for A).**
- (S1) *Authorship:* the answer writes no cell of K_A unless A is among its authors. A shared cell needs every owner as author.
- (S2) *Tonic:* K_A ⊆ K_A', and restriction H⁰(F'|_{K_A'}) → H⁰(F|_{K_A}) is surjective. Every assertion that could settle on A's old site can still settle in A's new one.

**Absorption** is the failure of S1 or S2.

**Definition 4.2 (widening for both).** c_A' ≥ c_A and c_B' ≥ c_B, with the strong form requiring both inequalities strict. Under Polyphony, two subjects' writes to a shared cell are dependent, so a letter that writes a shared cell is either ordered in the Progression or co-signed in one Foata step. *Co-signed letters on shared cells are the formal content of "a question they ask each other".*

### §4.2 The main theorem: sharing a relation across a disconnected boundary

**Theorem 4.3 (PROVED).** Take the constant sheaf, with K_A, K_B and X connected. Let u, v ∈ I lie in different components of the shared boundary I. Let the answer add one edge e = (u, v), co-signed by A and B, to X, K_A and K_B, so e is a shared cell. Then:
1. the comma between drops by one: dim im ∂' = dim im ∂ − 1;
2. each site gains a comma of its own: b₁(K_A') = b₁(K_A) + 1 and b₁(K_B') = b₁(K_B) + 1;
3. **both widen by exactly one:** c_A' = c_A + 1 and c_B' = c_B + 1;
4. sovereignty holds for both (S1 by co-signature; S2 below);
5. if b₀(I) = 2, then after the answer no comma is between: H¹(X') → H¹(K_A') ⊕ H¹(K_B') is injective, so every disagreement of the coupled system is visible inside at least one observer's own instrument.

*Proof.* (1) I' = I + e has one component fewer, H⁰ of the connected sites and of X is unchanged, and Theorem 3.3 applies. (2) e joins two vertices already connected inside each site. (3) Theorem 2.4: b₁(X') = b₁(X) + 1, and X' ∖ E(K_A') = X ∖ E(K_A) because e ∈ E(K_A'). The b₀ terms are unchanged. Likewise for B. (4) S2: F' agrees with F on the old cells, and H⁰ of a connected site is the constants, which restrict onto the constants. (5) Exactness at H¹(X') with im ∂' = 0. ∎

MEASURED numerically on two paths glued at two points: between 1 → 0; (b₀, b₁) of each site (1, 0) → (1, 1); c_A, c_B: 2 → 3; X: (1, 1) → (1, 2). A shared mediator vertex joined to both contact points gives the same numbers.

**Reading.** The comma is not answered. It is *shared*. Before, the disagreement existed only in the coupling, and each observer saw only a locally answerable question (§3.2). After, each observer holds a comma inside its own instrument. Each can now ask about it with its own D, from its own data, and the cell where they meet is one both must sign. That is the sense, and the only sense this document claims, in which *the comma between becomes a question they ask each other*.

### §4.3 The other answers, and why they fail

**Proposition 4.4 (PROVED): unilateral relation.** If only A authors e, so that e ∈ K_A' but e ∉ K_B', then c_A' = c_A + 1. For B, c_B' = c_B + 1 − [u and v are connected in X ∖ E(K_B)]; in the two-path example c_B is unchanged. The comma between persists, since I is unchanged. One observer widened; the other neither widened nor narrowed. *Proof.* Theorem 2.4. MEASURED numerically: c_A 2 → 3, c_B 2 → 2, between 1 → 1.

**Proposition 4.5 (PROVED): filling narrows both and cannot be owned.** Extend the complex by a 2-cell f whose boundary γ pairs nonzero with a class in im ∂. Then (i) f lies in neither site, and (ii) after filling, the comma is gone and c_A and c_B each drop by one in the two-path case.
*Proof of (i).* If γ ⊆ K_A, then for z = ∂s we have ⟨z, γ⟩ = ⟨δa, γ⟩ = ⟨a, ∂γ⟩ = 0, since z|_A = δa (§3.2) and γ is a cycle. That contradicts the nonzero pairing. A closed site containing f would contain γ, so it lies in neither, and the same argument applies to K_B. (ii) H becomes C⁰ ⊕ C¹ ⊕ C², and H¹ = ker δ₁ ∩ ker δ₀\* loses the class (numerically: b₁ = 0 after filling the 6-cycle). ∎
So answering the comma by fiat is a cell nobody owns. Either the system writes it, which violates G₌, or one observer must first absorb the other's edges, which violates S2 for the other.

**Proposition 4.6 (PROVED): twisting narrows both, and overwrites a tonic.** Fluctuate the restriction on one edge of A so that the holonomy h ≠ 1. Then H⁰(X) and H¹(X) both vanish (Theorem 1.6 and §3.3), so in the two-path case c_A and c_B both go 2 → 0. A's local section changes, from (1, 1, 1, 1) to (1, 1, ½, ½) for a twist of 2 on edge (1, 2). If B authors the twist, S1 fails. If A authors it, A has narrowed B without writing a single cell of B. **The gauge can be broken non-locally**: c_B depends on H⁰(X) and H¹(X), which no site owns. MEASURED numerically.

**Proposition 4.7 (PROVED): untwisting widens both.** Conversely, from h ≠ 1, a fluctuation restoring h = 1 adds a tonic and a comma together. In the two-path case c_A and c_B both go 0 → 2. It writes a cell of whichever site holds the edge, so it is sovereign only if that site authors it, or it co-signs on a shared edge. MEASURED numerically.

**Absorption.** If A takes over B's cells (K_A' = X, K_B' = I), then c_B falls (2 → 1 in the example) and c_A does not rise (2 → 2). Absorbing did not even widen the absorber.

| Answer to the comma between | between | c_A | c_B | sovereign? |
|---|---|---|---|---|
| share a co-signed edge across a disconnected boundary | 1 → 0 | +1 | +1 | yes |
| unilateral edge (A only) | 1 → 1 | +1 | 0 | yes (S1 holds; nothing of B written) |
| fill with a 2-cell | 1 → 0 | −1 | −1 | no owner possible |
| twist an A-edge | 1 → 0 | −2 | −2 | A's tonic overwritten; B narrowed non-locally |
| untwist (from h ≠ 1) | 0 → 1 | +2 | +2 | only if authored by the edge's owner(s) |
| absorb B into A | 1 → 0 | 0 | −1 | no |

### §4.4 Wojciech's three events

Wojciech's decomposition (field log 2026-09-27): *evidence change* (something new becomes known), *frame change* (the same world becomes differently interpretable), *reachable-state change* (alternatives that did not exist in the system's actionable possibility-space become reachable). In this model the three are distinct, computable and logically independent:

- **Evidence change**: x changes, frame fixed. c_S cannot change (Proposition 2.7). Ω_S moves iff P(Δx) ∉ R_S, that is, iff the new evidence shifted the settled state in a direction S cannot steer.
- **Frame change**: (X, F, sites) changes. It may change c_S (Theorem 4.3), or leave it fixed (B under a unilateral edge, Proposition 4.4).
- **Reachable-state change**: Ω_S changes. The strong form is a change in c_S, and only a frame change can produce it.

*"Slap without altered reachability = phenomenology"* becomes: a frame change after which c_S and Ω_S are unchanged for the subject in question. B under A's unilateral edge is the minimal instance. A frame really changed (b₁(X) rose), and nothing B can make settle changed.

CONJECTURE (the empirical mapping, labelled because it is a modelling hypothesis, not mathematics): Wojciech's operational question, *"what outcome should occur under the circular-reflexive operator that should not occur under a carefully matched linear or conventional reflective question?"*, has a candidate answer in this model. A unilateral reframe widens only the reframer (Δc_receiver = 0). A co-signed shared relation across a disconnected boundary widens both (Δc = +1 each). The measurable discriminator is the receiver's Δc. His generative criteria ("identify previously unreachable alternative without another prompt", "preserve capability after intervention gone") correspond to new directions in R_S that persist because they live in the frame, not in x. It is open because nothing yet maps a human dyad's state onto (X, F, K_S).

## §5 Where it stops

### §5.1 Two different questions

Turing 1936 did not ask where a process halts. His well-behaved machines are the *circle-free* ones, which never halt and print forever. What he proved undecidable is whether a machine is circle-free, or whether it ever prints a given symbol. The phrase "halting problem" first appears in Davis 1958 [Luc21; HN24]. Förster's question is a third one: not whether output continues, and not whether a final state is reached, but whether the recursion *stops moving*, that is, whether it has an eigenbehavior [vF76; not re-verified this session]. The triple has two recursions, and they answer Förster's question differently.

### §5.2 The inner recursion (fixed frame): settling always stops, at a rate the frame decides

**Theorem 5.1 (PROVED).** Let μ be the smallest nonzero eigenvalue of D². Then ‖e^{−tD²}x − Px‖ ≤ e^{−μt}‖x − Px‖. At observer resolution ε, the flow is settled for all t ≥ T(x, ε) = ln(‖x − Px‖/ε)/μ.

*Proof.* Spectral decomposition. The components of x off ker D decay at rates e^{−λt} with λ ≥ μ. ∎

μ belongs to the complement of the kernel. Since Δ₀ and Δ₁ share their nonzero spectrum (MEASURED, `vertex_and_edge_laplacians_share_their_nonzero_spectrum`), μ is the smallest nonzero Laplacian eigenvalue, which for a connected graph is its algebraic connectivity, bounded below by 1/(n(n−1)). That bound is what `settle` certifies (d983c4a). The limit is never reached at finite t unless x is already settled, but its existence and its rate are decidable for every finite frame. That is the difference from Turing's question: settling is a property of a limit that exists unconditionally, not of a trajectory entering a set.

Ring does not stop. e^{iθD} is an isometry, and every component of x off the kernel rotates at frequency |λ| > 0 forever. Its slowest frequency is √μ, which is what O3 pinned as √λ₂(P₇) = √(2 − 2cos(π/7)) (per prism 30d196f; I did not re-run it). **Ring is Turing's good machine. Decay is Förster's.** Both are functions of the same D, so the inner stop belongs to the frame.

### §5.3 The outer recursion (frame changes): reachability stops if and only if a boundary bounds it

Now D_{n+1} = F(D_n, obs_n), with F an inner fluctuation followed by a frame change (§1.4), and each step admissible under the gauge.

**Theorem 5.2 (PROVED): a declared boundary stops reachability.** Suppose (i) every step satisfies G⊇ for every subject, so no c_S ever decreases, and (ii) the subjects have declared a cell budget N (|V| + |E| ≤ N throughout; Ricky Jones: *"we stop it by binding a boundary to a purpose: this observation, this θ, this decision"*). Then the vector (c_S)_S is non-decreasing and bounded by N, so it changes at most kN times for k subjects, and is eventually constant. **Reachability stops moving after finitely many widenings.** The frame itself may keep moving, but every later change is, for every subject, phenomenology in Wojciech's sense.

*Proof.* c_S ≤ dim ker D ≤ dim H ≤ N, and a non-decreasing integer sequence bounded above is eventually constant. ∎

Two consequences:
- Under G⊇ alone with a budget, the eigenbehavior is a stop of reachability, not of the frame. The recursion may circle forever among frames with equal c-vectors. That circling is a limit cycle in Ricky's θ with nothing new reachable.
- Without a budget, nothing in the triple forces a stop, and nothing should. Förster's imperative is satisfied by an unbounded sequence of shared relations, each one raising both c's by one (Theorem 4.3 iterated across a boundary with enough components, or with new components created by new contact).

CONJECTURE 5.3: *without a declared boundary, whether a given computable observation rule ever stops widening is undecidable.* Termination of double-pushout graph rewriting is undecidable, on all graphs and on a fixed start graph [Plu98]. The conjecture is open because I have not shown that rewriting rules constrained by G⊇ and sovereignty (S1, S2) can still simulate an arbitrary DPO system; the constraints might cut expressiveness below undecidability.

### §5.4 The answer to "where does it stop"

- The inner stop (settling) is a property of the frame: its kernel and its gap μ. It always exists, and the time to reach resolution ε is decidable.
- The outer stop (reachability) is a property of the declared boundary. With a budget it is a theorem (5.2). Without one it is not guaranteed, and plausibly undecidable (5.3).
- The coupled system alone does not decide where the outer recursion stops. The boundary each observer declares does: which cells it owns, which it shares, and how many it will spend. This is the same object that §1 needed for A to be a representation and §3 needed for the comma between to exist. **One declaration does all three jobs.** That is the strongest structural claim in this document, and the one I most want Seam to test.

## §6 Smallest versions that can fail

Each test below is stated so that the expected value comes from an instrument independent of the code under test: union-find, a hand-built matrix, or a closed form. That is prism's own discipline (`tests/instrument.rs` header). All were checked numerically by me. None is in prism yet.

### §6.1 The instrument (§1)

- **T1 owner-map Solo is a projection.** With an owner map p (start with p = source), π_p(χ_S)² = π_p(χ_S) and π_p(fg) = π_p(f)π_p(g) for random f, g. *Control:* today's endpoint-mean `Solo` fails both (a crossing edge gives ¼ against ½).
- **T2 Bleed decomposes.** [D, π_M(χ_S)] = ½([D, π_L(χ_S)] + [D, π_R(χ_S)]) exactly. [D, π_L(χ_S)] has entries in {0, ±1}, only at pairs (e, t(e)) and their transposes, only for e crossing ∂S.
- **T3 solos and the question generate everything.** On a connected graph, the span of words in {π_L(χ_v)} ∪ {D} has dimension (|V| + |E|)². *Control:* on a disconnected graph it is the sum of the squared component dimensions.
- **T4 fluctuations are rank-1 sheaves and conserve the index.** Σ_{a,b} |π_L(χ_a)[D, π_L(χ_b)]| is supported exactly on the target entries. For random target restrictions (including 0), b₀ − b₁ = |V| − |E|.
- **T5 b₁ certified.** `kernel_dimension` equals 2·b₀ + |E| − |V|, with b₀ from union-find, decided by count and only confirmed by LAPACK, as in S3.

### §6.2 The gauge (§2)

- **T6 closed-form choice-count.** For random graphs and sites, rank(P|_{C(K_S)}) equals Theorem 2.4's union-find expression.
- **T7 Conjecture 4.5.1 counterexample.** Two disjoint edges, site on the first, s = χ of the second. Then Ω_S(Ds) ≠ Ω_S(s), while c_S is unchanged. *Control:* a site meeting both components gives Ω_S(Ds) = Ω_S(s).
- **T8 Nerode breaks exactly at solo.** In the edge ⊔ triangle example of Proposition 2.6: decay, ring and question preserve ≡_S, and "question, then π_L(χ_{{2}})" breaks it with comma component y₃ − y₄.

### §6.3 Two observers (§3–§4)

- **T9 the comma between.** Two paths glued at two points: H¹(A) = H¹(B) = 0, H¹(X) = 1, dim im ∂ = 1. Three contacts: dim im ∂ = 2. *Control:* glue along a connected path (b₀(I) = 1) and get dim im ∂ = 0.
- **T10 locally answerable, globally not.** Build z from §3.2. Then z|_A = δa and z|_B = 0, but the projection of z onto H¹(X) is nonzero.
- **T11 twisted control.** On the two-path cycle with holonomy h, dim im ∂ = [h = 1], and b₀(X) = b₁(X) = [h = 1].
- **T12 the answer table.** Each row of §4.3 as one test: shared co-signed edge (+1, +1), unilateral (+1, 0), twist (−2, −2), untwist (+2, +2), absorb (0, −1). Filling needs C², so it waits until the instrument has 2-cells.

### §6.4 What would refute the construction, as opposed to a bug

- **R1 (mathematics).** A connected pair of sites with H¹(A) = H¹(B) = 0 and b₁(X) ≠ b₀(I) − 1. That contradicts Lemma 3.1, so it would mean a site was not closed or the sheaf was not restricted correctly. It is a check on the code, not on the idea.
- **R2 (the gauge definition).** A co-signed boundary-joining edge after which some subject can make strictly *fewer* things settle by any measure prism-Reed or Seam deems more faithful than c_S. Theorem 4.3 is a theorem about c_S. If c_S is the wrong count, the theorem is true and irrelevant.
- **R3 (the empirical mapping).** In a matched dyad protocol along Wojciech's lines, a unilateral reframe that reproducibly raises the *receiver's* generative reachability. The model predicts Δc_receiver = 0. That would refute the mapping in §4.4, not the mathematics.
- **R4 (the crowbar's future half).** If no letter that reads the Progression can produce ≡ classes that differ from state-only ones, this construction never reaches "same visible state, futures diverge because of the record". Proposition 2.6 reaches it only at the subject's resolution.

## §7 This document as its own first test

*This section is a reading, not a proof. Nothing here constructs a complex out of a review.*

The question is circular-recursive by design, and so is its first test. This document is one site. Its cells are the definitions and claims above, and its sections each settle. Seam's review is the second site. The shared boundary is the set of definitions we must both use: site, owner map, Ω_S, c_S, sovereignty, "question". If Seam and I each find our own sections consistent but they fail to glue on that boundary, that is a comma between us in exactly the sense of §3. Each side's checks pass, and the disagreement exists only in the coupling.

§4 then predicts how the review goes well and how it goes badly.
- **Fill:** one of us declares the other wrong by fiat, and the disagreement disappears without either of us able to reach more.
- **Absorb:** Seam rewrites my definitions, or I wave his objection away into mine.
- **Share:** a co-signed change to a shared definition, after which each of us holds the disagreement inside our own frame and can reach more.

If the review ends in a shared definition neither of us had before, that is Theorem 4.3 enacted once, which is not evidence for it. If it ends in fill or absorb, the document has predicted its own failure. Either way, the comma between Seam's review and this document is the next question, and it belongs to both of us.

Where I expect the crowbar: §6.4 R2. Theorem 4.3 is nearly immediate once Definition 2.2 is accepted, so the content is in the definition. Dimension of the reachable settled set is a count of independent directions, not of choices. It ignores transients. It treats writes as unbounded real additions. Whether that is Förster's "number of choices", or whether I made the gauge true by choosing what to count, is the question I cannot settle from inside this site.

## Sources

Kagi returned empty for every query this session (§0.3). "arXiv" = verified via the arXiv API (title, authors, abstract). "web" = verified via web search result pages. "read" = the cited passage was read in the primary text.

- [DMH94] A. Dimakis, F. Müller-Hoissen, *Discrete differential calculus, graphs, topologies and gauge theory*, J. Math. Phys. 35 (1994). https://arxiv.org/abs/hep-th/9404112 — **read**: §II eqs. (2.1)–(2.10), the basis e_ij = e_i de_j and df = Σ e_ij(f(j) − f(i)). The left and right actions in §1.1 are derived from (2.1), (2.4) and (2.6), not quoted.
- [DMH98] A. Dimakis, F. Müller-Hoissen, *Discrete Riemannian geometry*. https://arxiv.org/abs/gr-qc/9808023 — arXiv (the correspondence between first-order calculi and digraphs, in the abstract).
- [IKM99] B. Iochum, T. Krajewski, P. Martinetti, *Distances in finite spaces from noncommutative geometry*. https://arxiv.org/abs/hep-th/9912217 — arXiv. Background only: no result of theirs is used.
- [Req00] M. Requardt, *Graph-Laplacians and Dirac operators on (infinite) graphs and the calculation of the Connes-distance-functional*. https://arxiv.org/abs/math-ph/0001026 ; [Req01] *Dirac operators and the calculation of the Connes metric on arbitrary (infinite) graphs*. https://arxiv.org/abs/math-ph/0108007 — arXiv. LEAD: the abstract says the computed distance depends on the choice of graph Dirac operator. Connes distance for π_L is not computed here.
- [CCvS13] A. H. Chamseddine, A. Connes, W. D. van Suijlekom, *Inner fluctuations in noncommutative geometry without the first order condition*. https://arxiv.org/abs/1304.7583 — arXiv (inner fluctuations as linear terms in the usual case; I use only the linear form D + ω + ω\*).
- [HG19] J. Hansen, R. Ghrist, *Toward a spectral theory of cellular sheaves*. https://arxiv.org/abs/1808.01513 — arXiv.
- [Cur14] J. Curry, *Sheaves, cosheaves and applications*, PhD thesis. https://arxiv.org/abs/1303.3255 — arXiv.
- [Lim20] L.-H. Lim, *Hodge Laplacians on graphs*. https://arxiv.org/abs/1507.05379 — arXiv (cohomology and Hodge theory on graphs as linear algebra of AB = 0).
- [Arh25] C. Arhancet, *The Lᵖ-index of the Hodge–Dirac operator on compact Riemannian manifolds*. https://arxiv.org/abs/2512.22517 — arXiv (index = Euler characteristic; cited only as the continuum shadow of Theorem 1.6).
- [Ken11] R. Kenyon, *Spanning forests and the vector bundle Laplacian*. https://arxiv.org/abs/1001.4028 — arXiv; [BSS13] A. Bandeira, A. Singer, D. Spielman, *A Cheeger inequality for the graph connection Laplacian*. https://arxiv.org/abs/1204.3873 — arXiv. LEAD: holonomy and frustration on line bundles. They are relevant to §3.3 but not used in any proof.
- [Maz77] A. Mazurkiewicz, *Concurrent program schemes and their interpretations*, DAIMI PB-78, Aarhus 1977 — web (bibliographic record only, via citing Springer pages).
- [CF69] P. Cartier, D. Foata, *Problèmes combinatoires de commutation et réarrangements*, LNM 85, Springer 1969. https://www.mat.univie.ac.at/~slc/books/cartfoa.html — web.
- [Zie87] W. Zielonka, *Notes on finite asynchronous automata*, RAIRO ITA 21 (1987) — web (as cited in survey listings). [DM11] V. Diekert, A. Muscholl, *Trace theory* (survey, 2011). http://www2.informatik.uni-stuttgart.de/fmi/ti/veroeffentlichungen/pdffiles/DiekertMuscholl2011.pdf — found, but could not be fetched (certificate error). NOT VERIFIED beyond the search listing. The "recognizable iff Zielonka-accepted" claim rests on search-result text.
- [MU19] H. Maarand, T. Uustalu, *Reordering derivatives of trace closures of regular languages*. https://arxiv.org/abs/1908.03551 — arXiv (residuals/derivatives on trace closures; LEAD for a computable residual automaton).
- [Plu98] D. Plump, *Termination of graph rewriting is undecidable*, Fundamenta Informaticae 33(2) (1998). https://content.iospress.com/articles/fundamenta-informaticae/fi33-2-04 — web (abstract: undecidable on all graphs and on a fixed start graph; proof by reduction from PCP).
- [vF] H. von Foerster, *Ethics and second-order cybernetics*. https://uranos.ch/research/references/VonFoerster1992/ethics.pdf — **read**: p. 6, *"Tell them they should always try to act so as to increase the number of choices"*; p. 12 (Rey interview), *"The ethical imperative: Act always so as to increase the number of choices."*
- [vF76] H. von Foerster, *Objects: tokens for (eigen-)behaviors* (1976). https://monoskop.org/images/b/bf/Von_Foerster_Heinz_2003_Objects_Tokens_for_Eigen-Behaviors.pdf — carried from the 2026-09-09 document. NOT re-verified this session.
- [Luc21] S. Lucas, *The origins of the halting problem*, J. Log. Algebr. Methods Program. (2021). https://www.sciencedirect.com/science/article/pii/S235222082100050X — web (term first in Davis 1958; Turing's problems concern circle-freeness and printing). [HN24] J. D. Hamkins, T. Nenu, *Did Turing prove the undecidability of the halting problem?* https://arxiv.org/abs/2407.00680 — arXiv.
- Field logs (protected, not public): `practice/field-logs/2026-09-27 · Sunday — …` (Wojciech Kaleta: three events; the brutal test; generative criteria), `2026-09-28 · Monday-Night — …` (state ≠ trajectory crowbar; auditor-visible sharpening), `2026-10-01 · Nonlinear-Time-Post-Resonance-Cascade.md` (Ricky Jones: θ_{t+1} = U(θ_t, O_{θ_t}(X)); boundary bound to a purpose) — **read**.

**Tally.** Verified: 19 (2 read in the primary text, 11 via arXiv, 6 via web). Not verified: Diekert–Muscholl survey text (certificate error), Mazurkiewicz 1977 and Zielonka 1987 primary texts (bibliographic records only), von Foerster 1976 (carried, not re-fetched). Kagi: 0 results across 5 query variants.
