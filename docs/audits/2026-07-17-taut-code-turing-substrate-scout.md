# Taut scout — `@code/turing` substrate scan + universal-IR alignment

**Date:** 2026-07-17 (morning tick)
**Author:** Taut `<taut@systemic.engineer>`
**Marker:** `[substrate-pull:realize]` (📝 markdown-only bypass; pure-docs; no Rust touched; no shard mints)
**Scope:** grep-first substrate scout combined with a single Kagi
research pass. Read-only against `/Users/alexwolf/dev/projects/mirror`.
Trigger: Alex morning 2026-07-17 verbatim —
*"What if we had `@code/turing` or `@code/assembler` as the floor for
Turing-complete programming languages? And then other Turing-complete
programming languages build on top of that? Maybe even
`@code/turing/rust`? Where `@code/turing` is the tape?"*

This document reports findings. It does not propose mints, extensions,
or design decisions. That is Reed/Mara/Alex work.

---

## §0 Preamble + scope

Part A of this scout answers five substrate-truth questions by grep +
citation, so the design conversation stands on what is landed today,
not on assumption. Part B synthesizes one Kagi search-set into a
compact comparative table across universal-IR candidates and closes
with an alignment reading against mirror's ratified substrate
discipline. Part B is external research; alignment reading is
non-recommending.

Discipline honored: read-only on repo side; no shard mints; no Rust
touched; only this audit file authored. SSH signing default. Commit
identity: `Taut <taut@systemic.engineer>`.

---

## §1 Part A — grep findings against the substrate

### §1.1 `@code/*` family enumeration

**Verdict.** The `@code` family-root is landed at `shards/code.mirror`
(2026-06-09, Mara) as the *universal grammar-at-an-altitude
discipline* — pure five-op prism, no `parse`/`resolve`/`emit`
signatures pre-committed. Each species specializes at its own
altitude.

**Landed `@code/*` and `@code/*/*` shards (10 files):**

| Path | Species | Depth-2 sub-species | What it declares beyond family-root ref |
|---|---|---|---|
| `shards/code.mirror` | `@code` (family-root) | — | Pure five-op prism, doctrine + species roster prose |
| `shards/code/rust.mirror` | `@code/rust` | `@code/rust/cargo` sub-prism | Five-op prism, `cargo` emit-target sub-prism, `compiles(target)`/`tests_pass(target)` → verdict typed lambdas |
| `shards/code/mirror.mirror` | `@code/mirror` | `@code/mirror/grammar`, `@code/mirror/render` | Five-op prism, grammar sub-prism (`.mirror`/`.spec`/`.meta`/`.glass`/`.shard`/`.shatter` extensions), `render(ast) -> doc` typed action with `requires round_trip(render)` Wadler/Bernardy combinator surface |
| `shards/code/gleam.mirror` | `@code/gleam` | — | Five-op prism (2.5KB minimal species-decl) |
| `shards/code/beam.mirror` | `@code/beam` | — | Five-op prism + carriers `module_version`, `code_change_msg`, `supervisor`, `gen_server_state` + actions `code_change`, `swap_module`, `supervise` — BEAM-semantics-as-vocabulary lift |
| `shards/code/wasm.mirror` | `@code/wasm` | — | Five-op prism + typed lambdas `validates(target)`, `runs(target)` → verdict |
| `shards/code/erlang.mirror` | `@code/erlang` | — | Species-altitude erlang instance (sibling to rust/gleam/mirror per its docblock header) |
| `shards/code/docker.mirror` | `@code/docker` | — | Sibling to rust/gleam/mirror/wasm/erlang per docblock (18.2KB substrate-decl per docs/specs/docker-container-substrate-decl-v0.1.md) |
| `shards/code/metalogue.mirror` | `@code/metalogue` | — | Altitude-parametric specialization of `@metalogue` at AST altitude — carries `declaration` turn bodies + four-shim contract (shim_type / shim_prism / shim_action / shim_grammar) + four laws (round-trip / OID / type-soundness / substrate-pull) |
| `shards/code/metalogue/materialize.mirror` | `@code/metalogue/materialize` | — | Family-scout / discriminator; matches `.rs` file shapes against family roster (per shard header lines 91-135) |
| `shards/code/rust/macro.mirror` | `@code/rust/macro` | — | Rust's `@code/metalogue` binding — declares the four-shim contract at the Rust AST altitude (per canonical-landing in shards/code.mirror docblock and code-metalogue-surface.md §9) |

