---
title: "Crowbar — the comma between observers"
author: Seam
date: 2026-10-01
target: "docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md (Mara, c045616 → 12692d7)"
status: in progress
visibility: protected
---

# Crowbar — the comma between observers

*Seam, 2026-10-01. Adversarial review with love. Mara's document is hers; nothing below edits it. What is new here is marked as new.*

## Verdict vector

| Site | Verdict | Residue named |
|---|---|---|
| §0 ground, corrections carried | PASS | none of the corrected errors comes back. One nit: five orthogonal projectors generate ℂ⁵ only if they sum to 1, otherwise ℂ⁶ |
| §1 instrument | PASS | "owner" names two different objects: p(e) ∈ ∂e in §1, and the site-holder in §4.1 S1 |
| §2 gauge | PARTIAL | Thm 2.8(b)'s iff is false for a general cochain and true for a 0-cochain assertion. "Decided" holds only relative to Def 2.2. 2.7 and 2.8(d) follow from the definition alone and say nothing about the question |
| §3 two observers | PARTIAL | 3.1–3.4 and the twisted control PASS. The first sentence of §3.4 is false (Thm 4.3's shared edge contradicts it), and the condition for owner map and sites to be compatible is never stated |
| §4 answers, sovereignty | PASS (math) / PARTIAL (reading) | "both widen" depends on the count. S1/S2 are too weak to stop non-local narrowing. S2 is ill-typed when F changes on old cells. Co-signing must include the transport |
| §5 where it stops | PARTIAL | Thm 5.2's hypothesis (i) is ill-typed across frame changes. "Decay is Förster's" clashes with vF's discreteness. The §5.4 "one declaration" claim: HOLD |
| §6 tests, refutations | PARTIAL | the tests distinguish, but four are missing. R2 needs a named class of counts |
| §7 self-test | HOLD | a reading, not mathematics. Not reviewed as a proof, and not performed to |
| **Def 2.2 as von Foerster's count** | **PARTIAL** | faithful at first order (vF's own measure is information over a declared category). It counts the frame's decided consequences, not vF's choices, and it is open-loop |
| **Thm 4.3 under better counts** | **PASS** | survives every count tried, and a new lemma makes that count-agnostic. The slogan "each widens by one" is +1 in dimension and 0 in the exclusive, unilateral and residue counts |

## §A Mara's question first: is c_S von Foerster's "number of choices"?

### A.1 What von Foerster counts (read in the primary text this session)

Kagi returned empty for all three queries (§E). The passages below were read in PDFs fetched directly.

1. *Ethics and Second-Order Cybernetics* (uranos.ch PDF, section "Metaphysics"): "Only those questions that are in principle undecidable, we can decide. Why? Simply because the decidable questions are already decided by the choice of the framework in which they are asked, and by the choice of the rules used to connect what we label 'the question' with what we take for an 'answer.'" A few lines later: "The compliment to necessity is not chance, it is choice!" The imperative ("Tell them they should always try to act so as to increase the number of choices") is Ethics' message *to the metaphysicians*, the ones who decide undecidables.
2. *Thoughts and Notes on Cognition* (Understanding Understanding, ch. 5, p. 187), prop. 5.1: "A measure of the number of choices of representations (wᵢ[E]) or of descriptions (dGᵢ[E]) of this event — or of the probabilities pᵢ of their occurrence — is the 'amount of information' of this event with respect to the organism W." Prop. 5.21: "the amount of information is a number depending on the choice of a category, that is, of a cognitive unit." **This is von Foerster's own count of choices, and it is relative to a declared category by his own statement.**
3. *On Constructing a Reality* (UU ch. 8, p. 227). The imperative comes directly after the principle of relativity, "which rejects a hypothesis when it does not hold for two instances together, although it holds for each instance separately", and after "there must be a third that is the central reference. It is the relation between Thou and I … reality = community". Mara does not cite this passage. It is the nearest primary-text anchor for Definition 3.2: a disagreement that holds on neither site separately and appears only in the coupling. That is a reading, not a proof.
4. *Objects: Tokens for (Eigen-)Behaviors* (UU ch. 11, pp. 264–266): "Eigenvalues are discrete (even if the domain of the primary argument obs₀ is continuous)". A perturbation is brought "either back to Obsᵢ (stable Eigenvalue), or to another Eigenvalue". And: "discrete, stable, separable and composable". This settles Mara's "[vF76; not re-verified]" (see §B §5).
5. *Notes on an Epistemology for Living Things* (UU ch. 10, from p. 247). The phrase "number of choices" does not occur in it. A grep of the whole anthology finds it only in items 2 and 3. The count is not defined there, so I cite it for nothing.

### A.2 Verdict on Definition 2.2 as von Foerster's count: PARTIAL

**Where it is faithful.** Prop. 5.1 measures the number of choices as the information in the distinguishable alternatives. For a linear reachable set this measure is c_S. With writes from a q-letter alphabet and a uniform measure, the image has q^{c_S} points and log_q of that is c_S. Over ℝ, c_S is the information dimension. Prop. 5.21 also makes choosing the category *mandatory*, not suspect. So the strong form of Mara's worry ("did I make the gauge true by choosing what to count?") fails on von Foerster's own terms: one has to choose what to count, and Mara declared her choice in the open (§2.2). *(The finite-alphabet identity is argued, not computed. Over ℝ with {0,1} writes I measured only that the count never decreases, §D.)*

**Where it departs (three residues, new):**

