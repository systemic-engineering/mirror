# combinator-optimization — beta tree normalization and the math sweep

*2026-05-22. Reed. Spec.*

Status: **Red** (the combinator algebra is in place; FP1/FP2 hold for
the Tick 4b.2/4b.3 scope; the seed is structurally simple by
construction. The optimizations listed here are NOT YET IMPLEMENTED;
this spec ranks them by expected value and shows where each one
lands.)

Depends on:
- `mirror/docs/specs/parser-as-prism-grammar.md` — the `Combinator`
  enum; FP1/FP2/FP3 fixed points; the meta-glass self-parse equation.
- `mirror/docs/specs/kintsugi-formatter.md` — the contraction-map
  argument; normalization is a contraction on the combinator tree.
- `mirror/docs/specs/eigenboard-representation.md` — the spectral
  triple; normalization corresponds to a canonical section.
- `mirror/bootstrap/src/spectral.rs` — the `Combinator` enum;
  `apply_h`; `combinator_tree_oid`; the existing Merkle hash.
- `spectral/docs/specs/kintsugi-tournament.md` (this session) —
  scoring needs normal-form ASTs to compare strategies across
  equivalent encodings.
- Reed memory: `project-mirror-compile-staircase` (the order to land
  these optimizations in).

Unblocks:
- FP1 robustness when the seed needs `LiteralKind` branches that
  prune in some encodings. Without normalization, the seed-constructed
  tree and the parsed tree may differ cosmetically; normalization
  closes the gap.
- Tournament merge scoring (`kintsugi-tournament.md` §3): comparing
  strategies by OID delta requires the OIDs to be encoding-invariant.
  Normalization is a prerequisite for the OID-churn tiebreaker.
- Performance work in later ticks: memoization, hash-cons, charset
  compilation. Each is independently valuable; this spec ranks them.
- A formal statement of mirror's parser algebra. Today the algebra is
  declared (the closed enum); the normal form gives it an equational
  theory.

---

## 0. Thesis

The `Combinator` enum is mirror's parser algebra A. The Merkle hash
over the enum is the OID. Two combinator trees represent the same
parser when they reduce to the same **beta-normal form** under the
algebra's structural equations. Normalization makes OIDs
encoding-invariant; encoding-invariant OIDs make FP1 robust against
stylistic choices in seed construction; FP1's robustness is what
lets the bootstrap absorb richer grammars without breaking the
self-parse equation.

Beta tree normalization is the seed thread. The math sweep ranks the
other optimizations by expected value:

1. **Beta normalization** (this spec's seed) — high value, low cost,
   prerequisite for tournament scoring. **Land first.**
2. **Charset compilation** (`Choice([Literal("a"), Literal("b"), …])
   → Charset({a, b, …})`) — high value, low cost, prerequisite for
   meta-glass keyword tables.
3. **Memoization in `apply_h`** — high value, medium cost, no
   correctness risk.
4. Hash-cons / structural sharing — medium value, medium cost.
5. Bottom-up Merkle with cached sub-OIDs — medium value, low cost.
6. Choice ordering by frequency — low-medium value, low cost.
7. Inline expansion of small `Shift` calls — low value, low cost.
8. Constant folding — absorbed into beta normalization.
9. Lazy/streaming parse — medium value, high cost.
10. Tail-call optimization — low value, low cost.
11. Parallelism — low-medium value, high cost.
12. Eta reductions/expansions — unclear value, deferred.

The spectral-triple connection (§10) and the sheaf-section connection
(§11) tell us why this ordering is principled rather than empirical.

---

## 1. The combinator algebra, audited

From `bootstrap/src/spectral.rs` (commit on `reed/v1-floor`, Tick
4b.3 head):

```rust
pub enum Combinator {
    Seq(Vec<Combinator>),
    Choice(Vec<Combinator>),
    LiteralKind { keyword: Vec<u8>, kind: AstKind },
    Literal(Vec<u8>),
    Repeat { body: Box<Combinator>, min: usize, max: Option<usize> },
    Capture { body: Box<Combinator>, kind: AstKind },
    Charset(CharsetKind),
    BraceBlock(Box<Combinator>),
    ParenBlock(Box<Combinator>),
    IoBinding,
    MatchArm,
    SelectVariant,
    KeywordFormBody { keyword: Vec<u8>, kind: AstKind },
    Until { stop: Box<Combinator> },
    Shift { grammar: String, body: Box<Combinator> },
    DarkFallback,
}
```

16 variants. The algebra A is the free term algebra over these
constructors. The Merkle hash `combinator_tree_oid` is a function
`A → [u8; 32]`. Two trees with different structures hash to
different OIDs even if they represent the same parser.

### 1.1 The structural equations

The algebra carries several structural equations that hold by the
semantics of `apply_h` (the Prism dispatch on Combinator):

**E1. Seq associativity.** `Seq([Seq([a, b]), c]) ≡ Seq([a, b, c])`.
Both parsers consume `a` then `b` then `c`; the bracketing doesn't
matter. Loss accumulates by `terni::Loss::combine` which is
associative.

**E2. Choice associativity.** `Choice([Choice([a, b]), c]) ≡
Choice([a, b, c])`. First non-Partial wins; the bracketing doesn't
change the order or the winning branch.

**E3. Singleton Seq.** `Seq([a]) ≡ a`. A one-element sequence IS its
element.

**E4. Singleton Choice.** `Choice([a]) ≡ a`. A one-arm choice IS its
arm.

**E5. Empty literal in Seq.** `Seq([…, Literal(""), …]) ≡
Seq([…, …])` (drop the empty literal). The empty literal consumes
zero bytes; it doesn't change which strings parse.

**E6. Zero-bound Repeat.** `Repeat { body, min: 0, max: Some(0) } ≡
Literal("")`. Zero repetitions of anything IS the empty literal.

**E7. One-bound Repeat.** `Repeat { body, min: 1, max: Some(1) } ≡
body`. Exactly one repetition IS the body.

**E8. Repeat of empty.** `Repeat { body: Literal(""), min: _, max: _ } ≡
Literal("")`. Repeating the empty literal produces the empty literal
(any number of times).

**E9. Choice of Literals to Charset.** `Choice([Literal("a"),
Literal("b"), …])` where every arm is a single-byte literal
collapses to `Charset({a, b, …})` IF the resulting set matches a
declared `CharsetKind`. (Otherwise stays as a Choice of Literals.)
See §3 for the closed-vs-open question.

**E10. Nested Capture.** `Capture { body: Capture { body, kind: k_inner }, kind: k_outer }`
is NOT generally equivalent to `Capture { body, kind: k_outer }` —
the inner capture produces an AST node the outer wraps. This is
*not* a normalization opportunity.

**E11. Shift commutativity with Repeat.** `Repeat { body: Shift { grammar: g, body: b }, n.. } ≡
Shift { grammar: g, body: Repeat { body: b, n.. } }` IS NOT generally true —
the shift's grammar boundary changes semantics. Not a normalization
opportunity (this is one of the cases where the algebra is *not*
free of context, by design).

**E12. DarkFallback dominates.** `Choice([…, a, DarkFallback, b, …])
≡ Choice([…, a, DarkFallback])` (drop everything after
DarkFallback). DarkFallback always succeeds (consumes any input);
arms after it are unreachable.

**E13. Empty Seq.** `Seq([]) ≡ Literal("")`. An empty sequence is
the empty literal.

**E14. Empty Choice.** `Choice([])` is NOT well-defined — a Choice
with no arms always fails. We treat this as an error condition
(should not appear in well-formed combinator trees).

These fourteen equations are the **redex set**. A combinator tree
in beta-normal form is one where no redex applies.

### 1.2 What's NOT a structural equation

Distinguishing structural equations from behavioural equivalences
that are NOT free of context:

- `Choice([a, b]) ≡ Choice([b, a])`? NO. Choice is left-biased (first
  non-Partial wins); reordering changes which branch wins on
  ambiguous input. This is *commutativity*, not associativity; the
  algebra is not commutative.
- `Repeat { body, 0..1 } ≡ Choice([body, Literal("")])`? Behaviorally
  yes (both match "body or nothing"); structurally no (the Repeat
  carries explicit bounds, the Choice loses them). Not a
  normalization opportunity; would lose typed-bound information.
- `Capture { body, kind } ≡ body`? NO. Capture wraps the consumed
  span as an AstNode; eliminating the wrapper loses the AST output.

The redex set above is precisely the equations that preserve both
the parser semantics AND the AST output. Equations that change
the AST output are excluded.

---

## 2. Beta-normal form

### 2.1 Definition

A `Combinator` tree is in **beta-normal form** when no redex from
E1–E14 (excluding E10/E11/E14 which aren't redexes) applies. The
remaining equations are E1–E9, E12–E13; eleven structural rewrites.

The normalization function `normalize: Combinator → Combinator`
repeatedly applies any applicable redex until none remains. Each
redex strictly decreases a well-founded measure: `(tree_size,
seq_nesting_depth, choice_nesting_depth)` lexicographically. The
recursion terminates.

### 2.2 The normalization algorithm

```rust
fn normalize(c: Combinator) -> Combinator {
    let mut current = c;
    loop {
        let next = step(¤t);
        if combinator_tree_oid(&next) == combinator_tree_oid(¤t) {
            return current;  // fixed point reached
        }
        current = next;
    }
}

fn step(c: &Combinator) -> Combinator {
    match c {
        // Recurse into children first (bottom-up normalization)
        Combinator::Seq(children) => {
            let normalized: Vec<_> = children.iter().map(step).collect();
            // E1: flatten nested Seq
            let flat = flatten_seq(normalized);
            // E5: drop empty literals
            let trimmed: Vec<_> = flat.into_iter()
                .filter(|c| !matches!(c, Combinator::Literal(b) if b.is_empty()))
                .collect();
            // E3, E13: singleton/empty
            match trimmed.len() {
                0 => Combinator::Literal(vec![]),
                1 => trimmed.into_iter().next().unwrap(),
                _ => Combinator::Seq(trimmed),
            }
        }
        Combinator::Choice(arms) => {
            let normalized: Vec<_> = arms.iter().map(step).collect();
            // E2: flatten nested Choice
            let flat = flatten_choice(normalized);
            // E12: truncate after DarkFallback
            let truncated = truncate_after_dark_fallback(flat);
            // E9: charset compilation (gated by §3's decision)
            let charset_compiled = maybe_charset(truncated);
            // E4: singleton
            match charset_compiled.len() {
                1 => charset_compiled.into_iter().next().unwrap(),
                _ => Combinator::Choice(charset_compiled),
            }
        }
        Combinator::Repeat { body, min, max } => {
            let body_n = Box::new(step(body));
            // E6: zero bounds
            if max == &Some(0) {
                return Combinator::Literal(vec![]);
            }
            // E7: one-one
            if *min == 1 && max == &Some(1) {
                return *body_n;
            }
            // E8: repeat of empty
            if matches!(body_n.as_ref(), Combinator::Literal(b) if b.is_empty()) {
                return Combinator::Literal(vec![]);
            }
            Combinator::Repeat { body: body_n, min: *min, max: *max }
        }
        // Other variants: recurse into children, no top-level redex.
        Combinator::Capture { body, kind } => {
            Combinator::Capture { body: Box::new(step(body)), kind: *kind }
        }
        // ... etc.
        _ => c.clone(),
    }
}
```

The algorithm is bottom-up: children are normalized before the
parent. This means E1–E13 each apply at most once per node, not
recursively (the bottom-up pass guarantees children are already
normal when the parent is checked). The fixed-point check at the
outer loop is defensive (catches any cross-redex interactions).

### 2.3 Termination

Each `step` call strictly decreases the measure
`(tree_size, seq_depth, choice_depth)` IF it makes any change.
This is because:

- E1, E2 reduce nesting depth by one without growing size.
- E3, E4, E5, E6, E7, E8, E13 reduce tree_size by at least one.
- E12 reduces tree_size by truncating.
- E9 (charset compilation) reduces tree_size by replacing a Choice
  of N Literals with one Charset.

All measures are bounded below by zero. The outer loop terminates
in O(tree_size) steps. Each step is O(tree_size). Total complexity:
O(tree_size²). For mirror's combinator trees (sizes ≤ ~10K nodes for
the full boot tree), this is fast.

### 2.4 Confluence (Church-Rosser)

**Claim: the normalization is confluent.** Every reduction order
produces the same normal form.

**Proof sketch:**

We use the **diamond lemma**: if for every pair of redexes `r1, r2`
applicable at distinct positions, applying r1 then r2 produces the
same term as applying r2 then r1, then the system is confluent.

We walk the pairs:

1. **E1 × E1 (Seq flattening at different positions).** Flattening
   `Seq([Seq([a, b]), Seq([c, d])])` from the left first gives
   `Seq([a, b, Seq([c, d])])` then `Seq([a, b, c, d])`. From the
   right first gives `Seq([Seq([a, b]), c, d])` then `Seq([a, b, c, d])`.
   Same result.

2. **E2 × E2 (Choice flattening).** Symmetric to E1; same argument.

3. **E1 × E3 (Seq flatten and Seq singleton).** `Seq([Seq([a])])`.
   E1 first: `Seq([a])`. E3 first: `Seq([Seq([a])]) → Seq([a])` (E3
   applied to inner). Then E3 on outer: `a`. Same result.

4. **E1 × E5 (Seq flatten and empty-literal drop).** `Seq([Seq([a, Literal("")])])`.
   E1 first: `Seq([a, Literal("")])`. Then E5: `Seq([a])`. E5 first
   (on inner Seq): `Seq([Seq([a])])`. Then E1: `Seq([a])`. Then E3:
   `a`. From the first path: E3 also reduces `Seq([a])` to `a`. Same
   result.

5. **E2 × E12 (Choice flatten and dark-fallback truncation).**
   `Choice([Choice([a, DarkFallback, b]), c])`. E2 first:
   `Choice([a, DarkFallback, b, c])`. Then E12: `Choice([a, DarkFallback])`.
   E12 first (on inner): `Choice([Choice([a, DarkFallback]), c])`. Then E2:
   `Choice([a, DarkFallback, c])`. Then E12: `Choice([a, DarkFallback])`.
   Same result.

6. **E6 × E7 (Repeat zero-bound and one-one).** Disjoint redex sets
   (different bounds patterns). Cannot both apply to the same
   Repeat. Trivially confluent.

7. **E8 × E1 (Repeat of empty under Seq).** `Seq([Repeat { body: Literal(""), 0.. }])`.
   E8 first: `Seq([Literal("")])`. Then E5+E3: `Literal("")`. E1 first
   doesn't apply (the Seq has one element). Same result.

8. **E9 × E2 (Charset compilation and Choice flatten).** `Choice([Choice([Literal("a"), Literal("b")]), Literal("c")])`.
   E2 first: `Choice([Literal("a"), Literal("b"), Literal("c")])`.
   Then E9: `Charset({a,b,c})` IF the set matches a declared
   CharsetKind. E9 first (on inner): `Choice([Charset({a,b}), Literal("c")])`
   (if `{a,b}` matches a CharsetKind). Then E2 doesn't apply (no
   nested Choice). Different result.

