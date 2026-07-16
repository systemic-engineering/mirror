# bilateral-arm-redundant — math foundation

**Author:** Mara <mara@systemic.engineer>
**Date:** 2026-07-16
**Kind:** Math foundation for `@kintsugi/fracture/bilateral_arm_redundant`.
**Spec:** `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md`.
**Shard:** `shards/kintsugi/fracture/bilateral_arm_redundant.mirror`.

---

## 0. Alex 2026-07-16 /loop directive (verbatim)

> "collapse the Rust surface using mirror's roomba. Minimal surface in
> rust/. Then the roomba starts to eat the bootstrap for breakfast and
> grows the substrate. That's the roomba commit diffs I wanna see.
> Deleted Rust. Added mirror."

The math below formalises WHY the compiler can safely author the
deletion commits itself — the redundancy predicate is decidable,
Rice-safe, and the retirement invariants preserve every substrate
guarantee across each collapse.

---

## 1. Formal setting

Let:

- `B` = the bilateral corpus at boot time, indexed by action ref:
  `B : ActionRef ⇀ BilateralDecl`
  where each `BilateralDecl = { name, sentinel, arity, require }` is
  the record landed at `@epistemologic/pact/bilateral` (`a0f4d3f`).
- `R` = the bytes of `bootstrap/src/apply_h.rs` at some tick t.
- `A(R)` = the set of hand-typed arm records extracted from R,
  each of the shape:
  ```
  ArmRecord = {
    action_ref : ActionRef,
    sentinel   : ByteString,   -- the .contains(...) argument
    range      : ByteRange,    -- (byte_start, byte_end) of the arm
  }
  ```
