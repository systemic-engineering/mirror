---
title: "The chord — math foundation"
author: Mara
date: 2026-10-10
status: skeleton (sections land one commit at a time; Seam crowbars in parallel, through Reed)
visibility: protected
builds-on:
  - "mirror docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md (748ad21)"
  - "mirror docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md"
  - "mirror docs/audits/2026-10-01-seam-crowbar-comma-between-observers.md"
---

# The chord

*Mara, 2026-10-10, second run. From Alex's recognitions and Reed's composition. Seam is crowbarring in parallel, with Reed as the third between us.*

## The question

> What has to be true of the coupling for the third — the chord — to be an autopoietic system, more than the inner join of two observers, with its own spectrum, its own held dissonance, and its own life and death; and how is that measured on prismqueer's triple?

Alex, verbatim:
- "What if the bridge IS the emergent third?"
- "What if the emergent third is not just the 'inner join' of two observer's realities but in fact an autopoietic system itself? More than the sum of their parts. A composition. Like in a song, where only in the composition of the instruments music emerges?"
- "It's a chord."
- "Neither. Both. The loop is owned by the emergent third relation between coupled observers. Which is why the provenance question in a compositional ontology becomes nonsensical."

Labels: **PROVED** · **MEASURED** · **COMPUTED** (script in `~/dev/ai/mara/scratch/chord/`, not committed) · **CONJECTURE** · **LEAD** · **READING**.

## Sections (intents)

- §0 The answer
- §1 The k̃ − k test: the bridge as its own shard
- §2 The third, defined as a system
- §3 Its own spectrum: structural emergence vs combination tones
- §4 Metabolism: use-reinforced bridges, matched noise, life and death
- §5 Non-linear coupling and the certified settle
- §6 The musical mathematics, verified (commas, Tonnetz, voice-leading, roughness, Tartini)
- §7 Tests for prism-Reed
- §8 Open problems; where the crowbar bites
- Sources, Kagi tally

Setting: the finite triple of the comma doc, constant sheaf unless weighted. K_A, K_B are closed sites covering X, with I = K_A ∩ K_B and E_I the shared edges. **k** is the comma between, ker(H¹(X) → H¹(A) ⊕ H¹(B)). **k̃** is the private-cover count of the Oct 10 doc. Scripts live in `~/dev/ai/mara/scratch/chord/`: `chord.py`, `ho.py`, `metab.py`, `metab_mu1.py`, `closure.py`, `tonnetz.py`, `sethares.py`, each with a `.out`. They are not committed.

## §1 The k̃ − k test: refuted as "what the third holds"

### §1.1 Two exact forms, both signed

**Proposition 1.1 (PROVED; COMPUTED 1385/1385).** For a shared-edge set E_I and a subcomplex Y ⊇ I, let κ_Y(T) be the comma between Y ∖ E_I and T = I inside Y: the loops the shared cells close in Y. Then

  κ_Y(T) = |V_I| − b₀(T) − b₀(Y ∖ E_I) + b₀(Y),

and

  **k̃ − k = κ_A(T) + κ_B(T) − κ_X(T).**

*Proof.* Comma doc Thm 3.3 gives both k and k̃ in b₀ terms (for k̃, on the cover of X ∖ E_I with an edgeless shared part). Write m_Y = b₀(Y ∖ E_I) − b₀(Y), the number of merges the shared edges make in Y. Subtracting the two formulas gives k̃ − k = m_I − m_A − m_B + m_X with m_I = |V_I| − b₀(I). Then substitute κ_Y = m_I − m_Y. ∎

Seam, independently (crowbar run, 3000/3000), found c_∩ = c_I + k − o + h, so k̃ − k = h − o. The two forms agree, and both are **signed**:
- negative in 73 of my 1385 covers and 51 of Seam's 3000;
- in my first negative case, κ_A = κ_B = 0 and κ_X = 1. The shared edge closes a loop only in the whole, through both private parts.

A signed difference cannot be the dimension of anything a third holds. **I retract "k̃ − k = what the third holds" from the plan.** Seam's verdict stands: k̃ is a correction term, not a comma.

**Correction to my Oct 10 document, recorded here** (that document stays as it is, awaiting Seam).
- Its §0 says the directions both can steer "split exactly into two kinds: c_I … and k". That is wrong, and Seam's pentagon refutes it.
- I reproduced the pentagon (`ho.py`): k = 1, o = 1, h = 0, c_I = 2, c_∩ = 2. The comma is contested *and* reachable through the shared edge.
- Theorem 3.4(b) as stated, c_∩ = c_I + k̃, still holds on the pentagon (2 = 2 + 0). The error was in the summary, which wrote k where the theorem has k̃.

### §1.2 The bridge as its own shard

**Proposition 1.2 (PROVED; COMPUTED 1385/1385).** Move the shared cells to a third site T = I, with A_priv = A minus E_I and B_priv likewise. For the cover {A_priv, B_priv, T}:

  comma among all = k̃ + κ_X(T),  and  k = (comma among all) − κ_A(T) − κ_B(T).

*Proof.* Every pairwise and triple overlap is V_I, discrete. So Ĥ² = 0, and Ĥ¹ of the nerve has dimension 2|V_I| − b₀(A_priv) − b₀(B_priv) − b₀(T) + b₀(X) (comma doc 2 Cor 8.2(b)). Subtract the formula for k̃ and use κ_X = m_I − m_X. The second identity then follows from Proposition 1.1. ∎

