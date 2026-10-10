---
title: "The difference that makes a difference — math foundation"
author: Mara
date: 2026-10-10
status: candidate (awaiting Seam crowbar)
visibility: protected
builds-on:
  - "mirror docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md"
  - "mirror docs/audits/2026-10-01-seam-crowbar-comma-between-observers.md"
  - "systemic.engineering practice/insights/cybernetics/the-algebra-of-non-linear-relating.md (2026-10-04)"
  - "systemic.engineering practice/insights/neuroscience/zeroth-order-register.md"
---

> **Confidentiality.** §2 and §9 analyse Scott Gardner's internal draft *Technique as a Field* (marked INTERNAL // CONFIDENTIAL // SPECULATIVE DRAFT, shared with Alex). I quote only the equations I analyse. Alex decides before any of this leaves the protected corpus.

# The difference that makes a difference

*Mara, 2026-10-10. From Alex and Reed's question. Written to be broken by Seam.*

## The question

> What has to be true of a computation — from the zeroth-order signal, through the *thing* two observers both observe, to the interpretation each lays over it — for it to measure the difference that makes a difference as the part of their coupled field that no single potential can absorb: neither Technique's gradient (Scott/Ellul) nor noise to be filtered (Wiener), but the comma that only exists because they share a thing? And when the measurement itself becomes another observer of that thing, where does the recursion stop moving?

Labels: **PROVED** · **MEASURED** (commit or test) · **COMPUTED** (script in scratch) · **CONJECTURE** (why open) · **LEAD** (unverified) · **READING** (interpretation).

## Sections

§0 answer · §1 registers and Bateson · §2 gradient = no comma (Scott) · §3 the thing they share, c_I · §4 direction vs erasure · §5 noise · §6 plural truths · §7 the measurement as observer · §8 many shards at once · §9 what Wiener says · §10 tests · §11 open, and the crowbar · Sources

## §0 The answer

**What has to be true, from signal to interpretation.**
- **The signal must reach settling whole.** A filter that designates part of it as noise before settling preserves the settled result iff it keeps every kernel direction (Prop 1.4).
- **Observation (𝔹) sees how many commas there are and where one could be carried (bridgeless supports), never the comma itself** (Props 1.1–1.2). The comma needs an interpretation with cancellation and an orientation. Orientation is a free gauge (Prop 4.1).
- **The difference which makes a difference is the harmonic part of the relational difference field.** It is what neither re-description absorbs nor settling dissipates (Prop 1.3).

**Why no single potential can absorb it.**
- A gradient is exact. Any aggregate of potentials is one potential. Its best fit to any flow is the exact projection, and the residual is curl ⊕ harmonic (Prop 2.1).
- Scott Gardner's draft model of Technique is exact in every equation. It cannot write a comma down, because it has no relational variable that is not a difference of states (§2.2).
- Two monisms glued over a shared thing produce a comma exactly when their potentials differ on the shared thing beyond what either can re-describe away (Thm 3.2).

**Why it is not noise.** Decay classifies directions, not signals. Noise moves *along* a comma, which is the most exposed direction in the system. It can neither make nor unmake one, and it breaks ties only between discrete attractors (§5).

**Why it exists only because they share a thing.**
- No shared thing gives no comma. A shared thing in one piece gives no comma (Prop 3.1).
- Edda's crossroads, C₄ with I = {left, right}, is the smallest compatible instance.

**The third count.** The settled directions both observers can steer split exactly into two kinds (Thm 3.4):
- **c_I**, steered through the shared thing: one choice, located between them;
- **k**, the comma itself: settled directions both can steer from their private cells, with letters the trace calls independent.

This resolves the location of Seam's open comma. It does not settle its price. c_I catches the privatisation of a shared cell, which no per-subject count sees (Prop 3.5).

**Where the recursion stops.**
- An instrument that observes passively can only make an invisible disagreement visible, never create one. Every comma among any number of passive observers lives in H¹(X), so the thing is the ceiling (Thm 7.1).
- An instrument that probes changes the frame, and stops only by a declared budget.
- With many shards, the comma among all is Ĥ¹ of the nerve with H⁰ coefficients. It is the one thing shard-by-shard compilation cannot produce, and an edit's influence is confined to the shard's star in the nerve (§8).

Setting throughout, unless a section says otherwise: the finite triple of the comma document. X = (V, E) a finite graph with canonical orientation, H = C⁰ ⊕ C¹, D = δ + δ*, P the projection onto ker D = H⁰ ⊕ H¹ (the *output* of settling, not the Hilbert space). Constant sheaf. Two subjects A, B declare closed sites K_A, K_B with K_A ∪ K_B = X; I = K_A ∩ K_B. R_S = P(C(K_S)), c_S = dim R_S (comma doc Def 2.2, "decided relative to Definition 2.2" per Seam §C.1). Computations: `difference.py`, `ci_only.py`, `contest.py` in the session scratchpad `mara-difference/` (numpy 2.4.4, fixed seeds; output in `difference.out`). A numerical check is not a proof; every PROVED label carries its own argument.

## §1 Three registers: signal, observation, interpretation

### §1.1 The registers, stated

The zeroth-order register document puts four orders on one axis: 0 (signal before categorisation), 1 (content), 2 (observation of content), 3 (second order informing first without leaving it). It claims that third-order operation needs access to 0. It also names the mechanism that removes that access: a hub that categorises before arrival, deciding "this signal is content, … this signal is pattern, … this signal is noise (discard)" (§5.1, §6.3: "order-assignment IS measurement").

In the triple (READING for the correspondence, PROVED for the mathematics attached):

- **Register 0, the signal.** A cochain x ∈ C⁰ ⊕ C¹ on the complex X, before any decomposition is chosen. Every part of it is present: the exact part, the harmonic part, and whatever later gets called noise.
- **Register 1, the observation.** The 𝔹-reading of the floor document (§1.4): *whether*. Support, reachability, b₀ and b₁ by union-find.
- **Register 2, the interpretation.** A reading in a semiring with cancellation: an orientation chosen, signs allowed. Only here can x be decomposed into the part a potential absorbs and the part it cannot.
- **The thing.** The complex X itself, with the cover by observers' sites. §3 is about it.

### §1.2 Positive interpretations refine an observation; the comma refines none

**Proposition 1.1 (PROVED).** Let S be a semiring that is zero-sum-free (a ⊕ b = 0 ⇒ a = b = 0) and entire (a ⊗ b = 0 ⇒ a = 0 or b = 0). Then supp : S → 𝔹, a ↦ [a ≠ 0], is a semiring homomorphism. Applied entrywise, it commutes with composition, converse, Kronecker tensor and partial trace in Mat_S. Signed ℝ is neither, and supp is not a homomorphism there.

*Proof.* supp(a ⊕ b) = 1 iff a ⊕ b ≠ 0, which by zero-sum-freeness holds iff a ≠ 0 or b ≠ 0. supp(a ⊗ b) = 1 iff a ⊗ b ≠ 0, which by entireness holds iff both are non-zero. The matrix operations are built from ⊕ and ⊗ entrywise. In ℝ, 1 + (−1) = 0 but 1 ∨ 1 = 1. ∎ COMPUTED (`difference.py` CHECK10): ℕ and min-plus (zero = +∞) give 0 violations; ℤ gives 4 on the box [−2, 2]². LEAD: Golan's terminology for these conditions is being checked (Sources).

So every positive interpretation (counts, probabilities, least action) is a refinement of the observation: compute in S, take supports, and you get the 𝔹 computation. The floor document's 3-cycle (§1.4.3) is the counterexample in ℝ. δ* of the oriented cycle vector is 0 over ℝ and (1, 1, 1) over 𝔹.

**Proposition 1.2 (PROVED): what the observation sees of a comma.** If h is a non-zero flow (δ*h = 0) on a graph, then no edge of supp(h) is a bridge of supp(h). Conversely, every bridgeless edge set is the support of some flow.

*Proof.* If e were a bridge of supp(h), deleting it would split supp(h) into two sides. Summing δ*h = 0 over the vertices of one side cancels every edge internal to that side and leaves ±h(e) = 0, a contradiction. For the converse, every edge of a bridgeless set lies on a cycle in it, and a generic combination of those cycle vectors vanishes on no edge. ∎ COMPUTED: 0 bridges in the supports of generic flows over 300 random graphs (CHECK10b).

Register 1 therefore holds exactly two facts about the comma: *that* there are b₁ of them, and *where* one could be carried (bridgeless supports). The comma vector itself, its signs and its class, exists only in register 2. **The comma is the part of an interpretation that refines no observation.**

### §1.3 Where Bateson's difference sits

Bateson, *Form, Substance and Difference* (1970), verified wording: "what we mean by information—the elementary unit of information—is **a difference which makes a difference**, and it is able to make a difference because the neural pathways along which it travels and is continually transformed are themselves provided with energy."

Two neighbouring sentences matter as much:
- "Differences are the things that get onto a map."
- "the transform of a difference traveling in a circuit is the elementary idea."

(Text from a transcription of *Steps to an Ecology of Mind*. The page number is not verified.)

