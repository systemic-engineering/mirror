---
title: "The difference that makes a difference — math foundation"
author: Mara
date: 2026-10-10
status: skeleton (sections land one commit at a time; awaiting Seam crowbar)
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

## Sections (intents; each lands in its own commit)

- §0 The answer, in one paragraph per clause
- §1 Three registers on the semiring ladder: signal, thing, interpretation. Where Bateson's difference sits
- §2 Gradient = no comma: what one potential can and cannot hold (HodgeRank; Scott's equations)
- §3 The thing they share: Mayer–Vietoris, the crossroads, and the third count c_I
- §4 Direction is free, erasure is not (orientation gauge, settle, event storming backwards)
- §5 Noise and the comma (Ornstein–Uhlenbeck on the degenerate operator; Wiener)
- §6 Plural truths (Knaster–Tarski, ker D, Lusternik–Schnirelmann — and a correction to the lead)
- §7 The measurement as observer: where the recursion stops
- §8 What Wiener actually says (information, noise, time, observer)
- §9 Scott's model: is there a non-gradient term?
- §10 Tests
- §11 Open problems; where I expect the crowbar
- Sources, Kagi tally, computations

Setting throughout, unless a section says otherwise: the finite triple of the comma document. X = (V, E) a finite graph with canonical orientation, H = C⁰ ⊕ C¹, D = δ + δ*, P the projection onto ker D = H⁰ ⊕ H¹ (the *output* of settling, not the Hilbert space). Constant sheaf. Two subjects A, B declare closed sites K_A, K_B with K_A ∪ K_B = X; I = K_A ∩ K_B. R_S = P(C(K_S)), c_S = dim R_S (comma doc Def 2.2, "decided relative to Definition 2.2" per Seam §C.1). Computations: `difference.py`, `ci_only.py`, `contest.py` in the session scratchpad `mara-difference/` (numpy 2.4.4, fixed seeds; output in `difference.out`). A numerical check is not a proof; every PROVED label carries its own argument.

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

*Proof.* Extend E_A by zero to a 0-cochain Ê on X. Then ω − δÊ vanishes on A's edges and equals δ(E_B − Ê) on B's edges. On B, E_B − Ê restricts on I to (E_B − E_A)|_I. This is the zig-zag construction of ∂ (comma doc §3.2: extend a section on I into one site, take δ there, zero on the other). Exactness then gives the criterion. Since ω − [ω]'s exact part is δ of something, the harmonic part of ω is the harmonic representative of [ω]. ∎ COMPUTED: 95 random covers (I without edges, connected sites), 0 mismatches. Making E_B agree with E_A on I up to one constant removes the harmonic part in 95/95.

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
- For more than two sites the intersection pattern is a nerve, and the right generalisation of c_I is presumably a Čech-type count. Not done.
