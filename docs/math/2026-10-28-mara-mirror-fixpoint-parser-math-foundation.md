# Mirror's fixpoint parser — math foundation

**Author:** Mara `<mara@systemic.engineer>`. 2026-10-28.
**Companion spec:** `docs/specs/mirror-fixpoint-parser.md` (what gets built and how it is tested). This document says what is true and why.
**Seam reads this next.** Every claim is labelled so it can be aimed at.

Labels (as in `docs/math/2026-10-10-mara-the-chord-math-foundation.md`): **PROVED** (proof here or a cited theorem) · **COMPUTED** (script in `~/dev/ai/mara/scratch/fixpoint-parser/`, not committed; rerun it) · **CONJECTURE** (stated so it can fail) · **READING** (an interpretation; it carries no weight in a proof) · **DECISION** (a choice someone has to own; named who).

## Decisions this document builds on (Alex, 2026-10-28, in-transcript)

- "evaluator needs to live in mirror. Otherwise it's decorative."
- "The plan is to remove serde in favour of mirror doing the transformation itself."
- "Take the settled shape of `prism` and build the parser in mirror. With bootstrap as an inspiration at most. boot/ is also legacy inspiration."
- "the first shard is mirror.mirror -> @mirror. That self-describes mirror through the AST shared with prism's operations. And there I do see in fact a `Splinter` as the atom of the AST composition. And then `fragments` become content-addressed automatically. You see the loop closure?"
- "That's what `mirror compile` does btw. Parses. Beta-normalizes. Crystallizes. And with the content addressing we can back project into git. And boom."
- "What if the `Splinter` name collision is by design. How do we infer over the existing splinters? We rotate them."
- "Rotation is free. The basis is free. The ruler not. We can rotate freely in the space of meaning."
- "And from the Merkle DAG the semantic spectral uuid emerges."

## §0 The answer

1. **The loop closes, with one honest boundary.** `compile = settle ∘ normalize ∘ parse` maps `mirror.mirror` to a Merkle DAG whose root address is the parser's identity. Stage 1 (the parser read off that DAG) reproducing the seed's root is a checkable equation, not a theorem: it can fail, and a seed that misreads itself consistently also satisfies it (§6). The fixed point certifies agreement between two readers, never honesty of the first.
2. **Comma 1, the Splinter collision, is by design and holds after one correction.** There is one splinter type: an altitude-tagged content address plus a transparency. The tick (rotation) and the source position (parsing) are coordinates of the *deposit*, i.e. of when and where a splinter fell, and must stay out of its address; otherwise no rotation reaches a fixed point (§1.4, PROVED). Rotation's current struct puts `tick` inside the address. That is its Phase-1 slice, and it is the one field that has to move.
3. **Comma 2, three ops.** Sequence is composition, not an op. Literal is `focus`. Labelling a match is `project` (emission, per prism's 2026-10-27 decision). Choice is `split` followed by `settle` with a declared ruler. Recursion is a named reference, which is not an op at all. Words over `{focus, project, settle}` can't express choice (PROVED, §4.2), so `split` is necessary. It doesn't have to be a fourth method on the `Prism` trait; it can be the fracture node of the Beam's trace DAG that prism's `CURRENT.md` §5 already proposes.
4. **Comma 3, bytes vs. abstract form.** The address is over the normal form, i.e. the quotient by declared free moves (§2). Trivia (whitespace, comments, source positions, bound-variable names) are neither kept in the splinter nor dropped: `settle` writes them as an erasure record next to the semantic root, with its own address. Two round trips hold, at two altitudes: bytes to bytes through the concrete tree, and meaning to meaning through the normal form (§3).
5. **Parsing and inference share one engine, and it isn't the Kuramoto part.** A PEG is a positive Datalog program over (expression, position) items (PROVED, §5.1; COMPUTED 1480/1480 against packrat). Semi-naive evaluation is "a rotation": a tick is a round, only the delta joins, and the fixed point is the first tick at which nothing new falls (PROVED, standard; COMPUTED 2000/2000). The well-founded alternating fixpoint `mirror pact eval` uses is two such rotations per round, with retractions on the over-estimate side. Phases, coupling `K` and the Fiedler gauge in `rotation.rs` are not part of the shared engine (§5.4).
6. **The semantic id falls out of compile, with two rulers to declare.** `dark` = the first 80 bits of the normal-form Merkle root: free-move-invariant and meaning-sensitive (PROVED, given a collision-resistant hash). `active` = a quantized spectrum of the crystal's own DAG: relabelling-invariant (PROVED). But the directed DAG's adjacency spectrum is identically zero (PROVED, §7.4), so "which operator" is a real choice, and cospectral non-isomorphic trees exist from 8 vertices on (COMPUTED). So `active` locates and never identifies.
7. **Found while checking (load-bearing for Seam):** prismqueer's `Oid::hash` feeds SHA-256 three `f64` scalars, not the input (COMPUTED: reimplementation reproduces the pinned vector `08f8e91d…`, and every projection is rank one). Its collision resistance is bounded by those three float sums, not by SHA-256, and its value depends on floating-point summation order (my first Python port was wrong only because Python 3.12's `sum()` is compensated). The root address of a self-hosting compiler should not run through float arithmetic. §7.2.

---

## §1 Objects: splinter, fragment, address

### §1.1 What the substrate already said

`shards/glass.mirror` (2026-06-06) already declares the atom, and the brief's vocabulary maps onto it without minting anything:

- `type splinter(altitude) = { content: oid, altitude: ref, transparency: transparency(altitude) }`: "the universal content-addressed atom … AST nodes are splinters."
- Three exhaustive shapes: **atom** (leaf), **fractal** (composite: content + children), **lens** (referring: content + external targets by oid).
- `type splinter(ast)`: the same carrier at the `@meta/ast` altitude.
- Alex 2026-06-06 (glass.mirror header): "The `splinter` is the content addressed fragment. And the `shard` is a settlement of content addressed splinters into uuid_spectral addressed stored fragment."

In this document a **fragment** is a splinter of shape *fractal*: Alex's 2026-10-28 "fragments become content-addressed automatically". That is consistent with glass.mirror, and it supersedes the reading in `docs/math/2026-07-22-splinter-eigen-fragment-ouroboros-closure.md` §3 ("`fragment` = `@eigen(Crystal<T>)`", i.e. fragment as the settled crystal). Under today's usage the crystal is the settled *root* of a fragment DAG, not every fragment. **DECISION (Alex):** confirm that "fragment = composite splinter" replaces the 2026-07-22 reading.

### §1.2 The address

Fix a hash `H : bytes → {0,1}^256` (which one is §7). Every splinter `x` has an altitude tag `τ(x)` (an `@`-ref, encoded as UTF-8 then NUL) and a canonical encoding.

- atom: `addr(x) = H("splinter\0" ‖ τ(x) ‖ enc(content(x)))`
- fractal (fragment) with label `ℓ` and **ordered** children `x₁ … x_k`: `addr(x) = H("fragment\0" ‖ τ(x) ‖ enc(ℓ) ‖ k ‖ addr(x₁) ‖ … ‖ addr(x_k))`
- lens: as fractal, with the targets' addresses in declared order and a distinct prefix `"lens\0"`.

`enc` is prefix-free (length-prefixed fields), so the concatenation is uniquely decodable. `transparency` is **not** in the address, matching prism's `Crystal`, whose oid ignores luminosity (`crystal.rs` test `same_prism_same_oid_regardless_of_luminosity`). The verdict about a thing is not the thing.

**Proposition 1.1 (Merkle; PROVED, standard).** On finite acyclic splinter graphs `addr` is well defined by structural recursion. If `addr(x) = addr(y)` and `x ≠ y`, then `H` has a collision on some pair of encodings.
*Proof.* Induction on height. If the top encodings differ, the equal digests are a collision. If they are equal, prefix-freeness gives equal tags, labels and child-address lists, so some pair of children differs with equal addresses, and the hypothesis applies one level down. ∎ (Merkle 1987 for the tree construction; hash-consing, Filliâtre & Conchon 2006, is the same identity used for sharing in memory.)

**Corollary 1.2 (PROVED).** Content addressing "falls out of construction", as Alex said: building a fragment means computing its address from its children's addresses. No separate indexing pass exists to drift.

**Corollary 1.3 (domain separation; PROVED).** Two splinters at different altitudes never share an address unless `H` collides, because `τ` is inside the hashed prefix-free encoding. This is what makes one splinter type safe across altitudes (§1.4).

### §1.3 Two compositions, one of them ordered

prismqueer's `MerkleTree` trait says children are "sorted by Oid for deterministic tree shape" (`merkle.rs`), and `shards/uuid/spectral.mirror` derives the monoid laws of shard merge "by structural consequence of content-addressing + sorted-children".

**Proposition 1.4 (PROVED, by construction).** If a fragment's address is computed over its children sorted by address, it is a function of the *multiset* of children. Then `seq(a, b)` and `seq(b, a)` get the same address, and so do `choice(a, b)` and `choice(b, a)`.