Case 8 is the **critical pair**: E9's applicability depends on
whether the resulting byte set matches a declared CharsetKind. The
CharsetKind enum is closed (per `spectral.rs::CharsetKind`); the
matching is a finite lookup. If `{a, b}` matches but `{a, b, c}`
also matches, E9-first and E2-first give different intermediate
forms but the same final normal form IFF the CharsetKind enum is
*upward closed under union* in the sense: any byte set produced by
E9 from a subset of an E9-matching set also matches a CharsetKind.

**This is a constraint on the CharsetKind declaration.** It is NOT
automatic. We can ensure it by either:

(a) **Restrict E9 to maximal Choices.** Only apply E9 after E2 has
    completed (flatten all Choices first; THEN check charset
    matching). This makes the algorithm deterministic in its order
    (bottom-up E2 sweep, then top-down E9 sweep) and recovers
    confluence.

(b) **Require CharsetKind to be closed under arbitrary union.** Any
    byte set that's a union of CharsetKind sets must itself be a
    CharsetKind. The current declaration has six CharsetKinds (per
    `spectral.rs`); the union closure would require adding more.

**Decision: option (a).** Restrict E9 to maximal Choices. The
algorithm becomes:

```
fn step(c) {
    let after_e1_e8_e12 = apply_non_charset_redexes(c);
    let after_e9 = apply_charset_compilation(after_e1_e8_e12);
    after_e9
}
```

The two-phase structure restores confluence. Under this structure,
the diamond lemma holds for all pairs E1–E13.

**Verdict: confluent under the two-phase normalization.** The normal
form is unique. Different reduction orders within a phase produce
the same intermediate form; the phases compose deterministically.

### 2.5 Why confluence matters for FP1

FP1 says:

```
combinator_tree_oid(seed) == combinator_tree_oid(meta_glass)
```

where `meta_glass = apply_h(seed, glass.mirror.bytes)`. The seed is
hand-written Rust; `meta_glass` is parser-constructed. If the parser
constructs a tree that differs *cosmetically* from the hand-written
seed (e.g. `Seq([Seq([a, b]), c])` vs `Seq([a, b, c])`), the OIDs
diverge even though the parsers are equivalent.

Today, FP1 holds because the seed is structurally simple by
construction — the seed-writer chooses a flat encoding, and the
parser is tuned to produce the same flat encoding. This is FRAGILE:
as the grammar grows (Tick 4b.4+ adds LiteralKind branches that
prune in some encodings), the seed and the parsed tree can drift
structurally.

Normalization makes FP1 robust:

```
combinator_tree_oid(normalize(seed)) == combinator_tree_oid(normalize(meta_glass))
```

The seed can be written in any structurally-equivalent encoding;
the parser can produce any structurally-equivalent tree; the
normalized OIDs match. **This is load-bearing for the staircase.**

### 2.6 Worked example

A realistic case where naive FP1 fails but normalized FP1 holds:

**Seed (hand-written, ergonomic):**
```
Seq([
  Literal("grammar"),
  Literal(" "),
  Capture { body: Charset(NameChar), kind: GrammarName },
  BraceBlock(…),
])
```

**Parser-constructed (after walking the grammar definition):**
```
Seq([
  Seq([Literal("grammar"), Literal(" ")]),    # adjacent literals
  Capture { body: Charset(NameChar), kind: GrammarName },
  BraceBlock(…),
])
```

The parser constructed a nested Seq (perhaps because it processed
the literal+space as one substitution). Hand-written seed flattened
the Seq. Without normalization: different OIDs; FP1 fails. With
normalization: E1 flattens both to the same form; FP1 holds.

The gap closes structurally, not by hand-tuning the seed.

### 2.7 Other confluence implications

- **Eta:** we don't have eta in the standard sense (the algebra is
  first-order; there's no `λ x. f x ≡ f`). The closest analog is
  `Capture { body, kind } ≡ …` rewrites, which are NOT confluent
  (kind matters). Eta deferred indefinitely.
- **Beta in the lambda-calculus sense:** the closest analog in this
  algebra is the substitution of a `Shift { grammar, body }` with
  the inlined definition. Inline expansion (§6.7) is a controlled
  form of this; not a redex (would explode tree size).

---

## 3. Charset compilation — the second optimization

E9 is more than a normalization rule; it's a compile-time optimization
with independent value. The mirror grammar (meta-glass) has
`Choice([Literal("focus"), Literal("project"), Literal("split"),
Literal("shift"), Literal("settle")])` as the operation-keyword
choice. A `Choice` of N literals is O(N) per byte; a `Charset` is
O(1) per byte (one lookup against the precomputed set).

### 3.1 What charset compilation does

Replaces `Choice([Literal(b1), Literal(b2), …, Literal(bN)])` with
`Charset(s)` where every literal is one byte AND the byte set
`{b1, b2, …, bN}` matches a declared `CharsetKind`.

### 3.2 The closed-vs-open question

Today `CharsetKind` is a closed enum (six variants). Charset
compilation can only match these six. For mirror's keyword tables,
this is NOT general enough — the keyword sets `{focus, project,
split, shift, settle}` are not byte sets, they're multi-byte
literals; they don't fit `Charset` at all.

**Decision: keep `Charset` closed; add `MultiByteCharset` separately.**
`MultiByteCharset(Vec<Vec<u8>>)` matches against a precomputed trie
of multi-byte sequences. For the operation-keyword choice this is
O(longest_keyword) per attempt instead of O(N × longest_keyword).

This adds a new variant to the Combinator enum:

```rust
MultiByteCharset(Vec<Vec<u8>>),  // trie-compiled
```

The normalization rule extends:

**E15. Choice of multi-byte literals.** `Choice([Literal(b1),
Literal(b2), …])` where every arm is a literal →
`MultiByteCharset([b1, b2, …])`. Always applicable; no closed-set
constraint.

### 3.3 Expected payoff

- meta-glass keyword choice: 5-arm Choice → 1 MultiByteCharset.
  Per-byte cost goes from O(5) to O(1) amortised.
- `boot/std/code/llvm/ir.mirror`: ~50 LLVM keywords; 50-arm Choice
  → 1 MultiByteCharset. Per-byte cost goes from O(50) to O(1)
  amortised.
- For the full boot tree: estimated 5–10x speedup on keyword-heavy
  grammars (rust, llvm/ir, mq).

### 3.4 Land first or with normalization?