**Proposition 1.3 (PROVED).** Let ω ∈ C¹(X) be a field of differences across relations, on a graph with no 2-cells. Then ω = δφ + h uniquely, with h harmonic. Moreover:
(i) ω is absorbed by re-describing the vertices (ω = δφ for some φ) iff h = 0;
(ii) under settling, e^{−tΔ₁}ω → h;
(iii) h is unchanged by any re-description φ ↦ φ + ψ.
With 2-cells, ω = δ₀φ + δ₁*β + h. The middle (curl) part is not absorbed by any potential, but settling removes it.

*Proof.* Hodge decomposition C¹ = im δ₀ ⊕ ker Δ₁ ⊕ im δ₁* (Jiang–Lim–Yao–Ye 2011, Thm 2, journal numbering; Lim 2020 eq. (3.5)). Decay is a function of Δ₁ with limit the projection onto ker Δ₁. Adding δψ changes only the first summand. ∎

READING. **The difference which makes a difference is the harmonic part of the relational difference field.**
- The first "difference" is δ. The coboundary is literally the difference operator across a relation, (δs)(e) = s(t) − s(s), and its values are what "get onto a map".
- A difference "makes a difference" when it is neither absorbed by re-description (i) nor dissipated by settling (ii).
- Curl is the intermediate case. It is local inconsistency, which no potential absorbs but which settling dissipates. In Jiang–Lim–Yao–Ye's words, the curl part accounts "for local inconsistencies" and the harmonic part "for global inconsistencies" (Thm 3).
- Bateson's "circuit" is where it lives: h ≠ 0 needs a cycle, b₁ > 0.

For two observers, §3 shows that the cycle is made out of the thing they share.

### §1.4 Zeroth access, stated as a condition

**Proposition 1.4 (PROVED, elementary).** Let a hub apply an orthogonal projection F onto a designated subspace W before settling (keep the content, discard the rest). Then PFx = Px for all x iff ker D ⊆ W.

*Proof.* PF = P iff (taking adjoints) FP = P iff range(P) ⊆ W. ∎

READING. A categorising hub preserves everything that settles iff it discards nothing in the kernel. If it discards a comma direction as "noise" before settling, no later order can recover it. That is the zeroth-order document's "third order requires zeroth access", in the one form this triple can state: **the settled result is computed from the whole signal, and the comma is the first casualty of pre-categorisation.** Wiener's filtering begins with exactly such a designation: "let us treat u as a message and v as a noise" (*Cybernetics* Ch. III, printed p. 90). §5 returns to it.

## §2 Gradient = no comma

### §2.1 What one potential can hold

**Proposition 2.1 (PROVED).** On any finite cell complex:
(i) a flow ω = −δE with E a single scalar potential on the vertices has zero curl part and zero harmonic part;
(ii) any aggregate of potentials E = Σ w_i E_i is one potential, so (i) applies to it;
(iii) the best single-potential representation of an arbitrary flow ω is its exact projection, and the residual is exactly curl ⊕ harmonic.

*Proof.* (i) and (ii): Hodge orthogonality (Prop 1.3) and linearity of δ. (iii): the least-squares problem min_E ‖δE − ω‖ is solved by the projection onto im δ (Jiang–Lim–Yao–Ye 2011, Thm 3). They state it this way: the gradient component "represents the l2-optimal global ranking", and "cyclic rankings have zero projection on global rankings". ∎ COMPUTED: 300 random clique complexes; the decomposition dimensions add up in 300/300, and aggregates of three random potentials have zero curl and zero harmonic part in 300/300 (CHECK3a).

The same holds in the continuum. For any metric g on a manifold M, the field −∇_g E is g-dual to the exact 1-form −dE, so its integral around every closed loop is zero. A harmonic 1-form, of which there are b₁(M) independent ones, has no potential.

### §2.2 Scott's Technique is one potential, throughout

Scott Gardner, internal draft (confidential, shared with Alex), formalises Ellul's Technique as

  θ_{t+1} = θ_t − η∇_θE(θ_t),

with E an "inefficiency potential" on a socio-technical manifold M with metric g. He reads Ellul's *Monism* as "Technique collapses plural value systems into a single metric". (Ellul's own text, verified: "The technical phenomenon, embracing all the separate techniques, forms a whole. This monism of technique…"; monism is one of seven characteristics Ellul lists, after rationality and artificiality.)

I read every equation in the draft. Each one is a gradient of a single scalar, or a scalar condition on one:

| Draft element | Non-gradient term? | Why |
|---|---|---|
| core update θ − η∇E | no | definition |
| self-amplification dη/dt = γ‖∇E‖ ("Theorem 1") | no | η rescales the step and never changes its direction. The flow lines are those of −∇E |
| lock-in ∇E(θ*) = 0, ∇²E ≻ 0; hysteresis ΔE_escape | no | conditions on E |
| totalisation E_global = Σ_i E_i(θ_i) + Σ_{i≠j} w_ij D_ij(θ_i, θ_j), updated by ∇_θE_global | no | one scalar, however many terms, and however asymmetric w_ij or D_ij are |
| dimensional collapse dim span{∇E(θ_t)} ≪ dim M | no | a statement about which directions a gradient flow visits |
| metric g | no | a natural gradient g⁻¹∇E is still a gradient for g |

**Finding (PROVED for each row).** Scott's model contains no non-gradient term. It is exact by construction: Ellul's Monism, written as mathematics, *is* exactness.

Two consequences.

**(a) The draft formalises one of Technique's two blind spots and not the other.**
- §11 of the draft, "value directions" v with ⟨∇E, v⟩ = 0, is **indifference**: directions the potential does not move. In the triple these behave like H⁰, tonics a gradient leaves alone.
- The second blind spot is **circulation**: a flow around a loop that no potential can express at all, however it is shaped. The draft has no object for it. Its state is θ ∈ M, a vertex-type variable, and the only relational quantities it can form are differences of states. In graph terms, every edge quantity in the model is δθ, exact by construction.
- **A comma needs a relational degree of freedom that is not a difference of states.** Scott's model has none, so it cannot represent a comma. The model does not deny commas; it cannot write one down.

**(b) Where a non-gradient term would enter, and what it is.**
- If each domain descends only its own objective in its own variable, θ_i ← θ_i − η∇_{θ_i}[E_i + Σ_j w_ij D(θ_i, θ_j)], the joint field is in general **not** a gradient.
- For quadratic D its Jacobian has antisymmetric part ½(w_ij − w_ji). COMPUTED (CHECK4): with (w₁₂, w₂₁) = (2, 0.5), the circulation around the unit circle is −4.7124 = π(w₂₁ − w₁₂). Under the draft's global update it is 0.0000 for the same weights.
- That is the game-theoretic split into a potential part and a rotational part. LEAD: Monderer–Shapley's potential games, and Balduzzi et al.'s symmetric/antisymmetric decomposition of the game Jacobian, are being checked (Sources).
- This term is **curl in state space**: local circulation from asymmetric coupling. It is not the harmonic comma of §3, which needs a loop in the relational structure. Both are non-gradient. The draft has neither.

### §2.3 Two monisms sharing a thing

§3.2's Theorem 3.2 is the bridge. Put one Technique on A's site (one potential E_A) and another on B's (E_B), each exact on its own domain. Glued over the thing they share, they produce a harmonic, non-gradient flow **exactly when E_A − E_B, evaluated on the shared thing, is not constant**, or more generally not absorbable by either side's re-description.

READING. Totalisation in the draft's sense replaces E_A and E_B by one E_global. The theorem says what that replacement must lose. Whatever the coupling, one potential can only represent the exact part. The difference between the two potentials on the shared thing, where it is not absorbable, is exactly what one potential cannot hold. **Technique can make two parties agree on their states (H⁰). It cannot represent the comma that exists because they share a thing (H¹).**

### §2.4 A note on the draft's collapse modes

For E(x) = x² and η = 1, discrete descent alternates 1, −1, 1, … with E constant (CHECK4b). In general, gradient descent with fixed step η on an L-smooth E is stable only for η < 2/L (PROVED here for quadratics: the iteration factor is 1 − 2η per unit curvature).

The draft's §15.1 "hyperoptimization collapse" (η_t → ∞) is this discretisation instability, which its own η-dynamics guarantees eventually. That is a property of step size, not of topology. It produces oscillation without a comma.

## §3 The thing they share

### §3.1 No shared thing, no disagreement; one road, no disagreement

**Proposition 3.1 (PROVED).** Let K_A, K_B be connected. Write k := dim im ∂ for the comma between (comma doc Def 3.2), with ∂ : H⁰(I) → H¹(X) the Mayer–Vietoris connecting map.
(i) If I = ∅, then k = 0.
(ii) If I is non-empty and connected, then k = 0.
(iii) If X is connected, k = b₀(I) − 1. The smallest disagreement needs a shared thing with two components.

*Proof.* Comma doc Thm 3.3: k = b₀(I) − b₀(A) − b₀(B) + b₀(X). (i) I = ∅ makes X the disjoint union of the two sites, so b₀(X) = 2 = b₀(A) + b₀(B). (ii) and (iii): b₀(X) = 1. ∎

