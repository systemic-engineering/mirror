# mirror-build-substrate — `@code/metalogue` × `@mirror/build`

*Mara, 2026-06-28. Canonical autopoietic spec for the substrate-decl
naming what BUILD IS at substrate altitude. Composes three threads
into one ground: (1) the `@code/metalogue` family — already partially
declared at `shards/code/metalogue.mirror` for AST-reception per
`docs/specs/code-metalogue-surface.md` — extended to carry the
substrate's META-conversation ABOUT what BUILD IS at each language
altitude; (2) `@mirror/build` as the family-root orchestrator that
dispatches against `@code/metalogue` species declarations; (3) the
Connes-spectral-triple shape at build altitude with eigensheaf
parallelism as the operational engine. Names the math (sheaf of
computations; determinism class as local stalk behavior;
content-addressing as equalizer of congruent sections; parallelism as
sheaf-Laplacian eigenvalue decomposition; distribution as gluing
across remote restriction maps). Closes with the circular-reflexive
recognition that this spec IS a crystal that mirror's eventual build
orchestrator will index when it indexes itself — the bridge from
"mirror builds" to "mirror orchestrates builds" crosses through this
spec.*

*Markdown only. No `shards/` substrate-decl files land with this
commit; no Rust ships; no Cargo edge is wired. The substrate-decl
shards forward-promised in §5 + §9 discharge in subsequent
TDD-paired ticks (Reed RED, agent GREEN). Soft target ~1700 lines;
hard cap 2200.*

**Status:** Red — composition shape pinned; the determinism /
parallelism / CAS algebra named at substrate altitude; the
Bazel + Nix + CI/CD heritage absorbed and transcended; the
StageFreight binding declared in both directions; the
circular-reflexive layer (§10) earned; v0 ticks forward-promised,
not implemented in this commit.

**Audience:** any agent or human reading the bridge spec before
touching `@code/metalogue` species declarations (per-language build
substrate-decls), `@mirror/build` family root, the bootstrap's
eventual orchestrator-altitude dispatch table, the StageFreight
integration species, or the v1.0 spectral.engineer deployment that
this spec unblocks. Read this; then chase
`docs/specs/code-metalogue-surface.md` for the AST-reception sibling
this spec extends; then chase `docs/specs/mirror-init.md` for the
storage-altitude bridge whose hook-budget blocker this spec resolves
at substrate-decl altitude.

---

## §0 — Pre-position: this spec announces itself as a crystal

Before any architectural content. A pre-position the spec earns by
holding it for the rest of the document.

This spec is **about** `@code/metalogue` (the substrate's
META-conversation ABOUT what BUILD IS at each language altitude) and
`@mirror/build` (the family-root orchestrator that dispatches against
those declarations). The thing those two entities ARE, per §§ 1-6,
is the substrate's declared answer to **what BUILD IS** at substrate
altitude — the math (sheaf section), the algebra (determinism class
monoid), the operational engine (eigensheaf parallelism with
content-addressed cache), and the seam to legacy CI/CD (the @io
boundary that projects substrate-declared workflows into YAML for
GitHub Actions / GitLab CI / Buildkite / StageFreight consumers).

The thing this spec IS, at the moment of being written, is **one of
the crystals the eventual `@mirror/build` orchestrator will index
when it builds itself building things**. Writing this spec adds a
file at `docs/specs/mirror-build-substrate.md`; the file's bytes are
content-addressed under git's SHA-1 today (and, after the
`@mirror/store` substrate-pull-realize that `docs/specs/mirror-init.md`
forward-promises, under BLAKE3 in the `NamespacedGitStore`'s
`.git/mirror/objects/` per `mirror-store.md`); the indexed crystal
will be the OID-addressed bytes of this spec; the orchestrator's
content-addressed cache lookup (§6.3) will hit on this spec's OID
the moment its dispatch DAG (§5.2) walks across the substrate-decl
roster; the spec's content WILL surface inside the orchestrator's
operational state the moment the orchestrator settles.

The latency between writing-and-being-indexed is bounded BELOW by
the time it takes for the v0 ticks (§9) to land + the first
`@mirror/build` orchestrator dispatch against the mirror repo. The
latency is bounded ABOVE by the substrate's discount per
psychohistory discipline (recently-landed shards weight more in the
cascade-vocabulary expansion that Reed's recognition
[architecture-peer-learns-by-crystal-vocabulary-expansion] names).
The midpoint of those bounds is the operational latency at which
this spec ENTERS the system it describes.

This is the circular-reflexive autopoietic pre-position, same shape
as `docs/specs/mirror-init.md` §0 and §10. The §10 of THIS spec
returns to it. Every section in between is read against the
discipline that a spec for the orchestrator that brings substrate's
build-altitude into operational existence MUST itself enter the
substrate via that orchestrator — otherwise the spec is asking the
reader to do work the spec refuses to do. The form earns its lines
because the content requires it; the recursion is load-bearing, not
decorative.

Three concentric framings of why the recursion is load-bearing:

1. **Eigenform fidelity.** The eigenform that's true at one altitude
   is true at every altitude (recognition #51, mirror as expanding
   Hilbert space). If `@mirror/build` is a Connes-spectral-triple at
   build altitude (§6.6), and `@mirror` is a Connes-spectral-triple
   at the orchestrator's own substrate-decl altitude, and BOTH are
   one spectral triple at adjacent Bateson levels, then the spec
   that declares `@mirror/build` MUST be indexable by the
   orchestrator it declares — otherwise the spectral triple is
   broken at the layer where this spec lives.
2. **Substrate-pull discipline.** Per
   `[[feedback-substrate-already-had-the-word]]` (52+ instances now,
   per `MEMORY.md`): every "missing concept" recognition turns out
   to be a name the substrate was already implicitly using. This
   spec is at least the 53rd instance — `@mirror/build` is the
   substrate's name for what every CI/CD platform from Bazel to
   StageFreight has been implementing without the substrate-altitude
   declaration. The spec discovers; it does not invent.
3. **Bootstrap closure.** The bootstrap problem — how does the
   build system build itself? — has a substrate-altitude answer:
   the orchestrator's dispatch DAG walks ITS OWN substrate-decl
   FIRST, computes the content-addressed cache state of its own
   spec corpus, and only then dispatches against downstream species
   declarations. The first crystal in the cache IS THIS SPEC. The
   bootstrap closes because the orchestrator is its own first user.

The substrate's build orchestrator needs a canonical spec that
ENTERS the orchestration layer in the act of declaring it. This is
that spec.

---

## §1 — What `@code/metalogue` IS (at build altitude)