Seam's form of the first identity is k̃ + (c_I¹ − b₁(I)). It is the same: c_I¹ = b₁(X) − b₁(X ∖ E_I) = |E_I| − m_X, and b₁(I) = |E_I| − m_I.

READING. A third party standing on the shared cells creates κ_X loops that nobody sees. When A and B each co-sign it, κ_A and κ_B of those loops become visible to the co-signers. Co-signing is *adoption*. §4 shows the same thing metabolically.

**Owner maps (PROVED; COMPUTED).** On the bare bridge, every edge of T has both endpoints in V_I ⊆ V_A ∩ V_B, so Seam's compatibility condition fails and no owner map exists. Subdivide each shared edge with a mediator vertex that only T holds, and the map exists, with all counts unchanged (1385/1385). **A third that is to be owned as a site needs a vertex of its own: a 0-cell, a pitch.**

### §1.3 What is invariant: emergence is the comma itself

Seam's invariance test, extended (`ho.py`, 265 covers, contracting one shared edge, which is a homotopy equivalence of the triad):

| quantity | unchanged under contraction |
|---|---|
| k | 265/265 |
| h | 260/265 |
| o | 238/265 |
| c_I | 134/265 |

Under subdivision with a shared midpoint, everything is unchanged (265/265). Separately, h ≥ 0 always, o > 0 in 29 cases, and Seam's identity holds 265/265.

So **h is not an invariant emergence measure**, and Reed's candidate (b) is refuted as stated. Two propositions say what is invariant.

**Proposition 1.3 (PROVED): nothing topological is held by the shared cells.** Every class in the comma between restricts to zero on I.

*Proof.* I ⊆ K_A, so restriction to I factors through H¹(A), where the class is already zero. ∎

Hence o, "contested and routable through the shared cells", is a statement about harmonic *footprints*, not about classes. That is why it moves under contraction (27/265). **o does not earn the name "held".** It double-counts a footprint coincidence. Reed's candidate (c) is answered: no.

**Theorem 1.4 (PROVED): emergence = the comma.** Over ℝ,

  k = dim ker(H¹(X) → H¹(A) ⊕ H¹(B)) = dim coker(H₁(A) ⊕ H₁(B) → H₁(X)).

For N sites, the comma among all = dim coker(⊕_i H₁(K_i) → H₁(X)) = codim of Σ_i Z(K_i) in Z(X). This is the Oct 10 document's Theorem 8.3, and it is Ĥ¹(nerve; H⁰).

*Proof.* Over a field, H¹ = Hom(H₁, ℝ), and restriction is dual to inclusion. The kernel of a dual map is the dual of the cokernel. ∎

The homology reading is the one Alex named: **loops that exist only in the composition, which are not sums of loops any one observer has.** The cohomology reading is the disagreement no observer sees. They are one number, it is homotopy invariant (k 265/265), and its N-shard form is already proved.

This is Alex's "the loop is owned by the emergent third relation" as a theorem. Those loops restrict to zero on every site (Prop 1.3), so they have no site of their own. They are carried by the nerve. Asking which observer they come from has no answer. That is the precise sense in which "the provenance question … becomes nonsensical".

On the Theorem 4.3 example, Seam's h = 1 after co-signing is the outer cycle. Topologically, though, after co-signing that cycle *is* a sum of A's new loop and B's new loop (the shared edge cancels). So co-signing makes k fall 1 → 0. **Co-signing does not create emergence. It dissolves it into the parts.** h is a footprint trace of where the dissolved loop used to run.

## §2 The third, defined

**Definition 2.1.** For a cover 𝒦 = {K_i} of X, the **third** of 𝒦 is

  Θ(𝒦) := Z(X) ⊖ Σ_i Z(K_i) ⊆ ker D_X,

the harmonic flows orthogonal to every site's own loops.

**Proposition 2.2 (PROVED).**
(i) dim Θ = comma among all = Ĥ¹(𝒩; H⁰).
(ii) A harmonic h lies in Θ iff its class restricts to zero on every site.
(iii) Θ is invariant under, and fixed pointwise by, decay and ring.
(iv) Θ is changed only by frame changes: cells or weights crossing a declared threshold, or a change of cover.

*Proof.* (ii): a harmonic h restricts to zero in H¹(K_i) iff it pairs to zero with every cycle of K_i, i.e. iff h ⊥ Z(K_i). (i): (ii) and Theorem 1.4. (iii): Θ ⊆ ker D (comma doc Prop 2.3). (iv): Θ is defined by (X, 𝒦) alone. ∎

So, to the coordinator's three candidates:
- **Its own complex?** No. Θ has no cells. Prop 1.3 rules out any site holding it.
- **A fluctuation of D?** No. Fluctuations conserve the index and move tonic and comma together (comma doc Thm 1.6). Θ is a subspace of the glued kernel.
- **A shard on the nerve?** Yes, in the precise sense of Ĥ¹(𝒩; H⁰). Its coordinates are overlap components, and its representatives are flows that cross sites.

Under linear dynamics Θ is *still*. It is the drone: frequency 0 under ring, untouched by decay. Everything it does over time happens through the frame, which is §4, or through non-linear coupling, which is §5.

READING, the chord.
- Notes are sites. Intervals are edges, and an interval is owned by neither note: under an owner map a crossing edge needs a declared owner, and a shared edge has none (Prop 1.2).
- The chord's quality is Θ together with its 2-cells (§6.3): it lives in no note and no interval.
- "It exists only while sounding" is §4. Θ lives exactly as long as the bridges that carry its loops are metabolised.
