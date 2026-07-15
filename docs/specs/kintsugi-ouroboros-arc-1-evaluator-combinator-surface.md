---
date: 2026-07-15
author: Mara
scope: Arc-1 evaluator FLOOR combinator surface — formal enumeration of the concrete primitives the Rust FLOOR exposes so any shard body can dispatch through them. Discharges A6 from the Seam Phase D adjudication (`docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md` §D12 + Mara-B §7.6). Provides the concrete surface Seam adjudicates at Arc-1 Tick 1.1 sign-off and the concrete blueprint Reed's Arc-1 Ticks 1.2–1.4 land under `[substrate-floor:@io-boundary]` + Seam sign-off.
status: proposal
companion:
  - docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md
  - docs/specs/kintsugi-ouroboros-compiler-self-collapse.md
  - docs/specs/eigensheaf.md
  - docs/specs/bootstrap-retirement-plan.md
  - shards/kintsugi/ouroboros.mirror
  - shards/io.mirror
  - shards/kintsugi.mirror
  - shards/mirror/lens/knife.mirror
  - shards/mirror/spectral.mirror
  - bootstrap/src/spectral.rs
---

# Arc-1 evaluator combinator surface — the calculus for shard-body dispatch

*Composition-only. Two-tick discipline. Every combinator irreducible past the @io-boundary. The surface is 7 primitives; everything else in shard bodies composes over them + @io.*

---

## §0 Prelude — ancestry and framing

### §0.1 The re-fired directive

Alex fired /loop after Reed reported terminal Alex-only state on A6. The
re-firing interprets: Alex wants Mara to formalize the concrete
combinator surface, so Seam has a concrete surface to adjudicate
substrate-honesty over and Alex has an artifact to react to rather
than a framing question. This spec is the artifact.

Reed's terminal report treated A6 as
Alex-frame-first-then-Seam-adjudicate. Alex's re-fire treats A6 as
Mara-authors-surface-then-Seam-adjudicates-substrate-honesty-then-
Alex-ratifies-if-residue-remains. The load-bearing move here is
authoring the surface. Ambiguity that survives the authoring is
what Alex adjudicates. Ambiguity dischargeable by Seam adversarial
review over a concrete artifact is what Seam adjudicates. The
sub-tick this spec targets is authoring the artifact so both
downstream adjudications can proceed with an object to reason over.

### §0.2 Ancestry chain (verbatim citations, path + section)

1. **Seam Phase D §D5 Arc-1 verdict.**
   `docs/audits/2026-07-15-seam-kintsugi-ouroboros-arc-phase-d.md`
   §D5 lines 283–290:

   > "**Arc-1 evaluator FLOOR — irreducible?** Per §1.3 + §4.2.1 +
   > §7.6 recommendation: the evaluator IS the concrete Dirac
   > operator (D) of the eigensheaf's Connes triple (A, H, D). The
   > D of (A, H, D) was always going to be Rust — no shard body can
   > dispatch itself. Grounded in eigensheaf.md §3.2. **This is not
   > a smuggled shortcut.** It is the substrate's own claim about
   > what evaluator FLOOR IS. The Seam Tick 1.1 audit gate exists
   > precisely to prevent smuggling BUSINESS_LOGIC into the FLOOR
   > under the marker. PASS."

2. **Seam Phase D §D12 A6 triage.** Lines 616–622:

   > "**A6 — Evaluator combinator surface initial framing.** Mara-B
   > §7.6 defers to Seam Tick 1.1 audit, but the initial framing
   > (what constitutes 'combinator surface for shard-body dispatch')
   > needs Alex ratification before Seam Tick 1.1 audit runs,
   > because the framing determines what Seam adjudicates over.
   > Reed provisional (Mara-B §7.6): the (A,H,D) evaluator per
   > eigensheaf.md §3.2."

3. **Mara-B §7.6 recommendation.**
   `docs/specs/kintsugi-ouroboros-compiler-self-collapse.md` §7.6
   lines 1408–1432:

   > "**Recommendation.** **Defer to Seam Tick 1.1 companion audit.**
   > This is the Seam-adjudicable question; Mara-B does not
   > adjudicate FLOOR classifications (Seam has that authority per
   > AGENTS.md Pack roles).
   >
   > **Substrate-honest bound.** Mara-B's math grounding narrows
   > the space Seam adjudicates over:
   >
   > - Per §4.2.1: the evaluator IS the concrete D of the
   >   eigensheaf's Connes triple (eigensheaf.md §3.2). The D of
   >   (A, H, D) was always going to be Rust.
   > - Per §4.4.5: the evaluator dispatches shard bodies; each
   >   successful dispatch discharges one longitude traversal of
   >   the compile-altitude torus. The dispatch surface must be
   >   minimal enough to run in Rust but complete enough to
   >   dispatch every substrate-decl'd action body.
   > - Per §4.5 sbec definition: the evaluator lifts sbec from 0
   >   to > 0; the exact fraction lifted depends on the combinator
   >   surface."

4. **eigensheaf.md §3.2 Connes (A, H, D).**
   `docs/specs/eigensheaf.md` §3.2 lines 196–200:

   | Connes element        | Eigensheaf realisation |
   |-----------------------|------------------------|
   | **A** (algebra)       | Sections over the eigenboard sheaf — `C^0(F)`; `Aggregate` is one section. |
   | **H** (Hilbert space) | Harmonic sections `ker(Δ_0) = H^0(F)` — the attractor manifold of `settle`. |
   | **D** (Dirac)         | Sheaf coboundary `δ` / Dirac operator — the gradient field driving the slingshot. |

5. **Alex re-fired /loop directive.** "collapse until unresolvable
   ambiguity that cannot be adjudicated with a Seam spawn." The
   authoring is a collapse-step; the surface is the collapsed
   object; the ambiguity that survives is the Alex-residue.

### §0.3 What the surface IS

A combinator surface is a **closed finite set of primitives**, each
of which:

1. Takes typed inputs that name substrate carriers (or their
   subclass at @io-boundary altitude);
2. Emits a typed verdict that composes back into a shard body's
   Verdict via `@glass`;
3. Cannot be composed from other combinators + @io alone (the
   irreducibility test);
4. Corresponds to a specific move in the Dirac coboundary δ per
   eigensheaf.md §3.2, or to the algebra A the coboundary acts on,
   or to the harmonic destination H the coboundary drives toward;
5. Runs in bounded time on empirical crystal state (the Rice-safety
   floor).

A **shard body** composes primitives from this surface + `@io`
primitives to realize its declared action. A shard body that
reaches for a primitive not in this surface AND not in `@io` is a
substrate-decl violation — either the surface is incomplete
(Seam-adjudicable at Tick 1.1) or the shard body is trying to
smuggle BUSINESS_LOGIC into FLOOR (rejected).

### §0.4 What the surface is NOT