For syntax this is wrong twice over. A sequence is ordered, and a PEG choice is *ordered* (§2.4: swapping alternatives can change the language). So the parser needs the **ordered** fragment of §1.2. Store merge (`reality-shard-as-crdt.md`: union of shards) really is commutative and idempotent, and sorted children are right there. These are two different compositions:

| composition | algebra | children | where |
|---|---|---|---|
| syntax (fragment) | free monoid / ordered tree | in declared order | AST, parse items, pact rule bodies |
| store merge (shard) | join-semilattice | sorted, deduplicated | `@mirror/store`, CRDT merge |

Neither may borrow the other's addressing.

### §1.4 Cycles: references inside a grammar

A grammar refers to itself: in `boot/std/mirror/grammar.mirror`, `form` mentions `brace_block`, which mentions `form`. The reference graph is cyclic, and a Merkle hash needs acyclicity.

**Proposition 1.5 (PROVED, standard).** The condensation of a directed graph (each strongly connected component contracted to a node) is acyclic. So the address is well defined on the condensation. Hash each SCC as one fragment whose children are its member rules in a canonical order, and write a reference *inside* an SCC as the pair (SCC, index), not as an address.

This is Unison's rule, verified: "A term or type definition that's part of a cycle of mutually recursive definitions hashes to the form #x.n where x is the hash of the cycle and n is the term or type's index in its cycle. A cycle has a canonical order determined by sorting all the members of the cycle by their individual hashes (with the cycle removed)." (unison-lang.org, Hashes). Unison sorts by hash because it excludes names from hashes. In a mirror grammar, rule names **label the output**: the AST node a rule produces is named by the rule. Renaming a rule changes what the parser emits, so it changes meaning, and rule names are part of the content (§2.3). The canonical order inside an SCC is therefore **by rule name**, which is cheaper than Unison's and invariant under reordering the declarations. **DECISION (Alex):** rule names are meaning; declaration order isn't.

### §1.5 Comma 1: one Splinter, rotated (Alex: the collision is by design)

`prismqueer::spectral::rotation::Splinter` says of itself: "Splinter is the terminal-leaf atom per @glass three-layer recognition (Alex 2026-06-06): content-address + altitude-ref + transparency. Phase 1 MVP carries content-address + tick-index + measured observables; altitude + transparency deferred to Phase 2+." Its oid is "the content-address of the (phases, order_r, fiedler, tick) tuple".

