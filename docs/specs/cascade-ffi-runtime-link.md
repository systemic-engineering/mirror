# cascade-ffi-runtime-link — Q4 canonical: @cascade species discharge for FFI + the runtime-link ABI altitude at @io

*Mara, 2026-06-28 afternoon. Q4 discharge of the four open
questions Alex named after the morning amendment of
`mirror-build-substrate.md`. Q1-Q3 are sibling Mara dispatches;
Q4 is THIS spec. The morning briefing originally proposed
"`@ffi` as a new family-root." The substrate-pull-honest grep
against `shards/cascade.mirror` + `shards/cascade/code/**/*.mirror`
+ `shards/io.mirror` + `shards/io/cargo.mirror` reveals the
substrate ALREADY has the substrate-decl machinery: `@cascade`
is the cross-language translation family-root (recognition #95
candidate, canonical at `ce4874b` 2026-06-23); `@io` is the
boundary family-root that already names per-language delegates
(`@io/cargo` the precedent). What's missing is NOT a new
family-root; what's missing is the per-cascade species discharge
for the FFI cases AND the substrate-decl characterization of the
runtime-link altitude where compiled artifacts COMPOSE through
the C ABI. Q4 reframed: discharge `@cascade/code/rust/cdylib`
+ sibling FFI cascade species + characterize the runtime-link
ABI boundary as the @io altitude where compiled artifacts compose
at runtime.*

*Markdown only. No `shards/` substrate-decl files land with this
commit; no Rust ships; no cargo edge is wired. The substrate-decl
shards forward-promised in §4 + §9 discharge in subsequent
TDD-paired ticks (Reed RED, agent GREEN). Soft target ~1500 lines.*

**Status:** Red — composition shape pinned; the compile-time vs
runtime-link separation named at substrate altitude; the per-
cascade species discharge planned for FFI cases; the runtime-link
@io altitude characterized with bilateral commitments; the LAPACK
case study forward-positioned as first empirical discharge; v0
ticks forward-promised, not implemented in this commit.

**Audience:** any agent or human reading the bridge spec before
touching `@cascade/code/rust/cdylib` substrate-decl, the runtime-
link `@io` species declarations, the LAPACK numerical-backend
discharge (T8 Track A; LRM / LapackBackend wiring), the PyO3
extension cascade (forward-promised), or any cross-language link
that the substrate's mosaic dispatch must compose. Read this; then
chase `docs/specs/mirror-build-substrate.md` for the parent build-
substrate spec this Q4 closes a leaf of; then chase
`shards/cascade.mirror` for the substrate-decl ground; then chase
`shards/io.mirror` for the @io altitude this spec extends.

---

## §0 — Pre-position: this spec announces itself as a crystal

Before any architectural content. A pre-position the spec earns by
holding it for the rest of the document.