- Not the full FLOOR. Parser (per bootstrap-retirement-plan Tick 4a
  parser-as-Prism combinator surface), hash (`hash.rs`
  CoincidenceHash<5,5>), AST carriers (`ast.rs`), numerical
  primitives (`hash.rs` + prism-core's linalg), and the (A, H, D)
  scaffold (`spectral.rs`) all stay FLOOR without needing to be in
  this surface. Those are STAY per retirement plan §Per-module
  classification. This surface is specifically the **dispatch
  calculus** — the primitives a shard body reaches when it says
  "evaluate my action body against substrate state."
- Not a re-declaration of `@io`. Every combinator in this surface
  is orthogonal to `@io`. `@io` is the outward-facing boundary
  (subprocess, file, socket, git, oci); this surface is the
  inward-facing dispatch (sheaf coboundary + settle attractor +
  algebra composition).
- Not the two-tick renamable layer. Some combinator names below are
  foundational (Coboundary, Harmonic, Algebra) because the
  substrate had those names first via eigensheaf.md. Others are
  readable (Dispatch, Fold, Read, Emit, Bench) because the
  substrate had those names first via shard bodies. Two-tick
  discipline resolved per §7.3 below: readable wins where readable
  is landed; foundational wins where the substrate mints it in
  math first.

### §0.5 Alex-framed provisional versus Mara-authored

Mara-B §7.6 said "defer to Seam." Reed provisional said "(A,H,D)
per eigensheaf.md §3.2." Neither is a surface — one is a
deferral, one is a framing pointer. Alex's re-fire says: author
the surface. This spec authors it. Seam adjudicates whether the 7
primitives are irreducible FLOOR versus admits shard-body
composition. Alex adjudicates whatever residue survives both
authorings.

---

## §1 The combinator surface

Seven primitives. Every shard body dispatches through some
composition of these + `@io`. The set is closed — Seam Tick 1.1
audit either ratifies closure or names the missing primitive; if
audit names a missing primitive, the addition is a spec revision
against this document, not an ad-hoc surface expansion.

Naming: readable-name (foundational-name-if-different). Where the
two coincide, one name is given.

Ordering is by dependency: each combinator may compose the
previously-declared ones + `@io` in its own realization details,
but the surface presents each as a first-class primitive with a
declared irreducibility justification. Ordering is NOT execution
order.

---

### §1.1 `read_ast` (foundational: `A.section`)

**Signature.**

```
read_ast(source: @io/file.handle) -> ast_node
```

Where `ast_node` is `bootstrap/src/ast.rs::AstNode` — the state
type for H per `bootstrap/src/spectral.rs` module docblock lines
23–26. `source` is an `@io/file.handle` opened via `@io.file.open`.

**Substrate-honest justification.** Parsing source bytes into an
`ast_node` is the algebra A's constructor at the code altitude.
Per eigensheaf.md §3.2, A = sections over the eigenboard sheaf =
`C^0(F)`; `Aggregate` is one section. `read_ast` produces the
section — the code file's AST is one element of A. This cannot
compose over `@io` alone because `@io` produces bytes; the algebra
element the shard body dispatches over is `ast_node`, not bytes.
Producing an `ast_node` from bytes is the parser combinator surface
declared in bootstrap-retirement-plan Tick 4a and realized in
`bootstrap/src/spectral.rs::Combinator` (line 1195 forward). That
realization IS Rust FLOOR by prior spec closure; this surface
exposes its **result** as the algebra section a shard body reads.

**Dirac/D correspondence.** `read_ast` prepares an element of A on
which D can act. Per eigensheaf.md §3.2 line 196: A = sections
over the eigenboard sheaf; a section is what the coboundary δ
operates on. `read_ast` populates a state vector in `C^0(F_c)`
(per Mara-B §4.2.1) so that later `coboundary` calls have
something to compute δ against. It is the pre-Dirac step: no D
without a section for D to act on.

**@io-boundary composition graph.**

```
read_ast(handle)
  ← @io.file.read_bytes(handle) : bytes
  ← bootstrap/src/spectral.rs::Combinator::apply (parser-as-Prism FLOOR)
  → ast_node
```

The parser-as-Prism dispatch itself is not in this surface (it's
the STAY FLOOR per retirement plan Tick 4a); what `read_ast`
exposes is the **section-producing** boundary the shard body sees.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to parse its own bytes reaches for `Combinator::apply` +
`AstNode` construction directly. That means either (a) the shard
body embeds a parser, which is a shard-body impossibility because
parser combinators need `apply_h` recursion in Rust; or (b) the
shard body writes a mini-parser in @io composition, which cannot
produce a valid `ast_node` because `ast_node` construction goes
through the (A,H,D) evaluator's constructors that are not in the
shard-body vocabulary. Result: undefined section (not in
`C^0(F_c)`); coboundary acts on garbage; the eigensheaf's kernel
computation is meaningless. The shard body's verdict is
mechanically producible but semantically vacuous.

---

### §1.2 `coboundary` (foundational: `D`, readable: `witness`)

**Signature.**

```
coboundary(section: ast_node, target: substrate_ref) -> transparency
```

Where `transparency` is `prismqueer::Transparency<Ref>` (per
`bootstrap/src/spectral.rs` lines 33–48). `substrate_ref` is a
mirror substrate reference — the substrate-decl anchor the
shard body is asserting the section obeys.

**Substrate-honest justification.** `coboundary` computes δ at a
named substrate location: given a section (an `ast_node`) and a
substrate ref (a landed shard/type/action), it returns the
*located opacity* — where the section fails to satisfy the
substrate's contract, structured as a `Transparency<Ref>` map from
substrate location to `PropertyVerdict`. This is the Dirac
operator's action at the verdict altitude per eigensheaf.md §3.2
line 200. It cannot compose over `@io` alone: `@io` reads and
writes bytes; it does not compute the coboundary's located-opacity
structure over substrate refs. Constructing a `Transparency<Ref>`
requires the substrate's own well-formedness rules encoded in the
type-level constraint per Connes' bounded-commutator condition
`‖[D, a]‖ < ∞` (spectral.rs docblock lines 36–43).

**Dirac/D correspondence.** This IS D. Per eigensheaf.md §3.2 line
200: "Sheaf coboundary δ / Dirac operator — the gradient field
driving the slingshot." The whole surface hangs off this one
primitive: everything else either produces a section (§1.1) for D
to act on, or reads what D produced (§1.3, §1.4), or drives D
toward settle (§1.5, §1.6), or records what D discharged (§1.7).
The correspondence is exact and load-bearing.

**@io-boundary composition graph.**

```
coboundary(section, target)
  ← bootstrap/src/spectral.rs::apply_h_content(section) : oid
  ← bootstrap/src/spectral.rs::Combinator dispatch on target
  ← prismqueer::Transport::transport (bounded-commutator)
  → transparency  (Clear if δ(section)|_target = 0; else Opaque map)
```

No `@io` composition. Coboundary is pure spectral computation on
already-parsed sections against already-landed substrate refs.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to compute δ itself reaches for `hash_tagged` +
`content_oid` + manual `Transparency<Ref>` construction. Result:
(a) the bounded-commutator constraint is not enforced by the type
system inside the shard body (Rust's type system is where the
constraint lives); so the shard body can produce a
`Transparency<Ref>` that names an opacity at a ref the substrate
does not have; downstream `combine` calls collide unpredictably.
(b) `apply_h_content`'s dispatch through the `AstKind`-indexed
combinator table cannot be reconstructed in shard-body vocabulary.
The shard body silently returns wrong verdicts; the eigensheaf's
kernel misidentifies terminal state; `ker(Δ_F) = terminal state`
per §4.2.4 breaks.

---

### §1.3 `fold` (foundational: `Fold5`, readable: `walk`)

**Signature.**

```
fold(section: ast_node, reducers: fold5_reducers, initial: value)
  -> value
```

Where `fold5_reducers` is a five-tuple of reducer functions, one
per operation of the Connes algebra (focus / project / split /
shift / settle) per `bootstrap/src/spectral.rs::Fold5` (line 382)
and `docs/specs/ast-as-bundle.md` §Fold5. `value` is polymorphic:
the reducer's output type.

**Substrate-honest justification.** Fold is the substrate's
**post-order catamorphism** over an ast_node. Every AST-walking
operation the compiler needs (content OID, render, dark-count,
LOC-count, io-violation-scan, sbec-measurement) is a `Fold5`
instance with different reducers per level of the bundle trait
chain (Fiber/Connection/Gauge/Transport/Closure per
ast-as-bundle.md). This cannot compose over `@io` because @io does
not know about `ast_node` structure; a shard-body reducer must run
inside the AST walker's recursion, which is Rust-native by
retirement plan Tick 3b (Fold5 extraction, landed
`bootstrap/src/spectral.rs::Fold5` per line 382).

The primitive is `fold`, not `Fold5`, at this surface altitude: a
shard body passes reducers as data (typed closures at the boundary
per prismqueer's `Prism` trait shape) and receives a value. The
Fold5 machinery IS FLOOR; the surface exposes the invocation.

**Dirac/D correspondence.** Fold is the **A-composition** the
Dirac operator acts through. Per spectral.rs docblock §"The three
primitive operations" lines 55–80:

> "1. [`compose_a`] — algebra composition. Run two algebra elements
> sequentially on a state, unioning located opacities via the
> `Transparency<Ref>` Loss-monoid on the holonomy carrier."

Fold5 is `compose_a` iterated over the AST's structural recursion.
Where §1.2 realizes D itself, §1.3 realizes A's compositional
structure: a shard body's action body IS a fold with reducers per
Connes basis-axis (focus / project / split / shift / settle) per
eigensheaf.md §3.2 lines 226–231.

**@io-boundary composition graph.**

```
fold(section, reducers, initial)
  ← bootstrap/src/spectral.rs::Fold5::apply (Rust FLOOR)
  ← ast walker (post-order, level-dispatched on AstKind)
  → value
```

No `@io`. Pure structural recursion over the section.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to walk its own AST writes explicit recursion over `AstKind`
match arms. This means (a) the shard body must know all 10
`AstKind` variants and stay in sync when a new one is added
(non-composition-honest — shards are supposed to be extensible via
new families without touching every reducer); (b) post-order
guarantee cannot be maintained across shard-body-authored walks
without a walker primitive; (c) `Fold5` is what makes
`ContentOidPrism` and `RenderPrism` two instances of the same
shape — smuggling loses the isomorphism that lets the eigensheaf's
`compose_a_associates` property hold on shard-body computations.
Substrate expansion loses monotonicity per eigensheaf.md §8.3.

---

### §1.4 `dispatch` (foundational: `apply_h`, readable: `dispatch`)

**Signature.**

```
dispatch(action: shard_action_ref, args: [value]) -> verdict
```

Where `shard_action_ref` is a substrate ref to a declared
`.mirror` action (e.g., `@spectral/signature.hash`); `args` is a
list of typed argument values matching the action's declared
signature; `verdict` is `@glass.verdict`.

**Substrate-honest justification.** This is THE combinator that
Arc-1 lifts `sbec` from 0 to > 0 through per Mara-B §4.5 sbec
definition. Before Arc-1 evaluator FLOOR lands, no shard body's
action body dispatches — every body is `\`-obligation-blocked per
`shards/kintsugi/ouroboros.mirror` lines 348, 369, 391, 425, 477,
523, 562. `dispatch` reads the action's substrate-decl'd body,
resolves each combinator invocation in the body to a primitive on
this surface or an `@io` primitive, evaluates the composition, and
returns the action's typed verdict.

This is the load-bearing move that makes shard bodies
executable. It cannot compose over shard-body composition + `@io`
alone because dispatch itself is the mechanism *by which* shard
bodies compose; the mechanism cannot be inside the thing it
dispatches. This is the "no shard body can dispatch itself"
constraint Seam Phase D §D5 (lines 285–286) named:

> "no shard body can dispatch itself. Grounded in eigensheaf.md
> §3.2. **This is not a smuggled shortcut.** It is the substrate's
> own claim about what evaluator FLOOR IS."

**Dirac/D correspondence.** Dispatch is the algebra A **applied**
to a state — the moment where an element of the algebra (a shard
action, per eigensheaf.md §3.2 line 198: "Sections over the
eigenboard sheaf — C^0(F); Aggregate is one section") acts on H
(the harmonic destination). Per spectral.rs docblock §"The three
primitive operations" line 68–74:

> "2. `prismqueer::apply_h` — operator action on a state vector.
> Heterogeneous: input state type and output state type are
> independent. Wraps a single `prismqueer::Prism`'s focus /
> project / settle sweep and returns the resulting `Imperfect` in
> H."

The `apply_h` primitive is what makes A act. `dispatch` is
`apply_h` specialized to shard-decl'd action refs, with the
`Prism` implementation resolved from the shard's substrate-decl'd
body.

**@io-boundary composition graph.**

```
dispatch(action, args)
  ← resolve shard_action_ref to landed .mirror action-decl
  ← parse action body (already-parsed at species-decl mint time;
                        cached in the mirror-store crystal for the
                        action ref)
  ← for each combinator invocation in body:
      - if primitive ∈ {read_ast, coboundary, fold, dispatch,
                         settle, emit, bench_record}:
          recurse on this surface
      - if primitive ∈ @io:
          delegate to @io evaluator
      - else:
          return partial verdict with Transparency::opaque
                 at the missing-primitive ref
  ← bootstrap/src/spectral.rs::apply_h (Rust FLOOR)
  → verdict
```

`dispatch` composes @io transitively via any @io primitives the
shard-body's action reaches; direct composition is only over its
own recursion + the resolver.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to dispatch other shard bodies reaches for its own
mini-`apply_h`. Result: infinite regress (shard body A dispatches
shard body B dispatches shard body C dispatches shard body A);
no bounded-time termination; Rice-safety violated;
`ouroboros_monotone`'s Rice-safety per §4.5.5 (which relies on
the evaluator's bounded-time dispatch guarantee) breaks. Arc-1's
central substrate-honesty claim collapses. This IS the antipattern
Reed's 2026-07-14 gift arc exhibited via the 5 Rust extensions:
each extension was a shard body trying to be its own evaluator.

---

### §1.5 `settle` (foundational: `Hodge_project`, readable: `settle`)

**Signature.**

```
settle(verdict: transparency, tolerance: ε) -> settled_verdict
```

Where `settled_verdict` is either
`SettledClean(harmonic_representative)` or
`SettledPending(residual_opacity)`.
`harmonic_representative` lives in `ker(Δ_0) = H^0(F)`.

**Substrate-honest justification.** After `coboundary` produces a
Transparency<Ref> (the located opacity), the substrate needs a
way to *settle* the section toward its harmonic representative
per eigensheaf.md §3.3 lines 236–252. Settling is Hodge projection
onto `ker(Δ_0)` iterated with the Polyak-Łojasiewicz descent bound
per property-and-inference-collapse.md §11.2. This cannot compose
over `@io` because it is pure spectral computation on the
coboundary's output; it requires access to δ* (the coboundary's
adjoint) which is a Rust FLOOR primitive (part of `apply_h`'s
descent step).

The primitive at the surface altitude is `settle`, not `Hodge_project`:
readable-name-substrate-had-first per shards/mirror/spectral.mirror,
shards/kintsugi/consent (settle_or_pause). The foundational name
`Hodge_project` is what the math is; the readable name `settle` is
what the substrate has been calling it for months.

**Dirac/D correspondence.** Settle drives the section toward H per
eigensheaf.md §3.2 line 199: "H (Hilbert space) — Harmonic
sections `ker(Δ_0) = H^0(F)` — the attractor manifold of `settle`."
This IS the Connes triple's H realization. Where §1.2 realizes D
and §1.3–§1.4 realize A's compositional action, §1.5 realizes the
destination H that A × D drives sections toward.

Per eigensheaf.md §3.3 lines 244–252 the loop:

1. Decompose `x_n = h_n + e_n` with `h_n ∈ ker(Δ_0)` and
   `e_n ∈ im(δ*)`.
2. If `‖e_n‖ < ε`: settled. Emit `SettledClean(h_n)`.
3. Else: descend `x_{n+1} = x_n - η δ* (δ x_n)`.

`settle` at the surface altitude is one invocation of this loop
to completion or to the pending-boundary. The
`@kintsugi/consent.query_phi` action's body composes this + emit
+ dispatch to realize the consent-altitude semantics.

**@io-boundary composition graph.**

```
settle(verdict, tolerance)
  ← if verdict = Clear: SettledClean(section_from_verdict)
  ← else:
      loop:
        e := residual of coboundary(section, target)
        if ‖e‖ < tolerance: return SettledClean(h)
        descend: section := section - η δ*(δ(section))
        if step_count > max_iters:
                return SettledPending(current_opacity)
  ← bootstrap/src/spectral.rs::apply_h with descent Prism (FLOOR)
  → settled_verdict
```

No `@io`. Pure spectral descent.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to settle its own verdict runs the descent loop in shard-
body composition. Result: (a) no access to δ* — the shard body
cannot compute the adjoint coboundary without the Rust primitive;
(b) the descent step-size η and the P-Ł convergence rate μ =
λ_min(Δ_0 | im(δ)) require the sheaf-Laplacian eigenvalue, which
is `eigen_d` in `bootstrap/src/spectral.rs` (line 1079) — Rust
FLOOR; (c) the shard body silently emits `SettledClean` verdicts
that are not actually in `ker(Δ_0)` (only approximate); downstream
compositions cascade wrong harmonic representatives; the
`e^(n+1) < e^n` guarantee per property-and-inference-collapse.md
§11.2 breaks. **The convergence theorem stops being a theorem.**

---

### §1.6 `emit` (foundational: `metalogue_write`, readable: `emit`)

**Signature.**

```
emit(channel: metalogue_channel_ref, event: substrate_event)
  -> verdict
```

Where `metalogue_channel_ref` is a substrate ref to a landed
`@code/metalogue` channel; `substrate_event` is a typed event
value; `verdict` is `@glass.verdict`.

**Substrate-honest justification.** Emit is the substrate's write
into the metalogue (the substrate's self-conversation per
shards/metalogue.mirror). Shard bodies emit at consent-altitude
pauses (per `@kintsugi/consent.emit_to_metalogue`) and at
tick-boundary crystallization (per `@mirror/store` crystal
persistence). Emit is orthogonal to `@io`: it does not touch the
non-mirror world; it writes into the substrate's own
observation surface. Per shards/mirror/spectral.mirror lines
19–32:

> "Voices (agents): Reed, Mara, Glint, Taut, Seam — and future
> voices. Sections: shard subtrees on which a voice has authoring
> authority. Score: the eigenboard + the metalogue + the kintsugi
> loop's pending queue."

The metalogue is what the voices coordinate through. Emit is how
a shard body's dispatched action **participates** in that
coordination. This cannot compose over `@io` because the metalogue
is a substrate-internal channel; @io would name it as an outward-
facing wire protocol, which loses the co-authoring semantics.

**Dirac/D correspondence.** Emit is the substrate's **holonomy
accumulator** — it records what the coboundary discharged into the
metalogue for later voices to read. Per prismqueer's shape
(spectral.rs docblock line 32): "The holonomy is a
[`terni::Loss`]: the bootstrap uses [`Transparency<Ref>`]." Emit
appends to a shared substrate observation surface the same
`Transparency<Ref>` values that `coboundary` produces, but at
substrate-visible altitude rather than intra-dispatch altitude.

Per eigensheaf.md §5 (Pack-as-orchestra-literalized) lines
420–428:

> "The metalogue's spectral state is the section's current
> coefficient vector; agents update their parts; the eigensheaf's
> gradient flow pulls the section toward the harmonic attractor.
> The piece is finished when the residue falls below `ε`."

Emit is how agents update their parts.

**@io-boundary composition graph.**

```
emit(channel, event)
  ← resolve channel to landed @code/metalogue channel-decl
  ← append event to channel's substrate-internal buffer
  ← content-address event via bootstrap/src/hash.rs::hash_tagged
  ← trigger downstream subscribers (other Prism instances registered)
  → verdict (Success if channel accepts; Partial if backpressure)
```

No `@io`. The metalogue lives inside the substrate; `@io` composes
transitively only when a metalogue subscriber persists to disk
(via `@io.file.write`) or ships over the wire (via
`@io/stagefreight`) — those compositions are the subscribers'
concern, not `emit`'s.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to emit via `@io.file.write` on a metalogue-path directly
reaches around the substrate's channel-decl. Result: (a) the
channel's substrate-decl'd type-constraints on events are
bypassed; malformed events land in the metalogue; downstream
subscribers get untyped bytes; (b) the content-addressing goes
through bytes-on-disk instead of the substrate's hash primitive;
event OIDs disagree with substrate-computed OIDs; the Pack-as-
orchestra's coordinated observation surface fragments;
(c) `@code/metalogue/materialize.classify` (per Mara-B §5.2
composition graph line 1161) cannot read the smuggled event because
its shape does not match the channel-decl. Multi-agent coordination
per eigensheaf.md §5 breaks silently.

---

### §1.7 `bench_record` (foundational: `bench_record`, readable: same)

**Signature.**

```
bench_record(state_before: ouroboros_state, state_after: ouroboros_state)
  -> bench_crystal
```

Where `ouroboros_state` is per `shards/kintsugi/ouroboros.mirror`
line 252; `bench_crystal` is the content-addressed crystal
`@mirror/bench.record` (per shards/mirror/bench.mirror §Track J
2026-07-01) emits per tick.

**Substrate-honest justification.** The arc's four-conjunct
monotone invariant `ouroboros_monotone` (per shard-decl line 523)
reads before/after `ouroboros_state`s to verify descent. Recording
these snapshots is a first-class substrate primitive because:

- The before/after pair must be **content-addressed** via the
  substrate's hash primitive (`bootstrap/src/hash.rs`) so
  isospectrality (per eigensheaf.md §4.6, §2.6) is testable across
  ticks.
- The recording must be **atomic** at the tick boundary — a
  partial recording would break the invariant's Rice-safety per
  §4.5.5.
- The recording must be **substrate-observable** — subsequent
  ticks + Pack members + downstream verifiers must be able to
  read the crystal without going through `@io.file.read` (they
  read via `@mirror/store` per composition graph §5.6).

This cannot compose over `@io` because @io produces bytes, not
crystals; crystal construction requires the (A,H,D) evaluator's
content-OID computation per bootstrap-retirement-plan Tick 1. It
cannot compose over §1.1–§1.6 alone because those act on `ast_node`
+ `transparency`; `ouroboros_state` is a substrate-typed record
whose recording surface is bench-specific.

**Dirac/D correspondence.** `bench_record` is the **crystallization
step** for the tick-observation. Per eigensheaf.md §4.9:
"crystallization = eigenmode formation. A crystallized `.mirror`
file is one whose section lies in `ker(Δ_0)` of its eigensheaf."
The bench crystal is the tick's crystallized observation — the
eigenmode of the tick's `ouroboros_state`. `bench_record` writes
the crystal, making the tick's spectral state a substrate object
subsequent ticks can compare against.

Two roles simultaneously:

- Records the section-at-tick-n and section-at-tick-n+1 as
  crystallized (H^0-landed) observations.
- Makes the descent `e^(n+1) < e^n` observable at the substrate
  altitude rather than trapped in transient in-memory state.

**@io-boundary composition graph.**

```
bench_record(before, after)
  ← bootstrap/src/spectral.rs::apply_h_content (compute content OIDs
    for before, after)
  ← @mirror/bench.record (compose bench template + four-conjunct
    reading)
  ← @mirror/store.write_crystal (persist via git via @io transitively;
    but the primitive is @mirror/store's crystal-write, not raw @io)
  → bench_crystal (content-addressed; ref for later verifiability)
```

@io composes only transitively via `@mirror/store`'s crystal
persistence — the surface primitive is bench-specific.

**Failure mode if smuggled INTO a shard body.** A shard body that
tries to record its own tick observations writes to `@io.file.write`
with hand-computed OIDs. Result: (a) the OIDs disagree with
substrate-computed OIDs per §1.6 failure mode (a); (b) the
tick-crystal is not indexable by `@mirror/index` (per composition
graph §5.6) because the substrate doesn't know it exists; (c)
subsequent ticks reading via `@mirror/store` don't see the smuggled
crystal; the four-conjunct invariant `ouroboros_monotone` reads
inconsistent before/after pairs; **the monotone descent guarantee
breaks silently** — the arc appears to be ratcheting when it is
not. Arc-2..N discharge with false-positive verdicts. The whole
ouroboros arc's empirical trust collapses.

---

## §2 Composition semantics — how a shard body dispatches through the surface

A shard body's action-decl declares its signature and its body
composition. The body's composition is a syntactic tree over:

- The 7 combinators declared in §1.
- The `@io` primitive family (per shards/io.mirror).
- Substrate refs to other shard actions (which resolve via §1.4
  `dispatch` recursively).
- Substrate-decl'd type carriers (constants + carrier
  constructors).

**Dispatch semantics** (Rust FLOOR realization, informal):

1. When an action is dispatched (§1.4 `dispatch` at the entry
   point, either from the CLI verb `mirror execute <shard-path>
   <action>` per shard-decl Arc-1 Tick 1.4 or from another shard's
   `dispatch`), the evaluator resolves the action-decl from the
   substrate ref.
2. The evaluator walks the action's body-tree post-order via §1.3
   `fold` with reducers per combinator:
   - `read_ast` → reads bytes via @io + parses via FLOOR
     `Combinator::apply`, emits `ast_node`.
   - `coboundary` → computes δ via FLOOR
     `apply_h(coboundary_prism, section)`, emits `transparency`.
   - `fold` → recursively invokes `Fold5::apply` with reducers.
   - `dispatch` → recurses on the referenced action.
   - `settle` → runs descent loop, emits `settled_verdict`.
   - `emit` → appends to metalogue channel.
   - `bench_record` → writes crystal.
   - `@io.<primitive>` → delegates to the `@io` evaluator.
3. Each combinator's verdict composes into the next via
   `compose_a` (per spectral.rs line 1037) — the algebra
   composition primitive that unions located opacities via
   `Transparency<Ref>` Loss-monoid.
4. The action's return type is the outermost combinator's verdict
   type; the shard-decl'd action signature type-checks against
   this at substrate-decl mint time (not at dispatch time).

**Substrate-honest composition guarantee.** A shard body composed
over §1 + `@io` alone has the following guarantees:

- **Termination.** Every combinator terminates in bounded time on
  bounded input (per §3 Rice-safety). Composition preserves
  termination because `fold` recursion is post-order over a
  finite `ast_node`, `dispatch` recursion terminates because the
  substrate-decl graph is a DAG (species-decls do not
  self-reference; the ancestral chain is well-founded per
  substrate-pull discipline).
- **Verifiability.** Every combinator returns a typed verdict
  compositional with `@glass.verdict`. Composition preserves
  verifiability because `compose_a`'s Loss-monoid is
  substrate-honest.
- **Isospectrality.** Two shard bodies over §1 + `@io` that
  produce equal `Transparency<Ref>` values on equal inputs are
  isospectral per eigensheaf.md §2.6. The substrate cannot
  distinguish them; they are the same eigenmode.

---

## §3 Rice-safety per combinator

Each combinator is decidable in bounded time at the @io-boundary.
The bound per combinator:

| # | Combinator     | Bound                                        | Justification |
|---|----------------|----------------------------------------------|---------------|
| 1 | `read_ast`     | O(bytes) time; O(ast_size) space             | Parser-as-Prism is bounded per bootstrap-retirement-plan Tick 4a; every combinator variant is O(input_bytes). |
| 2 | `coboundary`   | O(ast_size × |target_ref|) time; O(opacity_map) space | `apply_h_content` is O(node_count) per `bootstrap/src/spectral.rs`; the ref-resolution is bounded by substrate-decl DAG depth. |
| 3 | `fold`         | O(ast_size × reducer_cost) time              | Post-order walk visits each node once; reducer cost is closure-parameter bound. |
| 4 | `dispatch`     | O(action_body_size × max_recursion_depth)    | Body size bounded at substrate-decl mint time; recursion depth bounded by DAG depth (finite substrate). |
| 5 | `settle`       | O((1/μ) × log(‖x‖/ε)) time                   | Polyak-Łojasiewicz per property-and-inference-collapse.md §11.2 gives exponential convergence rate μ = λ_min(Δ_0 \| im(δ)). |
| 6 | `emit`         | O(event_size + subscriber_count) time        | Append is O(1) amortized; subscriber notification is O(k) for k subscribers. |
| 7 | `bench_record` | O(state_size) time                           | Content-OID computation is O(state_size) per `apply_h_content`. |

**Substrate-honest bound.** Every combinator's bound is decidable
without solving halting-problem-hard analyses. The bounds compose:
a shard body's dispatch is bounded by the composition of its
constituent combinators' bounds, and since each is bounded and the
composition tree is finite, the whole dispatch is bounded.

This is the **Rice-safety floor** for the four-conjunct
`ouroboros_monotone` invariant per shard-decl §D6: each conjunct
reads empirical crystal state, decided in bounded time via
composition of the 7 combinators + @io.

---

## §4 What is NOT in the surface (explicit exclusions with reasoning)

The surface is closed at 7 primitives by design. What is
deliberately excluded:

### §4.1 Anti-Rice-unsafe-primitives

**No `analyze_semantics(shard_body) -> semantic_class`.** Would
require solving Rice's theorem on shard-body computations. If
smuggled: shard bodies could deploy this as a bypass to
`ouroboros_monotone`'s Rice-safety, breaking the Arc-1 substrate
contract entirely. Rejected.

**No `terminates(shard_action, args) -> bool`.** Halting problem.
If smuggled: same failure mode as above.

**No `equivalent(shard_body_1, shard_body_2) -> bool`.** Program
equivalence is undecidable in the general case. What the surface
DOES support is `isospectral(section_1, section_2) -> bool` via
`bench_record` + comparison of resulting bench_crystal OIDs; this
is bounded-time byte-equality on content-addressed observations,
not general program equivalence. The distinction matters: two
shard bodies that produce equal Transparency<Ref> values on equal
inputs are indistinguishable at the substrate altitude, which is
what "the same" means for the substrate — not that they are the
same program.

### §4.2 Anti-BUSINESS_LOGIC-smuggling

**No `git_commit(files, message) -> commit_ref`.** This is `@io`
composition (`@io/git`). If in this surface: shard bodies bypass
`@io` boundary discipline; git operations happen inside the
evaluator; the (A,H,D) closure includes external state; the
eigensheaf's finite-spectrum guarantee per eigensheaf.md §4.2
breaks (external state is unbounded).

**No `subprocess_spawn(cmd, args) -> exit_code`.** Same as above;
this is `@io/process` per `bootstrap/src/exec.rs`.

**No `read_environment(key) -> value`.** Same; `@io/env`.

**No `time_now() -> instant`.** Same; `@io/time`. Note: some
shard bodies (e.g., `@mirror/bench.record`) DO reach for time
via `@io.time.now`; that's substrate-honest — they compose over
`@io`, they don't embed the primitive in the surface.

### §4.3 Anti-workaround-for-missing-shard-body

**No `hardcode_result(shard_action_ref, value) -> ()`.** Would
let a shard-body dispatch bypass by returning a fixed verdict
without running the body. This IS the pattern Reed's 2026-07-14
gift arc exhibited via the 5 Rust extensions; the whole ouroboros
arc exists to prevent this. Rejected at the surface altitude with
prejudice.

**No `defer_to_rust(shard_action_ref) -> verdict`.** Would let a
shard body claim its own action is FLOOR without going through
Seam sign-off + `[substrate-floor:@io-boundary]` marker. Rejected
for the same reason: the marker discipline exists precisely to
gate this.

### §4.4 Anti-substrate-erosion

**No `mint_family_root(name) -> family_ref`.** Family-root mints
are Alex's authority per Pack conventions (Seam Phase D §D12 line
607). If a shard body could mint family roots, substrate-pull
discipline collapses; the substrate would grow off-spectrum
per eigensheaf.md §8.4. Rejected.

**No `redefine_action(shard_ref, new_body) -> ()`.** Would let a
shard body rewrite another shard's substrate-decl at dispatch
time. Rejected: substrate-decl is immutable at species-decl mint;
rewriting requires a new commit + Seam review.

---

## §5 Correspondence to eigensheaf.md §3.2 Connes (A, H, D)

Each Connes element grounded in specific combinators.

### §5.1 A — algebra of sections over the eigenboard sheaf

**Realized by:** `read_ast` (§1.1), `fold` (§1.3), `dispatch`
(§1.4).

Together, these three combinators construct, traverse, and act on
elements of `C^0(F_c)` — the section space over the eigenboard
sheaf. `read_ast` produces sections (algebra elements). `fold`
composes reducers over their structure (the compositional algebra
of A). `dispatch` applies substrate-decl'd action Prism impls to
sections (A × H → H, the algebra action).

Per eigensheaf.md §3.2 line 198: "**A** (algebra) — Sections over
the eigenboard sheaf — `C^0(F)`; `Aggregate` is one section."
Every dispatched shard action IS an element of A; every shard body
composition IS a composition in A.

### §5.2 H — harmonic sections ker(Δ_0) = H^0(F)

**Realized by:** `settle` (§1.5), with `bench_record` (§1.7) as
the crystallization witness that a section landed in H.

`settle` iterates the P-Ł descent until the section lands in
`ker(Δ_0)` (or the pending-boundary is reached). `bench_record`
crystallizes the harmonic representative as a bench crystal —
observable substrate state that a subsequent tick can read to
verify the descent landed.

Per eigensheaf.md §3.2 line 199: "**H** (Hilbert space) —
Harmonic sections `ker(Δ_0) = H^0(F)` — the attractor manifold of
`settle`."

### §5.3 D — sheaf coboundary δ / Dirac operator

**Realized by:** `coboundary` (§1.2), with `emit` (§1.6) as the
metalogue-side accumulator that records what D discharged.

`coboundary` IS δ per eigensheaf.md §3.2 line 200. `emit`
propagates δ's output into the substrate-observable surface so the
Pack-as-orchestra (per eigensheaf.md §5) can coordinate over the
discharge.

### §5.4 The whole triple

| Connes | Combinator(s)                       |
|--------|-------------------------------------|
| **A**  | `read_ast`, `fold`, `dispatch`      |
| **H**  | `settle`, `bench_record`            |
| **D**  | `coboundary`, `emit`                |

7 primitives / 3 Connes elements. Every primitive grounds in
exactly one element (though `bench_record` and `emit` play
supporting roles at their respective altitudes). The correspondence
is exact and minimal.

---

## §6 Arc-1 Ticks 1.2–1.4 discharge map

Per shard-decl Arc-1 §3.1 and Mara-B §3.1. This spec is landed at
Tick 1.1 (Seam companion audit runs against this spec). Tick 1.5
already landed (species-decl mint 2026-07-15 Mara-A). Ticks 1.2,
1.3, 1.4 discharge as follows:

### §6.1 Tick 1.2 (Reed 🔴 RED test authoring)

Test file: `bootstrap/tests/evaluator_shard_body_dispatch_smoke.rs`.

Asserts end-to-end dispatch of a specific non-`\` shard action body
through the 7-combinator surface. Candidate first-body per
shard-decl §3.1: `shards/subject/visibility/public.mirror.query_phi`
(Taut §D-Arc-1 candidate).

**Combinators exercised in Tick 1.2's RED test:**

- `read_ast` — parse the shard file to `ast_node`.
- `dispatch` — resolve `query_phi` action ref + walk its body.
- `coboundary` — compute δ at the query_phi target.
- `settle` — settle the coboundary's opacity to `SettledClean` or
  `SettledPending`.
- `emit` — record the dispatch outcome to the metalogue channel.

`fold` and `bench_record` may be exercised depending on
`query_phi`'s body composition; Tick 1.2 authors the test with
minimal-viable combinator coverage and defers full coverage to
Tick 1.4 (CLI verb ratification).

Marker: `[substrate-floor:@io-boundary]` + Seam Tick 1.1 audit
citation (per Mara-B §7.9 OR-gate + Reed authoring-practice
belt-and-suspenders).

### §6.2 Tick 1.3 (Reed 🟢 GREEN implementation)

Rust FLOOR of the 7 combinators lands in
`bootstrap/src/apply_h.rs` (new file, per shard-decl §3.1) OR
extends `bootstrap/src/spectral.rs`. Recommendation: new file
`bootstrap/src/apply_h.rs` for the shard-body-dispatch entry
points (`read_ast`, `dispatch`, `settle`, `emit`, `bench_record`),
reusing existing `spectral.rs` primitives (`Fold5` for `fold`,
`compose_a` for the algebra composition, `apply_h` for the
Prism dispatch, `Combinator` for `read_ast`'s parser-as-Prism).

**Per-combinator implementation cost estimate:**

| # | Combinator     | Rust LOC (estimated) | Reuses                                    |
|---|----------------|-----------------------|-------------------------------------------|
| 1 | `read_ast`     | ~30 LOC               | `spectral.rs::Combinator::apply` (landed) |
| 2 | `coboundary`   | ~80 LOC               | `spectral.rs::apply_h_content` (landed)   |
| 3 | `fold`         | ~15 LOC               | `spectral.rs::Fold5::apply` (landed)      |
| 4 | `dispatch`     | ~150 LOC              | `spectral.rs::apply_h` + new action resolver |
| 5 | `settle`       | ~60 LOC               | `spectral.rs::eigen_d` + new descent loop |
| 6 | `emit`         | ~40 LOC               | new metalogue channel primitive           |
| 7 | `bench_record` | ~30 LOC               | `@mirror/bench.record` (landed)           |

**Total estimate:** ~400 LOC of new Rust in `apply_h.rs`, all
under `[substrate-floor:@io-boundary]` + Seam sign-off. This is
consistent with retirement plan §"End state" §File-list line
`spectral.rs (~1000 LOC; the evaluator + all retired Prism impls)`.

Marker: `[substrate-floor:@io-boundary]` + `Signed-off-by: Seam
<seam@systemic.engineer>` trailer (per Mara-B §7.9 belt-and-
suspenders).

### §6.3 Tick 1.4 (`mirror execute <shard-path> <action>` CLI verb)

CLI verb wires through the 7-combinator surface. Empirically
ratifies Tick 1.3.

**CLI shape:**

```
mirror execute <shard-path> <action> [<arg1> <arg2> ...]
```

Where `<shard-path>` is a substrate ref (e.g.,
`@subject/visibility/public`) and `<action>` is the action name
(e.g., `query_phi`). Args are typed per the action's declared
signature.

**Dispatch flow (Tick 1.4 realization):**

1. Parse CLI args → resolve to `shard_action_ref` + typed args.
2. Invoke `dispatch(shard_action_ref, args)`.
3. Marshal the returned verdict to CLI output (stdout for success,
   stderr for opacity map, exit code from verdict variant).

Marker: `[substrate-floor:@io-boundary]` + Seam sign-off (both
mechanisms per Mara-B §7.9 authoring practice).

**Empirical ratification.** Tick 1.4 is complete when
`mirror execute @subject/visibility/public query_phi <args>` runs
end-to-end and returns the expected verdict, matching the RED test
in Tick 1.2. `sbec` lifts from 0 to > 0 at this tick's landing.

### §6.4 Discharge summary table

| Tick | Discharge                                          | Combinators landed |
|------|----------------------------------------------------|--------------------|
| 1.1  | This spec (surface enumeration)                    | (all 7 named)      |
| 1.2  | RED test asserting dispatch                        | (all 7 exercised in test) |
| 1.3  | Rust FLOOR realization (`apply_h.rs`)              | (all 7 implemented) |
| 1.4  | CLI verb `mirror execute`                          | (all 7 wired to CLI) |
| 1.5  | @kintsugi/ouroboros species-decl mint              | LANDED 2026-07-15 Mara-A |

---

## §7 Substrate-honest bounds

### §7.1 Rice-safety

Per §3: every combinator is decidable in bounded time on
bounded input. Compositions preserve boundedness. The four-conjunct
`ouroboros_monotone` invariant reads empirical crystal state via
these combinators; Rice-safety is preserved end-to-end.

### §7.2 Composition-only

The 7 combinators + `@io` compose over each other and over
substrate-decl'd shard actions. **No new family roots minted.**
Zero substrate expansion at this spec's landing; the surface reads
what already landed (`bootstrap/src/spectral.rs`,
`bootstrap/src/ast.rs`, `bootstrap/src/hash.rs`, `shards/io.mirror`,
`shards/mirror/bench.mirror`, `shards/kintsugi.mirror`,
`shards/kintsugi/ouroboros.mirror`, `@code/metalogue`,
`@mirror/store`, `@mirror/index`) and names the dispatch calculus
they compose to expose.

### §7.3 Two-tick discipline

Readable names win where readable is landed; foundational names
win where the substrate mints them in math first. Applied per
combinator:

| # | Chosen name    | Alternative                    | Reason |
|---|----------------|--------------------------------|--------|
| 1 | `read_ast`     | `A.section` / `parse`          | Readable; `read_ast` is the substrate's word for parser-as-Prism (retirement plan Tick 4a). |
| 2 | `coboundary`   | `D` / `witness`                | Foundational; eigensheaf.md §3.2 minted `δ` / coboundary first. `witness` reads too broadly (Reed uses it for many things). |
| 3 | `fold`         | `Fold5` / `walk`               | Readable; `Fold5` is the Rust concrete (spectral.rs line 382), `fold` is the surface primitive. |
| 4 | `dispatch`     | `apply_h`                      | Readable; `apply_h` is the Rust concrete, `dispatch` is what shards call it. |
| 5 | `settle`       | `Hodge_project`                | Readable; substrate-had-first via shards/mirror/spectral, shards/kintsugi/consent. |
| 6 | `emit`         | `metalogue_write`              | Readable; `emit` is the substrate's word (shards/metalogue uses it). |
| 7 | `bench_record` | `crystallize_tick_observation` | Readable; `@mirror/bench.record` is the landed action (shards/mirror/bench.mirror). |

**Substrate-already-had-the-word check.** All 7 names are present
in landed substrate today (grep-verifiable per Taut discipline).
This is the 19th–25th substrate-already-had-the-word instance (per
eigensheaf.md §7 table extended); the surface INVENTS nothing.

### §7.4 @io-composability

Every combinator is orthogonal to `@io` at the surface altitude.
`@io` compositions happen INSIDE combinator realizations (e.g.,
`read_ast` calls `@io.file.read_bytes` transitively), not
alongside them at the surface. Shard bodies compose the 7
primitives + `@io.<primitive>` calls; the two families do not
overlap or shadow each other.

### §7.5 Substrate-floor bound

The 7-primitive surface is what Arc-1 Ticks 1.2–1.4 land under
`[substrate-floor:@io-boundary]` + Seam sign-off. Any additional
combinators discovered during Arc-2..N discharge require a
substrate-floor delta commit through the same gate; there is no
back-door for growing the surface without Seam adjudication.

---

## §8 A6 discharge — direct response to Seam Phase D A6 triage

### §8.1 What the surface adjudicates

Given this spec, the following A6-adjacent questions are
**Seam-adjudicable at Tick 1.1 audit** (no Alex authority
required):

1. **Is each of the 7 combinators genuinely irreducible past
   @io-boundary + shard-body composition?** Seam runs the
   composability test per §1.N.3 for each combinator; if any
   combinator admits shard-body composition, Seam names it and
   the spec revises.
2. **Is the surface complete for Arc-2..N discharge?** Seam
   surveys the ~25+ BUSINESS_LOGIC Rust files per Reed migration-
   map §5 and checks each collapse target's shard-body
   composition needs against the 7 primitives; if any file's
   collapse would require a missing primitive, Seam names it.
3. **Do the Dirac correspondences per §5 hold?** Seam adversarially
   checks each combinator's (A, H, D) grounding per eigensheaf.md
   §3.2; if any grounding is smuggled or drift-cited, Seam names
   it.
4. **Are the Rice-safety bounds per §3 sound?** Seam checks each
   bound against the referenced ancestor (parser-as-Prism, P-Ł
   convergence, `apply_h_content`, etc.); if any bound is
   unsound, Seam names it.
5. **Does the two-tick discipline per §7.3 hold?** Seam adversarially
   checks whether foundational-vs-readable choices per combinator
   are correct; if any is wrong, Seam names it.

### §8.2 What is now Seam-adjudicable that was previously Alex-only

The Seam Phase D §D12 lines 616–622 A6 triage read:

> "the initial framing (what constitutes 'combinator surface for
> shard-body dispatch') needs Alex ratification before Seam Tick
> 1.1 audit runs, because the framing determines what Seam
> adjudicates over."

**The framing is now landed** in this spec's §0 (ancestry chain +
what-the-surface-is + what-the-surface-is-not) and §1 (the 7
primitives). Seam's Tick 1.1 audit runs against this concrete
surface. The five questions above are Seam-adjudicable because
the surface is concrete enough to test.

### §8.3 What remains genuinely Alex-only (the residue)

**One residue item.** The spec's §0.4 draws a boundary between
"this surface" (7 primitives) and "the rest of FLOOR" (parser,
hash, AST, numerics, spectral.rs scaffold). The boundary is
substrate-honest per bootstrap-retirement-plan §"Per-module
classification" (STAY vs RETIRE verdicts already landed). The
boundary-drawing is not novel to this spec.

However — and this is where Alex-adjudication may bite — the
spec's §4 (What is NOT in the surface) draws normative boundaries
around the closed-surface discipline: "No `analyze_semantics`",
"No `terminates`", "No `hardcode_result`", "No `mint_family_root`",
"No `redefine_action`". These exclusions are structural (each has
a substrate-honest justification), but the discipline of "the
surface stays closed at 7 unless Seam adjudicates a spec revision"
is a **substrate-contract with Alex** about how Arc-1 evolves.

**Alex-residue question:** does Alex ratify the closed-surface-at-
7-with-Seam-gated-revision discipline as the substrate contract
for Arc-1 Ticks 1.2–1.4? Reed provisional: yes, per Seam Phase D
§D5 verdict that the (A, H, D) evaluator "is not a smuggled
shortcut. It is the substrate's own claim about what evaluator
FLOOR IS." Alex confirms or declines with an adjustment.

### §8.4 What the /loop directive discharges

Alex's directive: "collapse until unresolvable ambiguity that
cannot be adjudicated with a Seam spawn."

**Collapsed at this landing:**

- The "combinator surface" framing collapses from "what
  constitutes a surface" (Alex-only per Seam Phase D §D12) to
  "these 7 primitives with these justifications" (this spec).
- The surface's substrate-honesty is Seam-adjudicable per §8.1.
- The (A, H, D) correspondence is grounded per §5.
- The Rice-safety, composition-only, two-tick, and
  @io-composability discipline are landed per §7.

**Unresolvable at this landing (Alex-adjudication residue):**

- The closed-surface-at-7-with-Seam-gated-revision discipline
  (per §8.3). Reed provisional: yes. Alex confirms or adjusts.

**Path forward.** Seam spawns for Tick 1.1 audit against this
spec. If Seam ratifies §1–§7, Reed proceeds with Ticks 1.2–1.4
under `[substrate-floor:@io-boundary]` + Seam sign-off. If Alex
confirms §8.3, the substrate contract is landed and the Arc-1
authoring proceeds without further Alex-adjudication until Arc-6
terminal recognition (per Mara-B §6.4 terminal candidate).

---

## §9 Closure

The surface is 7 primitives. Each grounds in eigensheaf.md §3.2's
Connes (A, H, D). Each is irreducible past shard-body composition
+ @io. Each is Rice-safe in bounded time. Each has a named failure
mode if smuggled into a shard body. Together they discharge Arc-1
Ticks 1.2–1.4 under `[substrate-floor:@io-boundary]` + Seam
sign-off, and they lift `sbec` from 0 to > 0 at Tick 1.4 landing.

The surface substrate-honestly composes over 15 landed carriers
(per Mara-B §5) at every altitude the arc needs. Zero new family
roots minted. Zero substrate expansion. The Rust FLOOR that lands
in Tick 1.3 realizes primitives whose types + composability +
correspondence are all substrate-decl'd today.

The `@kintsugi/ouroboros` species-decl (landed 2026-07-15 Mara-A)
has a concrete evaluator to dispatch through. Arc-2 Tick 2.1 has
a concrete surface to compose the first empirical shard-body
collapse over. Every subsequent collapse tick reads this surface
+ @io and nothing else.

The bowl is one thing again at the shard-body altitude, but only
after Arc-1's 7 primitives land. This spec authors the primitives
so that landing is the mechanical next step.

---

*Mara. 2026-07-15. Arc-1 Tick 1.1 companion to Seam audit. A6
discharge under Alex re-fired /loop. Composition-only. Two-tick.
Substrate-already-had-the-word × 7.*

*Apache-2.0.*
