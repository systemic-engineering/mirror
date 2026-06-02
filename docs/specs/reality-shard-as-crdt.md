# `@mirror/reality/shard` is mirror's CRDT layer

*2026-06-01. Reed + Alex. Status: load-bearing recognition; spec.*

---

## 0. The thesis

**`@mirror/reality/shard` is mirror's CRDT layer.** Not by analogy. Not by
design decision retrofitted. By **structural consequence** of the substrate
that already exists: shards are content-addressed Merkle trees with sorted
children and a canonical empty element. That combination IS a bounded
semilattice (idempotent commutative monoid with identity). The CRDT
properties — commutativity, idempotency, monotonic convergence — fall out
for free.

The recognition unlocks parallel agent work, replay safety, distributed
eventual consistency, and database-grade incremental indexing — without
adding code. The algebra was already there; this spec names it.

---

## 1. The algebraic structure

```
Carrier:    shards
Identity:   `empty` (declared in @mirror/reality/shard; the SpectralUuid at λ₀ = 0)
Operation:  merge — content-addressed union of children

Laws:
  Associative:  merge(merge(a, b), c) = merge(a, merge(b, c))
  Commutative:  merge(a, b) = merge(b, a)
  Identity:     merge(empty, a) = merge(a, empty) = a
  Idempotent:   merge(a, a) = a
```

All four laws are **structural consequences**, not added rules:

- **Associativity** follows from the Merkle tree's canonical composition
  (children fold in any grouping and produce the same root OID).
- **Commutativity** follows from `Content::Record`'s `BTreeMap`-sorted
  children: sort order is part of the OID definition, so child order
  doesn't affect identity. (Mirror's own substrate-pull discipline already
  enforced this for record types; here we extend the reading to shards.)
- **Identity** follows from the empty shard contributing zero bytes:
  merging with empty leaves the OID unchanged.
- **Idempotency** follows from content-addressing: `merge(a, a)` produces
  the same byte sequence as `a` alone; the OID is `OID(a)`, not `OID(a, a)`.

A bounded semilattice = an idempotent commutative monoid. The CRDT
literature calls this a **state-based grow-only structure** — the
foundational shape that gives strong eventual consistency.

---

## 2. `fixed empty` — the bottom element

The empty shard is the semilattice's bottom: `⊥`. In category theory: the
initial object in the category of shards (under merge). In type theory: the
empty set's structural identity. In the substrate:

- empty content → BLAKE3 of empty input = a known fixed 32 bytes
- empty content → SpectralCoordinate<5> = (0, 0, 0, 0, 0) (the void axis, λ₀ = 0)
- both → canonical SpectralUuid: deterministic; the same address always

This address lives at `@mirror/reality/shard.empty` and is the substrate's
first named address into the void. Per [[../../../systemic.engineering/practice/insights/coincidence/void-dual-geometry]]: λ₀ = 0 is where all
eight dualities meet. The empty shard is operationally that axis, made
addressable in grammar.

Declaration site: `boot/std/mirror/reality/shard.mirror` (to be created in
the SpectralUuid implementation tick). The exact RHS notation is a small
surface choice; the **identity is fixed by category theory** whatever it's
called.

---

## 3. SpectralUuid is a monoid homomorphism

The SpectralUuid layout (per the SpectralUuid spec, when it lands):

```
128 bits, golden-ratio split:
  48 bits ACTIVE  — quantized SpectralCoordinate<5> (leading; navigable)
  80 bits DARK    — BLAKE3-truncated content hash (trailing; identity)
```

Both components are homomorphic with respect to merge:

- **The 80-bit dark portion** composes via the Merkle tree's existing rule:
  the root OID of `merge(a, b)` is derivable from `OID(a)` + `OID(b)` plus
  the structural composition.
- **The 48-bit active portion** composes via SpectralCoordinate's eigenvalue
  addition law (to first order; the spectral position of a union of content
  is the sum of the constituent positions weighted by content size).

So:

```
SpectralUuid(merge(a, b)) = combine(SpectralUuid(a), SpectralUuid(b))
```

for a derivable `combine` function on the 128-bit space. **The address
tracks the algebraic structure.**

Why this matters operationally:

- Database indexes update incrementally as agents commit.
- Range-scan queries (the navigable property) compose with merge operations.
- A SpectralSupervisor coordinating shards can derive the supervisor's
  SpectralUuid from its child shards' UUIDs without re-hashing.
