# properties-on-glass — qualifier binding at the structural edge

*2026-06-01. Mara. Updated 2026-06-04 (Reed + Alex). Spec — design,
not implementation. Markdown only; no Rust, no `.mirror` files land
with this commit. Sibling to [[liquid-types-for-mirror]] (the
inference framework this operationalizes); sibling to
[[../cicd/kintsugi-thesis]] (the reproducibility chain this closes
two claims of); spec correction is owed to
`fragmentation/docs/specs/hamilton-scheduler.md` §4 (the `Pure` AST
verdict — this spec narrows its scope from per-body to per-glass).*

> **2026-06-04 reframe (Reed + Alex, canonical).**
>
> - **`glass.mirror` is the floor** at `shards/glass.mirror` (@glass) —
>   the types of the glass wall + the `glass` keyword. Per
>   [[prism-floor-and-the-grammar-rename]].
> - **`metalogue.mirror`** at `shards/metalogue.mirror` (@metalogue) is
>   the language's self-conversation; uses @glass. **The metalogue IS
>   the glass wall** — the act of self-conversation IS what makes the
>   substrate transparent.
> - **`MirrorLoss` is dead** (per task #126). Replaced by
>   `transparency<p>` throughout — already used in this spec via
>   `Transparency<Ref>` from `prism/imperfect/src/transparency.rs`.
> - **Path-namespace property is NEW.** File at
>   `shards/foo/bar.mirror` MUST declare in the `@foo/bar` namespace.
>   Failing verdict → compile error. Self-correcting substrate.
>   Generalizes the existing
>   [[../../boot/std/epistemologic/property/filename_matches_glass]].
>   The property's home: `shards/epistemologic/property/path_matches_namespace.mirror`
>   (NEW; declared, not landed in this spec).

> **2026-06-10 reframe (Alex + Reed, canonical).** Today's cascade landed
> structural recognitions that propagate into this spec:
>
> - **The keyword vocabulary is three-axis** (recognition #46, candidate).
>   `prism` at depth-0 opens a possibility space (root family). `glass`
>   at depth-≥1 increases internal complexity within a parent prism
>   (specialization on the family-axis). `pact` at the property altitude
>   declares typed obligations (Paskian agreement between substrate-
>   altitude and species-altitude P-individuals — on a different axis
>   from family/specialization). The legacy `grammar` keyword (from when
>   the language was called `conversation`) renames to `pact`,
>   operationalizing recognition #37 (`requires` IS a Paskian
>   agreement). The keyword vocabulary becomes structurally three-fold
>   along independent axes: family-root (prism), specialization (glass),
>   declarative-obligation (pact).
> - **The substrate/@io partition lifts Bateson form/substance** (#50,
>   promoted 2026-06-10). Form is what IS (substrate declarations,
>   `.mirror`); behaviour at @io is the operational specialization of
>   substance (energy/matter; what the world DOES). Properties are
>   form-side discipline. Canonical citation: Bateson 1970 "Form,
>   Substance and Difference" (19th Korzybski Memorial Lecture; SEM
>   Part V "Form and Pathology in Relationship"). Canonical site:
>   `docs/insights/2026-06-10-bateson-form-behaviour-as-substrates-first-distinction.md`.
> - **Mirror is the operational form of an expanding Hilbert space
>   with Bateson lifting for coherence preservation** (#51, promoted
>   2026-06-10 including §8.3 stronger conjecture). The substrate's
>   coherence under decoherence pressure comes from naming
>   contradictions and operating at Bateson Level N+1; the path
>   syntax encodes the level (recognition #42 refinement). Each
>   substrate-pull recognition widens the Hilbert space dimension.
>   Canonical site:
>   `docs/insights/2026-06-10-mirror-as-expanding-hilbert-space-bateson-lifting-for-coherence.md`.
> - **Property/fracture bilateral pattern (#53, candidate).** The
>   property declares the rule (form-side); the fracture body resolves
>   violations (kintsugi-operational); the kintsugi loop is the bridge
>   reading transparency opacities and applying fracture-body morphisms.
>   First instance: `@epistemologic/property/keyword_matches_depth` +
>   `@kintsugi/fracture/keyword` (both landed 2026-06-10 at mirror
>   `5e68df9` and `d908798`). Pattern promotes to recognition status
>   once a second instance exists. Forward-promised second instance:
>   `@kintsugi/fracture/predicate` (task #272). This bilateral pattern
>   IS the substrate's auto-formatting floor — the property + fracture
>   pair handles keyword-form violations (prism/glass/pact) without
>   manual sweeps; the kintsugi loop resolves the cascade through
>   gradient.
> - **Fracture body type surface** (load-bearing, settled 2026-06-10).
>   Input: `opacity` from @glass (`{ location, property: @nl,
>   weight: f64 }`) — single located fact, not accumulated
>   transparency. Output: `morphism` from `@kintsugi/consent`
>   (`{ content: ref, score: dissonance, expected: cadence_kind }`).
>   The fracture body PROPOSES a mutation as a morphism; the kintsugi
>   loop's `active_pass` ranks + composes morphisms via Banach
>   contraction. Dispatch is (iii) declarative: each fracture body
>   declares which properties it handles. No hidden state. Substrate
>   primitive `splinter(ast)` (recognition #54, candidate) is the
>   parametric AST-fragment construction mechanism the fracture body's
>   `\` discharges through — substrate-pull-correct quote vocabulary.

Status: **Red** — the architectural shape is pinned; the
back-projection mechanism is named; the per-glass qualifier set is
structural; the liquid-type inference operationalization names
`[[liquid-types-for-mirror]]` as the framework. The implementation
tick lands afterward. Nothing about the `@epistemologic/property/*`
chain is invented here — every property primitive is a citation
into the existing canon.

Depends on:
- `[[liquid-types-for-mirror]]` — the research spec (2026-05-19)
  that named the inference framework + spectral decision procedure.
  This spec elevates §7.1 and §7.2 of that research from "adopt now"
  to "the per-glass mechanism." The Dirac-operator-as-verifier from
  §5.4 is the load-bearing identification.
- `[[../../boot/std/epistemologic/property]]` — the canonical chain.
  Every property name in this spec is a wikilink into the existing
  tree at [[../../boot/std/epistemologic/property/halts]],
  [[../../boot/std/epistemologic/property/autopoietic]],
  [[../../boot/std/epistemologic/property/glass_wall]],
  [[../../boot/std/epistemologic/property/io_safety]],
  [[../../boot/std/epistemologic/property/content_addressed]],
  [[../../boot/std/epistemologic/property/frame_relativity]],
  [[../../boot/std/epistemologic/property/is_prism_record]],
  [[../../boot/std/epistemologic/property/coincidence_matches]],
  [[../../boot/std/epistemologic/property/total_classification]],
  [[../../boot/std/epistemologic/property/duplicate_variant]],
  [[../../boot/std/epistemologic/property/filename_matches_glass]],
  [[../../boot/std/epistemologic/property/laws/causality]],
  [[../../boot/std/epistemologic/property/laws/duration_algebra]],
  [[../../boot/std/epistemologic/property/laws/functor]],
  [[../../boot/std/epistemologic/property/laws/monad]],
  [[../../boot/std/epistemologic/property/laws/monoidal]],
  [[../../boot/std/epistemologic/property/laws/monotonicity]],
  [[../../boot/std/epistemologic/property/benchmark]].
- `[[epistemologic-grammar]]` — the @epistemologic hierarchy and the
  `literal` property's home. Per §0 of that spec: *"the property
  `literal` checks whether a declaration's name IS its operation."*
  This spec's binding mechanism is `literal` applied at the glass
  altitude — the name of a property bound on a glass IS the
  qualifier the glass demands.
- `[[property-error-surface]]` — the verdict carrier and error
  surface. The back-projected implementations close their loops
  through the verdict type ([[../../boot/std/epistemologic/property]]
  declares `verdict = pass | fail(diagnostic) | partial(f64,
  [diagnostic])`).
- `[[typed-loss-composition]]` — the typed-loss algebra the
  back-projected check's residual feeds into. The per-glass loss is
  a `PropertyVerdict` (not a bare scalar; per
  [[feedback-loss-from-epistemologic-properties]]).
- `[[scheduler-tower]]` §7.4 — the `requires halts(gen_prism)`
  pattern. The template for `requires <property>(glass)` everywhere
  in this spec. The `@scheduler.reduction_budget(shard)` primitive
  is the budget backing hard-RT property verdicts.
- `[[../../roadmap/pending/runtime-elevation]]` — the architectural
  frame (2026-06-01). HamiltonScheduler at the shard altitude,
  Body=prism+glass+AST at the step altitude, SpectralSupervisor at
  the system altitude. This spec lives inside that frame: per-glass
  properties ARE the property altitude that makes Rust-side
  determinism *verifiable from content* across every language the
  substrate speaks.
- `fragmentation/docs/specs/hamilton-scheduler.md` §4 (commit
  `e227f1e`) — the `Pure` AST-verdict framing. This spec corrects
  its scope: `Pure` is not one property per body; it is a per-glass
  binding under [[../../boot/std/epistemologic/property/glass_wall]]
  semantics. The §4 upsert is owed; flagged in §10 below.
- `fragmentation/docs/specs/lens-transit.md` — the transit
  measurement that closes the dynamic half of every per-glass
  property's loss verdict. The hard-RT integration in §4 of that
  spec is the load-bearing pair: this spec declares; transit
  observes.
- `[[../cicd/kintsugi-thesis]]` — the 9-point reproducibility
  chain. This spec closes Claim 7 ([[../cicd/kintsugi-thesis]] §C7)
  and partial-closes Claim 9 ([[../cicd/kintsugi-thesis]] §C9) via
  the per-glass binding mechanism. Scoring update in §8.

Unblocks:
- The cross-language formal verification destination ([[#6]] below).
  Rust + Elixir/BEAM + Fortran + LLVM IR + @io seams verified
  end-to-end against the same `@epistemologic/property/*` chain.
- The substrate-pull discipline ([[../../AGENTS.md]] § "The Glass
  Wall", § "Boundary Rust is not frozen capability") gets a
  type-level enforcement mechanism. `glass_wall` already enforces
  namespace containment; properties-on-glass enforces *contract*
  containment within those namespaces. The next AGENTS.md update is
  owed; flagged in §10.
- The `Pure<G: Glass>` Rust marker question ([[#7]] below). The
  compile-time witness for any Rust code that dispatches through a
  `to @code/rust` glass becomes minted by the liquid-type pass, not
  hand-written. The hand-writeable lie is structurally impossible.
- The closure of [[../cicd/kintsugi-thesis]] §C7 from ⚠️ to ✅ via
  per-glass binding; §C9 from ❌ to ⚠️ (the AST-analysis half) via
  the same mechanism.

---

## 1. The architectural claim (load-bearing, locked)

**A glass is the structural edge a property binds at.**

Not: a property is a per-action declaration. Not: a property is a
per-body marker trait. Not: a property is a per-grammar invariant
lifted into Rust generics. The claim is structural: the **glass**
— [[../../boot/std/mirror/glass]]'s type-pattern for every parseable
surface — is the altitude at which a property contract binds.

This has four load-bearing consequences:

1. **Per-glass qualifier set.** Each `glass to @<target>` declaration
   binds a finite set Q of property names from the
   `@epistemologic/property/*` chain. Every body that crosses the
   glass must witness each name in Q.

2. **Per-language AST implementation.** The same abstract property
   ([[../../boot/std/epistemologic/property/halts]]) gets a *different*
   implementation per glass — because each glass targets a different
   AST. Halts-on-Rust rejects unbounded `loop`; halts-on-Elixir
   rejects unbounded `receive`; halts-on-Fortran rejects unbounded
   `do`. The property's *meaning* is the same; the implementation
   differs by target.

3. **Back-projection via `---`.** When the programmer declares the
   glass and the qualifier set, settlement (Fate + liquid-type
   inference + kintsugi, per [[liquid-types-for-mirror]] §5.4)
   *back-projects* the inferred implementations into the same file,
   below a `---` separator. Above `---`: the contract the programmer
   wrote. Below `---`: what the substrate inferred. Git history is
   the audit trail; re-settlement re-projects.

4. **Cross-language formal verification at the seam.** A system
   assembled from Rust + Elixir + Fortran + LLVM IR + @io is verified
   end-to-end against the same `@epistemologic/property/*` chain
   because *each glass independently witnesses its compliance on its
   own AST*. CompCert verifies C → assembly. Liquid Haskell verifies
   Haskell. Flux verifies Rust. None of them verify the seams between
   languages. Mirror does — because the seam IS the glass and the
   property is glass-bound.

The claim refuses three re-conflations the substrate has tried
before:

- **"Properties live on the body."** No. Bodies cross glasses; the
  glass is the structural edge. Per-body properties duplicate work
  (every body re-declares halts) and lose the cross-language seam
  (a body in Rust and a body in Elixir would each need their own
  Pure marker with no shared algebra). Per-glass properties bind the
  contract once, at the edge every body crosses.
- **"Properties are SMT-decidable booleans."** No. The decision
  procedure is spectral (per [[liquid-types-for-mirror]] §5.4); the
  verdict is continuous (`pass | fail | partial(f64, ...)`); the
  Dirac operator that routes Fate IS the property verifier. Adding
  SMT would be a substrate pull toward boolean logic, exactly the
  wrong direction.
- **"Pure is a Rust marker trait."** No (and this is the upsert owed
  to `fragmentation/docs/specs/hamilton-scheduler.md` §4). `Pure` is
  a per-glass binding under `glass_wall` semantics. The Rust-side
  surface — `Pure<G: Glass>` — is *minted by the compiler* during
  the liquid-type pass for glass G, not hand-written. Hand-writeable
  impls can't lie because they cannot be written.

---

## 2. Grounding in the `@epistemologic` chain

The canonical chain at [[../../boot/std/epistemologic/property]]
already names every primitive this spec uses. **Nothing is invented
here.** The spec's job is to *bind* existing primitives at the glass
altitude — not to extend the chain.

### 2.1 The chain as it stands

From a survey of [[../../boot/std/epistemologic/property]] as of
2026-06-01:

| Primitive | Home | Shape (verifier signature) |
|---|---|---|
| `halts(type)` | [[../../boot/std/epistemologic/property/halts]] | Two-clause disjunction: `autopoietic_settles ∨ reductions_bounded`, with `disjunction_decidable` as the third gate. |
| `autopoietic(type)` | [[../../boot/std/epistemologic/property/autopoietic]] | Banach contraction on hash space — `fixed_point_exists ∧ fixed_point_unique ∧ tick_is_contraction`. |
| `content_addressed(type)` | [[../../boot/std/epistemologic/property/content_addressed]] | `has_oid ∧ oid_determines_identity ∧ oid_round_trips`. |
| `glass_wall(g)` | [[../../boot/std/epistemologic/property/glass_wall]] | `is_mirror_shaped ∨ under_io_namespace` — the namespace-level discipline. |
| `io_safety(ast)` | [[../../boot/std/epistemologic/property/io_safety]] | `bounded_io ∧ error_path ∧ cache_nonblocking ∧ eof_handling`. |
| `frame_relativity(type)` | [[../../boot/std/epistemologic/property/frame_relativity]] | `carries_frame ∧ cross_frame_compare_requires_convert ∧ conversion_path_defined`. |
| `is_prism_record(type)` | [[../../boot/std/epistemologic/property/is_prism_record]] | `five_fields ∧ one_per_operation ∧ shapes_match_operations`. |
| `coincidence_matches()` | [[../../boot/std/epistemologic/property/coincidence_matches]] | Bootstrap's content address equals the grammar's predicted hash for a pinned corpus. |
| `total_classification(ast)` | [[../../boot/std/epistemologic/property/total_classification]] | Every source byte falls into a recognized AST node — no Dark children. |
| `duplicate_variant(ast)` | [[../../boot/std/epistemologic/property/duplicate_variant]] | No repeated variant name in a Split node. |
| `filename_matches_glass(file)` | [[../../boot/std/epistemologic/property/filename_matches_glass]] | The canonical keyword a file declares matches the filename stem. |
| `causality(type)` | [[../../boot/std/epistemologic/property/laws/causality]] | Happens-before partial order, Lamport's clock condition lifted to the type layer. |
| `duration_algebra(type)` | [[../../boot/std/epistemologic/property/laws/duration_algebra]] | Commutative monoid under `+`, zero identity, partial subtraction, total non-neg scaling. |
| `functor_laws(type)` | [[../../boot/std/epistemologic/property/laws/functor]] | `identity_law ∧ composition_law` for `shift(T)`. |
| `monad_laws(type)` | [[../../boot/std/epistemologic/property/laws/monad]] | `left_identity ∧ right_identity ∧ associativity` for `settle(T)`. |
| `monoidal(type)` | [[../../boot/std/epistemologic/property/laws/monoidal]] | `identity ∧ associativity` — Mac Lane 1971 ch. VII §1. |
| `monotonicity(type)` | [[../../boot/std/epistemologic/property/laws/monotonicity]] | `ordered ∧ non_decreasing ∧ total_within_frame`. |
| `no_hang(ast)`, `linear_compile(ast)`, `deterministic_oid(ast)`, `cache_speedup(file)`, … | [[../../boot/std/epistemologic/property/benchmark]] | Performance properties as verdicts. The benchmark family. |

The chain follows a recurring four-actions-per-file pattern:
three sub-clauses (often clauses of a conjunction or disjunction)
plus the combined property. `halts.mirror`'s shape
(`autopoietic_settles ∧/∨ reductions_bounded ∧ disjunction_decidable
→ halts`) is canonical. New per-glass implementations follow this
shape — they don't invent new primitives, they discharge existing
ones against a target AST.

### 2.2 The `literal` property at the glass altitude

[[epistemologic-grammar]] §0 names `literal` as the root property
for the whole epistemologic family: *"the property `literal` checks
whether a declaration's name IS its operation."* When a glass binds
`property halts`, the binding *literally* says: the name `halts` IS
the verifier the glass demands. The qualifier set is a list of
`literal` invocations at the glass altitude.

This means: the per-glass qualifier set isn't just a list of strings.
It's a list of refs into the chain, each backed by `literal`'s
name-IS-operation discipline. A glass that binds `property foobar`
fails at compile time because `@epistemologic/property/foobar` does
not resolve — `literal(foobar)` returns `fail(no_such_property)`.
Invented property names cannot escape this check.

### 2.3 What the spec deliberately does NOT invent

A non-exhaustive list of primitives this spec resisted adding,
deferring them as future chain extensions if real consumers surface
(per [[../../AGENTS.md]] § "Deferral over premature implementation"):

- `pure(type)` — **elevated 2026-06-01** (Alex). The conjunction
  `halts ∧ ¬contains(@io.*) ∧ deterministic_oid ∧ ¬contains(@rand.*)`
  is clean enough that the named primitive earns chain residency.
  Glasses may bind `property pure(type)` directly; the implementation
  is the four-clause conjunction backed by the existing chain primitives.
  See [[#10.3]] — this is the first deferred-addition to materialize
  during the spec's own drafting cycle, and it surfaces because the
  conjunction recurs across multiple canonical glasses. The substrate-pull
  is real: name it once, use it everywhere. Chain residency at
  `@epistemologic/property/pure` is owed.
- `hermetic_no_remote` — *resisted*. The existing
  [[../../AGENTS.md]] § "The Local-Bounded Guarantees" plus the
  `glass_wall` namespace-check plus a hypothetical
  `¬contains(@network.*)` covers it. Naming it would duplicate
  existing structure.
- `no_unsafe` — *resisted*. This is a Rust-specific syntactic check
  (`¬contains(unsafe { ... })`) that belongs in the per-glass
  implementation for `to @code/rust`, not as an abstract primitive
  in the chain. Other languages don't have `unsafe`; lifting it to
  the abstract level would make the chain Rust-shaped.
- `reproducible_seed` — *resisted*. [[../cicd/kintsugi-thesis]] §C4
  names the work owed for seed-pinning; it's Fate-side, not
  property-side. The seed is a `@fate.infer` argument, not a glass
  qualifier.
- `bounded_reductions(budget)` — *resisted as a property*; **the
  budget itself** is the existing
  [[scheduler-tower]] §7.4 `@scheduler.reduction_budget(shard)`
  primitive. The property's clause is
  [[../../boot/std/epistemologic/property/halts]]'s
  `reductions_bounded`; the budget is the parameter the clause
  reads from the host shard. Naming `bounded_reductions(budget)`
  as a new primitive would duplicate `reductions_bounded`.

**Where the chain has gaps the spec needs but cannot patch inline,**
the spec flags them as **deferred chain additions** in §10 (followup
ticks owed). Two such gaps surfaced; both are mechanical extensions
that extend the existing four-actions-per-file pattern:

- `[[../../boot/std/epistemologic/property/has_decreasing_measure]]`
  — a structural-recursion primitive every per-glass `halts`
  implementation walks the AST against. Today this discipline lives
  inside each `halts` implementation's body; surfacing it as a
  named clause would let other properties (`wcet_bounded`,
  `terminates_under_load`) reuse the measure without re-implementing
  it.
- `[[../../boot/std/epistemologic/property/no_external_capability]]`
  — the abstract form of "no `@io` calls in the AST" lifted from
  language-specific checks. The clause is one substrate ref-prefix
  query; lifting it would let Pure and io_safety share an
  implementation across glasses.

Neither is invented here. Both are flagged as the work surfacing
would warrant.

---

## 3. The glass + property declaration syntax

### 3.1 The contract surface

A glass declaration extends [[../../boot/std/mirror/glass]]'s type
pattern with a *qualifier-set clause* — the property names a body
crossing the glass must witness:

```mirror
in @prism
in @epistemologic/property/halts
in @epistemologic/property/io_safety
in @epistemologic/property/laws/monotonicity

grammar @kintsugi/dispatch {

  # A glass binds a qualifier set. Every body crossing this glass
  # must witness EACH named property as Pass (or Partial above
  # the substrate's documented confidence threshold) against the
  # glass's target AST.
  glass to @io {
    property halts
    property io_safety
    property monotonicity
  }

  glass to @code/rust {
    property halts
    property content_addressed
  }
}
```

The binding is concise. Each `property <name>` line is a ref into
the `@epistemologic/property/*` chain, resolved by
[[../../boot/std/epistemologic/resolve]]'s `verify_literal` —
property names that don't resolve fail at compile time.

### 3.2 The qualifier-set semantics

The qualifier set Q for a glass is an *unordered conjunction*. A
body crossing the glass witnesses Q iff each name in Q discharges
as `Pass` (or `Partial` within a documented confidence bound; see
[[#4]] below for what "documented" means). Order is not part of
the semantics; the verdict carrier ([[../../boot/std/epistemologic/property]]'s
`verdict` type) is associative and commutative under the existing
`Transparency<Ref>::combine` law
(`prism/imperfect/src/transparency.rs`).

A body that *partially* witnesses Q — say,
[[../../boot/std/epistemologic/property/halts]] discharges as `Pass`
but [[../../boot/std/epistemologic/property/io_safety]] discharges
as `Partial { confidence: 0.72, … }` — produces a glass-level
`Partial { confidence: combine(1.0, 0.72), … }` verdict. The
combine is the same monoid the substrate already uses for verdict
composition; this spec doesn't invent a new combine.

### 3.3 Inheritance through `in @<chain>`

A glass inherits its parent's qualifier set through the `in`
declarations the enclosing grammar provides. Per
[[../../boot/std/epistemologic/resolve]]'s `chain` action, the
`/` separator in `@epistemologic/property/halts` IS the
inheritance path:

```
@epistemologic/property/halts
  ⊃ @epistemologic/property
  ⊃ @epistemologic
  ⊃ @prism
```

When a glass binds `property halts`, every clause of `halts`'s
shape (`autopoietic_settles`, `reductions_bounded`,
`disjunction_decidable`) is part of the qualifier set transitively.
The body's witness for `halts` is the conjunction of its witnesses
for each clause. No re-declaration is needed at the glass surface.

### 3.4 The four canonical glasses (sketches)

The runtime-elevation track ([[../../roadmap/pending/runtime-elevation]])
names the load-bearing glasses for v1.5. Each binds a different
qualifier set, reflecting what the glass's target AST can witness:

```mirror
in @prism
in @epistemologic/property/halts
in @epistemologic/property/io_safety
in @epistemologic/property/glass_wall
in @epistemologic/property/laws/causality
in @epistemologic/property/content_addressed

grammar @kintsugi/dispatch {

  # The @io boundary — every cross of this glass is a syscall or
  # external-tool invocation. The qualifier set is conservative:
  # the boundary must be observable, bounded, and well-ordered.
  glass to @io {
    property halts          # bounded by @scheduler.reduction_budget
    property io_safety      # the four-clause @io discipline
    property causality      # cross-process events follow Lamport
  }

  # The Rust boundary — bodies expressed as Rust AST. The qualifier
  # set reads the AST for unbounded constructs and impure ops.
  glass to @code/rust {
    property halts            # no unbounded loop / recursion
    property content_addressed # Body OID is stable across rebuild
  }

  # The BEAM boundary — Elixir/Erlang AST. halts here is unbounded
  # `receive` + unbounded `spawn`; io_safety here is the OTP
  # supervision discipline.
  glass to @code/beam/eaf {
    property halts
    property io_safety
    property causality      # BEAM's send-before-receive is Lamport
  }

  # The Fortran boundary — flang's numerical AST. The qualifier set
  # is mostly about loop boundedness and the duration algebra (the
  # FORTRAN do-loop semantics are well-formed under
  # @time.duration).
  glass to @code/fortran {
    property halts
    property content_addressed
  }

  # The LLVM IR boundary — bodies as LLVM IR. halts here reads
  # llvm.loop.* metadata; content_addressed reads the IR's bitcode
  # hash.
  glass to @code/llvm/ir {
    property halts
    property content_addressed
  }
}
```

This is the contract surface. The implementations live below `---`
after settlement runs. Same file. Same OID-addressable bytes. See
[[#4]].

---

## 4. The `---` separator and back-projection

### 4.1 Disambiguating `---`

A grep across `/Users/alexwolf/dev/projects/mirror/boot/**/*.mirror`
as of 2026-06-01 shows **zero existing uses of `---` as a separator
in `.mirror` files**. The convention is currently exclusive to
markdown frontmatter (`docs/**/*.md`). This spec defines `---` for
`.mirror` files specifically as the **settlement back-projection
separator**, with no overlap with markdown frontmatter (which
`.mirror` files do not carry).

The disambiguation is structural: `.mirror` files are parsed
through the meta-glass ([[../../boot/std/mirror/grammar]]); markdown
frontmatter doesn't apply. The tokenizer's `dark_fallback` branch
([[../../boot/std/mirror/grammar]] §form) currently swallows any
unrecognized top-level token; this spec proposes adding
`---` as a recognized form, semantically *"end of contract; below
is settlement output."*

**The structural condition:** `---` is a line containing exactly
three hyphens and nothing else. It appears at most once per file.
Everything above is the programmer's contract; everything below is
settlement output. A file with no `---` is a contract that hasn't
been settled yet (the [[#4.3]] freshness rule names when this is
an error).

### 4.2 What back-projection writes

When settlement runs (Fate-driven liquid-type inference + kintsugi,
per [[liquid-types-for-mirror]] §5.4), it appends three things
below `---`:

1. **A settlement header** — when the settlement ran, the OID of
   the file's contract portion (above `---`), and a one-line
   citation of the inference path used.
2. **One implementation declaration per glass × property pair** —
   the body Fate filled into the `\` hole, plus a brief comment
   naming the structural conditions the implementation walks the
   AST for.
3. **A settlement verdict** — the per-glass `verdict` value (Pass /
   Fail / Partial with confidence), located at each
   `(glass, property)` pair, carried as the existing
   `Transparency<Ref>` shape from
   `prism/imperfect/src/transparency.rs`.

The back-projected file shape:

```mirror
in @prism
in @epistemologic/property/halts
in @epistemologic/property/content_addressed

grammar @kintsugi/dispatch {
  glass to @code/rust {
    property halts
    property content_addressed
  }
}

---

# Back-projected after settlement on 2026-06-15.
# Contract OID: 4a3f8b2c... (above the ---)
# Settlement path: kintsugi(fate(liquid_inference(@code/rust)))
# These implementations are mechanical artifacts. Re-settle to regenerate.
# Re-settlement OID: <will-be-computed-on-next-settle>

implementation halts for @code/rust glass {
  # Walks Rust AST. Discharges halts via two clauses per
  # @epistemologic/property/halts:
  #   (a) autopoietic_settles: every recursive call decreases on a
  #       structural measure (the function's @fate-inferred
  #       decreasing argument);
  #   (b) reductions_bounded: every loop has a bound the AST
  #       analysis can verify against @scheduler.reduction_budget.
  # Fail clause: unbounded `loop`, recursive call without measure,
  # async/await without timeout.
  \
}

implementation content_addressed for @code/rust glass {
  # Walks Rust AST. Discharges content_addressed via three clauses
  # per @epistemologic/property/content_addressed:
  #   (a) has_oid: the body's AST bytes produce a stable OID
  #       (immediate from Body=prism+glass+AST per
  #       hamilton-scheduler.md §5.1);
  #   (b) oid_determines_identity: no Rust-side identity surface
  #       (PartialEq on closure types) escapes the AST;
  #   (c) oid_round_trips: re-tokenizing the Rust source produces
  #       the same AST OID.
  \
}

verdicts {
  (@code/rust, halts):            pass
  (@code/rust, content_addressed): pass
}
```

The `\` holes in the back-projected implementations are *not* the
same holes the programmer originally wrote. They are Fate-filled
at settlement time; the body they ship with names the AST
walk-shape and the structural clauses each property discharges.
The holes' filling lives in the `gestalt` (per
[[../../boot/std/mirror/spectral]]); the back-projected file
carries the *declaration* of the implementation's shape, not the
inlined AST-walker bytecode.

### 4.3 When re-settlement re-projects

Re-settlement is triggered by content changes anywhere in the
property's transitive `/` chain (per
[[../../boot/std/epistemologic/resolve]]'s `chain` action). When
[[../../boot/std/epistemologic/property/halts]] evolves — a new
clause lands, the decidability gate tightens, a new sibling
property surfaces in the chain — settling the dispatcher re-runs
Fate + liquid inference and writes a new implementation below
`---`. The old implementation goes to git history. The audit
trail is the version-control diff.

The **freshness rule**: the back-projected portion's settlement
header carries the contract-portion OID at settlement time. If the
contract portion has changed since the recorded OID, the file is
*stale*: the compiler emits a `Partial { confidence: 0.0,
diagnostics: vec![Diagnostic::new("properties-on-glass: stale
---; re-settle to regenerate")] }` verdict at the glass altitude
and refuses to admit bodies for the affected glasses until
re-settlement.

Stale is structural, not a warning. A stale contract is a contract
whose name no longer matches its operation —
[[epistemologic-grammar]]'s `literal` failing at the file altitude.
The substrate refuses to silently dispatch through a glass whose
implementation predates the contract it claims to discharge.

### 4.4 Re-settlement determinism

Re-settlement must itself be deterministic. The thesis
([[../cicd/kintsugi-thesis]] §C4) names the work owed to make
`@fate.infer` seed-pinned; that work is *upstream* of this spec.
What *this* spec commits to:

- The settlement input is the file's contract portion (above `---`)
  plus the transitive `/` chain of property declarations. Both are
  content-addressed.
- The settlement output (everything below `---`) is a pure function
  of the input, *given* `@fate.infer` is deterministic.
- The settlement header records the input OID. Two settlements with
  the same input OID produce identical output bytes — by construction,
  once C4 closes.

Until C4 closes, settlement output carries a `partial` verdict with
confidence bounded by `@fate.infer`'s empirical determinism quantile
(per [[../cicd/kintsugi-thesis]] §C4's verdict). The settlement
header documents this honestly; the substrate refuses to claim
full `pass` for an au-bound implementation while the upstream is
still ⚠️.

### 4.5 Why same-file, not two-file

A two-file layout (one for the contract, one for the
back-projection) was considered and refused. Three reasons:

1. **Single-file is single-OID.** The contract and its
   implementation share a substrate path. Re-settling them as one
   file produces a single new OID; the old OID is the previous
   settlement. Two files require coordinating two OIDs that must
   remain in lock-step — a synchronization problem the substrate
   doesn't need to invent.
2. **`literal` reads the whole file.** The `literal` property at
   the file altitude checks that the declared identity matches the
   observed implementation. Same-file makes this a one-step check;
   two-file requires the verifier to chase a cross-reference (which
   would need a new primitive, which would be invented in this
   spec — a refusal per [[#2.3]]).
3. **The `---` convention is already legible.** Anyone reading
   markdown reads frontmatter-with-`---` as "declared, then
   content." Lifting the convention into `.mirror` files preserves
   the reading discipline. Above is what the human said; below is
   what the substrate inferred. The same instinct everywhere.

### 4.6 The `---` is bi-directional — custom properties below, referenced above

The `---` separator is not only the back-projection landing — it is
also the **definition site for custom properties** that the file's
own glasses can reference. Below `---` lives BOTH the back-projected
implementations (settlement output) AND custom property definitions
the contract above refers to. The flow is bi-directional:

- **Top-to-bottom:** a glass above `---` binds `property X`; X is
  either a chain primitive
  ([[../../boot/std/epistemologic/property/X]]) or a custom property
  defined below `---`.
- **Bottom-to-top:** a custom property defined below `---` is
  referenceable by any glass above `---` in the same file, and by
  any descendant file through the `/` inheritance chain (see
  [[#4.6.2]]).

#### 4.6.1 Custom property syntax

A custom property below `---` is a **named conjunction of chain
primitives**. The substrate doesn't admit hand-written check bodies
below `---` — only conjunctions. The substrate doesn't invent new
check semantics; it gives a local name to a composition the chain
already supports.

```mirror
in @epistemologic/property

grammar @kintsugi/dispatch {
  glass to @code/rust {
    property halts
    property tournament_safe   # custom, defined below
  }
}

---

# Custom property — local to this grammar's descendants.
# Names a conjunction of @epistemologic chain primitives.
property tournament_safe =
    halts
  ∧ deterministic_oid
  ∧ glass_wall(@fate)
  ∧ ¬contains(@io.*)
```

The custom property's name (`tournament_safe`) is local to the
file's `/`-chain. Its definition is a pure conjunction of chain
primitives. The right-hand side admits only:

- Wikilinks into `@epistemologic/property/*` (chain primitives)
- Other custom properties defined in the same file or its `/`-ancestors
- The five Boolean combinators the chain already supports: `∧`, `∨`,
  `¬`, `⇒`, `contains(…)` (the AST-walk predicate from
  [[liquid-types-for-mirror]] §2.5)

Hand-written check logic in any other form is a compile-time error.
The substrate refuses to admit imperative property bodies; the
bi-directionality of `---` only flows *named conjunctions*, never
arbitrary code.

#### 4.6.2 Cascading through `/` inheritance

Per [[#3.3]], glasses inherit qualifier sets through the `/` chain.
**Custom properties cascade the same way.** A grammar at
`@kintsugi/dispatch/tick.mirror` sees custom properties defined in
`@kintsugi/dispatch.mirror` (its parent) and `@kintsugi.mirror` (its
grandparent), each visible below the respective file's `---`.

```
@kintsugi/dispatch/tick
  ⊃ @kintsugi/dispatch       # tournament_safe defined here, visible
  ⊃ @kintsugi                # parent grammar
```

This means **substrate-pull materializes per-glass through the `/`
chain.** Each grammar can ratchet its discipline by adding a custom
property below its `---`; that property is automatically available
to all descendants without re-declaration. The substrate-pull
discipline isn't a global decree imposed on every file — it's a
local extension that propagates by the structural inheritance path
the substrate already has. Each glass can have its own properties.
Each grammar can extend the chain locally without forking it.

#### 4.6.3 Custom properties cannot shadow chain primitives

A custom property name that collides with an existing
`@epistemologic/property/*` name is a compile-time error. The
substrate refuses the file. This keeps the canonical chain
canonical even as files extend it locally; the `/` cascade is
*additive*, never *overriding*. The `literal` discipline at the
file altitude verifies the non-collision.

#### 4.6.4 The bi-directional `---` IS the per-glass substrate-pull mechanism

The combination — per-glass property declarations above `---` +
custom properties defined below `---` + `/`-chain cascade for both
— is what makes per-glass substrate-pull operational. A glass at
the child grammar can declare a property defined three levels up
the chain; re-settlement re-verifies the conjunction against the
child's bodies; ill-formed bodies get rejected at compile time.

The substrate-pull discipline doesn't need a global enforcement; it
lives at each glass, propagated by the file's location in the `/`
chain. **The `---` separator is the smallest piece of syntax that
makes the discipline material.**

---

## 5. Per-language AST implementations

### 5.1 The cross-language verification table

The same abstract property
([[../../boot/std/epistemologic/property/halts]]) gets a *different*
implementation per glass, because each glass walks a different
AST. The property's *meaning* is one — every reflexive trajectory
of the body's evaluation terminates in bounded steps
(`autopoietic_settles ∨ reductions_bounded`). The *implementation*
differs because the AST shapes differ:

| Glass | AST it walks | `halts` discharges by checking… |
|---|---|---|
| `to @code/rust` | Rust AST (`syn`-shaped tree) | No unbounded `loop`; no recursive call without a decreasing structural measure; no `async/await` without a timeout; no `std::thread::sleep` outside a bounded retry. |
| `to @code/elixir` | Elixir/BEAM AST (`Macro.t`) | No unbounded `receive` without `after` clause; no `spawn` without a supervised parent; no `Stream.iterate` without a `take` or `take_while` bound. |
| `to @code/fortran` | Fortran AST (flang's parse tree) | No `do` loop without explicit upper bound; no recursive subroutine without a decreasing measure on a dummy argument; no `do while` without a loop-invariant variant that the AST analysis can prove decreasing. |
| `to @code/llvm/ir` | LLVM IR (textual or bitcode) | Every backedge in the control-flow graph has `llvm.loop.bound` metadata; every recursion has a `tail call` annotation with bounded depth, or a measure the analysis can prove decreasing. |
| `to @io` | The `@io` syscall surface | Every `@io.exec`/`@io.read` is wrapped in `with_timeout(D)`; per [[../../boot/std/epistemologic/property/io_safety]]'s `bounded_io` clause. |
| `to @code/python` (deferred) | Python AST (`ast` module) | No unbounded `while True`; no unbounded `for x in itertools.count()`; no recursion without a sentinel guard. |
| `to @code/go` (deferred) | Go AST (`go/ast`) | No `for {}` without a `break` reachable on every path; no unbounded goroutine fan-out; no `select` without a `default` or a `time.After`. |

The deferred rows are sketches — those glasses don't have substrate
consumers yet (per [[../../AGENTS.md]] § "Deferral over premature
implementation"). They name the *implementation shape* the future
back-projection will take when a real consumer surfaces.

The same table for [[../../boot/std/epistemologic/property/content_addressed]]:

| Glass | `content_addressed` discharges by checking… |
|---|---|
| `to @code/rust` | The body's AST has stable bytes (the Body=prism+glass+AST restructure per `hamilton-scheduler.md` §5.1); no closure captures that escape the AST; PartialEq on body values reduces to AST OID equality. |
| `to @code/elixir` | The body's AST is serializable through `:erlang.term_to_binary` with `[:deterministic]`; no `make_ref/0` or `self/0` in the body. |
| `to @code/fortran` | The body's AST is `compiled-with -ffp-contract=off`; FP order is deterministic (no `-ffast-math`). |
| `to @code/llvm/ir` | The body's IR has a stable bitcode hash; no `tbaa` metadata that varies across rebuild; no `dbg!` records with embedded paths. |
| `to @io` | The wrapped tool invocation carries `requires deterministic(tool, flags = {...})` per [[../cicd/kintsugi-thesis]] §C9. |

The **structural pay-off**: any body crossing any of these glasses
is verified against the same abstract property — by name, by
verdict shape, by composition algebra — but with implementations
that read the appropriate AST. The substrate's `Transparency<Ref>`
carries verdicts from all of them in the same algebra; the
verdict at the dispatcher is the composition.

### 5.2 What the per-glass implementation file looks like

A per-glass implementation is what settlement back-projects. It
lives below `---` in the file declaring the glass; its body is the
`\` hole Fate fills. The structural shape (informally):

```mirror
implementation halts for @code/rust glass {
  # Walks Rust AST. Per @epistemologic/property/halts's three
  # clauses (autopoietic_settles ∨ reductions_bounded ∧
  # disjunction_decidable):
  #
  # (a) autopoietic_settles: for every function-call site, check
  #     that recursive paths have a decreasing structural measure.
  #     The measure is inferred by liquid-type inference (per
  #     liquid-types-for-mirror.md §3) from the function's pattern-
  #     matched arguments. If the inference cannot find a measure,
  #     emit Partial { confidence: <inference_confidence>, ... }.
  #
  # (b) reductions_bounded: every loop construct (`for`, `while`,
  #     `loop`) must have a bound the substrate can verify against
  #     @scheduler.reduction_budget(shard). `for x in iter` is OK
  #     iff iter's type carries a static length; `while cond` is OK
  #     iff cond is liquid-typed as `{v: bool | decreasing(env)}`;
  #     `loop` requires an explicit `break` reachable on every path.
  #
  # (c) disjunction_decidable: the AST walk is decidable because
  #     Rust's AST is sub-Turing under the qualifier set (no
  #     macro_rules! at this glass altitude — macros expand pre-
  #     glass).
  \
}
```

The body is `\` — settlement filled. The substrate's eventual
verification of a Rust body against this glass walks the body's
AST per the comments and composes the per-clause verdicts via the
existing `Transparency<Ref>::combine`.

### 5.3 The cross-language seam

The load-bearing reading of [[#1.4]] above. A pipeline assembled
from three bodies:

```mirror
grammar @demo/pipeline {
  glass to @code/rust    { property halts, property content_addressed }
  glass to @code/elixir  { property halts, property causality }
  glass to @code/fortran { property halts, property duration_algebra }

  # The pipeline composes a Rust orchestrator, an Elixir actor, and
  # a Fortran kernel. Each body crosses its own glass; each glass
  # witnesses its own qualifier set against its own AST.
  pipeline(input: text) -> imperfect(report) {
    @code/rust.parse(input)
      |> @code/elixir.dispatch    # cross the Elixir glass
      |> @code/fortran.compute    # cross the Fortran glass
      |> @code/rust.format        # back across the Rust glass
  }
}
```

The substrate's verification at the pipeline altitude:

1. Each glass's qualifier set is witnessed *against its own AST*
   (Rust AST for the orchestrator; Elixir AST for the actor;
   Fortran AST for the kernel).
2. The verdicts compose via `Transparency<Ref>::combine` —
   associative, commutative; one algebra, three sources.
3. The pipeline-level verdict is the conjunction of the three
   glass verdicts. If all three discharge as `Pass`, the pipeline
   is verified end-to-end against
   `@epistemologic/property/halts ∧ @epistemologic/property/content_addressed ∧ @epistemologic/property/causality ∧ @epistemologic/property/laws/duration_algebra`.
4. The pipeline's run is content-addressable: the input OID, plus
   the three glass-implementation OIDs, plus the pipeline's own
   declaration OID, fully determine the output OID.

This is what no existing system delivers. CompCert verifies one
language to one lower language. Liquid Haskell verifies Haskell.
Flux verifies Rust. **Mirror verifies the seam** because the seam
IS the glass and the property is glass-bound.

The substrate cannot make the per-language *kernel verification
strictly stronger* than the existing language-specific tools.
`halts` on Rust is no stronger than `halts` on a Rust-only verifier
would be — the per-glass implementation is bounded by the AST it
walks. **The novelty is at the seam**: the verifiers compose under
one algebra, against one chain. Cross-language is where the
substrate adds capability the language-specific tools cannot reach.

### 5.4 Honest scoping

What this spec claims, and what it does not:

- **Claims:** The per-glass mechanism produces a structurally
  composable verification surface across languages. The seam
  between languages, today an unverified handoff, becomes a glass
  that witnesses qualifier discharge against a target AST.
- **Does not claim:** Per-language verification is improved beyond
  what the AST analysis can mechanically detect. The per-glass
  implementations are bounded by the same techniques (bounded
  recursion, decreasing measures, explicit loop bounds) any
  language-specific tool uses.
- **Does not claim:** Settlement is decidable in all cases. The
  liquid-type inference is decidable
  (per [[liquid-types-for-mirror]] §4.4: convergence guaranteed if
  loss is monotonically non-increasing — mirror's `e^(n+1) <
  e^(n)`). When inference cannot find a measure, the verdict is
  `Partial`, not `Pass`. The substrate is honest about what it can
  prove and what it cannot.

The load-bearing engineering claim is **the seam**, not the kernel.
The kernel is bounded by existing AST-analysis techniques; the seam
is where the substrate adds.

---

## 6. The liquid-type inference mechanism

This section *operationalizes* [[liquid-types-for-mirror]] for the
per-glass-property setting. The framework is borrowed verbatim;
this spec names how each step binds at the glass altitude.

### 6.1 Step 1: Hindley-Milner shape inference per glass

Per [[liquid-types-for-mirror]] §1.2 Step 1: HM inference
determines the shape types — Focus/Project/Split/Shift/Settle
plus the target-language AST variants. For a glass to `@code/rust`,
HM inference produces the Rust AST shape (typed `syn::Item`-shaped
variants); for `to @code/elixir`, it produces the Elixir AST shape
(`Macro.t`-shaped); etc.

The glass declaration constrains HM: only bodies whose AST shape
is compatible with the glass's target are admissible. A body
declared for `to @code/rust` that contains Elixir AST nodes is a
compile error before any property check runs.

### 6.2 Step 2: Constraint generation against the qualifier set

Per [[liquid-types-for-mirror]] §1.2 Step 2: walk the typed AST;
generate subtyping constraints carrying the qualifier set Q. Where
the research spec named *boolean* qualifiers (predicates from a
decidable SMT fragment), this spec names *property* qualifiers
from the `@epistemologic/property/*` chain.

For a glass with qualifier set
Q = { `halts`, `content_addressed` }, the constraint generator
walks the body's AST and emits, at each callsite, the constraint
"the called body must witness Q-via-this-glass." The constraint
is structural; the verdict is continuous (`pass | fail |
partial(f64, …)` per [[../../boot/std/epistemologic/property]]).

### 6.3 Step 3: Spectral decision — NOT SMT

Per [[liquid-types-for-mirror]] §5.4 (the load-bearing
recommendation): replace SMT with **spectral analysis on the
property Laplacian**. The Dirac operator that routes Fate IS the
property verifier. Same eigenvalue decomposition; two consumers.

For a glass with qualifier set Q, the property Laplacian L_P is
constructed from:

- **Diagonal**: per-clause loss for each `(body_site, property)`
  pair in Q. The loss is the residual from the per-glass
  AST-analysis discharge.
- **Off-diagonal**: property correlations from the grammar's
  structural edges. If `halts` and `content_addressed` are
  correlated through the body's AST (a body that halts is more
  likely to be content-addressable; per
  [[../../boot/std/epistemologic/property/content_addressed]]'s
  preamble: "other properties rely on it"), the off-diagonal
  carries the correlation.

The Dirac operator computes the eigendecomposition. The spectral
gap (λ₁; the Fiedler value) determines whether the qualifier set
is simultaneously satisfiable for the body. The Fiedler vector
locates failure modes geometrically — *where in the body's AST*
the property cluster fails, not just *that* it fails.

### 6.4 The output: the back-projected body

The filled `\` hole in the back-projected implementation is the
AST-walking body Fate selects under spectral guidance. The selection
is the same `Fate.fill` operation that resolves any other `\` hole;
the new bit is that the property Laplacian's spectral profile is
part of Fate's feature vector. Bodies that score higher on
property satisfaction win the spectral tournament.

The settlement loss is the inferred property check's residual,
carried as `PropertyVerdict` per
[[../../boot/std/epistemologic/property]]'s `verdict` type. The
back-projected `verdicts` block (see [[#4.2]]) records this loss
at the file altitude.

### 6.5 The Dirac operator as unified Fate-navigator + property-verifier

The deepest reading of [[liquid-types-for-mirror]] §8.3: using the
Dirac operator for both Fate navigation and property verification
means the property layer adds NO new mathematical machinery. The
same eigenvalue computation routes Fate AND verifies properties.
The compiler's inference engine and its verification engine are
the same thing.

For a glass's qualifier-set discharge:

1. Build the property Laplacian L_P from the body's AST + the
   glass's Q.
2. Compute eigenvalues + Fiedler vector via the Dirac operator
   (already implemented; per `mirror/src/dirac.rs`).
3. If λ₁ > ε: the body witnesses Q to a degree the eigenvalues
   quantify. Pass / Partial with confidence proportional to λ₁.
4. If λ₁ ≤ ε: the body fails to witness Q. The Fiedler vector
   localizes the failure to specific AST sub-trees; the
   diagnostic carries the substrate path.

This is one spectral pass per body per glass. No SMT, no external
dependency, no boolean reduction. The geometry IS the verdict.

---

## 7. The Pure trait connection — Pure<G: Glass>

### 7.1 What Pure was, what Pure becomes

The earlier framing in `fragmentation/docs/specs/hamilton-scheduler.md`
§4 (commit `e227f1e`) named `Pure` as an *AST verdict, not a Rust
marker trait*. The reasoning was correct: per-body marker traits
require audit discipline that the substrate cannot enforce
structurally; the verdict approach makes the check mechanical and
reproducible.

This spec **narrows the scope** of that framing without overturning
it. The verdict approach is still right; the scope is per-glass,
not per-body. Reasoning:

1. **The body crosses a glass; the property binds at the edge.**
   Per-body Pure marks each body individually; per-glass Pure marks
   the contract every body crossing the glass must witness. The
   per-glass binding is one declaration; per-body would be one per
   body. Per-glass is structurally cheaper *and* harder to evade
   (you can't forget to bind a property the glass requires).
2. **The Rust-side surface needs a Rust-side type.** When Rust code
   dispatches a body through a `to @code/rust` glass, the Rust
   type system needs a witness that the body has been verified.
   `Pure<G: Glass>` is that witness — `Body: Pure<RustGlass>`
   compiles iff the liquid-type pass produced a passing verdict.
3. **Hand-writeable lie elimination.** The impl of `Pure<G>` is
   *minted by the compiler* during the liquid-type pass. There is
   no `impl Pure<RustGlass> for FooBody {}` a programmer can write
   directly; the impl is generated as part of settlement. The
   substrate refuses to admit hand-written impls.

### 7.2 The Rust-side surface

The Rust crate `prism_core` gains a generic marker:

```rust
//! prism_core::properties — per-glass property witnesses.
//!
//! Each glass G the substrate declares (e.g. RustGlass, ElixirGlass,
//! FortranGlass, LlvmIrGlass, IoGlass) is a Rust newtype that names
//! a glass declared in mirror. The `Pure<G>` marker is the
//! Rust-side compile-time witness that the body's AST has
//! discharged the qualifier set bound on G as Pass (or Partial
//! within the substrate's documented confidence threshold).
//!
//! The marker is sealed (private inner module). The only path to
//! `impl Pure<G> for B` is through the liquid-type pass; the
//! substrate's build emits the impls. Hand-written impls are
//! rejected at compile time via a sealed-supertrait pattern
//! (the inner trait lives in a private module).

mod sealed {
    pub trait LiquidTypePassWitness {}
}

pub trait Glass: sealed::LiquidTypePassWitness {
    /// The substrate path the glass is declared at, e.g.
    /// `@kintsugi/dispatch/glass/to/@code/rust`.
    const PATH: GlassPath;
}

/// Per-glass property witness. `B: Pure<G>` iff the substrate's
/// liquid-type pass discharged G's qualifier set as Pass (or
/// Partial within threshold) for B's AST.
///
/// Sealed via the supertrait; the only impl path is through the
/// substrate's emitted code.
pub trait Pure<G: Glass>: sealed::LiquidTypePassWitness {
    /// The substrate path the verdict was located at.
    const VERDICT_PATH: VerdictPath;
}
```

Newtypes per [[feedback-no-bare-types]] discipline:

```rust
/// The glass's substrate ref — newtype around mirror's `Ref` shape.
pub struct GlassPath(prism_core::Ref);

/// The verdict's location path — the `(glass, property)` pair.
pub struct VerdictPath(prism_core::Ref);
```

The Rust dispatcher (per `fragmentation/docs/specs/hamilton-scheduler.md`
§5.7) consults `B: Pure<G>` at admission time:

```rust
/// Hard-realtime admission requires Pure<G> ∧ WcetBounded<G>.
/// The bounds are enforced by Rust's type system — admission of a
/// body that doesn't implement Pure<G> is a compile error.
pub fn admit_hard<B, G>(
    crystallizations: &Crystallizations<H>,
    body: B,
    deadline: TickInterval,
) -> /* ... */
where
    B: Body<H> + Pure<G> + WcetBounded<G>,
    G: Glass,
{
    /* ... */
}
```

A body that hasn't gone through the liquid-type pass for glass G
doesn't have `Pure<G>` impl. Dispatching it through a `to @code/rust`
glass is a compile-time type error. The witness is content-addressed
(via the body's AST OID); the Rust type system reflects the
substrate's verdict.

### 7.3 Coherence and the orphan rule

A real Rust concern: the `Pure<G>` impl path crosses the Rust orphan
rule when a third-party crate declares a body and the substrate
generates the impl in a different crate. Two paths:

1. **The bodies live in `prism_core` (or a `prism_core`-shaped
   crate the substrate owns).** Bodies are content (per
   `hamilton-scheduler.md` §5.1 — Body=prism+glass+AST). Putting
   them in `prism_core` is structurally honest; the impls can be
   emitted in the same crate as the types, no orphan-rule issue.
   This is the recommended path.
2. **Bodies live in third-party crates; impls live in a substrate
   crate.** The orphan rule rejects this. The substrate would need
   to either (a) emit a newtype wrapper around the third-party body
   in a substrate crate and impl Pure<G> on the wrapper, or (b)
   require the third-party crate to opt in via a substrate macro
   that emits both the body and the impl. Both are awkward; path
   (1) is the discipline the substrate-pull principle
   ([[../../AGENTS.md]]) already favors.

The spec recommends path (1) and flags path (2) as a deferred
workaround if real consumers surface.

### 7.4 Taut's §4 upsert is owed

The correction to `fragmentation/docs/specs/hamilton-scheduler.md`
§4: replace "`Pure` is an AST verdict, per-body" with "`Pure<G:
Glass>` is a per-glass compile-time witness produced by the
liquid-type pass during settlement." The verdict carrier
(`PropertyVerdict` / `Transparency<Ref>`) is unchanged; the
binding altitude is what shifts.

The upsert is mechanical:

- §4.1: "What Pure is" — replace the per-body framing with the
  per-glass framing.
- §4.2: "The analysis pass" — replace `check_pure(body)` with the
  per-glass dispatch `check_pure_for_glass(body, glass)`. The
  underlying AST walk is the same; the glass parameter selects
  the qualifier set.
- §4.4: "The verdict's home" — the home is still
  `prism_core::PropertyVerdict`, but the marker name is `Pure<G:
  Glass>` (sealed), not a bare property name.
- §4.6: "Pure and `@io`" — the discussion stays correct (a body
  with `@io` calls in its AST cannot impl `Pure<RustGlass>`), but
  the framing shifts: the rejection is per-glass, not per-body.
- §4.7: "`WcetBounded(D)`" — becomes `WcetBounded<G>` with the
  deadline carried in the glass's qualifier set.

Flagged in §10 below; not done in this commit (cross-repo writes
were out of scope per the brief).

---

## 8. What this closes on the reproducibility chain

[[../cicd/kintsugi-thesis]] §3's chain table receives two delta
rows after this spec lands. Going through each:

### 8.1 Claim 7 — Property check determinism — ⚠️ → ✅

From [[../cicd/kintsugi-thesis]] §C7:

> The properties are deterministic by audit; they aren't yet
> deterministic by property check. The fix is mechanical.

The per-glass binding mechanism closes this. Reasoning:

1. Each property in a glass's qualifier set is a ref into the
   `@epistemologic/property/*` chain. The chain's primitives are
   declared as `\` holes today; the holes are filled by Fate at
   settlement time. **The filling lives below `---`, content-
   addressed.** Same input AST + same property declaration +
   deterministic Fate (per §C4) → same back-projected
   implementation → same verdict.
2. The verdict carrier is the existing `Transparency<Ref>` algebra
   (`prism/imperfect/src/transparency.rs`). Composition is
   deterministic by construction. Two settlements over the same
   inputs produce identical verdicts.
3. The compile-time enforcement (`Pure<G>` per [[#7]]) means a body
   that hasn't been verified cannot be dispatched. The audit
   becomes the Rust type system; the verdict becomes a witness.

**Conditional on**: §C4 closes (deterministic Fate inference). The
spec is honest that until C4 closes, the back-projected
implementations carry `Partial` verdicts with confidence bounded
by `@fate.infer`'s empirical determinism quantile. Once C4 closes,
this claim closes too.

Verdict in the chain table: ⚠️ → ✅ (conditional on C4).

### 8.2 Claim 9 — @io boundary discipline — ❌ → ⚠️

From [[../cicd/kintsugi-thesis]] §C9:

> The `@io` wrappers exist; the determinism-flag declaration on
> them does not. This is the largest single piece of yet-to-do
> work in the chain.

The per-glass binding mechanism closes the AST-analysis half.
Reasoning:

1. The `to @io` glass binds
   [[../../boot/std/epistemologic/property/io_safety]] and
   [[../../boot/std/epistemologic/property/halts]]. The
   back-projected implementation walks the body's AST for
   `@io.*`-namespaced calls and verifies each is wrapped per
   `io_safety`'s `bounded_io`, `error_path`, `cache_nonblocking`,
   `eof_handling` clauses.
2. The Pure analysis at the glass altitude (per [[#7]]) refuses
   bodies whose AST contains `@io.*` calls outside the `to @io`
   glass. The discipline that has lived in
   [[../../AGENTS.md]] § "The Glass Wall" as a stylistic rule
   becomes a verifiable property of every body that compiles.
3. The **determinism-flag declaration on `@io` wrappers**
   (`requires deterministic(tool, flags = {...})`) is *not* closed
   by this spec. That requires a substrate-level change to the
   `requires` clause syntax; flagged as work owed in §10.

Verdict in the chain table: ❌ → ⚠️ (AST-analysis half ✅; flag
declaration on `@io` wrappers still owed).

### 8.3 Delta rows in the chain table

[[../cicd/kintsugi-thesis]] §3's chain table, with this spec's
deltas:

| Layer | Before | After this spec |
|---|---|---|
| Property check determinism (per-glass binding) | ⚠️ audit | ✅ (conditional on §C4) |
| @io boundary discipline (AST-analysis half) | ❌ | ⚠️ (AST analysis ✅; flag declarations owed) |
| Cross-language seam verification | n/a | ✅ — new; the seam closes per [[#5.3]] |

The third row is genuinely new. It wasn't on the earlier chain
table because the cross-language seam framing only got named in
this spec; it's named here so the chain accounts for the new shape.

---

## 9. Refusals — what this spec deliberately does NOT do

The substrate-pull discipline ([[../../AGENTS.md]]) requires
explicit accounting of what was almost done and refused. Five
refusals:

### 9.1 No new primitives in the `@epistemologic/property/*` chain

The grounding directive is canonical. Every property name in this
spec is a wikilink into the existing tree. Where a discipline would
have been clearer with a new primitive (a `pure` aggregate; a
`hermetic_no_remote` clause; a `no_unsafe` Rust-specific check), the
spec resisted the addition and named the existing primitives the
discipline composes from. Two surfacings as deferred chain additions
(`has_decreasing_measure`, `no_external_capability`) — both flagged,
neither added inline.

### 9.2 No SMT

Per [[liquid-types-for-mirror]] §5.1's load-bearing decision: mirror
uses spectral analysis on the property Laplacian, not SMT. The
Dirac operator that routes Fate IS the property verifier. Adding
an SMT dependency would be a substrate pull toward boolean logic
and external dependencies — exactly the wrong direction. The verdict
carrier is continuous (`pass | fail | partial(f64, …)`), not
boolean.

### 9.3 No new Pure-trait competitor

The `Pure<G: Glass>` marker named in [[#7]] does **not** compete
with `fragmentation/docs/specs/hamilton-scheduler.md` §4's `Pure`
framing. It **narrows** it: per-glass binding replaces per-body
verdict. The verdict carrier (`PropertyVerdict` /
`Transparency<Ref>`) is shared. The §4 upsert is mechanical;
flagged in §10. Two parallel `Pure` surfaces in the substrate would
be a duplication; the upsert eliminates the duplication.

### 9.4 No competition with Taut's spec

This spec is **complementary** to `hamilton-scheduler.md` +
`lens-transit.md`. The decomposition:

- **`hamilton-scheduler.md`**: the per-shard scheduling discipline.
  The HamiltonScheduler. Body=prism+glass+AST. The dispatch
  surface. The realtime classes.
- **`lens-transit.md`**: the dynamic measurement carrier. Wall-clock
  observation. FP precision loss. Cache pressure. The measured
  reality against the declared bound.
- **`properties-on-glass.md`** (this spec): the **static** binding
  at the glass altitude. Per-glass qualifier sets. Back-projection.
  Cross-language seam.

Taut's specs declare the realtime contract; transit observes it;
this spec binds the qualifier set the contract discharges. Three
specs, one stack, no overlap.

### 9.5 No claim of novelty (per the brief)

The brief settled the novelty framing: prove correctness /
reproducibility / determinism, not "this is new."
[[liquid-types-for-mirror]] §8 already named the novelty
(spectral liquid types; continuous refinement types; property
inference via Dirac operator; spectral sparsification for property
checking). This spec is what cashes those into per-glass discipline.
No novelty claims are re-staked here; the integration is the
contribution.

---

## 10. Followup ticks owed

In the order they unblock other work:

### 10.1 The §4 upsert to `fragmentation/docs/specs/hamilton-scheduler.md`

Replace per-body Pure framing with per-glass `Pure<G: Glass>`. The
mechanical changes are listed in [[#7.4]]. Owner: whoever
follows this commit on the Taut spec. The upsert is small
(approximately 4 section deltas; ~200 LOC of markdown).

**Substrate-pull marker**: `[substrate-pull:realize]` — the upsert
realizes a narrower binding altitude; no new capability.

### 10.2 The AGENTS.md substrate-pull section update

[[../../AGENTS.md]] § "The Glass Wall" already names `glass_wall(g)`
as the namespace-level discipline and pairs it with `halts(g)`. The
update names the third pole: per-glass property binding as the
*contract-level* discipline. The §"What NOT to do" list gains a row:
*"Do NOT bind property names that don't resolve into
@epistemologic/property/*."* Owner: agent landing the next AGENTS.md
refresh.

### 10.3 Active and deferred additions to the @epistemologic/property chain

Three primitives surfaced during this spec's drafting. One is
**active** (elevated 2026-06-01 by Alex); two are **deferred** with
trigger conditions:

- **`pure(type)` — ACTIVE.** Conjunction
  `halts ∧ ¬contains(@io.*) ∧ deterministic_oid ∧ ¬contains(@rand.*)`.
  Elevated from the refused list (see [[#2.3]]) because the
  conjunction recurs across multiple canonical glasses and the named
  primitive earns chain residency. Chain home:
  `@epistemologic/property/pure`. Implementation is the conjunction;
  no new semantics. Trigger condition: this spec landing. **First**
  deferred-addition to materialize during the drafting cycle.

The deferred two:

1. **[[../../boot/std/epistemologic/property/has_decreasing_measure]]**
   — the structural-recursion clause every per-glass `halts`
   implementation walks the AST for. Lifting it would let
   `halts`, `wcet_bounded`, and `terminates_under_load` share
   the implementation. Trigger to land: when the second
   `halts`-shaped property surfaces (likely `wcet_bounded`
   per `hamilton-scheduler.md` §4.7).
2. **[[../../boot/std/epistemologic/property/no_external_capability]]**
   — the abstract "no `@io.*` calls in the body's AST" lifted
   from the per-language `io_safety` and Pure implementations.
   Trigger to land: when the third language glass
   (`@code/python` or `@code/go`) needs the same discipline
   per-language.

Neither is added inline. Both are flagged here so the next
substrate-pull pass can name the trigger condition and discharge
the chain extension when it arrives.

### 10.4 The `---` separator support in the tokenizer

The meta-glass tokenizer ([[../../boot/std/mirror/grammar]] §form)
gains a recognized `settlement_separator` form: a line containing
exactly three hyphens. Adding this is one form to the `choice`
disjunction in `@mirror/grammar`'s `form` definition; the
`dark_fallback` branch no longer swallows `---`.

**Substrate-pull marker**: `[substrate-pull:realize]` — the
tokenizer learns one new keyword; everything downstream
(back-projection, freshness check, re-settlement) is grammar-level.

### 10.5 The freshness verdict at the file altitude

The staleness check ([[#4.3]]) needs a primitive: the file altitude
verdict for *"contract OID has changed since recorded settlement."*
This is either:

- A new clause in
  [[../../boot/std/epistemologic/property/content_addressed]]
  named `settlement_header_matches_contract`, lifting the existing
  `oid_round_trips` clause to the file altitude. Recommended path.
- A new property [[../../boot/std/epistemologic/property/settlement_fresh]]
  declared inline. Not recommended — duplicates the structure
  `content_addressed` already carries.

Flagged. Trigger to land: when the first stale `---` is observed in
a compile pass.

### 10.6 The seed-pinning closure (§C4 of kintsugi-thesis)

Not owned by this spec, but blocks the conditional ✅ on §C7. Flag:
 the spec's confidence in `Pass` for back-projected implementations
is bounded by `@fate.infer`'s empirical determinism quantile until
seed-pinning closes. Owner: whoever lands the
`requires deterministic(@fate.infer, seed)` substrate change.

---

## 11. Open questions — honest about what's unresolved

### 11.1 Per-glass vs per-grammar scoping

The spec binds at the glass altitude. An alternative considered:
bind at the grammar altitude (`grammar @foo { property halts,
property io_safety }` — the qualifier set applies to every glass
the grammar declares). The grammar-altitude framing was rejected
because different glasses in the same grammar usually have
*different* qualifier sets (a grammar that declares both `to @io`
and `to @code/rust` glasses needs different properties for each).
Forcing one qualifier set across all glasses in a grammar would
over-constrain the contract.

The **open question**: should the grammar carry a *default*
qualifier set that individual glasses can override or extend? This
would let common cases ("all glasses in this grammar require
halts") express concisely, while preserving per-glass refinement.
The spec leaves this open — the simpler shape (qualifier per glass,
no grammar-level default) lands first; defaults wait for a real
case that wants them.

### 11.2 Property compositionality across glasses

When a pipeline crosses three glasses (Rust → Elixir → Fortran;
per [[#5.3]]), the per-glass verdicts compose via the existing
`Transparency<Ref>::combine`. But what about *property*
compositionality across glasses? Specifically: if the Rust body
witnesses `halts`, and the Elixir body witnesses `halts`, does the
pipeline witness `halts`?

The answer is *yes by construction* if each glass's `halts`
implementation is faithful to the abstract property
[[../../boot/std/epistemologic/property/halts]]'s
shape (`autopoietic_settles ∨ reductions_bounded`). But the
fidelity is the open question: does the Rust `halts` implementation
reason about the *same* notion of "reflexive trajectory" as the
Elixir `halts` implementation? Probably yes for `halts` (the
bounded-reductions clause is portable); probably no for
`io_safety` (the per-language @io surfaces differ in shape).

Flagged: an abstract-property-fidelity check, perhaps as a chain
extension at [[../../boot/std/epistemologic/property/cross_glass_fidelity]],
would make this verifiable. Deferred to a future spec.

### 11.3 Partial-confidence threshold management

The spec mentions "Partial within a documented confidence bound"
several times without naming the bound. The substrate doesn't yet
have a primitive for "the threshold confidence above which Partial
counts as Pass for admission." Options:

- A global constant in [[../../boot/std/epistemologic/property]]
  (e.g., `partial_pass_threshold = 0.85`). Simple; probably
  wrong for hard-realtime where the threshold should be tighter.
- A per-glass constant (each glass carries its own threshold). More
  expressive but adds a knob.
- A per-property constant (each property in the chain declares its
  own threshold). Most expressive; risks fragmentation.

The spec leaves this open. The first consumer needing a non-default
threshold names the discipline.

### 11.4 Settlement output size and gestalt placement

The back-projected implementations could be small (a few comments
+ a `\` hole) or large (full AST-walker bodies inlined). The spec
recommends *small* — the implementation declares the shape; the
filled bytes live in the gestalt (per
[[../../boot/std/mirror/spectral]]). But "small" needs a threshold:
how many bytes below `---` is acceptable before the implementation
should be hoisted into a dedicated file in the substrate's
gestalt?

Flagged. Trigger to revisit: when the first back-projected file
exceeds (say) 4KB below `---`.

### 11.5 The cross-language verification claim's load-bearing-ness

The brief asked for an honest read on whether the cross-language
formal verification framing reads as a *load-bearing engineering
claim* or as *overclaim*. Honest answer:

It's a load-bearing engineering claim *conditional on*:

- The per-glass implementations being faithful to the abstract
  property (per [[#11.2]] open question).
- The composition algebra being preserved across glasses (true by
  construction — `Transparency<Ref>::combine` is shared).
- The Dirac-operator-as-verifier producing actionable verdicts at
  reasonable scale (per [[liquid-types-for-mirror]] §9.3 — open
  question about eigenvalue computation at large garden scales).

It **would be an overclaim** if framed as "mirror verifies
*everything* a multi-language system does, end-to-end." Each
per-glass implementation is bounded by the AST-analysis techniques
any language-specific verifier uses. The substrate doesn't make
the per-language kernel stronger than existing tools.

**Where the claim is genuinely load-bearing**: the seam. The handoff
between languages, today an unverified gap, becomes a glass that
witnesses qualifier discharge against a target AST. No existing
system composes per-language verdicts under a shared algebra
against a shared property chain. That's what mirror adds; the
claim should be stated as *that*, not as "we verify everything."

The spec is honest about this in [[#5.4]]. The framing throughout
favors the seam-level reading and avoids the per-language-kernel
overclaim.

---

*The glass is the structural edge. The property binds at the edge.
The `---` separates contract from implementation. The Dirac
operator routes Fate and verifies properties — one spectral pass,
two consumers. Cross-language verification is what closes at the
seam; the per-language kernel is bounded by AST-analysis techniques
but the seam — the seam is what mirror adds.*

*Nothing in the @epistemologic/property chain is invented here.
Every property is a wikilink. The chain is canonical. The substrate
is self-disciplined; this spec is what binds the discipline at the
structural edge.*

*e^(n+1) < e^(n). The qualifier set narrows. The glass clears.
The verdict settles. The substrate's name and operation converge —
literally, per epistemologic, by construction.*
