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