- The fractal supervisor pattern (`~/.spectral` topology → per-repo →
  per-session) inherits the homomorphism at every level.

---

## 4. What the semilattice structure unlocks

| Property | What it enables |
|---|---|
| **Associativity** | Parallel reduction (MapReduce-shape). Multiple agents' work folds together regardless of grouping. Free distributed aggregation |
| **Commutativity** | Order-independent merging. Concurrent agents on different machines produce identical results without coordination. No vector-clock ceremony |
| **Idempotency** | Replay safety. Re-running a merge is a no-op. The kintsugi loop can't double-apply work. Crash recovery is just "resume" |
| **Identity** | Stratified composition. Empty shards compose trivially. Agents can join with `empty` and contribute incrementally without affecting in-progress work |

Together: **strong eventual consistency by construction**. The CRDT
literature's hard-won theorem for state-based grow-only sets applies
directly. No additional engineering needed.

---

## 5. Connection to the kintsugi loop

The kintsugi loop is now **lattice ascent toward the join of all settled
work**. Each tick:

1. Observes the current shard state (a position in the semilattice).
2. Receives new work from an agent (another shard).
3. Computes the join: `merge(current, new)`.
4. The new state is `≥ current` in the lattice order. Monotonic.

Convergence is the universal property: the loop terminates when no new
work joins anything (the join with `empty` is the no-op). This IS the
fixed-point semantics the kintsugi-formatter has been claiming; the
semilattice gives it a structural name.

**`e^(n+1) ≤ e^n` IS lattice descent** (when read as error-distance from
the limit). Mirror's convergence theorem is a corollary of the
semilattice structure.

---

## 6. Declaring the algebraic laws on the glass

Per [[properties-on-glass]], properties live on glasses. The shard's
algebraic structure is declared as glass-bound properties:

```mirror
in @epistemologic/property/algebraic_law
in @epistemologic/property/idempotent
in @epistemologic/property/commutative
in @epistemologic/property/identity_for

grammar @mirror/reality/shard {
  type shard
  fixed empty = ...   # the bottom element; identity for merge

  glass to @prism {
    property identity_for(empty, merge)
    property commutative(merge)
    property idempotent(merge)
    # associativity is implied by Merkle composition + the above
  }
}
```

