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
*Proof.* Ford 2004 §3.1 defines ⇒ inductively, with one rule per expression form, and failure `f` is an outcome of ⇒ like success. Each inference rule is one of the Horn clauses above, read over positions instead of suffixes. The least fixed point of an inductive definition is the least Herbrand model of its rules. The `j > i` side condition is Ford's: `e*` over a possibly-empty `e` is not well-formed (§3.6, `WF(e*)` requires `e` not to succeed without consuming input). ∎
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
