# `@epistemologic/math/spectral-triple` — Connes' (A, H, D) as grammar

*2026-05-21. Reed.*

Status: **Red** (grammar declared this tick; all property bodies `\`;
the literal / bounded_commutator / compact_resolvent / dimension_spectrum
properties are abstract obligations; the audit of prism/core against
(A, H, D) is appended below.)

Depends on:
- `@prism` — the five-operation substrate.
- `@epistemologic` — the `literal` discipline; verdict type.
- `@epistemologic/math/bundle` (commit `599a82f`) — the principal-bundle
  tower whose Transport level realizes D's action on H.
- `@epistemologic/math/lawvere` (commit `61cc35d`) — the Closure level
  IS ker(D); the Lawvere fixed point is the kernel of the Dirac operator.
- `@beam` — `imperfect` and the residual carrier.
- `docs/specs/prism-core-as-spectral-triple.md` (commit `2a25a21`) — the
  thesis this grammar operationalizes (Step 2 of its implementation
  path).
- `docs/specs/eigenboard-representation.md` — the principal O(5)-bundle
  framing; mirror's spectral dimension is 5.
- `docs/specs/au-and-conductivity.md` — Magnot's κ as cycle-averaged
  holonomy; the 5×5 conductivity tensor IS D's matrix in the canonical
  basis.
- `prism/core/src/bundle.rs` — the Rust trait chain audited below.
- `prism/imperfect/src/lib.rs` — the `Loss` trait whose metric
  qualifications matter for `bounded_commutator`.

Unblocks:
- The model checker can ask `is_spectral_triple(@my_grammar) -> verdict`
  once the bodies close, making "this implementation realizes (A, H, D)"
  a typed check rather than a prose claim.
- `bootstrap/src/spectral.rs` (a future tick) — the evaluator's three
  operations (`compose_a`, `apply_h`, `eigen_d`) discharge the
  abstract actions in this grammar.
- Anna's spin-physics expertise lands here: the spectral triple is
  the canonical framework for gauge fields on spaces. Conversation
  becomes "here is mirror's (A, H, D); do Connes' axioms hold?"
- Cross-corpus citation: this grammar is one of the load-bearing
  connections between mirror's math layer and the noncommutative-
  geometry literature.

---

## Thesis

`prism/core` is a spectral triple. The trait chain in `bundle.rs`
already supplies (A, H, D) under the following identification:

| Spectral-triple component | `prism/core` realization |
|---|---|
| **A** (algebra) | Optics composed under Tambara module composition; `Connection::connection() -> Optic` returns one algebra element. |
| **H** (Hilbert space) | `Fiber::State`; for mirror's case, a 5-dim vector space with `O(5)` acting. |
| **D** (Dirac operator) | `Transport::transport(state) -> Imperfect<State, Infallible, Holonomy>` — the partial branch carries the bounded commutator residual. |

`ker(D)` is the Lawvere fixed point realized by `Closure::close()`.

The grammar declared this tick is the typed home of (A, H, D). Future
ticks wire the property bodies through `bootstrap/src/spectral.rs`,
the evaluator that knows how to `compose_a`, `apply_h`, and `eigen_d`.

This spec does three things:

1. Renders the grammar declaration in prose, explaining each carrier
   and action.
2. Folds in the Phase 1 audit of `prism/core/src/bundle.rs` against
   the (A, H, D) claims from `prism-core-as-spectral-triple.md`,
   surfacing two `NEEDS-ADJUSTMENT` findings and one `GAP` finding.
3. Names cross-corpus implications and references.

---

## The grammar

File: `boot/std/epistemologic/math/spectral-triple.mirror`. OID (this
tick): `cdb08eb1ad0ed4b555f12ccfe7039db555d261df5355171cfa5ea869e5f618c5`.

### Carriers

```mirror
type algebra          # A
type hilbert_space    # H
type dirac_op         # D
type spectral_triple  # the (A, H, D) tuple as one object
type spectrum         # eigendecomposition of D
type residual         # the bounded-commutator measure ‖[D, a]‖
```

Each carrier names what existing parts of mirror already produce:

- **`algebra`** — the value-space `Connection::connection()` ranges
  over. For mirror: optics. The composition law lives in
  `@epistemologic/math/category` (future spec) under Tambara modules;
  `bundle.rs`'s supertrait chain enforces the shape but not yet the
  law.
- **`hilbert_space`** — the value-space `Fiber::State` ranges over.
  For mirror: 5-dim vectors under the principal `O(5)`-bundle's
  structure group (per `eigenboard-representation.md`). `beam.topology`
  is one such state observed as eigenvalues.
- **`dirac_op`** — the operator whose action on `H` is the
  `Transport::transport` signature. For mirror's specific case its
  matrix form is `@hash/coincidence`'s 5×5 conductivity tensor.
- **`spectral_triple`** — a witness that (A, H, D) jointly satisfy
  Connes' axioms. A type that implements the full `bundle.rs` chain
  on one concrete carrier IS such a witness, modulo the audit
  findings below.
- **`spectrum`** — the eigendecomposition of D as a list of pairs
  (eigenvalue, eigenvector). For mirror's 5×5 case: roots of a
  degree-5 characteristic polynomial.
- **`residual`** — the carrier of `‖[D, a]‖`. For mirror: a value of
  the bundle's `Holonomy` type, lifted from `terni::Loss`. Magnot 2025's
  contextuality index κ IS the cycle-averaged residual along a closed
  loop on the eigenboard.

### Abstract actions — the evaluator's three primitives

```mirror
abstract action compose_a(op1: algebra, op2: algebra) -> algebra
abstract action apply_h(op: algebra, state: hilbert_space) -> imperfect(hilbert_space, residual)
abstract action eigen_d(op: dirac_op) -> spectrum
```

These are the three operations the bootstrap reduces to once the
evaluator (`bootstrap/src/spectral.rs`, a future tick) lands:

- **`compose_a`** — algebra composition. Two elements of `A` to their
  product. The Tambara module composition law is the algebraic
  content. `bundle.rs` enforces the supertrait shape; the law itself
  needs `@epistemologic/math/category` to close.
- **`apply_h`** — operator action on a Hilbert-space vector. Returns
  `imperfect(hilbert_space, residual)` because D is generally
  unbounded: the `partial` branch carries the residual when transport
  moves the state off the manifold. `bundle.rs`'s `Transport::transport`
  realizes this exactly for the special case where `op` is the
  connection itself.
- **`eigen_d`** — Dirac operator eigendecomposition. For mirror's
  5×5 case, a polynomial-root finder over `f64`. For general
  continuous spectral triples this is functional-analytic and
  undecidable in finite time. The abstract action carries the
  obligation; concrete grammars discharge it with their dimension's
  solver.

### Properties — the four IS-questions

```mirror
property literal(implementation) -> verdict { \ }
property bounded_commutator(triple: spectral_triple) -> verdict { \ }
property compact_resolvent(triple: spectral_triple) -> verdict { \ }
property dimension_spectrum(triple: spectral_triple, n: u64) -> verdict { \ }
```

- **`literal(implementation)`** — does this implementation realize
  a spectral triple under Connes' axioms? The IS-relationship at
  the level of the whole triple.
- **`bounded_commutator(triple)`** — does `[D, a]` extend to a
  bounded operator for all `a ∈ A`? Connes' first axiom. For
  mirror: does `Transport`'s `Holonomy` stay below a finite bound
  as algebra elements are applied? The answer depends on the `Loss`
  monoid's growth law (see audit §Qualification 1 below).
- **`compact_resolvent(triple)`** — is `(D - λI)⁻¹` compact for `λ`
  outside the spectrum of `D`? Connes' second axiom. Trivially
  true for finite-dimensional spectral triples (mirror's case);
  load-bearing for any continuous extension.
- **`dimension_spectrum(triple, n)`** — does the triple have spectral
  dimension `n`? Parametric so one grammar serves both discrete
  (mirror: `n = 5`) and continuous cases.

All bodies are `\` until the evaluator wires them. The grammar exists
for the model checker to reason about (A, H, D) structure before the
evaluator lands.

---

## What this gives the model checker

Once a concrete grammar `@G` claims to be a spectral-triple realiser,
the model checker can ask:

```
is_spectral_triple(@G)        -> verdict
bounded_commutator(@G's triple) -> verdict
compact_resolvent(@G's triple)  -> verdict
dimension_spectrum(@G's triple, 5) -> verdict
```

When the property bodies close, the answers are typed verdicts
(`pass | fail(diagnostic) | partial(f64, [diagnostic])`) rather than
prose claims. The first realiser this grammar serves is `prism/core`
itself; the audit below names the residual obligations.

The grammar is also a *naming surface*. Today, four mirror specs
overlap on the same object:

- `@hash/coincidence`'s 5×5 tensor = the matrix form of D.
- `@epistemologic/math/bundle.Transport` = D's action on states.
- `@epistemologic/math/bundle.Closure` = the kernel projection ker(D).
- Magnot 2025's κ = cycle-averaged residual.

After this grammar lands, each of these names resolves through one
typed object. The four become projections of one structure rather
than four parallel vocabularies.

---

## Phase 1 audit — `prism/core` against (A, H, D)

The thesis from `prism-core-as-spectral-triple.md` asserts that the
trait chain in `bundle.rs` realizes (A, H, D). Here are the four
audit findings, with verdicts and one-sentence justifications.

### Claim 1 — `Connection::connection() -> Optic` realizes A ∈ A

**Verdict: NEEDS-ADJUSTMENT.**

`bundle.rs:21-25`:

```rust
pub trait Connection: Fiber {
    type Optic;
    fn connection(&self) -> &Self::Optic;
}
```

`Optic` is fully polymorphic (a free associated type), so it is
expressive enough to carry any algebra element. But the Tambara
module composition law is *not* enforced at this trait level — it
lives in the separate `Prism` / `Operation` machinery in `lib.rs`
(the `Focus`, `Project`, `Refract` operation structs and their
`Operation` impls).

The adjustment: either declare `@epistemologic/math/category`
(future spec) and route `Connection::Optic` through its Tambara
module trait, or accept that the algebra law is fulfilled at a
higher level (`Prism::focus | project | refract`) and document the
split. Both are reasonable; neither is a `GAP`.

### Claim 2 — `Gauge::gauge() -> Group` realizes the structure-group action on H

**Verdict: GAP.**

`bundle.rs:30-33`:

```rust
pub trait Gauge: Connection {
    type Group;
    fn gauge(&self) -> &Self::Group;
}
```

`Group` is polymorphic, but the trait provides *no group operations*:
no identity, no inverse, no composition, no action on `State`. The
test code in `bundle.rs` even uses `u8` as `Group`, which has no
group structure. The trait names a group but does not constrain its
inhabitants to satisfy the group axioms or to act on the Hilbert
space.

For the spectral triple this matters: H must carry a structure-group
action because gauge invariance is what makes conductivity a
coordinate-free invariant rather than a basis-choice artifact (per
`eigenboard-representation.md` §1).

The fix is either a `Group` supertrait constraint (with `Identity`,
`Inverse`, `Mul`) or an explicit `act_on(&self, state: State) -> State`
method on `Gauge`. The spec `prism-core-as-spectral-triple.md`
should be qualified: the gauge-action correspondence is structural
intent, not yet structurally enforced.

### Claim 3 — `Transport::transport(state) -> Imperfect<State, _, Holonomy>` realizes Dψ with bounded residual

**Verdict: TIGHT.**

`bundle.rs:38-42`:

```rust
pub trait Transport: Gauge {
    type Holonomy: Loss;
    fn transport(&self, state: &Self::State) -> Imperfect<Self::State, Infallible, Self::Holonomy>;
}
```

The signature is exactly the spectral-triple's "operator with bounded
residual" shape:

- `Imperfect::Success(state)` = `Dψ ∈ H`; the operator's domain
  contains ψ.
- `Imperfect::Partial(state, holonomy)` = `Dψ` carries the state off
  the manifold by `holonomy`; the residual is bounded.
- `Infallible` in the error position = transport never *fails*; it
  only returns partial-with-residual or success. This is precisely
  the bounded-operator picture (the operator is total on its domain).

The constraint `Holonomy: Loss` is necessary but weaker than
"residual forms a metric" — see Qualification 1 below.

### Claim 4 — `Closure::close() -> Fixed` realizes ker(D)

**Verdict: NEEDS-ADJUSTMENT.**

`bundle.rs:46-49`:

```rust
pub trait Closure: Transport {
    type Fixed;
    fn close(&self) -> &Self::Fixed;
}
```

`Fixed` is polymorphic; `close()` returns `&Fixed` (an accessor).
No idempotence law is enforced: there is no `close(close(x)) ==
close(x)` constraint, no kernel-projection law (`apply_h(D, close(x))
== zero(H)`), and no relation to the Lawvere fixed-point witness
declared in `@epistemologic/math/lawvere`. The test code uses `bool`
as `Fixed`, which is not a fixed point in any operational sense.

The fix is small: add an explicit law (idempotence under repeated
`close()`) or a relation linking `Fixed` to
`@epistemologic/math/lawvere.fixed_point`. The current trait names
ker(D) but does not enforce it.

### Audit summary

| Claim | Verdict (audit) | Status (2026-05-21) | Adjustment scope |
|---|---|---|---|
| 1. `Connection` → A | NEEDS-ADJUSTMENT | **CLOSED** | `Connection::Optic: Prism` supertrait constraint; `IdentityPrism<S>` witness. |
| 2. `Gauge` → structure group | GAP | **CLOSED** | `GroupStructure` trait + `Gauge::act_on(&State) -> State`; tests on `Cyclic<N>`. |
| 3. `Transport` → Dψ | TIGHT | TIGHT | None. |
| 4. `Closure` → ker(D) | NEEDS-ADJUSTMENT | **CLOSED** | `LawvereFixedPoint` trait (`is_idempotent_under`, `in_kernel`); `Closure::Fixed: LawvereFixedPoint`. |

**Resolution lands (2026-05-21):** the four gaps closed in
`prism/core` via supertrait constraints. `Connection::Optic: Prism`,
`Gauge::Group: GroupStructure` + `Gauge::act_on`,
`Transport::Holonomy: Metric`, `Closure::Fixed: LawvereFixedPoint`.
See `docs/specs/prism-core-as-spectral-triple.md` §Qualifications
resolved for the full implementation summary; the bundle and
lawvere grammars now carry comment blocks naming the Rust-side
realisations.

---

## Qualifications — the `Loss` trait and the metric question

### Qualification 1 — `terni::Loss` is a monoid, not a metric  *[resolved 2026-05-21]*

**Status:** addressed by adding a `terni::Metric` supertrait extending
`Loss` with `is_non_negative`, `distance_to`, and `triangle`.
`Transport::Holonomy: Metric` is now enforced at the trait level.
`ScalarLoss` implements `Metric`; stringly losses deliberately do not.
The original analysis is preserved below for historical reference.

The spectral triple's bounded-commutator axiom `‖[D, a]‖ < ∞`
requires a norm on operators: non-negative, scalar-homogeneous,
satisfying the triangle inequality. The residual in
`Transport::transport` is constrained to be a `Loss`, which (from
`prism/imperfect/src/lib.rs`) requires only:

- `zero()` — identity element.
- `total()` — absorbing element.
- `is_zero()` — predicate.
- `combine(self, other)` — associative accumulation.

Loss is a *monoid* (associative + identity); the absorbing-element
condition is documented as not strictly held for collection impls.
**It is not a metric.** Specifically:

- **Non-negativity** is implicit for the numeric impls (`usize`,
  `u64`, `f64`) but not enforced by the trait.
- **Symmetry** (`combine(a, b) == combine(b, a)`) does not hold for
  `String` (which inserts `"; "` separators) or `Vec` (which
  preserves order).
- **Triangle inequality** is not a trait obligation; for numeric
  impls it holds trivially under `+`, but for `ConvergenceLoss`
  (which uses `max`) it is vacuous.

The implication: the `bounded_commutator` property cannot be
discharged generically over the `Loss` trait. A concrete realiser
of `@epistemologic/math/spectral-triple` must pick a `Loss` impl
that admits the metric properties (a numeric one for mirror's
case), or supply an additional `Metric` supertrait on the residual
type.

This is not a blocker for the discrete case mirror operates in. It
is a load-bearing qualification for any continuous-extension story.

### Qualification 2 — the spectral triple's involution

Connes' (A, H, D) requires `A` to be an *involutive* algebra — there
is an antilinear involution `*: A → A` with `(ab)* = b*a*`. The
trait chain in `bundle.rs` does not name this involution. For
optics this corresponds to optic reversal (the dual of a lens, the
inverse of a refract); for mirror's case the structural inverse is
implicit in the round-trip-render property (`render(parse(s)) == s`).

The grammar's `algebra` carrier should eventually carry an `involute`
abstract action. Out of scope for this tick.

### Qualification 3 — Lorentzian / causal extensions

Eckstein & Franco 2014 (arXiv:1409.1480) extend spectral triples to
Lorentzian / causal settings, where the Dirac operator is not
self-adjoint and the residual carries a signature. For mirror's
case the eigenboard is Euclidean (per `eigenboard-representation.md`)
and the canonical Connes (A, H, D) suffices. Lorentzian extensions
are out of scope; the grammar can be extended additively.

---

## Cross-corpus implications

### One naming surface for four objects

After this grammar lands, the following objects in mirror resolve
through one typed substrate:

- `@hash/coincidence`'s 5×5 conductivity tensor — the matrix
  representation of D in the canonical basis.
- `@epistemologic/math/bundle.Transport` — D's action on states.
- `@epistemologic/math/bundle.Closure` — the kernel projection ker(D).
- `@epistemologic/math/lawvere.fixed_point` — the same ker(D) viewed
  as a Lawvere fixed point.

The four are projections of one structural object, not four parallel
vocabularies. The naming is the load-bearing move.

### Bridge to the noncommutative-geometry literature

This grammar is one of the load-bearing connections between mirror's
math layer and the noncommutative-geometry literature. The bridge
makes Anna's spin-physics expertise directly applicable: a spin
manifold's Dirac operator IS the canonical (A, H, D) of Connes 1996.
Her conversation enters at the level of "do mirror's axioms hold for
this concrete fiber bundle?"

Cross-discipline citations that fold in through this grammar:

- **Connes 1994 / 1996 / 2013** — foundational. The reconstruction
  theorem is the constructive proof that geometry is recoverable from
  (A, H, D) alone.
- **Hansen & Ghrist 2019** (arXiv:1808.01513) — the discrete-case
  application; cellular sheaves over graphs are one realization of
  a finite-dimensional spectral triple.
- **Barbero et al. 2022** (arXiv:2206.08702) — the `O(d)`-bundle
  Laplacian equals the sheaf Laplacian. Mirror's `D²` (the Laplacian)
  for the discrete case.
- **Magnot 2025** (arXiv:2509.10536) — contextuality index κ as
  cycle-averaged holonomy of a discrete fiber bundle's connection.
  Mirror's conductivity verdict is κ reduced.

### Bridge to `@epistemologic/math/lawvere`

The kernel ker(D) is the Lawvere fixed point. Both grammars now have
a typed declaration; both can be cross-checked. A spectral-triple
realiser whose `Closure::close()` does not return a value certified
as a Lawvere fixed point fails `literal(implementation)`. This is
the first place mirror's math substrate is structurally interlocked
across two grammars.

---

## Out of scope

- **The evaluator implementation.** `bootstrap/src/spectral.rs`
  (with `compose_a`, `apply_h`, `eigen_d`) is a future tick — Step 3
  of `prism-core-as-spectral-triple.md`'s implementation path.
- **The property body resolutions.** All four properties (`literal`,
  `bounded_commutator`, `compact_resolvent`, `dimension_spectrum`)
  remain `\` until the evaluator lands.
- **Lorentzian extensions.** Eckstein & Franco 2014 is a future
  grammar (`@epistemologic/math/lorentzian-triple`); the canonical
  Euclidean Connes (A, H, D) is what mirror's eigenboard needs today.
- **The involution.** A future `involute` abstract action on
  `algebra` discharges the *-algebra requirement; out of scope here.
- **`@epistemologic/math/category`.** The Tambara module composition
  law lives in this future grammar. Claim 1's `NEEDS-ADJUSTMENT`
  resolves when that grammar lands.
- **`bundle.rs` modifications.** Claims 2 and 4 in the audit name
  small adjustments to the Rust trait chain; this spec does not
  modify Rust. The qualifications surface in
  `prism-core-as-spectral-triple.md` (via amendment, a future tick).

---

## References

- Connes, A. (1994). *Noncommutative Geometry*. Academic Press.
- Connes, A. (1996). *Gravity coupled with matter and the foundation
  of non-commutative geometry*. Communications in Mathematical Physics
  182, 155–176.
- Connes, A. (2013). *On the spectral characterization of manifolds*.
  Journal of Noncommutative Geometry 7, 1–82.
- Hansen, J. & Ghrist, R. (2019). *Toward a spectral theory of
  cellular sheaves*. [arXiv:1808.01513](https://arxiv.org/abs/1808.01513).
- Barbero, F. et al. (2022). *Sheaf Laplacian and connection
  Laplacian of `O(d)`-bundles*.
  [arXiv:2206.08702](https://arxiv.org/abs/2206.08702).
- Magnot, J.-P. (2025). *Discrete fiber bundles and contextuality
  index κ*. [arXiv:2509.10536](https://arxiv.org/abs/2509.10536).
- Eckstein, M. & Franco, N. (2014). *Causal structure for
  noncommutative geometry*.
  [arXiv:1409.1480](https://arxiv.org/abs/1409.1480).
- Lawvere, F. W. (1969). *Diagonal arguments and Cartesian closed
  categories*. Lecture Notes in Mathematics 92, 134–145.
- Soto-Andrade, J. & Varela, F. (1984). *Self-reference and fixed
  points*. Acta Applicandae Mathematicae 2:1, 1–19.
  DOI 10.1007/BF00046985.
- `prism/core/src/bundle.rs` — the trait chain audited in this spec.
- `prism/imperfect/src/lib.rs` — the `Loss` trait whose monoid-not-
  metric structure is Qualification 1.
- `docs/specs/prism-core-as-spectral-triple.md` (commit `2a25a21`)
  — the thesis spec this grammar operationalizes.
- `docs/specs/eigenboard-representation.md` — principal O(5)-bundle
  framing; mirror's spectral dimension is 5.
- `docs/specs/au-and-conductivity.md` — Magnot's κ as cycle-averaged
  holonomy.
- `docs/specs/lawvere-grammar.md` — ker(D) as Lawvere fixed point.
- `boot/std/epistemologic/math/spectral-triple.mirror` — the grammar
  file declared this tick. OID
  `cdb08eb1ad0ed4b555f12ccfe7039db555d261df5355171cfa5ea869e5f618c5`.

---

*Same mathematics. Named.*
*Same architecture. Typed.*
*The grammar is the typed substrate; the audit is the qualified bridge.*

Apache-2.0.