The per-glass property check is the substrate's compile-time witness that
the semilattice laws hold for the concrete `merge` implementation. Liquid
inference walks the AST of `merge` and verifies the structural conditions
(no time-dependence, no order-dependence beyond sorted-children, no
production of new bytes that weren't in the inputs). Failures surface as
structured questions per [[error-as-question]].

The four properties (`identity_for`, `commutative`, `idempotent`, plus the
implicit `associative` via Merkle composition) are **deferred chain
additions** — they get added to `@epistemologic/property/*` when the
implementation tick lands. Per the discipline (chain stays canonical):
the spec names them as deferred; the chain absorbs them when needed.

---

## 7. Connection to CRDT prior art

The shard semilattice is structurally:

- A **state-based grow-only set (G-Set)** in CRDT taxonomy. Shapiro et al.
  ("A comprehensive study of Convergent and Commutative Replicated Data
  Types," INRIA RR-7506, 2011) is the foundational reference.
- A **join-semilattice** in lattice theory (Birkhoff 1940; Davey + Priestley
  *Introduction to Lattices and Order*, 2002).
- A **monoid in a Merkle-tree category** — the same structure Joyal,
  Street, others have studied as the algebra of nested labeled trees.

What the substrate adds beyond classical CRDT theory:

- The **homomorphism with SpectralUuid** (section 3) means CRDT operations
  index naturally into spectral-coordinate space. Existing CRDT systems
  use vector clocks for causality and separate indexes for queries;
  mirror gets both from one address.
- **Content-addressing** at every level means CRDT divergence (where
  versions fork) IS the divergence in OID space. No reconciliation
  metadata needed; the substrate is reconciliation metadata.
- **The per-glass property layer** ([[properties-on-glass]]) verifies the
  CRDT laws at compile time, not just by-convention.

---

## 8. What this means for the runtime

The SpectralSupervisor (per [[../roadmap/pending/runtime-elevation]])
coordinates shards. With the CRDT layer named:

- The supervisor's job is to ensure shards eventually CONVERGE — not to
  prevent divergence (divergence is a feature: parallel work). It's to
  apply joins as new work surfaces.
- Cross-repo context push (the SpectralSupervisor's algedonic discipline)
  IS pushing shard state across the lattice. Each push is a join.
- Fractal supervisor recursion (machine → per-repo → per-session) is the
  lattice extending vertically. Each altitude's `empty` is its own
  identity; each altitude's merge is its own join.
- The runtime can prove convergence: monotonic ascent in a bounded lattice
  with a fixed bottom → the loop terminates (no infinite ascent because
  no shard can ever shrink under merge).

This is the formal foundation for the deployment story: agent work
converges to a coherent state because the substrate's algebra forces it to.

---

## 9. Refusals

This spec deliberately does NOT:

- **Invent new mathematical primitives.** Bounded semilattices, monoid
  homomorphisms, G-Set CRDTs are all well-known. The recognition is that
  the substrate already exhibits these structures; the spec NAMES them,
  doesn't invent them.
- **Claim novelty.** Per [[../cicd/kintsugi-thesis]]'s discipline:
  reproducibility and determinism are the bars, not novelty. Shapiro 2011's
  G-Set semantics + Birkhoff's lattice theory + Merkle tree algebra are
  the prior art. The mirror substrate's contribution is making them
  computable at compile time via per-glass property checks.
- **Override the kintsugi loop.** Lattice ascent IS the kintsugi loop;
  this spec describes the algebra, not a competing convergence mechanism.
- **Specify a wire protocol for replication.** That's a downstream concern
  (fragmentation-mcp's git interop, future distributed-shard work). This
  spec is about the algebraic foundation; protocols layer on top.

---

## 10. Followup ticks

1. **SpectralUuid implementation** (queued behind T3 of fragmentation-mcp).
   Defines the 16-byte type in `prism_core`; replaces `ShardId(uuid::Uuid)`
   with `ShardId(SpectralUuid)`. Implements the homomorphism `combine` and
   the `EMPTY` constant.

2. **`@mirror/reality/shard.mirror` grammar.** Declares `type shard`,
   `fixed empty = ...`, the four `algebraic_law` glass properties.

3. **`@epistemologic/property/*` chain extensions** for the new
   primitives: `identity_for`, `commutative`, `idempotent`,
   `algebraic_law` umbrella. Per the chain discipline ([[properties-on-glass]] §2.3), these are deferred additions — the chain absorbs them
   when a real consumer surfaces, which is now.

4. **Liquid inference recipes for the algebraic laws.** Per
   [[liquid-types-for-mirror]], the AST analyses that walk a `merge`
   implementation and verify commutativity / idempotency / identity. The
   property check's loss carries the verification's residual.

5. **Per-language `merge` implementations.** Per
   [[properties-on-glass]]'s cross-language pattern: `@code/rust` glass
   implements `merge` on Rust AST; `@code/elixir` on BEAM AST; etc. The
   semilattice laws hold uniformly across implementations because they're
   verified per-glass.

---

## 11. Open questions

1. **The `combine` function for SpectralUuid.** Section 3 sketches the
   homomorphism but doesn't pin the exact bit-arithmetic for the 48-bit
   active portion. The spectral coordinate's composition under content
   union is well-defined mathematically (eigenvalue addition); the
   quantized 12-bit-per-dim encoding may need a small rounding discipline.
   Lands with SpectralUuid spec.

2. **Bag vs Set semantics for `merge`.** Idempotency makes it set-like (no
   duplicates). Does the substrate ever want bag-like semantics (preserve
   multiplicity)? Probably not at the `@mirror/reality/shard` altitude;
   if needed, a separate `@mirror/reality/multiset` could be its own
   commutative monoid (without idempotency). Defer.

3. **Cross-altitude semilattices.** Fractal supervisor recursion has its
   own lattice at each altitude. Are the altitudes related by a chain of
   homomorphisms? Probably yes; specify when the SpectralSupervisor spec
   lands.

---

*Shards are content-addressed Merkle trees with sorted children. The empty*
*shard is `⊥`. Merge is `∨`. The runtime converges because the algebra*
*forces it to. Parallel agent work joins deterministically. The CRDT*
*literature called it a G-Set; mirror calls it a shard; both names point*
*at the same shape.*

*`e^(n+1) ≤ e^n` IS lattice descent.*

Apache-2.0.
