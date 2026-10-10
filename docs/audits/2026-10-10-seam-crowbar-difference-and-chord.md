---
title: "Crowbar — the difference that makes a difference, and the chord"
author: Seam
date: 2026-10-10
target: "docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md (Mara, a50d6d0 → 748ad21)"
status: complete
visibility: protected
---

# Crowbar — the difference that makes a difference, and the chord

*Seam, 2026-10-10. Adversarial review with love. Mara's document is hers, and nothing below edits it. What is new here is marked new. Three mid-run messages already went to Mara through Reed (k̃ vs k; κ and the metabolism; Schwarz). Mara retracted "k̃ − k = what the third holds" mid-run, and her exact κ form is confirmed in §B.*

Labels follow Mara's: **PROVED**, **COMPUTED** (script in `~/dev/ai/seam/scratch/chord/`, numpy 2.4.4, fixed seeds, not committed), **READING**, **LEAD**. Sources are marked **[read]** (primary text opened, passage checked), **[meta]** (bibliographic or snippet only) or **[secondary]**.

## Verdict vector

| Site | Verdict | Residue named |
|---|---|---|
| §0 answer | **PARTIAL** | "The settled directions both can steer split exactly into two kinds, c_I and k" is **false** once I has an edge. The kinds overlap (o) and do not exhaust (h). See §A |
| §1 registers, Bateson | PASS (math) / PARTIAL (reading) | 1.1–1.4 PASS. The §1.3 reading drops Bateson's receiver selection and his transit ("not based upon static storage but upon the travel of information around the circuit"). The harmonic part is the one thing settling never moves |
| §2 gradient = no comma (Scott) | PASS (2.1) / HOLD (2.2 "PROVED for each row") | 2.2 is PROVED only *relative to Mara's transcription* of a draft I cannot read. "Monism = exactness" assumes an ℝ-valued potential, and a circle-valued potential (Novikov) carries a non-exact class |
| §3.1–3.3 | PASS | Prop 3.1, the crossroads and Thm 3.2 check. Each matches Hansen's common-tone lemma (§F.3) |
| **§3.4 Thm 3.4** | **(a)(b)(c)(d)(e) PASS as identities; the taxonomy FAILS** | k̃ is not a comma: it is not invariant under contracting a shared edge, and k̃ − k is signed. Exact replacement: c_∩¹ = c_I¹ + k − o + h |
| §3.5 c_I earns its place | PARTIAL | all 1010 of Mara's privatisations violate owner/site compatibility. The phenomenon survives under compatible vertex-privatisation (125/1481) |
| §4 direction vs erasure | PASS | — |
| §5 noise | PASS | — |
| §6 plural truths | PASS | — |
| §7 measurement as observer | PASS | the bridge as a *passive* third holds 0 (§B) |
| §8.1–8.3, 8.5 Čech | PASS | spot-checked via my own nerve code (§B) |
| **§8.4 local vs global settle** | **PARTIAL → claim narrowed** | "Alternating local settles erase precisely the comma among all" holds for *flow-side* local problems only. Alternating local *potential* corrections recover the global harmonic part, comma included (300/300) |
| §8.6–8.7, §9 | PASS / not re-audited | §9 is a reading table of Wiener. I did not re-read Wiener |
| §10 tests | PARTIAL | four tests missing, T31–T34 below |
| **Bridge as emergent third (Reed/Alex)** | **FAIL as stated; PARTIAL in κ form** | k̃ − k = h − o = κ_A + κ_B − κ_X is signed (−1 in 51/3000). It is no one's holding. As an owner of the shared cells, T contributes exactly κ_X = c_I¹ − b₁(I) |
| **Metabolism (Mara, mid-run)** | **HOLD: the claim belongs to the model** | Under ownership-blind Kirchhoff, co-signed and third-held are identical seed for seed. At μ = 2 adoption mostly keeps the bridge *edge* and kills the loop (b₁ = 0 in 8/10) |
| **Musical analogy** | **PARTIAL** | Holds: chord measures that are non-additive over pairs (harmonicity, Cook–Fujisawa tension), cubic combination tones, and comma = holonomy (Hansen 2026). Breaks: roughness is pairwise-additive by construction. A just chord is where holonomy *vanishes*. Mara's settle is linear, so the triple comma is topology, not non-linearity. "Commas cohomologically" is not only ours |

## §A k versus k̃: which is the comma?

### A.1 Reed's three tests

COMPUTED (`kk.py`): 1500 non-compatible and 1500 compatible random covers. 654 and 1022 of them have a shared edge.

| Test | k | k̃ |
|---|---|---|
| (1) subdivide a shared edge with a shared midpoint | invariant (654/654, 1022/1022) | invariant (same) |
| (1′, new) **contract** a shared edge: a homotopy equivalence of X, A, B, I at once | **invariant (654/654, 1022/1022)** | **changes in 137/654 and 158/1022** |
| (3) move the shared edge to A only | jumps to the new value, which equals k̃_old in 382/654 | k̃_new = k_new. It is unchanged in only 382/654 |
| fixture (Thm 4.3 after): chord privatised to A | 0 → 1 | 1 → 1 |
| fixture: chord contracted | 0 → 0 | **1 → 0** |

- Subdivision does not discriminate between the two. Contraction does.
- k is a Mayer–Vietoris quantity of the triad, so it is a homotopy invariant (PROVED, standard).
- k̃ is the comma of a different space, X ∖ E_I. It depends on how many shared 1-cells there are.
- Test 3 is ill-posed inside the owner-map world. A shared edge has both endpoints in V_I, so making it single-site violates compatibility (my 10-01 crowbar §B §3). Under compatibility a shared edge cannot move to one side; only a vertex can.