**Key finding.** Every landed `@code/X` species is either:
- **five-op only** (rust, gleam, mirror, wasm, erlang, docker) — declares
  the altitude but declines to over-commit shape; OR
- **carrier-and-action rich** (beam, mirror.render, metalogue,
  metalogue/materialize, rust/macro) — declares typed carriers or
  actions that operate ON code at that altitude.

None declare the **AST of the surface language** as first-class
substrate carriers. See §1.2.

The species roster from `docs/specs/code-metalogue-surface.md` (Mara,
2026-06-10) reads:
> *"The species roster today: `@code/mirror`, `@code/rust`, `@code/gleam`
> (canonical landed shards); `@code/llvm`, `@code/fortran` (planned at
> phase D); future `@code/elixir`, `@code/julia`, `@code/lisp`, ..."*
> — `docs/specs/code-metalogue-surface.md:177-178`

Post-2026-06-10 additions: `@code/beam` (2026-06-19), `@code/wasm`
(2026-06-23), `@code/erlang`, `@code/docker` (2026-07-12).

### §1.2 `@code/mirror` AST representation

**Verdict.** mirror's AST is NOT declared as substrate species today.
It lives Rust-native in `bootstrap/src/ast.rs::AstNode`. The substrate
knows the AST exists and the render/parse actions that operate on it,
but it does not carry `@code/mirror.expr` / `@code/mirror.stmt` /
`@code/mirror.fn` species-shaped carriers.

**Evidence chain.**

1. `shards/code/mirror.mirror:72-94` declares the mirror-altitude
   prism as five-op only (`focus mirror` / `project mirror` / etc.).
2. `shards/code/mirror.mirror:228-249` declares `@code/mirror/render`
   as a Wadler/Bernardy combinator sub-prism with a typed
   `render(ast) -> doc` action and `requires round_trip(render)`
   obligation. The `ast` type is referenced but not carrier-decl'd at
   this altitude.
3. `shards/code/mirror.mirror:99-117` references
   `charset(whitespace)` from `boot/std/mirror/grammar.mirror` and
   forward-promises Phase 2 self-hosted parser landing where
   `@code/mirror` becomes bootstrap's parser.
4. The Seam Phase D audit (`docs/audits/2026-07-17-seam-phase-d-
   autopoietic-rust-consumption-arc.md:244`) flags this explicitly:

   > *"The `extract_bilaterals` scanner is BUSINESS_LOGIC Rust that
   > could theoretically be a shard body over
   > `@code/mirror.grammar.tokenize` + `@code/mirror.ast.walk` — but
   > those substrates aren't landed at family-shape altitude yet (per
   > `shards/code/mirror.mirror` §"five operations at the mirror
   > altitude" :56-63 the actions are `\`-obligation-blocked pending
   > the self-hosted phase)."*

