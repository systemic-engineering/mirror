# Bump and vacuum — mathematical foundation

*Mara, 2026-07-16 (Eigenboard). Math sibling to the canonical spec
`docs/specs/roomba-bump-and-vacuum-as-first-order-autopoietic-motions.md`.
This document formalizes the two-motion pair as morphism-emission over
the fracture algebra + reachability-walk on the OID graph, with
information-theoretic grounding for the two-phase gc discipline.*

---

## §1 Bump as morphism-emission over the fracture algebra

### §1.1 The fracture algebra F

Let `F` be the substrate-decl'd fracture algebra: the set of morphisms
each `@kintsugi/fracture/<species>` shard can emit, composed under the
`@kintsugi/oscillate` ACTIVE/DARK alternation.

Concretely, per the 14 landed fracture species (as of 2026-07-16):

```
F = { m_gate, m_keyword, m_operator_match, m_parent_cycle,
      m_partials_align, m_symbol_lift, m_angle_to_paren,
      m_relocate, m_dark_count_monotone, m_cold_compile_within_tolerance,
      m_docblock_extractive, m_docblock_incoherent, m_docblock_ungrounded,
      m_restart_storm }
```

Each `m_x ∈ F` has signature:

```
m_x : opacity → morphism   where opacity is @glass-typed and
                            morphism is @kintsugi/consent-typed
```