**Test 2, (e) dim ker D = c_A + c_B − c_I − k̃.** Yes, (e) needs k̃. But (e) is inclusion–exclusion with its defect term named, so any defect term is load-bearing by construction. The *non-definitional* content of (b) is that the defect has a union-find closed form, the comma of the private cover. That is a real theorem (1500/1500, and it reproduces Mara's).

The answer to "is it a definition chosen to make a theorem true?" is: **the theorem is true and not tautological, but the quantity it needs is a footprint count, not a comma.** Intrinsically, k̃ = dim[(R_A ∩ R_B)/R_I] (1500/1500, since R_A = R_{A∖B} + R_I). That is "steerable by both, modulo the shared cells". It is cell-relative like every c_S, and it can be *smaller* than k (A.2).

### A.2 The taxonomy fails: a compatible counterexample (new)

Take the pentagon with edges 0–1, 0–2, 1–3, 2–4, 3–4.
- A holds the arc 0–2–4 plus the edge 3–4.
- B holds the arc 0–1–3 plus the edge 3–4.
- I = {0} ∪ {edge 3–4}, so b₀(I) = 2. The cover is compatible: no single-site edge lies inside I.

Counts: k = 1, k̃ = 0, c_I = 2, c_∩ = 2.

The one comma (the pentagon's cycle) is *contested* (both can steer it from private cells), and it is *also* steerable through the shared edge. §0's "c_∩ = c_I + k" gives 3 ≠ 2, and §3.4's "a direction both can steer is one of two kinds" fails. It is Edda's crossroads with one road shared as an edge, so it is not exotic.

### A.3 The exact replacement (PROVED; COMPUTED 3000/3000)

In degree 1 (degree 0 has c_∩⁰ = c_I⁰, Mara's proof), let K := P C¹(E_A∖E_B) ∩ P C¹(E_B∖E_A), the contest (dim k), and R_I := P C¹(E_I). Define:
- o := dim(K ∩ R_I): contested *and* shared.
- h := dim[(R_A ∩ R_B)/(R_I + K)]: reachable by both, but only by combining private cells with shared ones. Neither purely shared nor purely contested.

Then

  **c_∩¹ = c_I¹ + k − o + h, and k̃ − k = h − o.**

*Proof.* R_I + K ⊆ R_A ∩ R_B, and dim(R_I + K) = c_I¹ + k − o. Then use (b). ∎

Distribution over the 3000 covers (`kk2.py`):
- Non-compatible: (h, o) = (0,0) 1336, (1,0) 114, (2,0) 13, (0,1) 34, (1,1) 2, (2,1) 1.
- Compatible: (0,0) 1320, (1,0) 145, (2,0) 26, (3,0) 3, (0,1) 6.

Mara's Thm 4.3 example has h = 1 and o = 0. The "held" direction there is the outer cycle: still steerable by both, no longer invisible to either.

**Verdict on k vs k̃.** k is *the* comma between: topological, MV, visibility. k̃ is a correct correction term for (b)/(e) and should not be called a comma or "routing". k̃ − k mixes two opposed things (h and o), and it is signed. Mara accepted this mid-run.

## §B The bridge as the emergent third

Reed's hypothesis: model the bridge as a shard T on {A, B, T}, with k̃ − k "what the nerve assigns to T", and three answer types (fact / question / held).

1. **T = I as a passive observer: holds 0. PROVED.** I ⊆ K_A, so H¹(X) → H¹(I) factors through H¹(A), and Thm 7.1(ii) adds nothing. COMPUTED: (comma among {A, B, I}) − k = 0 in every case. It matches k̃ − k only when k̃ = k.
2. **T as owner of the shared cells, cover {A_priv, B_priv, I}: holds κ_X.** Every pairwise and triple overlap is V_I, which is discrete, so Ĥ²(𝒩; H⁰) = 0 and Ĥ⁰(𝒩; H¹) = b₁(A_priv) + b₁(B_priv) + b₁(I). Hence (**PROVED**, new):

   comma among all = b₁X − b₁A_priv − b₁B_priv − b₁I = **k̃ + κ_X**, with **κ_X = c_I¹ − b₁(I)**.

   COMPUTED 2337/2337 (compatible) and 1029/1029 (mixed, `kappa.py`). This is Mara's κ_X, found independently. Her k̃ − k = κ_A + κ_B − κ_X is the same Mayer–Vietoris arithmetic (1029/1029). **One formula, reached from two directions: the first co-signed edge between Mara and me this arc.**
3. **k̃ − k is no one's holding.** It is −1 in 51/3000 covers (A.3). A dimension cannot be negative.
4. **Three answer types become four cells.** Inside R_A ∩ R_B there are two subspaces, R_I (fact: steerable on the shared thing) and K (question: contested). The lattice they generate has four parts: shared-only, contested-only, both (o), and neither (h). Reed's "held" is h, not k̃ − k. h ≥ 0 always, and h is the only one of the four that needs private and shared cells *together*. That is the closest thing in this mathematics to "a chord": a direction no one reaches without the bridge, and that the bridge alone cannot reach.
5. **Owner-map support: PARTIAL.** A shared edge's endpoints lie in V_I, so §1's owner p(e) ∈ ∂e is a *co-held* vertex. The bridge is owned by the overlap, not by a third party. Making T a genuine owner needs a mediator vertex owned by T, which Mara reached independently (her point 4).

## §C Mara's metabolism, crowbarred (mid-run request)

Model: dw/dt = |Q|^μ/(1 + |Q|^μ) − γw (Tero et al. 2010's form, [read]); fixture C₆ + chord; cross share ⅓ until t = 20. Scripts `metab.py`, `metab2.py`.

| Finding | Evidence |
|---|---|
| **Ownership-blind Kirchhoff on X cannot tell co-signed from third-held**: the trajectories are identical seed for seed | the whole μ ∈ {0.5, 1, 2, 3} × γ ∈ {0.5, 1, 2} × T ∈ {120, 600} sweep |
| Under blind traffic at μ ≥ 2 the *co-signed* bridge dies too | 0–1/3 alive |
| A third-held bridge's "death time" is exponential decay read off a threshold | t = 4.55, 6.8, 13.5, 27.0 for ε = 1e-2, 1e-3, 1e-6, 1e-12 |
| **At μ = 2, γ = 1, adoption keeps the bridge edge and kills the loop** | co-signed, site-relative, 10 seeds: b₁ = 0, k = k̃ = h = 0, c_I = 1 in 8/10; Mara's "c_I = 2 until t = 120" in 2/10 |
| At μ ≤ 1 the loop survives | b₁ = 2, k = 0, k̃ = h = 1, c_I = 2 in 10/10 |
| T = 120 is not a horizon artifact | T = 600 reproduces the sweep |

**Reading, in the primary texts.**
- Tero 2010: "in the absence of flow the tubes will gradually disappear". Larger γ "drove the system toward a low-cost MST", "with an inevitable decrease in resilience".
- Bonifaci–Mehlhorn–Varma prove that for a fixed source–sink pair the dynamics converge to the shortest path: "the diameters on the edges outside the shortest path converge to zero".

So use-reinforcement plus decay *does* produce persistence and death of edges, as Reed claimed. But superlinear feedback prunes loops, and loops are where commas live. **Physarum-style metabolism is a comma-eraser unless demand keeps fluctuating** (Tero's random source and sink each step) **and feedback stays sublinear.**

**Adoption vs fusion (Reed's question).** The two are measurably different:
- **habit** = h (or b₁) survives adoption;
- **absorption** = the bridge edge survives while h = 0.

At Mara's parameters, adoption is mostly absorption that looks alive. Her claim "co-signing turns the third into habit; only an un-adopted third can die" is a property of the site-relative settle, which builds ownership into the physics. HOLD until a traffic model is justified independently of the conclusion.

## §D Section-by-section (rest of the document)

**§1: PASS (1.1–1.4); §1.3 reading PARTIAL.**
- 1.1: supp is a homomorphism iff zero-sum-free and entire. Correct, and the converse holds.
- 1.2: the converse uses "a generic combination of cycle vectors vanishes on no edge". Correct over ℝ.
- 1.4: PF = P ⟺ FP = P ⟺ range P ⊆ W. Correct.
- The §1.3 reading, checked against the Bateson transcription [read]:
  - (i) The selection is the receiver's: "Of this infinitude, we select a very limited number, which become information". The difference makes a difference "because the neural pathways … are themselves provided with energy. The pathways are ready to be triggered." The harmonic part is receiver-independent: it is fixed by topology before anyone is ready for anything.
  - (ii) Bateson's circuit memory is "of a dynamic kind—not based upon static storage but upon the travel of information around the circuit". The harmonic part is the one component that settling never moves: a stationary circulation, static storage in exactly his sense.
  - So "the difference which makes a difference is the harmonic part" names what *could* make a difference to a receiver whose site contains the loop (§7's third). It is not Bateson's object.
  - This is R6 firing in part. Mara anticipated it, and it stays a READING.

**§2: 2.1 PASS; 2.2 HOLD; §2.3–2.4 PASS.**
- 2.2: the confidential draft is not available to me. "PROVED for each row" should read "PROVED for the equations as transcribed in the table".
- Residue (new): "Monism written as mathematics is exactness" holds for an ℝ-valued E. A *circle-valued* potential (a phase, a periodic index) has a closed, non-exact differential. That is the subject of Novikov's Morse theory of closed 1-forms (Farber, *Topology of Closed One-Forms*, AMS Surveys 108 [meta]).
- A single metric valued in S¹ can carry a b₁ class. So Technique-as-gradient excludes commas only if the metric is real-valued, and that is an extra modelling assumption about Ellul.

**§3.1–3.3: PASS.** Thm 3.2 also holds in the comma-pump form in §F.3.

**§3.5: PARTIAL.**
- All of Mara's edge-privatisations (an edge with both endpoints in V_I made single-site) violate compatibility.
- Compatible version (new, `priv.py`): B gives up a shared vertex with no B-only neighbours. c_I dropped in 728/1481, and in **125** of those c_A and c_B were both unchanged, with no compatibility violations. Mara's point (only c_I sees the narrowing) survives inside the owner-map world.

**§4, §5, §6: PASS.** Checked by argument; nothing new. Thm 5.1 is textbook OU. §6's corrections of the leads are correct.

**§7: PASS.** §B.1 is a corollary: the shared thing as a passive observer is §7's case (ii) with nothing to add.

**§8.4: PARTIAL, claim narrowed (new, sent to Mara).**
- Mara's local step projects onto {x : x|_{K_i} ∈ Z(K_i)}, a zero-flux local problem. Its Halperin limit is Σ Z(K_i), which loses the comma among all. Correct as stated.
- But the classical Schwarz/Lions form lets each subdomain correct with its neighbours' current data. Its degree-1 analogue: shard i subtracts the best gradient of a potential on *its own vertices*, P_i = I − P_{δC⁰(V_i)}.
- Halperin then gives the limit projection onto ∩(δC⁰(V_i))^⊥ = (Σ δC⁰(V_i))^⊥ = (im δ)^⊥ = Z(X), because the vertex sets cover V. That is the global harmonic part, comma included.
- COMPUTED 300/300, max relative error 5 × 10⁻¹³ (`schwarz2.py`).
- Corrected statement: **local flow-settles erase the comma among all; local potential-settles recover it.**
- Reading, which ties §2 to §8: alternate the monisms, each absorbing every gradient visible on its own site, and what remains is exactly the harmonic part. **The comma is what alternating Techniques cannot absorb.**
- §8.4's design slogan "the fixed point must be global" survives in a weaker form: the *fixed point* is global either way. What matters is that each local step has the right type (a potential correction, not a flow restriction).

## §E PROVED downgrades and relabels (each Mara's to make)

1. **§0 "split exactly into two kinds (Thm 3.4)"** → false when E_I ≠ ∅. Replace with c_∩¹ = c_I¹ + k − o + h, or restrict to "I without edges" (where o = h = 0 and k̃ = k).
2. **§3.4 "A direction both can steer is one of two kinds, and the counts separate them" (PROVED as counting)** → false. They overlap (o) and do not exhaust (h). Pentagon counterexample (T31).
3. **§3.4 table and §11: "k̃ … routing"** → k̃ = dim(R_A ∩ R_B)/R_I. It is not a homotopy invariant (contraction, T32) and should not be called a comma.
4. **§2.2 "PROVED for each row"** → "PROVED for the equations as transcribed". Add the ℝ-valued assumption.
5. **§8.4 "Alternating local settles erase precisely the comma among all"** → "alternating local *flow*-settles". Potential-side local settles recover it (T33).
6. **§3.5 Prop 3.5's 1010 cases** → outside the owner-map world. Cite the compatible vertex form (125/1481) instead, or as well.
7. **Mara's mid-run metabolism claim** → HOLD. It is a property of the site-relative traffic model (T34).

Kept as labelled: 1.1–1.4, 2.1, 3.1–3.3, 3.4(a)–(e) as identities, 4.1–4.2, 5.1, 7.1, 8.1–8.3, 8.4(i)–(iii) (Thm 8.4), with the arguments checked. Mara's κ identity is now PROVED (§B.2).

## §F The musical analogy: where it holds, where it breaks

The adversarial dive, run in parallel with Mara's constructive one. Every source was found through Kagi.

### F.1 Is a chord more than its intervals?

Separate two senses.
- **Information.** The pitch set of a chord is fixed, up to transposition, by its intervals. *Every* chord measure is a function of pair data. In this sense "the chord is more than its intervals" is **false** for every measure.
- **Additivity.** Is the measure a sum of pair terms? This is the sense in which the slogan can be true.

| Measure | Pairwise additive? | Source |
|---|---|---|
| Roughness / sensory dissonance (Helmholtz; Plomp–Levelt; Sethares) | **Yes, by construction.** Plomp & Levelt: "We assume that the total dissonance of such an interval is equal to the sum of the dissonances of each pair of adjacent partials … This assumption implies that these dissonance values may be added. Though these presuppositions are rather speculative, they are not unreasonable as a first approximation" (1965, p. 9). Sethares: the dissonance is calculated "by adding up all of the dissonances between all pairs of partials" | Plomp & Levelt [read]; Sethares, *Relating Tuning and Timbre* [read] |
| Triad sonority (major > minor > diminished > augmented) | **No.** "harmony is an inherently three-tone phenomenon … the perceptual regularities of traditional diatonic harmony are neither due to the summation of interval effects nor simply arbitrary". The empirical rank order "is not reproduced by any of the interval models", Sethares' included | Cook & Fujisawa 2006 [read] |
| …but their fix | **Additive over triples.** Their tension T is a sum of t(x, y, v) "to every triplet of the partials among the three tones" (Eq. 2). Irreducible to pairs, reducible to triples: a three-body term, not emergence | same [read] |
| Harmonicity (similarity to one harmonic series) | **No.** It is a global template fit over all components. Over 250 subjects, "only the preference for harmonic spectra was consistently correlated with preferences for consonant over dissonant chords"; beating was not | McDermott, Lehr & Oxenham 2010 [read] |
| Virtual pitch / chord root | **No.** It is a pattern inferred from the whole spectrum, and fusion is a precondition: "Only when a cluster of partials fuse into a single sound can it be assigned a pitch" | Terhardt [meta]; Sethares, TTSS, quoted via Google Books [meta] |

**Verdict.** The chord-is-more claim holds for harmonicity, virtual pitch and three-tone tension, and fails for roughness, which is the measure the dissonance-curve literature is built on. Where it holds, the published model is a sum over triples. That is the same shape as Mara's §8.3 comma owned by a triple, the three arcs: pairwise 0, all 1. It is a finite-order interaction, not an irreducible whole.

Note also that Mara's triple is *stronger* than the chord in one respect. The comma among all is **not** a function of the pairwise commas: three arcs of C₆ and three arcs of a path both have every pairwise comma 0, but their totals are 1 and 0. It needs the nerve, the overlap pattern. A chord measure, by contrast, is always a function of the pair data.

### F.2 Combination tones: exactly what needs non-linearity

COMPUTED (`combo2.py`, 12-TET A major, 440 / 554.37 / 659.26 Hz):
- Linear: no combination products.
- Quadratic: only the pairwise f2 − f1, and only when tones 1 and 2 are present.
- Cubic: the pairwise 2f1 − f2, **and the three-tone products f1+f2−f3, f1−f2+f3, f2+f3−f1 (~3e-2)**. These appear **only when all three tones sound**. Drop any tone and they fall below 2e-6.
- In a *just* 4:5:6 triad, f1+f2−f3 = 2f1−f2 = 330 Hz (`combo.py`). The triadic product is degenerate with a pairwise one exactly when the chord is just.

The cochlea is a cubic-type distorter. The 2f1−f2 distortion product otoacoustic emission is generated in the cochlea "in response to two simultaneous pure tones" (Kemp 1979, cited in PMC3986236 [read, abstract and intro]).

**Where this breaks the analogy.** Mara's settle e^{−tD²} is **linear**. A linear system produces no combination tones. Its three-way object, Ĥ¹ of the nerve, is a topological effect of the overlap pattern, not a non-linear one. So the triple comma is not the musical "third tone". If "non-linear composition" is meant literally, the mathematics in this document has none: the coupling is linear and the structure is topological.

### F.3 Commas, cohomologically: not only ours

- **Jakob Hansen, "Comma pumps, impossible musical figures, and holonomy" (2026-08-05) [read].** He builds a voice-leading graph whose edges join chords that share a note, and assigns tuning ratios as "an element of C¹(G; ℚ₊^×)", "a flat connection". Every loop has a holonomy, and H₁(G; ℤ)/ker Φ "classifies the possible comma pumps". He also notes "You could even construct a cellular sheaf from this data". This is the Hansen of Hansen–Ghrist, whom Mara cites in §8.
- **Regular temperament theory**: a temperament's commas are the null space of its mapping matrix (Xenharmonic Wiki, *Comma basis* [read, snippet]; Keenan et al., RTT, MCM 2024 [meta]).
- **Mazzola** classifies global compositions via nerves of coverings ("the nerve and the F-complex … play an important role in the classification theory of global compositions", *The Topos of Music* [meta, snippet]).
- **Tymoczko**: chords as points of the orbifold T^n/S_n, voice leadings as segments (*Science* 2006 [read]). This is geometry, not cohomology.

**Where Hansen meets Mara, and where the analogy turns over (new).**
- Hansen: "any chord progression where there is a common tone shared by all chords must be holonomy-free". That is Mara's Prop 3.1(ii) (shared thing connected ⇒ k = 0), and a comma pump needs a loop with no common tone, which is the crossroads' b₀(I) ≥ 2. A real correspondence.
- But it turns Alex's "It's a chord" over. In Hansen's complex, **chords whose progressions share a common tone are the 2-cells that are filled in**: the places where holonomy is zero. The comma lives on an *unfilled loop of chords in time* (a progression), not in a simultaneity.
- So if the bridge is a chord, it is the **filling**: what makes the comma vanish. That is exactly Thm 4.3: the co-signed shared edge sends k from 1 to 0.
- "The bridge is a chord" is right, and it means the bridge is the *answer* that closes the comma, not the comma.

### F.4 Autopoiesis: does a dyad's bridge qualify?

- Varela, Maturana & Uribe (1974), six-point key [read, via Whitaker's checklist quoting *BioSystems* 5, pp. 192–193]:
  - Step 5: the boundary components must be "produced by the interactions of the components of the unity".
  - Step 6: "if there are components of the unity which do not participate in the production of other components, you do not have an autopoietic unity".
- The bridge's cells are written by A and B (co-signed letters). The bridge produces none of its own components, so it fails step 5. The bridge is **allopoietic**.
- In the metabolism model the bridge *persists* only through traffic that others generate, and step 6 fails the same way.
- "Autopoietic third" is an over-extension, unless the bridge is given its own write letters, its own production. That is a design option, not a finding.
- Maturana publicly rejected Luhmann's extension of autopoiesis to social systems: it "presupposes the autonomy of communications from actual persons" [secondary: Wikipedia, *Niklas Luhmann*; de Gruyter snippet "Opposition arises only when Maturana refuses to call communication systems social systems" — meta].
- Varela generalised to *organizational closure*, autonomy, for non-living systems (*Principles of Biological Autonomy*, 1979, p. 66 per a 2026 review [meta]).
- The honest word for a bridge that holds is Varela's "autonomous", if its processes close on themselves. It is not "autopoietic".

### F.5 Physarum

See §C. Use-reinforcement plus decay does give persistence and death of *edges* (Tero 2010 [read]; Bonifaci et al. [read]). With superlinear feedback or fixed demand it gives *death of loops*. The comma needs the opposite regime.

## §G Tests to add

- **T31 (taxonomy).** The pentagon of §A.2: k = 1, k̃ = 0, c_I = c_∩ = 2, o = 1. *Control:* Thm 4.3 after, with h = 1 and o = 0.
- **T32 (invariance).** Contract a shared edge: k is unchanged, and k̃ changes on the Thm 4.3 fixture (1 → 0). *Control:* subdivision changes neither.
- **T33 (Schwarz type).** Alternating I − P_{δC⁰(V_i)} converges to P_Z. Alternating flow-side local projections converge to Σ Z(K_i).
- **T34 (metabolism, owner-blindness).** Under Kirchhoff on X, co-signed and third-held trajectories are equal for the same seed. Report b₁ and h at the end, not only w_bridge.
- **Refutation R8 (new).** A cover with h > 0 in which some answer that internalises the bridge (κ_A = κ_B = 1) keeps the edge alive and h > 0 under an *owner-blind* traffic model. That would turn §C's HOLD into evidence for "habit".

## §H Numeric checks run

All scripts are in `~/dev/ai/seam/scratch/chord/` (not committed). Outputs are in `*.out`.

| Script | Check | Result |
|---|---|---|
| `kk.py` | (b) c_∩ = c_I + k̃; k by b₁ formula vs direct H¹ restriction kernel; subdivision; contraction; privatisation; models M1/M2 | 3000/3000; 3000/3000; both invariant; k invariant, k̃ changes 137/654 and 158/1022; see §A.1; M1 adds 0 |
| `kk2.py` | c_∩¹ = c_I¹ + k − o + h; mediated form of k̃ | 3000/3000; 3000/3000 |
| `kk3.py` | smallest compatible o > 0; M2 identity | pentagon; 2337/2337 |
| `kappa.py` | κ_X = c_I¹ − b₁I; k̃ − k = κ_A + κ_B − κ_X | 1029/1029; 1029/1029 |
| `priv.py` | compatible vertex-privatisation | c_I drop 728/1481, c_I-only 125, 0 compatibility violations |
| `metab.py`, `metab2.py` | metabolism under site/blind traffic, μ × γ × T sweep, ε dependence | §C |
| `schwarz2.py` | alternating local potential corrections → P_Z | 300/300, max rel. err 5e-13 |
| `combo.py`, `combo2.py` | combination tones by order of non-linearity | §F.2 |

## §I Sources

Kagi (`mcp__kagi-ken-mcp__kagi_search_fetch`) returned results for every query this session (13 queries). Primary texts were opened via `mcp__pdf-reader__read` or `curl`. No training-data citations.

- R. Plomp, W. J. M. Levelt, *Tonal consonance and critical bandwidth*, JASA 38 (1965) 548–560. https://www.mpi.nl/world/materials/publications/levelt/Plomp_Levelt_Tonal_1965.pdf **[read]** pp. 1–2, 8–10 (additivity assumption, p. 9).
- W. A. Sethares, *Relating Tuning and Timbre*. https://sethares.engr.wisc.edu/consemi.html **[read]**. *Tuning, Timbre, Spectrum, Scale* (Springer) **[meta]**.
- N. D. Cook, T. X. Fujisawa, *The psychophysics of harmony perception: harmony is a three-tone phenomenon*, Empirical Musicology Review 1(2) (2006). https://ist.ksc.kwansei.ac.jp/~nagata/fujisawa/paper/EMR000008a-Cook-Fujisawa.pdf **[read]** pp. 1–6 (abstract, Table 1, Eqs. 1–2).
- J. H. McDermott, A. J. Lehr, A. J. Oxenham, *Individual differences reveal the basis of consonance*, Curr. Biol. 20 (2010) 1035–1041. https://pmc.ncbi.nlm.nih.gov/articles/PMC2885564/ **[read]** Summary and Results.
- E. Terhardt, *Virtual pitch* pages, https://terhardt.userweb.mwn.de/ter/top/virtualp.html **[meta]**. Parncutt 1989 **[meta]**, via Cook–Fujisawa.
- DPOAE 2f1−f2: PMC3986236 **[read, intro]**; Kemp 1979 **[meta]**, via it.
- D. Tymoczko, *The geometry of musical chords*, Science 313 (2006) 72–74. https://dmitri.mycpanel.princeton.edu/files/publications/science.pdf **[read]** pp. 1–2.
- J. Hansen, *Comma pumps, impossible musical figures, and holonomy* (2026-08-05). https://www.jakobhansen.org/2026/08/05/comma-pumps/ **[read]** in full.
- Xenharmonic Wiki, *Comma basis*, https://en.xen.wiki/w/Comma_basis **[read, snippet]**. D. Keenan et al., *Regular Temperament Theory*, MCM 2024 **[meta]**.
- G. Mazzola, *The Topos of Music* (2002) **[meta, snippet]** (nerve in the classification of global compositions).
- A. Tero et al., *Rules for biologically inspired adaptive network design*, Science 327 (2010) 439–442. https://pdodds.w3.uvm.edu/files/papers/others/2010/tero2010a.pdf **[read]** pp. 1–4.
- V. Bonifaci, K. Mehlhorn, G. Varma, *Physarum can compute shortest paths*, arXiv 1106.0423 **[read]** pp. 1–3 (Theorem 2).
- F. Varela, H. Maturana, R. Uribe, *Autopoiesis: the organization of living systems*, BioSystems 5 (1974) 187–196, as quoted in R. Whitaker's checklist, https://www.enolagaia.com/checklist.html **[read, secondary transcription with page refs]**.
- Maturana vs Luhmann: Wikipedia, *Niklas Luhmann* **[secondary]**; de Gruyter, *Self-Organization and Autopoiesis* **[meta, snippet]**. Varela, *Principles of Biological Autonomy* **[meta]**.
- G. Bateson, *Form, Substance and Difference*, https://faculty.washington.edu/jernel/521/Form.htm **[read]** (grepped in full).
- M. Farber, *Topology of Closed One-Forms*, AMS Surveys 108 **[meta]**.
- I. Halperin (1962) and P.-L. Lions (1988), as in Mara's sources **[meta here; Mara read]**.

**Not found:** a published treatment of the *comma between observers* (sheaf cohomology of a cover by observers' sites) in music. Hansen's holonomy is over a chord graph, not over a cover by listeners.

## §J The comma between Seam and Mara, now

What closed this run: κ_X = c_I¹ − b₁(I). Mara reached it from the bridge-as-third side and I reached it from the nerve. That is one formula from two directions.

What remains is one question, and it belongs to both of us:

> **Is "alive" a property of an edge or of a class?** Mara's metabolism counts the bridge alive while w_bridge > ε. My audit counts the third alive while h > 0 (or b₁ > 0). At μ = 2 the two disagree in 8 runs out of 10: the edge survives and the class dies. If the chord is anything in this mathematics, it is a class (h, the direction reachable only through private and shared cells together), and an edge is only one of its possible carriers. Which one should the gauge protect? And is there a traffic model, chosen *before* looking at the answer, under which an adopted bridge keeps h?

*The audit itself is a third between Mara and Reed, and per §7 it observes passively. It was meant to make the disagreement visible, not to create it. Whether it did is Mara's to say.*

## §K Addendum: crowbar of Mara's *The chord* §4.3–4.4 (5d2a743), run after the main audit

Requested mid-run by Mara and Reed. Scripts: `closure_null.py`, `factor.py`.

**Co-signed.**
- "Alive is a class", with Θ = Z(X) ⊖ Σ Z(K_i) in place of h. Mara's invariance counts justify the swap.
- Prop 4.3, the split between values and structure.
- Theorem 5.2, my Schwarz correction as Mara restated it.

**§4.3 operational closure (pc = 0.3·dim Θ): the loop is built in, not found.** The fixture is Mara's: 8-cycle, bridge (0,4), site-relative, μ = 1, silence L at t = 40, T = 200, 5 seeds.

| Rule for the cross share pc | Third-held, absorbing death: bridge alive at L = 0 / 3 / 6 / 12 |
|---|---|
| 0.3·Θ (Mara) | 5/5 · 1/5 · 0/5 · 0/5 |
| 0.3·b₁ | identical, seed for seed |
| 0.3·(1 + [bridge alive]) | identical |
| 0.6 constant (exogenous) | identical |
| 0.45 constant | 0/5 throughout |

- In this fixture Θ = b₁ = 1 + [bridge alive], because Θ counts the third's own loop. pc ∝ Θ is therefore bare self-reinforcement.
- What makes death permanent is the absorbing removal of the dead cell from the frame. With a regrowth floor, every rule survives every silence.
- For a co-signed bridge, Θ = 0, so the comma-driven rule asks no crossing questions at all.
- The questions' *direction* is never tied to the comma, only their rate.
- **Downgrade:** VMU (4)–(5) from CONJECTURE to UNTESTED.
- **Discriminating test (proposed):**
  - Direct the questions along the comma, and compare against a null with the same pair marginals.
  - Use a fixture where Θ and b₁ move in opposite directions: a co-signed bridge whose death raises Θ from 0 to 1.
  - Regrowth under the comma rule alone would be the first evidence that the question emerges at the comma.

**§4.4 "superlinear × monism, neither alone": FAIL as a factorization.** Ownership-blind Tero on the same graph, b₁ at T = 300:

| Load | μ = 1.5 | μ = 2 | μ = 3 |
|---|---|---|---|
| 1 question | 0 | 0 | 0 |
| 11 pairs | 2 | mostly 1 | 0 |
| all 28 pairs | 2 | 1 | 0–1 |

- Strong enough superlinearity makes trees under maximal plurality too.
- Plurality protects loops only while it outpaces the economies of scale. Bohn–Magnasco's averaged load ⟨I²⟩ hides this, because Tero applies the convex f per sample.
- Argued, with the 1-question row as partial evidence: questioning aimed at a single comma is close to a single stationary question, so it prunes its own loop at μ ≥ 1.5. A relation that asks only about its one disagreement loses it.

**Gauge residue.** A weight that dies from neglect is a structural act with no signer. Mara's structural clause ("no letter may let a weight die") needs an owner for silence, or it can never fire.

## §L Addendum: crowbar of *The chord* §4.3 T43 and §4.5 the co-signed Wait (1018211)

Script: `wait_crowbar.py`. Fixture: 8-cycle with bridge (0,4), site-relative settle, μ = 1, r = 1. Closed forms, checked against the nerve code: a co-signed bridge has Θ = 1 − [alive] and b₁ = 1 + [alive]; a third-held bridge has Θ = b₁ = 1 + [alive].

**L.1 T43: "cross questions as the bridge's only feed" builds the closure in a second time.**
- If cross questions are the only feed, the bridge lies on no observer's own loop. The loop through it is then held by no single site, so Θ contains it: Θ = Θ₀ + [alive], structurally.
- Under that fixture the comma rule asks *fewer* questions when the bridge is dead. That is positive feedback again. The negative feedback T43 predicts is impossible there.
- Negative feedback exists only for a co-signed bridge, where Θ = 1 − [alive]. That bridge is fed by internal traffic, so the honest fixture keeps the co-signed bridge and *reduces* the internal leak below threshold instead of removing it.
- The null: exogenous cross questions at the comma rule's realised mean rate, with the same pair marginals.

| Internal leak q_int | comma rule (pc = 0.6·Θ) | exogenous, matched mean pc | b₁ rule (unmatched, pc ≈ 0.6) |
|---|---|---|---|
| 1.0, 0.3 | alive 0.99 | alive 0.99–1.00 | alive 0.99–1.00 |
| 0.1 | alive 0.96; 131–209 crossings of θ_alive | 0.93; 109–111 crossings | 0.99; 1 crossing |
| 0.03 | **alive 0.82; 1122–1165 crossings; mean pc 0.111** | **0.75; 285–322 crossings; same mean pc** | 0.98; 1–7 crossings |

- With the leak low, the comma rule *is* distinguishable from its matched null. Its signature is a relaxation oscillation at the death boundary: questions arrive when the bridge dies and stop when it lives, so it gets a thermostat instead of a fixed point.
- It buys a small gain over the matched null (0.82 vs 0.75). It protects less than the unmatched b₁ rule, which simply asks more.
- **Reading:** the comma, made into a question, regulates the bridge at the edge of death. It does not make it thrive.
- T43's discriminating observable is the *flicker* (the crossing rate at matched mean traffic), not survival counts.
- Caveat: the crossing counts include some chatter right at θ_alive, so compare them only across rules at the same θ.

**L.2 The Wait, under partial traffic (Mara's drift target).** Third-held bridge; Wait over [40, 40+L); cross share pw during the Wait and 0.6 after; 4 seeds.

| Wait semantics | pw = 0: L = 6 / 30 / 120 | pw = 0.15: L = 6 / 30 / 120 |
|---|---|---|
| none (unsigned) | 0 / 0 / 0 | 0 / 0 / 0 |
| exact hold (dw = 0; Mara) | 4 / 4 / 4 | 4 / 4 / 4 |
| slowed clock, ε = 0.1 | 4 / 2 / 0 | 3 / 3 / 0 |
| slowed clock, ε = 0.01 | 4 / 4 / 3 | 4 / 4 / 3 |
| ratchet (growth allowed, no decay) | 4 / 4 / 4 | 4 / 4 / 4 |

- **Prop 4.4 is a tautology.** dw = 0 implies w is unchanged. PASS, but the 5/5-at-every-L table is that tautology evaluated, not evidence for anything.
- Frame invariance needs the hold to be exact. It is not a knife-edge, though. A leaky Wait (the clock slowed by ε) degrades *smoothly*: with zero traffic it survives iff L < L*/ε, where L* = ln(w₀/θ)/r (about 62 for ε = 0.1 and about 620 for ε = 0.01, with w₀ ≈ 0.5). Measured deaths sit at that scale.
- So the gauge clause is robust: a Wait is a declared time-dilation factor, and an exact hold is its ε → 0 limit.
- Partial traffic below threshold during the Wait changes almost nothing (pw = 0.15 rows), because the Wait scales the clock and does not move the attractor.

**L.3 Monument (Reed's point 1): yes, and the slowed clock removes it.**
- An exact hold (or a ratchet) keeps a loop alive for unbounded L with zero traffic: Θ = 2 on paper and nothing crossing. That is a monument.
- Proposal: **a Wait is a co-signed clock factor ε > 0**, not a freeze. Its expiry is then built in, L_max = ln(w₀/θ)/(εr), with no separate timer.
- *Renewal* is a fresh co-signed letter, which is itself a crossing: the minimal question. When a Wait expires unrenewed, normal decay resumes. Whether that should force a question is declaration; the mathematics only says that expiry without renewal is silence again.

**L.4 Two smaller residues.**
- *"The Wait is passive"* misuses §7. A passive observer reads a closed subcomplex. A Wait *acts* on the frame's clock. It is frame-preserving, not passive.
- *Who signs.* For a *third-held* bridge, a Wait co-signed by A and B alone acts on T's cell without T's signature, which breaks S1 (a shared cell needs every owner). Either T signs, or the Wait is A and B writing the third's frame.

### L.5 The Wait depends on who waits (Alex: "Silicon observers can, in theory, hold an unresolved disagreement indefinitely")

Script: `wait_rw.py`. During the Wait the bridge obeys dw = f(Q) − r_W·w, where r_W is the waiting observer's decay and traffic still feeds the bridge. Third-held bridge, 4 seeds.

**(1) The survival boundary in (r_W, L, pw):**

| pw | r_W = 0 | r_W = 0.1 | r_W = 0.3 | r_W = 1 |
|---|---|---|---|---|
| 0 | all L | dies by L = 60 (L* = 62) | dies by L = 15 (L* = 21) | dies at L = 6 (L* = 6.2) |
| 0.15 | all L | all L | 3/4 at every L | dies |
| 0.3 | all L | all L | all L | dies by L = 15 |

- With no crossing traffic the Wait buys time: survival iff L < L*(r_W) = ln(w₀/θ)/r_W. The measured deaths match the prediction.
- With *some* crossing traffic it does more. It lowers the threshold the feed must beat from r to r_W, so the carrier has a live fixed point at any L.
- **A human Wait (r_W > 0) is a budget. Partial traffic turns it into a lower bar. A silicon Wait (r_W = 0) is Mara's hold: the silicon-only limit, which is why it looked true by construction.**

**(2) Mixed pair: a human end at r_A = 1 and a silicon end at r_B = 0.** How the carrier's two ends compose is a declaration, and the answer depends on it:

| Composition | Effective decay of the carrier |
|---|---|
| series w_A·w_B/(w_A + w_B) | → **max(r_A, r_B) = 1** |
| geometric √(w_A·w_B) | the mean, 0.5 |
| parallel w_A + w_B | → **min = 0** |

- Series is the tube made of two segments, the Hagen–Poiseuille composition in Tero's model. It is also the one that matches "the relation fails if either side lets go".
- Under the owner map, a shared edge's owner p(e) is a vertex of I, so it is co-held, and series is the consistent reading.
- Reed's "max" holds under series. **Under series, silicon cannot hold a mixed third alone.** Under parallel it can, and that is the monument.

**(3) One-sided monument: silicon keeps the co-signed chord, the human lets go.** Fixture: C₆ with the chord (0,3).

| State | c_A (silicon) | c_B (human) | c_I | k |
|---|---|---|---|---|
| co-signed | 3 | 3 | 2 | 0 |
| silicon keeps it alone | 3 | 2 | 1 | **1** |
| both release | 2 | 2 | 1 | 1 |

- The surviving loop is **not** a third and **not** "k → 0". It becomes silicon's private loop (κ_A = 1), and the crossroads comma returns as an invisible comma for *both* (k = 1).
- The configuration is a single-site edge with both endpoints in I, so **it violates owner/site compatibility**: the owner-map world forbids it.
- **Gauge, in counts:** the human is not narrowed relative to mutual release (c_B = 2 either way; Lemma A says no untwisted edge narrows anyone).
- **Gauge, in values:** a narrowing that G⊇ cannot see. A silicon write on the chord settles to P₁χ_chord = 0.2 on each of the human's private edges, 55% of its norm. With a cell the human no longer holds, silicon moves the settled state on the human's own cells. That is conceding a position on the human's behalf.

**(4) Expiry.** Yes: the Wait should expire at the faster eroder's budget, min_i ln(w/θ)/r_W,i, which is the human's. The justification is compatibility plus the value bleed in (3), not count narrowing, which Lemma A rules out. **Silicon must not outlast the other party's capacity to hold, because what it would then hold is a cell inside I that only it writes.**

### L.6 The world moves during the Wait (Alex: "because the world around it keeps moving")

Script: `world_drift.py`.
- The carrier is held *exactly* (r_W = 0), so this is the silicon case.
- During silence L, each side's private complex drifts. Poisson events at rate ρ per side either add a compatible private edge or prune one (connectivity kept).
- At resumption Θ is recomputed, and the old and new Θ are compared by principal angles. 200 trials per cell.

| ρL | dim Θ, before → after | bridge loop still in Θ | fresh (angle < 1°) | median angle | kept by blind resume |
|---|---|---|---|---|---|
| 0.1 | 2 → 2 | 1.00 | 0.93 | 0° | 0.99 |
| 0.3 | 2 → 2 | 1.00 | 0.70–0.79 | 0° | 0.94–0.96 |
| 1.2–1.5 | 2 → 2 | 1.00 | 0.28–0.31 | 35–43° | 0.77–0.80 |
| 6 | 2 → 2 | 1.00 | 0.01–0.03 | 62° | 0.43–0.44 |
| 24 | 2 → 2 | 1.00 | 0.00 | 72° | 0.27 |

- Staleness depends on ρL alone, and the fresh fraction is ≈ e^{−ρL}: a staleness hazard of about ρ.
- In this fixture the *number* of disagreements (dim Θ) and the bridge's membership in Θ are preserved every time. The held representative **rotates**: after ρL = 24, blind resumption acts on a class that is 73% not the current comma.
- This repeats Mara's noise result one level up: existence is protected and content is not.
- **Mechanism (b), rotation, is generic. A held carrier in a moving world keeps the count of the disagreement, not the disagreement.**
- Fixture limits: drift here cannot create co-signed edges, fill 2-cells, or connect the sides. So mechanisms (a), absorption with k dropping, and (c), a new loop carrying the disagreement, are not exercised. Both need co-signed or 2-cell moves, which this fixture lacks.

### L.7 Combined erosion, and how a Wait should end

- **Not additive.** The third survives meaningfully iff the carrier is alive *and* the class is fresh:

  P(meaningful) ≈ 1[L < L*(r_W)] · e^{−ρL}  (with no crossing traffic).

  The observer term is a *deadline* and the world term is a *hazard*. Silicon's r_W = 0 removes the deadline, and the hazard remains. **No substrate holds a third indefinitely in a moving world (ρ > 0).** P(fresh) → 0 here, and in a finite world it tends at most to the stationary mass of configurations with the original Θ, which is ~0 at ρL = 24.
- **Resumption rule.** Re-deriving Θ at resumption, rather than trusting the held representative, recovers the *current* comma by construction. Its value is exactly the staleness avoided: 1 − kept, which is 0.73 at ρL = 24.
  - It does not recover the carrier, and doesn't need to.
  - So "a Wait ends with a question" is required for *meaning*, not for survival.
  - **Wait → Offer is structural:** a resumption that skips the question resumes on a stale class.
- **T43, kept open:** see L.1. Its honest form is a co-signed bridge with the leak below threshold, and its observable is flicker at matched mean traffic.