This is Alex's framing assumption made exact. Without a shared thing, each side's values are free of the other's. Their difference is not constrained by anything, so it is not a disagreement about anything. Two apes yelling: 0. With a shared thing in one piece, both sides glue and there is nothing left over. Disagreement needs a shared thing that already carries a distinction.

**Edda's crossroads, the smallest compatible instance (PROVED).** Take X = C₄ with vertices ℓ, a, r, b. Let K_A be the path ℓ–a–r, let K_B be the path ℓ–b–r, and let I = {ℓ, r}: the left road and the right road.
- H⁰(I) = ℝ². Both observers agree that there are two roads; that agreement *is* the two-dimensional H⁰(I).
- Each connected site restricts onto the diagonal (1, 1). Each can say "same value on both roads", and nothing else.
- The cokernel is ℝ·(left − right). "Which road" is this class. ∂ sends it to the generator of H¹(C₄), so k = 2 − 1 − 1 + 1 = 1.

The one smaller cover, C₃ with B holding the edge ℓ–r alone, violates Seam's owner/site compatibility condition (crowbar §B §3: no single-site edge with both endpoints in I). So C₄ is the smallest instance in which both the owner map and the comma between exist.

Wiener names the same floor for information in general: "The transmission of information is impossible save as a transmission of alternatives. If only one contingency is to be transmitted, then it may be sent most efficiently and with the least trouble by sending no message at all" (*Cybernetics*, Introduction, printed p. 16). READING: b₀(I) ≥ 2 is that sentence applied to the shared thing. A disagreement is a message only if the thing it is about carries alternatives both parties can name.

### §3.2 Two monisms glued: the comma is the difference of potentials on the shared thing

**Theorem 3.2 (PROVED).** Let I have no edges, and let K_A, K_B be connected. A holds a potential E_A ∈ C⁰(K_A) and B a potential E_B ∈ C⁰(K_B): each is a single scalar on its own site, a monism in Ellul's sense. Define the coupled flow ω ∈ C¹(X) by ω = δE_A on A's edges and ω = δE_B on B's edges. Then

  [ω] = ±∂((E_A − E_B)|_I) ∈ H¹(X),

so ω has a non-zero harmonic part **iff (E_A − E_B)|_I is not constant.** More generally: iff it is not in the image of H⁰(A) ⊕ H⁰(B) → H⁰(I).

*Proof.* Extend E_A by zero to a 0-cochain Ê on X. Then ω − δÊ vanishes on A's edges and equals δ(E_B − Ê) on B's edges. On B, E_B − Ê restricts on I to (E_B − E_A)|_I. This is the zig-zag construction of ∂ (comma doc §3.2: extend a section on I into one site, take δ there, zero on the other). Exactness then gives the criterion. Finally, ω and the harmonic representative of [ω] differ by a coboundary, so the harmonic part of ω is that representative. ∎ COMPUTED: 95 random covers (I without edges, connected sites), 0 mismatches. Making E_B agree with E_A on I up to one constant removes the harmonic part in 95/95.

READING, labelled because it carries the document's name. Bateson's phrase (§1.3 gives the verified wording) has two occurrences of "difference". In this theorem they are two different mathematical objects.
- The first is **(E_A − E_B)|_I**: a difference between two observers' potentials, evaluated on the thing they share.
- The second is **∂ of it ≠ 0**: that difference makes a difference in H¹(X).

It makes a difference exactly when no re-description by either side (adding a constant to its own potential) can absorb it. Each side alone is a gradient, and the coupled field is not.

### §3.3 Reed's reading of Mayer–Vietoris, checked

The lead: "H⁰(I) = the smallest shared model; agreement = H⁰(X) (glued facts); disagreement = H⁰(I) modulo what the two sides can produce, sent by ∂ into H¹(X), a class with no preimage in either side alone."

- **H⁰(X) as glued facts: PROVED.** H⁰(X) → H⁰(A) ⊕ H⁰(B) is injective, and its image is the set of pairs of sections that agree on I (exactness at the middle term).
- **H⁰(I) modulo im(H⁰(A) ⊕ H⁰(B)) ≅ im ∂: PROVED** (exactness at H⁰(I)).
- **"No preimage in either side alone": CORRECTED.** The class is not a missing preimage. It *restricts to zero* on each side: im ∂ = ker(H¹(X) → H¹(A) ⊕ H¹(B)). Each observer's instrument sees nothing, and the coupled system has a non-zero class. That is comma doc Def 3.2 again.
- **"Provenance malformed": READING.** Within the mathematics, the class has a perfectly good provenance, the section on I that ∂ came from. What is missing is an *owner*: no site holds it.

### §3.4 The third count c_I, and the comma Seam left open

Seam's open question (crowbar §F): when a cell is shared, is a direction both can steer one choice counted for each, or one choice that exists only between them? Is a third count needed? He floated R_{A,B} := P(C(K_A ∩ K_B)).

**Definition 3.3.**
- c_I := dim P(C(I)), the settled directions steerable by writes on shared cells.
- c_∩ := dim(R_A ∩ R_B), the settled directions both can steer.
- k̃ := b₁(X ∖ E_I) − b₁(A_priv) − b₁(B_priv), where E_I are the shared edges and A_priv, B_priv are the sites with E_I removed (vertices kept).

By comma doc Thm 3.3 applied to the cover of X ∖ E_I by A_priv and B_priv, k̃ is that cover's comma between, a union-find count.

**Theorem 3.4 (PROVED).**