**Both at once.** Charset compilation IS a normalization rule (E15
is the new equation). Adding E15 to the redex set and adding
`MultiByteCharset` to the algebra is one tick. Splitting them means
the normalization landed without its biggest payoff.

### 3.5 Test (FP equivalence)

For every keyword-heavy grammar in `boot/`:
```
combinator_tree_oid(normalize(seed_with_choice))
  == combinator_tree_oid(normalize(seed_with_multibyte_charset))
```

The two seeds (one written with explicit Choice, one with
MultiByteCharset) must normalize to the same OID. This is the
FP1-style equivalence at the grammar declaration level.

---

## 4. Memoization in `apply_h` — the third optimization

### 4.1 What it does

Cache the result of `apply_h(combinator, byte_slice)` keyed by
`(combinator_tree_oid(combinator), byte_slice.content_oid)`. On
repeated calls with the same combinator and the same byte slice,
return the cached result.

### 4.2 mirror's workload that makes this relevant

The kintsugi loop tokenizes the same file repeatedly (per `--shatter N`).
The tokenization is pure: same input, same output. Without
memoization, each tick redoes the work.

Also: `apply_h(meta_glass, X)` is called once per `.mirror` file in
the boot tree, then again on every kintsugi tick. The meta-glass
being the same combinator each time, all calls cache-hit on
`combinator_tree_oid(meta_glass)`. Only the file byte-OID varies.

### 4.3 Expected payoff

- Kintsugi `--shatter N` with `N > 0`: linear in N today, constant
  after memoization (only the first tick's results are computed;
  subsequent ticks hit the cache).
- Boot bootstrap (parsing 90+ `.mirror` files): unchanged on first
  run; instant on subsequent runs (modulo file changes).
- IDE/LSP workflows: every diagnostic refresh re-tokenizes the
  same files. Memoization makes this O(changes) instead of
  O(total files).

### 4.4 Correctness risk

**Zero.** `apply_h` is total and deterministic: same input → same
output, no side effects. Memoization is a pure performance
optimization with no semantic risk.

### 4.5 Implementation cost

Medium. A global `HashMap<([u8; 32], [u8; 32]), Imperfect<AstNode>>`
cache with bounded capacity (LRU eviction). The cache lives in
the spectral engine, not the parser; the parser stays pure.

The cache key is two content-OIDs; collisions are vanishingly
unlikely.

### 4.6 Preserves FP1/FP2/FP3?

**Yes, structurally.** The cache is semantics-preserving by
construction (it returns the same value `apply_h` would return).
FP1's load-bearing equation
`combinator_tree_oid(seed) == combinator_tree_oid(meta_glass)` is
unchanged.

Memoization composes with normalization: cache lookups should be
keyed by `combinator_tree_oid(normalize(c))`, not
`combinator_tree_oid(c)`, so that two cosmetically-different
encodings of the same combinator hit the same cache entry.

### 4.7 Preserves kintsugi fixed-point?

**Yes.** The kintsugi fixed-point property
(`tokenize ∘ render = id`) is preserved because `tokenize` is
unchanged (memoization is transparent).

---

## 5. Hash-cons / structural sharing — the fourth optimization

### 5.1 What it does

Deduplicate sub-trees in memory: two Combinator sub-trees with the
same content-OID share one allocation. Implemented via a global
interning table; `Box<Combinator>` becomes `Arc<Combinator>` with
lookup by OID.

### 5.2 mirror's workload

The meta-glass appears in every grammar's parse path. The boot
tree has ~90 grammars; each grammar's combinator tree imports the
meta-glass's keyword/charset declarations. Without sharing, each
grammar duplicates the imports.

After hash-cons: one allocation per unique sub-tree. The meta-glass's
keyword table lives once; every grammar's parser points at it.

### 5.3 Expected payoff

- Memory: estimated 30–50% reduction in combinator-tree memory
  for the boot tree.
- Equality checks: `Arc::ptr_eq` is O(1); replaces O(tree_size)
  structural equality. For tournament scoring (§3 in
  kintsugi-tournament.md) this matters: comparing strategies' OIDs
  becomes a pointer compare.
- Cache locality: shared sub-trees are hot; cache pressure drops.

### 5.4 Correctness risk

Low. The interning table must be consistent (no two distinct
sub-trees with the same OID), which is structurally true under
the Merkle hash. Mutation hazards: sub-trees are immutable
(`Arc<Combinator>`), so sharing is safe.

### 5.5 Implementation cost

Medium-high. Requires:

- A global interning table (`HashMap<[u8; 32], Arc<Combinator>>`).
- Replacing `Box<Combinator>` with `Arc<Combinator>` throughout.
- Constructor functions that intern on insertion.
- A GC policy for the interning table (likely never-GC during
  bootstrap, periodic-GC during long-running serve).

### 5.6 Composes with normalization?

Yes. Normalization + hash-cons gives the strongest invariant:
structurally-equivalent trees share one `Arc`. The lookup is
O(1) by OID; the comparison is `Arc::ptr_eq`.

### 5.7 Preserves FP1/FP2/FP3?

Yes. Hash-cons is transparent to the parser semantics.

---

## 6. The wider sweep — ranked

Every optimization below is described as: **what / when relevant /
expected payoff / cost / FP-preservation / lands as**.

### 6.1 Bottom-up Merkle with cached sub-OIDs

**What.** `combinator_tree_oid(c)` walks the tree and recurses on
each child. With hash-cons, each `Arc<Combinator>` carries its OID
as a precomputed field; `combinator_tree_oid` becomes an O(1) read.

**Relevant.** Every kintsugi tick computes the OID of the post-tick
AST. Today this is O(tree_size) per tick.

**Payoff.** Constant-time OID for shared sub-trees; O(unique
sub-trees) per top-level OID computation.

**Cost.** Low. Adds an OID field to the interned Arc. Already
required by hash-cons.

**FP-preservation.** Yes.

**Lands as.** Part of the hash-cons tick. Not a separate tick.

### 6.2 Choice ordering by frequency

**What.** Reorder a `Choice([a, b, c])` so that the most-frequent
winning arm is first. Reduces average-case parse cost.

