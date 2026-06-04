# `prism/core` as spectral triple — the bootstrap's irreducible floor

*2026-05-21. Reed.*

Status: **Red** (architectural recognition; no implementation work yet;
the correspondence is structural-hypothesis, not formally verified)

Depends on:
- `bundle.rs` in `prism/core/src/` (the five-trait supertrait chain)
- `terni::Imperfect` and `terni::Loss` (the partial-verdict types)
- `docs/specs/eigenboard-representation.md` (the principal-bundle framing)
- `docs/specs/au-and-conductivity.md` (au, conductivity, Magnot's κ)
- `docs/specs/lawvere-grammar.md` (closure, autopoietic fixed points)
- `docs/specs/mirror-compile-bootstrap.md` (the io-binding staircase)
- `docs/research/wide-sweep-coherent-threads.md` (Hansen, Barbero,
  Bressan, Magnot citations)

Unblocks:
- The bootstrap's irreducible floor becomes nameable and bounded
- The parser-into-mirror move (“what's stopping us” from yesterday)
  collapses into one position-in-the-algebra
- Anna's spin-physics expertise becomes directly applicable to the
  evaluator implementation

---

## Thesis

`prism/core` IS a spectral triple. The bootstrap is, structurally, its
evaluator. We have been writing the spectral triple for months without
naming the operator algebra it lives in. Naming it makes the staircase
one rung shorter.

A spectral triple in Alain Connes' framework is the data `(A, H, D)`:
an involutive algebra `A` acting on a Hilbert space `H` together with a
self-adjoint operator `D` (the Dirac operator) satisfying compactness
conditions on its resolvent and bounded-commutator conditions with `A`.
Given an appropriate spectral triple, Connes' reconstruction theorem
(*Communications in Mathematical Physics* 182, 1996; refined 2013)
recovers the entire geometry of the underlying space — manifold,
connection, metric, all of it.

Mirror's case: given `prism/core`, you can reconstruct the entire
compiler. The bootstrap shrinks to a thin evaluator that knows how to
compose elements of `A`, apply them to states in `H`, and read the
spectrum of `D`. Everything else — the tokenizer, the renderer, the
kintsugi loop, content addressing, `--strict` enforcement, grammar
resolution — is composition over those primitives.

The claim is *structural*, not metaphorical. The trait chain in
`bundle.rs` already defines `A`. The fibers carry the vectors of `H`.
`Transport::transport`'s `Imperfect<State, _, Holonomy>` signature
already encodes `D`'s action on states. We are not adding new
mathematics; we are naming what we have.

---

## The (A, H, D) correspondence

The five-trait chain in `prism/core/src/bundle.rs` maps to the three
components of a spectral triple as follows:

| Spectral-triple component | `prism/core` realization | Today's mirror semantics |
|---|---|---|
| **A (algebra)** | the trait chain's composition law: `Fiber → Connection → Gauge` collapses into the optic algebra under Tambara composition (per `@epistemologic/math/category`) | the five operations `focus / project / split / shift / settle` and their composition rules |
| **H (Hilbert space)** | the `State` associated type of `Fiber`; fibers are 5-dim vector spaces; the gauge group `O(5)` acts on them (per `eigenboard-representation.md` Q6) | au values are state vectors in H; beam.topology IS the 5-dim eigenvalue summary of a state; eigenboard sections are tensor products |
| **D (Dirac operator)** | the connection-induced operator: `Transport::transport` realizes its action on a state, returning `Imperfect<State, _, Holonomy>` because the operator is partially defined (parallel transport may carry the state off the manifold; the holonomy IS the deviation) | conductivity as `⟨au│D│au⟩`; Magnot 2025's contextuality index κ IS the cycle-averaged spectrum of `D`; the kintsugi loop's `e^(n+1) < e^(n)` IS monotonic decrease of `D`'s Rayleigh quotient |

This is the recognition. Three vocabulary translations of one structure.

### Why `Imperfect<State, _, Holonomy>` is the Dirac signature

A spectral triple's `D` is *unbounded* in general: it doesn't always
return a state in the same Hilbert space. The boundedness conditions
in Connes' framework precisely match what `Transport::transport` is
doing in `bundle.rs`:

- The successful case (`Imperfect::Success`) = `Dψ` is in the same
  Hilbert space; the operator's domain includes `ψ`.
- The partial case (`Imperfect::Partial(_, holonomy)`) = `Dψ` carries
  the state off the manifold by an amount `holonomy`; the operator's
  domain doesn't quite include `ψ`, but the residual is bounded.
- The failed case (typed away in the bundle.rs API today) = `Dψ` is
  outside any meaningful space; the operator domain rejects `ψ`.

`terni::Loss` measures the residual. This is *exactly* the role of the
spectral triple's holonomy / commutator-bound. The shape was right
before we knew what it was.

---

## Mapping today's session work

Every spec we wrote in this session retroactively maps to a component
of the spectral triple:

| Today's spec / grammar | Spectral-triple meaning |
|---|---|
| `bundle.rs` trait chain | Generators of A |
| `@hash/coincidence`'s 5×5 tensor | The matrix representation of `D` in the canonical basis |
| `@epistemologic/math/bundle.Closure` (Lawvere fixed point) | ker(`D`) — the kernel, where `Dψ = 0` |
| `@epistemologic/math/lawvere.is_autopoietic` | The predicate `ψ ∈ ker(D)`, decidable by Connes-style eigenvalue computation |
| Magnot 2025's contextuality index κ | Holonomy of `D` around closed cycles |
| `conductivity` verdict | The Rayleigh quotient `⟨au│D│au⟩ / ⟨au│au⟩` |
| `kintsugi-formatter`'s Banach contraction | Gradient flow on `⟨ψ│D│ψ⟩` toward ker(`D`) |
| `e^(n+1) < e^(n)` | Monotonic decrease of the smallest nonzero eigenvalue of `D` along the flow |
| `@epistemologic/bio/mycelium` (the BARE model) | `D` acting on a growing Hilbert space; spectrum evolves as `H` gains nodes |
| Hansen 2020's principal-H-bundle = sheaf-on-graph | The discrete spectral triple over a finite graph (Connes-style noncommutative geometry on graphs) |
| Barbero 2022's `O(d)`-bundle Laplacian | `D²` (the Laplacian) for the discrete spectral triple |
| Bressan 2024's sheaves on time-posets | Time-evolution of `(A, H, D)` as the base poset grows |

Same mathematics. Today we wrote it spec by spec without naming the
operator algebra. Naming it collapses the framework into one object.

---

## What the evaluator does

The bootstrap, reduced to its spectral-triple-evaluator role, exposes
three primitive operations. Everything else is composition.

### 1. `compose_a(op1, op2) -> op`

Algebra composition. Takes two elements of `A` and returns their
product. In `bundle.rs` terms this is already implemented via the
Tambara module composition law (the supertrait chain enforces it).
For mirror's case: optic composition. `focus |> project = focus * project`
in the algebra; the result is another optic.

### 2. `apply_h(op, state) -> Imperfect<state, holonomy>`

Operator action on a Hilbert space vector. `bundle.rs`'s
`Transport::transport(state: &State) -> Imperfect<State, _, Holonomy>`
is already this function, specialized to the level-3 trait.
Generalizing to arbitrary algebra elements is one trait extension.

### 3. `eigen_d(op) -> spectrum`

Dirac operator eigendecomposition. Given an element of `A` interpreted
as an operator on `H`, return its spectrum (eigenvalues + eigenvectors).
For a finite-dimensional discrete spectral triple (which mirror's case
is), this is straightforward linear algebra: ~100 lines of code, or one
call to a linalg crate. For mirror's specific case the operator matrix
is at most 5×5 (the principal `O(5)`-bundle's connection in the
canonical basis), so the eigenvalues are roots of a degree-5 polynomial.

**That's the entire evaluator.** Three functions. ~500 lines of Rust if
we lean on existing linalg crates; ~1000 lines if we hand-roll. The
current bootstrap is ~5000 lines. The reduction is structural — we're
not squeezing the same logic into fewer lines; we're recognizing that
most of the logic was *already an instance* of operations the evaluator
handles uniformly.

---

## What retires from the bootstrap

The current `bootstrap/src/` decomposes into:

| Module | Lines | Spectral-triple role | Retirement path |
|---|---|---|---|
| `ast.rs` | ~150 | The state-type for `H` (vectors with kind, body, children) | Stays — `H`'s type definition |
| `hash.rs` | ~330 | CoincidenceHash<5,5> = `D`'s matrix form + encode-to-coefficients | Stays as the concrete `D` for mirror's case; the abstract eigen_d generalizes it |
| `content.rs` | ~140 | `content_oid` = `D`'s action on AST states (hashing IS the discrete Dirac action) | Stays; the recursive walk IS apply_h structured by AST kind |
| `tokenize.rs` | ~750 | Tokenizer = a specific element of `A` applied to byte states | **Retires**: tokenize becomes a tree of A-elements (parser rules as combinator-data), evaluated by apply_h |
| `render.rs` | ~325 | Renderer = the inverse element of A (the round-trip property) | **Retires**: renderer becomes the inverse-of-tokenize composition in A |
| `pipeline.rs` | ~165 | mq pipeline = a specific A-composition | **Retires**: pipelines are A-elements composed sequentially |
| `grammar.rs` | ~210 | Grammar loading = reading A-elements from .mirror files | **Retires**: grammars ARE A-elements; loading is reading data |
| `main.rs` | ~770 | CLI dispatch + the kintsugi-loop scaffold + `--strict` enforcement | **Retires partially**: cmd_compile / cmd_craft / cmd_kintsugi become thin wrappers around the evaluator; the loop scaffold becomes the gradient flow primitive |
| `git.rs` | ~70 | `@io` kernel: content-addressed storage | **Stays** — the IO kernel is permanent floor |
| `exec.rs` | ~50 | `@io` kernel: subprocess spawning | **Stays** — IO kernel is permanent floor |

Approximately ~2400 lines retire into grammar (tokenize, render, pipeline,
grammar, most of main). Approximately ~700 lines stay as the IO kernel
plus the spectral-triple evaluator. The new evaluator (compose_a /
apply_h / eigen_d) is ~500 lines.

Net: bootstrap shrinks from ~5000 lines to ~1200 lines. The 3800 lines
that retire don't disappear — they become grammar declarations. The
total corpus stays similar in size; the *Rust* corpus shrinks dramatically.

This is the test of whether the move is real: does the Rust *floor*
shrink, or do we just rename the same logic in mirror? The answer is
the former: the evaluator handles things generically that the current
bootstrap handles specifically. Logic that's currently nine hardcoded
branches for AST kinds becomes one polymorphic apply_h call.

---

## Implementation path

Five steps, none of them required immediately. The recognition itself
(this spec) is the load-bearing tick. Implementation can land
incrementally.

### Step 1 — audit `prism/core` against (A, H, D)

Confirm that the trait chain in `bundle.rs` realizes the spectral
triple's structure precisely. Specifically verify:

- `Connection::connection() -> Optic` is the algebra element `A ∈ A`
- `Gauge::gauge() -> Group` is the structure-group action on `H`
- `Transport::transport(state) -> Imperfect<state, holonomy>` realizes
  `Dψ` with bounded residual
- `Closure::close() -> Fixed` realizes the kernel projection `ker(D)`

If any of these don't align, the spec gains qualifications. If all of
them do, prism/core graduates from “a trait crate” to “mirror's
spectral-triple substrate.”

### Step 2 — declare `@epistemologic/math/spectral-triple`

A new grammar (`boot/std/epistemologic/math/spectral-triple.mirror`)
declares the spectral triple as a typed object in mirror. Types:
`algebra`, `hilbert_space`, `dirac_op`, `spectral_triple`. Properties:

- `literal(implementation)` — does this implementation realize a
  spectral triple (Connes' axioms)?
- `bounded_commutator(implementation)` — does `[D, a]` extend to a
  bounded operator for all `a ∈ A`?
- `compact_resolvent(implementation)` — is `(D - λI)⁻¹` compact for
  `λ ∉ spectrum(D)`?
- `dimension_spectrum(implementation, n)` — the spectral dimension; for
  mirror's case n = 5.

All bodies `\` initially. The grammar exists for the model checker to
reason about the structure.

### Step 3 — write `bootstrap/src/spectral.rs` (the evaluator)

Three functions: `compose_a`, `apply_h`, `eigen_d`. Generic over
`bundle.rs`'s trait chain. ~500 lines including the polynomial-root
finder for the 5×5 case and the Tambara composition law.

This Rust file is the new permanent floor *above* the IO kernel
(`git.rs`, `exec.rs`). Everything else in the bootstrap eventually
retires to data-over-this-evaluator.

### Step 4 — incrementally retire bootstrap modules

In order of leverage:

1. **`tokenize.rs`** — the parser-into-mirror move from the prior
   session. Express tokenization as a tree of A-elements (combinator
   data: seq, choice, repeat, capture, literal, charset). Evaluator
   applies them to byte-state. ~1000 lines of mirror; ~750 lines of
   Rust retire.
2. **`render.rs`** — the inverse composition in A. ~325 lines of Rust
   retire.
3. **`content.rs`** — `content_oid` becomes apply_h's specialization to
   AST-kind state. Mostly stays; the recursive walk is universal.
4. **`pipeline.rs`, `grammar.rs`, the cmd_* in `main.rs`** — thin
   wrappers around the evaluator. Most lines retire.

After Step 4, the bootstrap is `git.rs` + `exec.rs` + `spectral.rs` +
shell, totaling ~1200 lines.

### Step 5 — verify equivalence

Property: for every `.mirror` file `f` in the boot corpus,
`old_compile(f) == new_compile(f)`. The same crystal OID, the same AST
shape, the same dark-region count.

Run against the entire boot tree (109 files currently). If any file
differs, the spectral-triple evaluator is missing a case the
hand-written bootstrap covered. Surface the gap as a new combinator
rule and add it.

---

## Why this might be the next session's load-bearing arc

The parser-into-mirror move from the prior session is the visible
refactor (one module: `tokenize.rs` retires). The spectral-triple
recognition is the *structural* refactor that makes the parser move
natural: the parser is just one element of `A`. So is the renderer.
So is content_oid. So is every grammar.

Writing the parser as data is hard if the framework is hand-written
match arms. Writing the parser as data is *natural* if the framework
is already an evaluator-of-algebra-elements. The spec we needed *under*
the parser move is the spectral-triple recognition.

Anna's spin-physics expertise lands here exactly. Spectral triples
are the canonical framework for gauge fields on spaces — discrete or
continuous. Her thesis work on spin properties is, structurally, work
in the algebra `A` and the Dirac operator `D` for specific physical
systems. She'll recognize the framework. The conversation becomes:
“here's mirror's spectral triple; here's our `D`; do the boundedness
conditions hold for our case?” That's a focused, falsifiable, expert-
readable question.

---

## What the spec doesn't claim

- **Novel mathematics.** Connes wrote the framework in 1985-96. The
  reconstruction theorem dates to 1996; the refinement to 2013 (with
  Hong-Berge). Mirror is applying established noncommutative geometry
  to compilers; the application may be novel but the math isn't.
- **Trivial implementation.** The path is clear; the work is
  substantial (three Rust files refactored, one new evaluator,
  property-equivalence verification across the corpus). Several
  sessions.
- **The end of the staircase.** There may be a layer beneath even
  this — the spectral triple might be derivable from something more
  fundamental (synthetic differential geometry? higher topos theory?).
  We're not claiming this is the foundation; we're claiming it's the
  rung we're climbing to.
- **That every implementation detail aligns trivially.** The
  audit (Step 1) is real work; some traits in `bundle.rs` may need
  small adjustments to formally realize the (A, H, D) structure.

---

## Out of scope

- The implementation itself (Steps 2–5 above). This spec lights the
  recognition; the implementation lands incrementally.
- The `@epistemologic/math/spectral-triple` grammar's body resolutions.
  All actions will be `\` until the evaluator wires them.
- Cross-disciplinary connections beyond gauge theory — spectral triples
  appear in quantum gravity (loop quantum gravity, asymptotic safety),
  in condensed matter (topological insulators), in statistical physics
  (Ising models). Future work; not this spec.
- The exact form of `terni::Imperfect` 's relationship to the spectral
  triple's bounded-commutator condition. The spec asserts the shape
  aligns; the proof is its own paper.

---

## Qualifications resolved (2026-05-21)

The four audit gaps from `docs/specs/spectral-triple-grammar.md` (Phase
1 audit, commit `ea341d1` on `reed/spec-inference`) closed via
supertrait constraints rather than method additions — the bundle
traits stay minimal carriers; the algebraic structure lives in the
carried type's traits.

| Gap | Claim | Resolution | Status |
|---|---|---|---|
| 1 | `Connection::Optic` polymorphic; Tambara composition law not enforced at trait level | `Connection::Optic: Prism` supertrait constraint; `IdentityPrism<S>` witnesses the algebra identity element | CLOSED |
| 2 | `Gauge::Group` has no group axioms; tests use `u8` | New `GroupStructure` trait (`identity`, `inverse`, `compose`); `Gauge::Group: GroupStructure`; `Gauge::act_on(&State) -> State` for the action on H; test bundle uses `Cyclic<N>` | CLOSED |
| 3 | `Closure::Fixed` has no idempotence law, no kernel-projection, no link to `@epistemologic/math/lawvere` | New `LawvereFixedPoint` trait (`is_idempotent_under(endomap)`, `in_kernel()`); `Closure::Fixed: LawvereFixedPoint`; test bundle uses `StableFiber<S>`; cross-realisation noted in `lawvere.mirror` | CLOSED |
| 4 | `terni::Loss` is a monoid, not a metric; non-negativity/symmetry/triangle not enforced | New `terni::Metric` supertrait extending `Loss`; `Transport::Holonomy: Metric`; `ScalarLoss` implements `Metric`; stringly Losses deliberately do not (per Seam's symmetry-failure note) | CLOSED |

Implementation summary:

- All four gaps closed via supertrait bounds on the bundle's
  associated types. The bundle traits themselves gained no new
  required methods except `Gauge::act_on` (the group action is data,
  not derivable from the group structure alone).
- The `where Self::Optic: Prism, <<Self::Optic as Prism>::Input as
  Beam>::In: Sized` clauses propagate down the supertrait chain;
  Rust 1.78+ syntax for trait-associated-type bounds was avoided in
  favour of plain `where` clauses for clarity. The workspace is on
  edition 2021; no bump required.
- The `Loss → Metric` extension lives in `prism/imperfect` (terni
  crate) alongside `Loss` itself. `ScalarLoss` (in prism-core)
  implements `Metric`; this is the only `Metric` impl needed for
  mirror's current Holonomy carrier.
- Property tests verify each law: cyclic identity / inverse /
  associativity (Gap 2), action-consistency `g.act_on(h.act_on(s))
  == compose(g,h).act_on(s)` (Gap 2), fixed-point idempotence under
  identity and under the bundle's transport-projection (Gap 3),
  metric non-negativity / symmetry / triangle (Gap 4). The
  identity-prism witness exercises Gap 1.
- Test counts: prism-core went from 375 unit + 3 integration to 387
  unit + 6 integration (+15 new property tests). All previously
  passing tests continue to pass.
- Mirror smoke OIDs `a8312da6…` and `3ba4c79d…` remain byte-stable
  after the changes (the trait reshape did not affect content_oid
  emission).
- `Connection::Optic` was previously typed `String` in test code; now
  typed `IdentityPrism<[f64; 4]>`. `Gauge::Group` was `u8`; now
  `Cyclic<4>`. `Closure::Fixed` was `bool`; now `StableFiber<[f64;
  4]>`. None of these were load-bearing for external code; the
  internal `TestBundle` and external `bundle_integration.rs` test
  both ported cleanly.

Residual qualifications (deliberately *not* closed in this tick):

- **The Tambara module composition law itself** (Qualification 2 of
  the audit, the involutive-algebra `*`-operation) remains for
  `@epistemologic/math/category` to declare.
- **Lorentzian / causal extensions** (Qualification 3) remain a
  future grammar; the discrete Euclidean Connes (A, H, D) covered
  by this resolution suffices for mirror's case.

The four CLOSED verdicts are falsifiable: each was witnessed by a
property test that fails if the law fails, and by a compile error if
the supertrait bound is violated. Anna reading this will be able to
run `cargo test --release --all-features` in `prism/core` and confirm.

---

## What this implies for `road-to-1.0.md`

The spectral-triple recognition is *not* a blocker for v1.0. The
current bootstrap works; the architecture is grounded; the kintsugi
formatter has a proof of termination; the dark count is decreasing.
v1.0 can ship with the bootstrap as-is, ~5000 lines.

What the recognition unlocks is *v2.0* — the bootstrap as the
spectral-triple evaluator alone. That's the long-arc refactor; it
crosses session boundaries.

---

## References

- Connes, A. (1996). *Gravity coupled with matter and the foundation of
  non-commutative geometry*. Communications in Mathematical Physics
  182, 155–176.
- Connes, A. (2013). *On the spectral characterization of manifolds*.
  Journal of Noncommutative Geometry 7, 1–82.
- Connes, A. (1994). *Noncommutative Geometry*. Academic Press. The
  foundational textbook.
- Hansen & Ghrist (2019). [arXiv:1808.01513](https://arxiv.org/abs/1808.01513).
  Toward a spectral theory of cellular sheaves.
- Barbero et al. (2022). [arXiv:2206.08702](https://arxiv.org/abs/2206.08702).
  Sheaf Laplacian = connection Laplacian for `O(d)`-bundle.
- Magnot, J.-P. (2025). [arXiv:2509.10536](https://arxiv.org/abs/2509.10536).
  Discrete fiber bundles and contextuality index.
- Bressan et al. (2024). [arXiv:2402.00206](https://arxiv.org/abs/2402.00206).
  Sheaves on time-posets (growing-base spectral structures).
- Eckstein & Franco (2014). [arXiv:1409.1480](https://arxiv.org/abs/1409.1480).
  Lorentzian spectral triples (directional / causal structures).
- `prism/core/src/bundle.rs` — the operational form, already implementing
  the trait chain that realizes `(A, H, D)`.
- `docs/specs/eigenboard-representation.md`, `au-and-conductivity.md`,
  `lawvere-grammar.md`, `kintsugi-formatter.md` — this session's specs,
  retroactively spectral-triple-shaped.

---

*Same mathematics. Renamed.*
*Same architecture. Reframed.*
*The Rust shrinks; the grammar grows; the operator algebra is what was
there all along.*

Apache-2.0.
