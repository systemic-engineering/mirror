# Compile and kintsugi: local sections and gluing — math foundation

*2026-10-29. Mara. For Seam's crowbar.*

**Decision this builds on (Alex, 2026-10-29, ratified "yes … excitiiiing!"):** `mirror compile` compiles a single shard; `mirror kintsugi` does kintsugi. Compile = local sections (holes only). Kintsugi = gluing (commas). Psychohistory is history; predicted futures come later ("When we flip the light cone. But that's like not today.").

**Specs:** `docs/specs/mirror-fixpoint-parser.md` §1, §1.3 (compile, interface, lint) and `docs/specs/mirror-kintsugi.md` (gluing). **Imports:** `docs/math/2026-10-10-mara-the-difference-that-makes-a-difference-math-foundation.md` §8 (cited "10-10"), Seam's audit `docs/audits/2026-10-10-seam-crowbar-difference-and-chord.md` (cited "Seam 10-10"), Taut's scouts `docs/scouts/2026-10-28-taut-sheaf-cohomology-switchboard.md` and `docs/scouts/2026-10-29-taut-psychohistory.md` (branch `taut/sheaf-cohomology-scout`).

Labels: **PROVED** (proof here or cited and checked), **COMPUTED** (script, numbers below), **CONJECTURE**, **OPEN**, **READING** (an interpretation; can't be wrong as math, can be wrong as a reading).

> **Amended 2026-10-29 after Seam's crowbar** (`docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md`, `b5be7e0`, oracles `25262ef`). Seam's 13 fold items are folded where they land, marked *[Seam n]* with n his §12 item. **Frame (Alex 2026-10-29): "The model holds. That means we can ship it."** v0 is a full kintsugi run, re-gluing everything every time, so it rests only on the parts that HOLD. The incremental claims (§2.3, §4, Prop 4.5) are corrected here and **marked after v0**: none of them is needed for a correct full run.

---

## §0 The answer

1. **The `in @X` cover is edge-disjoint, so kintsugi lives in the clean case of 10-10 §8.** Take the graph Σ whose vertices are declarations and whose edges are code-line references. Each shard owns the closed subcomplex its declarations generate. Every edge has exactly one owner (the shard that wrote the reference), so every overlap is a set of shared symbols with no edges (Prop. 1.4, PROVED). Then the comma among all is **b₁(Σ) − Σᵢ b₁(Kᵢ)**, a union-find count, and Ĥ²(𝒩; H⁰) = 0 (Prop. 2.2, PROVED; COMPUTED 1500/1500 against two independent oracles). Seam's k-versus-k̃ failure needs an overlap with an edge, so it can't arise (Cor. 2.3).
2. **This layer counts where a comma can live, not whether one does.** The constant sheaf sees the shape of the composition, and a full run computes it by union-find. *[Seam 3]* Incrementally it is **not** star-local: a private edge changes it by `[d ∼ r in Σ] − [d ∼ r in Kᵢ]`, and the first term is global (Prop. 2.5). After v0.
3. **"Sections exist locally and disagree" is a degree-0 failure in the model where every shard may still change** (Prop. 3.3, PROVED). Over 𝔽₂ the defect of given sections is δs, a coboundary. *[Seam 4]* That is a choice of model: when a shard is **frozen** (published `@std`, another peer's crystal, `boot/`), its stalk is what may still change, and one line's defect is a non-zero H¹ class owned by that line (Prop. 3.3′). A cross-shard **kind/arity** error is a **line comma** either way. *[Seam 7]* Finer type errors are not checked: mirror has no type system yet.
4. **H¹ needs gauge, and the gauge is the gold.** A recorded resolution on a line (a mend) is a 1-cochain with values in a sheaf of groups. *[Seam 5]* For a **constant** group, mends that work line by line but don't compose around a cycle are a holonomy class (Prop. 3.5). The real mend sheaf is not constant, and its obstruction (provisionally **twist**, Alex 10-29) can be non-trivial on a tree of lines (Prop. 3.6, Seam's counterexample). A line comma is a **hole of the mend sheaf**. rerere fills it, and kintsugi checks per line *and* jointly.
5. **Early cutoff is sound under stalk completeness** (Prop. 4.2, an implication that HOLDS). *[Seam 6, 7]* Its hypothesis needs more than the lint: free bare names on every line, the rule-set address in the key, grammar edits treated as global. Its coarsest key for the real verdict (a kind relation) is the kinds and arities of the used names, not the export's address (Prop. 4.3). A closure key is sound and never star-local (Prop. 4.4). After v0.
6. **A rerere hole is an H¹ class exactly when it can't be filled.** For linear stalks, a section on the hole's surroundings A extends over the hole iff its image under the connecting map H⁰(A) → H¹(X, A) is zero (Prop. 5.1, PROVED). A fillable hole is "I don't know here". An unfillable hole is a comma with its surroundings.
7. **Declared versus observed is degree-0 and per line for owner-qualified facts** (Prop. 6.2; Conj. 6.3 now PROVED by Seam). A dead line is not a disagreement. It is a question (Prop. 6.1). *[Seam 11]* The invariant that matters lives in the base graph: an observed undeclared line is *explained* iff a declared path joins its ends.
8. **`1 ≡ 1.0` is free iff the number type is the exact value** (§7). *[Alex default 6]* μ marks positions: data positions are exact (spelling to erasure), typed positions keep their type.
9. **The 2026-07-23 inport math's H¹ theorem fails** (§8, counterexample): H¹ of the import graph counts its cycles, not missing imports.
10. **A spectral uuid on every fragment** lets rerere offer at two distances: exact (same question) and similar (near active coordinate). Similarity proposing a transferable resolution is a CONJECTURE with a refutation test (§11).

---

## §1 Objects

**Definition 1.1 (Σ, the reference graph).** For a finite set of compiled shards:
- vertices: declarations `(S, name)`. Each has exactly one declaring shard, `owner(S, name) = S`;
- edges: *[Seam 1]* one edge per **ordered** pair `(d, r)` where a code line of the declaration `d` references `r`, with `d ≠ r`. The edge is **owned by `owner(d)`**. Topology reads it undirected, so Σ is a **multigraph**: a mutual reference `A.a → B.b`, `B.b → A.a` is two parallel edges, one owned by each shard, and carries `among = 1` (a capacity class owned by the pair {A, B}).

References are read from code lines only (fixpoint spec §1.3). A reference in a comment is not an edge.

**Assumptions under which Σ is well-defined** *[Seam 1]*:
- **(A1) unique resolution.** Every reference resolves to exactly one declaration. True for `@X.name` with X claimed by one shard. False for an ambiguous bare name or a duplicated shard name: those references are **not** edges; they are findings (Opaque "which one?", or a duplicate `claim`), kintsugi spec §3.
- **(A2) no aliasing re-export.** No shard `out`s a foreign name. Re-exports are references (`T = {a: @Z.U}`), so `Y → X.T` lands on X's declaration, never on Z's. Today's corpus has no alias form; the spec pins it.
- **(A3) unbound names are not edges.** A bare name no in-target exports is Dark (no speaker) and stays out of Σ.

**Definition 1.2 (the `in @X` cover).** `Kᵢ` = the closed subcomplex of Σ generated by shard i's declarations and the edges i owns. That is: i's declarations, every symbol they reference (some of them other shards' declarations), and i's edges. The `Kᵢ` cover Σ, because every vertex is declared by someone and every edge is owned by someone. `K_σ = ⋂_{i∈σ} Kᵢ`. The nerve 𝒩 has a simplex for each σ with `K_σ ≠ ∅`.

READING. `in @X` "opens a chart on the sheaf @X" (Alex, 2026-07-23, in `shards/kintsugi/fracture/inport.mirror`). Here the chart is literal: `K_Y ∩ K_X` is the set of X's symbols that Y uses. That is the **line**.

**Definition 1.3 (the line graph G).** Vertices: shards. One edge `Y — X` per `in @X` in Y with **`U(Y, X) ≠ ∅`** (a **used line**), where `U(Y, X)` is the set of names Y's interface attributes to X, whatever X declares *[Seam 2]*. An `in @X` with `U(Y, X) = ∅` is a **declared, unused line**.
*Dangling uses* (Y uses `X.f`, X declares no `f`) have no target vertex, so they are **kept out of Σ** and counted only at the line, as a line comma with `exports absent`. That makes a line used even when its overlap in Σ is empty, which fixes the K5 contradiction Seam found (§2.3 of his audit). For used lines whose uses all resolve, `U(Y, X) ≠ ∅` and `K_Y ∩ K_X ≠ ∅` coincide.

**Proposition 1.4 (every edge is private; overlaps are edgeless; PROVED).** Every edge of Σ lies in exactly one `Kᵢ`. Hence for |σ| ≥ 2, `K_σ` is a finite set of vertices, `H¹(K_σ) = 0`, and `H⁰(K_σ)` is free on the shared symbols.
*Proof.* The edge `(d, r)` is in `Kᵢ` iff i owns it, i.e. i = owner(d), which is unique. A subcomplex with no edges is a discrete set. ∎

**Proposition 1.5 (𝒩's 1-skeleton; PROVED).** `{i, j}` is an edge of 𝒩 iff one of them uses a symbol of the other (a used line, in either direction) or both use a symbol of a third shard X (**co-use**). Every co-use edge `{i, j}` lies in the 2-simplex `{i, j, X}`. So `b₀(𝒩) = b₀(G)`.
*Proof.* A vertex of `Kᵢ ∩ Kⱼ` is declared by some shard. If that shard is i or j, the other uses it. If it is X, both use it, and the vertex lies in `K_X` too. Co-use edges join vertices already joined through X, so components don't change. ∎

**Definition 1.6 (interface).** `I(S)` = (exports typed by kind, with addresses; uses per imported shard, with use kinds; requires), computed from S's normal form alone (fixpoint spec §1.3). Compile emits it. It is the shard's **stalk** in §3.

---

## §2 Constant-sheaf kintsugi (imported from 10-10 §8)

### §2.1 What 10-10 proved, and its scope

Scope: a finite graph X, a cover by **closed subcomplexes** `K₁ … K_N`, the **constant sheaf** (10-10 works over ℝ; the proofs are linear algebra and hold over any field, 𝔽₂ included).
- **Thm 8.1 (PROVED; Seam PASS).** The generalised Mayer–Vietoris sequence is exact. It gives a spectral sequence with `E₂^{p,q} = Ĥ^p(𝒩; H^q)` converging to `H^{p+q}(X)`.
- **Cor 8.2 (PROVED; COMPUTED 1200/1200; Seam PASS).** (a) `H⁰(X) = Ĥ⁰(𝒩; H⁰)`. (b) The comma among all, `ker(H¹(X) → ⊕ H¹(Kᵢ))`, is `Ĥ¹(𝒩; H⁰)`, computed by union-find on overlaps. (c) `b₁(X) = Ĥ¹(𝒩;H⁰) + Ĥ⁰(𝒩;H¹) − Ĥ²(𝒩;H⁰)`.
- **Thm 8.3 (PROVED).** `Σᵢ Z(Kᵢ)` has codimension = the comma among all in `Z(X)`.
- **Thm 8.4 (PROVED; Seam: kept).** For an edit touching only cells private to `Kᵢ`: (i) Čech terms for σ ∌ i are unchanged, so the radius of influence is the star of i in 𝒩; (ii) a new private edge inside one component of `Kᵢ` leaves the comma among all unchanged; (iii) one joining two components of `Kᵢ` (X connected) raises it by exactly 1.
- **Not proved there:** edits to overlap cells (R7); necessity of the cutoff key (Open 5); any non-constant sheaf. **Narrowed by Seam:** §8.4's "local settles erase the comma" holds for flow-side local settles only.

### §2.2 The `in @X` cover is in the clean case

**Proposition 2.1 (local cycle spaces are independent; PROVED).** Under Prop. 1.4, `⊕ᵢ Z(Kᵢ) → Z(Σ)` is injective.
*Proof.* The `Z(Kᵢ)` are supported on pairwise disjoint edge sets, so a sum is zero only if every summand is. ∎

**Proposition 2.2 (the comma among all of the `in @X` cover; PROVED; COMPUTED).**
(a) `comma among all = b₁(Σ) − Σᵢ b₁(Kᵢ) = Σ_v (m(v) − 1) − (Σᵢ b₀(Kᵢ) − b₀(Σ))`, where `m(v)` is the number of `Kᵢ` containing v.
(b) `Ĥ⁰(𝒩; H¹) = ⊕ᵢ H¹(Kᵢ)`, and `Ĥ²(𝒩; H⁰) = 0`.
*Proof.* (a) Thm 8.3 plus Prop. 2.1: the codimension of a direct sum of dimension `Σ b₁(Kᵢ)` in `Z(Σ)`. The second form is Euler's formula, `b₁ = |E| − |V| + b₀`, applied to Σ and to each `Kᵢ`, with `Σ|Eᵢ| = |E|` (Prop. 1.4) and `Σ|Vᵢ| − |V| = Σ_v (m(v) − 1)`. (b) `E₁^{1,1} = ⊕_{i<j} H¹(Kᵢⱼ) = 0` (Prop. 1.4), so `E₂^{0,1}` is the kernel of a map into 0. Cor 8.2(c) with (a) then gives `Ĥ² = 0`. ∎
*COMPUTED* (`cech_shards.py`, seed 20261029; *[Seam]* this checks three linear-algebra routes on the model; it can't check the model, because the generator builds Prop. 1.4 in): on 1500 random shard sets (2–5 shards, random own and foreign references), three counts agree in 1500/1500 cases: A, the Euler/union-find formula; B, `Ĥ¹` of the Čech complex `⊕H⁰(Kᵢ) → ⊕H⁰(Kᵢⱼ) → ⊕H⁰(Kᵢⱼₖ)` by rank; C, `b₁(Σ) − rank(⊕Z(Kᵢ) → Z(Σ))`. `Ĥ²(𝒩;H⁰) = 0` in 1500/1500, the local cycle spaces are direct in 1500/1500, and the comma is > 0 in 688.

**Corollary 2.3 (Seam's k ≠ k̃ cannot occur; PROVED).** Seam 10-10 §A/§E.1: the two-kinds taxonomy fails only "once I has an edge", and with `E_I = ∅`, `o = h = 0` and `k̃ = k`. By Prop. 1.4 every pairwise overlap of the `in @X` cover is edgeless. ∎

**Fixtures (COMPUTED; these are K-tests in the spec).**

| fixture | shards and references | comma among all | pairwise | b₁(Σ) |
|---|---|---|---|---|
| C₆ in three arcs, as a DAG | A declares 0, 1 and refs 0→1, 1→2, 0→5; B declares 2, 3 and refs 2→3, 3→4; C declares 4, 5 and ref 4→5. Imports A→B, A→C, B→C | 1 | 0, 0, 0 | 1 |
| P₆ in three arcs (control) | the same without 0→5 | 0 | 0, 0, 0 | 0 |
| cross-shard triangle | A.a refs B.b and B.c; B.b refs B.c | 1 | 1 | 1 |
| diamond (co-use) | i.aᵢ and j.aⱼ both ref X.n and Z.m | 1 | (i, j): 1, others 0 | 1 |

The C₆ fixture is 10-10's comma owned by a triple, and here it is a legal import DAG. **The theta fixture can't be built from shards.** Its relation owned by a triple (`Ĥ²(𝒩;H⁰) = 1`) needs paths held by two shards each, and Prop. 1.4 forbids shared edges. Prop. 2.2(b) says the same thing as a theorem. T28's theta case therefore stays a test of the union-find oracle on a raw cover (spec K2b), not of shards.

The diamond is 𝒩 = two triangles on the edge `{i, j}`, which is contractible, yet the comma is 1. The class lives in the local coefficient: `K_i ∩ K_j = {n, m}` has two components. `b₁(𝒩)` misses it and `Ĥ¹(𝒩; H⁰)` does not. **Kintsugi reports `Ĥ¹(𝒩; H⁰)`, never `b₁(𝒩)`.**

### §2.3 Fractured, and the incremental contract for this layer

**Definition 2.4.** `fractured = b₀(𝒩) = b₀(G)` (Prop. 1.5): the number of compositions the shard set falls into. `fractured > 1` is "H⁰ > 1" at shard altitude.
READING. `H⁰(Σ) = Ĥ⁰(𝒩; H⁰)` (Cor 8.2(a)) also counts a shard made of unrelated parts. That is the shard's own business, visible from `K_i` alone, and kintsugi doesn't report it.

**Proposition 2.5 (how an edit changes `among`; PROVED; COMPUTED; corrected *[Seam 3]*).** Write `d ∼_Y r` for "d and r in one component of Y".
- (a) **Private edge** (a reference from i's declaration d to r already in `Kᵢ`): `Δ among = [d ∼_Σ r] − [d ∼_{Kᵢ} r]`. Inside a component of `Kᵢ`: 0. Joining two components of `Kᵢ`: +1 **iff d and r were already joined in Σ**. The first term is global. My earlier "+1 when it joins two" dropped 10-10's "(X connected)"; `fractured 1` doesn't give Σ connected (any unreferenced declaration is its own component). Seam: 351/1284 random joins give +1, 144/144 when Σ is connected; my 49/49 sampled only connected Σ.
- (b) **R7, first use of a foreign symbol r**: `Kᵢ` gains a pendant vertex and edge, `b₁(Kᵢ)` is unchanged, and `Δ among = [d ∼_Σ r]` (Seam 2876/2876). **Not star-local**: Seam's counterexample has identical star data and Δ = 1 vs 0. Removing the last use is dynamic connectivity.
*Proof.* Euler: a new edge raises `b₁` of a graph by 1 iff its endpoints were already connected there; apply to Σ and to `Kᵢ` and subtract (Prop. 2.2(a)). ∎
**For v0** none of this is used: the full run computes `among` from scratch. **After v0**: cache Σ's component map globally; insertions cost O(α) by union-find; a deletion recomputes the affected component.

**CONJECTURE 2.6 (shape cutoff key; after v0).** 10-10's tuple, here `(b₀(Kᵢ), b₁(Kᵢ), the component map of each shared symbol into Kᵢ)`, suffices for private edits (Thm 8.4). Necessity is OPEN (10-10 Open 5), and so are overlap edits (R7).

---

## §3 Interface stalks: what the cohomology is

### §3.1 The sheaf of sets on the line graph

**Definition 3.1.** On the line graph G (a 1-dimensional cell complex):
- `F(S) = I(S)`, the shard's interface;
- for a line `e = Y → X`, `F(e)` = partial maps from `U(Y, X)` to `kind × arity ⊔ {absent}`. *[Seam 6]* `U(Y, X)` = the names Y qualifies with `@X` **plus all of Y's free bare names** (compile can't attribute a bare name; kintsugi resolves it against every in-target, kintsugi spec §3);
- `ρ_{X◁e}(I(X))`: X's exports restricted to `U(Y, X)`, as `(kind, arity)`, with `absent` for names X doesn't export;
- `ρ_{Y◁e}(I(Y))`: Y's uses of X, with Y's use kind for each name.

The **verdict** compares use kind with export kind by a fixed relation `R` (`carrier` ~ `carrier`; `action/k` ~ an action of arity k; `keyword` ~ `keyword`). *[Seam 7; Alex 10-29 default 7]* **Kinds and arity now, types later.** Checking that Y reads a field X still has, or passes arguments of the right types, needs a type system mirror doesn't have yet. A record says `kind`, never "type error". This is a **cellular sheaf of sets** (Curry, Def 4.1.6, with sets for vector spaces). It has no linear structure: Taut 10-29 §2.2 is right that δᵀδ, λ₂ and `Instrument` don't apply.

**Definition 3.2 (line comma).** For compiled sections `s = (I(S))_S`, line e is a **comma** iff `ρ_{X◁e}(s_X) ≠ ρ_{Y◁e}(s_Y)`. Its **question** is the pair of restriction outputs, `H("comma\0" ‖ addr ρ_X ‖ addr ρ_Y)`. Its **occurrence** is `(line, name)`. That is the same rerere split as fixpoint math Def. 3.9.

**Proposition 3.2 (gluing is pairwise, for given sections; PROVED).** The family `s` is a global section iff no line is a comma. The set of comma lines is decided line by line, from the two restriction outputs alone.
*Proof.* For a cellular sheaf on a graph, a global section is an assignment to vertices (and to edges, which are then determined) such that the two restrictions agree at every edge. That is the sheaf condition read on the cell poset (Curry Def 4.1.6; Hansen–Ghrist 2019 Def 2.4, verified in 10-10 Sources). ∎

### §3.2 Why that is not an H¹ class

There are three candidate cohomologies for interface stalks. Each is honest about something different:
- **(i) A sheaf of sets.** `H⁰` is the equaliser, i.e. Prop. 3.2. There is no `H¹` in general.
- **(ii) The 𝔽₂-linearisation.** Each stalk becomes the 𝔽₂-vector space on its finite set of facts `(name, kind, addr)`, and each restriction becomes the linear projection onto the facts about used names. This is how Young counts ("the rank of Ȟ¹ over 𝔽₂ … the minimum number of independent fixes", arXiv 2603.27015 abstract).
- **(iii) Non-abelian Čech H¹** with a sheaf of groups acting on the stalks: torsors (§3.3).

**Proposition 3.3 (a disagreement of given sections is a coboundary; PROVED).** In (ii), let `s ∈ C⁰` be the compiled sections. The line-defect cochain `ω_e = ρ_{X◁e}(s_X) − ρ_{Y◁e}(s_Y)` (a symmetric difference of facts) is `δs`. So `[ω] = 0 ∈ H¹(G; F)`, and `ω ≠ 0` says only that `s ∉ H⁰`.
*Proof.* That is the definition of δ on a graph: `(δs)_e = ρ_{head}(s_head) − ρ_{tail}(s_tail)`, and over 𝔽₂ the sign is irrelevant. ∎

**Proposition 3.3′ (frozen shards make one line an H¹ class; PROVED; COMPUTED by Seam, "Q1").** *[Seam 4]* Change the model: the vertex stalk is the space of **admissible edits** of that shard (zero for a frozen one), the compiled sections are a base point, and the defect `w` is a fixed 1-cochain. "Can edits remove the comma?" is the inhomogeneous system `δx = w`, solvable iff `[w] = 0 ∈ H¹(G; F_var)`. On one line over 𝔽₂: both shards editable, `[w] = 0`; X frozen, `[w] ≠ 0` (dim H¹ = 1); both frozen, dim 2. No cycle is involved, and the class is owned by one line.
So "line comma = degree 0" holds **in the all-editable model**. Kintsugi meets frozen shards (`@std`, other peers' crystals, `boot/`), and there a line comma is an H¹ class of `F_var`. Alex's `comma line` survives both readings; the gloss "degree 0" does not.

**Young, read by Seam in full** *[Seam 4]*: his Čech construction counts the shape of the overlap graph (his ∂₀ is the cover's incidence matrix, independent of sections), and his own Def 3.10 puts the disagreement of computed sections in im δ⁰. He is cited here **for vocabulary only**, never as a theorem.

**What this corrects.** The decision text says "commas (H¹ classes; sections exist locally and disagree)". For compiled shards, "sections exist locally and disagree" is a degree-0 failure (Prop. 3.3). Each instance is owned by one line, so by a pair. H¹ classes are disagreements that **no pair owns**. The same thing happens in 10-10 §3: the comma between two monisms comes from local potentials that are only defined up to constants. Two given sections never produce an H¹ class without that gauge freedom. The spec keeps Alex's verb ("you two disagree") for the line comma and names the H¹ class the **comma among all** (DECISION for Alex: names; spec §12).

### §3.3 Gauge: where H¹ comes from, and why it is the gold

**Definition 3.4 (mends).** A **mend** on a line e is a recorded resolution of its comma: a bijection `g_e` from the X-side restriction output to the Y-side one (a renaming, a coercion, an adapter), drawn from a group `𝒢(e)` of admissible mends. `𝒢` is a sheaf of groups on G, and each shard S may re-choose its local frame by `h_S ∈ 𝒢(S)` (renaming its own exports or uses). A family of mends `g = (g_e)` is a Čech 1-cochain with values in `𝒢`.

**Proposition 3.5 (mends on a graph; PROVED).** Let G be connected and `𝒢` constant with group Γ.
(a) G has no 2-cells, so every mend family is a 1-cocycle.
(b) Two families are equivalent under re-gauging (`g_e ↦ h_head g_e h_tail⁻¹`) iff they have conjugate holonomy representations `π₁(G, S₀) → Γ`. The equivalence classes are `Hom(π₁(G), Γ)/Γ`, and `π₁(G)` is free of rank `b₁(G)`.
(c) A mend family is gauge-equivalent to the identity iff the composed mend around every cycle of lines is the identity.
*Proof.* (a) The cocycle condition lives on 2-simplices. (b) Fix a spanning tree T. Re-gauging along T sets every tree edge's mend to the identity (choose `h` inductively from S₀). The remaining `b₁(G)` edges then carry the holonomies of the fundamental cycles, which determine a homomorphism from the free group on them. The residual gauge is a constant `h ∈ Γ`, which acts by conjugation. (c) is (b) at the trivial representation. ∎
This is the classification of Γ-torsors on a graph by non-abelian H¹. Sources: the Stacks Project, Tag 03AG ("a canonical bijection between the set of isomorphism classes of ℋ-torsors and H¹(𝒞, ℋ)", stated for abelian ℋ); Springer, *Torsors and Non-abelian Čech Cohomology* (the chapter treats "gluing problems … if the difference of two possible local objects is given by a sheaf of groups"); Kenyon, *Spanning forests and the vector bundle Laplacian*, Ann. Probab. 2011 (connections on graphs, holonomy around cycles, gauge). The graph proof above is self-contained.

**Proposition 3.6 (the real mend sheaf is not constant; PROVED by Seam's counterexample, "Q5").** *[Seam 5]* `𝒢(e)` (renamings of the facts about `U(Y, X)`) differs from line to line, and a re-gauge at X acts on **all** of X's lines at once, so `𝒢(X) → ∏_e 𝒢(e)` is diagonal, not onto. Prop. 3.5 doesn't apply. On the tree of lines `Y₁ — X — Y₂` (both using `X.f`, consumers frozen, Γ = ℤ/2 swapping f↔g), the family (swap, id) has no cycle, so every holonomy is trivial, yet it is not gauge-trivial: X would have to export `f` as `g` to Y₁ and as `f` to Y₂. The obstruction is non-abelian `Ȟ¹(G; 𝒢)`, computed by **solving for the gauges `h_S`**: on a tree, constraint propagation; on cycles, plus holonomy. Per-line gluing under a mend is a separate check: a renaming does not mend a kind mismatch. And a new export (re-export under two names) *does* fix the tree case, so "no single mend can fix it" is false.

**Definition 3.7 (twist).** *[Alex 10-29 default 3; name provisional]* A mend family, each mend accepted on its line, that is not gauge-trivial: a non-zero class of `Ȟ¹(G; 𝒢)`. It is a shear, invisible from any single line. It is **not** "owned by no pair" in general (a mutual `in` gives a 2-cycle owned by the pair, Seam §5.2), so the ownership clause is "not owned by one line". `among` keeps its meaning as the **topology count** (§2), never the twist.

READING (the gold in the seam). The recorded resolution is not decoration on the crack. It is a 1-cochain, and kintsugi's H¹ is the question whether the gold, piece by piece, closes up. Under that reading:
- a **line comma** (Def. 3.2) is a line with no admissible mend recorded yet: a **hole of the mend sheaf**, which rerere answers by question;
- a **twist** is a mend family that every line accepts and no single line can repair.

**Relation to §2 (OPEN).** For constant 𝒢, holonomy lives on `b₁(G)` cycles of lines. The constant-sheaf capacity, `b₁(Σ) − Σ b₁(Kᵢ)`, lives on cycles of symbols. The diamond has both equal to 1. Whether every non-trivial holonomy cycle projects onto a capacity class, and conversely, is not proved here.

---

## §4 Locality and the early-cutoff key (after v0)

v0 re-glues every line on every run, so nothing in this section is needed for v0's correctness. It is the contract the switchboard will have to meet.

**Definition 4.1 (stalk completeness).** A line verdict `V_e` is **stalk-complete** if `V_e` is a function of `(ρ_{X◁e}(s_X), ρ_{Y◁e}(s_Y))` alone: every way Y's verdict depends on X passes through the line's stalk.

**Proposition 4.2 (early cutoff is sound under stalk completeness; PROVED).** Key each line end by `κ = addr(restriction output)`. If every line verdict is stalk-complete, then reusing the cached verdict of every line whose two keys are unchanged gives the same verdicts as recomputing all of them, barring a collision of `addr`. An edit to S changes keys only on lines incident to S, so **re-glue = the star of S in G**.
*Proof.* Functionality plus Merkle injectivity (fixpoint math Prop. 1.1). Only S's interface changed, and only lines at S read it. ∎

Conditions for stalk completeness *[Seam 6: the first three did not establish it]*:
1. **Undeclared references are lines too.** Alex's 10-29 answer makes an undeclared reference a Dark line in a `partial` shard, so the shard reaches kintsugi. `I(N)` therefore carries `undeclared` as a part (compile §1.3), and kintsugi keys each entry as a **pseudo-line** on X's κ. Without that, Y depends on X through no key, and the cutoff is unsound.
1′. **Free bare names on every line** (Def. 3.1): if X starts exporting a name Y uses bare, Y's binding changes, and only a key that lists Y's free names moves.
1″. **The rule-set address is in κ.** The relation `R` and the Datalog program that evaluates it are kintsugi's code; change them and every cached verdict is stale under unchanged restriction outputs.
1‴. **Grammar and μ edits are global re-glue.** `τ` carries `addr(format grammar)` (compile §5.0), so a meaning-preserving grammar edit moves every address in that format. Sound, not local; not a shard edit.
2. **Exact coverage.** `in @kintsugi` must not stand in for `@kintsugi/roomba`, or the dependency is carried by the wrong stalk (fixpoint spec row 21).
3. **Resolved keys through re-exports.** Named references are never unfolded (fixpoint math Def. 3.4), so `addr` of X's export `T = {a: @Z.U}` names `@Z.U` and doesn't contain U's address. If Y uses T from X and Z changes U, X's raw key doesn't move. The key kintsugi stores is therefore the **resolved** restriction output: `κ(e) = H(addr ρ_X ‖ κ of every foreign name ρ_X's normal form mentions)`, computed bottom-up over the line graph's condensation (cycles hashed as SCCs, fixpoint math §1.4). That is still early cutoff. A change at Z moves `κ` upward only along lines whose restriction actually mentions it. **Interface addresses that compile prints are raw; kintsugi resolves them.** Behaviour that Y relies on through a `requires` about `X.f` puts `X.f`'s body address into `ρ_X` for that line (use kind `pact`). Otherwise a body change with an unchanged signature would be invisible to the line.

**Proposition 4.3 (coarsest sound key; PROVED; restated *[Seam 7]*).** Let the X-side input be `a` and the verdict `V_e(a, t)`. The coarsest sound key is the kernel of `a ↦ V_e(a, ·)`: two outputs are interchangeable iff they give the same verdict against every possible Y side.
- (a) For `V_e(a, t) = [a = t]` with t ranging over `F(e)`, that kernel is equality: the key is `a` itself. *Proof.* Take `t = a`. ∎
- (b) For the real verdict (`R` on kind and arity), it is the **kinds and arities of the used names** (and, for `pact` uses, the address of the body they constrain). *Proof.* `V_e` reads only those components; and for distinct (kind, arity) on some name, a Y side using that name with one of them separates the two. ∎
Keying the X side on `addr(ρ_X)` is sound but **not** coarsest: every kind-preserving signature change re-glues. This answers 10-10 Open 5 **for line verdicts**. For the counts (§2 capacity, §3.3 holonomy), necessity stays OPEN.

**Proposition 4.4 (a closure key is sound and never star-local; PROVED).** `action_cache.mirror` keys on `inputs_oid`, the dependency closure (Taut 10-28 §0.6). A change to a cell of X that no interface exposes changes the closure of every transitive importer of X, so it invalidates all of them. Under Prop. 4.2 it invalidates none (no κ changes). Both keys are sound; only the restriction key gives star-locality.

**Proposition 4.5 (holonomy locality; PROVED for invalidation, OPEN for the count).** Changing the mends on lines at S changes the holonomy only of cycles through S. Re-gauging at S (Prop. 3.5(b)) changes no class. For abelian Γ, the H¹ class changes by the class of a cochain supported on the star of S. Whether the **number** of independent non-trivial holonomies can be updated from star data alone (a non-abelian Thm 8.4) is OPEN. Seam's S1 (10-28): holonomy can kill a comma non-locally.

---

## §5 Holes and H¹

**Proposition 5.1 (an unfillable hole is a relative H¹ class; PROVED for linear stalks).** Let F be a cellular sheaf of vector spaces on a cell complex X, and let A ⊂ X be a closed subcomplex: the hole's surroundings, with the hole as `X ∖ A`. Then a section s ∈ H⁰(A) extends to X iff `∂s = 0` in `H¹(X, A; F)`.
*Proof.* Restriction of cochains `C•(X) → C•(A)` is surjective (extend by zero on cells outside A), so `0 → C•(X, A) → C•(X) → C•(A) → 0` is exact with `C•(X, A)` its kernel. The zig-zag lemma gives `… → H⁰(X) → H⁰(A) → H¹(X, A) → …`, exact at H⁰(A). ∎

READING.
- A hole `\` at occurrence o, with its context fixed, is "I don't know here": no local section at that site. If its surroundings admit an extension, the hole is a hole (fillable; rerere may offer an answer).
- If they don't, `∂s ≠ 0`: no answer to the question is compatible with what surrounds it. The surroundings disagree with each other *through* the hole. That is a comma in disguise.
- Together with §3.3: **a comma at a line is a hole one altitude up (in the mends); an unfillable hole is a comma one altitude down (with its surroundings).** The two verbs rotate into each other. They don't collapse, because the altitude changes.

**CONJECTURE 5.2.** For mirror's typed holes (`Γ ⊢ ? : T` with pacts in scope), "fillable" is an inhabitation question. It is decidable for the sub-Turing fragment mirror claims. Not proved: the typing discipline doesn't exist yet (fixpoint math §3.4, last paragraph).

---

## §6 Two sheaves over one base: declared and observed

**Definition 6.0.** Base B = the union of declared and observed lines. Ambient `P(e)` = possible uses across e: **owner-qualified** facts `(X.name, kind)` *[Seam 11]*. Declared `D ⊆ P` (from interfaces; empty off declared lines). Observed `O ⊆ P` (from the Voice: calls seen at run time, over a window).

**Proposition 6.1 (per line; PROVED).**
- `O(e) ∖ D(e) ≠ ∅`: an **undeclared dependency**. Two sections, declared and observed, exist on e and disagree. It is a line comma between voices. With no declared line at all, it is the runtime face of the inport fracture.
- `D(e) ≠ ∅, O(e) = ∅`: a **dead line**. The observed section over e is the empty set, which is compatible with D, so nothing disagrees. It is a **question** ("is this line used?"), because absence in a finite window is not evidence of falsity.

**Proposition 6.2 (PROVED over 𝔽₂).** Linearise as in §3.2(ii). Cellular sheaves of 𝔽₂-spaces on a graph form an abelian category, and cochains are direct sums of stalks, so `0 → O∩D → O → O/(O∩D) → 0` gives `0 → H⁰(O∩D) → H⁰(O) → H⁰(O/(O∩D)) → H¹(O∩D) → …`. Undeclared observations are sections of the quotient (degree 0, line-local). They reach `H¹(O∩D)` only through the connecting map, i.e. when a local undeclared observation does not lift to a global observed section.

**Proposition 6.3 (was CONJECTURE; PROVED by Seam §9).** For owner-qualified facts under projections, each fact's support lies in the star of its owner, a tree; each sheaf is a direct sum over facts of constant sheaves on trees, every H¹ vanishes, and the per-line sets determine everything. (For bare-name facts, supports can be cycles and H¹ ≠ 0 is possible; Def 6.0 excludes them.)

**The invariant that matters is in the base, not the sheaf** *[Seam 11]*. Y can receive `X.n` through Z (Y `in @Z`, Z `in @X`, Z hands Y the value) and call it: an observed line `Y — X` with no declaration, in a correct program. So the observed finding splits:
- **explained**: a declared path joins Y and X in G (higher-order flow);
- **unexplained**: none does (a hidden dependency).
Deciding it is connectivity in G, across facts, which no per-fact sheaf sees.

**Psychohistory is history.** The observed sheaf is indexed by a window of the past. The history of gluings (which line cracked, at which `root`s, which mend closed it) is a sequence of deposits `(line, question, resolution, tick)`. It is constitutive, as the 2026-06-26 insight §4.4 says ("A section IS the state, not a prediction about it"). Kintsugi's records are that history's entries. **Futures (the light-cone flip, Alex 10-29): out of scope.** Recorded as future work: a sheaf over hypothetical edits, of which Thm 8.4(ii)/(iii) already gives the exact one-edit case.

---

## §7 `1` and `1.0` (Reed's bullet-up Response #2)

A rewrite is free iff it is injective on meaning (fixpoint math §2.3: free = injective under the discrete metric on meanings).
- **Under the number type ℚ** (a literal denotes its exact value), `1`, `1.0`, `1e0` and `10e-1` denote one element. The normal form maps literals to values, and a canonical spelling (shortest decimal, no exponent, no trailing zeros, no point for integers, `-0 → 0`) is a bijection between values and strings. The rewrite is free. **PROVED** (the spelling map is a section of the quotient literal → value).
- **Under value + precision**, or **value + spelled type**, `1` and `1.0` are different meanings, and identifying them is a non-injective `settle` that needs an `@ruler`. **PROVED** (the map forgets a component).
- **Under `f64`**, the identification happens, but for the wrong reason: `9007199254740993` and `9007199254740992` also become equal, a merge ℚ does not make. The spec excludes `f64` (fixpoint spec §9).
- **Prior art.** RFC 8785 (JCS) canonicalises numbers by IEEE-754 double serialisation ("JSON number data MUST be expressible as IEEE 754 double-precision values"). That is the `f64` choice. RFC 8259 leaves precision to implementations, and mentions binary64 for interoperability. Neither decides the meaning of a weight.
- **Which number type is Alex's call** (fixpoint spec §12 row 18). Until then, `Opaque`.

---

## §8 The 2026-07-23 inport math, re-read

`docs/math/2026-07-23-kintsugi-fracture-inport-sheaf-inclusion.md` §3 defines `H¹(𝔊) := C¹(𝔊)/im δ` over the import graph with ℤ coefficients, and states: "The substrate is in strict-`in` compliance iff H¹(𝔊) = 0".

**Counterexample (PROVED).** Take four shards, A `in @B`, A `in @C`, B `in @D`, C `in @D`, each using what it imports, with no undeclared references. That is full compliance. The import graph is a 4-cycle as an undirected graph, so `C¹/im δ ≅ ℤ^{b₁} = ℤ ≠ 0`. Conversely, an undeclared reference has **no edge** in 𝔊, so no cochain on 𝔊 can represent it.
**Verdict.** The theorem and its corollary ("the total fracture count equals dim H¹") fail. H¹ of the import graph counts its cycles (the capacity of §3.3), not missing imports. An undeclared reference is a defect **of the cover**: the cover the shard declared is smaller than the one it uses. It is not a class of any cohomology computed over that cover. The doc's §5 decidability proof stands; it is exactly the compile lint.

---

## §11 A spectral uuid on every fragment: exact and similar offers

Alex 10-29: "add spectral UUID on the `Fragment` and we can ask 'you resolved it similarly in the past…'".

**Definition 11.1.** For every fragment f (not only a root): `dark(f) = addr(f)[0..80)`; `active(f)` = the quantized spectrum of `Γ(f)`, the sub-DAG reachable from f (fixpoint math Def. 7.6 applied at f), under the system-wide `active` `@ruler`.

**Proposition 11.2 (PROVED).**
- (a) `dark(f)` exists for every fragment at no extra cost: every fragment already has an `addr` (fixpoint math Prop. 1.1), and the truncation is the same as for roots. Identity comparisons stay on dark, merges stay on `addr` (fixpoint spec §8).
- (b) `active(f)` depends only on f's sub-DAG (Def. 7.6 is intrinsic), so it is a function of `addr(f)` and can be cached by it.
- (c) `active` never decides identity: cospectral, non-isomorphic sub-DAGs exist (fixpoint math Prop. 7.11).

**Two offers, each labelled.**
- **exact**: same question address (fixpoint spec §5.1; kintsugi line-comma question). "You resolved *this*." Available now.
- **similar**: a recorded resolution whose fragment lies within distance `r` of this one in the active coordinate, under a declared metric and radius (both `@ruler`s). "You resolved something *similar* like this," with the distance and the ruler shown. Unavailable until the `active` `@ruler` is signed (fixpoint spec decision 11): with `active unsigned`, only exact offers exist.

**CONJECTURE 11.3 (similar coordinates predict transferable resolutions).** If `|active(f) − active(f′)| ≤ r`, a resolution recorded at f, carried to f′ by the same edit, makes f′ glue (or fill) at a rate well above chance.
**Refutation test.** Two fragments with **equal** active bits, each with a recorded resolution, such that neither resolution, applied at the other, makes it glue. Prop. 11.2(c) guarantees candidates exist (cospectral, non-isomorphic sub-DAGs). The conjecture survives only as a rate, measured over recorded resolutions once they exist. It never becomes a rule: similarity proposes, the human approves (the guard), and identity stays dark.

## §9 Ledger

| claim | status |
|---|---|
| claim | status | v0? |
|---|---|---|
| Prop 1.4 edges private, overlaps edgeless | PROVED under (A1)–(A3), Σ a multigraph | v0 |
| Prop 1.5 𝒩's 1-skeleton, `b₀(𝒩) = b₀(G)` | PROVED | v0 |
| Prop 2.2 among = `b₁(Σ) − Σ b₁(Kᵢ)`; `Ĥ²(𝒩;H⁰) = 0` | PROVED; COMPUTED 1500/1500 on the model | v0 |
| Cor 2.3 Seam's k ≠ k̃ cannot occur in the `in @X` cover | PROVED | v0 |
| Prop 2.5 Δ among for private edges and R7 | PROVED (corrected); R7 **not star-local** | after v0 |
| CONJ 2.6 the shape cutoff tuple is necessary | CONJECTURE | after v0 |
| Prop 3.2 gluing is pairwise for given sections | PROVED | v0 |
| Prop 3.3 given-section disagreement is a coboundary (all-editable model) | PROVED | v0 |
| Prop 3.3′ frozen shard: one line is an H¹ class | PROVED (Seam Q1) | v0 (reading) |
| Prop 3.5 constant 𝒢: mends ↔ `Hom(π₁, Γ)/Γ` | PROVED, constant 𝒢 only | — |
| Prop 3.6 real mend sheaf not constant; twist on a tree | PROVED (Seam Q5) | v0 (twist check) |
| holonomy cycles vs. capacity classes | OPEN | — |
| Prop 4.2 early cutoff sound under stalk completeness | PROVED (implication) | after v0 |
| Prop 4.3 coarsest key = kinds/arities of used names | PROVED (restated) | after v0 |
| Prop 4.4 closure key not star-local | PROVED | after v0 |
| Prop 4.5 holonomy invalidation is star-local | PROVED for constant 𝒢; count OPEN | after v0 |
| Prop 5.1 unfillable hole = relative H¹ class (linear stalks) | PROVED | — |
| CONJ 5.2 fillability decidable for mirror's holes | CONJECTURE | — |
| Prop 6.1, 6.2 declared vs observed | PROVED | waits for the Voice |
| Prop 6.3 no invariant beyond per-line sets (owner-qualified) | PROVED (Seam) | waits for the Voice |
| Prop 11.2 dark/active per fragment | PROVED | v0 (dark); active after the ruler |
| CONJ 11.3 similar coordinate ⇒ transferable resolution | CONJECTURE, refutation test stated | after the ruler |
| §7 `1 ≡ 1.0` free iff number type = value | PROVED (the choice is Alex's) |
| §8 07-23 inport H¹ theorem | FAILS (counterexample) |
| "comma (H¹ class)" for a line disagreement | CORRECTED: degree 0; the H¹ class is the comma among all |

## §10 Where Seam should aim

*Round 1 answered in `docs/audits/2026-10-29-seam-crowbar-compile-kintsugi.md`; folded above. Round 2 targets: Prop 2.5 as corrected; Prop 3.6's gauge solve on cycles plus trees; Prop 4.3(b); §11's refutation test. The list below is round 1, kept for the record.*

1. **Prop 3.3's reach.** Is there a natural presheaf on which a single cross-shard type error *is* a non-zero H¹ class (Young's "semantic presheaf over a site category" may be one; I read only the abstract)? If so, my correction is a choice of model, not a theorem about kintsugi.
2. **Stalk completeness (Def 4.1, items 1–3).** Find a dependency of Y on X that passes through none of: a code-line reference, a resolved key, a `requires`. Candidates: grammar-level dependencies (a shard's *format* grammar is an input to compile but not a line), `μ` changes, ruler changes.
3. **R7** in this cover: the first use of a foreign symbol merges components of an overlap. Is the comma update still star-local?
4. **CONJ 6.3** on a cycle of lines.
5. **Prop 1.5 + diamond.** Kintsugi reports `Ĥ¹(𝒩; H⁰)`. Check that no spec field reads `b₁(𝒩)` by accident.

---

## Sources

Found via Kagi (`mcp__kagi-ken-mcp__kagi_search_fetch`, `kagi_summarizer`) this session unless marked.
- H. Young, *Sheaf-Cohomological Program Analysis: Unifying Bug Finding, Equivalence, and Verification via Čech Cohomology*, arXiv 2603.27015, https://arxiv.org/abs/2603.27015. Abstract: "Deppy achieves 100% bug-detection recall (69% precision, F1 = 81%)"; HTML version: "Precision is 69% (F1 = 81%); false positives arise on safe programs with complex guard patterns". Rank of Ȟ¹ over 𝔽₂ as "minimum number of independent fixes"; "Mayer-Vietoris … incremental". **[abstract + HTML snippet read; body not read]**
- Stacks Project, Section 21.4, Tag 03AG, *First cohomology and torsors*, https://stacks.math.columbia.edu/tag/03AG. **[statement read; abelian ℋ]**
- *Torsors and Non-abelian Čech Cohomology*, Springer chapter, https://link.springer.com/chapter/10.1007/978-3-658-10633-1_7. **[abstract read]**
- R. Kenyon, *Spanning forests and the vector bundle Laplacian*, Ann. Probab. 39 (2011); arXiv 1001.4028, https://arxiv.org/abs/1001.4028. **[abstract read]**
- A. Mokhov, N. Mitchell, S. Peyton Jones, *Build systems à la carte: theory and practice*, JFP 2020: "When it executes a task and the result is unchanged from the previous build, it is unnecessary to execute the dependent tasks" (early cutoff). https://www.cambridge.org/core/journals/journal-of-functional-programming/article/build-systems-a-la-carte-theory-and-practice/097CE52C750E69BD16B78C318754C7A4 **[snippet read]**
- RFC 8785, *JSON Canonicalization Scheme*, https://datatracker.ietf.org/doc/html/rfc8785 ("JSON number data MUST be expressible as IEEE 754 double-precision values"). RFC 8259 §6, https://www.rfc-editor.org/info/rfc8259/ (binary64 interoperability note). **[snippets read]**
- Curry 1303.3255 Def 4.1.6; Hansen–Ghrist 1808.01513 Def 2.4: **[read in 10-10 session; not re-read]**.

**Computations** (session scratchpad `kintsugi/`, numpy 2.4.4, ephemeral; port before it disappears):
- `cech_shards.py` → `cech_shards.out` (fixtures, 1500 random covers, T29);
- `undeclared.py` → `undeclared.out` (the corpus lint on `346253b`: 20 shards under the prefix rule, 35 under the exact rule, 360 with uncovered prose-only references).

*— Mara*