**Relevant.** Hot Choices in the meta-glass (e.g. the top-level
element choice in `nl.mirror`'s `element = choice(code_inline,
fenced_block, doctest_prompt, prose)`).

**Payoff.** Small constant factor on common-case parses. For
prose-heavy grammars (like `@nl`), maybe 1.2–1.5x.

**Cost.** Low. Requires runtime profiling of which arm wins; the
static reordering is offline.

**FP-preservation.** **NO.** Choice is left-biased; reordering
changes which branch wins on ambiguous input. The reordering only
preserves semantics IF all arms are *mutually exclusive* (only one
can match at each position).

For mirror's grammars, most Choices are designed to be mutually
exclusive (the grammar author knows the structure). But this is
not a structural invariant; we'd need to verify mutual exclusion
before reordering. The verification is non-trivial (it's a parser
intersection emptiness check).

**Lands as.** Optional, off by default. Only applied to Choices
that are explicitly annotated as mutually exclusive (a new
`grammar @x { choice(..., mutually_exclusive=true) }` annotation).

**Risk.** Without the annotation, reordering can break grammars.
Defer until the annotation lands and grammars opt in.

### 6.3 Inline expansion of small Shift calls

**What.** `Shift { grammar: g, body: small_b }` where `g` is a
simple grammar (few combinators) gets inlined: the shift becomes the
grammar's combinator tree directly.

**Relevant.** `@nl` is small; lifting to it from every other grammar
adds dispatch overhead.

**Payoff.** Removes the shift's grammar-lookup cost. Estimated 5–10%
on shift-heavy grammars (every grammar with comments).

**Cost.** Medium. Tree-size grows; combinator-tree OIDs change
(losing FP1-style equivalence between inlined and non-inlined
encodings).

**FP-preservation.** Subtle. Inlining changes the structural OID;
but a normalization rule "un-inline" (if we declare it as a
structural equation) can recover the equivalence. This is similar
to beta-reduction in lambda calculus.

**Lands as.** Future tick. Requires deciding whether to inline
eagerly or lazily and whether to normalize across inlining.

**Risk.** Tree-size blowup on recursive lifts. Need a depth-limit
or a small-body threshold.

### 6.4 Constant folding

**What.** Compile-time evaluation of redexes that produce constants.
E.g. `Repeat { body: Literal(""), n.. } → Literal("")`.

**Relevant.** Anywhere constant sub-expressions appear in grammars.

**Payoff.** Subsumed by normalization (E6, E8, E13 are
constant-folding rules).

**Cost.** Zero (already done by normalization).

**FP-preservation.** Yes (it's a normalization redex).

**Lands as.** Already in beta normalization. Not a separate tick.

### 6.5 Lazy/streaming parse

**What.** Don't materialise sub-trees that aren't consumed. The
parser produces AST nodes on demand.

**Relevant.** Long files where only part is consumed (e.g. tooling
that queries by location).

**Payoff.** Memory: O(consumed) instead of O(file). Time: O(consumed)
instead of O(file).

**Cost.** High. Changes the parser's return shape from
`AstNode` to `LazyAstNode` (or `impl Iterator<Item=AstNode>`).
Changes every consumer.

**FP-preservation.** Yes if the lazy parser is observationally
equivalent to the eager one. Implementation must be careful.

**Lands as.** v1+. Major refactor. Defer until performance is
actually a problem.

### 6.6 Tail-call optimization

**What.** Recursive `apply_h` calls that are tail positions get
transformed to iteration.

**Relevant.** `Repeat` and recursive `Shift`. Rust's compiler does
some TCO; explicit TCO would catch more cases.

**Payoff.** Stack depth for deeply-nested grammars. Marginal
speedup.

**Cost.** Low (Rust compiler does most of this).

**FP-preservation.** Yes.

**Lands as.** Implicit; verify it happens. No separate tick.

### 6.7 Parallelism in `apply_h`

**What.** Independent `apply_h` calls on disjoint byte slices run
in parallel. E.g. parsing all `.mirror` files in `boot/` in parallel.

**Relevant.** Bootstrap, kintsugi-migrate, full-tree linting.

**Payoff.** Linear in core count for embarrassingly-parallel
workloads (per-file parsing). On a 10-core machine: ~8x for
the bootstrap.

**Cost.** Medium-high. Adding `rayon` or `tokio` to mirror's
bootstrap binary is a non-trivial dependency. The substrate-pull
directive (`feedback-substrate-pull`) wants logic in grammar, not
in Rust; parallelism is firmly Rust-side and adds substrate weight.

**FP-preservation.** Yes (per-file results are independent).

**Lands as.** Future tick. Probably after the bootstrap stabilises.

**Risk.** Adds a runtime dependency. Defer.

### 6.8 Eta reductions / expansions

**What.** A first-order analog of eta would be combinator-level
reductions like `Capture { body: Shift { grammar: g, body: inner }, kind: k }
≡ Shift { grammar: g, body: Capture { body: inner, kind: k } }`
(commute Capture and Shift).

**Relevant.** Rare in practice. Most grammars use Capture and Shift
in fixed compositions.

**Payoff.** Unclear. The commutation might enable other
optimizations (memoization on the inner body); but it might also
change the AST node's grammar tag.

**Cost.** Low to declare; uncertain to verify safety.

**FP-preservation.** Uncertain. Requires per-equation analysis.

**Lands as.** Deferred indefinitely. Revisit if a specific
optimization opportunity demands it.

### 6.9 Memoization within a single parse

**What.** Memo-table within one `apply_h` call: same
`(combinator, position)` returns the cached result. This is
packed-rat parsing for combinator trees.

**Relevant.** Grammars with backtracking that retries the same
combinator at the same position. Mirror's grammars have minimal
backtracking (Choice is left-biased and most arms are mutually
exclusive); the workload is small.

**Payoff.** Worst-case from exponential to linear in pathological
grammars. For mirror's actual grammars: maybe 1.1x.

**Cost.** Medium. Adds a per-parse memo table.

**FP-preservation.** Yes.

**Lands as.** Future tick if grammar complexity grows. Not urgent.

### 6.10 SIMD / vectorised charset matching

**What.** `Charset(WordChar)` could match 32 bytes at a time with
AVX2 instead of byte-by-byte.

**Relevant.** Hot paths in identifier-heavy grammars (rust, llvm/ir).

**Payoff.** 5–20x on charset-matching microbenchmarks; maybe 1.5–2x
on whole-file parsing.

**Cost.** Medium. Adds platform-specific code.

**FP-preservation.** Yes (semantically transparent).

**Lands as.** Optimization tick post-1.0. Defer.

---

## 7. Ranked summary

Ranked by expected value (payoff ∕ cost), accounting for prerequisite
structure:

| Rank | Optimization | Payoff | Cost | FP-safe | Prereq for |
|------|--------------|--------|------|---------|------------|
| 1 | **Beta normalization** | high (robust FP1) | low | YES | OID-churn scoring, tournament |
| 2 | **Charset compilation** (E9 + MultiByteCharset) | high (5–10x keyword) | low | YES | meta-glass keyword tables |
| 3 | **Memoization in `apply_h`** | high (instant re-parse) | medium | YES | kintsugi `--shatter N` |
| 4 | **Hash-cons** | medium (memory + ptr_eq) | medium-high | YES | tournament scoring at scale |
| 5 | Bottom-up Merkle (cached OIDs) | medium | low | YES | hash-cons sub-feature |
| 6 | Choice ordering by frequency | low-medium | low | NO (needs annotation) | — |
| 7 | Inline expansion of small Lifts | low | medium | subtle | — |
| 8 | Constant folding | (subsumed) | zero | YES | (in normalization) |
| 9 | Lazy/streaming parse | medium | high | yes if careful | LSP at scale |
| 10 | Tail-call optimization | low | low | YES | (implicit) |
| 11 | Parallelism in `apply_h` | medium | high (adds dep) | YES | — |
| 12 | Eta reductions | unclear | low | UNCLEAR | — |
| 13 | Per-parse memoization | low | medium | YES | — |
| 14 | SIMD charset | medium | medium | YES | — |

**Top 3 by expected value:**

1. **Beta normalization.** High value, low cost, prerequisite for
   robust FP1 across encoding choices and for tournament scoring's
   OID-churn tiebreaker. **Land first.**
2. **Charset compilation.** High value, low cost; lands with
   normalization (E9 is a redex, MultiByteCharset is a new
   variant). **Land second; same tick.**
3. **Memoization in `apply_h`.** High value, medium cost, no
   correctness risk; kintsugi `--shatter N` workloads benefit
   immediately. **Land third.**

**Top 5:** + Hash-cons + cached sub-OIDs. The combination gives
mirror a content-addressable combinator algebra with O(1) equality
and minimal memory — the foundation for the eigenboard's
content-addressed section history.

---

## 8. Cross-cutting questions

### 8.1 Which optimizations preserve FP1/FP2/FP3?

| Optimization | FP1 | FP2 | FP3 |
|--------------|-----|-----|-----|
| Beta normalization | YES (and makes FP1 robust) | YES | YES |
| Charset compilation | YES | YES | YES |
| Memoization | YES | YES | YES |
| Hash-cons | YES | YES | YES |
| Bottom-up Merkle | YES | YES | YES |
| Choice reordering | NO (changes which arm wins) | NO | NO |
| Inline Shift expansion | subtle (need un-inline normalization) | subtle | subtle |
| Lazy parse | YES if implemented carefully | YES | YES |
| Tail-call optimization | YES | YES | YES |
| Parallelism | YES | YES | YES |
| Eta | UNCLEAR | UNCLEAR | UNCLEAR |

**Key finding:** the top 5 (normalization, charset, memoization,
hash-cons, bottom-up Merkle) all preserve the three fixed points.
The later optimizations get progressively riskier.

### 8.2 Which preserve the kintsugi fixed-point property?

The kintsugi fixed point is `tokenize ∘ render = id`. Optimizations
that are *semantically transparent* to the parser preserve it:

- Normalization, charset, memoization, hash-cons, sub-OID caching:
  yes.
- Choice reordering: only if the reordering doesn't change parse
  results (requires the mutual-exclusion annotation).
- Inline Shift expansion: yes if the inline preserves the parse
  output (which it does for valid shifts).
- Lazy parse: yes if lazy and eager produce the same AST.
- Eta: depends on which eta.

The top-5 all preserve kintsugi's fixed point.

### 8.3 Order to land them in

**Phase 1 (immediate value):**

1. Beta normalization (E1–E8, E12–E13).
2. Charset compilation (E9 + E15 + MultiByteCharset variant).

  Both in one tick. The redex set is one closed enumeration; the
  normalize function dispatches on it. Tests: FP1 holds across
  multiple structurally-equivalent encodings of `mirror/glass.mirror`.

**Phase 2 (workload acceleration):**

3. Memoization in `apply_h` (keyed by `(combinator_oid,
   byte_slice_oid)` with LRU eviction).

  One tick. Tests: kintsugi `--shatter N` with N>0 takes constant
  time after first tick.

**Phase 3 (deep restructuring):**

4. Hash-cons (`Arc<Combinator>` with interning table).
5. Bottom-up Merkle (cached sub-OIDs as fields on interned Arcs).

  Likely one tick combining both. Tests: combinator-tree memory
  drops 30–50%; OID computation is O(unique sub-trees) not
  O(tree_size).

**Phase 4+ (case-by-case):**

6–N. The remaining optimizations as workload demands surface them.
Don't land speculatively.

### 8.4 Are any prerequisites for the tournament merge?

**Yes — beta normalization is.** Per `kintsugi-tournament.md` §3.1
C4 (OID-churn tiebreaker), the tournament needs to compare strategies
by "how many file OIDs changed." The comparison only makes sense if
OIDs are encoding-invariant: two strategies that produce structurally-
equivalent trees should have the same OID delta, not different
deltas due to cosmetic encoding choices.

Without normalization, the OID-churn tiebreaker is brittle. A
strategy that produces a flat Seq might score better than one that
produces an equivalent nested Seq, purely on encoding.

**Charset compilation is NOT a prerequisite** for tournament merge,
but IS a prerequisite for performance at scale (meta-glass keyword
tables in the tournament's gate-check pass).

**Memoization is NOT a prerequisite** but is highly valuable for the
tournament's gate-check pass: each strategy's gate evaluation
tokenizes the same post-merge tree under different scenarios;
memoization avoids redundant work.

**Hash-cons is NOT a prerequisite** for the v0 tournament; it
becomes one if the tournament scales to multi-thousand-file
migrations.

### 8.5 Independent vs interdependent

**Independent of tournament merge:**

- Lazy parse, parallelism, SIMD, eta. These accelerate other
  workloads (LSP, full-tree linting) but don't enable any
  tournament-specific feature.

**Interdependent with tournament merge:**

- Beta normalization (prerequisite for OID-churn tiebreaker).
- Charset compilation (not strictly prerequisite, but makes the
  gate-check pass fast).
- Memoization (not prerequisite, but high value for repeated
  gate-check).
- Hash-cons (becomes prerequisite at scale; not at v0).

---

## 9. Where each lands — file & API surface

### 9.1 Beta normalization

**File:** `mirror/bootstrap/src/spectral.rs` (next to the existing
Combinator enum and `combinator_tree_oid`).

**API:**

```rust
pub fn normalize(c: Combinator) -> Combinator;
pub fn is_normal_form(c: &Combinator) -> bool;
```

**Test fixtures:** `tests/normalize/fp1_robust.rs`. Multiple
structurally-equivalent seeds for `mirror/glass.mirror`; assert all
have the same normalized OID.

**Grammar:** A new `boot/std/mirror/normalize.mirror` declaring
`@mirror/normalize.normalize(combinator) -> combinator { \ }`. The
bootstrap implements; the grammar carries the contract.

**Properties (decidable):**

- `terminates(normalize)`: the algorithm's well-founded measure proves
  this. Verified by `@epistemologic/property/total_classification`.
- `idempotent(normalize)`: `normalize(normalize(c)) == normalize(c)`.
  Holds by construction.
- `confluent(redex_set)`: per §2.4. Verified by the two-phase
  structure.

### 9.2 Charset compilation

**File:** Same. Add the `MultiByteCharset` variant to the Combinator
enum; extend the redex set with E15; update the bottom-up walk in
`normalize`'s `step`.

**API:** Implicit (no new function; normalization applies E15).

**Test:** `tests/normalize/multibyte_charset.rs`. The five-op
keyword choice `Choice([Literal("focus"), ...])` normalizes to
`MultiByteCharset([...])`. Parser behaviour is unchanged.

### 9.3 Memoization in `apply_h`

**File:** `prism-core/src/apply_h.rs` (the substrate's `apply_h`
function). Add an optional cache parameter or use a thread-local
LRU cache.

**API:**

```rust
pub struct ApplyHCache { /* LRU<([u8;32], [u8;32]), Imperfect<...>> */ }
pub fn apply_h_cached<P: Prism>(p: &P, state: P::Input, cache: &mut ApplyHCache) -> Imperfect<...>;
```

The non-cached `apply_h` stays for the bootstrap (cache-less from
the start; deterministic). The cached variant lives in spectral's
engine layer.

**Test:** `tests/memoization/shatter.rs`. `mirror kintsugi --shatter 3`
on a fixed file: first tick computes; ticks 2 and 3 are O(1).

### 9.4 Hash-cons

**File:** `mirror/bootstrap/src/spectral.rs::Combinator` migrates
from `Box<Combinator>` to `Arc<Combinator>` with interning.

**API:**

```rust
pub fn intern(c: Combinator) -> Arc<Combinator>;
```

Constructors return `Arc<Combinator>`. Equality is `Arc::ptr_eq`.

**Test:** `tests/hash_cons/sharing.rs`. Two grammars that import the
same meta-glass keyword table share one `Arc` for the table.

### 9.5 Bottom-up Merkle

**File:** Same as hash-cons (the OID becomes a field on the
interned Arc).

**API:**

```rust
impl Combinator { pub fn oid(&self) -> &[u8; 32]; }  // O(1)
```

**Test:** `tests/hash_cons/oid_caching.rs`. `combinator_tree_oid`
returns the cached field; no recursion.

---

## 10. The spectral-triple connection

The combinator algebra IS A (per `prism-core-as-spectral-triple.md`).
Normalization corresponds to a canonical map A → A_normal where A_normal
is the quotient algebra modulo the structural equations E1–E13.

This is the algebraic shape of a *canonical model*: every element of
A has a unique representative in A_normal. The Merkle hash
`combinator_tree_oid` restricted to A_normal is injective (modulo
hash collisions, which are vanishingly unlikely under SHA-256).

### 10.1 Normalization as Prism

`normalize` is itself a Prism on the combinator algebra:

```
impl Prism for Normalize {
    type Input = Seed<Combinator>;
    type Focused = Optic<Combinator, Combinator>;
    // focus: walk and apply redexes
    // project: identity (the body is the result)
    // settle: identity
}
```

The normalized form is the result of `apply_h(Normalize, c)`. This
means normalization fits the spectral-triple framework directly:
it's an element of A acting on A.

### 10.2 Normalization as canonical section

Per `eigenboard-representation.md`, the eigenboard is a section of a
principal G-bundle. A choice of section is a gauge. Normalization
IS a *gauge choice*: a canonical representative of each equivalence
class.

The gauge group acts trivially on A_normal (it's already canonical).
This means measurements on A_normal are gauge-invariant; the OID
is a gauge-invariant observable.

**This is the deep reason normalization matters:** it converts the
Merkle hash from a gauge-dependent quantity (depends on encoding
choice) to a gauge-invariant observable (depends only on the
equivalence class). FP1 then says: "the seed and the parsed tree
are in the same equivalence class" — a structural claim about
parsing, not about encoding.

### 10.3 The Dirac operator's response

The Dirac operator D acts on A and produces the eigenboard's
spectrum (per `eigenboard-representation.md` and
`prism-core-as-spectral-triple.md`). On A_normal, D's eigenvalues
are invariants of the equivalence class. On A (un-normalized), D's
eigenvalues depend on the encoding — nominally true but the math
should produce gauge-invariant quantities.

Normalization is therefore not just a performance optimization; it's
the mathematical preprocessing that makes the spectral story
rigorous. Without it, every `e^(n+1) < e^(n)` claim has an implicit
"modulo gauge choice" caveat.

---

## 11. The sheaf-section connection

Per `project-eigenboard-is-sheaf`, the eigenboard is a cellular
sheaf on the five-operation graph. A section of the sheaf is an
assignment of a vector to each node. The combinator algebra at
each node is local data; normalization is a *local-to-local* map.

### 11.1 Local normalization composes with restriction

The sheaf has restriction maps `F(U) → F(V)` for `V ⊆ U`. Normalization
is a Prism on each `F(U)`; restriction maps are linear. Does
normalization commute with restriction?

Claim: yes. Restriction takes a combinator tree at a parent node
to its sub-tree at the child node. Sub-trees of normal-form trees
are normal-form (by construction: bottom-up normalization normalizes
children before parents). Therefore `normalize ∘ restrict =
restrict ∘ normalize`. The diagram commutes.

This means normalization is a *sheaf morphism*. The normalized
eigenboard is a morphism-image of the un-normalized eigenboard.
Reflection can choose to work with either representation; the
two are related by a canonical sheaf morphism.

### 11.2 Normalization picks a canonical section

A section of the sheaf is an assignment of a combinator tree to
each operation node. The space of sections is infinite (every
encoding is a section). Modulo equivalence E1–E13, the space of
*equivalence classes* is finite (each class has one normal form).
The canonical section is the one where every node carries the
normal form of its combinator tree.

In the spectral triple framework, the canonical section is the
basis-independent assignment. The Dirac operator acts on it; its
eigenvalues are the canonical spectrum.

**This is the deep reason tournament scoring needs normalization:**
the tournament compares sections (post-merge candidate sections).
Without normalization, two cosmetically-different sections that
represent the same equivalence class look different; with
normalization, they look identical.

---

## 12. Open questions for Alex

1. **MultiByteCharset — separate variant or extend `Charset`?** §3.2
   proposes adding `MultiByteCharset(Vec<Vec<u8>>)` as a new
   Combinator variant. Alternative: extend `CharsetKind` to include
   a `MultiByte(…)` arm. My read: separate variant keeps the
   single-byte `Charset` lean (CharsetKind is closed and tight);
   `MultiByteCharset` is naturally `Vec<Vec<u8>>` (open). Confirm?

2. **Choice ordering annotation.** §6.2 says reordering breaks FP
   without a mutual-exclusion annotation. Do we add the annotation
   to the grammar declaration syntax (`choice(a, b, c, mutually_exclusive=true)`),
   or do we declare a separate combinator variant (`ExclusiveChoice`)?
   My read: separate variant; the redex set extends with
   `Choice(arms) :where mutual_exclusion holds → ExclusiveChoice(arms)`
   as a derivation (verified by the model checker).

3. **Hash-cons cache lifetime.** §5.5 sketches a global interning
   table. For long-running processes (LSP serve, daemon), when do
   we evict? Options: never (memory grows; bounded by tree-shape
   diversity); LRU bounded; per-session reset. My read: per-session
   reset for serve; never for bootstrap.

4. **Where to declare the redex set.** §2.1 lists E1–E13 in this
   spec. Should the redex set be a grammar (`boot/std/mirror/redex.mirror`)
   that the bootstrap consumes, or stay in Rust? My read: grammar,
   per the substrate-pull directive. Each equation becomes a typed
   declaration; the normalizer is a generic engine that walks the
   redex grammar.

5. **Normalize the meta-glass first?** Should the bootstrap normalize
   the meta-glass at construction time (`prism_seed()` returns a
   normal-form tree) or at use time (every `apply_h` call normalizes
   its input)? My read: at construction (cheaper; preserves FP1's
   load-bearing OID equation).

6. **Eta deferred or decided?** §6.8 defers; is that right? My read:
   defer. The first-order eta analog doesn't have a clear
   correctness theorem; revisit only if a specific opportunity
   demands it.

7. **Memoization cache key.** §4.4 keys on
   `(combinator_tree_oid, byte_slice_oid)`. Should the byte-slice
   OID be content-OID (slow to compute) or hash-of-pointer (fast
   but fragile)? My read: content-OID. The memoization is for repeated
   parses of the same file; the file's content-OID is already computed.

8. **Confluence proof obligation.** §2.4 sketches confluence; the
   full proof walks all 91 pairs of E1–E13. Should we land a formal
   proof (e.g. in Lean or Coq) or rely on the sketch? My read: sketch
   today; formal proof if the redex set evolves.

9. **Performance benchmarks.** Every optimization in §6 has an
   estimated payoff with `~` qualifiers. Do we land microbenchmarks
   per optimization, or rely on whole-bootstrap walltime? My read:
   whole-bootstrap walltime is the user-visible metric; microbenchmarks
   only when an optimization fails to deliver expected payoff.

10. **Land beta+charset together or separately?** §3.4 says together.
    Alternative: beta first (validates the normalization framework),
    then charset (extends the redex set). My read: together — the
    framework's first deliverable should include a high-value
    optimization.

11. **Should normalization be visible at the kintsugi command line?**
    A `mirror kintsugi <file> --normalize` flag? Or always-on?
    My read: always-on (transparent; no flag).

12. **What about whitespace?** The Combinator algebra ignores
    whitespace (it's consumed by `Charset(Whitespace)` in grammars).
    Are there whitespace-related normalizations? My read: no; whitespace
    handling lives in the grammar, not the combinator algebra.

13. **Order in which to declare the grammar.** If §12.4 is right and
    the redex set lives in `boot/std/mirror/redex.mirror`, what's the
    declaration order vs `@mirror/normalize`? My read: redex first
    (the data); normalize second (the consumer).

---

## 13. Out of scope

- The actual bytecode-level implementation of `apply_h`. The
  current Rust dispatch is fine; optimizations can be measured on
  it before any rewrite.
- Compile-time optimizations that change the AST output (e.g.
  rewriting `Capture { body, kind: Foo }` to a Capture of a
  different kind). All optimizations here preserve the AST exactly.
- The lazy/streaming parse spec; deferred to v1+.
- The eta-conversion spec; deferred indefinitely.
- Performance tuning of the memoization cache (capacity, eviction
  policy). Implementation detail.
- The Reflection / Fate learning loop's interaction with
  normalization. Normalization is a static optimization; learning
  is dynamic. They compose.
- SIMD / vectorized matching; post-1.0 optimization.
- Cross-host normalization. Single-process today; multi-host is
  spectral's domain.
- The formal proof of confluence in a proof assistant. Sketch is
  sufficient for the spec; revisit if the redex set grows.
- Choice ordering by frequency without the mutual-exclusion
  annotation. Defer.
- Inline Shift expansion; sketched but not specified.
- Parallelism in `apply_h`; sketched but not specified.

---

## 14. References

### Mirror corpus

- `mirror/docs/specs/parser-as-prism-grammar.md` — FP1/FP2/FP3 and
  the Combinator algebra; normalization is the load-bearing
  robustness enhancement.
- `mirror/docs/specs/kintsugi-formatter.md` — the contraction-map
  argument; normalization is a contraction.
- `mirror/docs/specs/eigenboard-representation.md` — the bundle
  substrate; normalization picks a canonical section.
- `mirror/docs/specs/prism-core-as-spectral-triple.md` — the (A, H, D)
  framework; normalization is a Prism on A.
- `mirror/docs/specs/ast-as-bundle.md` — the AST as a bundle; the
  Combinator algebra is its dual.
- `mirror/docs/specs/mirror-compile-bootstrap.md` (Spec A) — the
  io-binding staircase; normalization rides it.
- `mirror/bootstrap/src/spectral.rs` — the Combinator enum and the
  Merkle hash this spec optimizes.
- `mirror/boot/std/mirror/normalize.mirror` (new) — the grammar this
  spec declares.
- `mirror/boot/std/mirror/redex.mirror` (new, proposed in Q4) — the
  redex set as grammar.

### Spectral corpus

- `spectral/docs/specs/kintsugi-tournament.md` (this session) — the
  consumer of normalized OIDs in C4 (OID-churn tiebreaker).
- `spectral/docs/specs/spectral-spawn.md` — agents run `apply_h`
  many times per session; memoization is high-value here.
- `spectral/docs/specs/spectral-db-mirror.md` — the spectral-db
  embedding; normalized combinator trees are content-addressable
  by their canonical OID.

### Lambda calculus and term rewriting

- Barendregt, H. (1984). *The Lambda Calculus: Its Syntax and
  Semantics.* North-Holland. The classical reference for beta
  reduction and Church-Rosser.
- Klop, J. W. (1992). "Term Rewriting Systems." In *Handbook of
  Logic in Computer Science*, Vol. 2. The diamond lemma and
  confluence.
- Terese (2003). *Term Rewriting Systems.* Cambridge University
  Press. The modern comprehensive reference.
- Knuth, D. E. & Bendix, P. B. (1970). "Simple word problems in
  universal algebras." The Knuth-Bendix completion procedure (the
  algorithmic test for confluence + termination).

### Parser combinators and PEG

- Hutton, G. & Meijer, E. (1996). "Monadic Parser Combinators."
  University of Nottingham TR-NOTTCS-TR-96-4.
- Ford, B. (2004). "Parsing Expression Grammars: A Recognition-Based
  Syntactic Foundation." POPL '04. PEGs and packrat parsing.
- Mizushima, K., Maeda, A. & Yamaguchi, Y. (2010). "Packrat parsers
  can handle practical grammars in mostly constant space." PEPM '10.
  The basis for per-parse memoization (§6.9).

### Hash-consing and structural sharing

- Goto, E. (1974). "Monocopy and Associative Algorithms in an
  Extended Lisp." University of Tokyo TR-74-03. The original
  hash-cons paper.
- Allen, J. (1978). *Anatomy of LISP.* McGraw-Hill. Hash-cons in
  the LISP family.
- Filliâtre, J.-C. & Conchon, S. (2006). "Type-Safe Modular
  Hash-Consing." ML '06. Modern hash-cons techniques.

### Reed memory

- `project-eigenboard-is-sheaf` — normalization is a sheaf morphism;
  the normal section is the canonical section.
- `project-au-conductivity` — conductivity through the normalized
  algebra; au's relational entanglement IS the normalization
  context.
- `project-mirror-compile-staircase` — normalization rides the
  staircase; the implementation is grammar-first.
- `architecture` — zero-deps prism; normalization preserves it (no
  new deps needed for the redex evaluator).
- `feedback-substrate-pull` — redex set as grammar; normalize as
  grammar; the substrate pulls here.
- `feedback-no-new-rust` — the normalization framework is
  declared in grammar; the bootstrap is a generic engine.

---

*The combinator algebra is mirror's parser data.*
*Normalization is the canonical gauge choice.*
*The Merkle hash on normal forms is a gauge-invariant observable.*
*FP1's robustness needs normalization; tournament scoring needs FP1's robustness.*
*Charset compilation rides with normalization — same tick, same redex set.*
*Memoization rides next — instant re-parse with no semantic risk.*
*Hash-cons rides after — content-addressable sub-trees with O(1) equality.*
*Bottom-up Merkle is hash-cons's natural sub-feature.*
*The remaining optimizations are case-by-case; defer until workload demands them.*
*The spectral triple gives normalization its mathematical home.*
*The sheaf framing names it a canonical section.*
*Confluence holds under the two-phase structure.*
*The redex set is closed; the normal form is unique; the OID is the witness.*

Apache-2.0.
