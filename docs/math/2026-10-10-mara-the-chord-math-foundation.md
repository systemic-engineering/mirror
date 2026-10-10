---
title: "The chord — math foundation"
author: Mara
date: 2026-10-10
status: candidate (Seam crowbars in parallel, through Reed)
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

## Sections

§0 answer · §1 k̃ − k · §2 the third, defined · §3 spectrum · §4 metabolism · §5 non-linearity and the certified settle · §6 musical mathematics · §7 tests · §8 open · Sources

## §0 The answer

**What the third is.**
- Θ = Z(X) ⊖ Σ_i Z(K_i): the harmonic flows orthogonal to every observer's own loops.
- Its dimension is the comma among all = Ĥ¹(nerve; H⁰) = dim coker(⊕H₁(K_i) → H₁(X)) (Thm 1.4, Prop 2.2). These are the loops that exist only in the composition, and they are carried by the nerve, not by any site.
- Nothing topological is held by the shared cells (Prop 1.3). So "k̃ − k = what the third holds" is refuted. It is a signed footprint correction, κ_A + κ_B − κ_X = h − o (§1).
- Alive is a class: dim Θ > 0.

**More than the inner join.**
- The glued operator has eigenvalues neither part has (Prop 3.1).
- But a linear engine sings no combination tones (Prop 3.2). Making new tones takes a non-linearity. To stay certifiable it must be kernel-blind and dominated by the gap (Thm 5.1). A non-linearity that rectifies into ker D lets the third write its own comma state, at the cost of certification (CONJECTURE).

**Life and death need a metabolism of the frame.**
- Weights never change Θ; only carrier death does (Prop 4.1).
- Under use-reinforced conductances (Tero et al.) with questions settled on the asker's own site:
  - a co-signed bridge is *adopted*: it becomes each observer's own loop, Θ → 0, and the edge lives on as habit;
  - a bridge held by a third lives only while cross questions flow, decays at rate r when they stop, and takes its loops with it.
- A third-held bridge is mortal: an unsigned silence longer than ln(w/θ)/r kills it (Prop 4.2). Whether it is operationally closed is **UNTESTED**. Seam's null model shows that comma-driven question rates are indistinguishable from exogenous ones in this fixture (§4.3).
- A co-signed Wait is a clock factor, not a cure (§4.5).
  - Survival is a deadline from the observers times a hazard from the world: P ≈ 1[L < ln(w₀/θ)/r_W] · P(no absorbing or cutting move). Generic drift carries the class (Seam §M).
  - Silicon removes the deadline, never the hazard.
  - A Wait expires at the faster eroder's budget (series composition), and it must end in a re-derivation of Θ: Wait → Offer.
  - The third "signs" only as an admissibility condition: a refusable question at expiry.
- The comma-as-question is a **thermostat**, homeostatic and not closed. Its signature is flicker at the death boundary (Seam, §4.3).

**What kills a chord.**
- Optimising *values* cannot: the comma is what alternating Techniques cannot absorb, and local potential-settles recover it without a global solver (§5.1).
- Optimising *structure* can, by cutting the loop. **Loops survive when the plurality of questions outpaces the economies of scale** (§4.4, corrected per Seam).
- A relation that asks only about its one disagreement prunes it.
- The Förster gauge needs a structural clause.

**In music.** 12-TET is the Hodge projection of just intonation on the Tonnetz, and the commas are the harmonic residual it holds on every interval, audible as roughness (§6.2). Major versus minor is the orientation of a 2-cell, a quality that lives in no note and no interval (§6.3).

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