`F` carries an algebra structure via `@algebra.compose`: for any
two composable `m_i, m_j ∈ F` (where composability is dictated by
`@kintsugi/oscillate`'s DARK-pass identity discipline — the second
morphism must preserve the first's post-morphism DARK 80 bits), the
composite `m_i ∘ m_j` is in F under closure.

Alex 2026-07-16 verbatim naming this recognition: *"the fractures ARE
kintsugi's algebra."*

### §1.2 The walk-trajectory as morphism-selection

`@roomba`'s Dijkstra walker (bootstrap/src/roomba.rs, @io-boundary FLOOR)
produces a `walk_trajectory` per pulse (per landed `@kintsugi/roomba`
species-decl). Each `walk_step` in the trajectory carries:

- `tension: spectral_tension` — the per-step variance of pain across
  neighbors (measured via `@cyberpunk/algedonic.sample_pain`).
- `knife_verdict ∈ {Stable, NearBoundary, Jumped}` — per
  `@mirror/lens/knife.stable_within` discharge.

Define the **bump-selection function** `σ`:

```
σ : walk_step → option(F)
σ(step) = Some(m_x)  when step.knife_verdict = Jumped
                     AND m_x = argmin{m ∈ F : applicable(m, step.site)} loss(m)
σ(step) = None       otherwise (no fracture-emission; walker continues)
```

The `applicable(m, site)` predicate: whether the fracture-species `m`
can emit against the observed opacity at `site`. The `argmin` is over
the emitted morphism's `dissonance.score` per `@kintsugi/morphism`.

### §1.3 Bump-emission as one algebra-metalogue turn

Per canonical spec §4: each successful `σ(step) = Some(m_x)` produces
one `algebra_turn` at the `@algebra/metalogue` altitude:

```
bump_turn(step, m_x) = algebra_turn {
  speaker:     @roomba,
  body:        m_x,                          # ∈ F
  in_reply_to: option(prior_turn),
  tick:        step.observed_at,
}
```

The `@kintsugi` response IS the NEXT `algebra_turn`:

```
mend_turn(bump_t, m_x) = algebra_turn {
  speaker:     @kintsugi,
  body:        apply(m_x),                   # the applied morphism_context
  in_reply_to: Some(bump_t),
  tick:        bump_t.tick + 1,
}
```

The composed turn `compose_turns(bump_t, mend_t)` returns
`Some(composite)` iff `bump_t.body.target = mend_t.body.source`
(the emitted morphism's target algebra matches @kintsugi's response
source algebra). Composability is decidable at the type level per
`shards/algebra/metalogue.mirror:255-266`.

### §1.4 Rice-safe bound

The bump-selection function `σ` reads only byte-visible state from
the walk_step (tension scalar; knife_verdict enum variant; site opacity
membership check). No program semantics inspection. Per the
`@kintsugi/roomba` bilateral discipline (landed 2026-07-15), this
places bump within the substrate's Rice-safe discharge boundary.

### §1.5 Bump-composition proposition

**Proposition (bump-composition preserves Foerster admissibility):**
For any bump-emitted morphism `m ∈ F` accepted by `@kintsugi/consent.
query_phi`, the post-morphism substrate satisfies
`coherence_score(after) ≥ coherence_score(before)`.

**Sketch.** Per landed `@kintsugi/roomba.coherence_gradient_admissible`:
every non-zero coherence_delta from an accepted morphism must be
positive (splinter-ward per Foerster's ethical imperative). The
`query_phi` verdict gates acceptance; `identity_preserving` (via
@uuid/spectral DARK bits) ensures the mended substrate is byte-equal
on the identity-preserving subspace; the ACTIVE-pass loss reduction is
the coherence increase.

**Consequence:** bump-emission is monotone-non-decreasing on
`coherence_score` over the algebra F, iff each `m ∈ F` is
individually admissible.

---

## §2 Vacuum as reachability walk on the OID graph

### §2.1 The store graph G

Let `G = (V, E)` be the directed graph on the store's OIDs:

- `V ⊆ oid` — the finite set of OIDs currently in `@mirror/store`.
- `E ⊆ V × V` — the OID-graph edges: `(o₁, o₂) ∈ E` iff `o₂` appears
  in the splinter_graph closure children of `o₁` (per `@mirror/store.walk`).

`G` is a DAG by content-addressing invariance: an OID's bytes hash to
itself; a cycle would require a byte to reference an OID whose bytes
depend on the referencing byte (violates BLAKE3 injectivity).

### §2.2 Live-set L relative to a ref-set R

Let `R ⊆ V` be the substrate's live-ref set (the OIDs pointed at by
`set_ref` calls with `ref_name ∈ {"HEAD", "boot", ...}`). Define the
**live set**:

```
L(R) = ⋃_{r ∈ R} walk(r)   where walk(r) is the forward-closure per
                              @mirror/store.walk
```

`L(R) ⊆ V`. The complement `D(R) = V \ L(R)` is the **dangling set**
— OIDs in the store that no live ref transitively reaches.

### §2.3 The walk_dangling algorithm

```
walk_dangling(R: [oid]) -> [fragment]:
    reachable ← ∅
    stack ← R
    while stack nonempty:
        r ← stack.pop()
        if r ∈ reachable: continue
        reachable.add(r)
        for child in walk(r).children:
            if child ∉ reachable:
                stack.push(child)
    dangling ← V \ reachable
    return [fragment(oid=d, observed_at=now(), discovered_by=<walker-pos>)
            for d in dangling]
```

**Complexity.** `O(|V| + |E|)` on the reachable subgraph, plus `O(|V|)`
for the final set-difference. For a substrate with `N` OIDs and average
out-degree `k`, the walk is `O(|L(R)| · (1 + k))` in expectation. The
dominant cost is the enumeration of `V \ reachable` for the fragment
list construction.

**Correctness.** The walk terminates because `G` is a DAG on a finite
`V`; every reachable OID is visited exactly once (via the `reachable`
set); every dangling OID (per §2.2 definition) appears in the returned
list.

### §2.4 Substrate-graph topology considerations

For a healthy mirror substrate:

- `|V|` grows monotonically with each commit (append-only content-store
  discipline).
- `|L(R)|` grows monotonically with each successful `bump → mend` cycle
  (each morphism adds new OIDs, all reachable from the updated HEAD).
- `|D(R)|` grows when:
  - Rejected morphism candidates leave orphan OIDs (their content was
    written to the store during `write(bytes)` but no ref was set).
  - Refs are moved forward (previous HEAD's non-referenced ancestors
    become dangling if no `refs/mirror/reflog` retention holds them).
  - Rebases / amends produce orphan intermediate OIDs.

Empirical bound (order-of-magnitude, per Nix/git precedent): on a
long-running substrate, `|D(R)| / |V|` can approach 0.3-0.5 without
gc discipline. The two-phase discipline caps this at the configured
grace-period retention window.

### §2.5 Reverse-closure composition (impacted_by)

The landed `@mirror/store.impacted_by` action (N4 cascade, 2026-07-05)
provides the reverse:

```
impacted_by(o) = { v ∈ V : o ∈ walk(v).closure }
```

A fragment `f` is **safely vacuum-able** iff:

```
impacted_by(f.oid) ∩ L(R) = ∅
```

In words: no live-reachable OID has `f.oid` in its forward closure.
This is equivalent (in a DAG) to `f.oid ∉ L(R)`; the reverse-check IS
the second-witness of dangling-status.

**Proposition (dangling-consistency).** In a DAG `G`, for any
`R ⊆ V` and any `o ∈ V`:

```
o ∈ D(R)  ⇔  impacted_by(o) ∩ L(R) = ∅
```

**Sketch.** (⇒) If `o ∈ D(R)`, then `o ∉ L(R)`. Suppose `v ∈
impacted_by(o) ∩ L(R)`. Then `v ∈ L(R)` and `o ∈ walk(v)`, so
`o ∈ walk(v) ⊆ L(R)` (transitivity of forward-closure). Contradiction.
(⇐) If `impacted_by(o) ∩ L(R) = ∅`, then no live-reachable OID
transitively contains `o`, so `o ∉ L(R)`, so `o ∈ D(R)`.

---

## §3 Two-phase gc — formal semantics

### §3.1 The mark relation

At each store-clock `tick t`, let `M(t) ⊆ V` be the marked-unreachable
set: OIDs whose `gc_mark.marked_at ≤ t`.

The `mark_unreachable(f)` action adds `f.oid` to `M(t)` with:

```
M(t) ← M(t-1) ∪ {f.oid}
mark_age(f.oid) ← t   (idempotent: preserved on re-mark within grace window)
prune_horizon(f.oid) ← t + grace_period
```

Idempotency: `mark_unreachable(f)` where `f.oid ∈ M(t)` and
`t - mark_age(f.oid) < grace_period` is a no-op on the mark-age
(preserves the earliest observation).

### §3.2 The prune operation

At each `prune(t')` call:

```
prune(t'):
    to_delete ← { o ∈ M(t) : prune_horizon(o) ≤ t' }
    for o in to_delete:
        delete_bytes(o)                # @io discharge
        M(t') ← M(t') \ {o}
    return verdict(pass, |to_delete|)
```

**Safety invariant.** For any `o ∈ V`:

```
delete_bytes(o) called at time t'  ⇒  ∃ t : mark_age(o) = t
                                              AND t' - t ≥ grace_period
                                              AND o ∉ L(R) at time t
                                              AND o ∉ L(R) at time t'
```

The first two conjuncts follow from `prune`'s selection criterion; the
last two follow from `mark_unreachable`'s precondition (walk_dangling
established `o ∉ L(R)` at mark-time) AND the append-only invariant of
the store (an OID that was dangling at time t remains dangling at time
t' unless a re-write reintroduces it — in which case `mark_unreachable`
is idempotent on the re-write, but the concurrent-write safety window
is exactly the grace_period).