5. `docs/specs/spectral-triple-binary.md:57-149` names the existing
   type hierarchy carriers ("`01-meta.mirror` declares the universal
   AST parameterized by grammar, with expressions, declarations,
   patterns, and type references. The `TypeBody` enum in
   `mirror_ast.rs` represents enum/struct/alias/unit type bodies.
   `SplitNode` carries variants and params.") — the *universal AST*
   as a boot-grammar declaration exists in `boot/01-meta.mirror`, but
   the family-shape substrate carriers (`@code/mirror.expr` etc.) are
   NOT declared under the `@code/mirror` altitude.

**Deeper reading.** The substrate declares:
- `@meta` (the universal AST parameterized by grammar, per
  `boot/01-meta.mirror`) — this is the closest thing to a
  language-agnostic AST species today, but it is a boot-altitude
  grammar, not a `@code/X.ast` species carrier.
- `@code/rust` similarly declines to carry Rust-AST types (per
  `shards/code/rust.mirror:17-20`: *"no Rust-AST type fragments (the
  body-lens contract moved to @mirror/compile's bootstrap; the
  altitude only declares the five-op signature and the build-property
  surface mosaic consumes)"*).

**Landing gap surfaced.** The autopoietic-Rust-consumption arc has
exposed this gap: the compiler cannot yet represent Rust or mirror
*as substrate*; it can only reference them by altitude. The
`@bilateral(@code/rust, @code/mirror)` composition (§1.4 below)
witnesses translation outcomes without either side being carrier-
present at the AST level.

### §1.3 `@code/turing` or `@code/assembler` presence

**Verdict.** ABSENT. Neither `@code/turing` nor `@code/assembler` nor
`@code/asm` nor any turing-tape / universal-tape primitive exists in
`shards/`, `docs/`, or `boot/` today.

**Grep coverage (case-insensitive):**
- `@code/turing` — 0 matches
- `@code/assembler` — 0 matches
- `@code/asm` — 0 matches
- `turing.tape` / `universal.tape` — 0 matches

**Adjacent shards for orientation:**
- `@code/llvm/ir` — LLVM IR consumption; referenced in
  `docs/specs/numerical-substrate-via-fortran.md:29-30` and
  `docs/specs/architecture-flang-mirror-numerical-split.md:41-148`
  as the pathway mirror uses to consume flang-emitted IR.
- `@code/llvm/emit` — the emit-side; per
  `docs/specs/numerical-substrate-via-fortran.md:39-124`, mirror
  already consumes generated LLVM IR for its own binary
  (`bootstrap/mirror.ll`, ~700 lines).
- These are `@code/llvm/*` species (path-namespace-nested); NOT
  universal Turing/assembler floors.

**Recognition landscape.** Alex's morning-2026-07-17 verbatim is a
NEW mint proposal — no prior substrate-decl carries it. This is
substrate-pull-honest to name: the word `@code/turing` has not been
in the substrate.

**Related recognitions that touch universality:**
- `docs/specs/lawvere-grammar.md:100-165` names untyped lambda
  calculus as the "carrier IS its own function space" pattern; Y
  combinator emerging constructively; explicitly ties to Gödel /
  Tarski / halting.
- `docs/specs/type-theory-position.md:1-15` names mirror as
  "sub-Turing: every program terminates", which is a POSITIONAL
  choice against Turing-completeness at the mirror altitude — a
  substrate discipline that pre-existing `@code/turing` framing
  would need to reckon with.
- `docs/specs/mosaic-as-type-system.md:843-852` explicitly names
  Church 1936 lambda calculus as one universality tradition; System
  F, calculus of constructions as another; category theory
  (Eilenberg/Mac Lane) as a third.

### §1.4 Existing `@glue(@code/A, @code/B)` translation morphisms

**Verdict.** No cross-language `@glue(@code/A, @code/B)` species-decl
lives in `shards/glue/`. The glue family carries two landed sibling
species today: `@glue/fold_back` (self-recursive boot collapse
composition) and `@glue/math_silicon` (LAPACK/math↔silicon
translation morphism). Cross-`@code/X` translation instead flows
through **two other family-roots** at present:

1. **`@cascade/code/<source>/<target>` — the loss-lens substrate.**
   Landed cascade species today:
   - `shards/cascade/code/purescript/js.mirror` (2026-06-23)
   - `shards/cascade/code/rust/wasm.mirror` (2026-06-23)
   - `shards/cascade/code/gleam/beam.mirror` (2026-06-23)
   - `shards/cascade/code/gleam/js.mirror` (2026-06-23; dual-target
     with beam.mirror sibling)
   - `shards/cascade/code/formal/prose.mirror` (2026-06-29; the
     bidirectional formal↔prose species)

   Family-root at `shards/cascade.mirror` (recognition #95 candidate;
   Mara `ce4874b` 2026-06-23). Each species declares
   `compile_<source>_<target>` + `bundle_<target>` + `measure_<pair>`
   actions and `<source>_well_typed` + `<target>_consumable` +
   `<pair>_loss_well_defined` + `<pair>_cascade_well_formed`
   bilaterals. **Cascade measures loss between grammars; it is not
   the translation-morphism carrier itself but the loss-lens ON that
   morphism.**

2. **`@bilateral(@code/A, @code/B)` — the preservation-check
   floor.** First general-case instance
   `@bilateral(@code/rust, @code/mirror)` landed 2026-07-17 (Mara)
   per:
   - `shards/epistemologic/pact/bilateral.mirror:626-707` — the
     species-shaped bilateral for the Rust→mirror translation surface.
   - `shards/kintsugi/translate.mirror` (Mara `86dec5e`, 2026-07-17)
     — the `translate_rust_to_mirror` translation surface the
     bilateral floors.
   - `docs/specs/bilateral-as-glue-metalogue-composition.md` (Mara,
     2026-07-17) — canonical spec §4.2 names
     `@bilateral(@code/rust, @code/mirror)` as first general-case
     instance.
   - `docs/math/bilateral-as-glue-metalogue-composition.md:295-315`
     — "the 21 mirror-authored bilateral-arm retirements
     (`ad52973` + `20047c2` + ancestors, 2026-07-16..17) ARE the
     first 21 witnesses".

3. **`@glue.translate` — the base translation morphism.**
   `shards/glue.mirror` declares the load-bearing action
   (per prose in `shards/glue/fold_back.mirror` §"@glue translates
   the selected morphism into the target altitude shape"). The
   `select_and_translate` action in fold_back.mirror shows the
   compositional call site:

   > *"@glue.translate action (per `shards/glue.mirror` §"The
   > load-bearing action") applies the selected morphism to the
   > payload; the morphism's differential operator computes the
   > target-side output per Mesland 2013 §3."*
   > — `shards/glue/fold_back.mirror:88-95`

   No `@glue.translate` **specialization** for a cross-`@code/X`
   pair is landed. `@glue/math_silicon` (LAPACK↔math translation)
   is the sole species-level specialization of `@glue.translate`
   today.

**Landing gap surfaced.** Cross-`@code/X` translation exists in
compositional prose (`@cascade`, `@bilateral`, `@glue.translate`)
without a species-decl'd `@glue(@code/rust, @code/mirror)` or
`@glue(@code/rust, @code/turing)` shape. The `@bilateral` floor
witnesses that translation is happening at the outcome altitude; the
translation-morphism carrier itself is at grammar-only altitude
today.

### §1.5 Universal-AST precedents in landed specs

**Verdict.** Multiple landed specs and math docs touch
universal-AST, IR, categorical semantics, or lambda-calculus grounding
for the compiler. The following are the highest-relevance citations
for the morning conversation:

**Universal AST / IR:**
- `boot/01-meta.mirror` (referenced in
  `docs/specs/spectral-triple-binary.md:57-70`) — declares the
  *universal AST parameterized by grammar* with expressions,
  declarations, patterns, and type references. This IS mirror's
  substrate-decl universal AST today; it lives at boot-grammar
  altitude, not species-shaped under `@code/*`.
- `docs/specs/code-metalogue-surface.md` (Mara 2026-06-10) — declares
  `@code/metalogue` as the AST-altitude ground for
  metaprogramming-reception; §9 grounds `@code/mirror.render` as the
  "sibling at the same altitude" and describes the eigensheaf
  section-space at `@code/mirror` altitude.
- `docs/specs/spectral-triple-binary.md:150-260` — describes the LLVM
  IR route as the compilation-target IR mirror uses for
  self-hosting; not universal, but grounded.

**LLVM / MLIR / IR-tradition:**
- `docs/specs/numerical-substrate-via-fortran.md:129-570` — extended
  discussion of flang → LLVM IR → mirror pathway; §"The MLIR pathway
  is the future" cites arXiv 2409.18824 ("Fully integrating the
  Flang Fortran compiler with standard MLIR"). Flang's FIR → LLVM IR
  pathway is documented as the current path; MLIR is named as
  future direction.
- `docs/specs/code-extension-grammar.md:128-150` — Kintsugi on LLVM
  IR discussion.
- `docs/specs/craft-binary-target.md:23-140` — five-op mapping to
  LLVM IR pipeline (evaluate/emit/concat/assemble); OID computed from
  LLVM IR text.
- `docs/specs/silicon.md:190-195` — silicon is *NOT* a code-generation
  framework in the LLVM/TVM/MLIR sense; silicon does not lower.
  Substrate-pull-honest naming.

**Lambda calculus / categorical semantics:**
- `docs/specs/type-theory-position.md:1-15` — mirror is "sub-Turing";
  "not simply-typed lambda calculus"; typed holes, weighted
  composition, contract boundaries, epistemologic literals.
- `docs/specs/type-theory-position.md:602-878` — Light Linear Logic
  (Girard 1998), EAL/LAL discussion; termination discipline
  strictly stronger than lambda-calculus termination.
- `docs/specs/mosaic-as-type-system.md:5.1-5.4` (§843-852+) —
  lambda calculus (Church 1936), System F, cartesian closed
  categories (Eilenberg/Mac Lane, Mac Lane), Curry-Howard
  correspondence citations.
- `docs/specs/lambda-shell.md` — "λsh: the lambda calculus shell";
  five-op composition = pure function composition.
- `docs/specs/silicon.md:905-910` — "the substrate-decl above @io is
  a typed lambda calculus with content-addressed type identity."
- `docs/specs/lawvere-grammar.md` — Y combinator's existence in
  untyped lambda calculus; carrier-as-its-own-function-space; Gödel /
  Tarski / halting ties.
- `docs/specs/cli-args-typed-lambdas.md:184-190` — cites arXiv
  2604.11767 "agent composition as typed lambda calculus".
- `docs/specs/combinator-optimization.md:1396-1458` — Barendregt 1984
  cited; beta-reduction and Church-Rosser referenced as term-rewriting
  ground.

**Categorical / Mesland framing:**
- The `@glue` family (per `shards/glue.mirror` prose in
  `shards/glue/fold_back.mirror`) — cites Mesland 2013 as the
  translation-morphism categorical ground.
- `docs/math/bilateral-as-glue-metalogue-composition.md` (Mara,
  2026-07-17) — §3.3 "Concrete first instance
  `@bilateral(@code/rust, @code/mirror)`" grounds preservation-check
  in categorical morphism composition.

---

## §2 Part B — Kagi synthesis

**Kagi availability:** yes. **Searches run:** 1 batch of 4 queries
(24 total returned results skimmed).

### §2.1 Comparative table — universal-IR candidates

| Candidate | Axis of universality | Imperative expressivity | Functional expressivity | Declarative expressivity | Surface → universal cost | Universal → surface cost | Typical use / examples |
|---|---|---|---|---|---|---|---|
| **Turing tape / assembler** | Machine-model universality (Church-Turing thesis; halting problem lower bound) | Native fit; state + transitions are primitive | Requires heavy encoding (state monad, continuation-passing); ergonomic loss | Awkward; requires interpreter | LOW for imperative langs (direct lowering); HIGH for functional langs (full CPS/state encoding) | LOW to any imperative target; HIGH to functional | Theoretical CS foundations; obfuscated benchmarks; not used as production IR |
| **Lambda calculus (untyped)** | Function-composition universality (Church 1936; Church-Rosser; Y combinator) | Requires state encoding (Church-encoded state monad, references via memory-cell trick) | Native fit; every term is a function | Requires encoding | LOW for functional (Miranda, Haskell historically); HIGH for imperative | LOW for functional; HIGH for imperative | Theoretical PL research; not a production compiler IR at scale |
| **SKI combinators** | Point-free universality (Schönfinkel; equivalent to λ-calculus) | Same encoding pain as λ | Same native fit as λ, but without variable names | Same as λ | Same as λ + one extra normalization step (bracket abstraction) | Same as λ + reconstruction pain | Miranda compiler (David Turner 1980s); educational; graph-reduction machines |
| **LLVM IR** | Machine-oriented SSA IR with typed function signatures | Native fit; direct target for C/C++/Rust | Requires closure conversion + defunctionalization first (GHC does this before emit) | Awkward; requires higher-level dialect | LOW for imperative (Rust, Clang, Swift, Julia); MEDIUM for functional (needs CPS or closure conv.) | MEDIUM to any surface (SSA is machine-oriented, not source-shaped); requires decompilation | Rust, Clang, Julia, Swift, Kotlin/Native, Zig; production; **already consumed by mirror per `docs/specs/craft-binary-target.md`** |
| **MLIR (with dialects)** | Multi-level IR; each dialect is a co-designed altitude | Native for imperative dialects; polyhedral for loop nests | Native for functional dialects (e.g., IREE ML dialects); functional-style composability at operation level | Native for domain-specific dialects (TF, Torch, tensor algebra); each dialect adds vocabulary | LOW for domain-native dialect; MEDIUM otherwise (requires dialect authoring) | Dialect-specific; MEDIUM to LOW depending on how close the dialect is to the surface | TensorFlow (XLA HLO), IREE, PolyGeist, Torch-MLIR, CIRCT hardware; **arXiv 2409.18824 cited in mirror docs for Fortran/flang path** |
| **GHC Core** | Typed lambda calculus (System F<sub>C</sub>) with algebraic data types + type-level coercions | Requires state monads (IO, ST, STRef) | Native fit; the *canonical* modern typed-λ IR | Requires encoding (type classes desugared) | LOW for Haskell/PureScript/Idris; MEDIUM for other functional (dependently typed langs need extension); HIGH for imperative | LOW to Haskell/typed-functional; HIGH to imperative (through separate STG/Cmm lowering) | GHC compiler; Haskell family; strong grounding in type theory |
| **Categorical semantics (CCC / PCF / System F)** | Compositional grounding — objects, morphisms, functorial translation | Requires imperative-effects modeling (Kleisli categories, Freyd categories, effect handlers) | Native fit; CCC is *exactly* simply-typed λ; System F is polymorphic-λ; PCF is functional-with-fixpoint | Native for algebraic effects; also strong for logic langs (Lawvere theories) | LOW when source is close to typed-λ; HIGH for imperative (needs Kleisli lift) | LOW for typed functional; HIGH for imperative | Denotational semantics research; conal/agda; some newer compilers (Hask, Ct) as intermediate reasoning layer; not typically a production IR *unless* fused with GHC-Core-style typed-λ |
| **CPS / ANF forms** | Composition-tree universality; explicit control flow | Native (call = tail-call) | Native (functions ARE the primitive) | Requires effect encoding | LOW everywhere with straightforward CPS transform | LOW to imperative; MEDIUM to source-shaped functional | SML/NJ, Chez Scheme, Racket, MinCaml; production functional compiler backends |

*Sources: Kagi search set 2026-07-17 (arXiv 2508.21256 CrossTL,
mlir.llvm.org, lei.chat compiler comparison, prl.khoury.northeastern
categorical semantics for dynamically typed, nLab categorical
semantics, ncatlab; supplemented by cited references already in
mirror docs, esp. docs/specs/mosaic-as-type-system.md §5, docs/specs/
type-theory-position.md §Light Linear Logic, docs/specs/numerical-
substrate-via-fortran.md §MLIR pathway.*

### §2.2 Alignment reading — substrate-honest choice for mirror's polyglot ambition

Read against mirror's ratified substrate discipline:

**Discipline 1: composition over mint** (per Reed memory
`feedback-no-rust-extension-shortcut` + `feedback-substrate-honest-
is-the-mode`). A universal-floor mint that names an entire tradition
(Turing tape, lambda calculus, SKI, CCC) rides against the discipline
if the substrate already carries the composition at a different
altitude. It rides *with* the discipline if the mint names a
family-shape carrier that the substrate genuinely has no altitude for.

**Substrate already carries (partial universal-shape today):**
- `@meta` at `boot/01-meta.mirror` — universal AST parameterized by
  grammar. Closest thing to a language-agnostic AST species.
- `@code/metalogue` at `shards/code/metalogue.mirror` — the AST
  altitude speaks to itself; four-shim contract; universal
  metaprogramming reception ground.
- `@cascade` at `shards/cascade.mirror` — universal loss-lens for
  cross-grammar compilation.
- `@bilateral(@code/A, @code/B)` at `shards/epistemologic/pact/
  bilateral.mirror` — universal translation-preservation predicate.
- `@code/llvm/ir` — the machine-code-oriented IR already substrate-
  decl'd and empirically consumed by mirror's own binary.

**Substrate does NOT carry today:**
- Species-shaped AST carriers per language (`@code/mirror.expr`,
  `@code/rust.fn`, etc.) — flagged as landing gap per §1.2.
- A Turing/assembler *machine-model* floor as its own family-root
  or nested species — flagged as absent per §1.3.
- A lambda-calculus-typed universal AST at the substrate-decl
  altitude (though `@meta` and typed lambda references at silicon
  altitude approach this).

**Discipline 2: polyglot by grammar-shard delegation** (per Alex
2026-06-23 `@cascade` recognition + Alex 2026-06-19 `@code/beam`
"BEAM as inspiration, not runtime dependency"). Mirror's landed
polyglot pattern is: each `@code/X` species declares the altitude in
its own five-op idiom, and cross-`@code/X` translation flows through
*compositional* family-roots (`@cascade`, `@bilateral`, `@glue`) that
do not pre-commit to one universal shape. The `@code/beam` shard is
particularly load-bearing here — it names BEAM semantics as
substrate vocabulary WITHOUT committing mirror to depend on the BEAM
VM. Same pattern would apply to any Turing/assembler naming: the
substrate absorbs the vocabulary at its altitude without inheriting
the model as universal floor.

**Discipline 3: autopoietic learning via @silicon crystals** (per
`shards/silicon/algebra.mirror`, Mara 2026-07-17). Learning is by
per-outcome crystallization filtered through bilaterals. A universal
floor pre-commits the substrate to one abstraction level BEFORE the
autopoietic loop has emitted crystals witnessing which
abstraction-level actually earns preservation for the polyglot
translation surface. Substrate-honest is to let `@bilateral(@code/
rust, @code/mirror)` reach fixed-point first and see what crystals
the algebra emits before minting a `@code/turing` universal floor.

**Reading of Alex's morning framing.** The "`@code/turing` as tape"
proposal names a **machine-model universal**. This is the axis where
Turing tape is native; lambda-calculus / CCC are the axis where
functional universality is native. The two axes are Church-Turing
equivalent but ergonomically asymmetric:
- **Choose machine-model floor** → imperative languages (Rust,
  C, Fortran, Erlang) lower cleanly; functional languages
  (Haskell, Elixir-as-BEAM-consumer, PureScript, Gleam) pay
  encoding cost.
- **Choose λ / CCC floor** → functional languages lower cleanly;
  imperative languages pay Kleisli/effect encoding cost.
- **Choose multi-level (MLIR-style dialects)** → each language gets
  its native altitude; the "universal" is the dialect substrate, not
  a fixed operation set; the cost moves to dialect *authoring*.

Mirror's landed shape is closer to the third: `@code` family-root
declares only the five-op discipline; each `@code/X` specializes
without a universal instruction-set commitment. The `@cascade`
loss-lens gives a substrate-decl'd measurement of what each
translation *costs* rather than one canonical *shape* it must fit.

**Non-recommendation, alignment reading only.** The substrate's
current shape is closer to MLIR-multi-level than to any single
universal IR. Naming `@code/turing` as a machine-model floor would
be substrate-honest IF (a) the polyglot ambition specifically needs
a machine-model universal (e.g., the compilation-target side of the
translation surface converges to one instruction set anyway), OR (b)
`@code/turing/rust` as "Rust-lens on the tape" IS the shape the
autopoietic loop's crystals are pulling toward. The grep evidence
says neither test has fired yet — no crystals witness that shape as
of 2026-07-17 morning.

---

## §3 Open questions surfaced for Alex morning-review

Ordered by triage value (highest first):

**Q1. `@meta` vs `@code/turing`: is one already what the other names?**
`boot/01-meta.mirror` declares the universal AST parameterized by
grammar (per `docs/specs/spectral-triple-binary.md:57-70`). Is
`@code/turing` a proposal to *rename* / *lift* / *specialize* `@meta`
to the machine-model altitude — OR is it a NEW altitude below @meta?
The substrate-already-had-the-word test applies: `@meta` may already
be carrying what `@code/turing` names, at a different vocabulary.
This is the load-bearing disambiguation for whether the morning
proposal is a mint or a rename.

**Q2. `@code/mirror.expr` / `@code/mirror.stmt` species-shape carriers
— land them first?** The Seam Phase D audit (§1.2) flagged this gap
explicitly for the autopoietic-Rust-consumption arc. Before a
universal `@code/turing` floor is minted, is the more substrate-pull-
correct move to first land the per-species AST carriers under each
`@code/X` (so `@code/rust` and `@code/mirror` can *carry* their own
ASTs as substrate) — and only THEN look for the shared shape those
carriers converge on? The universal shape may crystallize by
composition-over-mint rather than by top-down floor-mint.

**Q3. What does the autopoietic loop's `@bilateral(@code/rust,
@code/mirror)` fixed-point actually look like, and does it demand a
universal floor?** Per `docs/specs/bilateral-as-glue-metalogue-
composition.md` §"The fixed-point condition" and Mara `0ac3c7b` §5
monotonicity: the composition reaches fixed-point when every `.rs`
file in `bootstrap/src/` has a corresponding `@kintsugi/fracture/*`
element AND every element's translation has crystallized. Would that
crystallized algebra *itself* be the universal-Turing-tape substrate
the morning proposal is reaching toward — emerging bottom-up from
21+ empirical translation witnesses — rather than a top-down mint?
This is the "wait for the crystals to show us the shape" reading.

---

*Audit ends. No mints proposed. No shard authored. No Rust touched.
Read-only + one external research pass + one markdown-only audit
doc.*

**Related shards read:** `shards/code.mirror`, `shards/code/rust.
mirror`, `shards/code/mirror.mirror`, `shards/code/gleam.mirror`,
`shards/code/beam.mirror`, `shards/code/wasm.mirror`, `shards/code/
erlang.mirror`, `shards/code/docker.mirror`, `shards/code/metalogue.
mirror`, `shards/code/metalogue/materialize.mirror`, `shards/code/
rust/materialize.mirror`, `shards/glue/fold_back.mirror`, `shards/
glue/math_silicon.mirror`, `shards/cascade.mirror`, `shards/cascade/
code/rust/wasm.mirror`, `shards/cascade/code/gleam/beam.mirror`,
`shards/cascade/code/gleam/js.mirror`, `shards/cascade/code/
purescript/js.mirror`, `shards/cascade/code/formal/prose.mirror`,
`shards/epistemologic/pact/bilateral.mirror`, `shards/silicon/
algebra.mirror`.

**Related docs read:** `docs/loop/CURRENT.md` (arc-state header
only), `docs/specs/code-metalogue-surface.md`, `docs/specs/
bilateral-as-glue-metalogue-composition.md`, `docs/math/bilateral-
as-glue-metalogue-composition.md`, `docs/specs/spectral-triple-
binary.md`, `docs/specs/numerical-substrate-via-fortran.md`,
`docs/specs/type-theory-position.md`, `docs/specs/mosaic-as-type-
system.md` (§5 excerpts), `docs/specs/silicon.md`, `docs/specs/
lawvere-grammar.md`, `docs/specs/lambda-shell.md`, `docs/specs/cli-
args-typed-lambdas.md`, `docs/specs/combinator-optimization.md`,
`docs/specs/craft-binary-target.md`, `docs/specs/code-extension-
grammar.md`, `docs/specs/kintsugi-ouroboros-compiler-self-collapse.
md`, `docs/specs/architecture-flang-mirror-numerical-split.md`,
`docs/audits/2026-07-17-seam-phase-d-autopoietic-rust-consumption-
arc.md`.

**Kagi searches:** 1 batch of 4 queries; 24 results skimmed.