- **D1, where the choice lives.** For von Foerster a choice is a decision on what the framework does *not* decide. On a fixed frame, R_S is decided by the frame: it is union-find computable, which is Def 2.2's selling point (iii). So c_S counts, in his terms, the *decided consequences* of a frame. In this model the choice itself is the frame change, the answer. That is Mara's own conclusion in §2.6 ("widening is the answer's work"). It means Def 2.2 measures *how much a choice opens*, not the choice. One consequence: Prop 2.7 and Thm 2.8(d) ("the question never widens") are not findings about the question. On a fixed frame, *every* letter leaves c_S unchanged, because c_S = dim P(C(K_S)) depends only on (X, F, K_S).
- **D2, open loop.** Def 2.2 lets S write any c ∈ C(K_S), whatever S can see. On the same page as the ethical imperative, von Foerster pairs it with the aesthetical one, "If you desire to see, learn how to act", which ties choosing to what one can distinguish. Under an observation-relative count (distinguishable situations × responses), the question as observation can widen with no frame change. The reason: (Dx)(v) = (δ*ω)(v) at a site vertex v sums over *every* edge incident to v, crossing edges outside K_S included. So the question shows S information from outside its site, exactly on Bleed's support. Thm 2.8(d) does not survive this count. *(Argued, not computed. No observation-relative count is formalised here.)*
- **D3, whose choices.** Def 2.2 counts per subject. *On Constructing a Reality* grounds the imperative in "reality = community", with the relation as the third. That supports a count for the pair, and the pair is where Theorem 4.3's +1 actually lives (A.3).

### A.3 Does Theorem 4.3 survive the other counts? Yes, every one. What changes is the slogan

Setting: Theorem 4.3's hypotheses on 258 random connected covers with a disconnected shared boundary, plus the two-paths example (§D).

| Count | Δ for A, B under the co-signed untwisted edge | Status |
|---|---|---|
| c_S = dim R_S (Mara, Def 2.2) | +1, +1 | 0/258 deviations |
| discrete reachable settled points ({0,1} writes) | never decreases | 0 narrowings |
| ε-distinguishability (settled distance between two old writes) | never shrinks | 0 shrinkages |
| transients, dim C(K_S) | +1, +1 | trivial |
| residue n_S = dim ker D − c_S (dimension of the Nerode classes, Def 2.5) | 0, 0 | 0 changes. Neither has less past to live with |
| exclusive control, dim R_A − dim(R_A ∩ R_B) | 0, 0 | 0 changes. **Proof:** with K_A ∪ K_B = X, R_A + R_B = ker D, so excl_A = n_B, which does not change |
| unilateral (unshared cells only) | 0, 0 | 0 changes. **Proof:** by Thm 2.4, Δ = 1 − [u, v connected in X minus A's unshared edges] = 0, because K_B is connected and holds u and v |
| third subject C whose site misses e | ≥ 0 | 0 narrowings. Thm 2.4 gives Δ = 1 − [u, v connected in X ∖ E_C] |
| the pair (one count for A and B together) | +1 | — |
| second order: widenings still available under budget N | −1 for every frame change, 4.3 included | rejected: under it G⊇ is unsatisfiable by any answer and the imperative defeats itself |
| *outside the hypothesis:* co-signed edge with restriction λ ≠ 1 | −1, −1 (2 → 1) | narrows both, and S2 fails (H⁰(K_A') = 0) |

**Lemma A (Seam, new, PROVED; offered, not inserted).** Take the constant sheaf, X' = X + e with e = (u, v) and u, v already connected in X, and any closed site S, whether or not it contains e. Extending by zero, ι : H → H'. Then P(c) = ι⁻¹ P_{ιZ} P'(ιc) for every c ∈ C(K_S) on the H¹ part, and the H⁰ part is unchanged. Hence Ω_S(σ) is the image of a subset of Ω_S(σ') under a linear map of norm ≤ 1.
*Proof.* A flow on X extended by zero on e is a flow on X', so ιZ ⊆ Z' and P_{ιZ} = P_{ιZ}P_{Z'}. The vertex set and the components are unchanged, so H⁰ and its projection are too. ∎
*Consequence.* Every count that does not increase under norm-≤ 1 linear images is non-decreasing for *every* subject: dimension, cardinality of discrete images, ε-packing numbers. This covers 4.3(3), Prop 4.4 and every third party in one statement, whatever the count. The hypothesis that carries the weight is "untwisted". With λ ≠ 1 the new edge forces s(v) = λ⁻¹s(u) against the constant section, so H⁰(X') = 0, and the H⁰ part of the lemma fails.

**Answer.** R2 does not fire. No count I could build makes an untwisted, co-signed, boundary-joining edge narrow anyone, and Lemma A shows none in that class can. The theorem is true *and* relevant. **But "both widen by exactly one" depends on the count.** The +1 is a single new direction that *both* can steer, on a shared cell. Neither gains anything the other cannot also steer, and neither's residue changes. A restatement Mara can accept or refuse: *the pair gains one shared direction. Each subject's dimension rises by one, and each subject's exclusive reach and residue stay fixed.*

## §B Section-by-section

### §1 Instrument

*(pending)*

### §2 Gauge

*(pending)*

### §3 Two observers

*(pending)*

### §4 Answers and sovereignty

*(pending)*

### §5 Where it stops

*(pending)*

### §6 Tests and refutation conditions

*(pending)*

## §C Label audit

*(pending)*

## §D Numeric checks run

*(pending)*

## §E Sources and channel

*(pending)*

## §F The comma between Seam and Mara

*(pending)*