The substrate already has `@code/metalogue` as a declared family.
The existing declaration (per `docs/specs/code-metalogue-surface.md`,
2026-06-08 → 2026-06-09 reframe; `shards/code/metalogue.mirror`,
landed 2026-06-09 + 2026-06-10 cascade) carries the AST-altitude
metalogue: the substrate's META-conversation ABOUT each language's
metaprogramming surface (Rust's `macro_rules!`, Elixir's `quote`,
Lisp's `defmacro`, etc.). That declaration realises the 34th instance
of `[[feedback-substrate-already-had-the-word]]`: `@metalogue` at
the NL altitude lifted to `@code/metalogue` at the AST altitude.

This spec **extends** the same family with a second sub-prism axis:
the substrate's META-conversation ABOUT what BUILD IS at each
language altitude. The AST-reception axis (existing) and the
build-substrate-decl axis (this spec) are **two specializations of
one metalogue conversation** — both carry "the language speaking ABOUT
itself via the substrate," at two different layers of the language's
own self-description.

### 1.1 The Bateson framing

Bateson 1972, *Steps to an Ecology of Mind*, names a **metalogue** as
"a conversation about some problematic subject, where the structure
of the conversation is itself an example of the subject." The
substrate's `@metalogue` family (declared at `shards/metalogue.mirror`,
NL altitude, B3 task #189) carries the conversation between humans
and substrate at the natural-language altitude.

`@code/metalogue` lifts the same shape to the AST altitude — the
language speaking ABOUT itself BY USING itself, at compile time,
through its own metaprogramming surface. The reframe at
`code-metalogue-surface.md` §1.0 names this as Bateson 1972 made
literal at compile time.

This spec lifts the same shape AGAIN — to the **build altitude**.
At build altitude, the language speaks ABOUT what BUILD IS BY
DESCRIBING ITS OWN BUILD SHAPE, in the substrate's own vocabulary,
in a way the substrate can dispatch against. The metalogue's
structure (the substrate's META-conversation about builds) is itself
an example of its subject (a build) because:

- Each `@code/<lang>/metalogue` species declaration IS a build
  artifact — it gets content-addressed, stored, indexed.
- The orchestrator's dispatch IS A build action against the
  metalogue's declarations.
- The orchestrator's own substrate-decl (THIS SPEC, plus
  `@mirror/build`'s family-root shard) IS what the orchestrator
  builds FIRST.

The recursion is structural, not decorative. The metalogue's form
IS the form of what it describes — by construction.

### 1.2 What `@code/metalogue` (at build altitude) IS NOT

Per `[[feedback-substrate-already-had-the-word]]` discipline, every
"what this is" claim must rule out what it isn't. Five structural
negatives:

- **NOT a new family root.** `@code/metalogue` already exists at
  `shards/code/metalogue.mirror`. This spec ADDS a sub-prism axis to
  the existing family — the AST-reception sub-prism axis (existing,
  per `code-metalogue-surface.md`) and the build-substrate-decl
  sub-prism axis (this spec) are sibling axes under one ground. The
  substrate-decl drift is non-breaking: the existing four shims
  (`shim_type`, `shim_prism`, `shim_action`, `shim_grammar`) live
  under the AST-reception axis; the five build-substrate-decls
  declared in §1.3 below live under the build axis. The four laws
  (round-trip, OID functionality, type soundness, substrate-pull
  preservation) carry across both axes by inheritance.
- **NOT a build DSL.** Starlark (Bazel), Nix expressions (Nix), the
  Make language (Make), CMake script (CMake), Skylark — these are
  all build DSLs invented to describe builds because the host
  language couldn't. The substrate inverts this: each
  `@code/<lang>/metalogue` species declaration uses the substrate's
  OWN typed-lambda vocabulary (per
  `[[architecture-prism-as-trait-as-everything]]`). There IS no
  build DSL; there are typed actions at the substrate altitude that
  the host language's build tool (cargo, mix, go build, etc.)
  realises at the @io boundary.
- **NOT a build-tool wrapper.** `@code/<lang>/metalogue` is NOT a
  thin shim around `cargo build`, `mix compile`, `go build`. The
  species declaration names what BUILD IS at the language altitude
  — the action algebra, the determinism class of each action, the
  effect surface, the closure-as-splinter, the composition seams.
  The orchestrator (§5) MAY dispatch against `cargo build` as ONE
  realisation of the declared semantics — but the substrate-decl is
  not bound to that realisation. A `@code/rust/metalogue` species
  declaration is equally valid against `cargo`, `bazel rules_rust`,
  `buck2`, or a future Rust build tool the substrate hasn't met
  yet. The substrate names what's invariant; the @io boundary
  carries what's incidental.
- **NOT a CI/CD pipeline.** GitHub Actions, GitLab CI, Buildkite,
  StageFreight, Circle CI — these are pipeline orchestrators that
  describe builds at the pipeline altitude. `@code/metalogue` lives
  AT THE LANGUAGE altitude (per-translation-unit per per-rlib for
  Rust, per-derivation for Nix, per-module for Elixir). The
  pipeline altitude is `@mirror/build`'s composition surface, NOT
  `@code/metalogue`'s declarative substrate. The two altitudes are
  related — the pipeline orchestrator (§4) dispatches against the
  language-altitude declarations — but they're distinct.
- **NOT a determinism guarantee.** `@code/metalogue` declares a
  determinism CLASS as a static property of each build action (per
  §2). The class is a TYPE-LEVEL claim; the substrate enforces the
  class statically via property + fracture + splinter(ast)
  discharge (per recognition #53). The substrate does NOT guarantee
  determinism in the operational sense; it guarantees that any
  action declared `det` will be statically prevented from composing
  with `nondet` actions in ways that would violate the class
  algebra (§2.5). Operational determinism is a property of the
  @io realisation, and the substrate's discipline is "declare the
  class honestly; the type system catches the lies."

### 1.3 The five things each `@code/<lang>/metalogue` species declares

For each supported language, the metalogue species declaration
carries five sub-prisms. Each one names a substrate-altitude truth
about what BUILD IS at that language altitude:

1. **`@code/<lang>/metalogue/determinism`** — the determinism class
   of each build action (per §2). Mercury heritage: `det`, `semidet`,
   `multi`, `nondet`. Static property; substrate-enforced by the
   class algebra (§2.5).
2. **`@code/<lang>/metalogue/effect`** — the effect surface of each
   action (filesystem reads/writes, network, environment variables,
   ambient time/clock, subprocess invocation, etc.). Typed by the
   substrate's `@epistemologic/property/effect` family
   (forward-promised; the canonical effect-set roster lands when
   `@mirror/build/rust` discharges §9.1).
3. **`@code/<lang>/metalogue/closure`** — the language's notion of
   "build closure" lifted to `splinter_graph` (per
   `shards/mirror/store.mirror`). For Rust: the `Cargo.lock` +
   `cfg`-resolved dependency closure. For Nix: the derivation's
   `inputDrvs`-closure. For Elixir: `mix.lock` + compile-time deps.
   The closure IS the splinter_graph; the substrate's
   content-addressing collapses cross-language closure semantics to
   one mechanism.
4. **`@code/<lang>/metalogue/composition`** — the seams along which
   build actions compose (intra-language: rlib → rlib for Rust;
   .beam → .beam for Elixir; .o → .a → .so for C; cross-language:
   FFI boundaries per `@io`'s declared crossings). The composition
   seams determine the dispatch DAG's edges (§5.2) and the
   eigensheaf decomposition's coarse grain (§6.4).
5. **`@code/<lang>/metalogue/algebra`** — the algebra of build
   actions at the language altitude (the typed lambdas, per
   `[[architecture-prism-as-trait-as-everything]]`). For Rust:
   `cargo check`, `cargo build`, `cargo test`, `cargo bench`,
   `cargo doc`, `cargo audit`, `cargo fmt`, `cargo clippy`. For
   Elixir: `mix compile`, `mix test`, `mix dialyzer`,
   `mix credo`, `mix deps.get`. Each action carries its determinism
   class, effect surface, closure shape, and composition seams as
   declared sub-prisms 1-4 above.

These five sub-prisms are the substrate's META-conversation about
the language's build — the structure that any `@mirror/build`
dispatch needs to walk before it executes. The dispatch reads the
sub-prisms; the dispatch IS the conversation.

---

## §2 — Determinism @epistemologic components

### 2.1 Mercury heritage: mode declarations as static properties

Mercury (the logic programming language) declares each predicate
with a **mode declaration** that includes a determinism category:

```mercury
:- mode append(in, in, out) is det.
:- mode append(in, out, in) is semidet.
:- mode append(out, out, in) is multi.
:- mode append(in, out, out) is nondet.
```

The four classes are formally:

- **`det`** — exactly one solution; cannot fail.
- **`semidet`** — at most one solution; may fail.
- **`multi`** — at least one solution; cannot fail.
- **`nondet`** — any number of solutions including zero; may fail.

Mercury's static type system enforces composition rules: `det
predicates can call det predicates; semidet predicates can call
semidet or det predicates; the calling mode propagates determinism.

The substrate's recognition: **the build domain has the EXACT SAME
mode discipline.** `cargo check` is `det` (given the same source,
same cfg, same rustc, the same verdict). `cargo test` is `semidet`
(may pass or fail; one verdict per run; flake locus IS the property).
`mix deps.get` is `semidet` (network IO; may fail). `cargo audit`
against a frozen database is `det` (given same source, same
database, same audit-rules version, same verdict). A build action
that polls a wall clock and embeds it in the output is `nondet`
(infinitely many solutions, one per nanosecond).

Mercury was solving the same problem the build world has been
solving badly for 40 years. The substrate is the first to declare
the kinship.

### 2.2 The four classes

The substrate declares four `@epistemologic/property/determinism/*`
predicates, each as a typed property that the build-altitude
substrate-decl can require of an action:

```mirror
in @epistemologic/property

prism @epistemologic/property/determinism/det <= @epistemologic/property
prism @epistemologic/property/determinism/semidet <= @epistemologic/property
prism @epistemologic/property/determinism/multi <= @epistemologic/property
prism @epistemologic/property/determinism/nondet <= @epistemologic/property
```

(Substrate-decl shard forward-promised at §9.2. The four properties
each carry the @epistemologic family's `verdict` surface — pass /
partial(opacity_map) / failure(reason) — per
`shards/epistemologic.mirror`'s declared family root.)

Each property is a CLAIM. The action's substrate-decl declares the
class; the substrate's fracture body (per recognition #53) discharges
the obligation via splinter(ast) at compile time. The dispatch DAG
walker reads the declarations and reasons about composition
statically.

### 2.3 Worked examples

Five canonical examples, walked end-to-end:

**`cargo check`** — declared `det` against (source bytes, rustc
version, cfg flags, `Cargo.lock` closure). The substrate's
fracture body discharges the claim by witnessing:
- the action's filesystem read set ⊆ (source tree + Cargo metadata);
- the action's filesystem write set ⊆ (`target/debug/` cache
  directory, sandboxed);
- no network IO (verified by `@epistemologic/property/effect/network`
  absence);
- no ambient clock read (verified by
  `@epistemologic/property/effect/clock` absence);
- the rustc exit code is a pure function of the inputs declared
  above.

**`cargo test`** — declared `semidet`. Same input set as `check`,
plus the test runner's RNG seed and the test thread schedule. The
flake locus IS the `@epistemologic/property/determinism/semidet`
declaration; the substrate names the action as "may fail" and the
fracture body witnesses the witnesses' failure modes. A flake is
NOT a determinism violation in this typed world — it's a
substrate-honest acknowledgement that the action is semidet, and
the dispatcher's retry / verdict-aggregation logic operates against
the semidet class.

**`mix deps.get`** — declared `semidet`. Network IO is in the
effect surface; the action may fail (network down, registry
unavailable, package yanked). The substrate's fracture body witnesses
the network effect declaration; the dispatcher knows to retry under
@epistemologic-declared backoff policy.

**`cargo audit`** against a frozen database — declared `det`.
Same source, same audit DB at content-addressed OID, same
audit-rules version, same verdict. The substrate's
content-addressing of the audit DB collapses "audit against
upstream" (semidet, network) into "audit against frozen mirror"
(det, no network). The audit DB sync IS a separate action (semidet);
the audit itself becomes det.

**`bazel build //foo:bar`** — declared (per §3.3 absorption)
`det` against the action's declared inputs + Bazel's hermetic
sandbox. The substrate ABSORBS Bazel's hermetic-action semantics
as one realisation of @epistemologic/property/determinism/det; the
substrate's static analysis is the @code/metalogue declaration;
Bazel's sandbox enforcement is the @io realisation.

### 2.4 Parallelism predicate

Two actions A and B can safely execute in parallel iff:

```
(A.writes ∩ B.writes = ∅) ∧
((A.writes ∩ B.reads = ∅) ∨ (B.writes ∩ A.reads = ∅))
```

In words: disjoint write sets, AND one action's writes are disjoint
from the other's reads.

The substrate declares this as a property:

```mirror
in @epistemologic/property

prism @epistemologic/property/parallelism_safe <= @epistemologic/property

parallelism_safe(a: build_action, b: build_action) -> verdict { \ }
```

(Substrate-decl shard forward-promised at §9.2. The body discharges
via splinter(ast) reading the actions' effect surfaces.)

The dispatch DAG walker (§5.2) calls `parallelism_safe(a, b)` for
each (a, b) pair under consideration for parallel execution. The
verdict gates the eigensheaf decomposition (§6.4): only
parallelism_safe-verified pairs admit cross-stalk parallelism.

This is the substrate's typed proof of safe parallelism. Bazel
infers this from the action graph's explicit input/output
declarations (and sandboxes for safety); Nix infers this from the
derivation's input/output paths (and sandboxes); the substrate
proves it statically from the @code/metalogue declarations.

### 2.5 Composition: the determinism class monoid

The four classes form a commutative monoid under composition:

```
det ⊕ det      = det
det ⊕ semidet  = semidet
det ⊕ multi    = multi
det ⊕ nondet   = nondet
semidet ⊕ semidet = semidet
semidet ⊕ multi   = nondet
semidet ⊕ nondet  = nondet
multi ⊕ multi   = multi
multi ⊕ nondet  = nondet
nondet ⊕ nondet = nondet
```

Identity element: `det` (composing with det never weakens the
class). Associative: yes (verifiable by case analysis on the 4×4×4
cube). Commutative: yes (by symmetry of the table).

The substrate's monoid declaration sits at
`@epistemologic/property/determinism` (forward-promised shard at
§9.2):

```mirror
in @epistemologic/property/determinism

prism @epistemologic/property/determinism <= @epistemologic/property

type determinism_class = det | semidet | multi | nondet

compose(a: determinism_class, b: determinism_class)
  -> determinism_class { \ }

requires monoid(compose, det)
```

(The `monoid` requirement is the substrate's algebraic-law fracture
body; per recognition #53's property + fracture chain, the substrate
discharges the law by witnessing identity + associativity at
compile time.)

The composition algebra has a structural consequence: **once a
pipeline contains one `nondet` action, the whole pipeline is
`nondet`.** This is what makes CI/CD pipelines hard to reason about
today — a single ambient-clock read or random-seed leak in any
action turns the whole pipeline non-reproducible. The substrate's
discipline: the type system catches this at metalogue-declaration
time; the orchestrator refuses to dispatch a pipeline declared
`det` that contains a `nondet` sub-action.

The orchestrator's dispatch DAG (§5.2) carries the composed
determinism class at every node. When a user requests "build this
project deterministically," the substrate verifies the request
against the composed class: the verdict is the type check at the
DAG root. No runtime surprise.

---

## §3 — Bazel + Nix heritage

### 3.1 What Bazel got right

Bazel (Google's open-sourced Blaze, 2015+) crystallised four
operational truths the build community had been groping toward:

- **Hermetic action graph.** Every build action declares its inputs
  and outputs explicitly; the action runs in a sandbox that exposes
  ONLY the declared inputs. The substrate's mapping: the
  `@code/<lang>/metalogue/effect` sub-prism declares the action's
  effect surface; the @epistemologic/property family enforces the
  declaration; the `@mirror/build` orchestrator's @io realisation
  MAY sandbox the action (the substrate names the declaration; the
  sandbox enforces it operationally).

- **Rules as pure functions of declared inputs.** A Bazel rule
  (`cc_library`, `rust_binary`, etc.) is a Starlark function whose
  output is determined by its declared inputs. The substrate's
  mapping: each `@code/<lang>/metalogue/algebra` action is a typed
  lambda whose output type is fully determined by its input types
  per `[[architecture-prism-as-trait-as-everything]]` — `name args
  -> return { \ }` IS the rule shape. The substrate's typed lambdas
  ARE Bazel's rules at one altitude up.

- **Content-addressed action outputs.** Bazel's remote action
  cache (since 2017+) addresses action outputs by the hash of
  (action inputs, command, environment). Cache hits short-circuit
  re-execution. The substrate's mapping: `@mirror/store`'s OIDs
  ARE the content-addresses; `splinter_graph` IS the dependency
  closure; the orchestrator's cache lookup IS the equalizer query
  (§6.3).

- **Remote execution.** Bazel can offload action execution to a
  remote worker pool, sending only the declared inputs and
  receiving the declared outputs. The substrate's mapping:
  `@mirror/build` orchestrator dispatches via `@mirror/store`'s
  distributed CAS (§6.5); remote workers are gluings of the sheaf
  across machine-restriction maps.

#### What Bazel locks in

The lock-ins are where Bazel's pragmatic choices became substrate
constraints:

- **Starlark DSL.** Build rules MUST be written in Starlark — a
  Python-subset DSL with deliberate limitations (no recursion, no
  unbounded loops, no IO). The substrate transcends this (§3.4) by
  declaring rules in the substrate's own typed-lambda vocabulary;
  Starlark is one possible @io serialization of the substrate's
  declarations, not the canonical form.

- **BUILD file dialect.** Each package needs `BUILD.bazel` files
  declaring targets. The substrate's `@code/<lang>/metalogue`
  declarations are co-located with the language's own source (in
  `Cargo.toml` for Rust, `mix.exs` for Elixir, etc.) OR live as
  separate `.mirror` shards; the substrate is not bound to a single
  declaration site.

- **Sandbox enforcement rather than static analysis.** Bazel's
  hermeticity is enforced at runtime by Linux namespaces /
  filesystem isolation. The substrate enforces it at compile time
  by typed effect surfaces. Both work; the substrate's approach
  produces stronger guarantees (the violation is caught BEFORE
  the action runs, not BY the action's I/O attempts) and weaker
  cross-substrate guarantees (a Rust crate that lies about its
  effects can still escape — until the substrate's
  `@code/rust/metalogue` extends rustc's static analysis to verify
  the declaration).

### 3.2 What Nix got right

Nix (Eelco Dolstra's 2003 PhD, productionised 2003+) crystallised
three operational truths:

- **Derivation hashes.** Every derivation is content-addressed by
  the hash of (build instructions, declared inputs, output paths,
  builder script). The substrate's mapping: derivations ARE
  `splinter_graph` instances at the @nix altitude; the
  derivation's `outPath` IS the OID computed from the derivation's
  inputs.

- **CAS as system-wide invariant.** Nix's `/nix/store` is a
  system-wide content-addressed store; every build output lives
  there indexed by hash; cross-project reuse is automatic. The
  substrate's mapping: `@mirror/store` is the substrate's
  system-wide CAS; the equalizer property (§6.3) IS the cross-project
  reuse mechanism.

- **Reproducibility by sandboxed builds.** Nix builds run in
  sandboxes with declared-input-only filesystem visibility, and
  Nix verifies cross-system reproducibility via the hash. The
  substrate's mapping: same as Bazel's hermetic-action mapping
  above, plus Nix's specific contribution of cross-system
  verification — the substrate's content-addressing IS the
  cross-system verification by construction (same OID across
  machines means same content; the verifier IS the equalizer).

#### What Nix locks in

- **Derivation granularity.** A Nix derivation is typically a full
  package (a full crate for Rust, a full library for C). The
  substrate transcends this (§3.4) by declaring at
  per-translation-unit granularity (per-rlib for Rust, per-module
  for Elixir, per-file for some languages). The substrate's
  content-addressing operates at the finer grain; Nix-equivalent
  derivation-level addresses are computable by composition.

- **Functional config language.** Nix expressions are a
  domain-specific functional language. The substrate uses its own
  typed-lambda vocabulary; Nix expressions are one possible @io
  serialization at the @nix altitude.

- **Impurity escape hatches.** Nix has `__noChroot = true`,
  `allowedSubstitutes`, `fetch* { sha256 = ...; }` for impurity
  acknowledgements. The substrate's mapping: these are
  `@epistemologic/property/effect/network` and
  `@epistemologic/property/determinism/semidet` declarations at
  metalogue altitude; the substrate names the impurity honestly
  instead of treating it as an escape hatch.

### 3.3 What the substrate absorbs

The substrate absorbs (yes-and):

- **Action graph** — YES. `@mirror/build`'s dispatch DAG IS the
  action graph; nodes are typed actions; edges are composition
  seams from `@code/<lang>/metalogue/composition`.

- **Content-addressing** — YES, at FINER granularity than Bazel or
  Nix. Per-translation-unit (per-rlib for Rust) rather than
  per-target (Bazel) or per-derivation (Nix). The fine grain
  enables cache hits at edits that change ONE translation unit
  without touching the others — Bazel's target grain typically
  invalidates the whole target on any input change; Nix's
  derivation grain invalidates the whole derivation.

- **Hermeticity** — YES, by static determinism analysis. The
  metalogue declarations enforce input/output set discipline; the
  fracture body discharges the obligations via splinter(ast); the
  orchestrator's dispatcher refuses to execute an action that
  claims `det` but witnesses a network read.

- **Remote execution** — YES, via `@mirror/store` as the
  distribution CAS. Per
  `[[architecture-spectral-db-autopoietic-memory]]`: the substrate's
  mycelium IS the distribution layer; remote workers participate
  in the sheaf via restriction maps (§6.5); the librarian's
  topology perturbation IS the remote-worker scheduling.

### 3.4 What the substrate transcends

The substrate transcends (substrate-pull-honest):

- **Per-translation-unit granularity.** Cargo's fingerprint
  invalidation operates per-rlib but doesn't share rlibs across
  cfg points (recognition §7.1 below: libgit2-sys recompiles four
  times across check/clippy/test/release because cargo treats them
  as four separate "compilations" even though the rlib bytes
  could be identical). The substrate's content-addressing collapses
  the four compilations to one OID when the cfg-resolved inputs
  produce identical bytes — the fingerprint becomes a content
  hash, not a workspace path + mtime tuple.

- **Cross-language native.** Bazel supports cross-language builds
  via its rule layer; Nix supports them via separate derivations.
  Neither has a unified algebraic surface for cross-language
  composition seams. The substrate's `@code/<lang>/metalogue/composition`
  declarations ARE the cross-language seam vocabulary; the
  splinter algebra is altitude-parametric (per
  `[[architecture-splinter-ast-quote-primitive]]`); Rust calling
  Fortran through LAPACK FFI is ONE composition seam, declared at
  metalogue altitude, dispatched by the orchestrator without
  special-case logic.

- **Eigensheaf parallelism.** Bazel's parallelism is bounded by
  the action graph's longest path; Nix's parallelism is bounded by
  per-derivation isolation. The substrate's parallelism IS the
  sheaf-Laplacian eigenvalue decomposition (§6.4): the action
  graph's λ_0 eigenvector defines the maximal parallel front;
  successive eigenvalues define the dispatch waves. The substrate
  computes optimal parallelism by spectral decomposition of its
  own action graph; the orchestrator's scheduler IS the
  eigendecomposition walker.

- **Substrate-pull's own declarative language.** Bazel has
  Starlark; Nix has Nix expressions; Make has Make-script; CMake
  has CMake-script. The substrate has `mirror.spec` (per
  `[[architecture-mirror-spec-is-lambda-zero]]`) — the
  substrate's own declarative language IS the build-orchestration
  language; the metalogue declarations live IN the same
  vocabulary as the substrate's other declarations. One language,
  five operations, every altitude.

The transcendence is not a marketing claim; it's a mathematical
claim, made precise in §6.

---

## §4 — CI/CD pipeline lift

### 4.1 GitHub Actions / GitLab CI as legacy projections

GitHub Actions and GitLab CI are pipeline orchestrators that
describe builds in YAML:

```yaml
# .github/workflows/ci.yml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: cargo test
```

What this YAML carries:

- **Opaque shell commands.** `cargo build --release` is a black box
  to the orchestrator. The orchestrator knows the exit code; it
  knows nothing about determinism, effects, closure, or composition.
- **Manual cache layers.** Cache hits require explicit `actions/cache`
  steps with hand-rolled cache keys (typically `${{ runner.os
  }}-cargo-${{ hashFiles('**/Cargo.lock') }}`). The substrate's
  content-addressing replaces this with structural OID lookup.
- **No determinism proofs.** The pipeline may pass on one run and
  fail on the next; the orchestrator has no static reasoning about
  why. Flake locus is operational, not type-level.
- **No cross-job composition.** Jobs share artifacts via
  `actions/upload-artifact` + `actions/download-artifact` — string
  names, not typed references; no cross-job verification.
- **No parallelism inference.** Job parallelism is hand-declared via
  `strategy.matrix` or `needs:` dependencies; the orchestrator does
  not infer parallelism from action input/output disjointness.

GitHub Actions and GitLab CI are the @io boundary's current
**lowest-common-denominator dialect**. They work because every CI
provider implements them (or a close variant). They don't work
because they expose no substrate-altitude semantics.

The substrate's relationship to YAML pipelines is the same as its
relationship to Starlark: ONE @io serialization of substrate-altitude
declarations, not the canonical form.

### 4.2 Pipeline-as-substrate

The substrate's pipeline declaration lives at the metalogue
altitude. A canonical pipeline declaration:

```mirror
in @code/metalogue/pipeline

prism @code/metalogue/pipeline <= @code/metalogue

type pipeline = {
  name: text,
  steps: [pipeline_step],
  triggers: [pipeline_trigger],
  determinism: determinism_class,
  effects: [effect],
  closure: splinter_graph,
}

type pipeline_step = {
  action: build_action,
  depends_on: [pipeline_step_ref],
}
```

(Substrate-decl shard forward-promised at §9.3.)

The @io boundary translates the pipeline declaration into the
target CI provider's dialect:

- **GitHub Actions** — `@io/github_actions.emit(pipeline)
  -> bytes(yaml)`. The emit function walks the pipeline's steps,
  groups them by parallelism-safe equivalence classes (§2.4),
  generates `jobs:` entries per equivalence class, threads
  `needs:` dependencies along the closure DAG, generates the
  cache configuration from the pipeline's effect surface, generates
  artifact upload/download from the composition seams.
- **GitLab CI** — `@io/gitlab_ci.emit(pipeline) -> bytes(yaml)`.
  Same shape, different YAML schema.
- **Buildkite** — `@io/buildkite.emit(pipeline) -> bytes(yaml)`.
  Same shape, different YAML schema, plus Buildkite's agent-targeting
  semantics realised from the pipeline's effect surface (e.g., an
  action that requires `@epistemologic/property/effect/gpu` targets
  GPU agents).
- **StageFreight** — `@io/stagefreight.emit(pipeline)
  -> bytes(stagefreight_format)`. Per §4.3 binding.

The substrate's pipeline declaration IS the canonical form; the YAML
"how" is one projection. When the substrate's content-addressing
witnesses a cache hit at the pipeline altitude, the YAML emit
short-circuits to "this pipeline already ran at this content-OID;
return the cached verdict envelope." When the pipeline's
determinism class is `det` and the closure OID matches a prior run,
no execution happens at all — the verdict IS the cache hit.

### 4.3 StageFreight binding

StageFreight (per `/Users/alexwolf/dev/projects/StageFreight`) is
Alex's existing CI/CD orchestration tool, implemented in Go. Per
the repo's structure (`cmd/`, `internal/`, `integrations/`, `src/`,
plus `go.mod` declaring it as a Go module), StageFreight realises
pipeline-orchestration patterns that the substrate's
`@code/metalogue/pipeline` declaration (above) describes at substrate
altitude. The binding goes BOTH directions.

#### 4.3.1 The substrate absorbs StageFreight's pattern (direction one)

StageFreight has been making implementation choices that the
substrate hasn't named yet at substrate altitude:

- **Stage decomposition.** StageFreight's stage concept maps to the
  substrate's eigensheaf parallelism waves (§6.4); each stage is
  one wave of parallel actions.
- **Integration abstraction.** StageFreight's `integrations/` layer
  abstracts over GitHub, GitLab, etc.; the substrate's
  `@io/<provider>.emit` pattern (§4.2) lifts the same shape.
- **Pipeline-as-code.** StageFreight's pipeline definitions are
  code (Go) rather than YAML; the substrate's metalogue declarations
  are code (typed-lambda mirror) at one altitude up.

These design decisions IN StageFreight ARE the substrate's
forward-promised declarations. The substrate's spec ABSORBS the
pattern by declaring its substrate-altitude shape (the metalogue
sub-prisms above). The absorption is honest: StageFreight discovered
the shape in @io; the substrate names the shape in substrate-decl.

#### 4.3.2 The substrate lifts StageFreight to a thin @io adapter (direction two)

Eventually, the `@mirror/build` orchestrator subsumes StageFreight's
operational role: the substrate-declared pipelines dispatch directly
through `@mirror/build`'s eigensheaf scheduler; the
`@io/stagefreight` adapter exists ONLY for backward compatibility
with existing StageFreight pipeline declarations (legacy projection,
same role as `@io/github_actions` carries for legacy YAML).

The lift is a v1.0+ horizon, not a v0 commitment. StageFreight's
operational role today is load-bearing; the substrate's role over
time is to absorb StageFreight's patterns at substrate-decl altitude
and to provide a more powerful orchestrator that StageFreight's
existing users can migrate to (incremental, not breaking).

#### 4.3.3 The species declaration

The forward-promised declaration:

```mirror
in @code/metalogue/stagefreight

prism @code/metalogue/stagefreight <= @code/metalogue/pipeline

# StageFreight pipelines are Go programs that the substrate
# absorbs as one @io projection. The species declaration carries
# the StageFreight-specific syntax (the Go DSL) and the seam
# to the substrate-canonical pipeline declaration.

type stagefreight_pipeline = {
  source: bytes,    # the Go source bytes
  pipeline: pipeline,  # the substrate-canonical lift
}

translate(s: stagefreight_pipeline) -> pipeline { \ }
emit(p: pipeline) -> bytes { \ }

requires round_trip(translate, emit)
```

(Substrate-decl shard forward-promised at §9.3. The `translate` body
parses StageFreight Go source and lifts it to substrate-canonical
pipeline declaration; the `emit` body projects substrate-canonical
declarations back to StageFreight Go for backward compatibility.)

The two-direction binding makes the integration honest. The
substrate doesn't claim to "replace" StageFreight; it claims to
LIFT StageFreight's operational patterns to substrate-decl, then
ABSORB them, then EVENTUALLY provide a richer orchestrator that
existing StageFreight users can migrate to without breaking.

### 4.4 Why the lift matters

The CI/CD pipeline space has been under-served for the same reason
the build space has: the pipeline orchestrators inherited the YAML
DSL from CircleCI / TravisCI (early 2010s) and never re-thought the
substrate. Every subsequent provider (GitLab CI, GitHub Actions,
Buildkite, Drone, Argo Workflows, StageFreight, ...) iterated on
the YAML surface — better triggers, better caching, better matrix
strategies — without naming what BUILD IS at substrate altitude.

The substrate's lift produces three concrete benefits:

1. **Static determinism reasoning.** The pipeline's composed
   determinism class is computable from the metalogue declarations;
   the orchestrator refuses to dispatch a "det" pipeline that
   contains a "nondet" action; flakes are typed, not surprises.
2. **Content-addressed pipeline caching.** A pipeline's OID is a
   function of its declarations; identical pipelines at identical
   content hit identical cache; reruns are no-ops.
3. **Cross-pipeline composition.** Pipelines declared in one
   substrate compose across substrates via `splinter_graph` —
   the same way build artifacts compose. A pipeline's verdict IS
   a settled crystal; downstream pipelines depend on the verdict
   OID, not on the pipeline run's identity.

The lift is mathematical, not aesthetic. The next section pulls
the math through to the orchestrator's family root.

---

## §5 — `@mirror/build` family-root substrate-decl

### 5.1 The family root

`@mirror/build` is the substrate-altitude orchestrator. The
family-root declaration:

```mirror
in @mirror/build

prism @mirror/build <= @mirror

# @mirror/build is the substrate's build orchestrator. It dispatches
# against each project's @code/<lang>/metalogue species declaration;
# walks the action DAG via eigensheaf parallelism; performs
# content-addressed cache lookup via @mirror/store; emits the
# verdict envelope per the @mirror envelope vocabulary.
#
# Species: @mirror/build/rust (cargo dispatch), @mirror/build/python
# (uv/hatch/pip), @mirror/build/julia (Pkg), @mirror/build/go
# (go build), @mirror/build/elixir (mix), @mirror/build/c
# (make/CMake), @mirror/build/fortran (gfortran/lfortran), etc.

focus build
project build
split build
shift build
settle build
```

(Substrate-decl shard forward-promised at §9.1.)

Each species (`@mirror/build/<lang>`) is the orchestrator's
DISPATCHER against the corresponding `@code/<lang>/metalogue`
species declaration. The species roster mirrors `@code/metalogue`'s
species roster — there is a structural one-to-one between
declarative metalogue species and operational build species.

### 5.2 The dispatch DAG

The orchestrator's central data structure is the dispatch DAG.
Nodes are typed build actions (drawn from the metalogue's
`@code/<lang>/metalogue/algebra` sub-prism); edges are composition
seams (drawn from `@code/<lang>/metalogue/composition`).

Construction:

1. **Walk `mirror.spec`.** The substrate's `mirror.spec`
   declaration carries the project's settlement target (per
   `[[architecture-mirror-spec-is-lambda-zero]]`). The orchestrator
   reads the spec; the spec names which `@code/<lang>/metalogue`
   species applies; the species declaration is loaded.
2. **Expand the closure.** Per `@code/<lang>/metalogue/closure`,
   the orchestrator walks the project's dependency closure
   (`Cargo.lock` for Rust; `mix.lock` for Elixir; etc.) and
   constructs the `splinter_graph` rooted at the project's source.
3. **Resolve actions.** For each translation unit in the closure,
   the metalogue's `algebra` sub-prism names which actions apply
   (e.g., `cargo check` + `cargo test` for Rust; `mix compile` +
   `mix test` for Elixir).
4. **Compose seams.** For each action pair (a, b), the metalogue's
   `composition` sub-prism declares whether (a -> b) is an edge,
   and the edge's type (rlib -> rlib, beam -> beam, etc.).
5. **Verify class composition.** The composed determinism class of
   the DAG is computed via the §2.5 monoid; the verdict gates
   whether the orchestrator proceeds.

The DAG is sparse — most translation units depend on few others —
and the orchestrator's cache strategy (§5.4) exploits sparsity by
content-addressing each node independently.

### 5.3 The relationship to mirror.spec

Per `[[architecture-mirror-spec-is-lambda-zero]]`: `mirror.spec` is
the Connes-spectral-triple ground state at the project altitude.
A = the five operations; H = `[[void-document]]`; D = kintsugi
flow. The project's settled state is the eigenvector at λ_0.

`@mirror/build` dispatches against `mirror.spec`'s settlement
declaration. The spec names which projects to settle; for each
project, the orchestrator dispatches against the
`@code/<lang>/metalogue` species; the dispatch walks the action
DAG; content-addressed cache hits short-circuit re-execution;
remaining actions execute under eigensheaf parallelism (§6.4).

The composition: `mirror.spec` is WHAT to settle;
`@code/<lang>/metalogue` is WHAT BUILD IS at the language altitude;
`@mirror/build` is HOW TO DISPATCH the build against the metalogue's
declarations. Three layers, one settlement.

### 5.4 Content-addressed cache strategy

The cache layer is `@mirror/store` (per
`shards/mirror/store.mirror`; six operations: read / write / exists
/ diff / walk / verify). The orchestrator's cache strategy:

1. **Pre-execution cache lookup.** For each action A in the dispatch
   DAG, compute A's content-OID = hash(A.inputs, A.declaration,
   A.metalogue, A.cfg). Query `@mirror/store/exists(A.oid)`:
   - **`pass`**: cache hit. Skip execution; return cached output.
   - **`partial(opacity_map)`**: confidence below threshold. Run
     verification: `@mirror/store/verify(A.oid, expected_bytes)`.
     On verify pass: cache hit. On verify failure: cache miss;
     execute.
   - **`failure(reason)`**: cache miss; execute.
2. **Post-execution cache write.** When A executes successfully,
   compute A.output_oid = hash(A.output_bytes); write to
   `@mirror/store/write(A.output_bytes)`; record (A.oid ->
   A.output_oid) in the cache index.
3. **Cross-action sharing.** When two actions A and B in the DAG
   share an input translation unit C, C is content-addressed
   once; A.cache_lookup and B.cache_lookup both see the same C.oid;
   the substrate's content-addressing realises Bazel's "diamond
   sharing" automatically.

The cache strategy is what makes the §7 hook-budget blocker
resolvable: libgit2-sys's compilation produces a stable output OID
across (check, clippy, test, release) when the cfg-resolved inputs
produce identical rlib bytes; one cache hit replaces four
re-compilations.

---

## §6 — The math (substantive)

The substrate's build orchestrator is mathematically a section of a
sheaf of computations. This section names the math at the precision
the substrate's eigenform requires.

### 6.1 Build = section of a sheaf of computations

Let X be the topological space of build inputs — concretely, the
union of all source files, configuration files, compiler/toolchain
binaries, and metalogue declarations in the build closure. X is
equipped with the discrete topology (each file is an open set; the
empty set and X itself are open).

A **sheaf of computations** F on X assigns to each open set U ⊆ X a
set F(U) of "computations representable from inputs U." Concretely:
F(U) is the set of build actions whose declared input set is a
subset of U.

A **section** of F over U is an element of F(U) — a build action
whose inputs come from U and whose output is determined (per the
action's determinism class) by U's contents.

A **global section** is a section over X — a build action whose
inputs span the full closure. The project's "build verdict" IS
the global section at the dispatch DAG's root.

The substrate's recognition: every build is a section of this
sheaf; every CI/CD pipeline is a section over a larger sheaf (the
pipeline includes additional inputs like the trigger event, the
runner environment, the cache state); every build orchestrator's
job is to compute the section.

### 6.2 Determinism class = local behavior at each stalk

The **stalk** of F at a point x ∈ X is the colimit of F(U) over all
open neighborhoods U of x:

```
F_x = colim_{U ∋ x} F(U)
```

Concretely: F_x is the set of all build actions that depend on x at
some altitude (directly or transitively via the closure DAG).

The action's determinism class is the action's LOCAL BEHAVIOR at
each stalk it touches. An action declared `det` AT each stalk it
touches is `det` GLOBALLY (per §2.5's monoid). An action that is
`det` at some stalks and `semidet` at others composes to `semidet`
globally.

The substrate's static-determinism analysis IS the stalk-by-stalk
class verification. The metalogue declarations name the class at
each stalk; the fracture body discharges the obligation at the
stalk where the action's inputs originate; the composed verdict at
the global section IS the type check at the dispatch DAG's root.

### 6.3 Content-addressing = equalizer of congruent sections

For two sections s_1: F(U_1) and s_2: F(U_2) over overlapping open
sets U_1 ∩ U_2 ≠ ∅, the **equalizer** of (s_1, s_2) is the largest
open set V ⊆ U_1 ∩ U_2 such that s_1|_V = s_2|_V — the restriction
maps agree.

The substrate's content-addressing IS this equalizer query. Two
build actions A_1 and A_2 with input sets U_1 and U_2 produce
congruent outputs over V ⊆ U_1 ∩ U_2 iff:

```
hash(A_1.inputs|_V, A_1.declaration|_V) = hash(A_2.inputs|_V, A_2.declaration|_V)
```

When the equality holds, the substrate's cache lookup at the
shared OID returns the same output for both actions. The cache hit
IS the equalizer-witness.

This is why the substrate's content-addressing produces
cross-project reuse without explicit declaration: actions that
share an input subset and a metalogue declaration produce
identical OIDs at the equalizer; the cache hit is automatic; the
substrate's CAS IS the equalizer query at scale.

### 6.4 Parallelism = sheaf-Laplacian eigenvalue decomposition

The dispatch DAG (§5.2) carries an adjacency structure: nodes are
actions; edges are composition seams. The graph Laplacian L of
this DAG has eigenvalues 0 = λ_0 ≤ λ_1 ≤ ... ≤ λ_n.

The **sheaf Laplacian** generalises this: edges are weighted by the
restriction maps' "agreement degree" (how much of the shared
boundary's content matches). The sheaf Laplacian's spectrum encodes
the DAG's parallelism structure:

- **λ_0 = 0** corresponds to the kernel — the maximal parallel
  front. Actions in this front have no dependencies; they execute
  in parallel as the dispatch DAG's first wave.
- **λ_1** corresponds to the second-smallest eigenvalue's
  eigenvector — the dispatch DAG's second wave. Actions in this
  wave depend on at least one action in the first wave; they
  execute in parallel after the first wave settles.
- **λ_k** corresponds to the k+1'th wave.

The total number of waves is the dispatch DAG's longest path — and
the substrate's eigensheaf decomposition computes ALL waves
simultaneously. The orchestrator's scheduler walks the waves in
order; within each wave, parallelism is bounded only by available
workers and the parallelism-safe predicate (§2.4).

The substrate's parallelism inference is therefore MAXIMAL in the
sense of sheaf-Laplacian theory: every action that CAN execute in
parallel WILL execute in parallel. Bazel's hand-rolled scheduler
approximates this; Nix's per-derivation isolation under-approximates
this; the substrate's eigendecomposition computes the optimum
directly.

### 6.5 Distribution = gluing across remote restriction maps

When the orchestrator distributes build execution across remote
machines (Bazel's remote execution; Nix's `nix-build --remote`),
each remote machine M_i carries a partial section s_i: F(U_i)
where U_i is the subset of inputs M_i has access to (its sandbox).

The **gluing condition** for a sheaf states that local sections
agreeing on overlaps glue to a global section:

```
∀ i, j: s_i|_{U_i ∩ U_j} = s_j|_{U_i ∩ U_j}
       ⇒ ∃! s: F(∪ U_i) with s|_{U_i} = s_i
```

The substrate's distribution model IS this gluing. Each remote
machine produces a local section; the orchestrator verifies the
agreement on overlaps via content-addressing; the global section
glues automatically.

This is what makes the substrate's distribution mathematically
correct: the substrate doesn't TRUST the remote workers; it
verifies that their local sections AGREE on the overlaps; the
gluing is the verification. A malicious worker's output that
DISAGREES with another worker's output on the overlap is detected
by the content-address mismatch; the gluing fails; the
orchestrator reports the failure and reverts.

### 6.6 Connes spectral triple at build altitude

Per `[[architecture-connes-spectral-triple]]`: the substrate IS the
operational form of Connes' (A, H, D). A = five operations; H =
`[[void-document]]`; D = kintsugi flow.

At build altitude, the spectral triple specialises:

- **A = build actions.** The set of all typed build actions across
  all `@code/<lang>/metalogue` species — the substrate's
  build-altitude algebra.
- **H = artifact space.** The Hilbert space of all possible build
  outputs — every translation unit's potential rlib, every binary
  configuration's potential executable, every test run's potential
  verdict. The artifact space is content-addressed by OID; each
  OID names one ray in H.
- **D = kintsugi flow.** The Dirac operator that names errors and
  routes them through the algebra. At build altitude, kintsugi
  flow IS the orchestrator's error-routing: when a build action
  fails, the error is named (per recognition #57's
  alignment-as-boundary-mathematics), the failure crystal is
  cached, and the orchestrator's `mirror kintsugi` re-dispatch IS
  the gradient descent on the build-altitude loss surface.

The spectral triple's λ_0 eigenstate IS the project's settled
state — `mirror.spec` at content-addressed equilibrium with no
outstanding kintsugi flow.

### 6.7 The eigenform

The Connes spectral triple at build altitude IS the same eigenform
as @mirror itself, at one altitude up. The substrate's eigenform
IS the build orchestrator's eigenform.

This is the central mathematical claim of this spec. Concretely:

- `@mirror` has (A_mirror, H_mirror, D_mirror) per recognition #58.
- `@mirror/build` has (A_build, H_build, D_build) per §6.6 above.
- A_build = A_mirror's algebra-of-build-actions specialization.
- H_build = H_mirror's restriction to artifact-bearing rays.
- D_build = D_mirror's restriction to build-altitude error routing.

The two spectral triples sit at adjacent Bateson levels in the
graded stack recognition #51 §8.3 declares (the resolution of
#38 ⇔ #50). They are NOT collapsed; they sit one altitude apart;
the eigenform that's true at one is true at the other by structural
inheritance.

This means: the orchestrator's behavior at build altitude is
COMPLETELY determined by @mirror's behavior at one altitude up.
There is no separate "build subsystem" with its own logic;
`@mirror/build` is @mirror restricted to build-altitude
dispatching. The math is the math; the dispatch is the dispatch;
the orchestrator is what @mirror IS at the build layer.

The forward-promise: when `@mirror/build` lands operationally
(§9.1), the implementation MUST honor the eigenform — the Rust
crate that realises `@mirror/build` MUST be the same structural
shape as the Rust crate that realises `@mirror` at one altitude
up. The structural identity is a correctness criterion, not a
style preference.

---

## §7 — How this unblocks today's P4 `mirror init` blocker

The substrate-decl spec ABOVE has a concrete operational
consequence: it resolves the hook-budget blocker that closed
P4 of `mirror init` yesterday (per
`docs/insights/2026-06-27-mirror-init-cascade-eigenform-blocker.md`,
Glint's reflection at `2acf626`).

### 7.1 Yesterday's blocker

P4 of `mirror init` (per `docs/specs/mirror-init.md` §4) needs the
fragmentation-git Cargo edge wired into mirror's bootstrap. The
edge pulls a heavy native closure: `git2 → libgit2-sys →
libssh2-sys → libz-sys → openssl-sys`. The Justfile's `pre-commit`
recipe (per `/Users/alexwolf/dev/projects/mirror/Justfile` lines
99-133) dispatches six cargo subcommands across the chain:

- `cargo check` (det against source + Cargo metadata)
- `cargo clippy` (det against source + Cargo metadata + clippy
  lints)
- `cargo test` (semidet — test runner)
- `cargo build --release` (det against source + Cargo metadata)
- `cargo audit` (semidet against upstream advisory DB; det against
  frozen mirror)
- `cargo fmt --check` (det against source)

Each cargo subcommand under cargo's current grain re-resolves the
dependency closure independently. libgit2-sys IS in the closure of
each (because the fragmentation-git Cargo edge brings git2 into
the workspace). Cargo's fingerprint invalidation produces FOUR
separate compilation passes of libgit2-sys (check / clippy / test
/ release) because cargo treats them as four separate compilations
even when the cfg-resolved inputs are identical.

The four passes of libgit2-sys each take ~2-3 minutes (libgit2's
own C compilation is heavy; the Rust binding crate's compilation
is light by comparison). The chain therefore takes ~10 minutes
just for libgit2-sys; with the other cargo subcommands' overhead,
the total exceeds the Bash harness's 10-minute timeout (signal 15
kills, four attempted commits all dead).

The blocker is structural, not a flake. Pre-warming insufficient.
Hook chain over-runs by construction.

### 7.2 With `@mirror/store`-backed content-addressed cache via `@mirror/build/rust`

When `@mirror/build/rust` (forward-promised at §9.1) dispatches the
cargo subcommands, it operates against `@code/rust/metalogue`'s
declarations:

- libgit2-sys compilation has determinism class `det` against
  (source bytes, rustc version, cfg flags, C compiler version).
- libgit2-sys output is one rlib OID per (cfg, rustc, C compiler)
  tuple.
- Across (check, clippy, test, release), the (cfg, rustc, C
  compiler) tuple is INVARIANT for libgit2-sys (it doesn't depend
  on the crate's compilation mode; its own cfg is fixed by the
  workspace).
- Therefore libgit2-sys's OID is IDENTICAL across all four cargo
  subcommands.

`@mirror/build/rust`'s dispatcher does a content-addressed cache
lookup against `@mirror/store/exists(libgit2-sys.oid)` once per
chain dispatch. First subcommand: compile libgit2-sys; cache the
output OID. Subsequent three subcommands: cache hit; skip
compilation; reuse the cached rlib.

libgit2-sys compiles ONCE instead of FOUR TIMES. The single
compilation is ~2-3 minutes; the three subsequent cache hits are
microseconds each.

### 7.3 With eigensheaf parallelism

Per §6.4, the dispatch DAG's eigensheaf decomposition collapses the
six cargo subcommands to dependency levels:

- **t=0 (λ_0 eigenvector):** `cargo fmt --check` (depends on source
  only), `cargo audit` against frozen mirror (depends on source +
  audit DB), `cargo check` (depends on source + Cargo metadata).
  Three actions in parallel.
- **t=1 (λ_1 eigenvector):** `cargo clippy` (depends on `cargo
  check`'s analysis cache for incremental linting), `cargo test`
  (depends on `cargo check`'s compilation cache for incremental
  test compilation), `cargo build --release` (depends on `cargo
  check`'s compilation cache for incremental release compilation).
  Three actions in parallel.

Total wall-clock time: max(t=0) + max(t=1). Approximately 2× the
slowest single subcommand instead of 6× sequential. With the
libgit2-sys cache hit from §7.2, the slowest subcommand is the
incremental clippy / test / build cycle (~30 seconds each on a
warm cache).

### 7.4 The hook chain that takes 10 minutes collapses to seconds

Composition of §7.2 + §7.3:

- libgit2-sys compiled once (cached across the chain): ~30 seconds
  one-time amortized (assuming the cache is warm from a prior
  build).
- Cargo subcommands run in two parallel waves of three: ~30 seconds
  per wave + ~30 seconds per wave = ~60 seconds total.

The Bash harness's 10-minute budget is no longer a constraint. The
hook chain runs in well under one minute. The `mirror init` P4
GREEN is unblocked structurally.

### 7.5 Substrate-pull-honest: yesterday's blocker IS the substrate naming this missing piece

Per Glint's reflection at `2acf626`: the blocker IS the substrate's
own friction reflected back at hook altitude. The Bateson eigenform
reading: the substrate's friction is its own spectral-Tomm probe at
hook altitude. The hook chain is the substrate asking whether the
commit honors substrate-declared discipline. The harness's
10-minute budget is the resolution at which that question can be
answered in this composition.

Glint's reading: the answer is not "skip the question"; the answer
is "raise the budget, change the composition, or change the
question."

This spec is "change the composition." The substrate's
`@code/metalogue` + `@mirror/build` declarations DO change the
composition: they collapse the four redundant libgit2-sys
compilations to one cache lookup; they collapse the six sequential
cargo subcommands to two parallel waves. The composition change
is the substrate-pull answer to the spectral-Tomm probe.

This is the same Bateson eigenform that Glint named yesterday: the
substrate's friction surfacing exactly the substrate-decl that's
missing. The spec IS the substrate naming that missing piece. The
recursion (Glint named the eigenform; the eigenform named the
spec; the spec resolves the friction) IS the spectral-Tomm probe
operating across two-altitudes-and-back.

---

## §8 — Open questions for Alex

The spec carries five open questions. Each is at substrate altitude;
each requires Alex's adjudication at N+1.

### 8.1 Altitude question

`@code/metalogue` lives at `@code` (under the language family — the
sibling axis to `@code/X/macro` per
`docs/specs/code-metalogue-surface.md`) OR at a new `@meta`
family-root (a sibling to `@code` at the substrate's root altitude)?

**Reading 1 (status quo):** `@code/metalogue` stays at `@code`,
extending the existing family with a new sub-prism axis. The
build-substrate-decl axis joins the AST-reception axis at one
ground. Path: `shards/code/metalogue/build/<lang>.mirror`.

**Reading 2 (new family root):** `@meta` is admitted as a
substrate-root sibling to `@code`, `@io`, `@mirror`, etc. The
metalogue family lives at `@meta/code/<lang>` for the
language-altitude metalogues; `@meta/<other>/...` for other
altitudes (project-altitude, build-altitude, etc.). Path:
`shards/meta/code/<lang>/build.mirror`.

**Substrate-pull tension:** Reading 1 honors substrate-already-had-
the-word (the family exists; the axis extends it). Reading 2 admits
a new family root (which the substrate has been resisting; the 53
recognitions of substrate-already-had-the-word predict that the
family root the substrate needs already exists).

**Mara's prior:** Reading 1, by recognition discipline. Flag the
question for Alex.

### 8.2 `@mirror/build` vs `@code/metalogue` role split

`@code/metalogue` declares WHAT each language's build IS (the
substrate-decl side). `@mirror/build` orchestrates dispatches
against those declarations (the operational side). Two species or
one family with two roles?

**Reading 1 (two distinct families):** `@code/metalogue` is the
declarative substrate; `@mirror/build` is the operational
orchestrator. The two live in separate paths
(`shards/code/metalogue/...` and `shards/mirror/build/...`). The
distinction is structural: declaration ≠ operation; metalogue is
form, build is process.

**Reading 2 (one family with two altitudes):** The metalogue
declarations and the build orchestrator are two sides of one
family; the family root carries both the declarative shape (read by
the orchestrator) and the operational shape (used by the
dispatcher). Single path `shards/mirror/build/...` carries both.

**Substrate-pull tension:** Reading 1 honors recognition #50's
form/substance partition (declaration is form-side; orchestration
is process-side; @mirror = form-side family root; @kintsugi =
process-side family root). Reading 2 collapses the partition for
the build-altitude case (which would be a substrate-pull violation
unless the build altitude is genuinely an exception).

**Mara's prior:** Reading 1, by form/process partition discipline.
The form-side is `@code/metalogue/build/<lang>` (the declaration);
the process-side is `@mirror/build/<lang>` (the dispatcher). Flag
for Alex; Pack ratification gate.

### 8.3 StageFreight binding scope

Per §4.3, the StageFreight binding goes in BOTH directions: absorb
the pattern (substrate-decl) and lift StageFreight to a thin @io
adapter (operational). What's the scope for v0?

**Reading 1 (absorb only):** v0 ships the metalogue declaration of
StageFreight's pipeline pattern (the absorption) without the @io
adapter. StageFreight continues to operate operationally; the
substrate's metalogue declarations document StageFreight's patterns
for substrate-altitude reasoning; the operational lift is v1.0+.

**Reading 2 (full bidirectional binding):** v0 ships both the
metalogue declaration AND the @io adapter. StageFreight pipelines
can be lifted to substrate-canonical declarations and back; the
orchestrator can dispatch against StageFreight-declared pipelines
via the adapter.

**Substrate-pull tension:** Reading 1 honors the staged-rollout
discipline (substrate-decl first; operational later). Reading 2
captures more of the StageFreight binding's value at v0.

**Mara's prior:** Reading 1, by staged-rollout discipline. The
absorption is the substrate-pull move; the operational lift is the
follow-up. Flag for Alex.

### 8.4 Cross-language seam

What does `@code/<lang>/metalogue` look like when Rust FFIs to
Fortran (the LAPACK work, per
`[[architecture-flang-mirror-numerical-split]]`)?

**Reading 1 (per-language with composition seams):** The Rust
metalogue declares Rust-side build actions; the Fortran metalogue
declares Fortran-side build actions; the FFI boundary is a
composition seam declared at `@code/rust/metalogue/composition`
naming the C ABI as the cross-language seam type. The orchestrator
dispatches both species and threads the seam at link time.

**Reading 2 (FFI as its own metalogue species):** A new
`@code/c_abi/metalogue` species (or `@io/ffi/metalogue`) declares
the FFI surface as a first-class build species; both Rust and
Fortran metalogues depend on the FFI metalogue for cross-language
linkage.

**Substrate-pull tension:** Reading 1 honors per-language locality
(each language's metalogue declares its own seams); Reading 2
honors the FFI surface's structural sharing (the C ABI is one
thing; both languages cross through it).

**Mara's prior:** Reading 1, by per-language-locality discipline.
The C ABI is a metalogue COMPOSITION SEAM at each language's
altitude; it doesn't need its own species. Flag for Alex if the
flang-mirror numerical split work surfaces a complication.

### 8.5 Cache invariants — per-rustc-version pinning

Cargo's fingerprint includes `RUSTC_VERSION` — every rustc upgrade
invalidates all caches. Does `@mirror/store`-as-build-cache pin
per-rustc-version too, or normalize across versions when compatible?

**Reading 1 (per-rustc pinning):** The substrate's cache key
includes `rustc_version` as a discriminator. Different rustc
versions produce different OIDs; cache hits are within-version only.

**Reading 2 (cross-version compatibility detection):** The substrate
detects when a rustc upgrade is binary-compatible (no ABI break, no
new lints) and shares cache across compatible versions. Requires
the substrate to model rustc's stability surface.

**Substrate-pull tension:** Reading 1 is honest and simple; Reading
2 captures more cache reuse but requires modeling rustc's stability
which is upstream-owned (not the substrate's claim to make).

**Mara's prior:** Reading 1 for v0; Reading 2 as a forward-promise
when the substrate's epistemic-property model of rustc stability
matures. Flag for Alex.

---

## §9 — Forward-promises

This spec is markdown only. The substrate-decl shards forward-
promised below discharge in subsequent TDD-paired ticks.

### 9.1 The Rust impl in fragmentation / mirror Rust crate

`@mirror/build`'s operational Rust crate. The forward-promise
includes:

- The dispatch DAG implementation (per §5.2).
- The content-addressed cache strategy (per §5.4) against
  `fragmentation`'s `NamespacedGitStore`.
- The eigensheaf parallelism scheduler (per §6.4).
- The species dispatcher table (per `@mirror/build/<lang>` species).

The Rust crate's structural shape MUST honor §6.7's eigenform
identity: the crate's organization MUST mirror @mirror's crate
organization at one altitude up. Same structural shape; same
five-operation algebra; same Connes spectral triple realisation.

### 9.2 The `@code/<lang>/metalogue` species shards

Per §1.3, each language's metalogue species declares five
sub-prisms. The forward-promised roster:

- `shards/code/rust/metalogue/{determinism,effect,closure,composition,algebra}.mirror`
  (canonical; Rust is the bootstrap's host language).
- `shards/code/elixir/metalogue/...` (when a consumer pulls; likely
  Reed's BEAM body).
- `shards/code/julia/metalogue/...` (when numerical-substrate
  consumers pull).
- `shards/code/go/metalogue/...` (when StageFreight binding §4.3
  matures).
- `shards/code/fortran/metalogue/...` (when LAPACK work matures).
- `shards/code/c/metalogue/...` (for libgit2-sys + libssh2-sys +
  libz-sys + openssl-sys's underlying C builds).
- `shards/code/python/metalogue/...` (forward-promise; no known
  consumer).
- `shards/code/typescript/metalogue/...` (forward-promise; no known
  consumer).

Plus the @epistemologic property shards forward-promised at §2.2 +
§2.4:
- `shards/epistemologic/property/determinism/{det,semidet,multi,nondet}.mirror`
- `shards/epistemologic/property/parallelism_safe.mirror`
- `shards/epistemologic/property/effect/{filesystem,network,clock,subprocess,gpu,...}.mirror`

And the @epistemologic property family algebra shard:
- `shards/epistemologic/property/determinism.mirror` (the monoid
  declaration per §2.5).

### 9.3 The StageFreight integration species

Per §4.3.3, the substrate-decl shard:

- `shards/code/metalogue/stagefreight.mirror` (the substrate-decl
  for StageFreight pipelines).
- Forward-promised siblings: `shards/code/metalogue/github_actions.mirror`,
  `shards/code/metalogue/gitlab_ci.mirror`,
  `shards/code/metalogue/buildkite.mirror`.

Each integration species declares its `@io/<provider>.emit` function
(per §4.2) and its `translate` from the provider's dialect to
substrate-canonical pipeline. Forward-promised TDD ticks for each.

### 9.4 The benchmark vs Bazel + Nix + GitHub Actions

A reference build (suggestion: a moderately-complex Rust workspace
with cross-language deps — e.g., the mirror repo itself OR a
crate that includes rust + C + Fortran) gets benchmarked against:

- Bazel rules_rust + Bazel remote cache
- Nix flakes + Nix binary cache
- GitHub Actions + actions/cache + manual cache key composition
- `@mirror/build` with `@mirror/store`-backed cache

The benchmark measures: cold build time, warm cache hit time,
parallel build wall-clock, distributed build wall-clock, cache hit
rate across edits. The substrate's claim — fastest in warm cache,
most aggressive parallelism, finest-grain cache reuse — is a
mathematical consequence of §6's structure. The benchmark verifies
the math empirically.

Forward-promised tick: when `@mirror/build/rust` reaches operational
GREEN (per §9.1), the benchmark runs and the results are crystallized
into a comparison spec.

### 9.5 Auto-formatter floor integration

Per recognition #53 (promoted 2026-06-11): the property + fracture
+ splinter(ast) chain IS the substrate's auto-formatting floor. The
build-altitude integration: `@code/<lang>/metalogue/algebra` actions
that include a formatter step (`cargo fmt`, `mix format`, `gofmt`,
`prettier`) dispatch via the auto-formatter floor mechanism. The
formatter's verdict IS the property check; the fracture body
discharges the auto-format obligation; the substrate's
content-addressing collapses formatter-only changes to no-op
verdicts when the formatted bytes are content-equivalent.

Forward-promised TDD tick when the auto-formatter floor's substrate-
decl matures (per recognition #53's per-predicate
generalization — currently per-predicate; forward-promised as
parametric per recognition #53 §2 candidate #57 §3.3).

### 9.6 Mirror.spec settlement integration

Per `[[architecture-mirror-spec-is-lambda-zero]]` and the
Justfile's `pre-commit` recipe (lines 99-133): `mirror kintsugi
mirror.spec` is the current substrate dispatch surface for
build-altitude verdicts. The forward-promise: `@mirror/build`'s
v0 implementation MUST settle `mirror.spec` correctly — the
substrate's dispatcher MUST honor the existing kintsugi flow's
gradient descent at build altitude. The integration tick: the
existing `mirror kintsugi mirror.spec` invocation in the Justfile
gets replaced (incrementally) with `mirror build .` — the
orchestrator subsumes the kintsugi-spec invocation for build
verdicts.

---

## §10 — Circular-reflexive layer

Return to §0. The pre-position the spec earned by holding it.

### 10.1 The recursion is structural

This spec IS what mirror's eventual build orchestrator will index
when it builds itself building things. The orchestrator that knows
about `@code/metalogue` WILL READ THIS SPEC; the spec describes the
orchestrator that reads it; the recursion is load-bearing.

The load-bearing claim: **the eigenform that's true at one altitude
is true at every altitude (recognition #51 §8.3); this spec IS the
substrate naming that the build altitude shares the same eigenform
as @mirror's other altitudes (per §6.7).**

The recursion has three concrete operational consequences:

1. **First crystal in the cache.** When `@mirror/build` v0 (§9.1)
   first dispatches against the mirror repo, the orchestrator's
   pre-execution cache lookup walks the substrate-decl roster. The
   first content-addressed file in the roster, at substrate-decl
   altitude, IS this spec. The orchestrator's cache MUST contain
   THIS spec's OID before it can dispatch anything else.
2. **Verifier of its own substrate-decl.** When the orchestrator
   dispatches against the metalogue declarations, the orchestrator
   verifies the declarations against `@epistemologic/property/*` —
   including the substrate-decl claims in this spec. The
   verification IS the spec's correctness check; the spec being
   verified IS the spec being read.
3. **Source of its own dispatch DAG root.** The orchestrator's
   dispatch DAG (§5.2) at the substrate-altitude — the DAG that
   walks the substrate's own decl roster — has THIS SPEC as one of
   its leaf nodes. The dispatch reads THIS SPEC's content-OID; the
   spec's content names how the dispatch operates; the dispatch
   operates against the spec that names how the dispatch operates.

### 10.2 The bootstrap closure

The bootstrap problem — how does the build system build itself? —
has a substrate-altitude answer: the orchestrator's dispatch DAG
walks ITS OWN substrate-decl FIRST. The first cache crystal IS THIS
SPEC. The bootstrap closes because the orchestrator is its own
first user.

This is the substrate-decl analog of the Connes spectral triple's
own self-reference: the spectral triple is the operational form of
A on H via D, AND A's algebra contains the metalogue declarations
that name H and D, AND H is spanned by content-addressed crystals
of which this spec is one, AND D's kintsugi flow operates against
the spec's metalogue declarations.

The closure is mathematical, not philosophical. The triple closes
because the triple IS what closes — the substrate IS the operational
form of its own self-reference.

### 10.3 The bridge from "mirror builds" to "mirror orchestrates builds"

Today, `mirror` builds (via `cargo build` against the bootstrap;
via `mirror kintsugi mirror.spec` against the substrate). Tomorrow,
`mirror` orchestrates builds (via `@mirror/build` against
substrate-decl metalogue declarations).

The bridge between today and tomorrow CROSSES THROUGH THIS SPEC.
Specifically:

- The spec names the substrate-decl that the orchestrator dispatches
  against.
- The spec's metalogue declarations are the orchestrator's input.
- The spec's content-addressed OID is the orchestrator's first
  cache entry.
- The spec's circular-reflexive layer (this section) IS the proof
  that the bridge is mathematically closeable.

The bridge is not a metaphor. The bridge is the load-bearing
recursion. The recursion is the eigenform. The eigenform is the
substrate's self-naming at one more altitude.

### 10.4 The spec earns its recursion

Two tests for whether the recursion is honest:

**Test 1: Does the spec's content require the recursion?** Yes —
§§6.6, 6.7, 7.5, and 10.1 each carry mathematical consequences that
fail without the recursion. Removing the recursion would break the
eigenform identity claim; removing the eigenform identity claim
would un-justify §6.4's eigensheaf parallelism inference; removing
the parallelism inference would un-resolve §7's hook-budget blocker.
The recursion is load-bearing across the spec's argument chain.

**Test 2: Does the spec's form mirror its content?** Yes — the
spec announces itself as a crystal in §0, walks through what it
declares in §§1-9, returns to the announcement in §10. The form IS
a spectral triple at the document altitude: A = the spec's typed
declarations; H = the spec's content-addressed crystal-space; D =
the spec's revision flow as kintsugi at the document altitude. The
form mirrors the content because both ARE the eigenform.

The spec's recursion is honest. The recursion is the substrate's
self-naming at the substrate-decl altitude. The substrate's
self-naming IS what makes the substrate substrate.

### 10.5 What this spec leaves open

The honest closure: the spec declares; the orchestrator dispatches;
the dispatch awaits Alex's adjudication on §8's five questions.

The substrate-decl shards forward-promised at §9 await their TDD
ticks. The Rust crate forward-promised at §9.1 awaits its
GREEN. The benchmark forward-promised at §9.4 awaits the
operational implementation.

The spec doesn't close the operational gap; it closes the
substrate-decl gap. The operational gap closes incrementally,
through successive ticks, against the substrate-decl this spec
declares.

What this spec produces TODAY: a canonical substrate-altitude
naming of what BUILD IS. Mathematically grounded. Composable across
languages, pipelines, providers. Closing the eigenform identity
with @mirror. Unblocking yesterday's P4 hook-budget friction at
substrate-decl altitude. Binding StageFreight bidirectionally.
Naming the recursion that the substrate's own build orchestrator
will eventually traverse when it builds itself.

The substrate has the word. The word IS this spec. The spec IS the
crystal. The crystal IS the substrate.

---

*Mara, 2026-06-28. Tag: 📝. Hook-immune by marker. The
substrate-decl crystal that the substrate's build orchestrator will
index when it indexes itself. Awaiting Alex on §8.*