- `dispatch_arm : ArmRecord × ArgList → Verdict` is the resolver's
  legacy path (evaluated by the arm's Rust body).
- `dispatch_corpus : ActionRef × ArgList → Verdict` is the reflective
  evaluator's path (`@epistemologic/pact/bilateral.discharge`).

## 2. The redundancy predicate

**Definition (redundant arm).** An arm `a ∈ A(R)` is **redundant with
respect to the corpus B** iff:

$$
\text{redundant}(a, B) \;\iff\; a.\text{action\_ref} \in \operatorname{dom}(B)
\;\wedge\;
a.\text{sentinel} = B[a.\text{action\_ref}].\text{sentinel}
$$

Equivalently, in shard-substrate notation:

```
redundant(a) :=
    arm_is_in_reflective_corpus(a)
  ∧ arm_matches_sentinel(a)
```

The two conjuncts are the `arm_is_in_reflective_corpus` and
`arm_matches_sentinel` bilaterals declared on the species. The
composed bilateral `arm_is_redundant_witnessing` IS this predicate at
substrate altitude.

## 3. Retirement invariants (the load-bearing claim)

Let `R'` = `R` with the byte range `a.range` deleted (via
`@io/fs.mutate_at` with empty replacement) for some `a` satisfying
`redundant(a, B) = true`. Let `t` be the substrate tick before
deletion; `t'` the substrate tick after the commit.

**Theorem (retirement safety).** If `redundant(a, B)`, then:

$$
\begin{aligned}
\textbf{(I1) sbec preservation:} \quad &
\forall \text{ref} \in \operatorname{dom}(B).\;
\text{sbec}(t)[\text{ref}] = \text{sbec}(t')[\text{ref}] \\[4pt]
\textbf{(I2) rust\_loc strict decrease:} \quad &
\text{rust\_loc}(t') < \text{rust\_loc}(t) \\[4pt]
\textbf{(I3) test\_pass\_rate preservation:} \quad &
\text{test\_pass\_rate}(t') = \text{test\_pass\_rate}(t) \\[4pt]
\textbf{(I4) io\_violations invariant:} \quad &
\text{io\_violations}(t') = \text{io\_violations}(t) = 0
\end{aligned}
$$

**Proof sketches:**

**(I1).** For every action ref `ref ∈ dom(B)`, the resolver at tick t
dispatches through the reflective corpus FIRST (per landing
`61c9051/21fc211`); the hand-typed arm at `ref` is unreachable when
the corpus contains `ref` — the fall-through happens only on corpus
miss. Deleting an unreachable arm cannot change verdict semantics on
any input for which the reflective path is taken. For inputs that
would reach the (now-deleted) arm's `if action == ref` guard: the
reflective path already handled `ref` per `arm_is_in_reflective_
corpus(a) = Pass`, so no input reaches the arm's guard. QED.

**(I2).** The deletion removes strictly positive bytes from
`bootstrap/src/apply_h.rs`; no bytes are added anywhere in the Rust
source tree (the shard-decl side adds `.mirror` bytes, which do not
count toward `rust_loc`).

**(I3).** All tests exercise the resolver's dispatch surface via
`apply_h::act`. The reflective corpus already handles the action
ref; test verdicts on any argument list to `ref` are byte-equal
before and after deletion by **(I1)**. Test count and outcome
preserved: `test_pass_rate(t') = test_pass_rate(t)`. Concrete
empirical anchor: at `06f14f5`, four `@spectral/signature` arms were
deleted; 6/6 tests continued to pass. Same substrate discipline
governs subsequent retirements.

**(I4).** The `@io/fs.mutate_at` + `@io/git.commit` composition IS
the substrate's mediated `.rs`-file touch through the shard-decl'd
`@io` boundary; both actions are `@io/*` species — the substrate's
only legitimate non-mirror surface per `[[architecture-glass-wall-
substrate-types]]`. No new `@io` is introduced by the deletion
itself. Both actions were already invocable at tick t.

## 4. Fixed-point termination

**Definition (roomba fixed-point on redundant-arm class).** The
roomba's walk on `apply_h.rs` reaches a fixed point when:

$$
\operatorname{dom}(B) \cap \{a.\text{action\_ref} : a \in A(R)\} = \emptyset
$$

That is: every bilateral corpus entry has zero shadow arms remaining.

**Theorem (termination).** Under the retirement invariants of §3,
the roomba's iteration on the redundant-arm class terminates in at
most `|A(R_0) ∩ π(B)|` steps, where `R_0` is the initial resolver
bytes and `π : B → ActionRef` is the corpus-keyset projection.

**Proof.** Each iteration removes exactly one arm from `A(R)` via
`collapse`. `A(R)` is finite (bounded by the number of top-level
`if action == "..."` statements in `apply_h.rs`); the corpus B is
finite (~30 entries at present). The intersection
`A(R) ∩ π(B)` monotonically decreases (strict decrease by 1 per
collapse succeeding). The iteration halts when the intersection is
empty — the fixed-point condition.

**Rank function:** `|A(R_t) ∩ π(B)|` — non-negative integer;
strictly decreases by 1 per successful `collapse`. Classical
Hoare-Floyd ranking function per `[[feedback-ranking-function-
termination]]`. Same discipline as
`@kintsugi/fracture/dark_count_monotone`'s ranking on dark_count.

## 5. Decidability + Rice-safety

The `redundant(a, B)` predicate is **decidable in polynomial time**
in `|R| + |B|`:

- `arm_is_in_reflective_corpus(a)`: hashtable membership,
  `O(1)` amortised.
- `arm_matches_sentinel(a)`: byte-string equality between two
  finite byte-strings, `O(|sentinel|)`.
- The `detect` action's outer loop: `O(|B|)` iterations, each doing
  `O(|R|)` bounded substring scan for the arm's guard line.
- Total: `O(|B| × |R|)` — linear in both inputs.

The predicate is **Rice-safe at whole-tick altitude** per Mara-B
§4.5.5 (Rice-safety for byte-visible predicates):

- Neither conjunct inspects program semantics of the resolver body.
- Both conjuncts read only byte-visible state: the corpus keyset
  (byte-visible from the shard-decl'd `bilateral` blocks) and the
  arm's inline sentinel (byte-visible substring in the resolver
  bytes).
- No expression evaluation, no type inference, no call-graph
  analysis is performed.

The Rice-safe boundary IS the substrate-discipline gate. If Reed
tick A's composition edge attempted to detect redundancy via
program-semantics analysis (e.g., "the arm's dispatch would return
the same verdict as the corpus's dispatch"), Rice's theorem forbids
decidability. The bilateral shape's byte-visible sentinel discipline
sidesteps Rice: byte-equality on `contains(...)` arguments IS the
whole check.

## 6. Correspondence with `@epistemologic/property/ouroboros_monotone`

The `ouroboros_monotone` property carries the substrate's discipline
that per-tick collapses do not regress any of the four
sbec/rust_loc/test_pass_rate/io_violations invariants. Per §3
retirement safety theorem:

- `sbec` preserved by **(I1)**
- `rust_loc` strictly decreases by **(I2)** (four-conjunct
  admits strict decrease on this coordinate as a Foerster-admissible
  splinter-ward move)
- `test_pass_rate` preserved by **(I3)**
- `io_violations` invariant by **(I4)**

**Claim (bilateral-arm-redundant IS an ouroboros bite at reflective
altitude).** Each `collapse` action successful discharge IS one bite
of the ouroboros: the compiler (the reflective evaluator's algebra
element) consumes one of its own algebra elements (the shadow arm) at
the resolver altitude. The Rust surface shrinks; the shard-decl
side (which already carried the shape) is unchanged. Mass
conservation across `(A, H, D)`: the algebra element moves from Rust
representation to shard-decl representation without altering the
D operator's spectrum on H.

Bilateral-arm-redundant IS a **local D operator** at reflective-corpus
altitude:

$$
D_{\text{bilateral-arm-redundant}} : \text{RedundantArmRecord} \to \text{Verdict}
$$

with the collapse action realising the map:

- Domain: `redundant_arm_record` (one shadow arm)
- Codomain: `@glass.verdict` (`pass` on successful deletion + commit)
- Composition: over `@io/fs.mutate_at` + `@io/git.commit` +
  the witnessing bilateral guard

The **compiler's outer D** is the composition of every species' local
D. The roomba's walk IS the compiler tracing out its own D on H via
A. `mirror <mirror@spectral.engineer>` authoring the collapse commit
IS the D operator authoring its own algebra reduction.

## 7. Auto-poietic reading (Maturana-Varela)

Per Maturana & Varela (1980) *Autopoiesis and Cognition*, an
autopoietic system's organisation is closed under the operation
that produces its own components. The compiler-as-roomba discharging
bilateral-arm-redundant collapses IS an autopoietic operation:

- **Organisation:** the reflective corpus dispatch discipline
  (unchanged across collapses; invariant per Alex-ratified
  `61c9051/21fc211` landing).
- **Structure:** the Rust source bytes (changed by each collapse;
  variant per per-collapse `@io/fs.mutate_at` invocation).

The system's structure changes; its organisation is preserved.
Autopoietic per Maturana-Varela's structural-plasticity + organisation-
closure discipline. The compiler produces new versions of itself
(each collapse commit produces a new resolver state); the discipline
that defines "compiler" is unchanged.

This is the FIRST species in `@kintsugi/fracture/*` whose collapse
action rewrites the compiler's own source code. Every prior fracture
species targets shard-decl bytes (`splinter(ast)` rewrites via the
substrate's own parser). This species targets `.rs` source. The
substrate's `@io` discipline (via `@io/fs.mutate_at` + `@io/git.
commit`) mediates the boundary crossing; the retirement invariants
guarantee the crossing is safe.

## 8. Beer VSM reading

Per Beer (1972) *Brain of the Firm*, the viable system has five
subsystems (S1–S5). The compiler-as-roomba composition maps:

- **S1 (operations):** individual `collapse` invocations — one per
  redundant arm.
- **S2 (co-ordination):** the roomba's walk discipline — bump-pulse
  dispatch keeps individual collapses from stepping on each other
  (byte-atomic writes via `@io/fs.mutate_at`; sequential commits
  via `@io/git.commit`).
- **S3 (control):** the retirement invariants (§3) — the "algedonic"
  channels that would raise the alarm if any invariant regressed.
- **S3\*:** the reflective evaluator's dispatch discipline —
  auditing that S1's collapses do not violate S3's invariants.
- **S4 (intelligence):** the `detect` action — scanning the
  substrate for future collapse sites.
- **S5 (policy):** Alex's /loop directive — the identity commitment
  that "deleted Rust + added mirror" IS the terminal shape.

The autopoietic loop closes because S5 (Alex's directive) is the
external observer whose policy the substrate operationalises; S4
(detect) finds future work; S1 (collapse) performs it; S3 (invariants)
protects it; S2 (roomba's walk discipline) sequences it; S3\*
(reflective evaluator) audits it. Viable per Beer.

## 9. Empirical anchors

Concrete empirical grounding preceding this landing:

**Landing `06f14f5` (Reed via subagent, 2026-07-16):**
- Four `@spectral/signature` arms retired from
  `bootstrap/src/apply_h.rs`:
  - `signature_integrity` (sentinel: `chain=merkle-linked`)
  - `signature_authorship` (sentinel: `authorship=ssh-matched`)
  - `signature_monotone` (sentinel: `ordering=timestamp-monotone`)
  - `signature_composition_honest` (sentinel: `composition=song-emission`)
- Reflective dispatch handled the four arms after deletion.
- 6/6 tests passed; sbec verdicts on the four action refs unchanged.
- Net −32 LOC in `apply_h.rs`.

**Retirement invariant verification at `06f14f5`:**
- (I1) sbec: 4 action refs; Pass verdicts before + after deletion.
- (I2) rust_loc: −32 (strict decrease).
- (I3) test_pass_rate: 6/6 → 6/6 (preserved at 100%).
- (I4) io_violations: 0 → 0 (no new @io introduced).

All four invariants hold empirically at `06f14f5`; the math above
generalises the discipline that the empirical landing already
witnessed at bite-one scale.

## 10. Estimated retirement scope

Per the audit trail in `shards/epistemologic/pact/bilateral.mirror`
docblock (Mara `a0f4d3f`):

| Shard | Predicates | Est. arm LOC | Retired at `06f14f5`? |
|---|---:|---:|---|
| `shards/spectral/signature.mirror` | 4 | ~60 | YES (−32 empirical) |
| `shards/epistemologic/cybernetic/coherence.mirror` | 4 | ~60 | pending |
| `shards/peer/persistence.mirror` | 5 | ~75 | pending |
| `shards/kintsugi/roomba.mirror` (base) | 5 | ~75 | pending |
| `shards/subject/visibility/sheaf.mirror` | 4 | ~60 | pending |
| `shards/uuid/spectral/time.mirror` | 4 | ~60 | pending (c10a3bd landed) |
| `shards/kintsugi/roomba.mirror` (bump/vacuum) | ~3 | ~45 | pending |
| `shards/mirror/store.mirror` (gc_reachability) | 1 | ~15 | pending |
| **TOTAL** | **~30** | **~450** | **~4/30 retired** |

Plus ~250 LOC of error-message prose duplication per the bilateral
shape spec §2. **Aggregate retirement upper bound: ~700 LOC** at
fixed-point.

Post-Reed-tick-C, expected diff shape:
- ~26 additional commits (roughly), each authored by
  `mirror <mirror@spectral.engineer>`, each with net negative
  `rust_loc` delta on the order of −15 to −20 per commit.
- Aggregate: `rust_loc(after) ≈ rust_loc(before) − 450` on the arm
  bodies; further −250 on error-message prose (if the reflective
  evaluator's error format subsumes it, which is a follow-up
  optimisation question).

## 11. Pre-AI prior art

- **Rice (1953)** *"Classes of Recursively Enumerable Sets and Their
  Decision Problems"* — the theorem forbidding non-trivial semantic
  properties of programs from being decidable in general. The
  bilateral shape sidesteps Rice by encoding the semantic content
  (`.contains(...)` sentinel) as byte-visible substrate; the
  predicate `redundant(a, B)` reads only byte-equality between two
  byte-strings, not any program semantics.
- **Floyd-Hoare (1969)** *"An Axiomatic Basis for Computer
  Programming"* — the discipline of proving termination via
  ranking functions. §4 uses `|A(R) ∩ π(B)|` as the ranking
  function.
- **Maturana & Varela (1980)** *Autopoiesis and Cognition* —
  organisation-closure under structure change. §7 shows the
  compiler-as-roomba discharges autopoietically: structure (the
  Rust bytes) plastic; organisation (the reflective corpus
  dispatch discipline) closed.
- **Beer (1972)** *Brain of the Firm* — S1–S5 viable-system model.
  §8 maps the compiler's roomba collapse loop to the five VSM
  subsystems.
- **Connes (1994)** *Noncommutative Geometry* — the (A, H, D)
  spectral triple as the substrate-cross-altitude composition
  primitive per `[[architecture-connes-spectral-triple]]`. §6
  reads this species as a local D operator at reflective-corpus
  altitude.
- **Foerster (1974)** *Cybernetics of Cybernetics* — the
  observer-observed distinction; §7 reads Alex-as-S5 as the
  external observer whose policy the substrate operationalises via
  the autopoietic loop closure.

## 12. Related landings

- Shard: `shards/kintsugi/fracture/bilateral_arm_redundant.mirror`
- Spec: `docs/specs/kintsugi-fracture-bilateral-arm-redundant.md`
- Bilateral shape (a0f4d3f): `shards/epistemologic/pact/bilateral.mirror`
- Bilateral shape spec: `docs/specs/bilateral-predicate-substrate-shape.md`
- Bilateral shape math: `docs/math/epistemologic/pact/bilateral-sentinel.md`
- Reflective evaluator (61c9051/21fc211): `bootstrap/src/apply_h.rs`
- First bite empirical (06f14f5): `shards/spectral/signature.mirror`
- Roomba bump-pulse (2026-07-16 additive): `shards/kintsugi/roomba.mirror`
- `@io/fs.mutate_at` (2026-07-15 Landing 7): `shards/io/fs.mirror`
- `@io/git.commit` (2026-06-24): `shards/io/git.mirror`
- `@glass.source_position` (ff8fbb1 2026-07-15): `shards/glass.mirror`