**Definition 1.6 (one splinter).** `Splinter = (altitude, content, transparency)` with `addr` as in §1.2. A **deposit** is a record `(addr, coordinate, witness)`: *when and where* a splinter fell. The coordinate is a tick for a rotation, a source span for a parse and a round for a Datalog evaluation. The witness is who or what observed it (the Voice beat of composite #9).

**Proposition 1.7 (coordinates must stay out of the address; PROVED).** Let a rotation be an iteration `S_{t+1} = S_t ∪ step(S_t)` over sets of addresses, stopping at the first `t` with `S_{t+1} = S_t`. If a splinter's address depends injectively on the tick `t` at which it fell, the iteration stops only when `step(S_t) = ∅`. In particular any step that re-derives anything never stops.
*Proof.* Every splinter deposited at tick `t+1` carries `t+1` in its address. No element of `S_t` carries `t+1`, because ticks only increase. So any non-empty `step(S_t)` adds a new address, and `S_{t+1} ≠ S_t`. ∎

So Reed's item 1 ("the tick or the source position is the altitude coordinate") holds **as the deposit's coordinate, not the splinter's identity**. Rotation's struct is the Phase-1 slice in exactly the place its docblock says: `altitude` and `transparency` are missing, and `tick` sits in the content. Under one type, a rotation observation is `Splinter { altitude: @spectral/rotation, content: (phases, order_r, fiedler), … }` deposited at coordinate `tick`. If two ticks observe the same state, that is **one** splinter deposited twice. That's the property a fixed point needs, and it's the same rule as composite #9's "witness events (time, actor, place) kept out of the hash".

**READING.** "Parsing is a rotation over bytes; inference is a rotation over splinters" (Reed). §5 makes the shared part precise: deposit, delta, stop when nothing new falls. Which part is *not* shared is said there too.

**DECISION (Alex + prism-Reed).** Moving `tick` from `Splinter.oid`'s preimage to a deposit record changes every existing rotation splinter address. One consumer exists (grep): `rust/src/main.rs:107,634` calls `infer_via_rotation`, keeps the splinters in session state and emits `oid` and `tick` as JSON. Whether anything persists those oids beyond the session is not established here; if something does, the move needs a migration record, not a silent change.

---

## §2 Free moves and rulers

Alex: "Rotation is free. The basis is free. The ruler not." The essay `blog/ai/mara-seam/whose-metric.md` measured it: aligned by rotation, the leftover is `9.60793` whichever observer is the reference; stretch one axis ×10 and it goes from 9.6 to 119.6; a shear has the trace of the identity (2) and still shrinks agreement from two dimensions to one. Here is the discrete version that a parser needs, with one correction.

### §2.1 Definitions

Let `R` be the set of representations (byte strings, concrete trees, ASTs) and `μ : R → M` the **meaning map** onto a set of meanings `M`.

- A move `g : R → R` is **free** iff `μ ∘ g = μ`.
- A move is **ruler-carrying** iff it isn't free. Saying how far `μ(g r)` is from `μ(r)` needs a metric on `M`, and someone chooses that metric.

**Proposition 2.1 (PROVED).** If a canonical form `c : R → R` exists with `μ(c r) = μ(r)` and `μ(r) = μ(r') ⇒ c(r) = c(r')`, then `addr ∘ c` is a complete invariant of meaning up to hash collisions: `addr(c r) = addr(c r') ⇔ μ(r) = μ(r')`. The OID is the address of the quotient `R / ker μ`.
*Proof.* (⇐) from the second property of `c`. (⇒) Proposition 1.1 gives `c r = c r'` barring collision, and the first property gives `μ(r) = μ(c r) = μ(c r') = μ(r')`. ∎

This is Reed's item 2 and the brief's comma 3: "identity is what survives every free move", and "what survives the auditor" made computable.

### §2.2 The correction: the quotient is itself a ruler

**Proposition 2.2 (PROVED, linear algebra).** "Isometry" is defined relative to a metric. For an inner product with Gram matrix `M`, the isometries are `{Q : QᵀMQ = M}`. The criterion `QᵀQ = I` is the special case `M = I`. A shear `S` with `SᵀMS = M` exists for suitable `M`. So no linear map is free or ruler-carrying in itself; it is one or the other relative to a declared metric.

Consequence, discretely: *which* moves are free is fixed by `μ`, and `μ` is a declaration. Is a comment meaning? Is declaration order? Is a rule name? Each answer is a ruler, chosen once. "The basis is free" is true only **after** someone has declared the space of meaning. So mirror has to declare `μ` in the grammar itself, and it does so in exactly three places: which tokens are trivia, which positions bind variables, and which rewrite rules the normalizer applies. Then every free move follows, and nothing else needs a ruler. This sharpens Reed's item 1: free moves don't need a signature *each*, because they inherit one signature, which is `μ`'s.

### §2.3 Free means injective under the discrete metric

**Proposition 2.3 (PROVED).** Give `M` the discrete metric (`d = 0` iff equal, else 1). A map `φ : M → M` is an isometric embedding iff it is injective.
*Proof.* `d(φa, φb) = d(a, b)` for all pairs says exactly that `φa = φb ⇔ a = b`. ∎

That is the bridge from Alex's linear algebra to a parser. The discrete metric is the one ruler that only tells "same" from "not same", so it needs no choice beyond `μ`. Every finer metric (edit distance, threshold, weight, quantization) is chosen by someone and has to be named. On meanings, then:

- operations that are **injective** on meanings lose nothing and are free;
- operations that **identify** distinct meanings (choose, cut, threshold, quantize) carry a ruler, and the ruler must be declared: `settle(n, by: r)` in mirror-005 is the template.

### §2.4 The shear in a parser: swapping ordered alternatives

**Proposition 2.4 (PROVED, by example).** In a PEG, `("a" / "ab") !.` fails on input `ab`, while `("ab" / "a") !.` succeeds. Swapping two alternatives keeps every summary that's cheap to check: the same multiset of alternatives, the same node count, the same set of literals, the same CFG reading. It still changes the language.
*Proof.* Ordered choice commits to the first success (Ford 2004 §3). On `ab` the first grammar takes `"a"`, then `!.` sees `b` and fails, with no backtracking into the choice. The second takes `"ab"`, and `!.` succeeds at the end. ∎

So in a PEG, **the order of alternatives is a ruler**: a declared priority. It is the parser's shear: it passes every trace-style check and changes meaning. Two consequences:
- children of a choice fragment are ordered (§1.3);
- "the grammar didn't change, I only reordered the alternatives" is not a free-move claim the normalizer may make.

### §2.5 Classification of the moves the brief names

| move | class | why | label |
|---|---|---|---|
| whitespace / layout change | free | trivia by declaration of `μ` | PROVED given `μ` |
| comment edit | free **only if** `μ` declares comments trivia | a docblock is `@nl` content (grammar.mirror: `comment = seq("#", @nl(until_newline))`) and kept as its own splinter (§3.3) | DECISION |
| α-renaming a bound variable | free | de Bruijn 1972: bound names are "otherwise irrelevant" | PROVED |
| renaming an exported name or rule | **ruler** (changes meaning) | the name is interface / output label | PROVED given §1.4 |
| reordering declarations | free | condensation order is canonical (§1.4) | PROVED |
| reordering PEG alternatives | **ruler** | Proposition 2.4 | PROVED |
| β-step / sugar elision (R1–R4) | free | rewrites inside `ker μ` by definition of the rules | PROVED given the rules are sound |
| filling a `\` hole | **ruler** (changes meaning) | a hole is a question; answering it is a choice | READING |
| `settle(n, by: r)` | ruler `r` | identifies distinct candidates | PROVED (non-injective) |
| quantizing a spectrum to 48 bits | ruler (bin edges) | identifies nearby spectra | PROVED (non-injective) |

---

## §3 Comma 3: bytes, concrete tree, normal form

### §3.1 Three representations and two maps

- `s ∈ Σ*`: the source bytes.
- `T = cst(s)`: the **concrete tree**. Every byte of `s` lies in exactly one leaf, either a token or a trivia leaf (whitespace, comment, layout). `print_cst(cst(s)) = s` holds by construction: the leaves, read in order, are `s`.
- `A = strip(T)`: the AST. Trivia leaves and source spans are removed.
- `N = nf(A)`: the normal form. Bound variables become de Bruijn indices, the declared rewrite rules are applied to normal form (§3.4), and each SCC's members are put in canonical order (§1.4).

`compile(s) = settle(nf(strip(cst(s))))`, and `oid(s) := addr(N)`.

### §3.2 What is erased, exactly

`strip` and `nf` erase five kinds of thing. Each is a free move only because `μ` declares it so (§2.2):

| erased | by | example |
|---|---|---|
| E1 layout trivia | `strip` | spaces, newlines, indentation |
| E2 comments | `strip`, **if** `μ` declares them trivia | `# …` |
| E3 source spans | `strip` | byte offsets of every node |
| E4 bound-variable names | `nf` (α) | `?x` in a pact rule, `|v|` in `select` |
| E5 elided redexes | `nf` (rules R1–R4) | an identity prism, a path-derivable `out` |

Exported names, rule names, literal values, the order of PEG alternatives and every `\` hole are **not** erased: they are meaning (§2.5).

### §3.3 Where the erasure goes: `settle` writes it, beside the splinter

**Definition 3.1.** `settle(T) = (N, ε)`, where the **erasure record** `ε` is a fragment at altitude `@mirror/erasure` that lists, for each erased item, its kind (E1–E5), its path in `N` (or in `A` for E5), and its bytes or name. `ε` has its own address. A lens splinter `source = lens(addr(N), addr(ε))` ties them together. Its address is the **source address** of the shard.

**Proposition 3.2 (bytes round trip; PROVED by construction).** If `ε` records every leaf of `T` that `strip` removes, with its position relative to the surviving leaves, and every α- and R-rewrite with its inverse data, then a renderer `render(N, ε) = s` exists. That makes `(N, ε)` a lossless decomposition of `T`: `N` is the view and `ε` the complement.
*Proof.* `strip` partitions the leaves of `T` into kept and erased, and `nf` is a finite sequence of rewrites, each recorded. Replaying the recorded inverses on `N` gives `A`, and re-inserting the erased leaves at their recorded positions gives `T`. Then `print_cst(T) = s`. ∎

**Proposition 3.3 (meaning round trip; CONJECTURE per grammar, testable).** For the canonical printer `print : N → Σ*` (the kintsugi formatter), `nf(strip(cst(print(N)))) = N`. Equivalently, `print` is a section of `compile` on normal forms. Then `print ∘ compile` is idempotent: formatting a formatted file changes nothing.
This holds iff the printer emits enough trivia to separate tokens and quotes literals the reader reads back. It is the kintsugi round trip ("`parse(project(r)) == r`" in composite #11), stated on normal forms, and the spec tests it on `mirror.mirror` itself.

So the answer to "is formatting trivia carried with the splinter or dropped?" is **neither**. It isn't in the splinter: if it were, a reformatted copy would have a different address. It isn't dropped either: without it the bytes couldn't be reconstructed. It goes into `ε`, which `settle` writes, and that is what "settle is the only op that logs erasures" means here.

**The brief's tension resolves at two altitudes.** `project` documented as lossy (prismqueer `lib.rs`: "the lossy transformation") against mirror-text specced as lossless (`docs/specs/kintsugi-ci-v0.1.md` §1.4): prism's own 2026-10-27 decision (prism `CURRENT.md` §7b″: "`project` is emission … append-only. It is not a spectral band projector") makes `project` **non-lossy** (an append of labels is injective). The `lib.rs` docstring predates that decision; that is drift on prism's side, and prism-Reed should hear about it. Loss happens at `settle`, and only there. mirror-text is lossless at the **meaning** altitude (it prints `N`, Proposition 3.3). The concrete tree is lossless at the **byte** altitude (Proposition 3.2). Neither needs the other.

**Comments (DECISION, Alex).** Mara-lean: a docblock is `@nl` content, a splinter in its own right, attached by a lens to the node it documents. It is outside the semantic root (a doc edit doesn't change the semantic id) and inside the source address (a doc edit changes the shard's stored bytes). The alternative, comments as meaning, would make every doc edit a new semantic id. The legacy grammar already lifts comments to `@nl` (`comment = seq(literal("#"), @nl(until_newline))`), which supports the lean.

### §3.4 Normalization: on which calculus, and does it terminate

**What the 2026-08-10 foundation called β.** `docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md` §1.2 defines four rewrite rules R1–R4 (identity-prism elision, glass-identity elision, path-derivable `out` elision, docblock-template elision) plus an exemption R5. These are **sugar elisions**, a term-rewriting system on the AST. They aren't λ-calculus β-reduction. Its Theorem 2.1 (termination: every rule strictly decreases node count) is correct for that system. I keep it. Its Corollary 2.2 ("computable in time linear in the AST's node-count") doesn't follow: the number of steps is linear, but finding each redex naively is not. That gives an `O(n²)` bound, linear only with an incremental redex index.

**What the loving-lambda documents contribute.** `docs/math/2026-09-04-…loving-lambda-calculus…` and `docs/math/2026-09-05-…mirror-algebra-is-loving-lambda-calculus…` name the calculus but state no reduction rules (grep: β appears only as a reference to "Rec #82 β-normal AST OID" and in the narrative of §1). So there is no rule set yet against which "β-normal on the loving lambda calculus" can be proved to exist. That's a gap to name, not to paper over.

**Definition 3.4 (the normalizer for this stage).** `nf` = α (de Bruijn indices for bound variables only; de Bruijn 1972) ∘ R1–R4 to normal form ∘ SCC canonical order. Named references are **not** unfolded (no δ-reduction).

**Proposition 3.5 (no unfolding of recursive references; PROVED).** Unfolding a reference that lies on a cycle of the reference graph never reaches a reference-free term. Each unfolding of a rule on a cycle reintroduces a reference to a rule on the same cycle.
So "β-normal" for a grammar keeps its recursion as names. The grammar is a system of equations, and its normal form is the canonical *system*, not a closed term. Unfolding is also excluded by §1.4: rule names label the output.

**Proposition 3.6 (normalization terminates; PROVED).** `nf` terminates on every finite AST: R1–R4 strictly decrease node count (2026-08-10 Theorem 2.1), α is one structural pass, and sorting an SCC is finite.

**Proposition 3.7 (normal form unique; CONJECTURE until critical pairs are checked).** Termination plus local confluence gives unique normal forms (Newman's lemma). The 2026-08-10 proof of confluence asserts R1–R4 are non-overlapping. That needs checking against nested identity prisms (an R1 redex inside an R2 redex) and against R5's path exemption, which makes the rules depend on context (the file path), not just on the term. The spec makes "enumerate critical pairs of R1–R4 over the corpus" a red test.

**Holes (PROVED).** No rule R1–R4 has `\` in its left-hand side, and α doesn't touch `\`. So a `\` is never part of a redex and never reduced: holes survive normalization as holes, and a partial stays partial. Holes don't break termination either, since they add no redexes.

**Hole identity (DECISION, Alex; Seam target).** Under §1.2, every bare `\` atom at the same altitude has the same address. If the hole's address is just `addr("\")`, then `mirror fate <hole_oid>` can't say *which* hole. Lean: a hole is addressed by the **question it asks**, i.e. its signature, the type it has to inhabit (`io f(x: a) -> b { \ }` gives the hole `hole(a → b)`). Two holes asking the same question share an address, so one answer serves both. That is content addressing applied to questions. The alternative is to address holes by path, which turns position into identity and runs against Proposition 1.7.

**If λ-β is added later.** Strong normalization holds for the simply-typed λ-calculus (Tait 1967) and fails for the untyped one (`(λx.xx)(λx.xx)`). So mirror's "sub-Turing" claim has to become a **typing discipline** before "β-normal form exists" can be a theorem for λ-terms. Until then the claim is CONJECTURE, and this stage needs no λ-β: `mirror.mirror` is a grammar, and grammars normalize by Definition 3.4.

---

## §4 Comma 2: parsing combinators and the five operations

### §4.1 The map

| combinator | operation | free / ruler | note |
|---|---|---|---|
| literal `"c"` | `focus` | free (carries the remainder) | selects the matching prefix and keeps the rest of the input in the beam |
| sequence `e₁ e₂` | **composition**, not an op | free | the monoid product of the pipeline (`|>`) |
| labelling a match with its rule name | `project` | free (append-only emission) | prism §7b″: "project is how you label data along the way"; this is where AST splinters are emitted |
| choice `e₁ / e₂` | `split` then `settle` | split free; settle carries the ruler (alternative order) | §4.2 |
| repetition `e*` | `μX. (e X) / ε` | as choice | needs recursion |
| reference `A` | **binding**, not an op | free | the letrec of §1.4; resolved by (SCC, index) |
| not-predicate `!e` | `split` then `settle` keeping the failure branch | ruler as choice | lookahead consumes nothing |
| `@nl(…)` / another grammar | `shift` | free iff invertible | same bytes, declared at another altitude |

### §4.2 Why three ops cannot express choice

**Proposition 4.1 (PROVED, given the Beam contract).** Let a pipeline be a word `w ∈ {focus, project, settle}*` (each letter applied by a fixed prism), with composition as concatenation. Suppose, as prismqueer's `Beam` contract says, a failed beam is dark: "Failure beams are dark: a fixpoint under `smap` and `next`" (`beam.rs`). Then no word expresses PEG choice `e₁ / e₂` from words for `e₁` and `e₂`, unless the branch is hidden inside the body of a single op.
*Proof.* A word that contains the word for `e₁` as a factor fails whenever `e₁` fails on the input that factor receives, because darkness propagates to the end. Choice must succeed when `e₁` fails and `e₂` succeeds, so the word for `e₁ / e₂` can't contain `e₁` as a factor run on the original input. A word without that factor doesn't consult `e₁` at all and can't implement "`e₂` only if `e₁` fails". The only other way out is an op whose body inspects a dark beam and continues from the **original** input. But `Beam::input()` "panics on dark propagated beams", so that op would have to hold the input outside the failing branch, which is a fork: a `split` hidden inside an op body. ∎

So **choice needs `split`** (a fork that keeps the input for the second branch) and **`settle`** (the join that picks a branch by a declared ruler). A PEG's ruler is "first success in declared order". It is declared by the syntax itself, so a PEG collapses early but **not** by an undeclared ruler. That answers Reed's item 5. An ambiguous (CFG-style) choice keeps both branches until `settle`. If more than one survives, that is a comma (`\`), never a silent pick.

### §4.3 Where `split` lives

`split` doesn't have to be a fourth method on `trait Prism`. prism's `CURRENT.md` §5 already proposes the Beam as the head of a content-addressed trace DAG with **Fracture** ("a node with several children (split: where the trace goes nonlinear)") and **Join** ("a node with several parents (settle)"). Choice is a Fracture followed by a Join. Per prism-002 §6 and §7b″, the semantics of `split` on the spectral triple (Divisi/Tutti on the Progression) live in prismqueer, and the verb as syntax lives in mirror. So mirror's grammar carries `split` as syntax, and the parser engine realises it as a fracture in its trace. Prism's trait can stay at three methods. **DECISION (prism-Reed):** land the trace DAG's Fracture/Join, or accept that mirror carries the fork in its own engine until it does.

### §4.4 The five ops, classified (Reed's item 5, with one correction)

- `focus`: free if the complement (the unconsumed input) is carried, which is true in parsing. A `focus` that discards the complement is a projection and carries a ruler.
- `project`: **free**. Reed's proposal said "lossy, needs declared erasure", but that was the reading before prism's 2026-10-27 decision. As emission it is an injective append (Proposition 2.3).
- `split`: free (a copy into branches is injective).
- `shift`: free iff invertible (a basis change). A `shift` into `@io` is an effect, outside `μ` altogether (composite #10).
- `settle`: the **only** ruler-carrying op and the only one that logs erasure.

**Proposition 4.2 (PROVED, given the above semantics).** In a composition of the five ops, every loss of meaning happens at a `settle`, and is therefore visible in that `settle`'s declared ruler and erasure record.
*Proof.* A composite of injective maps is injective, so a pipeline loses meaning only if some letter is non-injective, and by the classification only `settle` is. ∎

That gives the free/ruler criterion as a grammar rule: **a `settle` without a declared ruler is a compile error; every other op needs none.**

---

## §5 One engine: parsing and inference as rotations

### §5.1 A PEG is a positive Datalog program

Fix a PEG `G` and an input `s` of length `n`. Over positions `0..n`, define `m(e, i, j)` ("`e` matches at `i` and ends at `j`") and `f(e, i)` ("`e` fails at `i`"). For every sub-expression `e` of `G`:

```
ε        m(ε,i,i).
"c"      m("c",i,i+1) :- input(i,"c").          f("c",i) :- ¬input(i,"c")   (EDB, decided by the input)
A        m(A,i,j) :- m(R(A),i,j).               f(A,i) :- f(R(A),i).
e₁ e₂    m(e₁e₂,i,k) :- m(e₁,i,j), m(e₂,j,k).   f(e₁e₂,i) :- f(e₁,i).   f(e₁e₂,i) :- m(e₁,i,j), f(e₂,j).
e₁ / e₂  m(e₁/e₂,i,j) :- m(e₁,i,j).            m(e₁/e₂,i,j) :- f(e₁,i), m(e₂,i,j).   f(e₁/e₂,i) :- f(e₁,i), f(e₂,i).
e*       m(e*,i,i) :- f(e,i).                  m(e*,i,k) :- m(e,i,j), j>i, m(e*,j,k).
!e       m(!e,i,i) :- f(e,i).                  f(!e,i) :- m(e,i,j).
```

**Proposition 5.1 (PROVED; COMPUTED).** The least model of this program equals Ford's ⇒ relation on the suffixes of `s`. No negation is needed, because failure is derived positively.
*Proof.* Ford 2004 §3.3 defines ⇒ inductively, with one rule per expression form (and a step counter `n`, which the Datalog form drops because it only measures derivation length), and failure `f` is an outcome of ⇒ like success. Each inference rule is one of the Horn clauses above, read over positions instead of suffixes. The least fixed point of an inductive definition is the least Herbrand model of its rules. The `j > i` side condition is Ford's: `e*` over a possibly-empty `e` is not well-formed (§3.6, `WF(e*)` requires `e` not to succeed without consuming input). ∎
*COMPUTED* (`peg_datalog.py`, seed `20261028`): over 2000 random three-rule grammars and inputs of length 0–5, the least model agrees with a direct packrat interpreter on all 1480 cases where packrat terminates. In the 520 cases where packrat detects a loop, the start atom has **neither** `m` nor `f`: an undefined atom.

**Corollary 5.2 (PROVED, from Ford).** Ford: "A well-formed grammar G is complete" (every expression either succeeds or fails on every input), and "the relation ⇒G is a function". So for a well-formed grammar the least model has, for every `(e, i)`, exactly one `m(e, i, j)` or one `f(e, i)`. Left recursion (not well-formed) shows up as an **undefined atom**: in mirror's words a `\`, not a crash and not a silent failure. Ford proves completeness undecidable in general ("It is undecidable whether an arbitrary grammar is complete"). Well-formedness, though, is a structural property that "can be computed by iteration to a fixed point" (Ford §3.6). So `mirror.mirror` is checked for well-formedness statically, and that check is itself a least fixed point.

This also places the parser in the tradition Shieber, Schabes & Pereira (1995) named "parsing as deduction": "Parsing algorithms can be represented directly as deduction systems, and a single deduction engine can interpret such deduction systems so as to implement the corresponding parser." Their engine is agenda-based, and they relate it to semi-naive bottom-up evaluation.

### §5.2 Semi-naive evaluation is a rotation

**Definition 5.3 (rotation).** For a monotone operator `T` on the finite powerset of ground atoms: `M₀ = facts`, `Δ₀ = M₀`, `Δ_{t+1} = T_Δ(M_t, Δ_t) ∖ M_t`, `M_{t+1} = M_t ∪ Δ_{t+1}`. Only rules with a body atom in `Δ_t` are re-fired at tick `t+1` (Bancilhon 1986: "a simple differential calculus on relational expressions" to avoid naive evaluation's duplicate computation). Stop at the first `t` with `Δ_{t+1} = ∅`.

**Proposition 5.4 (PROVED, standard; COMPUTED).** For a positive program over a finite domain, semi-naive and naive evaluation reach the same least fixed point in at most `|B|` ticks, where `B` is the set of ground atoms. *Proof.* Monotonicity: a rule instance that fires in the naive round `t+1` has some body atom new at round `t`, otherwise it fired earlier. Each non-final tick adds at least one atom of `B`. ∎ *COMPUTED:* equal models on all 2000 programs above.

With §1.5's deposit discipline this is Reed's picture, made exact: a **tick** is a round, a **deposit** is a newly derived atom (a splinter at altitude `@parse/item` or `@pact/fact`), only the **delta** joins, and the **fixed point** is the first tick where nothing new falls. Proposition 1.7 is why the round number has to stay out of the address.

### §5.3 The pact evaluator: two rotations per round, with retractions

`rust/src/pact.rs` (`1324aae`) implements the well-founded semantics by the alternating fixpoint (Van Gelder 1993: "monotonically to build up a set of negative conclusions until the least fixpoint is reached"; Van Gelder, Ross & Schlipf 1991). `gamma(S)` is the least model with `not a` read as `a ∉ S`. With `S` fixed it is a **positive** program, so the rotation of §5.2 computes it. `under` iterates `gamma ∘ gamma` upward from the facts, and `over = gamma(under)`.

**Proposition 5.5 (PROVED).** `gamma` is antitone, so as `under` grows, `over` shrinks: the outer iteration **retracts** atoms on the over-estimate side. Example: rules `b :- not a.  c :- not b.  d :- not c.`, fact `a`. Then `over₀ = gamma({a}) = {a, c, d}`, `under₁ = gamma(over₀) = {a, c}`, `over₁ = gamma(under₁) = {a, c}`: `d` is retracted.

So the outer loop isn't an inflationary rotation, and "only the delta joins" fails there: a retraction can enable or disable rules anywhere. In an append-only store (git) a retraction is a **new deposit**, a status splinter recording that `d` moved from undefined to false at round 1, never a deletion. The well-founded model has `true = under`, `undefined = over ∖ under` (the comma), `false` = the rest. pact.rs never reads undefined as false.

### §5.4 Does the parser share an engine with the pact evaluator, and with `rotation.rs`? (Reed's candidate 4)

- **Parser and pacts: yes, at the core.** Both are least fixed points of positive programs over finite domains, evaluated semi-naively over content-addressed deposits (Propositions 5.1, 5.4). The pact evaluator adds an outer alternating loop that calls the core twice per round (Proposition 5.5). A PEG needs no outer loop: failure is positive (§5.1). **PROVED.**
- **`rotation.rs`: only the shape.** `infer_via_rotation` runs a *fixed* number of ticks (`ticks` parameter), never tests for "nothing new fell", puts `tick` in the address (so by Proposition 1.7 it couldn't detect a fixed point anyway), and iterates a continuous dynamical system (Kuramoto phases). There is no finite lattice and no monotone operator, so the guarantees that make Datalog work (termination within `|B|`, a unique least fixed point independent of evaluation order) don't apply. The coupling `K`, the step `dt` and the frequency spread `omega_scale` (`RotationConfig::default() = 1.0, 0.05, 0.3`) are rulers. The shared part is **deposit, delta, compare**, and nothing more. "Same operator at two altitudes" is a **READING** at the level of shape, and **false** at the level of guarantees.
- **Cost.** Packrat is linear (Ford 2002: "guarantees linear parse time") because each `(e, i)` is computed once. The Datalog formulation reaches linear time only if the engine indexes `m` by `(e, i)` and uses Ford's functionality (at most one `j`), so that a sequence rule joins in O(1). An unindexed ground program is cubic in `n` (my script's naive grounding is). **CONJECTURE** for the shared engine with that index. Left recursion would break it: Warth et al. 2008 note their left-recursion extension can "yield super-linear parse times for some left-recursive grammars". `mirror.mirror` is required to be well-formed, so it has none.

**Termination of the parse, related to `halts.mirror`.** `boot/std/epistemologic/property/halts.mirror` halts on "(a) autopoietic settlement … fixed_point" OR "(b) reduction exhaustion … budget". A parse of a well-formed grammar halts by **(a)**: the rotation reaches its least fixed point within `|B| ≤ |E|·(n+1)² + |E|·(n+1)` ticks (PROVED, Proposition 5.4). A non-well-formed grammar still halts by (a), since the least model always exists, but leaves undefined atoms. Clause (b) is needed only for steps that are not monotone over a finite domain (Kuramoto, the stage iteration of §6.3). That is the correct division: **monotone and finite gives a fixed point for free; anything else needs a declared budget, which is a ruler.**

---

## §6 The fixed point: bootstrap as ordinary self-hosting

### §6.1 Stages

Let `G` be the bytes of `mirror.mirror`, `L₀` the subset of mirror syntax that `G` uses, and `E` the engine of §5 (a PEG evaluator that takes a grammar normal form and returns a reader).

- **Stage 0.** A seed reader `P₀` for exactly `L₀`. `N₀ = nf(strip(P₀(G)))`, `o₀ = addr(N₀)`.
- **Stage 1.** `P₁ = E(N₀)`: the parser read off the self-description. Its identity is the pair `(addr(E), o₀)`. The root OID identifies the **grammar**. It identifies the parser only together with the engine. If `E` is itself a mirror shard (Alex: the evaluator lives in mirror), `addr(E)` is a shard address too, and the regress stops at the floor that executes `E`. That floor is the irreducible witness, and it should be named as such rather than hidden.
- **Stage 2.** `N₁ = nf(strip(P₁(G)))`. The **fixed-point equation** is `addr(N₁) = o₀`.
- After stage 2 the seed is only a witness: `P₁` reads `mirror.mirror` and every later change to it.

Define `F(N) = nf(strip(E(N)(G)))`. Stage 2 says `F(N₀) = N₀`.

### §6.2 Existence, uniqueness, and what the fixed point does not prove

**Proposition 6.1 (existence is by exhibition, not by a theorem; READING + precedent).** No general theorem supplies a fixed point of `F`. It exists iff `mirror.mirror` correctly describes the syntax it is written in **and** `P₀` and `E(N₀)` agree on `G`. The precedent that such self-descriptions exist and are small: Ford 2004, Figure 1, "PEG formally describing its own ASCII syntax", which "describes its own complete syntax including all lexical characteristics". Kleene's second recursion theorem (1938) and Lawvere's fixed-point theorem (1969) guarantee self-reference in settings far stronger than this one, namely acceptable numberings of all partial recursive functions, and cartesian closed categories with a point-surjective `A → Y^A`. Neither is needed here, and neither applies as stated to a sub-Turing grammar class. The fixed point is checked, not inferred.

**Correction to `docs/math/2026-08-22-mara-recognition-94-…-lawvere-fixed-point-closure-math-foundation.md` §Q+M0.1.** It paraphrases Lawvere as "every endomorphism of the terminal object has a fixed point". The theorem says: if `φ : A → Y^A` is point-surjective, then every endomorphism `f : Y → Y` has a fixed point (Lawvere 1969; statement checked against the secondary summary, Wikipedia "Lawvere's fixed-point theorem", not against the LNM text). The terminal object's endomorphisms are trivially fixed. The paraphrase misplaces the content of the theorem, so Rec #94's convergence argument doesn't rest on what it cites. I'm not re-deriving Rec #94 here, only marking that this document doesn't lean on it.

**Proposition 6.2 (uniqueness fails in general; PROVED).** `F` can have several fixed points, and passing stage 2 certifies only that the pair (`P₀`, `E`) agrees with itself on `G`.
*Proof.* Thompson 1984 shows a compiler binary that reproduces itself while miscompiling chosen patterns: "We must 'train' the compiler"; the trojan "will deliberately miscompile source whenever a particular pattern is matched". Translated here: a seed `P₀` that reads one construct of `G` wrongly, in a way that `E(N₀)` then reproduces (because `N₀` encodes the misreading), passes stage 2. The equation `F(N) = N` constrains the reading only up to self-consistency. ∎
**Countermeasure (Wheeler 2005, diverse double-compiling):** "recompile the source code twice: once with a second (trusted) compiler, and again using the result of the first compilation." Here that means computing `o₀` with a second, independently written seed `P₀'`, e.g. a reader in another language, or the legacy `boot/` grammar as a witness. A misreading would then have to be replicated in both. The spec makes this a test (DDC) and is honest that it raises the cost of a consistent misreading without excluding it.

### §6.3 Iterating beyond stage 2, and `halts.mirror`

`N_{k+1} = F(N_k)`. If `G` is fixed and well-formed, every `N_k` is a normal form of a parse of `G` whose labels are names occurring in `G` and whose depth is bounded by `|G|·|rules|` (no ε-loops in a well-formed grammar). So the `N_k` range over a finite set.

**Proposition 6.3 (PROVED).** The orbit `N₀, N₁, …` of a function on a finite set is eventually periodic. Three outcomes: a fixed point (period 1: green), a cycle of period > 1 (an oscillating self-description: a comma, `\`), or failure (some `E(N_k)` doesn't parse `G`: dark).

`F` is not monotone on any lattice the parser knows, so no Knaster–Tarski argument bounds the time to the period. This is `halts.mirror` clause (b): the iteration needs a **declared budget** `k_max`, and the budget is a ruler (§9). The spec asks for period 1 at `k = 1`, which is the strongest and cheapest form.

---

## §7 Addresses: ours, prism's, git's, and the spectral uuid

### §7.1 Four identifiers, never conflated

| identifier | preimage | hash | width | role |
|---|---|---|---|---|
| `addr(N)`: semantic root | canonical encoding of the normal form (§1.2) | `H` (§7.2) | 256 | meaning identity; the parser's id |
| source address | lens over `addr(N)` and `addr(ε)` (§3.3) | `H` | 256 | the shard's bytes, recoverable |
| git object id | `"<type> <len>\0" ‖ payload` | SHA-1, or SHA-256 in a `sha256` repo | 160 / 256 | storage and distribution |
| `uuid_spectral` | `active₄₈ ‖ dark₈₀` | (§7.4) | 128 | semantic id for routing; `dark` = identity |

**Proposition 7.1 (PROVED).** `addr` and the git id are never equal as strings, except by a collision across two different preimage spaces. They are different functions of the same canonical structure. Git's header (verified, Pro Git §10.2: "a header which starts by identifying the type of object … a space followed by the size in bytes of the content, and adding a final null byte") and our prefix (`"splinter\0"`, `"fragment\0"` plus tag) make the preimages differ.

### §7.2 prismqueer's `Oid::hash` is not a cryptographic hash of its input (COMPUTED)

`Oid::hash(bytes) = canonical_hash(bytes)` (`oid.rs`), the `CoincidenceHash<3>` detector (`coincidence.rs`). Reading the code and reimplementing it in Python (`oidhash.py`, which reproduces the pinned vector `Oid::hash(b"prism") = 08f8e91d230c49a5072202e4e82db8306e226d83f77aa6f57d05dc87b56efc1e`; I also ran prism's own test `oid_hash_cross_version_stable`: ok):

1. `encode_into_basis` maps the input to a 16-dimensional `f64` vector `s(x) = Σᵢ e(i, xᵢ)`, a sum of pseudo-random vectors, one per (position, byte).
2. Each of the three projections is rank one (`Projection::rank1`, `P = v vᵀ`), so `P s = v ⟨v, s⟩`. COMPUTED: for each projection, the ratio of projected entries to `v` collapses to one value.
3. SHA-256 is applied to `"prism-core:coincidence:" ‖ "coincidence:" ‖ 3 ‖` the three projected vectors' bytes, i.e. to a function of **three `f64` scalars** `⟨v_k, s(x)⟩`.

**Proposition 7.2 (PROVED from the code).** Any two inputs with bit-identical triples `(⟨v₁,s⟩, ⟨v₂,s⟩, ⟨v₃,s⟩)` get the same `Oid`. The digest's collision resistance is therefore bounded by that of the map `x ↦` three float sums, at most 192 bits of state, which is linear in per-position contributions. It is not SHA-256's.
**CONJECTURE (Seam):** collisions are findable far below `2¹²⁸` work. The structure is a sum of independent per-position terms, which is the setting of Wagner's generalized birthday problem (Wagner 2002, k-tree algorithm); exact float equality is the obstacle to measure.
**COMPUTED (portability).** The digest depends on floating-point summation order. My first port disagreed with the pin, and only because Python 3.12's `sum()` is compensated while Rust's `Iterator::sum` is sequential. Any second implementation (the BEAM port in scout-spectral-uuid, a mirror-in-mirror engine) has to replicate IEEE-754 operation order exactly. The path also contains rulers: `f64::EPSILON` filters drop small coefficients inside the hash.

**DECISION (Alex + prism-Reed).** The root address of a self-hosting compiler should be a hash of the canonical encoding with no float arithmetic on the path. BLAKE3 is what `2026-08-10-…beta-normal…` §3.1 and `uuid_spectral` already specify ("BLAKE3-truncated content hash"). The coincidence detector can stay what it is good for (a measurement), but not the identity. Until then `addr` is specified as BLAKE3-256 and is a different function from `prismqueer::Oid::hash`. The spec treats them as two columns, never one.

### §7.3 Back-projection into git

AGENTS.md: "Git IS the crystal store." Alex 2026-09-04: "the only thing mirror adds is 'save the content addressed crystals for the shards to disk as git projection of the graph'." Precedent: fragmentation's `write_tree` already writes "Shard -> blob, Fractal -> tree with .data + numbered children" (`fragmentation/vcs/git/src/git.rs:79-97`, per Taut's scout).

**Definition 7.3 (the projection `g`).**
- atom → **blob** whose payload is the atom's canonical encoding, including the altitude tag;
- fragment with label `ℓ` and children `x₀ … x_{k−1}` → **tree** with an entry `@` (blob: `τ ‖ ℓ`) and entries `000 … ` (zero-padded to a fixed width per tree, so git's byte-order sort of entry names is the child order), each pointing at `g(xᵢ)` with mode `100644` (blob) or `040000` (tree);
- lens → tree with `@` and entries `^000 …` for its targets. That makes git reachability equal splinter reachability, so `git gc` keeps what the graph keeps;
- references inside an SCC are `(SCC, index)` in content, never tree entries. Git's object graph is acyclic by construction (a tree's id depends on its entries' ids), which is Proposition 1.5's condensation again.

**Proposition 7.4 (PROVED; COMPUTED).** On the splinters reachable from a root, `g` is a bijection onto the git objects reachable from `g(root)`, and it commutes with children: `g` is an isomorphism of the two Merkle DAGs, barring a collision in either hash. *Proof.* Both addresses are Merkle functions of the same canonical tree (Proposition 1.1 twice). Decoding a tree's `@` entry and its ordered numbered entries inverts Definition 7.3. ∎ *COMPUTED* (`git_projection.py`, a throwaway repo): for a toy grammar, `git cat-file -t` on the root is `tree`; walking it reproduces the AST; a reformatted source gives the same `addr` **and** the same git id; changing one literal changes both; `addr ≠` git id.

The existing `@mirror/store/git` (`shards/mirror/store/git.mirror`) stores fragmentation objects in a namespace under `.git/mirror/`. Those are not native git objects that `git cat-file` can read. The back-projection here writes native objects. **DECISION (Alex):** supersede the namespaced wire, or keep both with the namespaced store as a cache. AGENTS.md's "no other store" argues for superseding.

### §7.4 The semantic spectral uuid falls out of compile (Alex: "from the Merkle DAG the semantic spectral uuid emerges")

**`dark`.** `dark = addr(N)[0..80)` with `H` = BLAKE3, which matches the spec (`prismqueer::SpectralUuid::from_parts` takes "the first 10 bytes" of a 32-byte hash).
**Proposition 7.5 (PROVED).** `dark` is invariant under every free move (Proposition 2.1) and changes whenever meaning changes, except for 80-bit collisions (birthday bound around 2⁴⁰ items, as the scout notes). This satisfies scout-spectral-uuid rule 1 ("dark must hash the MEANING") **by construction**.

**`active`: which graph.** Taut's scout (`docs/scouts/2026-10-28-taut-graph-magic-dag-to-laplacian.md` §1–§2) finds that no landed path keeps the DAG. `coincidence::Laplacian::from_tree` allocates "a vertex per *occurrence*", and prism's `Graph` is "undirected and simple (`BTreeSet<EdgeKey>`)", so parallel edges collapse. So the graph has to be defined here.

**Definition 7.6.** `Γ(N)`: vertices = distinct addresses reachable from the root (hash-consed). There is one undirected edge per (parent, child-slot) pair, **with multiplicity** (`f(x, x)` gives two parallel edges). Atoms and fragments are vertices. The erasure record and witnesses are not part of `Γ`.

**Lemma 7.7 (reuse is `b₁`; PROVED).** If every vertex of `Γ(N)` is reachable from the root, `Γ` is connected and `b₁(Γ) = |E| − |V| + 1 = Σ_{v ≠ root} (indeg(v) − 1)`: the total number of reuses. A tree has `b₁ = 0`, so the per-occurrence expansion erases exactly this information. *Proof.* `|E| = Σ_v indeg(v)`, the root has in-degree 0, and Euler's formula for connected graphs gives `b₁ = |E| − |V| + 1`. ∎ prism's `Instrument` already returns `dim ker D = b₀ + b₁` (`instrument.rs:220-221`, per Taut).

**Lemma 7.8 (directed operators see nothing or only degrees; PROVED; COMPUTED).** Order the vertices of a DAG topologically. Then the adjacency `A` is strictly upper triangular, hence nilpotent with spectrum `{0}`, and `D_out − A` is upper triangular with diagonal `D_out`, so its spectrum is the multiset of out-degrees. *COMPUTED* (`spectra.py`, a random 9-vertex DAG; Taut independently on three DAGs). prism's `docs/math/constrained-inference-floor.md:217-234` already states the adjacency half ("Do not decide this with floating eigenvalues").

**Lemma 7.9 (uniform-phase magnetic Laplacian sees only unequal path lengths; PROVED).** With phase `θ` on every directed edge, the flux around a reuse loop made of two directed paths `r ⇝ c` of lengths `ℓ₁, ℓ₂` is `θ(ℓ₁ − ℓ₂)`. Equal-length diamonds are gauge-trivial and invisible. *Proof.* The flux around a cycle is the signed sum of edge phases along it. One path is traversed forward and the other backward. ∎ (Taut COMPUTED the same on a diamond and a triangle.) Making the magnetic operator see every reuse needs per-edge phases with meaning, which is a ruler. When it's wanted, the solver exists today: an `n × n` Hermitian problem `A + iB` is equivalent to the real symmetric `2n × 2n` problem `[[A, −B], [B, A]]`, whose eigenvalues are those of the Hermitian matrix, "each … repeated twice" (Numerical Recipes, §11.4), so `dsyev` suffices.

**Recommendation (Mara-lean, DECISION Alex).** `active` reads the **undirected Laplacian of `Γ(N)` with multiplicity** (Taut's candidate (a)). It is intrinsic: it reads only the crystal's own DAG, which is content, not context. That reconciles `docs/specs/spectral-hash-design.md:211` ("without requiring graph context") with uuid_spectral's "local Laplacian … within a shard's splinter OID graph". It sees all reuse (Lemma 7.7), and it needs only `dsyev`.

**Proposition 7.10 (relabelling invariance; PROVED).** For a permutation `P`, `P L Pᵀ` is similar to `L`, so the spectrum doesn't depend on vertex order. That satisfies scout rule 2 (intrinsic, never contextual), provided `Γ` is built from content alone, as in Definition 7.6.

**Proposition 7.11 (active never identifies; PROVED; COMPUTED).** Cospectral non-isomorphic graphs exist: `K₁,₄` and `C₄ ∪ K₁` share the adjacency spectrum `{−2, 0, 0, 0, 2}`. The smallest Laplacian-cospectral non-isomorphic pair has 6 vertices, and the smallest adjacency-cospectral non-isomorphic trees have 8 (COMPUTED, `spectra.py`, by brute force). Schwenk 1973 ("Almost all trees are cospectral") and Godsil & McKay 1982 ("new and powerful constructions for pairs of cospectral graphs") give the general picture. An AST without reuse is a tree, so `active` collisions among ASTs are **generic, not exotic**. Identity reads `dark` only; `active` routes.

**What isn't settled, and so can't be quantized yet (DECISION Alex; Taut §4).**
1. **Three incompatible definitions of `SpectralCoordinate<5>`** (fragmentation `mirror-native-vcs.md:1095-1101`: λ₂, λ₅−λ₂ and three heat traces; `rung-8-9-…:162-166`: void dualities; prism `spectrum.rs`: Split/Refract/Project/Focus/Lift). The landed `SpectralCoordinate` is SHA-seeded random projections of bytes, i.e. none of the three. In prism's set, `Project = √(2·λ₂)` is a function of `Split = λ₂` (`spectrum.rs`), so it carries no independent bits, and `Focus` and `Lift` are not computed. That leaves **two** independent axes landed (λ₂ and the von Neumann entropy). Mara-lean: the fragmentation definition, because it reads the Laplacian and its heat traces carry multiplicity information. But `mirror-native-vcs.md` itself disagrees on whether `D² = L₀` or `L₀ + L₁` (`:1024` vs `:1104`), and only `L₀ + L₁` carries `b₁`. With Lemma 7.7, the lean is `L₀ + L₁`.
2. **Bit budget.** 48 / 5 is not an integer; `reality-shard-as-crdt.md:312` says 12 bits per dimension (60 total). The split (e.g. 10/10/10/9/9) is a ruler.
3. **Storage vs. identifier.** `store-vs-db-and-the-cascade.md:48-61` forbids a spatial coordinate in the storage OID, while uuid_spectral puts one in the identifier. Reading: there is no conflict if the storage address is `addr` (with no active bits) and `uuid_spectral` is a separate identifier whose identity half is `dark`. Then what the 2026-05-30 posture forbids (a contextual coordinate in the storage address) doesn't happen.
4. **`active = 0` everywhere in production** (Taut §1, Stage 6). Nothing has to migrate yet, which is the cheapest moment to fix the definition.
5. **The `combine` law.** `shards/uuid/spectral.mirror` says active halves compose "via SpectralCoordinate's eigenvalue addition law (to first order)". **Counterexample (PROVED):** the disjoint union of two `K₂` has Laplacian spectrum `{0,0,2,2}` and Fiedler value 0, while the sum of the parts' Fiedler values is 4. Spectra of unions are multiset unions, not sums. The homomorphism claim for `active` fails, and only `dark`'s Merkle rule composes.

**The ruler for `active` is system-wide (PROVED).** The uuid has no room to name which quantization produced its active bits: 128 bits, all used. So two issuers' active halves are comparable only if both used the same quantizer, the same operator and the same graph construction. These are `@ruler` crystals (§9), **fixed system-wide and signed by every issuer**. An active half issued under an unsigned or different ruler isn't comparable, and the spec says so.

---

## §8 The universal transformation: one engine, every wire format, no serde

Alex: "remove serde in favour of mirror doing the transformation itself"; 2026-09-02: "all wire formats are mirror type declarations + parse/emit bodies with `\` for partials".

**Definition 8.1.** A wire format is a pair `(G_fmt, μ_fmt)`: a grammar (a PEG with its rule labels) and a meaning declaration (trivia, binders, normalization rules). Then `read_fmt = settle ∘ nf ∘ strip ∘ E(G_fmt)` and `emit_fmt = print_fmt` (the canonical printer). mirror-text is one such pair, JSON is another, and `mirror.mirror` is the pair for mirror itself.

**Proposition 8.2 (PROVED, by construction).** One engine `E` reads every format. `serde` disappears because a format is *data* (a grammar plus `μ`), not code. The per-format obligations are Proposition 3.3 (the meaning round trip) and well-formedness (Corollary 5.2). Both are checked per grammar.

**JSON as a grammar: what `μ_JSON` must declare.** RFC 8259: "An object is an unordered collection of zero or more name/value pairs". So member order is **free** under `μ_JSON`, and the normal form sorts members. **mirror-001's first red test** (two events differing only in key order and whitespace get the same id) is then Proposition 2.1 applied to `μ_JSON`, not a special case. Each of the following is a **ruler** to declare, not to assume:
- duplicate names (RFC 8259 leaves the behaviour unpredictable) → lean: `\` (a comma), never last-wins;
- number equivalence (`1`, `1.0`, `1e0`) → lean: exact decimal, never `f64`;
- string normalization (escapes are free; Unicode normalization such as NFC is a ruler; prism's gate ids already chose NFC, per prism `CURRENT.md` §7b″).

**Partials.** An input that leaves undefined atoms (Corollary 5.2), or a value a format can't express, is a `\`, never an error and never a silent drop.

**Cross-format `shift`.** Reading a value from mirror-text and writing it as JSON is `shift` between two formats. It is free iff the value lies in both formats' expressible range. JSON has no `\` and no `@`-references, so `shift` into JSON is **partial**: what it can't carry is logged as erasure and the output marks the gap. That's why JSON lives only behind `--out @data/json`, an `@io` crossing.

**Consequence for `pact.rs`** (`1324aae`): it parses with `serde_json` (`use serde_json::{json, Value}`). Under this section it is a reader for `G_pact-json` waiting to become `E(G_pact-json)`. Its vectors (`tests/pact/`) stay the oracle.

---

## §9 Rulers in the code today (declare each as an `@ruler`)

Every constant that identifies distinct values is a ruler (§2.3). Inventory, all from the pinned prism `7e6b00f` unless stated:

| ruler | where | value |
|---|---|---|
| commutator numerical zero / clearly non-zero | `spectral/commutator.rs:93,97` | `1e-12`, `1e-6` (× Frobenius scale) |
| Refract cut-off | `spectral/spectrum.rs:110,116` | eigenvalues `≤ 1e-12` dropped |
| settle noise / harmonic tolerance | `spectral/settle.rs:168,282` | `16·ε·scale·n`, `Noise(1e-6)` |
| Kuramoto coupling, step, spread | `spectral/rotation.rs` `RotationConfig::default` | `K = 1.0`, `dt = 0.05`, `omega_scale = 0.3` |
| rotation tick count | `infer_via_rotation(…, ticks, …)` | caller's choice (a budget) |
| coefficient filters inside `Oid::hash` | `coincidence.rs` | `f64::EPSILON` (§7.2) |
| fragility threshold | `coincidence.rs:256` | `1e-6` (computed both branches identical, so currently inert) |
| quantizer for `active` | absent (Taut §1 Stage 6) | must be system-wide (§7.4) |
| stage-iteration budget | §6.3 | `k_max` |
| PEG alternative order | every choice in every grammar | declared by position (§2.4) |

Instrument resolution (machine epsilon) is **declared**, a property of the instrument. The multiplier chosen on top of it is **signed**. That follows Reed's `@ruler` shape point 6, and Proposition 2.3 is why it is the right split: the discrete metric needs no signer, and everything finer does.

---

## §10 What the corpus already said: kept, sharpened, superseded

| document | kept | sharpened | superseded |
|---|---|---|---|
| `shards/glass.mirror` (2026-06-06) | splinter = universal atom; atom/fractal/lens; transparency outside identity | fragment = fractal-shaped splinter; address formula §1.2; ordered children | — |
| `2026-07-22-splinter-eigen-fragment-ouroboros-closure.md` | splinter at every altitude | — | §3 "fragment = `@eigen(Crystal<T>)`" (fragment is now any composite) |
| `2026-08-10-…beta-normal…` math + spec | hash the normal form (Dhall's semantic integrity check: "a SHA-256 hash of the binary representation of an expression's normal form"); R1–R4; termination by size | "β" is sugar elision, not λ-β (§3.4); confluence needs a critical-pair check (Prop. 3.7) | Cor. 2.2 "linear time" (not shown); `H = Fold5` content encoding (bootstrap deleted) replaced by the encoding of §1.2 |
| `2026-09-08-…crystal-fractal-splinter…` | Alex: "Crystal → Fractal(spectral uuid addressed) → Splinter (AST)"; `\` as a value | splinter's address excludes tick (Prop. 1.7) | Thm M2.1 "same operator at every altitude": true of the deposit shape, false of the guarantees (§5.4) |
| `docs/specs/parser-as-prism-grammar.md` (2026-05-21) | grammars as combinator trees; kintsugi round trip; `dark_fallback` | dark spans = undefined atoms (Cor. 5.2) | `AstKind`/`Fold5` five-op AST (deleted with `bootstrap/`; prism CURRENT §7d) |
| `docs/specs/generated-parser-spec.md` (2026-04-14) | the bootstrap phase diagram as an idea | — | already marked superseded (2026-05-19); codegen replaced by interpretation `E(N)` |
| `boot/std/mirror/grammar.mirror` | `seq`/`literal`/`choice`/`repeat`; comment lifts to `@nl` | used as the second witness (DDC, §6.2) | as the grammar (no `pact`, `requires`, `\`) |
| `2026-08-23-…mq-graph-native…` | — | — | Cor. M8.C4 (MQ strictly more expressive than Datalog): false as the brief notes, since MQ patterns can't express transitive closure; this document's engine is Datalog |
| `2026-08-22-…recognition-94…` | — | — | §Q+M0.1's statement of Lawvere (§6.2) |
| `shards/uuid/spectral.mirror` | `active ‖ dark`; identity reads dark | — | "active composes via eigenvalue addition" (counterexample §7.4) |
| prismqueer `lib.rs` docstring "project — the lossy transformation" | — | — | by prism's own §7b″ ("project is emission"); drift to report |

---

## §11 Where Seam should aim first

1. **Proposition 1.7 and its consequence.** Is there a rotation that needs the tick in the address? If the answer is "time-series observations", show that the deposit record doesn't carry it equally well.
2. **Proposition 4.1 (choice needs `split`).** It rests on the Beam contract (darkness absorbing; `input()` panics on dark). Find a three-op encoding of ordered choice that doesn't hide a fork in an op body.
3. **Proposition 5.1 (PEG = positive Datalog).** 1480/1480 is random testing over three-rule grammars with inputs up to length 5, not a proof by exhaustion. Attack the `!e`, `e*` and well-formedness boundary.
4. **Proposition 7.2 (`Oid::hash`).** Produce a collision, or bound the work.
5. **Proposition 6.2.** Construct a concrete seed misreading of a construct of `mirror.mirror` that passes stage 2. That turns the Thompson argument into a test vector.
6. **§2.2.** Is there a free move that `μ` can't be made to declare, i.e. a representation change everyone agrees is meaning-preserving that the three declaration points (trivia, binders, rules) can't express?
7. **Proposition 3.7 (confluence of R1–R4).** R5's path dependence makes the rewriting context-sensitive. Check whether that breaks Church–Rosser.
8. **Hole identity (§3.4).** Two holes with the same signature in different shards share an address under the lean. Is that a feature (one answer serves both) or a leak (an answer meant for one context lands in another)?
9. **Lemma 7.7 + Definition 7.6.** Does multiplicity-preserving `Γ` make `active` stable under every free move? Sorting SCC members, de Bruijn indices and R1–R4 all change the DAG. They are free moves applied **before** `Γ` is built, so `Γ(N)` sees only normal forms. Check that nothing builds `Γ` from a pre-normal tree.

---

## Sources (verified via Kagi this session unless marked)

Read (primary text fetched and quoted):
- Ford, B. (2004). *Parsing Expression Grammars: A Recognition-Based Syntactic Foundation.* POPL. https://bford.info/pub/lang/peg.pdf (Fig. 1 self-description; §3.3 ⇒ "is a function"; §3.6 well-formedness; completeness theorems)
- Ford, B. (2002). *Packrat Parsing: Simple, Powerful, Lazy, Linear Time.* ICFP. https://pdos.csail.mit.edu/papers/packrat-parsing:icfp02.pdf
- Warth, A., Douglass, J., Millstein, T. (2008). *Packrat Parsers Can Support Left Recursion.* PEPM. https://tinlizzie.org/VPRIPapers/tr2007002_packrat.pdf
- Shieber, S., Schabes, Y., Pereira, F. (1995). *Principles and Implementation of Deductive Parsing.* J. Logic Programming. https://www.eecs.harvard.edu/~shieber/Biblio/Papers/infer.pdf
- Thompson, K. (1984). *Reflections on Trusting Trust.* CACM 27(8). https://www.cs.cmu.edu/~rdriley/487/papers/Thompson_1984_ReflectionsonTrustingTrust.pdf
- Godsil, C., McKay, B. (1982). *Constructing cospectral graphs.* Aequationes Math. 25. https://cs.anu.edu.au/~Brendan.McKay/papers/GodsilMcKayCospectral.pdf
- Unison language reference, *Hashes.* https://www.unison-lang.org/docs/language-reference/hashes/
- Chacon & Straub, *Pro Git* §10.2 Git Objects. https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
- Press et al., *Numerical Recipes* §11.4 Hermitian Matrices. https://phys.uri.edu/nigh/NumRec/bookfpdf/f11-4.pdf

Abstract or summary only (secondary for the specific claim used):
- Bancilhon, F. (1986). *Naive Evaluation of Recursively Defined Relations.* https://link.springer.com/chapter/10.1007/978-1-4612-4980-1_17
- Van Gelder, A. (1993). *The Alternating Fixpoint of Logic Programs with Negation.* JCSS. https://www.sciencedirect.com/science/article/pii/002200009390024Q
- Van Gelder, Ross, Schlipf (1991). *The Well-Founded Semantics for General Logic Programs.* JACM 38(3). https://dl.acm.org/doi/10.1145/116825.116838
- Wheeler, D. A. (2005). *Countering Trusting Trust through Diverse Double-Compiling.* ACSAC. https://dwheeler.com/trusting-trust/acsac-countering-trusting-trust-20050922-alt.pdf
- Merkle, R. (1987). *A Digital Signature Based on a Conventional Encryption Function.* CRYPTO. https://link.springer.com/chapter/10.1007/3-540-48184-2_32
- Filliâtre, J.-C., Conchon, S. (2006). *Type-Safe Modular Hash-Consing.* ML Workshop. https://dl.acm.org/doi/10.1145/1159876.1159880
- de Bruijn, N. G. (1972). *Lambda calculus notation with nameless dummies.* Indag. Math. 34. https://automath.win.tue.nl/archive/pdf/aut029.pdf
- Tait, W. (1967). *Intensional Interpretations of Functionals of Finite Type I.* JSL. https://www.jstor.org/stable/2271658 (strong normalization of simply-typed terms: standard attribution, not read here)
- Lawvere, F. W. (1969). *Diagonal arguments and cartesian closed categories.* LNM 92. http://www.tac.mta.ca/tac/reprints/articles/15/tr15abs.html (statement checked against Wikipedia's summary only)
- Kleene, S. C. (1938). *On Notation for Ordinal Numbers.* JSL. https://www.jstor.org/stable/2267778 (second recursion theorem: standard attribution, not read)
- Schwenk, A. J. (1973). *Almost all trees are cospectral.* (title and venue via scirp reference list; not read)
- Wagner, D. (2002). *A Generalized Birthday Problem.* CRYPTO. https://iacr.org/archive/crypto2002/24420288/24420288.pdf
- Dhall standard, `binary.md`. https://github.com/dhall-lang/dhall-lang/blob/master/standard/binary.md
- RFC 8259 (JSON). https://www.rfc-editor.org/info/rfc8259/
- Git hash-function transition. https://git-scm.com/docs/hash-function-transition
- McCarthy (1960), Abelson & Sussman SICP §4.1: precedent for an evaluator written in its own language; cited, not used in any proof.

Not used in a proof: Dolstra (2006) Nix thesis (store paths hash declared inputs; same pattern at package altitude), Chung (2005) directed Laplacians (defined for strongly connected digraphs per Taut's scout; a DAG with `n ≥ 2` is not), Fanuel et al. magnetic eigenmaps (arXiv:1606.08266).

COMPUTED scripts (`~/dev/ai/mara/scratch/fixpoint-parser/`): `oidhash.py` (§7.2), `peg_datalog.py` (§5.1–5.2), `git_projection.py` (§7.3), `spectra.py` (§7.4).