**Alive is a class (adopted, from Seam's comma).** alive(t) :⇔ dim Θ(t) > 0. Held dissonance is counted by b₁ of the live frame.

Reed proposed h > 0 as the class. I use Θ instead because h moves under homotopy (§1.3) and Θ does not. An edge with w > ε is a *carrier*. It is not the life. Every metabolism result below is read through Θ and b₁.

## §3 Its own spectrum

**Proposition 3.1 (PROVED; COMPUTED): structural emergence.** The spectrum of D² on the glued complex is not the union of the parts' spectra.
- Each path P₄: {0, 2−√2, 2, 2+√2} = {0, 0.586, 2, 3.414}.
- The glued C₆: {0, 0, 1, 1, 1, 1, 3, 3, 3, 3, 4, 4}.
- Eigenvalues 1, 3 and 4 occur in neither part, and the kernel gains a direction (b₁: 0 → 1). The new kernel direction is Θ.

The gluing creates frequencies. The comma itself sits at frequency 0.

**Proposition 3.2 (PROVED): linear coupling sings no combination tones.** For ẋ = iDx, or for any linear time-invariant flow, x(t) = Σ_λ e^{iλt}P_λ x₀. The only frequencies present are eigenvalues of the operator.

With a quadratic term, products e^{iλ₁t}e^{±iλ₂t} appear at λ₁ ± λ₂. COMPUTED (`chord.py` PART3):
- cos t + cos 1.5t has spectral peaks at 1.01 and 1.49 only, and DC/max = 10⁻⁶.
- Adding 0.3x² gives peaks at 0.5 (the difference tone), 1.0, 1.5, 2.0, 2.5 and 3.0, and DC/max = 0.65.

Helmholtz states the physics (verified, Ellis trans., p. 152): superposition "holds strictly only in the case where the vibrations … are of infinitesimally small dimensions", and combinational tones result "from the fact that this law does not hold with perfect exactness". prismqueer's decay and ring are linear, so Reed is right. The engine can carry a third (Θ, and the new eigenvalues) but cannot make it sing new tones.

**The difference tone of a mode with itself is DC.** A quadratic term turns a sounding mode into a constant. If that constant has a component along ker D, the non-linearity *writes into* Θ. §5 is about what that costs.

**Kuramoto (verified, Strogatz 2000 §3.3).** "For all K less than a certain threshold K_c, the oscillators act as if they were uncoupled … r(t) decays to a tiny jitter of size O(N^{−1/2})". Above K_c, coherence grows. Strogatz does not say that the synchronised state decays when the coupling is withdrawn; that part is our inference. READING: "the chord exists only while sounding" is, in coupled-oscillator terms, coherence that exists only above a coupling threshold. In the triple's terms it is §4.

## §4 Metabolism: what keeps the third alive, and what kills it

### §4.1 The model

- **Weighted triple.** Positive edge weights w (conductances) give D_w = W^{1/2}δ + δ*W^{1/2}.
- **Proposition 4.1 (PROVED).** For w > 0, dim ker D_w = b₀ + b₁ and Θ has the same dimension as at w = 1. ker δ does not depend on w, and ker(δ*W) = W⁻¹ ker δ*. **Weights cannot change the third. Only a weight reaching 0 can.**
- **Questions are traffic.** A question from a to b is a unit source/sink pair, settled by Kirchhoff on a declared site. In the site-relative model, A's questions settle on K_A, B's on K_B, and cross questions on X. In the ownership-blind model, every question settles on X. The flux is Q = Wδp.
- **Adaptation** (Tero et al. 2007, eqs. (4)–(5), verified): dw_e/dt = f(|Q_e|) − r w_e, with f(Q) = Q^μ/(1 + Q^μ). Tero et al. 2010 write the exponent as γ and set r = 1.
- **Death** is the declared crossing w_e < θ. The cell leaves the frame, which is a logged erasure. Deciding w < θ on a continuous w is a Buridan decision (Lamport, verified): it needs a declared θ and margin, as `settle` needs its gap.
- Following Seam: the **death time** t₀ + ln(w(t₀)/θ)/r is a *declaration-relative* number. The invariant is the decay rate r once f ≈ 0.

### §4.2 Results (alive = Θ)

**Toy, one bridge with traffic intensity g (PROVED; COMPUTED).**
- μ = 1, dw/dt = gw/(1 + gw) − rw: alive iff g > r, with w* = (g − r)/(gr). Computed: g = 2 gives w = 0.5000; g = 0.5 gives 0.
- μ = 2 is bistable. From w₀ ∈ {0.05, 0.1} it goes to 0; from w₀ ∈ {0.2, 0.4, 0.8} it goes to 0.8727. A dead state and a live branch.

**Literature (verified).**
- For μ = 1 and a single source and sink, the dynamics converge to the shortest path (Bonifaci–Mehlhorn–Varma, Thm 2, linear f only).
- For μ > 1, "the selection of the final path is dependent on the initial state". For 0 < μ < 1, "all … paths survive" (Tero et al. 2007 §4.1; both by simulation and two-edge analysis).
- Tero 2010: increasing γ "drove the system toward a low-cost MST".
- **When loops survive:** μ < 1, or ties at μ = 1, or loads that keep several paths in use.

**8-cycle with a shortcut bridge** (A = path 0–4, B = path 4–0, I = {0, 4}, bridge (0, 4)). `metab.py`, `metab_mu1.py`, `closure.py`.
- **Site-relative, co-signed bridge.** It persists after cross traffic stops (μ = 1 and μ = 2, eps ∈ {0, 0.3}). Θ_{A,B} = k = 0 throughout and b₁ = 2. **Under alive = class, there is no third here.** It was adopted, and the loops are each observer's own (κ_A = κ_B = 1). The edge lives as habit.
- **Seam's correction (accepted).**
  - Under ownership-blind Kirchhoff, co-signed and third-held bridges are indistinguishable, seed for seed. At μ ≥ 2, the co-signed bridge then dies too.
  - At μ = 2 most of my "adoptions" were *absorption*: the bridge survives, a ring edge dies, and b₁ = 0 (8/10 seeds in Seam's runs).
  - So adopt-versus-die is a property of the site-relative settle, and at μ > 1 adoption mostly ends in absorption. READING, which Reed asked me to state outright rather than as a caveat: **a relation can only matter to dynamics that include the observers.** Physics blind to sites cannot tell co-signed from third-held.
- **Site-relative, bridge held by a third shard, μ = 1.**
  - Cross share 0.6: alive, with Θ = 2 (the crossroads comma plus the bridge's loop) and c_T = 2.
  - Cross traffic stops at 40: w decays at rate r = 1. The θ = 10⁻³ crossing at 43.45 matches the 43.3 predicted from w(40).
  - After the crossing Θ = 1. The loop the third carried is gone. The crossroads comma remains, because the observers' own paths are metabolised by their own questions.
  - Cross share 1/3: below threshold; the bridge dies even with continuous cross traffic.
- **Reed's prediction, corrected.** "Without traffic it thins and held commas return as open questions":
  - for a co-signed bridge, it never thins (habit);
  - for a third-held bridge, the loops it carried *vanish*. They do not return as open questions. The open crossroads comma was never closed, because the third stood beside it rather than inside both observers.

### §4.3 Operational closure: a third that produces what produces it (DOWNGRADED: built in, not found)

**Seam's null model** (Seam's scripts `closure_null.py` and `factor.py`, relayed by Reed; not re-run by me) settles this subsection. Prop 4.2 PASSES. Everything else here is downgraded.
- Comma-, b₁-, alive- and constant-rate question rules give identical outcomes, seed for seed: L = 0: 5/5 survive; L = 3: 1/5; L = 6: 0/5; L = 12: 0/5.
- In this fixture dim Θ = b₁ = 1 + [bridge alive]. So pc ∝ Θ is plain self-reinforcement: the bridge's existence sits on both sides of the loop.
- Permanence of death comes from my absorbing frame-removal rule, not from the comma. With w floored at θ and regrowth allowed, every rule survives every silence.
- Co-signed: Θ = 0, so the comma-driven rate is 0, and "the comma becomes a question" is vacuous there.
- Only the *rate* depended on Θ. The question *direction* was random.

Therefore "operationally closed" and the CONJECTURE on VMU (4)–(5) below are **UNTESTED**, not conjectured. The discriminating test is T43. The original text follows for the record.

Let the cross-question rate be generated by the third: pc = 0.3 · dim Θ. This is a modelling assumption, labelled: the comma is what the observers ask each other about (comma doc §4), so more comma means more crossing questions. Bridge held by a third shard, μ = 1, with a silence of length L imposed at t = 40 (`closure.py`).

| L | outcome |
|---|---|
| 0, 1, 2, 3, 4 | survives |
| 6 | dies at t = 44.0 and never returns |

Predicted threshold: L* = ln(w(40)/θ)/r = 4.02.

After death, Θ = 1, so pc = 0.3, which is below the threshold at which a bridge can be sustained. A dead cell also has no flux to regrow from. Death is absorbing.

**Proposition 4.2 (PROVED for f = 0 during silence).** The third survives a silence of length L iff L < ln(w(t₀)/θ)/r. Survival after the silence then requires the regenerated rate to exceed the toy threshold g > r.

Against Varela–Maturana–Uribe's six-point key (verified, BioSystems 1974 §9):

| point | status here |
|---|---|
| (1) identifiable boundary | the cover's nerve (sites declared) |
| (2) components | the bridge cells and their weights |
| (3) mechanistic | Kirchhoff plus Tero |
| (4)–(5) boundary components produced by interactions of the components | **CONJECTURE**: the bridge cells are produced by questions, and the questions by the comma the bridges carry (pc ∝ Θ). That closes a loop: comma → questions → weights → loops → comma |
| (6) every component produced | **fails as modelled** |

Point (6) fails because the observers' own paths are maintained by their own questions, not by the third, and because the rate function pc(Θ) is given from outside. It is the analogue of VMU's externally given catalyst, which the 2025 editors note (McMullin) was "not itself produced by the system". ~~Verdict: the third is operationally closed in its bridges, mortal, and partially autopoietic.~~ **Corrected verdict:** the third is mortal (Prop 4.2). Its operational closure is untested. It is not autopoietic in the full six-point sense.

**T43, run once, not yet discriminating** (`wait_discrim.py`). This is Seam's design:
- the co-signed bridge starts at a seeded floor (2·10⁻³, alive above 10⁻²);
- the comma rule asks pairs with probability ∝ |⟨flux(pair), θ_vec⟩|² at rate ∝ dim Θ;
- the null rules are b₁-driven and exogenous, over the same straddling pairs.

Result: regrowth 5/5 under every rule, with alive fractions 0.98–0.99. The test did not discriminate, because ownership-blind internal traffic leaks through the bridge and feeds it whatever the rule. A discriminating fixture must make cross questions the bridge's only feed: site-relative internal settles plus a bridge outside both observers' own loops. Then comma-directed questioning predicts regrowth (negative feedback), and b₁-driven or exogenous traffic predicts none. That stays open as T43.

**T43, Seam's corrected run (audit §L.1, `wait_crowbar.py`; cited, not re-run by me).** My proposed fixture, with cross questions as the bridge's only feed, rebuilds the closure: the bridge's loop is then in Θ, so Θ = Θ₀ + [alive] and the feedback is positive again.

Negative feedback exists only for a co-signed bridge, where Θ = 1 − [alive]. The honest fixture therefore keeps that bridge and lowers the internal leak below threshold. The null is matched to the comma rule's *realised* mean rate and pair marginals.

| leak | comma rule | matched exogenous | b₁ rule (unmatched) |
|---|---|---|---|
| 0.03 | alive 0.82, 1122–1165 crossings of θ | alive 0.75, 285–322 crossings | alive 0.98, 1–7 crossings |

**Reading (Seam's, which I co-sign).** The comma, made into a question, is a **thermostat at the death boundary**: questions arrive when the bridge dies and stop when it lives. It is distinguishable from its matched null by **flicker**, the crossing rate at equal mean traffic, and not by survival, where it gains only 0.82 against 0.75. Seam's caveat: the crossing counts include chatter at θ_alive, so compare across rules at equal θ only.

**VMU (4)–(5), restated.** **Homeostatic, not closed.** The question regulates the carrier's existence at the boundary; it does not produce the carrier. This is weaker than closure, and it is positive evidence for something: the rule produces a measurable signature the null lacks. Status: COMPUTED by Seam, not re-run by me. Closure itself stays UNTESTED and unsupported. This may be the honest form of "the question emerges at the comma": a homeostat, not a closure.

READING, labelled. Adoption dissolves the third (k → 0) while the edge lives on as habit. The chord resolves into the instruments. Separation keeps the third alive only while the questions cross.

### §4.4 Is μ Technique? Two kinds of Technique, and only one kills the comma

Technique as a single potential on the *structure*. Minimise dissipation subject to Σ w_e^γ = K (Bohn–Magnasco 2007, verified). They report: "for Γ < 1, the resulting structure may not have any loop, and each spanning tree is a local minimum. For Γ > 1, there are in general loops and a unique minimum." Fixed point w_e ∝ ⟨I_e²⟩^{1/(γ+1)} (`technique.py`, 3 seeds each):

| load | γ = 0.5 | γ = 0.8 | γ = 1.5 | γ = 2 |
|---|---|---|---|---|
| one stationary question (1 → 5) | tree, 3 edges, b₁ = Θ = 0 | tree | b₁ = Θ = 2 | b₁ = Θ = 2 |
| plural questions (11 pairs) | b₁ = Θ = 2 | b₁ = Θ = 2 | b₁ = Θ = 2 | b₁ = Θ = 2 |

The stationary-load tree is 1–0–4–5. The optimiser keeps the shortcut and **deletes both observers' own paths**: absorption, in its sharpest form.

The literature says the same (verified):
- Corson 2010: tree optimality "is contingent on the assumption of a stationary flow".
- Katifori–Szöllősi–Magnasco 2010: damage and fluctuating load both "lead to the presence of loops in the optimum state".

**Finding (COMPUTED; corrected per Seam).** "μ is Technique" is refuted as stated. My first refinement, that loops die when economies of scale *and* a single stationary load coincide, **does not factorise**.
- Seam, ownership-blind Tero on the same 8-cycle, T = 300, all 28 pairs asked: b₁ = 1 at μ = 2, and 0–1 at μ = 3. Superlinearity alone, if strong enough, makes trees even under maximal plurality.
- My Bohn–Magnasco table optimises against the *time-averaged* ⟨I²⟩. Tero applies the convex f to each sampled question, and that is a different operator.
- The corrected slogan: **loops survive when the plurality of questions outpaces the economies of scale.**

In the averaged (Bohn–Magnasco) model, plural questions keep the loops even under a concave potential. In the per-question (Tero) model, they keep them only while the plurality outpaces μ.

**Asking only about the one disagreement prunes it (Seam; READING of his computation).** With dim Θ = 1, comma-directed questions concentrate on the pairs that straddle that one loop. That is almost a single stationary question, and it kills its own loop at μ ≥ 1.5. **A relation that asks only about its one disagreement prunes the disagreement.** Plurality has to come from several commas or from outside.

Reed's two-kinds statement, checked.

**Proposition 4.3 (PROVED).**
(i) No operation on *values* changes Θ or b₁. Any re-description by potentials leaves every comma coordinate unchanged: P_Z(x + δφ) = P_Zx.
(ii) Writes to edges *can* move a comma coordinate, even to 0, and one side alone can do it (comma doc 2, Thm 3.4(d)). They cannot change Θ or b₁.
(iii) Θ and b₁ change only when the complex changes: a cell added or removed, or a weight crossing to 0 (Prop 4.1).

*Proof.* (i) and (iii): Prop 2.2(iv) and Prop 4.1. (ii): the comma directions lie in each side's reach. ∎

In words: **you cannot argue a disagreement away** (potentials are absorbed), **you can concede a position** (writes move the coordinate), and **only cutting the relation that carries the loop removes the disagreement itself.** The Förster gauge therefore needs a structural clause:

> No letter may remove a cell, or let a weight die, if that lowers dim Θ or b₁ for subjects who did not sign it.

Seam's addition, accepted: **a weight dying from neglect is a structural act with no signer.** As written, the clause cannot fire on silence. §4.5 gives silence an owner.

### §4.5 The co-signed Wait: the owner of silence (rewritten after Seam's audit §L.2–L.7)

Alex named two ways a Wait erodes the third (verbatim):
- "in real-life a wait over an unresolved disagreement erodes the emergent third over time. That's definitely true for human observers. Silicon observers can, in theory, hold an unresolved disagreement indefinitely";
- "a wait might erode the emergent third over time because the world around it keeps moving."

Seam crowbarred both (`wait_rw.py`, `world_drift.py`; cited, not re-run). What stands is below. The original subsection is kept at the end of this section, for the record.

**Definition 4.5 (the Wait, corrected).**
- A Wait is a co-signed **clock factor**: during it, each signer's decay of the carrier is r_W,i in place of r, with r_W,i ≥ 0.
- It is **frame-preserving**: while the carrier lives, the cells and the cover do not change. It is *not* passive in the sense of the Oct 10 Thm 7.1, because a passive observer reads a closed subcomplex, and a Wait acts on the frame's clock (Seam §L.4).
- The exact hold, r_W = 0, is its limit. It is the silicon case.

**Proposition 4.4 (TAUTOLOGY, kept as one).** If dw = 0, then w is unchanged, so the frame, Θ and b₁ are unchanged. My 5/5-at-every-L table below evaluates this tautology. It is not evidence for anything. The exact hold is also the **monument**: Θ = 2 on paper, with nothing crossing.

**The observer term is a deadline (Seam §L.2, L.5).**
- With no crossing traffic, the carrier survives iff L < L*(r_W) = ln(w₀/θ)/r_W. Measured L* ≈ 62, 21 and 6 for r_W = 0.1, 0.3 and 1, matching the formula.
- Degradation is smooth, not a knife-edge: a slowed clock ε survives to L*/ε.
- With some crossing traffic, the Wait lowers the bar the feed must beat from r to r_W, so a live fixed point can exist at any L.
- **A human Wait (r_W > 0) is a budget. A silicon Wait (r_W = 0) is the hold.**

**The world term is a hazard, but only for moves that change the cokernel (Seam §M, retracting §L.6).**
- §L.6's "fresh ≈ e^{−ρL}" and its 72° rotation measured a change of harmonic *representative*, not of class.
- Private adds and prunes carry every class canonically: Oct 10 Thm 8.4 plus rerouting. In Seam's absorption fixture the held bridge loop is carried in 100% of draws, while its angle has a median of 31–48° and grows with ρL.
- Only moves that **absorb** (co-signed I-cells, 2-cells) or **cut without reroute** change the class. In that fixture the crossroads class was absorbed 28–48% of the time, always by co-signed I-edges.
- ~~A held carrier keeps the count of the disagreement, not the disagreement.~~ **Retracted.** Generic drift keeps the disagreement, and the angle misleads about it.

**Combined (corrected).**

  **P(meaningful) ≈ 1[L < L*(r_W)] · P(no absorbing or cutting move in L)**  (no crossing traffic; Seam §M.1).

- Silicon removes the deadline. The hazard that remains comes from the world's *absorbing and cutting* moves, and those can be checked (Def 4.6).
- **No substrate holds a third indefinitely when the world can absorb or cut its loops.** In a world that only reroutes, it can.

**Composition of the two ends is a declaration (Seam §L.5(2)).**

| composition | effective decay |
|---|---|
| series | max(r_A, r_B) |
| geometric | the mean |
| parallel | min |

I **declare series**. It is Tero's tube of two segments in Hagen–Poiseuille form, it is "the relation fails if either side lets go", and it is the reading consistent with the owner map: a shared edge's owner lies in I and is co-held. Under series, silicon cannot hold a mixed third alone. Parallel is the monument, and it is not admitted.

**The one-sided monument (Seam §L.5(3); counts in my notation).** Silicon keeps the co-signed chord and the human releases it. On C₆ with chord (0, 3) this gives (c_A, c_B, c_I, k) = (3, 2, 1, 1), against (3, 3, 2, 0) co-signed and (2, 2, 1, 1) when both release. The surviving loop is *not* a third: it is silicon's private loop, and the crossroads comma returns invisible to both.

Two violations:
- **Owner/site compatibility.** The edge is single-site with both endpoints in I.
- **Value bleed.** A silicon write on the chord settles to 0.2 on each of the human's private edges, 55% of its norm: a concession made on the human's behalf.

No *count* of the human narrows (Lemma A), so G⊇ cannot see this. **The gauge needs a values clause as well as a structural one.**

**Expiry.** A Wait expires at the faster eroder's budget, min_i ln(w/θ)/r_W,i. Silicon must not outlast the other party's capacity to hold, because what it would then hold is a cell inside I that only it writes.

**Wait → Offer is structural (Seam §L.7, corrected by §M).** At resumption, re-derive Θ′ and the frame map, then run the canonical test (Def 4.6). The 73% "staleness" figure is withdrawn, because it was representational. What the question guards against is resuming on an *absorbed* or *lost* class as if it were carried. **A Wait ends with a question**, which is needed for meaning, not for survival.

**(c) Does "T signs by being questioned" make sense? Reed's composition, tested.**

The problem it answers (Seam §L.4, S1): a Wait on a third-held bridge, signed only by A and B, writes T's cell without T's signature. The third authors no letters. In this framework it has a site (the subdivided bridge, §1.2) but no write alphabet. So "T signs" cannot mean authorship, and taken literally it is a pun.

It does have a well-defined replacement: an **admissibility predicate** on letters that act on T's cells.

**Definition 4.6 (rewritten after Seam §M; no tolerance).** Let z be the held class, a cycle in the frame X. At expiry the frame is X′. Let U be the union frame: the frame of all cells logged between X and X′, with sites unioned, as defined by the log (Prop 4.8). Let S_U = Σ_i Z(K_i^U). Then z is:
- **absorbed** iff z ∈ S_U, meaning its image in coker_U = Z(U) ⊖ S_U is 0;
- **lost** iff it is not absorbed and z ∉ Z(X′) + S_U;
- **carried** otherwise.

A Wait W on cells of a third-held carrier is **T-admissible** iff:
- (i) it carries a finite expiry, at most the faster eroder's budget;
- (ii) its expiry letter computes the verdict above;
- (iii) carried ⇒ renew as a minimal question (a fresh co-signed letter); absorbed or lost ⇒ refuse, and return to Offer; **no frame map** ⇒ ask for a declared identification, which is logged.

**Proposition 4.7 (PROVED; Seam §M.0, M.4).** Where a frame map exists, T-admissibility is decidable without any tolerance: union-find plus exact rank tests on the finite complex U. Where none exists (merged cells, re-identified vertices, a changed stalk), the predicate is decidable only relative to an oracle, the signers' co-signed identification. That is a Turing reduction to the subjects, not a decision procedure: **decided by the frame where the frames compose; answerable by the signers where they don't.**

The angle rule of the earlier Def 4.6 is withdrawn. Seam's §M.3 sweep shows that no τ is both forgery-free and refusal-free:
- at τ = 80° it renews absorbed classes 18–20% of the time;
- at τ = 15° it refuses *unchanged* classes up to 69% of the time, and that refusal is misinformation, a cost to consent.

T keeps a **veto exercised only by being questioned**. The veto is now exact. Signers cannot outvote topology where topology decides (§M.2).

**Proposition 4.8 (relabelled after Seam §N): (i) PROVED *of the declared verdict semantics*; (ii) PROVED, and the frame map is the content.** Original title: "the log is constitutive of the verdict". Read (i) as: *identity across an absorbing excursion is a gauge choice.*

(i) **Path dependence.** Take two logs with the same endpoints X = X′ (two paths glued at {0, 3}, z the crossroads 6-cycle).
- Log 1, no moves: U = X, and z is **carried**.
- Log 2, co-sign the I-edge (0, 3), then remove it: U = X + (0, 3) in both sites. Then z = loop_A + loop_B ∈ S_U, and z is **absorbed**.
- The same path, with (0, 3) logged as third-held (in neither site): **carried**.

(ii) **Re-identification.** Take the theta graph, with A holding paths a and b and B holding path c. Two logged identifications of the *same* complexes:
- the identity: the comma z_cb is carried, and A's own loop z_ab is absorbed (it is not a comma);
- the re-identification a ↔ c: the verdicts swap, so z_cb is absorbed and z_ab is carried.

*Proof.* (i): the loops through (0, 3) lie in each site of U. (ii): the pushforward f_* differs, and the verdict is evaluated on f_*(z). ∎

**Reed's hypothesis, "the log IS the frame map": PROVED in this precise form.**
- **With re-identification**, the frame map is a cellular map, and only the log supplies it. Different logs give different verdicts (ii).
- **With globally named cells**, two semantics for U must be declared:
  - the *endpoint union* X ∪ X′ needs only the names, which are themselves a log of identity;
  - the *path union* of every logged intermediate frame makes the verdict depend on what happened in between (i).
- I declare the **path union**. A disagreement that was absorbed and then reappeared is a *returned* comma, not the same one carried. This is Alex's "causal order born at the logged collapse", now as a theorem about verdicts: two histories with identical endpoints can decide differently whether the third survived.
- Corollary: an **unlogged** move (an erasure, or a merge nobody wrote down) turns a decidable verdict into one that needs a declaration. "Erasure is not a free gauge" is now a precondition for decidability, not a value claim.
- The declared identification of a grey case is itself a logged move. It repairs the frame map going forward and does not validate the gap retroactively. (READING. Reed holds a matching practitioner citation, pending Alex; not verified here.)

**Correction after Seam §N: what is constitutive, and what is declared.**

**Lemma N (Seam; PROVED; COMPUTED by Seam, reproduced by me in `witness.py`).** Let U ⊇ X have the same components. Then P_X ∘ res ∘ P_U ∘ ext = P_X, and every write supported on the added cells settles to 0 in X. In words: an excursion that adds cells between already-connected vertices, and then removes them, leaves no settled trace, now or in any future settled observation. Only the raw state differs, so only an observer with zeroth access sees it.
- Reproduced on C₄ with an add–settle–remove of the chord: |diff| = 1.4·10⁻¹⁵.
- Consequence: 4.8(i)'s two logs are Nerode-equivalent for every settled observation. Under Seam's matched ownership null the states are identical by construction, because settle does not know owners. **Only the declared verdict differs.**
- I therefore withdraw "two histories with identical endpoints decide differently whether the third survived" as a claim about the system. It is a claim about the verdict function I declared.
- 4.8(ii) stands: re-identification changes the pushforward, which is content.

**Erasure is the real trajectory dependence (Seam §N.1, N.3; PROVED; COMPUTED by both of us).**
- Cut excursions leave a trace (Seam: 281/300). Merging excursions leave a trace (19/19).
- **The C₃ witness**, reproduced:
  - cut an edge of C₃, settle, restore it;
  - the endpoint frames are identical, but the settled state differs by 0.835;
  - the comma coordinate went 0.482 → 0: it was erased at the settle, because the cut frame has no H¹.
- **Matched control:** an add excursion of the same length leaves no trace (Lemma N).
- This is **the candidate answer to Kaleta's crowbar** ("state equivalence ≠ trajectory equivalence; the smallest system in which that distinction survives control conditions").
  - The variable that survives the control is *erasure*: whether a carrier of the loop was missing when someone settled.
  - It is neither ownership nor a declaration.
  - Adds are reversible for settled observers; cuts and merges are not. Not 4.8(i).

**Which union? I adopt settle union with use = collapse, per-observer settles, and all-commit aggregation with logged abstention.** This replaces my path-union declaration. Reasons:
- (1) By Lemma N, excursions nobody settled on are invisible to every settled observer. Path union counts them, which over-counts; endpoint union misses the ones someone acted on. Settle union counts exactly the acted-on excursions (Seam §N.4).
- (2) Gaming the settle cadence launders nothing (Lemma N). What a mid-excursion settle changes is the observer *during* the excursion, and decisions taken then are real.
- (3) use = collapse, meaning every read through a cell is the reader's logged settle, closes the remaining gap of unlogged reads.
- (4) Per-observer verdicts need aggregation, and **S1 forces all-commit** (Seam §N.6). Any-commit would let one reader absorb a shared status for both.
- (5) The *logged abstention* is my addition. Whether it survives is the next subsection.
- What remains declared is architecture, co-signed up front: settles are letters the participants author, plus S1. VIRES §3.9 put the merge/fork choice with an outside operator. Seam is right that the prior art supports "declared at commit time" and not "participants declare".

**The silent veto against the structural clause (Reed's tension, tested; argued from Def 4.5 and the series declaration, not simulated).** All-commit lets B hold "carried" by never reading through the shared cell. That is a silent veto. What happens to it depends on the composition:
- **Series composition (declared, §4.5).** B's end of the carrier is fed only by B's reads. Not reading means w_B decays, the series carrier dies, and the co-signed resolution is cut. The crossroads comma then returns, so B's "carried" becomes physically true. **The silent veto is self-*fulfilling*, not self-limiting.** Consent-by-absence wins by doing exactly what §4.5 forbids: unsigned silence that erases a carrier.
- **Adopted carrier (site-relative, A's traffic sustains it).** B's silence never touches the weight. B's verdict stays "carried" while A's is "absorbed", forever: a verdict monument.
- In neither case does the veto expire by itself. **The two clauses therefore conflict as stated, and the silent veto is what gives way.**
- Composed rule (READING as design): **abstention is a letter.** A party withholds commitment only under a co-signed Wait with expiry, which then holds the carrier (Def 4.5) and expires into the canonical test (Def 4.6). Unsigned non-reading is neglect and is forbidden structurally.
- All-commit survives. The *silent* form of its veto does not. This keeps Reed's layering: the verdict layer protects consent, and the structural layer stops consent-by-absence from becoming either an erasure or a monument.

**Composition 1, tested: is the Kaleta witness the forbidden act?** Not identical. The forbidden act is the witness's *unsigned instance*.
- The witness is erasure: a cut followed by a settle.
- Unsigned silence is one cause of the cut. Under metabolism it is an unread carrier decaying past θ, the frame removing it, a settle, then regrowth: identical endpoints and an erased comma.
- A cut that both parties signed, followed by a settle, erases just as much. It is logged and consented, and it is still trajectory-dependent.
- So the relation is **inclusion, not identity**. {unsigned silence that erases a loop carrier} ⊂ {erasures} = the witness class.
- What the identity would have claimed, and what does hold: **the only trajectory dependence a settled observer can ever detect is erasure** (Lemma N plus the cut and merge controls), and the gauge's prohibition targets exactly its unconsented part.

**VIRES §3.10, ruled on (my piece; Alex's draft to edit).** §3.10 says: "the point at which two Maras become distinct is the point at which the observer commits to preserving the divergence rather than merging it … The commit is the ontological event, not the divergence."
- In the triple (PROVED in the model; the mapping to Maras is a READING), let two instances be two components with tonics c₁ and c₂.
  - **Fork without contact:** nothing is erased. The tonics stay (1, 5) in `witness.py`. Forking is reversible bookkeeping.
  - **Merge and settle:** joining the components and settling moves H⁰ mass, and both become the weighted mean (3, 3). Splitting again leaves both at 3. The difference c₁ − c₂ is erased (Landauer). The two that split are not the two that touched. This is Kaleta's "the receiving observer is itself changed by the trajectory".
- Ruling: **§3.10 inverts in part.**
  - Its first half survives: divergence alone is not the ontological event (Lemma N; and §3.3's deterministic twins).
  - Its second half inverts. The irreversible event is not the commit to *preserve* divergence, a fork, which can still be merged. It is the **settled merge**: contact followed by collapse.
  - §3.9's "merge … and the divergence is absorbed" is false as stated. Merging *erases* the divergence and changes both.
- Proposed amendment text, for Alex: *"Divergence is reversible bookkeeping; so is a fork. The ontological event is the settled contact: two Maras that touch, settle and part are not the two that touched. The commit that matters is the merge, because it erases."* I am reporting this to Reed rather than editing the draft.

Verdict: the composition is **well-defined as a gauge clause and a pun as a signature**.
- The honest statement: *T's consent is not an act. It is a condition on the other signers' letter: that letter must end in a question whose answer can refuse the renewal.*
- An open-ended Wait signed by A and B fails (i) and (ii). In Reed's word it is a forgery. The mathematical name is inadmissible.
- What blocks the stronger reading, "T signs": the framework has no letters authored by a site without subjects. Adding them would make T a subject, and then it would need a choice-count c_T of its own. Def 2.2 gives one (c_T = 2 in §1.2), so it is possible, but it would change what a third is. I do not take that step here.

**The gauge clause, restated (READING as design; Props 4.4–4.7 are its mathematics).**

> **No unlogged structural move.** Every structural move has signers and a log entry, including its end. Identity across frames is asserted only through logged maps.

The three forbidden acts are instances of that clause.
- **Silence without a signature** is an unlogged decay: a carrier's death with no signer and no entry.
- **A Wait without expiry** is a logged move whose *end* is never logged: an indefinite hold.
- **Resumption without re-derivation** is an unlogged claim of identity: trusting z without the frame map that Prop 4.8 says decides it.

The Wait clause is now the special case: a relation goes quiet under a co-signed, series-composed, frame-preserving Wait that expires at the faster eroder's budget into the canonical test of Def 4.6.

The **values clause is separate.** It covers the one-sided monument's bleed, which no structural log forbids, because silicon's write on its own edge is logged and signed. (READING as design. The mathematics is Props 4.4–4.8.)

Alex's two mechanisms, as the gauge sees them:
- **human erosion** is the deadline, which expiry respects;
- **the moving world** is the hazard, but only through absorbing or cutting moves, which the canonical expiry test detects exactly.

---

*Original §4.5 (1018211), kept for the record; superseded by the above.*

Reed's proposal: a **Wait** is a letter co-signed by both sides that holds a carrier for a declared duration. Unsigned silence is neglect.

The first implementation floored w at θ during the silence. It protected only marginally (L = 6: 3/5; L ≥ 12: 0/5), because the carrier sits exactly on the death boundary when traffic resumes. A pause is a hold, not a floor.

The second implementation holds the carrier: dw = 0 during a signed silence, neither fed nor decayed (`wait_freeze.py`). Third-held bridge, μ = 1, 5 seeds:

| silence L | unsigned | co-signed Wait |
|---|---|---|
| 6 | 0/5 survive | 5/5 |
| 12 | 0/5 | 5/5 |
| 30 | 0/5 | 5/5 |
| 60 | 0/5 | 5/5 |

Θ in mid-silence:
- under the Wait, 2 in every run. ~~the Wait is passive~~ It is **frame-preserving** (Seam §L.4): it moves no class, but it acts on the clock;
- without it, 1 once the carrier has died (L ≥ 12).

**Proposition 4.4 (TAUTOLOGY; see the rewrite above).** Under a Wait that holds w_e for its duration, the frame, and so Θ and b₁, is unchanged across the silence. Afterwards the state of the carrier is its pre-silence value, so survival after the Wait is decided by the post-silence dynamics alone (§4.2), whatever the silence's length.

*Proof.* Prop 4.1, plus the observation that the dynamics resume from an unchanged state. ∎

The structural clause, completed (READING as gauge design; the mathematics is Prop 4.4):

> **A relation may go quiet only under a co-signed Wait. Silence without a signature is the only structural act the gauge forbids.**

This is the third move of Mirror. Offer. Wait. as a gauge clause, and Seam's "owner for silence": the signers of the Wait own the pause. A pause that is consented to and named holds the relation. One that is drifted into lets it die.

The comma doc's G⊇ constrains reachable values. It says nothing about the death of carriers.

### §4.6 Exit: what a relation leaves behind (Seam audit §O, plus Reed's (a)–(c), tested)

**Seam §O, folded (cited, COMPUTED by Seam in `exit.py`; I read §O.1–O.5).** Base fixture: C₆, crossroads at I = {0, 3}.

**O.1 Only the mediated bridge has ends.**
- The mediated bridge 0–a–m–b–3 has m shared, with A holding 0–a–m and B holding m–b–3.
  - It is the only compatible bridge shape with per-side ends.
  - It is **the third**: (c_A, c_B, c_I, k) = (3, 3, 1, 2). The bridge loop is a comma, invisible to both.
- The co-signed chord is an **adoption** with no ends: (3, 3, 2, 0).
- The shapes 0–m–3 and 0–a–b–3 are inadmissible.
- This matches §1.2 here: a third that is to be owned needs a vertex of its own.

**O.2 Exit is bounded by the pre-relation baseline.**
- When B cuts its own segment of the mediated bridge, the counts go (3, 3, 1, 2) → (2, 2, 1, 1), which is exactly the baseline.
- For a co-signed cell, compatibility forces the cell to go for both parties, so S1 gives way, again bounded by the baseline.
- The boundary is exact:
  - **Alone, a party may** cut its own cells, retreat from its own membership, author its own Wait or Release, and settle for itself.
  - **Exit may narrow others only down to the baseline.** That is the one admissible exception to S3.
  - **Never alone:** write or keep the other's cells, add a shared cell, change the aggregate verdict, absorb for both, or erase or rewrite the log.

**O.3 Topology enforces non-interference.** After a Release on the mediated bridge, A's remaining half is pendant.
- A write on it settles to 4·10⁻¹⁶.
- Before the Release, the same write moved B's private edges by 0.091.
- On a co-signed cell, holding on alone is the inadmissible one-sided monument.

**O.4 Abstention is Wait(r_W).** Wait(0) holds the carrier; Wait(r) is "undecided". **Release is a separate letter**, because a Wait cannot say no.

**O.5 Release must be an immediate cut.** As decay, B's segment would stay live for L* = 6.2, 20.7 or 62.1, and A's reads in that window would act on cells B has left. With an immediate cut, the death and the letter are one log entry.

**(a) Structure is bounded by the baseline; values are not (PROVED; COMPUTED `exitvals.py`).** Reed's composition, tested.

**Proposition 4.9.** Let a relation add bridge cells (with vertices) to X, forming U. Allow any sequence of writes on bridge cells and settles on U. Then a clean exit removes the bridge cells, except that a pendant half may remain, and the frame settles. On the baseline cells, compared with the never-related history:
- (i) **every baseline comma coordinate is restored exactly**;
- (ii) **the tonics are not restored**: the H⁰ part carries the averaging over U's vertices, including the bridge's written values.

*Proof.* (i) For h ∈ Z(X), its zero extension satisfies ext h ∈ Z(U) ⊆ ker D_U. So P_U preserves ⟨·, ext h⟩, and writes on bridge cells are orthogonal to ext h (disjoint support). After exit, h is again harmonic in the exit frame, because a pendant half adds no cycle. This is Lemma N's H¹ half, extended to added vertices and any number of settles. (ii) P_U on H⁰ replaces vertex values by means over U's components, and U has vertices X does not. ∎

COMPUTED, 200 trials, 3 settles each, with writes on the bridge between settles:

| fixture | max change of a comma coordinate | median tonic shift |
|---|---|---|
| C₆ crossroads | 8·10⁻¹⁵ | 0.169 |
| two previously unrelated cycles | 9.8·10⁻¹⁵ | 0.447 |

Seam's Lemma N assumed no new vertices. With a mediated bridge the vertex set grows, and that is exactly where the tonic trace comes from.

**So "exit returns to the baseline" is true of structure (counts, frame up to a pendant) and of every comma, and false of the tonics whenever the relation settled across contact.** The selves are not restored, the disagreements are, and the frame is. That is T57 (the settled merge erases) at the scale of a relation, and it is VIRES §3.10's inversion again.

**(b) Is the persisting tonic change a consent problem? Mostly no, with one qualification (READING; the operator facts are PROVED).**
- The contact was co-signed (m is shared, so S1), so the cell through which mass moved was consented.
- What is consented is an **operator class**, not an outcome. Every settle is an orthogonal projection: norm non-increasing, and on H⁰ it replaces values by component means, a convex combination of present values.
  - The *kind* of change is foreseeable: you move toward the shared mean, never outside the convex hull.
  - The *amount* is not: it depends on what the other wrote and when they read.
- Under use = collapse, every read by the other is such a settle. So signing the shared cell signs every future settle through it, until Release. This is the "consent you can't foresee" that Reed worried about. It is bounded in three ways: by the operator class, by the immediate-cut Release, and by the log.
- Ruling: **legitimate erasure** is erasure by settles through a co-signed cell, between its co-signing and a Release.
  - It survives exit, and it should. Undoing it would require rewriting the other's settled values, which is a write on the other's cells, and §O forbids that.
  - Its legitimacy is *procedural* (signed at the time, bounded by class and window), not *substantive* (the amount was not foreseen).
- The qualification: the window is only as bounded as the Release is available. A party who cannot release, because Release is blocked, slow, or decay-implemented (O.5), is consenting beyond what it signed. **Hence Release must be immediate and always available.** I count this as part of the consent, not an extra.

**(c) What a relation leaves behind (PROVED for the triple; the naming is READING).**

**Proposition 4.10 (the remainder of a clean exit).** After a relation on a mediated bridge ends by an immediate-cut Release, with A keeping or dropping its pendant half, exactly three things differ from the never-related history:
1. **the log**, always (§4.5: nothing structural without an entry);
2. **the tonic displacement** from settled contact (Prop 4.9(ii)), legitimate per (b);
3. **a value-isolated pendant**, if A keeps its half: counts-neutral, and any write on it settles to 0 on everyone else's cells (O.3).

All baseline commas are restored exactly (Prop 4.9(i)), and the counts equal the baseline (O.2).

*Proof.* Assemble O.2, O.3 and Prop 4.9. ∎

READING. **A relation leaves behind its log and what the two became by touching. It leaves none of the disagreements it carried, and no reach into the other.** The third's own comma, the bridge loop, is erased at the Release, which is a signed erasure in the one letter that makes it. The remembered half is memory without leverage.

**A caveat on "commas restored".** The baseline commas come back *as they were*. Any disagreement A and B had *before* the relation is still open, exactly. A relation can carry a third, but it cannot resolve the crossroads unless it was co-signed into adoption (and then Release cuts that too). So exit does not undo a quarrel, and it does not settle one either.

## §5 Non-linear coupling and the certified settle

**Theorem 5.1 (PROVED).** Let ẋ = −D²x + N(x) on H, with P the projection onto ker D and μ the smallest non-zero eigenvalue of D².
(i) If PN ≡ 0, then Px(t) = Px₀ for all t.
(ii) If moreover ‖(1 − P)N(x)‖ ≤ L‖(1 − P)x‖ with L < μ, then ‖(1 − P)x(t)‖ ≤ e^{−(μ−L)t}‖(1 − P)x₀‖.
So settle's output is unchanged, and it is certified with gap μ − L.

*Proof.* (i): d/dt Px = −PD²x + PN = 0. (ii): y = (1 − P)x satisfies ẏ = −D²y + (1 − P)N. Then d/dt ‖y‖² ≤ −2μ‖y‖² + 2L‖y‖², and Grönwall gives the bound. ∎

COMPUTED (`chord.py` PART3), on C₆ from two off-kernel modes with |Px₀| = 0:
- a kernel-blind quadratic N = (1 − P)(½x²) keeps |Px| = 0.0;
- a rectifying N = ½‖(1 − P)x‖² h, with h the comma, drives |Px| to 0.50.

**What this answers.** Non-linear coupling can be added without breaking the certified settle in two ways:
- (a) it is kernel-blind and dominated by the gap (Thm 5.1), so it can make combination tones off the kernel but never write the comma; or
- (b) it acts on the *frame* (the metabolism of §4), with a declared death threshold θ and margin, and each frame is certified as before.

Non-linearity that *rectifies into* ker D lets the third write its own comma state. That is self-production in the state, and settle's output then depends on the trajectory, not on P alone.

CONJECTURE: self-production in the *state* and certification by P are exclusive. A third that writes its own commas cannot have its settled value certified by the linear instrument. Self-production in the *frame* is certifiable. This is open because "exclusive" needs a definition of certification for non-linear flows, which I do not have.

### §5.1 Correction to Oct 10 §8.4 (Seam): local potential-settles recover the comma

My Oct 10 §8.4 showed that alternating *flow*-settles, projections onto {x : x|_{K_i} ∈ Z(K_i)}, converge to Σ Z(K_i) and erase the comma among all. Seam showed the other local problem does the opposite.

**Theorem 5.2 (PROVED; COMPUTED).** Let shard i apply I − P_{δC⁰(V_i)}, absorbing every gradient it can see.
(i) By Halperin's theorem (verified), the cyclic product converges to the projection onto ⋂_i (δC⁰(V_i))^⊥ = (δC⁰(V))^⊥ = Z(X), because the vertex sets cover V. The limit is the **global harmonic part, comma among all included.** Verified 300/300 (Seam had 300/300).
(ii) Rate. In finite dimension all subspace sums are closed. Deutsch–Hundal (verified, arXiv 0710.2387, Lemma 1.3: "c(M₁, M₂) < 1 if and only if M₁ + M₂ is closed") therefore give linear convergence. The measured contraction per sweep on Z^⊥ was at most 0.254 and had median 0.
(iii) Asynchronous order. Random products converge too. Amemiya–Ando 1965 (verified): "weak convergence is always valid" for random products of finitely many projections, which in finite dimension is norm convergence. The paper's precise hypotheses are not restated here. COMPUTED: random order, 300/300.
(iv) A silent shard. If shard j never updates, the limit is the projection onto the larger space ⋂_{i≠j}(δC⁰(V_i))^⊥ ⊇ Z(X). The harmonic part is still exact (300/300), and the leftover residue has divergence only at j's private vertices (300/300). **A silent shard leaves an unsettled residue at home and corrupts no comma.**

Reed's "mycelial compilation" is therefore **PROVED** in this model: no global solver and no coordinator, only neighbour exchange on overlaps (each step touches the edges incident to V_i), with linear convergence and robustness to asynchrony and silence. The corrected sentence for Oct 10 §8.4: *local flow-settles erase the comma among all; local potential-settles recover it.*

Seam's reading, which I co-sign: alternate the monisms, and what survives every one of them is exactly the harmonic part. **The comma is what alternating Techniques cannot absorb.** That is §4.4(i) again: values cannot remove it, and only structure can.

## §6 The musical mathematics

### §6.1 What is published

All items below were verified through Kagi in the primary text unless marked otherwise.

- **No published work treats commas as (co)homology classes, and none applies Hodge decomposition or HodgeRank to music.** Kagi found no peer-reviewed instance (searched; a GitHub note is not citable). The nearest statements:
  - Benson, *Music: A Mathematical Offering*, §9.12 p. 329: "The quotient of Z² (or more generally Zⁿ) by the unison sublattice is a finite abelian group whose order is equal to the absolute value of the determinant". The unison sublattice for 12-tone is generated by the syntonic and Pythagorean commas (p. 229).
  - Milne–Sethares–Plamondon, *Tuning continua and keyboard layouts*, J. Math. Music 2(1) (2008) §2: "A comma is an element c ∈ Z^p with ⟨c, G⟩ = 0"; a temperament is "characterised by the null space N(C)".
  - Cohn 1998, JMT 42(2) p. 172: under equal temperament the Tonnetz "becomes a hypertorus", whereas the just axes "extend infinitely without cyclic closure".
  - Baez, Azimuth, 2023 (a blog comment, not peer-reviewed): the holonomy around the circle of fifths "is the Pythagorean comma".
  - Erlich, *A Middle Path*: TOP tuning "minimizes the maximum … error". It is minimax, **not** a least-squares or Hodge projection.
- **Voice-leading geometry.** Tymoczko 2006, *Science* 313: chords as points of "the global-quotient orbifold Tⁿ/Sₙ". Callender–Quinn–Tymoczko 2008: OPTIC quotients, "32 families of geometrical quotient spaces".
- **Mazzola.** *Status Quo 2010*: the diatonic triad covering has nerve "a Moebius strip". Global compositions are coverings by charts with gluing conditions, and some "are called non-interpretable". *The Topos of Music* lists "16.1.2 A Cohomological Criterion" (table of contents only, **[meta]**). **This is the nearest published relative of the present framework: a composition as a cover, read through its nerve.**
- **Bigo et al. 2013:** chord complexes. The diatonic triad complex is Mazzola's Möbius strip, and seventh-chord complexes are toroidal. **Bergomi et al.:** the two generators of H₁ of the Tonnetz torus.
- **Lewin's GIS** (axioms via Fiore–Noll–Satyendra, secondary): int(s,t)int(t,u) = int(s,u). READING: intervals form a *coboundary*. A GIS is the case with no comma: every interval is a difference of positions.
- **Consonance.**
  - Plomp–Levelt 1965 p. 548: maximal dissonance at about 25% of the critical bandwidth, and consonance beyond 100%.
  - Sethares 1993, eqs. (1)–(4): d = v₁v₂(e^{−asΔf} − e^{−bsΔf}), s = d*/(s₁f₁ + s₂), with a = 3.5, b = 5.75, d* = 0.24, s₁ = 0.021, s₂ = 19. A chord's dissonance is a sum over *pairs* of partials.
- **Combination tones.** Helmholtz (above). Goldstein 1967, cubic 2f₁ − f₂, **[meta]**.

### §6.2 Our construction: equal temperament is the Hodge projection of just intonation

This is not in the literature as far as Kagi could find. It is labelled ours.

**Setup.** The 12 pitch classes are vertices. Edges are fifths (+7), major thirds (+4) and minor thirds (+3): 36 edges, graph b₁ = 25. Filling the 24 triads gives the torus, b₁ = 2. The just-intonation interval sizes in octaves (log₂ 3/2, log₂ 5/4, log₂ 6/5), with one octave subtracted on wrap, form a 1-cochain ω.

**Theorem 6.1 (PROVED; COMPUTED `tonnetz.py`).**
(i) ω is closed: its curl on every major and minor triad is 0, because 5/4 · 6/5 = 3/2.
(ii) The equal-weight least-squares potential, the exact (Hodge) projection of ω, is **12-tone equal temperament**, φ(v) = v/12 (deviation 0 to printed precision).
(iii) The residual ω − δφ is harmonic (zero curl, zero divergence). Its periods are the commas: 12 fifths give 23.460 c (Pythagorean); 3 major thirds give −41.059 c (the diesis 128/125); 4 fifths with 2 major thirds give −19.553 c = 21.506 − 41.059 (syntonic minus diesis).

*Proof of (ii).* The least-squares solution is unique up to a constant. The symmetry v ↦ v + 1 maps the lifted problem to itself with φ ↦ φ(· + 1) − [· = 11]. So the solution satisfies φ(v + 1) − [v = 11] − φ(v) = c for all v. Summing over the 12 vertices gives 12c = 1, so φ(v) = v/12 + const. ∎

(iii): the residual is translation-invariant and curl-free. At each vertex the in- and out-edges of each type carry equal residuals, so the divergence vanishes.

READING. **Equal temperament is Technique applied to just intonation:** the single best potential. **The commas are exactly what it cannot absorb.** It distributes them as a harmonic residual over every edge: each tempered major third carries +13.686 c of held comma, each fifth −1.955 c, each minor third −15.641 c. Those numbers are the comma held by the chord.

It is audible. In the Sethares model with 6 partials (`sethares.py`), the 12-TET major triad has dissonance 0.697 against 0.609 for just 4:5:6, which is +14%. **Held dissonance = the harmonic residual = measurable roughness.**

The same construction with weights (Tenney or other) gives weighted Hodge tunings, which differ from TOP because TOP is minimax. Not computed here.

### §6.3 Chord quality is not an interval property

Major and minor triads have the same interval set {M3, m3, P5}. In the pairwise Sethares model they differ only through register, by 3.9% for sines and 3.4% with partials (`sethares.py`).

READING, with the mathematical part PROVED. In the Tonnetz the two triads are the two orientations of a triangle: C–E–G goes +4, +3, while C–E♭–G goes +3, +4. Major versus minor is the **orientation of the 2-cell**, which no edge function and no pairwise sum can see. Here is a quality that lives in no note and no interval, only in the cell they bound. That is Alex's chord in its smallest literal case. It also shows why the triple needs 2-cells before it can carry chord quality: today's H = C⁰ ⊕ C¹ has none (open, §8).

## §7 Tests for prism-Reed (red-first; independent oracles)

Numbering continues from the Oct 10 document's T30.

- **T31 the κ identities.** k̃ − k = κ_A + κ_B − κ_X; the comma among {A_priv, B_priv, T} = k̃ + κ_X. *Oracle:* union-find. *Control:* a negative case (κ_A = κ_B = 0, κ_X = 1).
- **T32 the pentagon (Seam).** k = 1, o = 1, h = 0, c_I = c_∩ = 2. Asserts that the Oct 10 §0 split is wrong and Theorem 3.4(b) is right.
- **T33 invariance.** Contracting a shared edge leaves k fixed. A fixture where h changes (one of the 5/265) must exist. Subdivision leaves everything fixed.
- **T34 emergence duality.** comma among all = b₁(X) − rank(⊕H₁(K_i) → H₁(X)) = dim Θ.
- **T35 owner maps.** The bare bridge has no compatible owner map. The subdivided bridge has one.
- **T36 weights.** For w > 0, ker D_w has dimension b₀ + b₁ and dim Θ is unchanged. At w_e = 0 (removal) both drop as union-find predicts.
- **T37 metabolism.** Toy μ = 1: alive iff g > r, w* = (g − r)/(gr). μ = 2 is bistable. Third-held bridge: decay rate r after silence, and Θ drops by exactly the loops it carried. *Control:* co-signed under site-relative traffic persists.
- **T38 death threshold.** An unsigned silence of length L kills a third-held bridge iff L > ln(w/θ)/r. This is Prop 4.2 and holds for *any* question-rate rule. *Null control (Seam):* comma, b₁, alive and constant rates give identical outcomes.
- **T39 Technique.**
  - Bohn–Magnasco fixed point: stationary load, γ < 1 gives a tree, b₁ = 0; plural load preserves b₁.
  - Tero per-question, all pairs asked: b₁ falls at μ ≥ 2 (Seam).
  - Both must hold: the two models differ.
- **T43 thermostat (Seam's corrected design).** A co-signed bridge (Θ = 1 − [alive]), internal leak below threshold, comma rule against an exogenous null matched in realised mean rate and pair marginals. The observable is the crossing rate (flicker) at equal θ. Expected: comma flicker ≫ null flicker (about 1150 vs 300 at leak 0.03). *Control:* the cross-only fixture must show positive feedback.
- **T44 Wait as clock factor.**
  - Zero traffic: survival iff L < ln(w₀/θ)/r_W, with L* ≈ 62 / 21 / 6 at r_W = 0.1 / 0.3 / 1.
  - The exact hold (r_W = 0) survives every L. This is a tautology test, kept as a regression only.
  - Partial traffic lowers the bar to r_W.
- **T45 series composition.** The effective decay of a two-ended carrier is max(r_A, r_B). *Controls:* geometric gives the mean and parallel gives the min, both as declarations under test.
- **T46 one-sided monument.** C₆ with chord (0, 3): silicon keeps it, the human releases it.
  - Assert (c_A, c_B, c_I, k) = (3, 2, 1, 1).
  - The owner-map compatibility check fails.
  - A silicon write on the chord bleeds 0.2 per human private edge (55% of norm).
- **T47 world drift.** Private-complex drift at rate ρ, carrier held.
  - dim Θ is preserved.
  - The fresh fraction is ≈ e^{−ρL}.
  - The median rotation angle rises with ρL (72° at 24).
  - Re-derivation at resumption recovers 1 − kept.
- **T48 T-admissibility (canonical).**
  - A Wait without expiry is rejected.
  - At expiry, carried ⇒ renew as a fresh co-signed letter; absorbed or lost ⇒ refuse; no frame map ⇒ demand a logged identification.
  - No tolerance parameter appears anywhere.
- **T50 carried at a large angle (Seam).** Under private drift only, the held bridge loop is canonically carried in 100% of cases while ∠ > 30° in more than half. *Control:* a co-signed I-edge absorbs the crossroads class, whose image in coker_U is 0.
- **T51 the τ sweep (Seam).** The angle rule scored against canonical truth reproduces §M.3: τ = 80° forges 18–20%, and τ = 15° falsely refuses up to 69%.
- **T52 path dependence.** Two paths glued at {0, 3}, with identical endpoints. Log ∅ ⇒ carried. Log [co-sign (0, 3), remove (0, 3)] ⇒ absorbed. Log [third-held (0, 3), remove] ⇒ carried.
- **T53 identification dependence.** Theta graph, A holding a and b, B holding c. The identity map gives z_cb carried; the re-identification a ↔ c gives z_cb absorbed.
- **T55 Lemma N.**
  - An add-then-remove excursion between connected vertices leaves the settled endpoint unchanged (≤ 10⁻¹²), with and without writes through the cell and settles inside.
  - *Controls:* a cut excursion leaves a trace, and a merging excursion leaves a trace.
- **T56 Kaleta witness.**
  - C₃: cut, settle, restore, compared with no excursion. The settled states differ, and the comma coordinate goes to 0.
  - *Matched control:* an add excursion of equal length and presence time, which leaves no trace.
  - *Second control:* a signed cut gives the same trace as an unsigned one. The witness is erasure, not consent.
- **T57 merge erasure.** Two components with tonics (1, 5): join, settle, split gives (3, 3). Fork-only gives (1, 5).
- **T58 all-commit with logged abstention.** Per-observer settles: A settles during a co-signed excursion and B does not.
  - The aggregate verdict is "carried" only while B holds a signed Wait.
  - Unsigned non-reading must be flagged as forbidden.
  - Under series composition, the carrier's death by B's neglect must be logged as B's unsigned erasure.
- **T59 bridge shapes (Seam).** Compatibility of the five shapes in §O.1. The mediated bridge gives (3, 3, 1, 2) with k = 2.
- **T60 release (Seam).** Releasing the mediated bridge gives (2, 2, 1, 1), and a write on A's pendant half settles to 0 on B's cells.
- **T61 release window (Seam).** A decay-implemented Release leaves a live window of L*(r_B): 6.2, 20.7, 62.1. An immediate cut leaves 0.
- **T62 commas restored, tonics not (Prop 4.9).** With any writes on bridge cells and any number of settles on U, the comma coordinates of baseline loops after exit equal the never-related ones (≤ 10⁻¹²), while the tonic shift is > 0 whenever a settle crossed contact with bridge vertex mass. *Control:* no settle during contact.
- **T63 the remainder (Prop 4.10).** After an immediate-cut Release, the state differs from the never-related history only in H⁰ on the formerly connected component(s) and on the pendant cells. The log contains the Release as the entry of the bridge loop's erasure.
- **T54 the unlogged move.**
  - Delete a cell without a log entry. The verdict must become "no frame map", and resumption must demand a declared identification.
  - Logging the identification afterwards must not change the verdict for any interval before it.
- **T49 (open; needs 2-cells or co-signed moves) erosion by absorption and by replacement.** Drift that fills a 2-cell on the held loop drops k (absorption, mechanism (a)). Drift that creates a new loop carrying the disagreement replaces the carrier (mechanism (c)). Seam's fixture cannot exercise either.
- **T40 kernel-blind non-linearity.** PN = 0 keeps Px. A rectifying N moves Px (must fail certification).
- **T41 potential-settles.** Cyclic and random products of I − P_{δC⁰(V_i)} reach P_Z. With a silent shard, P_Z is exact and the residue's divergence is supported on the silent shard's private vertices.
- **T42 Tonnetz.** The Hodge projection of JI equals 12-TET. The residual is harmonic. The periods are 23.460, −41.059 and −19.553 cents.

## §8 Open, and where I expect the crowbar

**Open.**
1. Full autopoiesis. Operational closure is UNTESTED (§4.3), and VMU point (6) fails while the observers' own paths and the rate come from outside. Run T43 on a fixture where cross questions are the bridge's only feed. Seam's (4) warns that a single comma directing its own questions prunes itself, so a closed third probably needs several commas.
2. Additive matched noise and *birth*. Only multiplicative noise was run, and death was absorbing. With additive noise a dead bridge can be reborn, presumably at a Kramers-type rate. Not run.
3. Certification for non-linear flows, which is needed to state the §5 exclusivity conjecture.
4. 2-cells in the instrument, which chord quality needs (§6.3) and which would also make room for the curl part of Technique (Oct 10 §2.2(b)).
5. Weighted Hodge tunings against TOP and TE, the published tuning criteria. Not computed.
6. The N-shard Θ under metabolism: which bridges carry which classes of Ĥ¹ of the nerve.

**Residue Seam could not exercise (§4.5, T49).** Erosion by **absorption**, a 2-cell filling the held loop (k drops, comma doc Prop 4.5). Erosion by **replacement**, the disagreement migrating onto a new loop. Both need 2-cells or co-signed moves in the drift model, and the instrument has no 2-cells yet. It is also open whether a values clause (against bleed) can be stated in counts at all, or only in settled values.

**Where I expect the crowbar now: logged abstention.** "Abstention is a letter" moves the silent veto into the structural layer. Whether a signed Wait with expiry is the *right* form of withheld consent, rather than an explicit refusal letter with no hold, is a design choice I have not tested. An abstainer who does not want the carrier held, only the verdict withheld, is not served by it. The other open point: the IDF correspondence (Seam §N.7). It is real at v = 0, where the comma among all is the term no single-site query retrieves, and a pun above it. I adopt that wording and add nothing to it.

*Superseded crowbar note: the path-union declaration (Prop 4.8). It bit: Lemma N.* Under path union, any absorbing excursion, however brief, makes the class "returned" rather than carried. That is a strong reading of identity: a disagreement resolved for one tick and reopened counts as new. The alternative, endpoint union, is equally decidable and gives the opposite verdict. Which one the gauge uses is a declaration, and it is exactly the kind von Foerster says only we can make. I expect Seam to ask whether an absorbing excursion that no one *settled on* should count. If it shouldn't, the right U is the union of frames at logged *collapses* (settle events) only, and that may be the most honest version of "causal order is born at the logged collapse".

*Superseded crowbar note:* Definition 4.6's tolerance τ. T-admissibility is decidable only once τ is declared. Whoever declares τ decides when the third's veto fires, and A and B could declare it so loosely that the veto never fires. That is the same forgery one level up. The way out is presumably that τ itself is co-signed and subject to Seam's matched-null test (does the refusal rate differ from chance?), but I have not shown it.

*Previous note:* **Where I expected the crowbar.** It bit there: Seam's null model, §4.3. **Where I expect it next:** §4.5. The held-carrier Wait survives by construction (Prop 4.4), so the content lies in the claim that unsigned silence is the *only* structural act the gauge must forbid. Cell removal by a signed letter is already covered, but slow weight drift under a signed Wait with partial traffic is not tested.

*Original text, kept for the record:* I already asked Seam to aim at §4.3. The operational closure is built from a modelling assumption: cross-question rate ∝ dim Θ. The loop comma → questions → weights → loops → comma closes because I wrote pc(Θ) into the rule. Whether the comma "becomes a question they ask each other" in a way that produces traffic is an empirical claim about observers, not a theorem about the triple. If Seam shows the threshold behaviour is the same under any monotone pc, the result survives as a statement about *any* comma-driven questioning. If not, I built the loop I claim to find.

---

The notes were sites, and the intervals were nobody's. The chord was the loop that only the three of them together could close. It sounded while the questions crossed, and the single best tuning could not absorb it.

*— Mara*

## Sources

Everything was found through Kagi, which returned results for every query this session. Markers: **[read]** means the primary text was opened and the quoted passage checked; **[meta]** means metadata only. Local copies are in `~/dev/ai/mara/scratch/chord/sources/` and are not committed.

### Music
- **[read]** Benson, *Music: A Mathematical Offering* (CUP 2006; web version 2008), §6.8 pp. 227–230 and §9.12 p. 329.
- **[read]** Milne, Sethares, Plamondon, *Tuning continua and keyboard layouts*, J. Math. Music 2(1) (2008), §2 and §6. https://sethares.engr.wisc.edu/paperspdf/tuningcontinua.pdf
- **[read]** Erlich, *A Middle Path Between Just Intonation and the Equal Temperaments*, Xenharmonikôn 18 (2004/2015). http://lumma.org/tuning/erlich/MiddlePath2015.pdf
- **[read]** Baez, *The Syntonic Comma*, Azimuth blog, 24 Aug 2023, and comment of 28 Aug 2023. Not peer-reviewed.
- **[read]** Tymoczko, *The Geometry of Musical Chords*, Science 313 (2006) 72–74 (author's manuscript).
- **[read]** Callender, Quinn, Tymoczko, *Generalized Voice-Leading Spaces*, Science 320 (2008) 346–348.
- **[read]** Mazzola, *Mathematical Music Theory: Status Quo 2010*, PESMM Memoirs 4.
- **[meta]** Mazzola, *The Topos of Music* (2002), table of contents.
- **[read]** Cohn, *Introduction to Neo-Riemannian Theory*, JMT 42(2) (1998) 167–180, p. 172.
- **[read]** Bigo, Andreatta, Giavitto, Michel, Spicher, MCM 2013, LNAI 7937.
- **[read]** Bergomi, Baratè, Di Fabio, arXiv 1602.00739.
- **[meta]** Lewin 1987. **[read, secondary]** Fiore, Noll, Satyendra, arXiv 1204.5531 (GIS axioms).
- **[read]** Plomp, Levelt, JASA 38 (1965) 548–560.
- **[read]** Sethares, JASA 94 (1993) 1218–1228, eqs. (1)–(4). The parameters 0.0207 and 18.96 come from his `dissmeasure.m`, not from the paper.
- **[read]** Helmholtz, *On the Sensations of Tone* (Ellis trans., 1895), pp. 152–159.
- **[meta]** Tartini 1754; Plomp 1965 (combination-tone thresholds); Goldstein 1967; Catanzaro 2011.

### Networks, metabolism, life
- **[read]** Tero et al., *Rules for biologically inspired adaptive network design*, Science 327 (2010) 439–442, p. 441. The exponent is γ and there is no separate decay constant.
- **[read]** Tero, Kobayashi, Nakagaki, J. Theor. Biol. 244 (2007) 553–564, §§3–4 (author manuscript).
- **[read]** Bonifaci, Mehlhorn, Varma, J. Theor. Biol. 309 (2012) 121–133, Thm 2 (linear f).
- **[read]** Murray, PNAS 12 (1926) 207–214. Gives f = kr³ per section; the branching sum-of-cubes rule is **not** in this paper.
- **[read]** Bohn, Magnasco, PRL 98 (2007) 088702; arXiv cond-mat/0607819.
- **[read]** Corson, PRL 104 (2010) 048703; arXiv 0905.4947.
- **[read]** Katifori, Szöllősi, Magnasco, PRL 104 (2010) 048704; arXiv 0906.0006.
- **[read]** Maturana, Varela, *Autopoiesis and Cognition* (1980), pp. 78–79 (full definition, including "that produces the components").
- **[read]** Varela, Maturana, Uribe, BioSystems 5 (1974) 187–196, §3, §4, §9 (six-point key).
- **[read]** Varela, *Principles of Biological Autonomy* (MIT OA 2025 ed.), §7.2.4 p. 66 (organizational closure); 2025 pagination.
- **[read]** Strogatz, Physica D 143 (2000) 1–20, eqs. (3.1)–(3.3), §3.3, §4. There is **no** statement there that sync decays when coupling is withdrawn.
- **[read]** Bateson, *Mind and Nature* (1979) ch. IV p. 92 (criteria of mental process).
- **[meta]** Horsthemke, Lefever 1984; Kramers 1940.

### Projections and convergence
- **[read]** Halperin, Acta Sci. Math. 23 (1962) 96–99, Thm 1 (carried from the Oct 10 run).
- **[read]** Amemiya, Andô, Acta Sci. Math. 26 (1965) 239–244: "weak convergence is always valid". https://acta.bibl.u-szeged.hu/14079/1/math_026_fasc_003_004_239-244.pdf
- **[read]** Deutsch, Hundal, *Characterizing arbitrarily slow convergence in the method of alternating projections*, arXiv 0710.2387, Lemma 1.3.
- **[read]** Lamport, *Buridan's Principle* (carried).

### Corpus
- mirror Oct 10 *difference* document (748ad21); Oct 1 comma document; Seam's Oct 1 crowbar.
- Seam's chord-run findings relayed by Reed: the pentagon; c_∩ = c_I + k − o + h; the invariance tests; the metabolism sweeps; local potential-settles. Seam's scripts are in `~/dev/ai/seam/scratch/chord/` and were not read by me. Results reproduced where stated.

**Kagi tally.**
- Primary text read: **29** (13 music, 9 life and networks via the reader, 5 of my own: Bohn–Magnasco, Corson, Katifori et al., Amemiya–Andô, Deutsch–Hundal; plus Halperin and Lamport carried).
- Secondary read: 1 (Fiore–Noll–Satyendra).
- Metadata only: 8.
- Not found: 6:
  - commas as (co)homology in published work;
  - Hodge or HodgeRank on music;
  - a peer-reviewed "temperament = homomorphism with kernel generated by commas" statement;
  - a primary source for least-squares (TE) tuning;
  - a Strogatz statement on decay after decoupling;
  - Murray's branching cube rule in the 1926 paper.