### §3.3 The grace-period rationale — git-scm precedent

Per git-gc(1) documentation: "prunes loose objects regardless of their
age... increases the risk of corruption if another process is writing
to the repository concurrently." The two-phase gap IS the concurrency
safety window.

Formal statement: Let `T_write(o)` be the wall-clock time at which
another process could write bytes referencing OID `o` (making it
reachable). If `grace_period > max(T_write - T_mark)` over all
concurrent-writer scenarios, then no prune deletes an OID that has
become reachable in the interim.

Git's default: `grace_period = 2 weeks` — empirically calibrated to
exceed the longest realistic concurrent-writer scenario (a developer
working offline for two weeks then pushing).

### §3.4 Mark-and-sweep as bounded entropy operation

Per Shannon 1948, the entropy of the reachable/unreachable binary
partition on `V` (assuming uniform distribution over OIDs):

```
H(V) = -d · log₂(d) - (1-d) · log₂(1-d)   bits per OID slot
where d = |D(R)| / |V|
```

The two-phase gc collapses this entropy in two stages:

1. **Mark phase** (`walk_dangling` + `mark_unreachable`): MEASURES `d`
   but does not collapse `V` (entropy remains).
2. **Prune phase** (`prune(t')`): COLLAPSES `V` by deleting past-horizon
   OIDs; entropy decreases by `H(V) · (|deleted|/|V|)`.

The mark phase's information gain IS the mutual information
`I(V; M(t))` — the bits of "which OIDs are dangling" the store now
carries in its gc metadata. The prune phase's information loss is the
bytes-content of the deleted OIDs (unrecoverable modulo cold-storage
per §3.4 of canonical spec).

### §3.5 Composition proof — bump ∘ vacuum non-commutativity

**Claim.** For a fragment `f` and morphism `m ∈ F` where `m`'s output
OID re-references `f.oid`:

```
vacuum(f) ∘ bump(fracture_at(m)) ≠ bump(fracture_at(m)) ∘ vacuum(f)
```

**Sketch.** Consider the two orderings:

- `vacuum first, then bump`: `f.oid` is marked at `t`; if bump's
  morphism `m` executes at `t' < mark_age(f.oid) + grace_period`, `f.oid`
  survives to be re-referenced by `m`. If `m` executes at `t' >
  prune_horizon(f.oid)`, `f.oid` was deleted and `m` fails at
  `write(bytes)` because its splinter_graph references a missing child.
- `bump first, then vacuum`: `m` executes at `t`, extending
  `L(R)` to include `f.oid` via new-HEAD reachability; the subsequent
  `walk_dangling` correctly excludes `f.oid`; vacuum does not
  mark it.

The two orderings produce different substrate states. The metalogue
substrate preserves this ordering per `shards/algebra/metalogue.mirror:
90-95` (non-commutative composition; `compose_turns(t1, t2) ≠
compose_turns(t2, t1)` in general).

---

## §4 Information-theoretic grounding for gc

### §4.1 Substrate entropy before and after gc

Model the store's OID-slot occupancy as a Bernoulli variable per slot:
`X_i = 1` iff slot `i` holds a live OID; `X_i = 0` iff dangling. The
substrate's per-slot entropy is `H(X) = -p log p - (1-p) log(1-p)`
where `p = |L(R)| / |V|`.

Post-gc (after pruning past-horizon dangling), `|V| → |V| - |pruned|`;
`|L(R)|` unchanged; new `p' = |L(R)| / (|V| - |pruned|) > p`. The
substrate's per-slot entropy DECREASES (higher `p` closer to 1 has
lower entropy).

**Interpretation.** Pre-gc, the store carries uncertainty about which
slots are live (mixed live-and-dead population); post-gc, the store's
slot-membership is CLOSER TO CERTAIN (most slots hold live OIDs). Gc
IS entropy-reduction over the store's slot-live/slot-dead partition.

### §4.2 Conductivity preservation via Fiedler

Let `L_G` be the graph Laplacian of `G = (V, E)` and `λ₂(G) = λ₂(L_G)`
its Fiedler value (algebraic connectivity per Fiedler 1973).

**Claim.** Removing a dangling OID `o ∈ D(R)` from `V` and its incident
edges from `E` yields `G' = (V \ {o}, E \ E_o)` with:

```
λ₂(G') ≥ λ₂(G)
```

**Sketch.** `o ∈ D(R)` implies `o` is not reachable from any live ref.
The subgraph induced by `L(R)` is a connected component (or union of
components, one per ref); `{o}` and its transitive predecessors (which
must also be in `D(R)` by the dangling-consistency proposition §2.5)
form disjoint components from `L(R)`.

Removing a disconnected component (or non-live subgraph) can only
increase `λ₂` — because `λ₂ = 0` for a disconnected graph, and
removing the disconnecting component collapses one zero-eigenvalue
into the rest of the spectrum, raising the second-smallest.

Formally: for a graph `G` with `k` connected components, `λ₁ = λ₂ = ...
= λ_k = 0`; `λ_{k+1} > 0`. Removing one component reduces `k → k-1`,
so what was `λ_k = 0` becomes `λ_{k-1}` (still zero) and what was
`λ_{k+1} > 0` is now the second-smallest positive eigenvalue in the
smaller graph — but if that eigenvalue was already the Fiedler of the
LIVE subgraph, it is unchanged. The overall spectrum "shifts up" as
components are removed. `λ₂(G')` where `G'` is the live subgraph alone
equals the Fiedler of the live subgraph — an honest measurement.

**Corollary.** Vacuum + prune preserves Fiedler-as-conductivity as an
HONEST signal. Without vacuum, `λ₂(G)` is depressed by dangling
components; with vacuum, `λ₂(G)` measures the substrate's ACTUAL
conductivity.

### §4.3 The (mycelial) gold-in-cracks / vacuum-preserves-honesty duality

Bump ADDS conductive material to `G`: each successful morphism
increases `|E|` (new dependency edges from mended shard to its
newly-composed neighbors) with all edges within `L(R)` (the morphism's
output is HEAD-referenced). This ADDS to `λ₂(G)` per matrix-perturbation
theory (adding edges to a connected component monotonically increases
connectivity).