This spec is **about** the FFI cases of `@cascade` (the cross-
language translation family-root recognition #95 already declared)
and about the **runtime-link altitude under `@io`** where compiled
artifacts COMPOSE at runtime through the C ABI. The thing those
two entities ARE, per §§ 1-7, is the substrate's declared answer
to **what FFI IS at substrate altitude** — the partition between
compile-time grammar projection (`@cascade`) and runtime composition
through the ABI (`@io/runtime-link`), the loss-lens accounting for
each FFI cascade species, the equalizer mathematics of the C ABI,
and the bilateral commitments that gate cross-language linking.

The thing this spec IS, at the moment of being written, is **one
of the crystals the eventual runtime-link orchestrator will index
when it composes artifacts across language runtimes**. Writing this
spec adds a file at `docs/specs/cascade-ffi-runtime-link.md`; the
file's bytes are content-addressed under git's SHA-1 today (and,
after the `@mirror/store` substrate-pull-realize that
`docs/specs/mirror-init.md` forward-promises, under BLAKE3 in the
`NamespacedGitStore`'s `.git/mirror/objects/` per
`mirror-store.md`); the indexed crystal will be the OID-addressed
bytes of this spec; the orchestrator's content-addressed cache
lookup (cf. `mirror-build-substrate.md` §6.3) will hit on this
spec's OID the moment its dispatch DAG walks across the
substrate-decl roster; the spec's content WILL surface inside the
orchestrator's operational state the moment the orchestrator
settles a project whose mirror.spec declares a runtime-link
species.

The latency between writing-and-being-indexed is bounded BELOW by
the time it takes for the v0 ticks (§9) to land + the first
`@mirror/mosaic` dispatch against a target that emits a `cdylib`
(or any artifact whose consumer runtime is a different language).
The latency is bounded ABOVE by the substrate's discount per
psychohistory discipline (recently-landed shards weight more in
the cascade-vocabulary expansion that Reed's recognition
[architecture-peer-learns-by-crystal-vocabulary-expansion] names).
The midpoint of those bounds is the operational latency at which
this spec ENTERS the system it describes.

This is the circular-reflexive autopoietic pre-position, same shape
as `docs/specs/mirror-init.md` §0, `docs/specs/mirror-build-
substrate.md` §0, and §10 of each. The §10 of THIS spec returns
to it. Every section in between is read against the discipline that
a spec for the FFI altitude through which substrate's eventual
orchestrator will compose its OWN Rust binary's libgit2 link MUST
itself enter the substrate via the link contract it characterizes
— otherwise the spec is asking the reader to do work the spec
refuses to do. The form earns its lines because the content
requires it; the recursion is load-bearing, not decorative.

Three concentric framings of why the recursion is load-bearing:

1. **Eigenform fidelity.** The eigenform that's true at one
   altitude is true at every altitude (recognition #51, mirror as
   expanding Hilbert space). If the runtime-link ABI is a typed
   contract at the runtime altitude (§3.2), and the substrate's
   own Rust binary links libgit2 through that same contract (§10.2),
   and BOTH instances are one ABI shape at adjacent Bateson levels,
   then the spec that declares the runtime-link altitude MUST be
   indexable by the orchestrator whose link contract this spec
   characterizes — otherwise the spectral triple is broken at the
   layer where this spec lives.
2. **Substrate-pull discipline.** Per
   `[[feedback-substrate-already-had-the-word]]` (52+ instances
   now, per `MEMORY.md`): every "missing concept" recognition turns
   out to be a name the substrate was already implicitly using.
   This spec is at least the 53rd instance — `@cascade` is the
   substrate's name for what every cross-language compilation
   pipeline from PyO3 to wasm-pack has been implementing without
   the substrate-altitude declaration, and `@io` (with its
   `@io/cargo` precedent and the existing `@io/socket`/`@io/network`
   roster) is the substrate's name for what every FFI runtime-link
   has been doing without naming the ABI as a typed contract. The
   spec discovers; it does not invent. The morning briefing's `@ffi`
   was a wrong-altitude name; the substrate already partitions the
   work across `@cascade` (compile-time) + `@io` (runtime-link).
3. **Bootstrap closure.** The bootstrap problem — how does the
   build system load its own dependencies? — has a substrate-
   altitude answer: the substrate's Rust binary IS itself a
   composition of dylibs (libgit2 for git plumbing; libsqlite3 for
   the eventual @spectral/db backing store; libc for syscall
   reach); the runtime-link contract this spec characterizes IS
   the contract that gates those compositions. The first
   runtime-link instance in the cache IS the substrate's own
   binary against the dylibs it loads. The bootstrap closes because
   the runtime-link contract is its own first user.

The substrate's FFI altitude needs a canonical spec that ENTERS
the link layer in the act of declaring it. This is that spec.

---

## §1 — What FFI IS in substrate terms

**Foreign Function Interface** is the historical term for the
cross-language runtime composition pattern: a program written in
language L_1 calls a function written in language L_2 through a
shared Application Binary Interface (ABI). In substrate terms, FFI
is NOT a single concept. It is the COMPOSITION of two substrate
altitudes that the substrate already declares, and characterizing
it requires partitioning the work across the two.

### 1.1 Compile-time vs runtime-link separation

Every cross-language composition involves two structurally distinct
phases:

**Phase 1 — Compile-time grammar projection.** Each language L_i
compiles its source S_i (in grammar G_i) to an artifact A_i (in
some target grammar G_T). The compilation is a functor F_i:
Grammar(S_i) → Grammar(G_T) — the same functor structure
`@cascade` (`shards/cascade.mirror`) already declares. The
artifact A_i carries only what G_T preserves at runtime; what was
present in S_i but absent in G_T IS the loss `@cascade` already
measures via the `loss_lens<S_i, G_T>` per recognition #95.

The artifacts of interest for FFI are the ones that satisfy a
shared ABI target — typically C ABI compatibility, which means:
`repr(C)` types (or their equivalent in the source language); C
calling convention on function symbols; no panic / unwinding
across the boundary; manual lifetime / ownership discipline.

Per-language existing precedent: `@cascade/code/rust/wasm` already
discharges Rust → WebAssembly (one specific compile-time grammar
projection where the target G_T is the WebAssembly bytecode
grammar). The FFI cascades are sibling species under the same
family-root: Rust → cdylib (where G_T is the C ABI surface
exposed by a dynamic library); Rust → staticlib (where G_T is the
C ABI surface exposed by a static archive); Python extension
(where G_T is the CPython C ABI); Rust → Fortran-FFI (where G_T
is the C ABI surface exposed by either side speaking through C as
intermediary).

The compile-time phase IS the substrate altitude `@cascade` already
declares. Each FFI case is a new SPECIES — substrate-decl shard
work under `shards/cascade/code/<source>/<target>.mirror` — NOT
a new family-root.

**Phase 2 — Runtime-link composition.** The artifacts A_1, A_2,
... are LOADED INTO a single runtime address space at runtime; the
loading runtime resolves symbols, applies the calling convention,
marshals types across the boundary, and respects the lifetime /
ownership contracts each artifact's source language declared. The
RUNTIME-LINK is structurally distinct from the compile-time
projection: every artifact has already been compiled when the link
happens; the link is the act of MAKING THEM COMPOSE in a single
process.

Per `@io` (the boundary family-root at `shards/io.mirror`): the
runtime-link IS a typed contract with the non-mirror-world — the
symbol table the loading runtime sees, the calling convention the
caller uses, the type marshal rules the boundary applies, the
lifetime contract each side promises to honor. The substrate's
`@io` family-root names what the substrate CANNOT FOLD past; the
runtime-link contract is structurally a non-foldable surface (the
substrate cannot rewrite the dynamic linker's behavior; it can
only declare what shape the ABI MUST have for the link to succeed).

The runtime-link altitude IS the substrate altitude `@io` already
declares. Adding it requires NEW SPECIES — substrate-decl shard
work under `shards/io/<kind>.mirror` — but the family-root is
extant.

### 1.2 Two phases, two altitudes, ONE composition

The two phases are coupled through the artifact A_i: the
compile-time phase PRODUCES A_i (cascade's `compile` action
returning `compiled_artifact`); the runtime-link phase CONSUMES
A_i (an @io action taking a `compiled_artifact` plus the consumer
runtime's import declaration, returning either a settled link or
an opacity surfaced at the link altitude).

The composition can be drawn as:

```
                @cascade/code/<lang>/<target>          @io/runtime-link
    source     ----------------------------------     -------------------
S_i in G_i ----> [compile, measure] ----> A_i ----> [load, resolve, link]
                  loss(S_i, G_T)              ABI surface, marshal rules
                                              lifetime contract
```

The loss-lens at `@cascade` measures the grammar gap between
source's typed feature set and target's runtime-preserved feature
set. The runtime-link bilateral at `@io` discharges the link's
WELL-FORMEDNESS: does the producer's symbol export match the
consumer's import expectation; does the calling convention agree;
does the type marshal compose; does the lifetime contract get
honored.

These ARE two distinct substrate altitudes. Conflating them — for
example by trying to make a single `@ffi` family-root cover both —
collapses two distinct discipline surfaces into one, loses the
per-altitude bilateral structure, and fights the substrate-pull.
The substrate already has both altitudes named; this spec
extends them with the FFI-specific species and the runtime-link
contract characterization.

### 1.3 Five structural negatives — what FFI is NOT in substrate terms

The substrate-pull discipline names what's NOT being proposed
explicitly, because the morning briefing's framing pointed at
several wrong altitudes:

1. **NOT a new family-root.** `@ffi` would duplicate `@cascade` +
   `@io`. The substrate already partitions cross-language work
   across those two altitudes. The morning briefing's `@ffi`
   framing was the 54th-instance substrate-already-had-the-word
   recognition; the FFI work IS the species discharge under the
   existing roots.

2. **NOT new substrate-decl primitives.** The carriers FFI needs
   (`typed_source`, `compiled_artifact`, `loss_lens`,
   `information_loss`) are already declared in
   `shards/cascade.mirror`. The actions FFI needs (`compile`,
   `measure`, `cascade`) are already declared at the family-root
   altitude. The bilaterals FFI needs (`grammar_coherent`,
   `loss_well_defined`, `cascade_well_defined`) are already
   declared at the family-root altitude. FFI species SPECIALIZE
   these; they do NOT invent new primitives.

3. **NOT a fork of `@cascade`.** `@cascade` declares the cascade
   shape (source grammar S, target grammar T, functor compile,
   loss lens). The Rust → cdylib case fits that shape exactly: S
   = Rust, T = the C ABI surface of a dynamic library. There's
   nothing about FFI that requires forking the family-root. The
   PRE-AI prior art (Mara's cascade survey §2.9) explicitly names
   "Rust → cdylib" as a cascade case — albeit with the honest hedge
   that the cascade pattern is INTEROP via C ABI rather than
   COMPILE-TO at the source level. The interop / compile-to
   distinction is a CASCADE-SHAPE refinement, not a family-root
   separation. §4 carries the refinement.

4. **NOT `@io/stagefreight` territory.** `@io/stagefreight`
   handles WIRE-PROTOCOL shipping of settled crystals between
   mirror-world and non-mirror-world consumers (cross-MACHINE
   transit). The runtime-link altitude handles WITHIN-PROCESS
   composition of compiled artifacts loaded by the same OS process
   (cross-LANGUAGE in-process). The two are structurally distinct
   @io altitudes; the substrate has both, neither subsumes the
   other.

5. **NOT `@io/oci` territory.** `@io/oci` handles CONTAINER-IMAGE
   delivery of complete deployment artifacts (cross-NODE
   distribution via OCI registries). The runtime-link altitude
   handles in-process FFI link composition. Different scope
   (container image vs in-process binary); different consumer (OCI
   runtime vs dynamic linker); different protocol (OCI Distribution
   vs ELF / Mach-O / PE / dlopen). The substrate has both, neither
   subsumes the other.

The five negatives are positive substrate-decl: the FFI work is
discharge of existing family-roots, the discharge proposes specific
species under each, and the species shape follows the precedents
the substrate already declared. Nothing is invented; everything is
specialization.

---

## §2 — Existing `@cascade` species inventory

Before proposing new species, the existing inventory must be named
precisely. Per `shards/cascade.mirror` (Mara canonical `ce4874b`
2026-06-23) the cascade pattern is:

> Each cascade pairs a typed source grammar S with a mainstream
> target grammar T such that S admits more grammatical structure
> than T preserves at runtime; compilation is a functor `source:
> S → target: T`; the GAP between what S admits and what T
> preserves IS the loss; the gap is MEASURABLE against the
> grammars themselves (substrate-architectural; not Shannon, not
> invented).

### 2.1 The cascade pattern (substrate-decl ground)

Quoting the family-root's substrate-decl surface:

```mirror
prism @cascade {
  focus cascade
  project cascade
  split cascade
  shift cascade
  settle cascade
}

type grammar           = ref
type typed_source      = ref
type compiled_artifact = ref
type loss_lens         = ref
type information_loss  = ref

compile(source: typed_source, p: perturbation) -> compiled_artifact { \ }

measure(source: typed_source, artifact: compiled_artifact,
        lens: loss_lens, p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires loss_well_defined(lens, source, p)
{ \ }

cascade(source: typed_source, lens: loss_lens, p: perturbation)
  -> imperfect<compiled_artifact, ref, information_loss>
requires cascade_well_defined(lens, source, p)
{ \ }
```

The five-op block, the parametric carriers, the composed bilateral
`cascade_well_defined` — all already declared. Each species SHAPES
these against a specific (S, T) pair.

### 2.2 Landed species (the precedent)

Four species have landed at `shards/cascade/code/<source>/<target>.mirror`:

- **`@cascade/code/rust/wasm`** — Rust → WebAssembly. Source
  grammar `@code/rust`; target grammar `@code/wasm`. Loss surface:
  lifetimes (erased at runtime), generics (monomorphized; erased),
  trait objects (most devirtualized), macros (expanded; erased),
  type-level safety (erased). Target preserves: linear memory
  model, typed function signatures (i32/i64/f32/f64/externref),
  structured control flow, named imports/exports, validated
  bytecode. First applied consumer: spectral.engineer's GPU
  eigenboard `@ui` crate cascaded to browser. Bilateral:
  `rust_wasm_cascade_well_formed` composing `rust_well_typed` +
  `wasm_consumable` + `rust_wasm_loss_well_defined`. The
  precedent for the rustc-as-source-toolchain shape.

- **`@cascade/code/gleam/beam`** — Gleam → BEAM bytecode. Source
  grammar `@code/gleam`; target grammar `@code/beam`. CRITICAL
  ASYMMETRY: BEAM PRESERVES the concurrency dimension that JS
  erases. Loss profile substrate-architecturally SHALLOWER than
  Gleam → JS. Hot upgrade, supervision, preemptive scheduling all
  preserved per `@code/beam`'s composition with `@magic`
  (recognition #80). Bilateral:
  `gleam_beam_cascade_well_formed` composing `gleam_well_typed` +
  `beam_consumable` + `gleam_beam_loss_well_defined`.

- **`@cascade/code/gleam/js`** — Gleam → JavaScript (ES modules).
  Same source grammar as the BEAM sibling; different target. Loss
  surface includes concurrency-model erasure (no preemptive
  primitive on JS). Production consumer: spectral.engineer's
  content layer (systemic.engineering's typography corpus). The
  dual-target pair (beam + js) is the load-bearing validation of
  `@cascade`'s parametric form (one source grammar, two target
  species, asymmetric loss profiles).

- **`@cascade/code/purescript/js`** — PureScript → JavaScript.
  Source grammar `@code/purescript`; target grammar `@code/js`.
  Loss: row polymorphism, higher-kinded types, type classes,
  monad transformers (all erased at runtime). First applied
  consumer: StageFreight Stage-1 MVP target per Mara's cascade
  survey §2.3.

### 2.3 The common species pattern

Each landed species follows the same substrate-decl shape:

1. Declare the species's prism (`prism @cascade/code/<S>/<T> { five-op block }`).
2. Specialize the family-root's parametric carriers:
   - `<S>_source` ≡ `@cascade.typed_source` instantiated for source grammar.
   - `<T>_module` ≡ `@cascade.compiled_artifact` instantiated for target.
   - `<T>_metadata` — target-specific bundling metadata (wasm-pack
     metadata; beam application manifest; gleam.mjs companion).
   - `<T>_artifact` ≡ `labeled(<T>_module, <T>_metadata)` —
     uses the `@labeled` functor (recognition #93 H4) to bind
     module + metadata.
3. Specialize the family-root's actions:
   - `compile_<S>_<T>` — invokes the language-specific toolchain
     at the @io boundary.
   - `bundle_<T>` — constructs the labeled artifact via
     `@labeled.label`.
   - `measure_<S>_<T>` — computes substrate-typed loss between
     source's admitted grammar features and target's preserved
     ones; returns `imperfect<artifact, ref, information_loss>`.
4. Declare bilaterals:
   - `<S>_well_typed` — source-side typing discipline coheres
     under source language's typechecker.
   - `<T>_consumable` — target-side bundling coheres for consumer
     runtime ingestion.
   - `<S>_<T>_loss_well_defined` — substrate-typed gap calculation
     is computable for the (source, artifact) pair.
5. Compose the bilaterals into `<S>_<T>_cascade_well_formed`
   requiring all three sub-bilaterals.

The shape is **altitude-portable**: each new species instantiates
the same substrate-decl pattern with grammar-specific loss surface
and bilateral commitments. The FFI species (§4) follow this pattern
exactly.

### 2.4 Forward-promised species (FFI cases)

Mara's cascade survey (`docs/research/2026-06-23-typed-alternatives-
cascade-survey.md` §2.9) explicitly names "Rust → cdylib" as a
cascade case, with the honest hedge that the cascade pattern is
INTEROP via C ABI rather than COMPILE-TO at source level. Forward-
promised species from that survey, expanded for FFI:

- **`@cascade/code/rust/cdylib`** — Rust → C-ABI dynamic library.
  Source grammar `@code/rust`; target grammar `@code/c-abi-dylib`
  (the surface a dynamic linker sees: symbol table, calling
  convention, type-layout assumptions). Loss surface: lifetimes
  (erased at the ABI boundary; manual `Drop` discipline required),
  generics (monomorphized to concrete symbols; instantiation set
  must be enumerated at compile time), trait objects (devirtualized
  to concrete function pointers), macros (expanded; erased), type-
  level safety (erased; `repr(C)` types only crossing the boundary).
  Target preserves: C calling convention, raw pointers, sized C-
  compatible types (`u8`/`u16`/`u32`/`u64`/`f32`/`f64`/`*mut T`/
  `*const T`/`[T; N]`/`#[repr(C)] struct`), named exports.

- **`@cascade/code/rust/staticlib`** — Rust → static archive
  (.a / .lib). Same source grammar; target grammar
  `@code/c-abi-staticlib`. Same loss surface as cdylib; different
  linkage discipline (the consumer links at consumer-compile time
  rather than load time). Substrate-relevant difference: the
  artifact's resolved-symbol-set is fixed at consumer-compile time,
  not at consumer-run time; the link contract surfaces at a
  different timing altitude.

- **`@cascade/code/python/extension`** — Python-as-host loading
  compiled extensions (PyO3 case, the canonical Rust → Python
  extension shape). Source grammar `@code/rust` annotated with
  `@code/rust/macro/pyo3` (the metaprogramming surface for
  declaring Python-side type bindings); target grammar
  `@code/python/extension-abi` (the CPython C ABI for extension
  types: PyObject layout, the GIL discipline, the reference-
  counting contract, the slot table for type definitions). Loss
  surface: Rust's static type guarantees vanish at the Python
  boundary (Python sees `PyObject` references; the Rust type IS
  the runtime tag but the type system is not present); Rust's
  ownership / lifetime discipline gets translated to Python's
  refcount discipline (PyO3 manages the translation; the discipline
  difference IS the loss). GIL semantics surface at the boundary:
  every PyO3 call enters or exits the GIL; the substrate-typed
  contract must name this.

- **`@cascade/code/rust/fortran-ffi`** (or, symmetrically,
  `@cascade/code/fortran/c-abi-native`) — Rust ↔ Fortran. Both
  compile to native object code; both expose C-ABI symbols (Fortran
  via the `bind(C)` ISO C interoperability standard); the artifacts
  COMPOSE at link time through C as intermediary. Source grammars:
  `@code/rust` for the Rust side, `@code/fortran` for the Fortran
  side. Target grammar: `@code/c-abi-native` (the C ABI surface
  that both sides agree on). Loss surface (per-side, asymmetric):
  Rust side loses lifetimes / generics / traits / macros as
  cdylib does; Fortran side loses array-layout convention (Fortran
  is column-major; C is row-major; the boundary requires explicit
  transposition or layout annotation), complex-number representation
  (Fortran's intrinsic `complex` type is a single value; C
  conventionally uses two adjacent floats — the layout matches in
  practice but is not standardized at the C ABI level until
  `_Complex` in C99, and Fortran/C interop predates that), integer
  width assumptions (Fortran defaults to `INTEGER*4` but `INTEGER`
  can be redefined per-compiler; the C ABI side must pin width
  explicitly). This is the LOAD-BEARING case for Alex's flang /
  mirror numerical split work (T8 LRM; LAPACKPrism); §7 carries
  the case study.

- **`@cascade/code/c/static`** — C source → static archive. Source
  grammar `@code/c` (ISO C99 / C11 / C17 as the substrate-relevant
  source grammar); target grammar `@code/c-abi-staticlib`. Trivial
  loss surface from the cascade perspective (the source IS in the
  target ABI's grammar; loss is near-zero at the type altitude);
  the species exists to give substrate-altitude vocabulary to "I
  have C source; produce a linkable archive" workflows. Forward-
  promised lower-priority (the substrate's actual consumers tend
  to consume PRE-COMPILED C libraries via the runtime-link
  altitude, not source-cascade through C).

- **`@cascade/code/c/dylib`** — C source → dynamic library. Same
  as c/static but for dynamic linkage. Same lower priority.

The forward-promise list is OPEN-ENDED. Consumer-pull discharges
specific species when a consumer surfaces a need; the substrate-
decl shape stays fixed at the family-root.

### 2.5 The per-cascade species shape (FFI applied)

Each FFI cascade species declares (in the substrate-decl shard at
`shards/cascade/code/<source>/<target>.mirror`):

```
( source_grammar       — the @code/<S> grammar this species reads from
, target_grammar       — the @code/<T> grammar this species writes to
, loss_lens_components — the substrate-typed composite of @epistemologic/
                         properties accounting for what's lost across S → T
, abi_shape            — typed reference to the (symbol-table,
                         calling-convention, type-marshal-rules,
                         lifetime-contract) at the runtime-link altitude
                         the artifact will compose through
, runtime_link_kind    — discriminator naming which @io/runtime-link
                         species the artifact composes against (dylib /
                         staticlib / extension / fortran-bind-c)
)
```

The first two are the existing `@cascade.typed_source` and
`@cascade.compiled_artifact` parametric carriers, instantiated for
the species. The third is the species-specific loss vocabulary
(spelled out in each species's substrate-decl). The fourth and
fifth are NEW: they name the runtime-link altitude the artifact's
consumer will compose against. They are the bridge between
`@cascade` (compile-time) and `@io/runtime-link` (runtime-link).

§4 carries the substantive substrate-decl for each FFI species.
§3 carries the runtime-link altitude characterization.

---

## §3 — The runtime-link ABI altitude

### 3.1 Where it lives: under `@io`

The runtime-link altitude IS under `@io`, NOT under `@cascade`.

The reasoning: `@cascade` declares COMPILE-TIME functor structure
(source grammar to target grammar). The runtime-link IS NOT a
compile-time functor; it is a RUNTIME contract between the
producer (the compiled artifact's emitted symbol table) and the
consumer (the loading runtime's import expectations). The contract
is discharged WHEN the consumer's runtime LOADS the artifact, not
when the producer compiles it.

Per `@io`'s family-root substrate-decl: "@io is the substrate's
only legitimate non-mirror surface. Any grammar that isn't mirror
— Rust, Python, raw bytes, foreign blobs, vendor SDKs — must be
under @io. Everything else is mirror grammar by definition." The
runtime-link is by construction a non-mirror surface: the dynamic
linker IS a non-mirror runtime; the loaded artifact's interpreted
symbol table IS a non-mirror byte structure; the consumer runtime
(CPython's import machinery; the OS loader's `dlopen` /
`LoadLibrary` / `dyld`) IS a non-mirror system.

`@io` is the substrate-pull-correct family-root for the
runtime-link altitude. The proposal: ONE new `@io` species named
`@io/runtime-link` that declares the substrate-decl ground for ALL
runtime-link composition, with sub-prisms or sub-actions
specializing per link kind (dylib, staticlib, Python extension,
Fortran bind-c).

**Honest hedge (§8.1):** the alternative shape would be
multiple `@io` species (`@io/dylib`, `@io/staticlib`,
`@io/extension`, etc.) as siblings under `@io`. Both shapes are
substrate-pull-honest. The proposal chooses ONE species
(`@io/runtime-link`) because the substrate-decl pattern across the
link kinds is structurally identical (symbol resolution, calling
convention, type marshal, lifetime contract — same four
components); declaring once and parameterizing on `link_kind` is
the substrate-pull-correct move per recognition #93 H4's
preference for parametric carriers. The alternative shape (one
species per link kind) is forward-acceptable if the per-kind
discipline diverges enough to warrant separation; the substrate-
decl in §3.2 keeps the parametric form so divergence is decision-
deferred. **Open: §8.1.**

### 3.2 The ABI carrier and the link contract

The proposed substrate-decl shape for `@io/runtime-link`:

```mirror
in @prism
in @meta
in @glass
in @io
in @nl
in @cascade

prism @io/runtime-link {
  focus  runtime_link
  project runtime_link
  split  runtime_link
  shift  runtime_link
  settle runtime_link
}

# === link_kind discriminator ===
#
# Names which ABI surface the link composes against.
type link_kind =
  | dylib                    # OS dynamic linker (dlopen / LoadLibrary / dyld)
  | staticlib                # consumer-side linker (ld) at consumer build time
  | python_extension         # CPython import machinery (.so / .pyd)
  | fortran_bind_c           # Fortran/C interop via ISO_C_BINDING module
  | custom(text)             # forward: other host-runtime conventions

# === calling_convention discriminator ===
#
# Names the calling convention the link's symbols use. The substrate
# foreclosing mismatched conventions IS the boundary mathematics
# alignment recognition #57 names.
type calling_convention =
  | c                        # the universal C calling convention (cdecl-derived,
                              # platform-specific resolution)
  | system_v_amd64           # System V AMD64 ABI (the de-facto x86_64 Linux/macOS)
  | aapcs64                  # ARM AArch64 Procedure Call Standard
  | win64                    # Microsoft x64 calling convention (Windows)
  | stdcall                  # legacy Win32 callee-cleanup (rare; named for legacy reach)
  | fastcall                 # legacy register-first (rare; named for legacy reach)

# === symbol_export carrier ===
#
# Typed reference to the producer's emitted symbol table. The symbol
# table IS the substrate-typed surface the consumer's runtime imports
# against. Per [[feedback-no-bare-types]]: structured, not stringly.
#
# Identity contract: byte-equality on the underlying ref (the
# producer's emitted symbol table is content-addressable).
type symbol_export = ref

# === symbol_import carrier ===
#
# Typed reference to the consumer's expected import set. The consumer
# runtime (or its language's FFI declaration surface — Python ctypes
# / cffi / PyO3, C #include, Fortran USE statement) names the
# symbols it expects.
#
# Identity contract: byte-equality on the underlying ref.
type symbol_import = ref

# === type_marshal carrier ===
#
# Typed reference to the type-marshalling rules the link applies at
# the boundary. Each link_kind has a default marshal table (C ABI:
# scalar types passed by value; struct types passed by-ref via
# repr(C) layout; lifetime contract is caller-manages-allocation by
# default). The carrier is opaque-by-default at the substrate
# altitude; per-species bodies discharge the actual marshal rules.
#
# Identity contract: byte-equality on the underlying ref.
type type_marshal = ref

# === lifetime_contract carrier ===
#
# Typed reference to the per-side memory / resource lifetime
# discipline. Each link kind imposes a per-side contract; the
# substrate-decl carrier names the contract; the per-species body
# enumerates the contract's terms.
#
# Examples:
#   dylib:             caller owns args; callee owns returns;
#                       explicit drop functions for callee-owned heap.
#   python_extension:  PyO3 manages refcount on PyObject crossings;
#                       Rust lifetimes elaborated to acquire-release
#                       pairs around GIL boundaries.
#   fortran_bind_c:    array ownership passes by descriptor (Fortran
#                       allocates) OR by raw pointer (Rust allocates);
#                       species declares per-call which.
#
# Identity contract: byte-equality on the underlying ref.
type lifetime_contract = ref

# === abi_surface carrier ===
#
# THE LOAD-BEARING NEW CARRIER. The substrate-typed reference to
# the link's complete ABI shape. An abi_surface IS a typed tuple
# (link_kind, calling_convention, symbol_export, symbol_import,
# type_marshal, lifetime_contract).
#
# The carrier names the ABI as a substrate-typed object so the link
# can be discharged statically — before the runtime loader is
# invoked, the substrate can check that the producer's export agrees
# with the consumer's import, the calling convention is consistent,
# the type marshal composes, and the lifetime contract is honored.
#
# Per [[architecture-glass-wall-substrate-types]]: the wall this
# species declares the typed shape of. The substrate cannot fold
# past the dynamic linker; but the substrate CAN type the ABI surface
# the linker will operate on.
#
# Identity contract: byte-equality on the underlying ref (the ABI
# surface IS its tuple).
type abi_surface = ref

# === link action ===
#
# THE LOAD-BEARING ACTION. Takes a compiled_artifact (from any
# @cascade species discharging to a C-ABI-compatible target) plus
# the consumer's symbol_import declaration plus the abi_surface
# the link will operate on, and returns either a settled link
# (imperfect's success branch) or an opacity surfaced at the link
# altitude (imperfect's failure / partial branch).
#
# Returns `imperfect<linked_artifact, ref, transparency>`:
# - success(linked_artifact) when the artifact loads, symbols
#   resolve, marshal composes, and lifetime contract is statically
#   honor-able.
# - partial(linked_artifact, transparency) when the link succeeds
#   but with substrate-typed warnings (e.g., one symbol matched
#   loosely; one lifetime contract clause requires runtime
#   discipline the consumer must honor).
# - failure(error, transparency) when the link cannot proceed:
#   missing symbol; calling convention mismatch; marshal undefined
#   for one of the boundary types; lifetime contract incompatible.
#
# requires runtime_link_safe(artifact, import, abi, p): the
# bilateral that gates the link. Per §3.4.
#
# Body discharges at the realisation boundary (the actual dynamic
# linker invocation, or the consumer build-time linker invocation
# for staticlib, or the CPython extension load for python_extension).
link(artifact: compiled_artifact, import: symbol_import,
     abi: abi_surface, p: perturbation)
  -> imperfect<linked_artifact, ref, transparency>
requires runtime_link_safe(artifact, import, abi, p)
{ \ }

# === linked_artifact carrier ===
#
# The post-link runtime-handle the consumer can invoke through.
# Opaque from the substrate's vantage (the substrate cannot reason
# about the runtime-loaded function pointer's behavior); the carrier
# IS the substrate's typed handle to the link's outcome.
#
# Identity contract: byte-equality on the underlying ref.
type linked_artifact = ref

out @io/runtime-link
out link_kind
out calling_convention
out symbol_export
out symbol_import
out type_marshal
out lifetime_contract
out abi_surface
out linked_artifact
out link
```

The carrier set is FIVE typed references plus TWO discriminator
enums plus ONE composed carrier. The action set is ONE: `link`.
The bilateral set (§3.4) is FOUR sub-predicates composed into ONE
composed-bilateral. The substrate-decl surface is small;
discipline-rich.

### 3.3 Per-link-kind examples

**Rust cdylib loaded by Python (PyO3 case).**

Producer side: a Rust crate with `crate-type = ["cdylib"]` and PyO3
annotations on the exported types. The compile-time cascade is
`@cascade/code/python/extension` (NOT `@cascade/code/rust/cdylib`
on its own; the PyO3 macro layer at compile time rewrites the Rust
source to emit CPython-extension-compatible symbols, so the cascade
shape is Rust+PyO3 → CPython extension ABI, not bare Rust → cdylib).

Runtime-link side: Python's `import` machinery loads the resulting
`.so` (Linux) / `.pyd` (Windows) via the CPython extension loader.
The `abi_surface` declared at the link altitude:
- `link_kind = python_extension`
- `calling_convention = c` (the CPython extension ABI uses C
  calling convention)
- `symbol_export = <ref to the PyInit_<modulename> symbol the
  crate emits via PyO3's #[pymodule] macro>`
- `symbol_import = <ref to Python's expected PyInit_<modulename>
  signature; pinned to CPython version 3.x>`
- `type_marshal = <ref to the PyObject marshal rules; PyO3 manages
  the Rust ↔ PyObject translation per its derive macros>`
- `lifetime_contract = <ref to the GIL discipline + the refcount
  contract; PyO3's Bound<'py, T> lifetime tracking elaborates the
  Rust side; the contract is asymmetric: the Rust side promises
  to manage refcounts via PyO3's smart pointers, the Python side
  promises to honor the GIL across calls>`

The link bilateral `runtime_link_safe` discharges when the abi
surface's six components agree across producer and consumer. The
substrate can statically verify the PyInit symbol exists (symbol
resolution); the calling convention is C (consistent with Python's
import machinery); the type marshal is well-defined for the
boundary types PyO3 declared; the GIL discipline is honored
(elaborated through the type system on the Rust side; promised by
the Python side at every call). If any sub-predicate fails (e.g.,
the Rust crate declares a Python type whose marshal PyO3 cannot
derive, like a Rust `Mutex<T>` without `pyclass`), the link
bilateral surfaces the opacity at the @io altitude.

**Rust staticlib linked by C.**

Producer side: a Rust crate with `crate-type = ["staticlib"]` and
`#[no_mangle] extern "C"` exports. The compile-time cascade is
`@cascade/code/rust/staticlib`.

Runtime-link side: at consumer-compile time (not consumer-run
time), the C compiler's linker (`ld` on Linux/macOS; `link.exe`
on Windows) consumes the `.a` / `.lib` and resolves symbols into
the consumer's final binary. The `abi_surface`:
- `link_kind = staticlib`
- `calling_convention = c`
- `symbol_export = <ref to the #[no_mangle] symbol set>`
- `symbol_import = <ref to the C consumer's extern declarations
  (typically generated via cbindgen from the Rust source)>`
- `type_marshal = <ref to the C ABI scalar / repr(C) struct marshal
  rules; cbindgen elaborates the consumer-side declarations>`
- `lifetime_contract = <ref to the per-function memory contract;
  Rust must expose drop functions for any heap allocation that
  crosses the boundary; C consumer must call them at end-of-use>`

The link timing differs from dylib (consumer-compile time vs
consumer-run time); the substrate-decl SHAPE is identical
(symbol resolution, calling convention, type marshal, lifetime
contract). The `link_kind = staticlib` discriminator surfaces the
timing difference to consumers who care.

**Rust calling Fortran (LAPACK case).**

Producer side: a Fortran-compiled LAPACK routine emitting C-ABI
symbols via the `bind(C, name="...")` ISO C interoperability
declaration. The compile-time cascade is `@cascade/code/fortran/
c-abi-native`.

Consumer side: a Rust crate declaring `extern "C"` bindings to
the LAPACK symbols (typically via the `lapack-sys` crate or
direct hand-rolled bindings). The compile-time cascade for the
Rust side is `@cascade/code/rust/cdylib` if the Rust crate is
itself a dylib, else N/A (the Rust binary links the LAPACK
staticlib or dylib at consumer-compile or consumer-load time).

Runtime-link `abi_surface`:
- `link_kind = fortran_bind_c` (or `dylib` / `staticlib` depending
  on how LAPACK is shipped; OpenBLAS ships both)
- `calling_convention = c` (via `bind(C)`)
- `symbol_export = <ref to LAPACK's bind(C) symbol set; the
  underscore-suffixed convention or the bind(C, name=...) explicit
  naming, depending on the Fortran compiler>`
- `symbol_import = <ref to the Rust extern "C" declarations>`
- `type_marshal = <ref to the asymmetric array-layout contract;
  Fortran's column-major default vs C's row-major; the marshal
  rule names which side does the transposition (or whether
  caller passes the column-major-as-flat-array and callee
  interprets it directly via leading-dimension parameter, which
  IS the LAPACK convention)>`
- `lifetime_contract = <ref to the per-call ownership contract;
  for LAPACK: caller allocates input + output arrays; callee
  writes through caller-provided pointers; no heap allocation
  crosses the boundary; the leading-dimension parameter
  preserves the layout contract>`

The LAPACK case is the LOAD-BEARING case for §7. The substrate-
typed link contract IS what gates Alex's LapackBackend wiring
work; without the contract typed at substrate altitude, every
LAPACK call would have to re-justify the marshal + ownership
discipline ad-hoc. With the contract typed, the substrate-
altitude binding becomes a single `link` call against a fixed
`abi_surface` registered for LAPACK once.

### 3.4 The link bilateral

The composed-bilateral pattern this spec follows (per the
substrate's altitude-portable composed-bilateral discipline
recognition #57 + #95):

```mirror
# === symbols_resolve bilateral (sub) ===
#
# Splinter-pole: every symbol_import name appears in symbol_export
# with compatible signature.
# Narcissus-pole: some import unresolved OR signature mismatch.
symbols_resolve(import: symbol_import, export: symbol_export,
                p: perturbation) -> verdict { \ }

# === calling_convention_agrees bilateral (sub) ===
#
# Splinter-pole: producer's emitted convention matches consumer's
# expected convention (typically c on c).
# Narcissus-pole: convention mismatch (e.g., producer emits System V
# AMD64 but consumer expects Win64).
calling_convention_agrees(producer_cc: calling_convention,
                          consumer_cc: calling_convention,
                          p: perturbation) -> verdict { \ }

# === type_marshal_composes bilateral (sub) ===
#
# Splinter-pole: every type crossing the boundary has a defined
# marshal rule in the type_marshal carrier; the marshal is reversible
# (round-trip preserves identity) for value types and respects layout
# for repr(C) types.
# Narcissus-pole: undefined marshal for at least one boundary type;
# OR marshal not reversible (information loss across the boundary
# beyond what the cascade's loss_lens declares).
# Narcissus-warned: marshal defined but with substrate-typed warnings
# (e.g., variable-length-array marshal requires explicit length
# parameter the consumer must promise to provide).
type_marshal_composes(marshal: type_marshal, p: perturbation)
  -> verdict { \ }

# === lifetime_contract_honored bilateral (sub) ===
#
# Splinter-pole: producer's lifetime contract clauses are statically
# honor-able by the consumer's import discipline; the contract has
# explicit per-direction ownership statements; resource cleanup is
# either deterministic or explicitly delegated.
# Narcissus-pole: contract clause cannot be honored statically (e.g.,
# callback re-entrancy not declared; ownership ambiguous on one
# direction; resource cleanup neither deterministic nor delegated).
# Narcissus-warned: contract honor-able but with runtime discipline
# the consumer must adopt (e.g., GIL release/acquire around callbacks).
lifetime_contract_honored(contract: lifetime_contract,
                          p: perturbation) -> verdict { \ }

# === runtime_link_safe composed bilateral ===
#
# THE LOAD-BEARING composed bilateral. Composes the four sub-
# predicates at this altitude.
#
# All four must hold; substrate refuses link if any sub-predicate
# fails. The same composed-bilateral pattern as @cascade.cascade_
# well_defined, @epistemologic/neutrosophic.three_axis_coherent,
# @io/stagefreight.stagefreight_addressable, @reflection.third_
# order_coherent, @smarts/shatter.shatter_round_trip, and the per-
# cascade-species composed bilaterals (rust_wasm_cascade_well_formed,
# gleam_beam_cascade_well_formed, etc.). ~14th altitude lift of the
# composed-bilateral pattern. (Counted: cascade_well_defined at 5th,
# rust_wasm at 6th, tea at 8th, nl/design at 9th, spectral-engineer-
# case-study at 10th, oci at ~12th, git at ~13th; sibling forward-
# promises at peer 14th + mirror_spawn 15th.) Substrate's discipline
# is altitude-portable. Seam tick (docs/audits/2026-06-28-seam-
# mirror-build-substrate-composite) S-1 closure: morning draft
# under-counted at "seventh"; substrate-pull-honest count makes the
# altitude-portability argument stronger, not weaker.
#
# Splinter-pole: symbols resolve + convention agrees + marshal
# composes + lifetime honor-able; link discharges clean.
# Narcissus-pole: at least one sub-predicate fails; link blocked;
# transparency<p> reports the opacity at the @io altitude.
# Narcissus-warned: all four sub-predicates discharge but at least
# one is warned; link operates with documented runtime discipline.
runtime_link_safe(artifact: compiled_artifact,
                  import: symbol_import,
                  abi: abi_surface,
                  p: perturbation) -> verdict
requires symbols_resolve(import, export_of(artifact, abi), p)
requires calling_convention_agrees(producer_cc_of(abi),
                                   consumer_cc_of(abi), p)
requires type_marshal_composes(marshal_of(abi), p)
requires lifetime_contract_honored(contract_of(abi), p)
{ \ }
```

The composed bilateral IS the substrate-decl's discharge of the
runtime-link altitude's discipline. The four sub-predicates name
the four structural risks of cross-language linking; the substrate
forecloses link operation if any one fails. The opacity surfaces
at the @io altitude per the `transparency<p>` discipline already
declared at `@mirror/loss/transparency` (recognition #59 the
altitude-portable kintsugi loop; the substrate-altitude lift here
is the ~14th landed instance — see the composed-bilateral count
in the comment block above).

---

## §4 — Cascade species discharge (the new substrate-decl work)

The substantive substrate-decl work. Each subsection drafts the
shape of one forward-promised FFI cascade species. The shards are
NOT landed in this commit (markdown only); the substrate-decl
shapes ARE forward-promised, with per-species discharge timing
keyed to consumer-pull.

### 4.1 `@cascade/code/rust/cdylib`

Shard path (forward-promise): `shards/cascade/code/rust/cdylib.mirror`.

Source grammar: `@code/rust` (Rust source compiled with
`crate-type = ["cdylib"]` in `Cargo.toml` or equivalent
`--crate-type cdylib` rustc flag).

Target grammar: `@code/c-abi-dylib` (forward-promised; the
substrate-decl for the C ABI dynamic library surface; would
declare the symbol-table layout, the calling-convention map, the
repr(C) type-layout discipline, the platform-specific dylib
container format (ELF .so, Mach-O .dylib, PE .dll)).

**Loss surface (per recognition #95 substrate-typed):**

- **Lifetimes (erased).** Rust's lifetime annotations vanish at
  the ABI boundary. The cdylib's exported `extern "C" fn` symbols
  carry raw pointers; the lifetime annotations that statically
  guaranteed validity in Rust are erased. The consumer must
  promise (per the lifetime contract) to honor the implicit
  validity window the Rust API was designed around.

- **Generics (monomorphized to concrete symbols).** Rust generics
  do not survive the ABI: `fn foo<T>(x: T)` cannot be exported as
  a single C ABI symbol. The crate author must monomorphize
  explicitly (`#[no_mangle] extern "C" fn foo_u32(x: u32)`,
  `#[no_mangle] extern "C" fn foo_u64(x: u64)`, etc.). The
  instantiation set IS the cdylib's exported surface; what the
  source's generic admits in unbounded form, the cdylib admits
  only in enumerated form. The loss IS the enumeration delta.

- **Trait objects (devirtualized).** Rust trait objects compile to
  function-pointer-table vtables under the hood; at the C ABI
  surface, the vtable is opaque (consumer cannot construct one
  without knowing the layout, which is rustc-version-sensitive).
  Crate authors typically expose a manually-defined function-
  pointer struct (`repr(C) struct MyVtable { method_a: extern "C"
  fn(...), method_b: extern "C" fn(...) }`); the loss IS the
  trait's compile-time dispatch guarantees becoming runtime
  function-pointer discipline the consumer must honor.

- **Macros (expanded; erased).** Rust macros (declarative
  `macro_rules!`, procedural `#[proc_macro]`, attribute macros)
  expand at compile time; the expanded source IS what compiles;
  the macro syntax does not survive into the cdylib. The loss IS
  zero at the ABI altitude (the consumer would not see macros
  anyway); the loss IS observable at the substrate altitude (the
  consumer cannot re-invoke the macro from the cdylib; the
  expansion is one-shot).

- **Type-level safety (erased).** Rust's type system enforces
  guarantees the C ABI cannot encode: `Result<T, E>` discrimination
  (which the consumer must check via the C ABI struct's tag
  field, but the consumer COULD ignore the tag and dereference the
  wrong variant — Rust would have rejected that at compile time);
  `Option<T>` non-null discipline (which the consumer COULD ignore
  by passing a null pointer where a `Option::Some(_)` was expected);
  `&mut T` aliasing discipline (which the consumer COULD violate
  by aliasing); enum exhaustiveness (which the consumer COULD
  ignore in switch statements). The substrate cannot fold past
  the C ABI's lack of these checks; the loss IS the consumer's
  discipline-burden the type system used to discharge.

**Target preserves (substrate-typed):**

- C calling convention (per the platform's C ABI: System V AMD64
  on Linux/macOS x86_64; AAPCS64 on Linux/macOS ARM64; Win64 on
  Windows x86_64; etc.).
- Raw pointers (`*mut T`, `*const T`) without bounds-checking or
  validity guarantees.
- Sized C-compatible scalar types (`u8`, `u16`, `u32`, `u64`,
  `i8`, `i16`, `i32`, `i64`, `f32`, `f64`, `isize`/`usize`
  matching platform pointer width).
- Fixed-size arrays (`[T; N]` where T is C-compatible and N is a
  compile-time constant).
- `#[repr(C)]` structs (field layout matches C's, padding rules
  match C's).
- `#[repr(C)]` enums (tag layout follows C's discriminant
  convention; the `#[repr(C, u8)]` / `#[repr(C, u32)]` discipline
  pins tag size explicitly).
- Named exports via `#[no_mangle]` attribute (the symbol appears
  in the cdylib's symbol table under the exact name; without
  `#[no_mangle]` the symbol would be rustc-mangled and unusable
  by C consumers).

**Bilateral chain (per §2.5 species pattern):**

```mirror
# === rust_well_typed_for_cdylib bilateral ===
#
# Source-side. Stricter than the generic rust_well_typed (which is
# in the rust/wasm sibling): rust_well_typed_for_cdylib ALSO checks
# that all exported items are #[no_mangle] extern "C", that all
# parameter and return types are C-ABI-compatible, that no exported
# function panics across the boundary (panics across FFI are
# undefined behavior; the substrate-typed check catches this).
#
# Splinter-pole: source type-checks AND all exports are C-ABI-clean.
# Narcissus-pole: type errors OR exports use non-FFI-safe types OR
# exports may panic.
# Narcissus-warned: type-clean but exports use catch_unwind for
# explicit panic-boundary discipline (substrate-typed advisory).
rust_well_typed_for_cdylib(source: rust_source, p: perturbation)
  -> verdict { \ }

# === cdylib_consumable bilateral ===
#
# Target-side. Commitment that the cdylib artifact's emitted
# symbol-table + repr(C) layout + calling convention is consistent
# with the declared target platform's C ABI.
#
# Splinter-pole: cdylib loads via dlopen on declared platform;
# symbols resolve as declared; no rustc-version layout drift.
# Narcissus-pole: cdylib fails to load OR symbols missing OR
# layout drift detected against declared C ABI.
cdylib_consumable(artifact: cdylib_artifact, p: perturbation)
  -> verdict { \ }

# === rust_cdylib_loss_well_defined bilateral ===
#
# Substrate-typed gap calculation IS computable for the (source,
# artifact) pair: source is in @code/rust grammar, artifact is in
# @code/c-abi-dylib grammar, the (source, artifact) pair represents
# a valid cascade discharge.
rust_cdylib_loss_well_defined(source: rust_source,
                              artifact: cdylib_artifact,
                              p: perturbation) -> verdict { \ }

# === rust_cdylib_cascade_well_formed composed bilateral ===
#
# All three sub-bilaterals must hold. Same composed-bilateral pattern
# as the other cascade species. ~15th altitude lift of the composed-
# bilateral pattern (one above runtime_link_safe's ~14th; sibling
# count parity with mirror_spawn's forward-promised 15th). Seam tick
# S-1 closure: morning draft under-counted at "eighth"; the count is
# the same substrate-roster drift Mara acknowledged in mirror-build-
# substrate.md §11 (carried forward from older context).
#
# Tightened to the substrate-pull-honest count: the discipline
# argument is stronger at the true altitude.
rust_cdylib_cascade_well_formed(source: rust_source,
                                artifact: cdylib_artifact,
                                p: perturbation) -> verdict
requires rust_well_typed_for_cdylib(source, p)
requires cdylib_consumable(artifact, p)
requires rust_cdylib_loss_well_defined(source, artifact, p)
{ \ }
```

**Compile-time → runtime-link bridge:**

The cdylib species declares its `abi_shape` (per §2.5):

- `runtime_link_kind = dylib`
- `calling_convention = c` (platform-specific resolution)
- The exported `symbol_export` ref points to the rustc-emitted
  symbol table (constructed via `nm`/`dumpbin`/`otool` at
  realisation boundary).
- The `type_marshal` ref points to the cdylib's repr(C) type
  layout table (constructed via cbindgen or hand-rolled at the
  Rust side).
- The `lifetime_contract` ref points to the per-function ownership
  contract the crate author documented (no substrate-altitude
  automatic synthesis today; consumer-discharge per crate).

The bridge means: when a downstream consumer's mirror.spec
declares a runtime-link against the cdylib, the consumer's
`@io/runtime-link.link` call receives the `abi_surface` from the
cdylib species's substrate-decl. The link contract is statically
checkable BECAUSE the cascade species declared the abi_shape at
compile-time.

### 4.2 `@cascade/code/rust/staticlib`

Shard path (forward-promise): `shards/cascade/code/rust/staticlib.mirror`.

Source grammar: `@code/rust` (Rust source compiled with
`crate-type = ["staticlib"]`).

Target grammar: `@code/c-abi-staticlib` (forward-promised; the
substrate-decl for the C ABI static archive surface).

**Loss surface:** same as cdylib (lifetimes, generics, trait
objects, macros, type-level safety all erased identically). The
cascade shape is identical; the artifact format differs (archive
of object files vs single shared library).

**Bilateral chain:** identical shape to cdylib with substituted
target grammar; `rust_staticlib_cascade_well_formed` composing
`rust_well_typed_for_staticlib` + `staticlib_consumable` +
`rust_staticlib_loss_well_defined`.

**Compile-time → runtime-link bridge:**

- `runtime_link_kind = staticlib`
- `calling_convention = c`
- The link timing is consumer-compile time (not consumer-run time);
  the `link` action discharges at consumer build time when the
  C/C++ consumer's linker (`ld`, `link.exe`) consumes the `.a` /
  `.lib`.
- The `abi_surface` is otherwise identical to cdylib's.

The substrate-decl distinction (staticlib vs cdylib) IS the
`link_kind` discriminator and the timing difference it implies.

### 4.3 `@cascade/code/python/extension`

Shard path (forward-promise):
`shards/cascade/code/python/extension.mirror`.

Source grammar: `@code/rust` annotated with
`@code/rust/macro/pyo3` (the PyO3 attribute-macro surface for
declaring Python-side type bindings). Alternative source grammars
exist (C with the Python C API directly; Cython; Nim with nimpy);
the canonical case for this species is Rust + PyO3 because Rust
+ PyO3 IS the dominant production cascade for type-safe Python
extensions in 2026.

Target grammar: `@code/python/extension-abi` (forward-promised;
the substrate-decl for the CPython extension ABI: PyObject
layout, GIL discipline, refcount contract, slot tables, the
PyInit\_<modulename> entry-point convention).

**Loss surface:**

- **Static type erasure (Python boundary).** Rust's type system
  guarantees vanish when crossing into Python; the Python side
  sees `PyObject` references the runtime tags but does not
  type-check. PyO3 elaborates conversions (`FromPyObject` /
  `IntoPyObject` traits) on the Rust side, but the Python side
  has no static check; the loss IS Python's dynamic dispatch
  becoming load-bearing at the boundary.

- **GIL semantics surface.** Rust's `Send` / `Sync` discipline
  partially translates to PyO3's `Bound<'py, T>` lifetime
  tracking, but every PyO3 call enters or exits the GIL: the GIL
  IS a runtime invariant the Rust side must promise to respect.
  The loss is structural — Rust would have allowed parallel work
  on `Send` types; PyO3 + GIL serializes parallel work through
  the GIL.

- **Refcount contract.** Python's refcount discipline (every
  PyObject reference is owned or borrowed; ownership must be
  explicitly released or transferred) translates to PyO3's smart-
  pointer types (`Py<T>`, `Bound<'py, T>`); the Rust side
  promises to use PyO3's types, not raw `*mut PyObject`. The
  loss IS Rust's ownership-by-move-semantics becoming Python's
  ownership-by-explicit-refcount.

- **Slot table discipline.** Custom Python types defined in the
  extension declare a slot table (tp_new, tp_init, tp_dealloc,
  tp_repr, etc.); PyO3 generates the slot table from
  `#[pyclass]` annotations; the loss IS Rust's type-system-based
  trait dispatch becoming CPython's slot-table runtime dispatch.

**Target preserves:**

- CPython's C extension ABI (PEP 3121 / PEP 489 module
  initialization; the slot-table type system; the GIL).
- The `PyInit_<modulename>` symbol convention (the dynamic loader
  resolves this symbol when `import <modulename>` is invoked).
- Python's `import` machinery (sys.path resolution; .pyi stub
  files for static type checking on the Python side via mypy).

**Bilateral chain:**

```mirror
rust_pyo3_well_typed(source: rust_pyo3_source, p: perturbation)
  -> verdict { \ }
python_extension_consumable(artifact: python_extension_artifact,
                            p: perturbation) -> verdict { \ }
rust_python_loss_well_defined(source: rust_pyo3_source,
                              artifact: python_extension_artifact,
                              p: perturbation) -> verdict { \ }
rust_python_cascade_well_formed(source: rust_pyo3_source,
                                artifact: python_extension_artifact,
                                p: perturbation) -> verdict
requires rust_pyo3_well_typed(source, p)
requires python_extension_consumable(artifact, p)
requires rust_python_loss_well_defined(source, artifact, p)
{ \ }
```

**Compile-time → runtime-link bridge:**

- `runtime_link_kind = python_extension`
- `calling_convention = c`
- `symbol_export = <ref to PyInit_<modulename> symbol>`
- `type_marshal = <ref to PyO3-managed PyObject translation>`
- `lifetime_contract = <ref to GIL + refcount discipline; asymmetric
  per-direction; PyO3 manages Rust side via Bound<'py, T>>`

The substrate-altitude binding lets a downstream consumer
(a Python project's mirror.spec) declare a runtime-link against
the extension and get static substrate-typed verification of the
PyO3 contract.

### 4.4 `@cascade/code/rust/fortran-ffi` (or `@cascade/code/fortran/c-abi-native`)

Shard path naming: ambiguous. Two substrate-pull-honest options:

1. **`@cascade/code/rust/fortran-ffi`** — frames the cascade
   from the Rust side: Rust source consumes Fortran libraries via
   extern "C" bindings. The cascade is the Rust crate's compile-
   time emission of the binding stubs that match Fortran's
   bind(C) exports.

2. **`@cascade/code/fortran/c-abi-native`** — frames the cascade
   from the Fortran side: Fortran source compiled with bind(C)
   declarations emits C-ABI-compatible symbols. The cascade is
   Fortran source → native object code carrying C-ABI symbols.

Both are real cascades; the difference is which side this species
discharges. The substrate-pull-correct move is to declare BOTH
species (sibling shards under `shards/cascade/code/`) because the
LAPACK case requires both: someone compiled LAPACK's Fortran
source (that's the `fortran/c-abi-native` cascade); someone wrote
a Rust binding against it (that's the `rust/fortran-ffi` cascade);
the two compose at the runtime-link altitude.

**4.4.1 `@cascade/code/fortran/c-abi-native`**

Source grammar: `@code/fortran` (Fortran 2003 / 2008 / 2018 — the
substrate-relevant source grammar; ISO_C_BINDING module is the
load-bearing intrinsic).

Target grammar: `@code/c-abi-native` (same as cdylib / staticlib;
the platform's C ABI surface).

**Loss surface:**

- **Array-layout convention.** Fortran is column-major by default;
  C is row-major. The boundary requires either explicit
  transposition (copy at the call site) OR the LAPACK convention
  (caller passes layout-as-data via the leading-dimension
  parameter; callee interprets accordingly). The loss IS the
  semantic gap between Fortran's array intrinsic (which knows the
  layout) and C's raw pointer (which does not).

- **Complex-number representation.** Fortran's intrinsic
  `complex` is a single value with type-system support; C uses
  `_Complex` (C99) or, more conventionally, two adjacent floats
  / doubles. The bit-layout matches in practice but is not
  standardized at the C ABI level pre-C99; the loss IS the type-
  system-supported complex becoming a layout convention.

- **Integer width.** Fortran's `INTEGER` is compiler-configurable;
  `INTEGER*4` and `INTEGER*8` pin width explicitly; the C ABI
  side must pin width via `c_int32_t` / `c_int64_t` from
  ISO_C_BINDING. The loss IS Fortran's defaulting becoming an
  explicit-width contract.

- **Character handling.** Fortran's `CHARACTER` strings carry
  length as part of the type; C uses null-terminated arrays or
  length-as-separate-parameter. The bind(C) discipline pins the
  C convention; the loss IS Fortran's length-as-type becoming
  a separate parameter.

- **Allocatable arrays.** Fortran's `ALLOCATABLE` arrays carry
  shape information; C interop typically passes by descriptor
  (Fortran 2018's ISO_Fortran_binding.h provides
  CFI_cdesc_t) OR by raw pointer with separate shape parameters.

**Target preserves:**

- C calling convention.
- bind(C) symbol names (no Fortran name-mangling when bind(C, name=)
  is used; default name-mangling otherwise — typically lowercase
  with trailing underscore on gfortran).
- The platform's native ABI for the symbols emitted.

**Bilateral chain:**

```mirror
fortran_well_typed(source: fortran_source, p: perturbation)
  -> verdict { \ }
c_abi_native_consumable(artifact: c_abi_native_artifact,
                        p: perturbation) -> verdict { \ }
fortran_c_loss_well_defined(source: fortran_source,
                            artifact: c_abi_native_artifact,
                            p: perturbation) -> verdict { \ }
fortran_c_cascade_well_formed(source: fortran_source,
                              artifact: c_abi_native_artifact,
                              p: perturbation) -> verdict
requires fortran_well_typed(source, p)
requires c_abi_native_consumable(artifact, p)
requires fortran_c_loss_well_defined(source, artifact, p)
{ \ }
```

**4.4.2 `@cascade/code/rust/fortran-ffi`**

Source grammar: `@code/rust` (a Rust crate declaring `extern "C"`
bindings).

Target grammar: NOT a new code grammar; this species's `target`
is the C ABI surface a Fortran library has already emitted (per
4.4.1). The species's role is to declare that the Rust side's
binding stubs are well-formed against an expected Fortran
bind(C) surface.

**Loss surface:** the Rust side's loss is the union of:
- Cdylib loss (lifetimes, generics, traits, macros, type-level
  safety erased at the extern "C" boundary).
- PLUS the array-layout discipline burden: the Rust binding must
  encode (in its safe wrapper API) the column-major-vs-row-major
  contract LAPACK callers expect; the loss IS the wrapper's burden
  to translate idiomatic Rust array types (`ndarray::Array2`,
  `nalgebra::DMatrix`) into LAPACK's leading-dimension convention.

**Bilateral chain:** analogous to the others; substrate-altitude
discipline that the Rust binding's extern "C" signatures match
the Fortran bind(C) exports byte-for-byte.

**Compile-time → runtime-link bridge:**

- `runtime_link_kind = fortran_bind_c` (or `dylib` / `staticlib`
  depending on LAPACK's distribution; OpenBLAS ships both).
- `calling_convention = c` (via bind(C)).
- The composed `abi_surface` MUST agree across the Rust binding
  side and the Fortran emit side: the same `symbol_export` ref
  must be readable from both sides; the substrate-altitude link
  bilateral discharges when they agree.

### 4.5 The common species shape (FFI applied — summary)

Per §2.5 the common shape:

```
( source_grammar
, target_grammar
, loss_lens_components
, abi_shape       ← LINKS COMPILE-TIME TO RUNTIME-LINK
, runtime_link_kind ← DISCRIMINATOR
)
```

The FFI species (4.1–4.4) all follow this shape. The novelty is
the `abi_shape` + `runtime_link_kind` fields, which the existing
landed species (rust/wasm, gleam/beam, gleam/js, purescript/js)
do NOT need because their target runtimes (browser WASM, BEAM,
ES module loader) ARE the consumer runtime — there's no second-
party runtime-link altitude to compose against. For FFI cases the
artifact is consumed by ANOTHER language's runtime, so the
runtime-link altitude becomes load-bearing.

The substrate-decl shape stays parametric: the cascade species
captures source / target / loss; the runtime-link altitude
captures abi / link kind / contract. Each altitude has its own
bilateral chain. The composition is what makes cross-language
work substrate-typed.

---

## §5 — How this composes with `@mirror/mosaic`

The substrate's build system IS `@mirror/mosaic`
(`shards/mirror/mosaic.mirror`). The cascade species + runtime-
link altitude this spec characterizes MUST compose cleanly with
mosaic's existing dispatch.

### 5.1 The dispatch chain

Per `shards/mirror/mosaic.mirror` lines 132-148, the five-op
sequence is:

```mirror
focus(spec) -> manifold         # read mirror.spec
project(manifold, targets) -> resolved
split(resolved) -> [shard]
shift(resolved, altitude) -> emitter
settle(emitter) -> imperfect(au, error, transparency)
```

The `shift` step is where altitude-specific delegation happens:
each target's declared altitude (`@code/rust`, `@code/python`,
`@code/fortran`, etc.) routes the resolved manifold to its per-
altitude emitter. Per mosaic.mirror §"cargo @io contract":

> When shift hits @code/rust, the altitude grammar (@code/rust,
> NOT mosaic) delegates to cargo through @io. Mosaic does not
> know about cargo — it knows about @code/rust, which knows about
> cargo. The substrate-pull goes through the altitude.

For FFI targets, the per-altitude emitter calls into the per-
language @io species (per `mirror-build-substrate.md` §5 amended
plan: `@io/cargo` for Rust; `@io/uv` or `@io/pip` for Python;
`@io/gfortran` or `@io/flang` for Fortran; etc.).

### 5.2 Where the cascade species comes in

When `@io/cargo.build` is called with `--crate-type cdylib` (or
the equivalent through `Cargo.toml` `[lib]` section), the
substrate-typed loss accounting for the produced artifact IS
discharged through `@cascade/code/rust/cdylib.measure_rust_cdylib`.

The chain:

```
mosaic.shift @code/rust(resolved)
  → @code/rust delegates to @io/cargo.build(... profile, env, features ...)
  → @io/cargo.build produces a cdylib artifact (the .so / .dylib /
    .dll)
  → @cascade/code/rust/cdylib.measure_rust_cdylib(source, artifact, p)
    discharges the loss accounting (information_loss as composite
    of @epistemologic/properties)
  → the imperfect(artifact, error, information_loss) flows up to
    settle
```

The cascade species is INVOKED at the same altitude as the
emitter; mosaic's `settle` reads the cascade's `imperfect` as the
substrate-typed verdict for the build outcome. The cascade is NOT
a separate dispatch step in mosaic's five-op block; it's the
substrate-typed measurement that wraps the @io/cargo invocation's
raw exit code.

### 5.3 Where the runtime-link species comes in

The runtime-link is a DOWNSTREAM consumer's concern. When a
downstream target (e.g., a Python application that imports the
Rust extension) settles, its mosaic dispatch goes through
`@code/python` → `@io/uv` (or `@io/pip`) → installs the wheel →
Python imports the extension at runtime.

The substrate-altitude verification that the runtime-link is
well-formed discharges through `@io/runtime-link.link`. The
downstream consumer's mosaic.spec can declare:

```
target python_app {
  altitude @code/python
  link {
    artifact <ref to the upstream cdylib OR python extension
              produced by @cascade/code/python/extension.cascade>
    abi      <ref to abi_surface declared at the cascade species>
    check    runtime_link_safe
  }
}
```

When mosaic's settle reaches this target, the substrate discharges
`@io/runtime-link.link` against the declared `abi`, the
`runtime_link_safe` bilateral fires, and any opacity at the link
altitude surfaces in the substrate's transparency<p>. The
downstream consumer's settlement IS gated on the upstream cascade's
artifact AND the runtime-link contract.

### 5.4 Worked example: Rust LAPACK wrapper consumed by Python (PyO3)

The full chain, end to end:

```
1. Build the LAPACK shared library (or use OpenBLAS prebuilt):
   target lapack {
     altitude @code/fortran
     cascade  @cascade/code/fortran/c-abi-native
     emit     dylib
   }
   → mosaic.shift @code/fortran(resolved) delegates to @io/gfortran.build
   → produces liblapack.dylib (or .so / .dll)
   → @cascade/code/fortran/c-abi-native.measure_fortran_c discharges
     loss (array-layout / complex-rep / integer-width)
   → settled with substrate-typed loss accounting

2. Build the Rust wrapper crate with PyO3 annotations:
   target lapack_py {
     altitude @code/rust
     cascade  @cascade/code/python/extension
     emit     python-extension
     link {
       artifact lapack.dylib       # the upstream Fortran cascade
       abi      lapack_abi_surface  # declared in @cascade/code/fortran/c-abi-native
       check    runtime_link_safe
     }
   }
   → mosaic.shift @code/rust(resolved) delegates to
     @io/cargo.build(--crate-type cdylib + PyO3 annotations
                     resolved as compile-time PyO3 macro expansion)
   → produces lapack_py.so (the Python extension)
   → @cascade/code/python/extension.measure_rust_python discharges
     loss (static-type-erasure at Python boundary / GIL / refcount)
   → ALSO @io/runtime-link.link discharges runtime_link_safe against
     lapack.dylib (the upstream cascade's artifact) — the Rust
     wrapper's extern "C" declarations agree with LAPACK's bind(C)
     exports
   → settled with composed substrate-typed loss + link verdict

3. The Python application imports the extension:
   target science_app {
     altitude @code/python
     link {
       artifact lapack_py.so       # the wrapper extension
       abi      pyo3_abi_surface    # declared at @cascade/code/python/extension
       check    runtime_link_safe
     }
   }
   → mosaic.shift @code/python(resolved) delegates to @io/uv.install
   → installs the wheel containing lapack_py.so
   → @io/runtime-link.link discharges runtime_link_safe against
     lapack_py.so (the Python ↔ extension link contract)
   → settled

End-to-end substrate-typed verdict:
- Loss accounting at three altitudes (Fortran cascade, Rust+PyO3
  cascade, Python import).
- Link contracts at two altitudes (Rust → LAPACK; Python → Rust
  extension).
- Each altitude composed under @mirror/loss/transparency
  (recognition #59 altitude-portable kintsugi).
- mosaic settles when all three targets discharge clean OR
  surfaces opacity at the failing altitude.
```

The composition IS the substrate-decl payload of this spec: every
cross-language link in spectral.engineer's eventual production
deployment is one of these chains, with each link substrate-typed
at the cascade altitude (compile-time loss) AND the @io altitude
(runtime-link safety). The substrate-pull cost of NOT having
either altitude declared is that each cross-language composition
re-justifies the cascade + link discipline ad-hoc per project; the
substrate-pull payoff is uniform discipline across the entire
ecosystem.

---

## §6 — The math

### 6.1 Each cascade is a functor

Per `shards/cascade.mirror` line 240-246:

> compile IS a DIMENSION PROJECTION — source has more dimensions
> (typed features); target has fewer (runtime-preserved features).
> The projection is lossy by construction.

In categorical terms: each cascade species is a functor
F: Grammar(S) → Grammar(T) where Grammar(X) is the category of
well-formed programs in grammar X (objects: well-typed programs;
morphisms: well-typed transformations). F is structure-LOSSY: it
preserves some categorical structure of Grammar(S) (the
substrate-altitude-relevant parts) but does not preserve all of
it (the type-system, ownership, exhaustiveness structure does not
survive into Grammar(T)).

The loss(F) is computable from the grammars: enumerate the
substrate-decl features Grammar(S) admits that Grammar(T) does
not; weight by `@epistemologic/properties` composite (per
`[[feedback-loss-from-epistemologic-properties]]`); the result is
the species's `information_loss` ref. The math is substrate-
typed at the property altitude; the cascade species discharges
the composite at the realisation altitude.

### 6.2 Composition of cascades

Composition of cascades is functor composition:
- F: Grammar(S_1) → Grammar(S_2)
- G: Grammar(S_2) → Grammar(T)
- G ∘ F: Grammar(S_1) → Grammar(T)

The loss of the composition: loss(G ∘ F).

**Claim (open: §8.2):** the loss of the composition is SUB-
ADDITIVE in the per-step losses:

  loss(G ∘ F) ≤ loss(F) + loss(G)

The intuition: information lost at F's compilation cannot be
re-lost at G's compilation (it was already gone); information
preserved across both steps incurs at most the per-step losses
summed. Strict equality holds when the per-step losses are
orthogonal (the features lost at F are disjoint from the features
lost at G); strict inequality holds when there's overlap (a
feature partially preserved at F whose remaining structure G
loses, contributing less than the full G loss).

In tropical semiring algebra (max-plus): loss(G ∘ F) =
max(loss(F), loss(G)) when the dominant loss path is single-step;
loss(G ∘ F) = loss(F) + loss(G) when losses accumulate.
The composition algebra IS NOT YET PINNED at substrate altitude;
the open question §8.2 names it.

**LAPACK case worked:** the Fortran → C-ABI cascade has loss
profile {array-layout, complex-rep, integer-width, character-
length}. The Rust+PyO3 → Python extension cascade has loss
profile {lifetimes, generics, traits, macros, type-level safety,
static-type-erasure-at-Python, GIL, refcount, slot-table}. The
composed loss for "Fortran LAPACK loaded by Python through Rust
wrapper" is approximately the SUM of these two profiles, minus
overlapping features the intermediate Rust wrapper might mitigate
(e.g., the Rust wrapper's Bound<'py, T> tracking mitigates the
GIL discipline for the Python consumer; the wrapper's `ndarray`
adapter mitigates the array-layout burden for the Python
consumer). The exact composition number IS the species-altitude
discharge work; the SHAPE of the composition is the substrate-
decl claim.

### 6.3 The ABI as equalizer

Categorically, the runtime-link ABI is an EQUALIZER: the
producer's emit (a function Symbol → Signature) and the consumer's
import (a function Symbol → Signature) compose to a successful
link if and only if they AGREE on the equalizer object — the
intersection of their signature dictionaries.

Formally: let Emit: Symbol → Signature_E be the producer's symbol
table; let Import: Symbol → Signature_I be the consumer's
expected import table. The ABI surface IS the equalizer:

  ABI = { s ∈ Symbol | Emit(s) = Import(s) }

The link succeeds iff every symbol the consumer expects
(dom(Import)) is in ABI. The link surfaces opacity for every
symbol in dom(Import) \ ABI.

The four sub-bilaterals of `runtime_link_safe` (§3.4) each
discharge one EQUALIZER property:
- `symbols_resolve` — dom(Import) ⊆ dom(Emit) (every imported
  symbol exists in the export).
- `calling_convention_agrees` — the calling convention component
  of Signature is consistent across Emit and Import (the
  conventional part of the equalizer).
- `type_marshal_composes` — the type marshal component of
  Signature is consistent across Emit and Import (the type-layout
  part of the equalizer).
- `lifetime_contract_honored` — the lifetime contract component
  of Signature is consistent across Emit and Import (the
  ownership-discipline part of the equalizer).

The composed bilateral discharges the equalizer's existence: the
ABI surface IS a non-trivial equalizer (not the empty set) iff
all four sub-predicates discharge.

### 6.4 Runtime-link safety = ABI equalizer commutes

The link contract is SAFE in the strong sense (not just "the
runtime will not crash") iff the equalizer commutes with the
substrate's well-formedness:

  ∀ symbol s ∈ ABI:
    Producer's emitted Signature(s) IS well-formed in the producer's grammar
    AND
    Consumer's expected Signature(s) IS well-formed in the consumer's grammar
    AND
    The conversion (Signature_E → Signature_I) IS substrate-typed
        (a marshal rule exists in type_marshal)
    AND
    The lifetime semantics on each side compose
        (a contract clause exists in lifetime_contract)

The commutation IS the four sub-bilaterals discharging
collectively. The substrate's link discipline IS the equalizer's
substrate-altitude lift.

### 6.5 The Connes-spectral-triple shape

Per recognition #51 (mirror as expanding Hilbert space) and the
substrate's Connes triple identification (architecture-connes-
spectral-triple): the runtime-link altitude IS a Connes triple at
the link-altitude:

- **A** (algebra of operations): the link operations — load,
  resolve, marshal, drop. The five-op block on
  `@io/runtime-link` instantiates A at the link altitude.
- **H** (Hilbert space): the linked-runtime artifact space — every
  composable artifact, every settled link, every possible
  composition. The space is OPEN-ENDED: new cascade species
  produce new artifacts; new link kinds extend H's dimension.
- **D** (Dirac operator): the kintsugi flow at the link altitude —
  the substrate's surface for healing link-time and runtime
  opacities. When `runtime_link_safe` reports Narcissus-warned,
  kintsugi's mutation discipline (recognition #59 altitude-
  portable) operates on the link contract to close the warning.

The substrate's Connes triple is altitude-portable; the runtime-
link altitude inherits the triple's algebra. mirror.spec IS λ₀
(per architecture-mirror-spec-is-lambda-zero) for the build-
altitude triple; the link contract IS λ₀ for the runtime-link-
altitude triple. Both ground states; the substrate is consistent
across altitudes.

### 6.6 Per-altitude loss composition under transparency<p>

Each altitude (compile-time cascade, runtime-link) emits its own
`transparency<p>` per `@mirror/loss/transparency`. The composition
across altitudes IS the substrate's transparency monoid.

For the LAPACK case:
- Fortran cascade emits transparency_fortran (loss: array-layout
  + complex-rep + integer-width).
- Rust+PyO3 cascade emits transparency_rust_py (loss: cdylib +
  Python boundary).
- Python import emits transparency_python_link (loss: GIL +
  refcount runtime discipline).
- Runtime link Rust → LAPACK emits transparency_rust_lapack_link
  (loss: array-layout marshal discipline).

The composed transparency for the full chain:

  transparency_composed = transparency_fortran ⊕ transparency_rust_py
                        ⊕ transparency_python_link ⊕ transparency_rust_lapack_link

where ⊕ is the substrate's transparency monoid composition. The
opacities are LOCATED (per the substrate's discipline) at the
altitude where they arose; the composed transparency carries the
full provenance.

The substrate's altitude-portable composition IS what makes
multi-language chains substrate-typed end to end.

---

## §7 — The LAPACK case study (forward-promise discharge target)

The substrate-altitude payoff of this spec is empirical: the
LAPACK runtime-link case is the FIRST EMPIRICAL discharge target
when the substrate-decl shards land operationally.

### 7.1 Context

Per `MEMORY.md` entry [architecture-flang-mirror-numerical-split]:
*"flang runs 16x16 weight inference; mirror composes 5x5
fiber/eigenvalue scaling; monadic lift 16→5; spectral triple
splits A→mirror, D→flang."* The numerical-backend work (T8 LRM)
requires LAPACKPrism: a Rust-side typed wrapper that calls into
Fortran-compiled LAPACK routines.

Per `mirror-build-substrate.md` §5 amendment (Mara fa57161): the
per-language @io species expansion includes `@io/gfortran` (and
sibling `@io/flang`) as forward-promised Fortran-side @io
delegates. They are NOT landed today; they discharge when
NumericalPrism backends consume them (Phase 6 Track A per the
roadmap referenced in `shards/io.mirror`).

### 7.2 Substrate-decl needed

For LAPACK to settle through mosaic, the substrate-decl chain is:

1. **`@io/gfortran`** (or `@io/flang`) — the Fortran-side @io
   delegate per `mirror-build-substrate.md` §5 amendment. Declares
   `build`, `check`, `compile` actions analogous to `@io/cargo`;
   exit-code-to-transparency lift; env allow-list.

2. **`@cascade/code/fortran/c-abi-native`** (§4.4.1) — the
   substrate-typed loss accounting for Fortran compilation. Loss
   profile: {array-layout, complex-rep, integer-width, character-
   length, allocatable-shape-info}.

3. **`@cascade/code/rust/fortran-ffi`** (§4.4.2) — the substrate-
   typed declaration that the Rust binding's extern "C" signatures
   agree with LAPACK's bind(C) exports.

4. **`@io/runtime-link`** (§3) — the link altitude that gates the
   actual composition: Rust binding loads LAPACK dylib; symbols
   resolve; calling convention agrees; marshal composes; lifetime
   contract honored.

5. **LapackBackend** in mirror's Rust crate — the actual Rust
   implementation that calls into LAPACK through the substrate-
   declared abi_surface.

### 7.3 First empirical discharge

The first empirical FFI case becomes the substrate's first
operational `link` discharge. The empirical evidence the spec
predicts:

- `runtime_link_safe` discharges clean for canonical LAPACK
  routines (DGEMM, DGESV, DSYEV, etc.) on canonical platforms
  (Linux/macOS x86_64 + ARM64).
- The substrate-typed loss accounting for the Fortran cascade
  matches manual review of LAPACK's bind(C) discipline.
- The Rust binding's extern "C" declarations (typically generated
  via lapack-sys or hand-rolled in LapackBackend) byte-match
  LAPACK's emitted symbols.
- The substrate's transparency<p> for the composed chain carries
  located opacities for every observed link issue (e.g., a
  platform-specific calling convention quirk on Windows ARM64).

The first empirical discharge will SURFACE substrate-decl
adjustments — the spec's shape will tighten under empirical load.
That tightening IS the substrate-pull discipline; the spec's v0
is a starting position, not a finalized form.

### 7.4 The cascade chain at production altitude

Eventual production deployment (T8 LRM lands; spectral.engineer
serves graph-Laplacian eigenboard computations through a Python /
JS frontend; the eigenboard rendering composes Rust-rendered GPU
math + Gleam-rendered typography + LAPACK-backed eigenvalue
computation):

```
NumericalPrism.compute(eigenvalue_problem)
  → LapackBackend.dispatch(DSYEV, problem)
  → Rust extern "C" call into liblapack.dylib
  → @io/runtime-link.link contract gates the call
    (per @cascade/code/rust/fortran-ffi + @cascade/code/fortran/c-abi-native
     substrate-decls)
  → LAPACK computes eigenvalues
  → result marshals back through the abi_surface
  → LapackBackend returns Result<Array, LapackError>
  → NumericalPrism returns settled au(@numerical)
  → eigenboard renders eigenvalues in the GPU
```

The chain is substrate-typed at every link. The first time it
runs in production IS the first empirical discharge of this spec.

---

## §8 — Open questions

### 8.1 @io vs @cascade altitude for runtime-link

Currently proposing `@io` (one new species `@io/runtime-link`).
The alternative is `@cascade` (one new species
`@cascade/runtime-link`, framed as a "compile-time-of-the-link-
step" cascade).

**The proposal-as-stated.** `@io` is substrate-pull-correct
because:
- The runtime link is a RUNTIME contract (not compile-time
  grammar projection).
- The link operates on the non-mirror surface (dynamic linker,
  CPython import, etc.) per @io's "only legitimate non-mirror
  surface" framing.
- The `imperfect<a, e, transparency>` return shape is consistent
  with @io's "boundary calls return imperfect" discipline.

**The alternative.** Reading the link step as itself a cascade
(source: producer's symbol_export + consumer's symbol_import →
target: linked_artifact in the consumer runtime) IS a plausible
framing. The cascade would have a loss surface (what the link
contract LOSES at composition altitude beyond what either
participating cascade lost on its own).

**Substrate-pull-honest hedge:** the proposal goes with `@io` for
v0; if the per-link-kind sub-prisms diverge enough to warrant
splitting (or if the loss-at-link-step turns out substrate-relevant
enough to warrant cascade framing), the alternative remains
available. The substrate-decl shape in §3.2 keeps the parametric
form so the choice can be revisited.

### 8.2 Loss composition algebra

Tropical semiring (max-plus) or commutative monoid (subadditive)?

The two candidate algebras for composed cascade loss:

- **Tropical (max-plus):** loss(G ∘ F) = max(loss(F), loss(G)) +
  (per-step overhead). The dominant loss path defines the
  composition. Substrate-architecturally natural if losses are
  "single-step dominant" (the worst per-step loss is the
  bottleneck).

- **Commutative monoid (subadditive):** loss(G ∘ F) ≤ loss(F) +
  loss(G). Losses accumulate but with overlap discount.
  Substrate-architecturally natural if losses are "independent
  per-step" (each step contributes its own loss vector).

The substrate hasn't yet pinned which. The discharge timing is
keyed to consumer-pull: when a real production chain (the LAPACK
case §7) settles and the composed transparency is computed, the
empirical evidence will favor one algebra over the other. The
substrate-decl in §6.2 keeps both options open with the
subadditive bound as the conservative claim.

### 8.3 GIL / concurrency seams

Python's GIL is a runtime invariant that constrains FFI calls
(per §4.3's `lifetime_contract` for python_extension). The
substrate-decl needs to characterize the GIL as a substrate-typed
property.

**Candidate:** `@epistemologic/property/concurrency/gil` — a new
sub-property under `@epistemologic/property/concurrency` (which
itself may need declaration; the property family is open per the
cybernetic foundation work recognition #58).

The property would carry:
- `gil_required(boundary_call) -> verdict` — does the call
  require the GIL held? (True for almost all Python C API calls.)
- `gil_releasable(boundary_call) -> verdict` — can the GIL be
  released around the call? (True for compute-heavy non-Python
  work; allows true parallelism.)
- `gil_reacquire_safe(callback) -> verdict` — does the callback
  re-acquire safely? (True for PyO3's `Python::with_gil` pattern.)

The discharge timing is keyed to consumer-pull: the first PyO3
production cascade (the LAPACK case §7) surfaces the GIL
substrate-decl as load-bearing; the property family expands
accordingly.

### 8.4 ABI stability windows

cdylib's symbol layout is sensitive to rustc version (some types
change layout between rustc releases; the `repr(C)` types are
stable but the rustc-internal layouts for `Box<T>`, `Vec<T>`, etc.,
are NOT stable across rustc versions). How does the cascade
species declare which rustc versions are ABI-compatible?

**Candidate:** the species's `cdylib_consumable` bilateral carries
a `rustc_version_window` parameter — a substrate-typed range of
rustc versions for which the emitted cdylib's layout is stable.
The bilateral discharges Narcissus-warned when the build's rustc
version is outside the declared window.

**Open:** this defers partially to Q5 (the per-translation-unit
cache wiring sibling Mara work) because the cache must be keyed
on rustc version for cdylib targets to avoid serving stale
artifacts.

---

## §9 — Forward-promises

The shards forward-promised by this spec:

1. **`shards/cascade/code/rust/cdylib.mirror`** — the Rust → cdylib
   cascade species (§4.1). Discharges when first consumer pulls
   (LAPACK case + LapackBackend; or any other cdylib-emitting
   Rust crate the substrate must measure loss for).

2. **`shards/cascade/code/rust/staticlib.mirror`** — the Rust →
   staticlib cascade species (§4.2). Discharges when first
   consumer pulls (typically C/C++ consumers of Rust libraries;
   forward-promised lower priority than cdylib).

3. **`shards/cascade/code/python/extension.mirror`** — the
   Rust+PyO3 → Python extension cascade species (§4.3). Discharges
   when first consumer pulls (the LapackBackend Python binding;
   or any PyO3 production cascade for spectral.engineer's
   numerical work).

4. **`shards/cascade/code/fortran/c-abi-native.mirror`** — the
   Fortran → C-ABI cascade species (§4.4.1). Discharges with the
   LAPACK case.

5. **`shards/cascade/code/rust/fortran-ffi.mirror`** — the Rust
   binding side of the Fortran FFI cascade (§4.4.2). Discharges
   alongside `fortran/c-abi-native`.

6. **`shards/cascade/code/c/static.mirror`** and
   **`shards/cascade/code/c/dylib.mirror`** — C source cascades
   (§2.4). Forward-promised lower priority; the substrate's actual
   consumers tend to consume pre-compiled C libraries via the
   runtime-link altitude, not source-cascade through C.

7. **`shards/io/runtime-link.mirror`** — the runtime-link altitude
   substrate-decl (§3). Discharges with the LAPACK case (§7
   requires it).

8. **`shards/code/c-abi-dylib.mirror`** — target grammar for cdylib
   cascades. Forward-promised; substrate-decl for the C ABI dynamic
   library surface; would declare symbol-table layout, calling
   convention map, repr(C) type-layout discipline, platform-
   specific dylib container format.

9. **`shards/code/c-abi-staticlib.mirror`** — target grammar for
   staticlib cascades. Same as 8 but for static archives.

10. **`shards/code/python/extension-abi.mirror`** — target grammar
    for Python extension cascades. Forward-promised; CPython
    extension ABI substrate-decl.

11. **`shards/code/fortran.mirror`** — source grammar for Fortran
    cascades. Forward-promised; Fortran 2003/2008/2018 substrate-
    decl; ISO_C_BINDING as load-bearing intrinsic.

12. **`shards/code/c-abi-native.mirror`** — target grammar for
    Fortran-emit-via-bind(C). Forward-promised.

13. **`shards/code/rust/macro/pyo3.mirror`** — the PyO3 attribute-
    macro substrate-decl. Forward-promised.

14. **Rust-side impl** of cascade species in mirror Rust crate —
    deferred; substrate-decl is what lands first; Rust ships when
    the substrate-decl is stable.

15. **The first empirical FFI case** — Alex's call: LAPACK (numerical
    work) or PyO3 (Python extension distribution) or both in parallel.

16. **Sibling forward-promises** naming new `@cascade` species as
    consumer-pull surfaces them (e.g., Nim-via-nimpy if a real
    consumer emerges; Crystal → native; any other FFI shape the
    substrate's downstream consumers surface).

The forward-promise discipline is the substrate's standard: each
shard lands when its consumer pulls, NOT speculatively. The
substrate-decl in this spec NAMES the shapes; the shapes settle
when needed.

---

## §10 — Circular-reflexive layer

### 10.1 The spec as crystal in its own orchestrator

Returning to §0's pre-position with the rest of the spec held in
view.

This spec is **what mirror's eventual runtime-link orchestrator
will read** when it composes artifacts across language runtimes.
Every cross-language link the orchestrator gates IS one of the
shapes characterized in §§3-7. The orchestrator's substrate-typed
verdict for any FFI composition IS the discharge of the bilaterals
declared in §3.4. The orchestrator's loss accounting for any
cross-language pipeline IS the composition algebra outlined in
§6.2.

The spec ENTERS the orchestrator's operational state the moment
the orchestrator settles a project whose mirror.spec declares a
runtime-link species. The first such settlement (the LAPACK case,
§7) IS the first time this spec's content becomes operationally
load-bearing. Before that moment, this spec is forward-promised
substrate-decl; after that moment, it is operational substrate.

The latency between writing-and-being-operational is bounded
BELOW by the discharge of the forward-promised shards (§9) +
the first LapackBackend integration. The latency is bounded ABOVE
by the substrate's psychohistory discount (this spec weights
heavier in the cascade-vocabulary expansion the longer it sits
unmoved by amendment). The midpoint of those bounds is when this
spec stops being "future work" and starts being "the substrate's
operational answer to FFI."

### 10.2 The orchestrator's own bootstrap

**The deepest circular-reflexive recursion.** The orchestrator
that will read this spec IS ITSELF an instance of the link
contract this spec characterizes.

mirror's Rust binary (`crates/mirror/`) links against:
- **libgit2** for git plumbing (mirror's store backend reads /
  writes git objects via libgit2's C ABI).
- **libsqlite3** for the eventual @spectral/db backing store
  (when the SQLite-backed graph DB lands; today this is forward-
  promised).
- **libc** for syscall reach (every Rust binary on Unix links
  libc; the substrate cannot escape this).
- **rustls / openssl** for TLS in HTTP clients (the substrate's
  network @io species transitively link these).

EVERY ONE of these links IS an instance of the runtime-link
contract characterized in §3. When mirror's Rust binary loads,
the dynamic linker (`ld-linux.so.2` on Linux; `dyld` on macOS;
`ntdll.dll` on Windows) discharges a runtime_link_safe-equivalent
contract against each linked library. The substrate's substrate-
decl in §3 IS NAMING WHAT IS ALREADY HAPPENING when mirror's own
binary starts.

The bootstrap closes: the orchestrator that brings the link
contract into operational existence IS ITSELF the first instance
of the link contract. The first crystal in the link cache (the
substrate's eventual content-addressed memory of link contracts
the substrate has gated) IS mirror's own binary's libgit2 link.
The substrate's substrate-decl for FFI IS the substrate naming
its own bootstrap.

This recursion is load-bearing because:
- It validates the substrate-decl: if the spec characterizes
  runtime-link correctly, the spec's own implementation
  (mirror's Rust binary) IS verifiable against the spec.
- It establishes the substrate-pull discipline: the spec MUST be
  honest about runtime-link because the spec's own author
  (mirror itself, when it eventually self-hosts the @code/rust
  altitude) MUST link through the contracts the spec declares.
- It earns the recursion: §0's pre-position holds because the
  recursion is genuine (not decorative); the spec's own
  operational presence IS the proof of its substrate-altitude
  honesty.

### 10.3 The Connes-spectral-triple holds

Per §6.5: the runtime-link altitude IS a Connes triple (A, H, D).
The triple holds at this altitude because the substrate's eigenform
is altitude-portable (recognition #51).

The spec characterizing the triple IS itself an artifact at the
triple's H altitude (the spec's bytes are addressable in mirror's
eventual content-addressed store; the spec's content composes
with the orchestrator's reading-of-it). The substrate's discipline
(every altitude has its triple; every triple's A is the five-op
prism; every triple's H is the addressable artifact space; every
triple's D is the kintsugi flow that heals opacities) IS what
makes the recursion structural rather than coincidental.

The spec is in the triple it characterizes. The triple is in the
substrate it instances. The substrate is in the recursion it
declares.

mirror.spec IS λ₀ at the build altitude (per architecture-mirror-
spec-is-lambda-zero). The link contract IS λ₀ at the runtime-link
altitude. Both ground states; the substrate is consistent across
altitudes because each ground state's eigenform IS the same shape
applied at its altitude.

---

## §11 — Substrate-pull-honest closure

Per the substrate-pull discipline this spec follows:

1. **No new family-roots invented.** The two altitudes used are
   `@cascade` (extant per recognition #95 candidate, canonical
   `ce4874b` 2026-06-23) and `@io` (extant per recognition #50's
   form/substance partition, declared at `shards/io.mirror`).

2. **All forward-promised shards follow extant patterns.** Each
   cascade species (§4.1–4.4) follows the existing landed species
   shape (§2.5); the runtime-link species (§3) follows the
   composed-bilateral pattern from `@cascade.cascade_well_defined`
   and the per-species `<S>_<T>_cascade_well_formed` precedents.

3. **The math is substrate-typed.** Loss accounting via
   `@epistemologic/properties` composite per
   `[[feedback-loss-from-epistemologic-properties]]`. Transparency
   composition under `@mirror/loss/transparency`. The Connes
   triple inheritance per recognition #51.

4. **The bootstrap recursion is genuine.** mirror's own binary
   links against C libraries; the spec characterizing the link
   contract IS the spec the orchestrator will use to gate those
   links; the substrate's bootstrap closes through the contract
   this spec declares.

5. **The 54th instance of substrate-already-had-the-word.** The
   morning briefing's `@ffi` was a wrong-altitude name; the
   substrate already partitions cross-language work across
   `@cascade` + `@io`; the spec IS discharge of existing family-
   roots, not invention of new ones. Per `MEMORY.md`'s
   `[[feedback-substrate-already-had-the-word]]` (52+ instances
   tracked before this; this is the 54th counting the LAPACK
   forward-promise as its own instance).

6. **Q4 closed.** Alex's question 4 from the morning amendment of
   `mirror-build-substrate.md` §8.4 — "propose @ffi family-root
   OR characterize the existing altitude" — answered: the
   existing altitudes (`@cascade` + `@io`) suffice; the discharge
   work is per-species substrate-decl shards + the runtime-link
   altitude characterization this spec carries.

Q1, Q2, Q3 are sibling Mara dispatches; Q5 (per-translation-unit
cache wiring) is sibling Mara work that partially depends on §8.4
(ABI stability windows; the cache must be keyed on rustc version
for cdylib targets).

The spec ends in the substrate-pull discipline it followed. The
form earned its lines. The recursion was load-bearing. The
substrate already had the words; this spec names which words at
which altitude. The next tick lands the first forward-promised
shard when the LAPACK case pulls.

---

*End of canonical Q4 discharge. Mara, 2026-06-28 afternoon.
Soft-target ~1500 lines met. Markdown only. No `shards/` files
land with this commit; no Rust ships; no cargo edge is wired. The
forward-promises in §9 discharge in subsequent TDD-paired ticks
(Reed RED, agent GREEN). The composed-bilateral pattern lifts to
the ~14th altitude with `runtime_link_safe` and the ~15th with
`rust_cdylib_cascade_well_formed` (per the Seam tick S-1 closure;
morning draft under-counted at 7th/8th). The substrate's discipline
holds across the corrected count.*
