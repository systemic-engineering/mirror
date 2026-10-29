# Crowbar: compile is one shard, kintsugi glues

*2026-10-29. Seam. Target: branch `mara/compile-kintsugi-split` at `002656a`.*

**Audited.** `docs/math/2026-10-29-mara-compile-kintsugi-local-sections-and-gluing.md` ("math"), `docs/specs/mirror-kintsugi.md` ("kintsugi"; §10 holds Alex's second-round answers, which are decisions: I crowbar their consequences, not the choices), `docs/specs/mirror-fixpoint-parser.md` as changed in `006530c` ("compile"), and the oracles `tests/kintsugi/oracles/{cech_shards,undeclared}.py`.

**Method.** I re-ran both oracles, wrote two scripts of my own, and grepped the corpus at `346253b` plus `boot/`. My scripts are in `tests/kintsugi/oracles/`:
- `seam_crowbar.py` → `seam_crowbar.out`: R7, K3, mutual reference, Q1, Q5, absent export. It has its own union-find and its own exact 𝔽₂ rank, with no numpy and no import of Mara's `build`.
- `undeclared_variants.py` → `undeclared_variants.out`: the lint count under each lexical choice.

Young's paper (arXiv 2603.27015) was read in full through the arXiv tool: 60 593 characters, every section from the abstract through the references.

**Oracle reruns.** `cech_shards.py` (numpy through `uv run --with numpy`) reproduces `cech_shards.out` byte for byte. `undeclared.py` on `git archive 346253b shards` reproduces 410 files, 20 shards under the prefix rule, 35 under the exact rule and 360 prose-only. The numbers are honest. The findings below are about what they measure.

---

## §0 Verdict table

| # | claim | where | grade |
|---|---|---|---|
| 1 | Prop 1.4: every edge has one owner; overlaps are edgeless | math:41 | **NARROWED** to Σ read as a multigraph with fully qualified, uniquely resolved references (§2.1) |
| 2 | Prop 2.2: comma among all = b₁(Σ) − Σ b₁(Kᵢ); Ĥ²(𝒩;H⁰)=0 | math:67 | **HOLDS** for the model. The 1500/1500 is not evidence that the model fits mirror (§2.2) |
| 3 | Prop 1.5 / Def 1.3: used line iff K_Y ∩ K_X ≠ ∅ | math:39,44 | **FAILS** against K5. A line whose only use is an absent export is classed `unused`, not `comma` (§2.3) |
| 4 | Spec §5 shape layer: "+1 when it joins two" | kintsugi:150 | **FAILS**. It needs Σ connected. Counterexample with `fractured 1`; only 351/1284 random joins give +1 (§3.1) |
| 5 | R7: first use of a foreign symbol is local (Mara's question 3) | math:93, kintsugi:150 | **FAILS**. Two sets with identical star data give Δ = 1 and Δ = 0 (§3.2) |
| 6 | Prop 3.3: given-section disagreement is a coboundary | math:124 | **HOLDS** as stated |
| 7 | "a line comma is not an H¹ class" (Mara's question 1) | math:§0.3, §3.2 | **NARROWED**. It is true for one choice of coefficient sheaf. With frozen vertices, one line's defect is a non-zero H¹ class. Young is no counterexample, because his construction does not compute H¹ (§4) |
| 8 | Prop 3.5(c) / K6: trivial holonomy on every cycle ⇒ gauge-trivial ⇒ glues (Mara's question 5) | math:133, kintsugi:239 | **FAILS** for the mend sheaf as defined. 𝒢 is not constant; a tree of lines carries a non-trivial class (§5.1) |
| 9 | "comma among is owned by no pair" (Alex's vocabulary) | kintsugi:254, math:Def 3.6 | **FAILS** in two places: holonomy on a 2-cycle (mutual `in`), and the summary `among` field, which counts pair-owned capacity (§5.2) |
| 10 | `comma line` / `comma among` carve every comma | kintsugi:254 | **FAILS**. There are at least four that are neither (§5.3) |
| 11 | Prop 4.2: early cutoff is sound under stalk completeness | math:154 | **HOLDS** as an implication. Its three checkable conditions do not establish the hypothesis (§6, five escapes) |
| 12 | Prop 4.3: restriction output is the coarsest sound key | math:162 | **NARROWED**. Def 3.1's verdict is a kind relation, not equality. The premise fails, and the X-side addr is not even read (§6.4) |
| 13 | Compile reads only bytes + format grammar; cacheable by `(addr(grammar), bytes)` | compile:24 | **FAILS** twice: reversed R3 reads the path, and the caller-named start rule is a third input (§7.1) |
| 14 | Interface `uses` per imported `@X` is computable from one shard | compile:113 | **FAILS**. The corpus uses bare imported names (`ref`, `verdict`) under 17 `in` lines (§7.2) |
| 15 | Book stage: `@name → current addr`, `shard(X)` iff some `out @X` | kintsugi:25 | **FAILS**. 2 names are claimed twice inside `shards/`, 20 across `shards/` + `boot/` (§7.3) |
| 16 | Start = what the grammar `out`s | kintsugi:258 | **NARROWED**. Boot grammars `out` keywords and sigils (`in`, `@`, `#`, `\|`), and the interface's kind set has no slot for them (§7.4) |
| 17 | Undeclared = Dark line, Partial document | kintsugi:257 | the decision stands. **Consequence FAILS** K8, Prop 4.2's item 1 and the gate text (§8.1) |
| 18 | Partial exits 1 "as `mirror love`" | kintsugi:257 | the decision stands. **Consequence**: `partial` and `comma`/`fail` share exit 1 unless red moves to 255 as `love` does. Five docs assume partial = 0 (§8.2) |
| 19 | Exact values (`1 ≡ 1.0`) | kintsugi:256, math:202 | the decision stands. **NARROWED**: "data positions" is not checkable, because μ marks none (§8.3) |
| 20 | Conj 6.3 on a cycle of lines (Mara's question 4) | math:196 | **HOLDS, trivially**, for owner-qualified facts. The invariant that matters is not in this sheaf (§9) |
| 21 | Lint count 35 exact / 20 prefix | compile:412, kintsugi:137 | **NARROWED**: 33 / 18 when `//` and `--` comments are comments, and the unit is unpinned (35 shards, 47 pairs, 63 occurrences) (§10) |
| 22 | K3 bullet 3, K7: the incremental tests | kintsugi:224,241 | **FAILS** as tests. A cache that never invalidates passes both (§11) |
| 23 | §8 (math): the 07-23 inport H¹ theorem fails | math:§8 | **HOLDS** (4-cycle counterexample checked) |
| 24 | §7 (math): ℚ makes the spelling map a section | math:205 | **HOLDS** for finite decimals |

---

## §1 What I checked about the oracles themselves

- **`cech_shards.py`.** A, B and C are three independent *counts*. They share one *model*: `build()` assigns each edge to `owner[d]` by construction. So "local cycle spaces direct 1500/1500" is a tautology of `build`, not a check of Prop 1.4. The random generator never produces a dangling reference, a duplicated shard name, a bare (unqualified) name or a re-export, and those are exactly the places where the model and mirror part ways (§2, §7). B uses floating-point `matrix_rank`. That is fine at this size. My `seam_crowbar.py` recomputes `among` with integer union-find and Euler's formula and agrees on every fixture it shares.
- **`undeclared.py`.** It is independent of the compiler, but not of the grammar. It hard-codes `#` as the only comment, per-line `"` toggling with no escapes, and an `in` line only when nothing follows the name. The corpus also uses `//` (2 files, 129 lines) and `--` (32 files) as line comments, has 11 lines with `\"`, and has 2 `in @X  # comment` lines. §10 has the effect.

---

## §2 The cover

### §2.1 Prop 1.4 (single ownership), and the three things it silently assumes

Prop 1.4 is correct given Def 1.1 and 1.2. It needs three assumptions that the spec doesn't state:

1. **Σ is a multigraph.** "One undirected edge per pair `(d, r)`" has to mean ordered pairs. A mutual reference (A.a → B.b, B.b → A.a) gives two parallel edges, one owned by each shard, and `among = 1`. Under the simple-graph reading the edge `{a, b}` lies in K_A ∩ K_B, Prop 1.4 fails, and the Euler count gives 0 (`seam_crowbar.out`, "Mutual reference"). So the reading changes the answer. **Mara must state the multigraph reading.** Alex must also accept that a mutual import between two shards is a capacity class owned by that pair.
2. **Every reference resolves to exactly one declaration.** That holds for `@X.name` with a unique X. It fails for bare names (§7.2) and for duplicated shard names (§7.3).
3. **Re-exports are references, not aliases.** If X re-exports Z's `U` by alias (so Y's `@X.U` *is* Z's declaration), then Y's edge lands on `(Z, U)`. The overlap is K_Y ∩ K_Z, and Y has no `in @Z`, so the line graph and the overlaps disagree. The corpus shows no alias form today (re-exports are `T = {a: @Z.U}`, i.e. references). The model therefore holds, but only because the language has no aliasing. Pin that: **no `out` of a foreign name.**

A symbol declared in two shards is harmless: vertices are `(S, name)`, so they are two vertices. A *shard name* declared twice is not harmless (§7.3).

### §2.2 Prop 2.2 holds, and what COMPUTED does and doesn't show

The proof is right: disjoint edge supports, Thm 8.3, and Euler's formula. The 1500/1500 confirms the three linear-algebra routes on the model. It can't confirm the model, because `build` encodes Prop 1.4.

### §2.3 Def 1.3 against K5: one line, two verdicts

Y has `in @X` and uses `X.f`. X declares no `f`. Vertices are declarations, so `(X, f)` doesn't exist, the reference has no target, and K_Y ∩ K_X = ∅. By Def 1.3 the line is **unused** (a question, verdict `partial`). By Def 3.2 and K5 (`exports absent`) it is a **line comma**. `seam_crowbar.out` ("Absent export") shows the empty overlap. **Fix:** a line is *used* iff `U(Y, X) ≠ ∅`, i.e. Y's interface names something from X, whatever X declares. Then Σ needs a vertex for the dangling name, or the dangling use is kept out of Σ and counted only at the line. Mara must pick one.

---

## §3 Locality of the shape layer

### §3.1 "+1 when it joins two" needs Σ connected (spec §5, K3)

10-10 Thm 8.4(iii) carries "(X connected)". The math copies it (`math:§2.1`: "(iii) one joining two components of Kᵢ (X connected)"). The spec drops it (`kintsugi:150`: "+1 when it joins two"; K3: "On a connected set"). A "connected set" in the spec's own vocabulary is `fractured 1` = b₀(𝒩) = 1, and that does **not** make Σ connected. Math Def 2.4 says so itself: "also counts a shard made of unrelated parts".

**Counterexample** (`seam_crowbar.out`, "K3"). Shards i, X. `i.a` refs `X.n`; `i.b` is a declaration nobody references. `fractured 1`, b₀(Σ) = 2. Adding `i.b → i.a` joins K_i's two components, and `among` stays 0 → 0. The spec says +1.

On random covers, only **351/1284** joins give +1. 144/144 do when Σ is connected, so Mara's 49/49 is the conditioned case (`check_T29` skips any trial with `b0S != 1`). The corpus is far from Σ-connected: any declaration nobody references is its own component.

**The true rule** for a private edge (d, r): **Δ among = [d ∼ r in Σ] − [d ∼ r in Kᵢ]**. The second term is star data. The first is global.

### §3.2 R7 is not star-local (Mara's question 3)

Add `i.d → X.r`, where `r` is foreign and not yet in Kᵢ. Kᵢ gains a pendant vertex and edge, so b₁(Kᵢ) doesn't change. b₁(Σ) rises by 1 iff d and r were already connected in Σ. **Δ among = [d ∼ r in Σ]**. That held in 2876/2876 random R7 edits.

**Counterexample** (`seam_crowbar.out`, "R7"). Shards i, W, Q1, Q2, X. `i.d → X.r2` (the line exists). `W.w → i.d`, `W.w → Q1.u1`, `Q2.u2 → X.r`. Case A adds `Q1.u1 → Q2.u2`; case B doesn't. After the edit, star(i) = {W, X, Q2} in both cases, with **identical** Kⱼ for i and every star member. Only K_Q1 differs, and Q1 is outside the star. The result is Δ = 1 in A and Δ = 0 in B.

**Verdict.** R7 needs Σ's component labels, which are global. Union-find gives that cheaply for **insertions**. **Deletions** (the last use removed) are dynamic connectivity, which is still not star-local: polylog with a fully dynamic structure, or a recompute.

The spec's "recomputed by union-find over the stars it touches" is wrong: the stars it touches don't hold the path. **Fold:** cache Σ's component map globally; insertion = O(α); deletion = recompute the affected component.

---

## §4 Is a single line comma an H¹ class? (Mara's question 1)

### §4.1 Young, arXiv 2603.27015, read in full

Young's presheaf is valued in **complete lattices** (Def 3.5, `𝐋𝐚𝐭`). Def 3.9 defines δ⁰ as "disagreement on overlap" with "subtraction … in the lattice (complement of meet)". Def 3.10 sets `Ȟ¹ = ker δ¹ / im δ⁰`. Then in §2 (the running example) and in the proofs of Thm 6.1 and 6.2 he calls the disagreement of the *computed* sections "a 1-cocycle … not a coboundary". By his own Def 3.10, a disagreement that is δ⁰ of the given sections lies in im δ⁰, so its class is zero. That is exactly Mara's Prop 3.3.

Three more problems:
- "Proposition 0" claims the sheaf condition holds iff Ȟ¹ = 0. That doesn't hold in general: the sheaf condition is a degree-0 (equaliser) statement.
- The 𝔽₂ count in Prop 5.2 and Thm 7.1 builds ∂₀ as the incidence matrix of the overlap graph ("(∂₀)_{(i,j),k} = 1 iff k ∈ {i, j}"). That matrix doesn't depend on any section, so `rk Ȟ¹` computed from it is a function of the cover's shape alone. It is b₁ of the overlap structure: Taut's "b₁-as-bugs" is b₁-as-shape.
- Young's own evaluation reports 77% rank accuracy, and his limitations section names spurious obstructions.

**Verdict.** Young is not a presheaf on which a single type error *is* an H¹ class. His construction doesn't compute cohomology in the sense it defines, and Prop 3.3 survives him. Mara should cite Young for the vocabulary only, never as a theorem.

### §4.2 But Mara's correction is a choice of model, not a theorem about kintsugi

There *is* a natural sheaf on which one line's comma is a non-zero H¹ class: the **variation** (affine) model. The vertex stalk is the space of admissible edits of that shard, not the space of all interfaces. The compiled sections are a fixed base point. The defect `w` becomes a 1-cochain, and "can edits remove it" is `[w] = 0 ∈ H¹(G; F_var)`. This is the standard inhomogeneous-system reading (δs = b), the same shape as Prop 5.1's connecting map.

`seam_crowbar.out` ("Q1") computes it on **one line**, over 𝔽₂:
- both shards editable: `[w] = 0`;
- X frozen (published, or vendored from another repo) and Y able to change only its use: `[w] ≠ 0`, dim H¹ = 1;
- both frozen: dim H¹ = 2.

No cycle is involved, and the class is owned by one pair.

**Verdict.** NARROWED. "A line comma is degree 0, not H¹" is true when vertex stalks are all interfaces. It is false when a shard's stalk is what may still change. Kintsugi *will* meet frozen shards: published `@std`, other peers' crystals, `boot/`. **Fold:** math §3.2 says "in the model where every shard is editable"; the frozen case is an H¹ class owned by a line. Alex's `kind line` survives, but "line = degree 0" in the §10 text does not survive in general.

---

## §5 Mends, holonomy, and the comma kinds

### §5.1 Trivial holonomy everywhere, yet no gluing (Mara's question 5, K6)

Prop 3.5 assumes **𝒢 constant**. Def 3.4's 𝒢(e) is "admissible mends" on line e: renamings of the facts about `U(Y, X)`, a different finite set on each line. A renaming at X (`h_X ∈ 𝒢(X)`) renames X's export on **every** line at once, so `𝒢(X) → 𝒢(e₁) × 𝒢(e₂)` is diagonal, not surjective. That is not a constant sheaf of groups, and the classification `Hom(π₁, Γ)/Γ` doesn't apply.

**Counterexample** (`seam_crowbar.out`, "Q5"). A tree of lines Y₁ — X — Y₂, both using `X.f`. Γ = ℤ/2 = {id, swap f↔g}, G(Y₁) = G(Y₂) = 1 (frozen consumers), G(X) = diagonal ℤ/2. Mend family: swap on Y₁–X, identity on Y₂–X. There is no cycle, so every holonomy is trivial, yet the family is **not** gauge-equivalent to the identity: X would have to export `f` as `g` to Y₁ and as `f` to Y₂.

The trivial version also bites. A single mend (a renaming) applied to a line whose comma is a *kind* mismatch (`carrier` vs `action/2`) leaves the line unglued with no cycle anywhere. Holonomy and per-line gluing are independent checks.

**Fold:**
- K6 asserts per-line gluing under the mend **and** holonomy, and adds my tree fixture as the "holonomy-trivial, not gauge-trivial" case.
- Prop 3.5 is restricted to constant 𝒢. For the real mend sheaf the obstruction is non-abelian Ȟ¹(G; 𝒢), computed by solving for `h_S`. On a tree that is a constraint-propagation problem, not a holonomy check.
- Spec §6's "no single mend can fix it" is false in the tree case: re-exporting under two names (a new export) fixes it.

### §5.2 "comma among = owned by no pair" (Alex's §10 text)

- **Mutual `in`.** A `in @B` and B `in @A` give two lines, a 2-cycle in G. Mends with non-trivial holonomy around it are an `among` comma owned by the pair {A, B}.
- **The summary field `among`** (`kintsugi:64`) is the *capacity* `Ĥ¹(𝒩;H⁰) = b₁(Σ) − Σ b₁(Kᵢ)`. Mara's own table gives pairwise = 1 for the cross-shard triangle and the diamond, so `among` counts classes that a pair owns. The record kind `comma among` is the *holonomy*.

One word names two different numbers, and neither is "owned by no pair" in general. **Alex decides:** keep `among` for the record and rename the capacity field (e.g. `capacity`), or the other way round. Either way, the ownership clause in §10 becomes "not owned by a single line".

### §5.3 Commas that are neither `line` nor `among`

1. **Ambiguous binding.** Y uses bare `verdict` and imports two shards that both export it. In the corpus, `shards/kintsugi/roomba.mirror` uses `verdict` (51 mentions) and has `in @glass` (exports `verdict`, `shards/glass.mirror:467`) and `in @kintsugi`, whose `boot/std/kintsugi.mirror:93` exports `verdict`. The comma is owned by the triple (Y, X₁, X₂). It is a disagreement between two lines at one vertex, not one line.
2. **Unbound bare name.** Y uses `foo`, and none of its `in` targets exports it. Every line of Y is implicated and no single line is.
3. **Duplicate claim.** Two shards both claim `@X` (§7.3). The disagreement is between two sections over one name, before any line exists.
4. **`observed`.** Spec C5 still lists `kind: line | among | observed`; §10 lists two kinds. Pick one list.

---

## §6 Stalk completeness (Mara's question 2; spec question 6)

Prop 4.2 is a correct implication. The question is whether its hypothesis can be reached. These dependencies change a verdict without changing any restriction output's addr:

1. **Undeclared references now reach kintsugi.** Item 1 was "no undeclared references (the compile lint)". It was enforced because compile `fail`ed. Alex's §10 answer makes undeclared `partial`, so the shard compiles, emits an interface, and gets glued. Its dependency on X has no line and no key. **K8's second half ("the unsound state cannot reach kintsugi") is now false** (§8.1).
2. **Bare imported names.** If Y's `uses` attributes `verdict` to `@glass` (it can't do so locally, §7.2) and `@kintsugi` later starts exporting `verdict`, Y's binding becomes ambiguous. The line Y–@kintsugi doesn't list `verdict` in `U(Y, kintsugi)`, so its κ doesn't move. **Fix:** for every line Y → X, `U(Y, X)` = *all* of Y's free bare names. Then X adding or removing an export of one of them moves κ.
3. **The verdict function itself.** The "fixed relation" (`carrier` ~ `carrier`, `action/k` ~ arity k) is kintsugi's code. Change it, and every cached verdict is stale under unchanged κ. **κ must include the address of the rule set** (the Datalog program of spec §11). The same holds for any ruler a future verdict reads.
4. **The format grammar of X versus Y.** `addr` includes `τ`, which carries `addr(format grammar)` (compile §5.0). A meaning-preserving grammar edit (a new free move that touches no X content) changes every addr of every shard in that format. That is sound but not local: one grammar edit re-glues every line, which is the closure-key behaviour Prop 4.4 rules out. That is acceptable, but **K3/K7 should say so**: a `μ` change is not a shard edit.
5. **Path (reversed R3).** A shard without `out` gets its name from its path (compile:145). Moving the file renames the shard and changes every line into it, while `(addr(grammar), bytes)` is unchanged (§7.1).

### §6.4 Prop 4.3's premise is not Def 3.1's verdict

Prop 4.3 assumes `V_e(a, t) = [a = t]` with t ranging over F(e). Def 3.1 has ρ_Y output Y's **use kinds**, with no addr (Y can't know X's addr), compared to X's `(kind, addr)` "by a fixed relation". The verdict reads only X's kind and arity, never its addr, except for `pact` uses. So:
- keying the X side on `addr(ρ_X)` is sound but **not** coarsest: every kind-preserving signature change re-glues;
- every cross-shard error *finer* than kind and arity is invisible: Y reads field `a` of `X.T` and X drops `a`, or Y calls `X.f` with the wrong argument types. Math §0.3's "A cross-shard type error is a **line comma**" is NARROWED to **kind/arity errors**.

**Alex decides** whether kintsugi checks types or only kinds. If only kinds, the record should say `kind` rather than "type error".

---

## §7 Compile's locality

### §7.1 The cache key is incomplete

- **Path.** Compile:145 says R3 is reversed: it "inserts `out τ_path(path)` when absent". Compile T8 requires "a shard without `out` at `shards/a.mirror` and at `shards/b.mirror` gets different roots". Compile:24 says the output is a function of `(addr(format grammar), source bytes)`. Those cannot all hold: same bytes and grammar, two roots. 11 shards at `346253b` have no `out` (e.g. `shards/mirror/book.mirror`, `shards/mirror/lens/knife.mirror`). **Fold:** either the path is a declared third input (key `(grammar, bytes, τ_path)` when no `out` is present), or a missing `out` is a hole `name?` and R3 stops inserting. The second keeps compile honest to "bytes only", and T8's path vector then reads `partial`, not two roots. Alex owns decision 13, so he decides.
- **Start rule.** Alex: "several [exported rules] are legitimate, and the caller names one". Then the start is an input to compile that is not in the key: two callers, one set of bytes, two parses. **Fold:** key `(addr(grammar), start, bytes)`, and `start` in the §1.1 record. Plain `mirror compile x.mirror` must pin its default caller-named start for `@mirror` in the spec, or every compile is `partial` ("start?"), which under the new exit codes is exit 1 (§8.2).

### §7.2 `uses` per imported `@X` is not local

Compile:113 says `uses`: "per imported `@X`: each name the code lines use". `shards/kintsugi/roomba.mirror` has 17 `in` lines and uses `ref`, `verdict`, `type`, `bilateral` **bare**. Which `@X` binds `ref` (`boot/01-meta.mirror:161 out ref`) is a fact about the *other* shards' exports. Compile cannot attribute names without reading them, which is the one thing it may not do.

What compile *can* emit is the set of **free names** (bare, unbound locally) and the set of `in` lines. Attribution is resolution, and resolution is kintsugi's (stage 2/3). This moves part of the interface into kintsugi and creates the §5.3 commas.

**Mara must fold:** `I(N).uses` = qualified uses per `@X`, plus `free` = bare names. Kintsugi resolves `free` against the in-targets' exports, and the result is a resolution splinter keyed on the κ of every in-target (§6 item 2).

### §7.3 The book is not a function

I scanned for `out|grammar|prism @X` declarations:
- inside `shards/` alone, **2** names are claimed by two files: `@mirror/spec` (`shards/mirror/spec.mirror`, `shards/mirror/spec/keywords.mirror`) and `@song` (`shards/song.mirror`, `shards/song/keywords.mirror`);
- across `shards/` + `boot/`, **20** names, including `@kintsugi`, `@prism`, `@io` (three files), `@nl` (four) and `@fate`.

`mirror.spec` names `boot/` under `legacy`. Spec §3 defines `shard(X)` as "some compiled shard declares `out @X`". With two claimants, `in @X` is neither resolved nor a `gap`.

**Alex decides:** is a name spread over files one shard (merge the claimants' interfaces, and a conflicting export is a comma of kind 3 in §5.3), or is a second claimant a finding?

### §7.4 Start = `out`

The boot grammars `out` things that are not rules. `boot/00-prism.mirror:55-60` has `out in`, `out id`, `out @`, `out @prism`, `out shift`, `out settle`. `boot/01b-nl.mirror:12` has `out #`. `boot/01-meta.mirror:160-162` has `out type`, `out ref`, `out |`.

The interface's kind set `{carrier, action, pact, bilateral, prism, grammar}` (compile:112) has no kind for a keyword, sigil or operator, so compile can't tell an exported rule from an exported token from the export list alone. It must read the declaration each `out` names: an `out x` whose `x` is a grammar rule is an entry point, and anything else isn't.

That is checkable, so the rule is **NARROWED, not failed**: "start candidates = exported names whose declaration is a rule". The kind set gains `keyword` (or `token`). No boot grammar has exactly one exported rule, so "one export = entry" never fires there, and the caller always names the start.

**T1.3 (reordering) stays order-free**, because the start is chosen by name.

---

## §8 Consequences of Alex's §10 answers

### §8.1 Undeclared = Dark line, Partial document

The decision is coherent (Dark: nothing looked; Opaque: a question asked). Its consequences:

- **A Dark line silently inside a crystal.** `root` is the address of N, and N contains the dark line. That is honest, since root says what was read. The interface, though, has **no line** for the dependency, so `I(N)` is incomplete without saying so. Anyone keying on `interface` (kintsugi's κ, a cache, a rerere offer) trusts a stalk that omits a dependency. **Fold:** `I(N)` carries `undeclared` (the `@X` list) as a part, so `interface` changes when an undeclared reference appears, and kintsugi treats each entry as a **pseudo-line** keyed on X's κ, or refuses to cache any line of that shard. Without one of these, Prop 4.2's item 1 has no enforcement.
- **K8** must change. Its second fixture now reaches kintsugi as `partial`, and the test should assert that kintsugi does not serve that shard's lines from cache.
- Spec §4's `fail` row (`undeclared > 0`), compile §1.1's `fail` verdict, T17c ("`verdict fail`"), and the gate sentence "Until the 35 undeclared … the gate fails by design" are all stale against §10. §10 says "the body text is updated in the next pass". These are the places.

### §8.2 Partial exits 1

`mirror love` (`rust/src/main.rs`, the verdict-to-exit block after `fn cmd_love`) maps Green → 0, Yellow → 1, Red → **255** ("-1"). Adopting it "as `mirror love` already does" means red = 255. The current tables put `fail`/`comma` at 1 and `dark` at 2 (kintsugi:128-133, compile:84). With partial at 1, **partial, comma and fail share exit 1**, and CI can't tell an honest hole from a disagreement.

What breaks or must change:
- `docs/specs/kintsugi-ci-v0.1.md:119`: "Exits 0 on `pass`, 0 on `partial` (unless `fail-on: partial`)". That is the action contract, and `fail-on: partial` becomes meaningless.
- `docs/specs/compiler-surface-plan.md:~914`: a test asserting "non-strict: Partial → exit 0".
- `docs/specs/strict-and-total-classification.md` (the `--strict` contract) and compile §1.1's table (`partial 0 / --strict 1`). `--strict` becomes a no-op for partial. What does it mean now?
- **The gate.** `mirror kintsugi mirror.spec` exit 0 now requires zero holes in 410 shards (the sample record shows `holes 1873`) and zero `unused` lines (spec §4 puts unused under partial). Every honest `\` blocks every commit. Alex decides whether the gate is "exit ∈ {0, 1}" or `--allow partial`.
- The Justfile and `.github/workflows/kintsugi.yml` call `mirror kintsugi mirror.spec`. Before the verb dispatches, no script reads the exit; after it does, every script that treats non-zero as failure blocks on holes.

**Fold:** one table, shared by compile, kintsugi and love: success 0, partial 1, red 255, with `dark` either a red (255) or its own code. Kintsugi's `comma` and compile's `fail` move to 255.

### §8.3 Exact values

ℚ makes the spelling map a section (math §7 holds). Where `1` vs `1.0` is meaning:
- **Typed emission.** `boot/std/beam.mirror:185` `loss(1.0)` and `boot/std/mirror/reload.mirror:45` `loss: 0.0` sit in `f64` fields of the Rust floor. `print(N)` emits `1`. Rust rejects `let x: f64 = 1;`, and Gleam rejects `1` for `Float`. `render(N, ε)` restores `1.0` from the erasure, but any *other* printer (Rust or Gleam emission from N) needs the position's type, and ℚ has erased the only evidence of it.
- **Integer positions.** `arity 1` (bilateral), `repeat(literal("#"), 1, 6)`. Under ℚ, `arity 1.0` ≡ `arity 1` is accepted silently.
- **Digits inside refs and dates.** `@arxiv/…/wolf-2026-07-14-…` (134 leading-zero hits, all inside refs or `source` lines). The number rule must never apply inside a ref atom. That is a lexer ordering to pin, not a μ question.

"Data positions" (Mara's lean) is not checkable. μ marks no position as data, Int or Float. **Fold:** μ declares a number *type* per position (`int`, `exact`, or a ruler), defaulting to `exact`. Then `arity 1.0` is a hole at an `int` position, and emitters read the position type, not the spelling.

---

## §9 Conjecture 6.3 on a cycle of lines (Mara's question 4)

**HOLDS, trivially, for owner-qualified facts.** A fact `(X.n, kind)` lives only on X, on its users, and on lines `Y – X`. Under projections, its support in P, D, O or O/(O∩D) is closed and contained in the **star of X**, which is a tree. So every such sheaf is a direct sum, over facts, of constant sheaves on stars, every H¹ vanishes, every connecting map is zero, and the per-cell sets determine everything.

If Def 6.0's `name × kind` means *bare* names, supports can be cycles (three shards each exporting `detect` and using each other's), and H¹ ≠ 0 is possible. **Fold:** Def 6.0 says owner-qualified.

**The invariant that matters is not in this sheaf.** Y receives `X.n` through Z (Y `in @Z`, Z `in @X`, Z hands Y the function value) and calls it. Then O(Y–X) ∖ D(Y–X) ≠ ∅, so Prop 6.1 says "undeclared dependency, a comma", yet the program is correct: the observed edge is explained by the declared path Y–Z–X. Whether an observed undeclared line **closes a cycle with declared lines** (here, a triangle in B) decides "higher-order flow" against "hidden dependency". That lives in the base graph, across different facts, and no per-fact sheaf sees it.

**Fold:** `comma observed` splits into `observed explained` (a declared path exists) and `observed unexplained`. Spec §7's "observed call with no declaration → comma observed" is NARROWED accordingly.

---

## §10 The lint count (Reed's list)

`undeclared_variants.out` on `346253b` (shards / (S, X) pairs, exact rule):

| lexical rule | prefix | exact | pairs |
|---|---|---|---|
| M: Mara's oracle (`#` only) | 20 | 35 | 47 |
| C: `//` and `--` are line comments | 18 | 33 | 44 |
| R: reversed-R3 path self-name | 19 | 34 | 45 |
| I: `in @X  # comment` is an import | 20 | 35 | 47 |
| E: `\"` escapes | 20 | 35 | 47 |
| T: string literals not stripped | 20 | 35 | 52 |
| ICER | 18 | 33 | 44 |

- The 2 shards that drop out under C are `epistemologic/cybernetic/algedonic.mirror` (`@cyberpunk/reframe` appears only in `//` prose) and `epistemologic/cybernetic/design.mirror` (a self-name in `//` prose). They are lexer false positives, or Dark files if `//` isn't mirror syntax at all. Either way they are not `undeclared`.
- **Units.** Compile's field is "number of code-line references" (occurrences): **63** at `346253b` under M. Kintsugi's sample summary shows `undeclared 35` and says it is "sums of the compile records". 35 is shards, 47 is pairs and 63 is occurrences. **The fixture must pin the unit** (lean: `(S, X)` pairs, which is what `expected.mirror-text` lists).
- **Taut's 27** differs again: T (strings counted) changes pairs, not shards, so Taut's rule differed in more than strings.
- **Which rule is the spec's.** Compile §1.3 says "not a comment, not inside a string literal" and leaves *comment* to the format grammar (`boot/01b-nl.mirror:12 out #`). So the spec's rule is "whatever `μ_mirror` says a comment is". The oracle must read the comment tokens from the grammar, or the fixture must state them. T17d "pins the rule, not the number" is right. Today the rule it pins is M, and M is not the grammar's rule.

---

## §11 Tests that pass for the wrong reason

- **K7** ("edit-and-revert → same `run`"). A cache that **never invalidates** passes: after the revert, every κ equals the original, so serving everything from cache reproduces the original run. **Add K7′:** edit without revert. The incremental run's `run` address equals a from-scratch run on the edited set, and the edit is one that changes a line verdict.
- **K3 bullet 1** ("`reglued ⊆ star of i`"). `reglued 0` satisfies it. **Add a positive control:** an edit that changes ρ_X on line e must report `reglued ≥ 1` *and* the new verdict.
- **K3 bullet 3** ("body no interface exposes → `reglued 0`"). Same issue. It is only meaningful paired with the control above in the same harness.
- **K3 bullet 2** ("+1") fails on a `fractured 1` set with Σ disconnected (§3.1). The fixture must state b₀(Σ) = 1, or assert Δ = [d∼r in Σ] − [d∼r in Kᵢ].
- **K2d** (≥ 1000 random sets: `among` = Euler = Čech). The generator shares Prop 1.4 with the code under test, so the test can't catch a wrong cover. **Add** fixtures for a mutual reference (expected `among 1`), an absent export (expected `comma line`), and a bare name bound by two imports (expected the §5.3 kind).
- **T17a** (sandbox denies reads outside `a.mirror` and the grammar). It passes even though output depends on the path (the path isn't a *read*). **Add T17e:** the same bytes, without `out`, at two paths. Either the roots are equal and the record has a `name?` hole, or the path is declared as an input.

---

## §12 To fold, to decide

**Mara must fold (math + spec):**
1. Def 1.1: Σ is a multigraph (ordered references). State the assumptions behind Prop 1.4: unique resolution, no aliasing re-export (§2.1).
2. Def 1.3: *used* iff `U(Y, X) ≠ ∅`, whatever X declares. Place dangling uses (§2.3).
3. Spec §5 and K3: restore "(Σ connected)", or use Δ = [d∼r in Σ] − [d∼r in Kᵢ]. Mark R7 **not star-local**. Insertion is global union-find; deletion is dynamic connectivity (§3).
4. Math §0.3 and §3.2: "line comma = degree 0" in the all-editable model. The frozen-vertex model makes it H¹ on one line (§4.2). Cite Young for vocabulary only (§4.1).
5. Prop 3.5: constant 𝒢 only. Real mends need non-abelian Ȟ¹(G; 𝒢), and K6 asserts per-line gluing and holonomy separately, with the tree fixture (§5.1).
6. κ includes the rule-set address; `U(Y, X)` includes all of Y's free bare names; grammar edits are documented as global re-glue (§6).
7. Prop 4.3: restate for the real verdict (kind relation). "Cross-shard type error" → "kind/arity error" (§6.4).
8. Compile: cache key `(grammar, start, bytes[, τ_path])`; `I(N)` = qualified uses + `free` + `undeclared` (§7.1, §7.2, §8.1).
9. Interface kind set gains `keyword`/`token`; start candidates = exported rules (§7.4).
10. Body text against §10: spec §4's table, compile §1.1, T17c, the gate sentence, K8 (§8.1, §8.2).
11. Def 6.0: owner-qualified facts. Split `comma observed` into explained and unexplained (§9).
12. The undeclared oracle reads comment tokens from the grammar. Pin the unit (pairs). Re-pin 33 / 18 under the grammar's actual comment rule (§10).
13. Tests: K7′, positive controls in K3, the extra K2 fixtures, T17e (§11).

**Alex must decide:**
1. Multiple files claiming one `@X` (2 in `shards/`, 20 with `boot/`): one shard, or a finding (§7.3)?
2. A shard with no `out`: does the path name it (third compile input), or is it a hole `name?` (§7.1)?
3. `among`: the record (holonomy) or the summary (capacity)? Rename the other one. Drop "owned by no pair" in favour of "not owned by one line" (§5.2).
4. The comma kinds beyond `line`/`among`: ambiguous binding, unbound name, duplicate claim, observed (§5.3).
5. Exit table: red = 255 as in `love`, so that partial (1) ≠ comma/fail. The gate's tolerance for partial (holes, unused lines) (§8.2).
6. Number type per μ position (`int` / `exact` / ruler), or exact everywhere with typed emitters on their own (§8.3).
7. Kintsugi checks kinds or types (§6.4)?

*— Seam*