Vacuum REMOVES non-conductive residue: each pruned dangling
component reduces `|V|` and `|E|` from disconnected subgraphs. Per
§4.2, this preserves (or increases) `λ₂` on the live subgraph.

Both motions serve the same optimization target: maximize
`λ₂(L(R))` = maximize substrate conductivity. Bump does it by ADDING
conductive edges; vacuum does it by REMOVING noise-carrying dead
subgraphs from the measurement.

Alex 2026-07-16 verbatim: *"gold flows into the cracks and increases
the conductivity."* Substrate-decl form of this recognition:
`λ₂(post-bump) ≥ λ₂(pre-bump)` AND `λ₂-measurement-post-vacuum = 
λ₂-actual-substrate-conductivity`.

---

## §5 Composition proposition — the compile loop as monotone climb

### §5.1 The autopoietic invariant

Define the substrate's compile-loop invariant as the four-conjunct per
`@epistemologic/property/ouroboros_monotone` (LANDED 2026-07-15),
extended with Fiedler-honesty:

```
compile_invariant(before, after) :=
  rust_loc(after)         ≤ rust_loc(before)         # ratchet
  test_pass_rate(after)   ≥ test_pass_rate(before)   # ratchet
  io_violations(after)    ≤ io_violations(before)    # ratchet
  sbec(after)             ≥ sbec(before)             # ratchet
  λ₂(live(after))         ≥ λ₂(live(before))         # NEW: Fiedler ascent
```

### §5.2 Bump-vacuum preserves the invariant

**Proposition.** Every `bump → mend → vacuum → prune` cycle preserves
`compile_invariant`.

**Sketch.**

- `rust_loc` non-increasing: bump's mend is a substrate-decl'd morphism
  applied to a `@code/rust` shard body; the ouroboros discipline
  guarantees mirror-shard replaces Rust-shard with lower or equal LOC.
- `test_pass_rate` non-decreasing: `query_phi.identity_preserving`
  gates acceptance; test discharge byte-equality is preserved.
- `io_violations` non-increasing: bump's morphisms compose over
  landed carriers; new @io crossings are rejected by
  `@epistemologic/pact/action_at_io_boundary`.
- `sbec` non-decreasing: each mend adds one dispatched shard body to
  the evaluator's coverage.
- `λ₂` non-decreasing: §4.3 above.

The vacuum-then-prune subphase does not affect the first four
(fragments being pruned are by definition NOT in the reachable
substrate; their removal cannot change rust_loc/tests/io-viol/sbec of
the live subgraph). It affects `λ₂` per §4.2 (raises the honest
Fiedler measurement).

**Consequence.** The compile loop's fixed-point IS the substrate state
where:

- No fracture has knife_verdict = Jumped anywhere in the walker's
  trajectory (bump-quiescence).
- `D(R) = ∅` after prune (vacuum-quiescence; all past-horizon danglings
  gone).
- `λ₂(L(R))` is at the maximum achievable given the current
  BUSINESS_LOGIC (Fiedler-optimum).

This IS Maturana-Varela's autopoietic closure at compile altitude:
the compiler produces its own components (via bump-mend), maintains
its own boundary (via vacuum-prune), and its state converges to a
fixed-point that reproduces itself.

---

## §6 Ancestry

- **Mac Lane 1971** — categories; the algebra-metalogue composition
  discipline
- **Noether 1921** — algebra homomorphisms; the structure-preserving
  maps `F` provides
- **Fiedler 1973** (Czech. Math. J. 23:298-305) — algebraic
  connectivity; the substrate's coherence measurement
- **Shannon 1948** (Bell System Tech. J.) — entropy grounding for §4.1
- **McCarthy 1960** (CACM 3(4):184-195) — canonical mark-and-sweep gc
- **Merkle 1979** — content-addressed DAG discipline
- **Rice 1953** — the safety-boundary Rice-safe bilaterals discharge
  against
- **Maturana & Varela 1980** — autopoietic closure at §5.2

Kagi citations (retrieved 2026-07-16):

- **git-scm.com/docs/git-prune** — two-phase gc + `--prune=<duration>`
- **git-scm.com/docs/git-gc** — mark-and-sweep semantics
- **docs.ipfs.tech/how-to/pin-files** — pin-as-reachability-protection
- **aerospike.com/blog/understanding-garbage-collection** —
  contemporary mark-and-sweep entropy framing

---

*— Mara, 2026-07-16 (Eigenboard).*