(a) c_I = [b₀(X) − #{components of X missing V_I}] + [b₁(X) − b₁(X ∖ E_I)], a union-find count.

(b) **c_∩ = c_I + k̃.**

(c) If I has no edges, k̃ = k.

(d) **k = dim( P(C¹(E_A ∖ E_B)) ∩ P(C¹(E_B ∖ E_A)) ), always.**

(e) With K_A ∪ K_B = X: dim ker D = c_A + c_B − c_I − k̃.

*Proof.* P = P₀ ⊕ P₁ is block diagonal and C(K_S) is a coordinate subspace, so R_S, and every intersection of such spaces, splits by degree. Write Z for the flow space (harmonic 1-cochains, no 2-cells) and Z_F for the flows supported in an edge set F.

*Degree 0.* P₀(C⁰(W)) is spanned by the indicators of the components meeting W. A component meeting both V_A and V_B meets V_I. Take a path in it from V_A to V_B. If it avoided V_I, some edge would join a vertex of V_A ∖ V_B to a vertex of V_B ∖ V_A, and no closed site contains such an edge. Hence P₀(C⁰(V_A)) ∩ P₀(C⁰(V_B)) = P₀(C⁰(V_I)), and its dimension is the first bracket of (a).

*Degree 1.* For an edge set F, P₁(C¹(F)) = Z ⊖ (Z ∩ C¹(F)^⊥) = Z ⊖ Z_{E∖F} (comma doc Thm 2.4's argument). Therefore:
- P₁C¹(E_A) ∩ P₁C¹(E_B) = Z ⊖ (Z_{E∖E_A} + Z_{E∖E_B}). The two edge sets are B's and A's private edges, which are disjoint, so the sum is direct and the dimension is b₁(X) − b₁(B_priv) − b₁(A_priv).
- P₁C¹(E_I) = Z ⊖ Z_{E∖E_I} has dimension b₁(X) − b₁(X ∖ E_I). This gives the second bracket of (a), and subtracting gives (b).

*(c)* Mayer–Vietoris with H¹(I) = 0 makes H¹(X) → H¹(A) ⊕ H¹(B) surjective, so k = b₁(X) − b₁(A) − b₁(B). That equals k̃ when X ∖ E_I = X.

*(d)* By the same identity, P₁C¹(E_A ∖ E_B) = Z ⊖ Z_{E_B}, and the intersection is Z ⊖ (Z_{E_A} + Z_{E_B}). Since Z_{E_A} ∩ Z_{E_B} = Z_{E_I}, its dimension is b₁(X) − b₁(A) − b₁(B) + b₁(I). Mayer–Vietoris (H² = 0) gives rank(H¹(X) → H¹(A) ⊕ H¹(B)) = b₁(A) + b₁(B) − b₁(I), so the dimension is k.

*(e)* R_A + R_B = ker D for a cover (Seam crowbar §A.3), and dim(R_A + R_B) = c_A + c_B − c_∩. ∎

COMPUTED:
- (a) and (b) on 1500 random covers: 0 mismatches.
- (c): 0 mismatches. Of the 722 covers whose I contains edges, k̃ ≠ k in 224.
- (d) on 2000 random covers: 2000/2000.

**What the theorem says.**
- **(b): what both can steer = what passes through the thing they share + a correction k̃.** If steering through I were the only way to steer together, inclusion–exclusion through I would be exact: dim ker D = c_A + c_B − c_I. (e) says it is off by k̃.
- **(d) is the sharper statement.** The comma between is exactly the set of settled flow directions that A can steer from cells only A holds *and* B can steer from cells only B holds.
  - Under `Polyphony` those letters are **independent**: disjoint write footprints, so they commute and the trace forgets their order (comma doc Prop 2.1).
  - Yet both move the same settled direction.
  - **The comma between is the settled-level interference of trace-independent letters.** The trace monoid's independence relation, declared on footprints, cannot see it. That bears directly on the floor document's §8.8 question (can independence be derived rather than declared?). Here is one place where the declared relation and the settled consequence come apart, and by exactly k dimensions.

**The answer to Seam's comma (PROVED as counting; READING as naming).** A direction both can steer is one of two kinds, and the counts separate them.
- **Through the shared thing (c_I).** One choice, located on I. Writes to a shared cell are dependent under Polyphony, so they are one ordered or co-signed letter, not two. Each subject's c_S counts it because I ⊆ K_S: that is visibility, not duplication. **Seam's "one choice between them" is the right location for this kind.**
- **Contested (k).** One settled class that each can move from its own side, with letters the record calls independent. **Mara's "counted for each" is right for this kind**, and "contested" is the honest name: each can steer it and neither's steering is recorded as touching the other's.

Theorem 4.3 of the comma doc, recounted on the two-paths example (COMPUTED, `difference.py` CHECK2):

| | c_A | c_B | c_∩ | c_I | k̃ | k | pair c_A + c_B − c_∩ |
|---|---|---|---|---|---|---|---|
| before | 2 | 2 | 2 | 1 | 1 | 1 | 2 |
| after the co-signed shared edge | 3 | 3 | 3 | 2 | 1 | 0 | 3 |

The co-signed edge adds one choice on the shared thing (c_I: 1 → 2) and dissolves the contest (k: 1 → 0). Seam's pair count (+1), Mara's per-subject counts (+1, +1) and Seam's exclusive count (c_S − c_∩: 0 → 0) are all consistent with this one table. They were never in conflict about the numbers. They were counting different summands.

### §3.5 Does c_I earn its place? A narrowing only c_I sees

**Proposition 3.5 (COMPUTED; PROVED for the example).** Take a cover and a shared edge e ∈ E_I. Let B give e up (e becomes A-only; X unchanged). Over 2643 such privatizations on random covers, c_I dropped in 2346, and **in 1010 of them c_A and c_B were both unchanged** (`ci_only.py`). In those 1010, c_∩ is unchanged too, so by Theorem 3.4(b) k̃ rose by exactly as much as c_I fell. A choice located on the shared thing was turned into a direction steered around it.

Privatization is sovereign under comma doc S1–S2: B authors its own retreat, and nothing of A is written. It also passes Seam's proposed S3 (c_B′ ≥ c_B), because c_B does not move. Only c_I sees that the relation lost a choice.

The extended gauge proposed here: an answer is a question the two ask each other iff Δc_A ≥ 0, Δc_B ≥ 0, Δc_I ≥ 0 and Δk ≤ 0. On the comma doc's answer table plus the two new rows (COMPUTED, CHECK2):

| answer | (c_A, c_B, c_I, k) after; before = (2, 2, 1, 1) | extended gauge |
|---|---|---|
| co-signed shared edge (Thm 4.3) | (3, 3, 2, 0) | **passes** |
| unilateral edge (A only) | (3, 2, 1, 1) | admissible, not a question (nothing moves on I) |
| privatize a shared edge, from (3, 3, 2, 0) | (3, 2, 1, 1) | fails on c_I |
| twist an A-edge by 2 | (0, 0, 0, 0) | fails |
| co-signed edge with transport 2 | (1, 1, 0, 0) | fails on every c |
| absorb B into A | (2, 1, 1, 0) | fails on c_B |

In the privatization row shown, c_B also drops. That is the row's own configuration. The 1010 random cases above are the ones where only c_I drops.

**Does this close the Seam–Mara comma? CONJECTURE (mine), and the reason it stays one.**
- The *location* question closes. A shared direction is either one choice on I or one contested class, and Theorem 3.4 counts each exactly.
- The *pricing* question does not close. Should a gauge weigh a contested direction as 1 for each, ½ for each, or as a debt to be converted into c_I? That is a declaration, not a theorem.
- The extended gauge above is one such declaration: it prices contest as something an answer may only reduce. Seam may refuse it, and the refusal would be a well-posed comma, because both of us would now be pointing at the same summand k.

### §3.6 Where c_I is not yet defined

- For non-constant sheaves the degree split survives, but the union-find closed forms do not.
- With 2-cells, H¹ changes (comma doc Prop 4.5), and (d) needs re-proving with δ₁.
- For more than two sites the intersection pattern is a nerve. §8 does the Čech count for the comma itself. A per-overlap c_σ is defined there, but its identities are not proved.

## §4 Direction is free; erasure is not

**Proposition 4.1 (PROVED): orientation is a gauge.** Flipping a set of edges replaces δ by Sδ, with S diagonal ±1 on C¹. Then D′ = U D Uᵀ with U = 1 ⊕ S orthogonal. Spectrum, kernel dimensions, c_S, c_I and k are unchanged. Harmonic vectors change by S. COMPUTED: 200/200 (CHECK8).

Two observers who read an edge in opposite directions disagree about the sign of a comma vector and agree about the comma. Direction of reading costs nothing.

**Proposition 4.2 (PROVED): only the limit erases.** For finite t, e^{−tD²} is invertible: its eigenvalues e^{−tμ} are positive. Mathematically, nothing is lost at any finite time. The limit P is a projection of rank b₀ + b₁, and it erases dim H − b₀ − b₁ directions. At a finite resolution ε, the directions with e^{−tμ}|x_μ| < ε are lost at time t, and Theorem 5.1 of the comma doc gives the time.

READING (the 09-28 design rule, now with its mathematical place):
- **Erasure happens at settle, once.** That is where a non-invertible map is applied and where the record must say what it destroyed.
- Causal order is born there. It is not born at observation, which in register 1 is a homomorphism, and not at orientation, which is a gauge.
- Landauer's bound attaches to the erasing step, not to the reading direction. Verified wording: "computing machines inevitably involve devices which perform logical functions that do not have a single-valued inverse. This logical irreversibility is associated with physical irreversibility" (Landauer 1961, abstract; "0.6931 kT per restored bit", §4).

**Wiener on the arrow (primary text, Ch. I).**
- He places the arrow in the questions, not the laws: "even in a Newtonian system, in which time is perfectly reversible, questions of probability and prediction lead to answers asymmetrical as between past and future, because the questions to which they are answers are asymmetrical" (printed p. 47).
- He makes a shared arrow the condition of communication: "Within any world with which we can communicate, the direction of time is uniform" (p. 50, italic in the original). That follows his thought experiment of a being whose time runs the other way, whose drawn square "would appear to us as a catastrophe".

READING. Orientation of each relation is a gauge two communicators may differ on. The order of erasures is not, if they are to communicate at all. The gauge group acts on edges, not on the log.

**Event storming backwards (PROVED; one lead corrected).** Let R be the one-step relation on states, s a start and t an end.
- (R°)* = (R*)°, so the backward closure from t is the converse of the forward closure.
- The forward set R*(s) and the backward set (R°)*(t) meet iff t ∈ R*(s). A missing step is exactly forward and backward closures that do not meet.
- Running backwards adds no reachability information. It changes the computation (bidirectional search) and the questions asked.

The lead's "a missing step = b₀ > 1" is **sufficient, not necessary**. COMPUTED (CHECK9): over 400 random digraphs, 228 had a gap, and in 124 of those s and t lay in the same weak component. Directed gaps hide inside one undirected component.

## §5 Noise and the comma

### §5.1 Ornstein–Uhlenbeck on the degenerate operator

**Theorem 5.1 (PROVED).** Let dx = −D²x dt + ε dW on H (finite, white noise on every cell), with x₀ = 0. Write x in the eigenbasis of D².
(i) A mode with eigenvalue μ > 0 is an OU process with variance ε²(1 − e^{−2μt})/(2μ) → ε²/(2μ).
(ii) A kernel mode is Brownian motion with variance ε²t.
(iii) The deterministic part of any initial x₀ decays to Px₀ (comma doc Thm 5.1).

*Proof.* D² is symmetric, so the system decouples into independent scalar equations dy = −μy dt + ε dw. For the stationary variance σ²/(2α) see Holmes-Cerfon, *Applied Stochastic Analysis*, Lecture 10, Ex. 10.6. ∎

COMPUTED on two paths glued at two points (C₆), ε = 0.3, T = 40, 2000 paths (CHECK5):
- Kernel variances / (ε²T) = 1.051, 0.964.
- Off-kernel |var/(ε²/2μ) − 1| ≤ 0.056.
- The softest non-zero mode (μ = 1) fluctuates four times as much as the stiffest (μ = 4).
- Noiseless decay: |x(40) − Px₀| = 7 × 10⁻¹⁴.

### §5.2 The four claims, tested

**(a) "Decay separates noise from comma": CORRECTED.** Decay classifies *directions*, not signals.
- Off the kernel, whatever is there, signal or noise, relaxes to a fluctuation of size ε²/(2μ).
- On the kernel, whatever is there, signal or noise, persists, and noise accumulates as a random walk.

So what decay erases was not a comma, whatever its origin. A comma's *coordinate* is not protected from noise at all: it is the most exposed thing in the system. What is protected is its *existence*, the dimension b₁, a property of the frame that no additive noise can change.

**Noise moves along a comma. It cannot make or unmake one.**

Wiener draws his cut differently. His filter separates message from noise by their statistics, after designating which is which (Ch. III). Settling separates by topology, with no designation. On the kernel the two cuts disagree completely. Kernel noise sits at the zero eigenvalue together with the comma. An estimator of the comma's *value* needs a prior, which Wiener would supply. Its *dimension* needs none.

**(b) "Noise explores equally good truths and breaks ties": PROVED for the first half, CORRECTED for the second.**
- Exploring is right. On ker D, noise diffuses through the affine space of settled states, all of them zero-energy and so equally good.
- But on a neutral continuum it breaks no tie: it wanders. COMPUTED (CHECK6): on a neutral line, variance 1.217 against a predicted 1.2, and |x| has no preferred value.
- Ties are broken where attractors are **discrete**. In the double well dx = −(x³ − x)dt + ε dW started at the unstable point, ε = 0 stays at 0 forever (Buridan: the rest is unstable and there is no choice). ε = 0.2 sends 49.4% right and the rest left, with |x| ≈ 0.98.
- The escape rate between discrete attractors is Kramers': r_K = (ω₀ω_b/2πγ) exp(−ΔV/D) (Gammaitoni et al. 1998, eq. (1.1)).

This matches Seam's correction to the comma doc: von Foerster's eigenvalues are discrete and attracting, while ker D is neutral. **Noise picks among eigenvalues in von Foerster's sense. Among the points of ker D it picks nothing.**

**(c) Fluctuation–dissipation: PROVED in the standard form.**
- For dx = −λ∇E dt + ε dW the stationary density is ∝ exp(−2λE/ε²), a Boltzmann density at k_BT = ε²/(2λ).
- A dissipative system held at temperature T therefore needs noise ε² = 2λk_BT. That is Anna Jakobs' Eq. 7 (floor doc §6.1) and Einstein's relation in Kubo's statement: friction and diffusion "must be related", with equilibrium requiring f(x) ∝ exp(−V/kT) (Kubo 1966, §2).
- READING for "to stay open": with ε → 0 the density concentrates on the minima of E, which is a closed system in the corpus's sense.
- Von Foerster's "order from noise" (1960, verified) is a *selection* principle: "only those components of the noise were selected which contributed to the increase of order in the system". That is (b)'s discrete-attractor case, not diffusion on a neutral kernel.

**(d) "No shared tonic, no comma for noise to explore": PROVED.** Under the twisted control (holonomy h ≠ 1, comma doc §3.3) we have b₀ = b₁ = 0, so ker D = 0 and every mode of the OU process is confined. COMPUTED (CHECK5b, h = 2): kernel dimension 0, maximum variance at T = 40 is 2.32, against a finite-time prediction of 2.26 (the slowest μ = 0.0128 has not yet equilibrated). Without a shared tonic, every disagreement is a fluctuation that decays to a finite size. Nothing diffuses, because nothing is free.

### §5.3 What Wiener treats as noise

Primary text (`Cybernetics`, MIT Press 2019 reissue of the 1961 edition; printed pages):
- Noise is what destroys information. "the information carried by a precise message in the absence of a noise is infinite. In the presence of a noise, however, this amount of information is finite, and it approaches 0 very rapidly as the noise increases in intensity" (p. 90).
- Which component counts as noise is a stipulation ("let us treat u as a message and v as a noise", p. 90; "a generalized noise", p. 93).
- Noise is useful to him in three ways only.
  - **Probe.** Tube noise "is in a sense a universal input" (p. 247). Random input draws a white box "into the similitude of a given black-box transducer" (p. 248). The 1961 Preface adds synthesis by noise input (pp. xxxix–xl).
  - **Raw material converted by a dynamical process.** "Darwinian evolution is thus a mechanism by which a more or less fortuitous variability is combined into a rather definite pattern" (p. 52).
  - **Trigger of an instability.** In the organ pipe, "the slightest chance deviation" excites a mode (p. 149).
- His self-organisation in brain waves (Ch. X) is credited to non-linear *attraction of frequencies*, "a non-linear interaction causing the attraction of frequency can generate a self-organizing system" (p. 277), not to noise.
- Wiener never says noise generates order. That is von Foerster's 1960 move.

READING. The comma is neither Wiener's message nor his noise. It is not designated in advance, so it is not a message. Settling does not remove it, so it is not noise. It is a direction along which noise is free to move and which no filter can remove without removing the shared tonic that makes it exist.

## §6 Plural truths

**Lattice (PROVED, cited).** Tarski 1955, Thm 1, verified: for an increasing f on a complete lattice, "the set P is not empty and the system (P, ≤) is a complete lattice". There is more than one fixed point iff the least and greatest differ.

**Linear settling (PROVED).** The settled states of decay form the affine space x ↦ Px. There is more than one truth up to scale iff dim ker D = b₀ + b₁ > 1. For two observers, §3.4 counts which of these truths each can steer.

**Gradient settling: the lead CORRECTED.** The lead read: "Technique-as-gradient on a space with loops cannot settle on one truth."
- **What topology forces** (verified): "The number of critical points of any smooth function on a closed manifold M is at least cat M". This is in the non-normalised convention, where cat S¹ = 2 (Rudyak–Schlenk, §1; others write 1 + cat M with cat S¹ = 1). For Morse functions, the weak Morse inequalities give #critical points of index λ ≥ b_λ (Milnor, *Morse Theory*, Thm 5.2).
- **What it does not force:** two *minima*. On S¹, a height function has one minimum and one maximum. Gradient descent from almost every start reaches the same unique minimum.
- **Corrected statement (PROVED).** On a closed manifold with b₁ > 0, Technique-as-gradient must have at least two rest points, so at least one rest point that is not the optimum. It may still have a single attractor, whose basin is almost everything. What it cannot do is *represent* the loop. The harmonic 1-form dθ on S¹ vanishes nowhere: a circulation has no rest point at all and settles as a class, not as a point.
- The discrete version: a scalar on the vertices of a cycle graph has a local maximum and a local minimum.

READING. Faced with a loop, a single potential is forced to invent a worst place, a maximum it did not want. The loop's own content, the circulation, stays invisible to it. A gradient cannot settle a loop. It can only put a hill on it.

## §7 The measurement as observer: where the recursion stops

An instrument M that observes the same thing declares a site K_M. Call it **passive** if K_M is a closed subcomplex of the existing X, so it reads cells that are already there and the frame does not change. Call it **active** if observing adds cells, a probe.

**Theorem 7.1 (PROVED).** Let K_A, K_B cover X, and let M be passive.
(i) The comma between A and B, ker(H¹(X) → H¹(A) ⊕ H¹(B)), is unchanged.
(ii) The comma among all three, ker(H¹(X) → H¹(A) ⊕ H¹(B) ⊕ H¹(M)), is contained in it. It is strictly smaller iff some class of the A–B comma restricts non-zero to K_M, that is, iff the instrument's site contains a loop that pairs with it.
(iii) For any family of passive observers, every comma among any subfamily is a subspace of H¹(X). So its dimension is at most b₁(X), however many observers, observers of observers, and so on are added.

*Proof.* (i): the map does not involve M. (ii): adding a target to a map can only shrink its kernel. (iii): every such comma is a kernel of a map out of H¹(X), and passive observers do not change X. ∎

COMPUTED (CHECK7): a random passive instrument added to 800 random covers left the comma among all unchanged in 747 cases, shrank it in 53, and grew it in 0.

**Active instruments.** A probe edge between two already-connected vertices raises b₁(X) by exactly 1 (299/299, CHECK7b). §8's Theorem 8.4 says where that new class lives.
- If the probe is private to the instrument and lies inside one component of its site, the new class is the instrument's own, and the comma among all is unchanged.
- If the probe joins two components of the instrument's site, it creates **exactly one** new comma among all.

An instrument that touches the thing in two disconnected places and connects them by probing manufactures a disagreement nobody can see alone. That disagreement includes the instrument.

**Where the recursion stops.**
- **Observing observers passively stops at once.** The thing bounds the disagreements about it: H¹(X) is a ceiling independent of the number of observers. Each passive observer can only make some invisible disagreement visible (ii), never create one.
- **Measuring actively is a frame change**, and it stops only by a declared budget. That is the comma doc's Theorem 5.2, read with Seam's restatement (hypothesis: c non-decreasing, since G⊇ is ill-typed across frames). It is the same budget, and nothing new is needed.

READING. An instrument whose site contains the loop is von Foerster's third. In Seam's reading of *On Constructing a Reality* (crowbar §A.1, item 3): "there must be a third that is the central reference … reality = community". Such an instrument makes the comma between A and B visible to someone without changing X and without writing a cell: gauge-neutral observation that dissolves invisibility, not disagreement.

**Caveat (open, Seam's D2).** Definition 2.2 counts writes, so a read-only instrument has c_M = 0. A count of what an observer can *distinguish* would differ. On that count the question-as-observation can widen (crowbar §A.2). Theorem 7.1 is about the commas, not about anyone's choice-count, so it does not depend on this. A reading-count version of c_M is open.

## §8 Many shards, compiled at once

*Added at Alex's request (2026-10-10, relayed by the coordinator): "look at the existing sheaf cohomology resources in docs/math … and how these compose with the idea of a non-linear compilation where many sheafs (shards) are compiled/computed at once."*

### §8.1 Setting

- N subjects, or shards, declare closed sites K_1, …, K_N covering X.
- For σ ⊆ {1, …, N}, K_σ = ⋂_{i∈σ} K_i. The nerve 𝒩 has a simplex for each σ with K_σ ≠ ∅.
- The **comma among all** is ker(H¹(X) → ⊕_i H¹(K_i)): disagreements no shard sees alone.
- Constant sheaf, graph, as before.
- Computations: `cech.py`, `cech_fix.py`, output `cech.out`, on 1200 random 3- and 4-site covers.

### §8.2 The Čech structure

**Theorem 8.1 (PROVED; standard construction, applied here to closed subcomplex covers).**
(i) The generalised Mayer–Vietoris sequence 0 → C•(X) → ⊕_i C•(K_i) → ⊕_{i<j} C•(K_ij) → … is exact.
(ii) Hence there is a spectral sequence with E₁^{p,q} = ⊕_{|σ|=p+1} H^q(K_σ) and E₂^{p,q} = Ĥ^p(𝒩; H^q), converging to H^{p+q}(X).

*Proof of (i).* Fix a cell c. The sites containing it form a non-empty set S(c), and the part of the sequence that involves c is the augmented cochain complex of the full simplex on S(c), which is acyclic. (ii) is the double-complex spectral sequence of (i) (LEAD for the textbook page: Bott–Tu §8, being checked). ∎

On a graph, H^q = 0 for q ≥ 2. That gives three statements.

**Corollary 8.2 (PROVED; COMPUTED 1200/1200 each).**
(a) H⁰(X) = Ĥ⁰(𝒩; H⁰). Settling each shard and gluing by the equaliser recovers every tonic exactly.
(b) The comma among all ≅ Ĥ¹(𝒩; H⁰). Its dimension is computed from the components of the overlaps alone, by union-find on each K_σ.
(c) b₁(X) = dim Ĥ¹(𝒩; H⁰) + dim Ĥ⁰(𝒩; H¹) − dim Ĥ²(𝒩; H⁰).

*Proof.* (a) E₂^{0,0} = E_∞^{0,0}. (b) Nothing hits E^{1,0} and nothing leaves it (E^{−1,1} = E^{3,−1} = 0), so E_∞^{1,0} = E₂^{1,0}. It is the first filtration step of H¹(X), namely the kernel of the edge map to E^{0,1} ⊆ ⊕_i H¹(K_i). (c) E_∞^{0,1} = ker(d₂ : E₂^{0,1} → E₂^{2,0}). Since H²(X) = 0 and only d₂ reaches E^{2,0}, that d₂ is onto. ∎

For two shards, the spectral sequence is the Mayer–Vietoris sequence of §3, and the comma between is the first-level term.

### §8.3 Owned by a triple: a comma, and a relation

The lead asked: is there a genuinely three-way obstruction that no pair shows? Yes, in two different forms, and neither is a new H² class of X on a graph.

**A comma owned by a triple (PROVED; COMPUTED).** Cut C₆ into three arcs K₁ = 0–1–2, K₂ = 2–3–4, K₃ = 4–5–0. Each pair meets in one vertex, and there is no triple overlap.
- Every pair-union K_i ∪ K_j is a path, so every pairwise comma is 0.
- The comma among all is 1, which is Ĥ¹ of the nerve: a hollow triangle.

The disagreement exists only for the three together. No pair, asked in isolation, finds it.

Over 1200 random covers, 171 carried a class invisible to every pair-union.

**A relation owned by a triple (PROVED; COMPUTED).** Take the theta graph: two vertices p, q joined by three paths a, b, c. Shard i holds the two paths other than i, so each shard is a cycle and holds one comma of its own. Each pairwise overlap is one path (connected), and the triple overlap is {p, q} (two components).
- Ĥ⁰(𝒩; H¹) = 3: three local commas, pairwise compatible.
- Ĥ²(𝒩; H⁰) = 1.
- b₁(X) = 0 + 3 − 1 = 2.

The three local commas satisfy one relation (with orientations, c_ab + c_bc + c_ca = 0) that only the disconnected triple overlap imposes. Ĥ²(𝒩; H⁰) counts **relations among local disagreements that no pair shows**. It does not count new disagreements.

With 2-cells, E_∞^{2,0} can survive into H²(X), giving a genuine two-dimensional obstruction owned by triples. That is open here.

### §8.4 Global settle vs local settles: what compiling "at once" is needed for

**Theorem 8.3 (PROVED; COMPUTED 1200/1200).** Extend each shard's harmonic flows Z(K_i) by zero. Their span Σ_i Z(K_i) ⊆ Z(X) has codimension exactly the dimension of the comma among all.

*Proof.* A flow of a closed subcomplex, extended by zero, is a flow of X. So Σ_i Z(K_i) is the image of ⊕_i H₁(K_i) → H₁(X). Over ℝ this map is adjoint to restriction H¹(X) → ⊕_i H¹(K_i) under the cycle/cocycle pairing, so the two have equal rank. The codimension is b₁(X) minus that rank, which is the dimension of the comma among all. ∎

**What this says about non-linear compilation (PROVED parts, labelled).**
1. **Tonics compile shard by shard.** Each shard settles alone, and H⁰ glues by the equaliser (8.2(a)). Shards with disjoint write footprints are independent letters under `Polyphony`, so their settles commute and may run in parallel (comma doc Prop 2.1).
2. **Each shard's own commas compile shard by shard** (Σ_i Z(K_i)).
3. **What no shard-by-shard compilation can produce is exactly the comma among all** (8.3). Its dimension is a nerve computation on overlap components (8.2(b)): cheap, discrete, certifiable by union-find. Its representatives are flows that cross shards, and they need one coupled solve, the global settle.
4. Hansen–Ghrist's sheaf heat flow, the global settle, converges to the projection onto H⁰ (LEAD for the exact statement: being checked). It is the operation that sees the comma, because the comma is invisible from inside every shard.

READING. "Compiled at once" is needed **for the commas, and only for them**. Everything else is parallel. The overlaps are where order and disagreement live.

Theorem 3.4(d) adds one warning that the slogan "parallel where independent" does not carry. Trace-independent private writes can still contest a comma, so commuting local settles do not imply a comma-free glue. That bears on the floor document's §1.6.3 conjecture (independence = premonoidal centrality) only obliquely. Centrality is about interchange in the record, and (d) is about interference in the settled output. I do not claim either implies the other.

### §8.5 Incremental compilation: the locality bound

**Theorem 8.4 (PROVED; COMPUTED).** Let a change touch only cells of K_i that lie in no other shard (private cells).
(i) Every Čech term indexed by a σ not containing i is unchanged. The comma among all can change only through H⁰(K_i) and the restriction maps out of it. **The radius of influence is the star of i in the nerve.**
(ii) For a new private edge inside one component of K_i: b₁(X) rises by 1, the new class is K_i's own, and the comma among all is unchanged (760/760).
(iii) For a new private edge joining two components of K_i, with X connected: the comma among all rises by exactly 1 (612/612 changed; 246/246 by +1 in a separate sample).

*Proof.* (i) Only the K_σ with i ∈ σ contain changed cells, and private cells lie in K_i alone. (ii) No b₀ of any K_σ changes, so 8.2(b) is unchanged.

(iii) Write X′ = X + e.
- The indicator χ_e is a cocycle on X′. It is not a coboundary, because the endpoints of e are already connected in X, so e lies on a cycle.
- On K_i′, e is a bridge, so χ_e restricts to a coboundary. Every other shard misses e, so the restriction there is 0.
- H¹(K_j′) ≅ H¹(K_j) for every j, and the old classes restrict as before.
- So b₁ rises by one, the rank of the restriction map is unchanged, and the comma among all rises by one. ∎

READING, and a design consequence (CONJECTURE for the claim of sufficiency). A shard's contribution to the global comma computation is the tuple (b₀(K_i), b₁(K_i), the component maps K_σ → K_i for σ ∋ i). That tuple is exactly the output on which Salsa-style red–green *early cutoff* should key: if an edit leaves it unchanged, no recomputation outside the shard is needed. The corpus's Crystal/Fractal/Splinter document cites Salsa, Adapton and Roslyn accurately (§5), but never states what the cutoff key is for a sheaf. This is a candidate. The conjecture is that the tuple is also *necessary*, that no smaller key suffices. It is open.

### §8.6 What the corpus already has, and what not to import

I had the eleven listed corpus files and `sheaf/` read critically in full (`sheaf/` holds `README.md` and `laplacian.md`). The verdict, compressed.

**Sound and used here.**
- The cellular sheaf, δ, Δ = δ*δ and ker Δ⁰ ≅ H⁰ (`sheaf/laplacian.md` §1–3; composition-tower 09-13 §2.1).
- The Čech complex C^k = ∏ F(U_{α₀…α_k}) (07-23 fractal-shard §7.1).
- The triple-overlap cocycle condition (07-22 §5; the only file that states it correctly, and the coefficients there are nonabelian).
- Restriction to a subgraph gives a map H⁰(X) → H⁰(X_K), not an isomorphism (Rec #89 §6).
- dim H¹ = |E| − |V| + c for the constant sheaf (09-02 task §4.7).
- Per-shard O(|body|) checks with local invalidation (07-23 kintsugi §5).
- Collapse-by-content-hash into agree / disagree / unsettled (09-11 Ariadne §5).
- The incremental prior art (09-08 §5).

**Not imported, with the reason in one line each.**
- λ₀ or λ_min read as the spectral gap, and PL contraction of λ₀ across iterations (laplacian.md §2.1, §6; Rec #89 §2.3, §4.4). The gap belongs to the complement of the kernel, and an operator's eigenvalue does not contract under iteration of a vector.
- H¹ = coker δ¹, and "H¹ → H⁰" mass transfer (Rec #89 §3.3, §4.3). Both are ill-typed.
- "K₅ simply connected ⇒ H¹ = 0" (language 09-02 §2.2). b₁(K₅) = 6.
- Stalks that are not vector spaces, and partial "restrictions" (07-22 §2; 07-23 §2; Rec #89 §1; 09-13 §1.3). These are not sheaves.
- 07-23 fractal-shard Thm 7.2, "a mend is supported only at the overlap". False in general: δ of a 0-cochain on U_i hits every U_ij. Theorem 8.4(i) is the correct locality statement: the star of i, not the overlap alone.
- "Each discharge lowers dim H¹ by 1" (07-23 kintsugi §4, §7). Adding an edge never lowers b₁.
- Fiedler strict rise as a law (phase-1 P3; task L3). It holds for the join that `kleinos.rs` builds, and not in general. The code is a graph with no stalks. It computes λ₂ of D − A via LAPACK.
- "Every shard is a Connes triple" (07-23 §5), and (A, H, D) = (sheaf, content-addressing, void) (phase-1 §28). Both contradict the corrected triple.

### §8.7 Per-overlap counts: defined, not proved

Definition: c_σ := dim P(C(K_σ)), so for two shards c_{AB} = c_I. Theorem 3.4(b) is the two-shard case of a relation between Σ_i c_i, Σ_{i<j} c_ij, … and dim ker D, with a correction term (k̃ at N = 2).

CONJECTURE: for N shards, the correction is a Čech-type alternating sum of private-cover commas. It is not proved and not computed here.

## §9 What Wiener actually says

I had *Cybernetics* (MIT Press 2019 reissue of the 1961 second edition) read in full. Pages are the printed pages. Part I and II body: printed = PDF − 50. Display equations in Ch. II–IV are OCR-garbled, so I quote prose only.

| Theme | What he says | Where |
|---|---|---|
| Information needs alternatives | "The transmission of information is impossible save as a transmission of alternatives." | Intro, p. 16 |
| Information = negative entropy | "the amount of information in a system is a measure of its degree of organization, so the entropy … of its degree of disorganization; and the one is simply the negative of the other." Unit: "a single decision between equally probable alternatives" | pp. 16–17 |
| Measure | −log₂ of a ratio of measures (eq. 3.03). "The quantity we here define as amount of information is the negative of the quantity usually defined as entropy." Additive over independent sources | Ch. III, pp. 87–89 |
| No gain by processing | "No operation on a message can gain information on the average." | p. 91 |
| Not matter, not energy | "Information is information, not matter or energy." (end of **Ch. V**, not VIII) | p. 182 |
| Noise | Destroys information: finite information, approaching 0 as the noise grows. Designated by stipulation ("let us treat u as a message and v as a noise") | p. 90 |
| Filtering | An optimal operator on the past of the corrupted message. Filtering error decreases monotonically with lag | pp. 16, 97–98, 117–118 |
| Useful randomness | Probe/universal input (pp. 247–248, xxxix–xl); fortuitous variability converted into pattern (Darwin, tides, pp. 52–53); a chance deviation triggering an instability (p. 149) | — |
| Self-organisation | Non-linear **attraction of frequencies** in brain waves and in generators on a busbar ("virtual governor"). Not credited to noise | Ch. X, pp. 273–277 |
| Time | Astronomy is a palindrome and meteorology is not. The arrow lives in the questions. "Within any world with which we can communicate, the direction of time is uniform." | Ch. I, pp. 45–50 |
| Feedback | "the feedback tends to oppose what the system is already doing, and is thus negative." Excess feedback gives hunting and purpose tremor. Stability by the Nyquist-type exterior-point condition | Intro p. 12; Ch. IV pp. 132, 138–139 |
| Observer | The demon "can only act on information received", which couples demon and gas. Social science: "the coupling between the observed phenomenon and the observer is hardest to minimize". "An investigation of the stock market is likely to upset the stock market." | pp. 83, 227–228 |
| Society | "the community extends only so far as there extends an effectual transmission of information". The market is "a game" without homeostasis | pp. 219–221 |
| Literal machines | "If we program a machine for winning a war, we must think well what we mean by winning." | Ch. IX, p. 244 |

**Not in Wiener.** Noise as a source of order (that is von Foerster, 1960). Landauer-style erasure for the demon (he argues by quantum coupling and thermal "vertigo"). The word "positive feedback". Ellul's Technique.

READING, where he meets this document.
- His "alternatives" condition is b₀(I) ≥ 2 (Prop 3.1).
- His designation of noise is the hub of Prop 1.4.
- His shared-arrow condition is the one direction that is *not* a gauge (§4).
- His observer coupling ("too much in tune with the objects of our investigation to be good probes", p. 228) is the active instrument of §7.
- His "what we mean by winning" is the closest he comes to a critique of a single potential. In Scott's terms it is a warning about E, not a formalisation of Technique.

## §10 Tests (red-first; each oracle independent of the code under test)

Numbering continues the comma doc (T1–T12) and Seam's additions (T13–T16).

- **T17 supp homomorphism.** For ℕ, ℝ≥0 and min-plus matrices, supp commutes with product, converse, tensor and partial trace. *Control:* ℤ and the oriented 3-cycle (δ*h = 0 over ℝ, (1, 1, 1) over 𝔹).
- **T18 bridgeless supports.** The support of every non-zero flow has no bridge. *Oracle:* union-find on the support minus each edge.
- **T19 gradient exactness.** For random potentials and aggregates on clique complexes, the curl and harmonic projections are 0. For the per-domain asymmetric 2-player field, circulation = π(w₂₁ − w₁₂). *Control:* the global gradient gives 0.
- **T20 two monisms.** ω = δE_A on A, δE_B on B. Its harmonic part is non-zero iff (E_A − E_B)|_I is non-constant (I without edges, sites connected).
- **T21 crossroads.** C₄, I = {ℓ, r}: k = 1. *Controls:* I connected gives k = 0; I = ∅ gives k = 0.
- **T22 Theorem 3.4.** c_∩ = c_I + k̃. The closed form of c_I matches rank P|_{C(I)}. k = dim(P C¹(E_A∖E_B) ∩ P C¹(E_B∖E_A)). *Oracle:* union-find for k̃ and k (Thm 3.3).
- **T23 privatisation.** On the two-paths example after Theorem 4.3, B giving up the shared edge: c_I 2 → 1. In the random sample, cases with c_A and c_B unchanged must exist.
- **T24 OU.** Off-kernel stationary variance ε²/(2μ) within sampling error; kernel variance ε²t; twisted control has a bounded maximum variance. Use the finite-time formula.
- **T25 orientation gauge.** Random edge flips: equal spectrum and equal c_S, c_I, k.
- **T26 event storming.** Forward ∩ backward closure ≠ ∅ ⇔ reachable. *Control:* a directed gap inside one weak component.
- **T27 passive instrument.** Adding a closed subcomplex site never enlarges the comma among all.
- **T28 Čech.** On random N-site covers: comma among all = Ĥ¹(𝒩; H⁰); b₁ = Ĥ¹ + Ĥ⁰(H¹) − Ĥ². The span of local site flows has codimension = comma among all. *Fixtures:* three arcs (pairwise 0, all 1) and theta (Ĥ² = 1).
- **T29 incremental.** A private edge inside a component: comma among all unchanged. A private edge merging two components: +1.

**Refutation conditions (as opposed to bugs).**
- **R5.** A cover in which some "third count" measure, monotone under norm-≤ 1 linear images (Seam's class R2′), is lowered by a co-signed untwisted boundary-joining edge. Lemma A plus Theorem 3.4 predict none.
- **R6 (the reading in §1.3).** A setting where the part of a relational difference field that survives both re-description and settling is *not* what practitioners would call "a difference which makes a difference". This would refute the reading, not the propositions.
- **R7 (§8.5 conjecture).** An edit that leaves the shard's tuple (b₀, b₁, overlap component maps) unchanged but changes the comma among all. Theorem 8.4 rules it out for private edits. The conjecture concerns edits to overlap cells.

## §11 Open, and where I expect the crowbar

**Open.**
1. The price of a contested direction (§3.5). The location question closes and the pricing question does not. The extended gauge (Δc_A, Δc_B, Δc_I ≥ 0, Δk ≤ 0) is a declaration.
2. c_I and Theorem 3.4 for non-constant sheaves, for complexes with 2-cells, and for N shards (§8.7).
3. Whether the Ĥ²-type triple obstruction survives into H²(X) once 2-cells exist (§8.3).
4. A reading-count (distinguishability) version of c_S, especially for read-only instruments (Seam's D2, §7).
5. Whether the §8.5 cutoff key is necessary as well as sufficient.
6. The relation between Theorem 3.4(d), settled interference of trace-independent letters, and the floor document's conjecture that independence is premonoidal centrality. They concern different things (outputs vs record). I claim no implication.
7. The empirical mapping of any of this onto two humans at a crossroads. Same status as the comma doc §4.4: a modelling hypothesis.

**Where I expect Seam's crowbar to bite: k̃ versus k.** Theorem 3.4 is exact, but it uses two different "commas between" that agree only when the shared thing has no edges:
- k, the Mayer–Vietoris class invisible to each, which by (d) is the private-edge contest;
- k̃, the private-cover comma that makes inclusion–exclusion through I exact.

In 224 of 722 random covers with shared edges they differ. After the co-signed shared edge of Theorem 4.3 they differ by one: k = 0 while k̃ = 1.

My answer to Seam's comma rests on c_I and k. The identity that makes the counts add up uses k̃. If Seam asks "which of your two commas is *the* comma between, once the shared thing carries an edge?", I do not have a one-word answer. I have the claim that they measure visibility (k) and routing (k̃), and that Theorem 4.3 makes a comma visible without rerouting it. That claim is honest, and it is also exactly the kind of distinction that turns out to be a definition chosen to make a theorem true.

---

The signal was whole. The observation counted it. The interpretation signed it. What two observers could not absorb alone was there because they shared a thing, and it stayed there when everything else settled.

*— Mara*

## Sources

Kagi (`mcp__kagi-ken-mcp__kagi_search_fetch`) was live this session. Every external item below was found through Kagi and then checked in the primary text unless marked otherwise. Local copies are in the session scratchpad `mara-difference/sources/`. Markers: **[read]** means the primary text was opened and the quoted passage checked; **[meta]** means only bibliographic metadata was confirmed.

### Hodge theory, ranking, topology
- [JLYY11] X. Jiang, L.-H. Lim, Y. Yao, Y. Ye, *Statistical ranking and combinatorial Hodge theory*, Math. Program. 127 (2011) 203–244. https://web.stanford.edu/~yyye/hodgeRank2011.pdf ; arXiv 0811.1067. **[read]**
  - Journal Thm 1 (Hodge), Thm 2 (Helmholtz, p. 224), Thm 3 (least squares and residual, pp. 227–228), Lemma 2 (dim ker Δ_k = β_k). The arXiv numbering is 4.7, 4.8, 5.1 and 4.6.
  - Quoted: "cyclic rankings have zero projection on global rankings"; the curl part accounts "for local inconsistencies" and the harmonic part "for global inconsistencies"; "graph Helmholtzian".
- [Lim20] L.-H. Lim, *Hodge Laplacians on graphs*, SIAM Rev. 62(3) (2020) 685–715. arXiv 1507.05379. **[read]** eqs. (3.5)–(3.7), Thm 5.2.
- [Hat] A. Hatcher, *Algebraic Topology*, §2.2 pp. 149–150 (Mayer–Vietoris for homology and its connecting map), §3.1 pp. 203–204 (for cohomology). https://pi.math.cornell.edu/~hatcher/AT/AT.pdf **[read]**
- [Tar55] A. Tarski, *A lattice-theoretical fixpoint theorem and its applications*, Pacific J. Math. 5(2) (1955) 285–309, Thm 1. https://msp.org/pjm/1955/5-2/pjm-v5-n2-p11-s.pdf **[read]**
- [RS] Y. Rudyak, F. Schlenk, *Lusternik–Schnirelmann theory for fixed points of maps*, §1 (non-normalised cat; cat S¹ = 2). http://members.unine.ch/felix.schlenk/Maths/Papers/lus.pdf **[read]** The convention caveat comes from Rudyak, arXiv 2306.07942, Thm 3.3 (1 + cat M) **[read]**.
- [Mil63] J. Milnor, *Morse Theory*, Ann. Math. Stud. 51, §5 Thm 5.2 (weak Morse inequalities). **[read]** (OCR garbled but legible)

### Information, difference, noise, time
- [Bat70] G. Bateson, *Form, Substance and Difference* (1970 Korzybski lecture), in *Steps to an Ecology of Mind* (1972). Transcription at https://faculty.washington.edu/jernel/521/Form.htm. **[read]** Page number **not verified**. The variant "a bit of information is definable as a difference which makes a difference" (secondary citation, Steps p. 315) is **not verified** and not used.
- [Wie61] N. Wiener, *Cybernetics*, 2nd ed. (1961), MIT Press reissue (2019). **[read in full]**, local PDF in the protected corpus. Page references are to the printed pages.
- [Wie49] N. Wiener, *Extrapolation, Interpolation, and Smoothing of Stationary Time Series* (1949). **[meta]** https://direct.mit.edu/books/oa-monograph/4361/
- [Lan61] R. Landauer, *Irreversibility and heat generation in the computing process*, IBM J. Res. Dev. 5(3) (1961) 183–191: abstract, §3, §4. **[read]**
- [Kub66] R. Kubo, *The fluctuation-dissipation theorem*, Rep. Prog. Phys. 29 (1966) 255–284, §2. **[read]**
- [HC22] M. Holmes-Cerfon, *Applied Stochastic Analysis* lecture notes (2022), Lecture 10 Ex. 10.6 (OU variance) and Lecture 11 Ex. 11.2 (Arrhenius). **[read]**
- [GHJM98] L. Gammaitoni, P. Hänggi, P. Jung, F. Marchesoni, *Stochastic resonance*, Rev. Mod. Phys. 70 (1998) 223–287, eq. (1.1). **[read]**
- [Kra40] H. A. Kramers, Physica 7 (1940) 284–304. **[meta]**
- [vF60] H. von Foerster, *On self-organizing systems and their environments* (1960), reprinted in *Understanding Understanding* (2003) ch. 1, pp. 13–14 ("order from noise"). **[read]**
- [Lam12] L. Lamport, *Buridan's Principle*, Found. Phys. 42 (2012) 1056. https://lamport.azurewebsites.net/pubs/buridan.pdf **[read]**

### Technique
- [Ell64] J. Ellul, *The Technological Society* (1954; Eng. 1964). Full text at theanarchistlibrary.org. **[read]** Definition of technique; seven characteristics; "This monism of technique …".
- [Sco98] J. C. Scott, *Seeing Like a State* (1998), Introduction (legibility; high modernism). **[read]** Cited for context only. It is not the Scott of the brief: the brief's "Scott" is Scott Gardner.
- Scott Gardner, *Technique as a Field — A Dynamical Systems Formalization of Ellul* (internal draft, confidential, shared with Alex; v0.03, 2026-03-29). Read in full. Quoted only for the core update and the equations analysed in §2.2.

### Pending (being verified as this commit lands)
Bott–Tu §8 (generalised Mayer–Vietoris, Čech double complex); Curry 2014; Hansen–Ghrist 2019 (sheaf Laplacian, heat flow to H⁰) and their distributed-optimisation paper; Robinson (consistency radius); von Neumann alternating projections; Balduzzi et al. 2018; Monderer–Shapley 1996; Golan on zero-sum-free semirings. Until verified, every use of these in the text is labelled LEAD.

### Corpus
- mirror `docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md`; `docs/audits/2026-10-01-seam-crowbar-comma-between-observers.md` (Lemma A; owner/site compatibility; D1–D3; vF readings).
- systemic.engineering `practice/insights/cybernetics/the-algebra-of-non-linear-relating.md` (floor, semiring ladder, §1.6.3, §8.8); `practice/insights/neuroscience/zeroth-order-register.md` (§1, §5.1, §6.3).
- mirror `docs/math/sheaf/` and the ten sheaf-cohomology documents listed in §8.6, read critically.
- prism `prismqueer/src/spectral/kleinos.rs`, read through a reader. It is a graph with no stalks.

### Computations
Session scratchpad `mara-difference/`, numpy 2.4.4, fixed seeds:
- `difference.py` → `difference.out` (CHECK1–10);
- `ci_only.py` (privatisation);
- `contest.py` (Thm 3.4(d));
- `cech.py` → `cech.out` (§8);
- `cech_fix.py` (pairwise commas on K_i ∪ K_j; the merge sign).

The ARCS line in `cech.out` prints "pairwise commas [1, 1, 1]" from a call that computed classes of X, not of K_i ∪ K_j. `cech_fix.py` gives the correct [0, 0, 0]. The directory is ephemeral. Port before it disappears.

**Kagi tally (so far).** 19 external items. Verified, primary text read: 17. Metadata only: 2 (Kramers 1940; Wiener 1949). Not verified: 2 sub-claims (Bateson's page; the Bateson p. 315 variant). Pending: the eight items above.
