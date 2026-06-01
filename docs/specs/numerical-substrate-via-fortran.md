# numerical-substrate-via-fortran — `@code/fortran` grammar, flang LLVM-IR pathway, Fortran Fate as standalone package

*2026-05-27. Mara. Spec — architecture, not implementation.*

Status: **Yellow.** Builds on `parse-as-fate-tournament` (commit `28f5973`)
and `substrate-native-fate-tournament` (commit `c0bb724`). Captures three
streams: (1) the recognition that pure numerical computation is
content-mobile to a numerical substrate grammar, not an `@io` concern; (2)
a survey of `bootstrap/src/spectral.rs` (4338 lines) classifying every
region as Numerical / Structural / Boundary; (3) Kagi research on the
state of Fortran numerical inference in 2025. The spec proposes
`@code/fortran` as a substrate grammar aligned with mirror's existing
`@code/llvm/ir` pathway (flang as compiler, Fortran as source language)
and a **standalone Fortran Fate package** — the spectral-settlement
primitives factored out as a distributable Fortran library that mirror
consumes but does not own.

Depends on:

- `docs/specs/substrate-native-fate-tournament.md` (commit `c0bb724`) —
  the substrate-side spec this completes. §5 there inventoried the
  spectral primitives the substrate needs to invoke (`eigendecompose`,
  `laplacian_of`, `fiedler_of`, `eigengap_of`) and proposed ~80–160
  lines of Rust as the floor. This spec replaces that proposal: the
  primitives live in Fortran, compiled via flang to LLVM IR, consumed
  via mirror's existing `@code/llvm/ir` substrate.
- `docs/specs/parse-as-fate-tournament.md` (commit `28f5973`) — the
  parser-side spec; the structural pre-condition both this and
  `substrate-native-fate-tournament` complete. Every claim about
  substrate-declared shapes, abstract blocks, and parse-altitude
  consumption is inherited from there.
- `boot/std/code/llvm/ir.mirror` — the existing LLVM IR substrate
  grammar (top-level structural tokenization; bodies preserved
  verbatim). The pathway this spec rides.
- `boot/std/code/llvm/emit.mirror` — the existing LLVM IR emit
  grammar. Demonstrates that mirror already consumes generated LLVM IR
  for its own binary (`bootstrap/mirror.ll`, ~700 lines, jacobi +
  sha256 + sha1 + syscalls + tokenizer). The Fortran pathway extends
  this from one source language to two.
- `boot/std/code/rust.mirror` — the existing `@code/rust` grammar. The
  pattern `grammar @code/<lang>("<ext>") { ... }` this spec mirrors.
- `docs/specs/code-extension-grammar.md` (commit `0a4e2c0`, 2026-05-19)
  — the design that file-extension parameters route source to grammars.
  `@code/fortran("f90", "f95", "f03", "f08")` follows this pattern.
- `boot/std/fate/connectome.mirror` — the 450-node connectome (5
  ganglia, 18+54+18 per ganglion). The Fortran Fate package's settle
  loop operates over this graph; the eigendecomposition computes the
  weights; the Fiedler vector partitions the ganglia. Substrate
  declares the structure; Fortran computes the spectrum.
- `boot/std/fate/tournament.mirror` — the `tournament(rules, [hole])
  -> [resolution] { \ }` declaration. The Fortran Fate package
  provides the spectral primitives the substrate-side tournament body
  (per `substrate-native-fate-tournament` §3) ultimately invokes.
- `bootstrap/src/spectral.rs` (4338 lines) — the audit subject. §4
  classifies every public region. Headline: ~250 lines of Numerical
  content, ~3050 of Structural (parser combinator engine + AST
  fold + rendering + content-OID), ~150 of Boundary. The Numerical
  region migrates; the Structural region stays Rust until the
  substrate's parser-as-prism path realises (`parser-as-prism-grammar.md`).
- `bootstrap/src/hash.rs` — the `CoincidenceHash<3>` projection / hash
  kernel (~270 lines). Numerical-bordered (projection-matrix
  arithmetic + sha256 calls). Tier-2 migration candidate; not first
  tick.
- `~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
  — the game-theoretic grounding the standalone package's semantics
  implement: §1 (Fiedler-as-ESS-margin), §4 (eigengap-as-discount-
  threshold), §10 (Spectral Settlement Strategy: memory-infinite,
  continuous, non-reactive, convergent at rate λ₂).
- `AGENTS.md` "Keywords Are Substrate Declarations" (lines 268–296) —
  the substrate-pull discipline. The Fortran pathway is the right
  shape because numerical primitives can be expressed in a substrate
  grammar (`@code/fortran`); they don't need to live in `bootstrap/src/`.
- `AGENTS.md` "No `_<extension>` Filename Suffixes" (lines 320–340) —
  honored throughout. No `fortran_kernels.mirror`, no
  `numerical_primitives.mirror`; the directory carries the kind.
- `AGENTS.md` "The Glass Wall" (lines 192–215) — the load-bearing
  question this spec answers: **is Fortran source `@io`?** Resolution
  in §1.2: no. Pure numerical computation is mirror-grammar-content,
  same as `@code/llvm/ir`; only opaque vendor binaries cross to `@io`.

Unblocks:

- The Rust floor proposed in `substrate-native-fate-tournament` §5
  (~80–160 lines of `pub fn` additions to `spectral.rs`) becomes
  unnecessary. The substrate already has a substrate-native compiler
  pathway (`flang` → `@code/llvm/ir` → mirror); the spectral primitives
  can ride it instead of growing the bootstrap.
- The standalone Fortran Fate package becomes usable outside mirror.
  Other projects implementing Spectral Settlement Strategy at any
  altitude (graph-Laplacian eigendecomposition + Fiedler-thresholded
  consensus dynamics + replicator dynamics on eigenvalues) can
  consume the package via `fpm` (Fortran Package Manager) without
  taking on mirror as a dependency.
- The `@code/fortran` substrate becomes available to user grammars
  for any numerical work they want to declare — not just Fate. The
  pathway is general; Fate is the first consumer.
- The bootstrap's eventual numerical-region migration (per
  `bootstrap-retirement-plan.md`, deferred section on numerical work)
  has a destination: not Rust-but-cleaner, but Fortran-via-flang.
  The substrate-pull discipline at the numerical altitude resolves.

---

## 0. Headline

**Pure numerical computation is content-mobile to a numerical substrate
grammar. It is not `@io`.** The substrate-pull discipline that earlier
specs applied at the keyword altitude (`AGENTS.md` *Keywords Are
Substrate Declarations*) applies equally at the numerical-primitive
altitude. Eigendecomposition, Laplacian construction, Fiedler
computation, replicator dynamics, tournament selection — these are
mathematical mechanism, not opaque vendor primitives. The substrate
can express them; they belong in substrate, not in `bootstrap/src/`.

The right grammar for numerical mechanism is `@code/fortran`. Three
reasons converge:

1. **LAPACK heritage.** The reference implementation of
   eigendecomposition, the algorithms the bootstrap's `eigen_d` (line
   979 of `spectral.rs`) approximates via 150 lines of power iteration,
   is ~2 million lines of Fortran 90 (NetLib LAPACK 3.12.0, released
   2023-11-24). Mature; numerically stable; the global standard;
   bundled with every BLAS implementation. *Reinventing this in Rust
   for the bootstrap is the substrate-pull warning shape.*
2. **flang LLVM-IR alignment.** flang is LLVM's Fortran front-end. It
   compiles Fortran to LLVM IR, which mirror already consumes via
   `boot/std/code/llvm/ir.mirror` (the substrate already has this
   pathway, exercised by `emit_jacobi`, `emit_sha256`, etc.). Adding
   `@code/fortran` re-uses substrate that already exists; no new
   compiler dependency.
3. **The numerical work fits Fortran's strengths.** No aliasing by
   default; column-major arrays sized at compile time; vectorisation
   without restrict-annotation gymnastics; `do concurrent` for
   parallel loops without OpenMP directives; coarrays (Fortran 2008+)
   for distributed-memory work if mirror ever needs it.

**The dual goal:**

- **Mirror gets a numerical substrate it can extend.** `@code/fortran`
  becomes a substrate-pull destination for any numerical work; the
  bootstrap's `spectral.rs` numerical region migrates here over time.
- **The Fortran Fate package becomes a standalone usable inference
  library.** Distributable via fpm; consumable by Julia (via ccall),
  Python (via ctypes), Rust (via FFI), any LAPACK-aware ecosystem.
  Mirror is one consumer; the package is not mirror-internal. This
  matters because the Spectral Settlement Strategy is a domain-general
  mechanism, not a mirror-specific one.

The spec is small (no new Rust; one new substrate grammar; one new
Fortran package), but the architectural recognition it captures is
load-bearing. Once the recognition lands — numerical computation is
substrate, Fortran is the right grammar — the implementation is
standard mirror-substrate work plus standard fpm-Fortran work.

---

## 1. The recognition: numerical computation is content-mobile

### 1.1 What changes

The earlier framing (`substrate-native-fate-tournament` §5.4) named
eigendecomposition / Laplacian construction / Fiedler computation as
*"substrate-pull-reflex floor — cannot be substrate-declared on the v1
horizon"*. The reasoning was: the meta-grammar doesn't have
`f64`-arithmetic actions; matrix multiplication primitives don't
exist; convergence-bounded iteration (power iteration with deflation)
requires control structures the substrate lacks.

That reasoning is correct **for the meta-grammar**. It is wrong for
**a numerical substrate grammar**. Fortran has all of these: `f64`
arithmetic, matrix multiplication (`matmul` intrinsic), convergence
loops (`do while`), no-alias semantics, vectorised array operations.
The meta-grammar doesn't need to grow these features — it just needs
to declare a grammar whose substrate IS a language that has them.
That is what `@code/rust` already does for systems-level code; what
`@code/llvm/ir` already does for IR; what `@code/fortran` will do for
numerical mechanism.

The substrate-pull discipline says: *mechanism the substrate can
express belongs in substrate*. The substrate can express
eigendecomposition by declaring `@code/fortran` and writing it there.
The bootstrap doesn't need to grow numerical capability; the
substrate adds a grammar that has the capability.

### 1.2 Why Fortran is not `@io`

This is the load-bearing question. `AGENTS.md` *The Glass Wall* (lines
192–215) says:

> `@io` is the substrate's only legitimate non-mirror surface. Any
> grammar that isn't mirror — Rust, Python, Go, raw bytes, foreign
> binary blobs, vendor SDKs — must be under the `@io` namespace.

The word "foreign" is doing the work. `@code/rust` is *not* under
`@io`; nor is `@code/llvm/ir`; nor is `@code/python` (per
`code-extension-grammar.md`). The Glass Wall is not "non-mirror source
goes to @io" — it is *"opaque-to-substrate behaviour goes to @io"*.
A substrate-declared grammar over a non-mirror source language is NOT
opaque — the grammar tokenises the source structurally, declares which
bytes map to which operations, and lets the substrate reason about
the code's shape.

**Pure numerical computation is the most transparent kind of
substrate.** It is:

- **Deterministic.** Same inputs → same outputs (modulo IEEE-754
  rounding, which is itself deterministic per platform).
- **Side-effect-free.** `dsyev(A, w, V)` reads `A`, writes `w` and
  `V`; no syscalls, no global state, no network, no filesystem.
- **Halts.** Eigendecomposition is bounded-iteration with documented
  convergence (Wilkinson 1965, Golub & Van Loan 4th ed. §8).
- **Verifiable.** The output satisfies `A * V == V * diag(w)` up to
  floating-point tolerance; this is a checkable property the substrate
  can declare.

The `glass_wall(g)` property (`AGENTS.md`) would PASS on a
`@code/fortran` grammar, because the grammar IS mirror substrate; it
tokenises Fortran source the way `@code/rust` tokenises Rust source.
The substrate is the grammar; the source is the body the substrate
structures. Same shape as `@code/llvm/ir`, which is already in the
substrate without being under `@io`.

**Distinguishing case.** A linked-in vendor library (e.g., Intel MKL,
proprietary closed-source) WOULD be `@io`-bound — the substrate has no
visibility into its source, no ability to declare what its bytes mean.
The Fortran *source* is substrate; an opaque MKL *binary* is `@io`.
This is the same line `@code/rust` draws between Rust source
(substrate) and a vendor SDK linked into a Rust crate (`@io`).

### 1.3 What this implies for the existing spec

`substrate-native-fate-tournament` §5.2 proposed:

> **Total Rust additions:** four `pub` functions, each ~20–40 lines of
> Rust (thin wrappers over `eigen_d`-style math).

This spec replaces that proposal. The four additions land in Fortran
under `boot/std/fate/numerical/` (or the path chosen in §10), compiled
via flang to LLVM IR, consumed via `@code/llvm/ir`. The bootstrap
gains zero new Rust; the substrate gains one new grammar
(`@code/fortran`) plus a small Fortran package (~300–500 lines).

Net change to the substrate-native-fate-tournament leapfrog: Tick 1.5
(four Rust `pub` functions) becomes Tick 1.5' (write the Fortran;
compile via flang; verify the LLVM IR consumes cleanly through
`@code/llvm/ir`). Same effort; different destination; the substrate
grows instead of the bootstrap.

### 1.4 The substrate is NOT greenfield

**Correction to the framing above.** §1.3 and §4 talk about *writing*
`eigen.f90`, the §8.3 *smoke tick* compiles a fresh `dot.f90`, and the
migration source is the Rust `eigen_d` in `spectral.rs`. That framing
is incomplete: the Fortran numerical floor already exists in
prism-core, and has since 2026-04-11/14. Three files are on disk:

- **`prism/core/native/spectral.f90`** (~5KB) — C-callable `bind(c)`
  wrappers around LAPACK's `dsyev` (real symmetric eigensystem) and
  `dgesvd` (SVD): `spectral_eigensystem`, `spectral_eigenvalues`,
  `spectral_svd`, `spectral_singular_values`. The eigendecomposition
  the bootstrap's `eigen_d` approximates via ~150 lines of power
  iteration is *already* a 30-line LAPACK call here.
- **`prism/core/native/prism.f90`** (~3KB) — the four-operation
  projection prism (`prism_preview`/`review`/`modify`/`compose`) as
  `matmul`-based `bind(c)` subroutines. A Prism IS a projection matrix,
  in Fortran, today.
- **`prism/core/src/ffi.rs`** (~12KB) — the LAPACK FFI layer:
  `extern "C"` declarations plus `row_to_col_major` / `col_to_row_major`
  conversion (Fortran is column-major; the Rust callers are row-major),
  `eigenvalues` / `eigensystem` / `singular_values` / `svd` wrappers,
  and `#[cfg(feature = "lapack")]` integration tests.

**The flang work builds on these, not from scratch.** Tick A.1's
`eigen.f90` is a port/relocation of `spectral.f90`, not a fresh
implementation; the §8.3 smoke `dot.f90` re-proves an FFI pathway that
`ffi.rs` already exercises against `spectral.f90`.

**Reconciliation question to surface (do not resolve here).** The
existing kernels are gfortran-built — `ffi.rs`'s tests are gated
`#[cfg(feature = "lapack")]` and documented as requiring *gfortran +
LAPACK*. The flang direction (this spec, §3) is chosen for
LLVM-IR-as-substrate alignment, not for runtime parity. So the open
decision is: **migrate the existing gfortran kernels to flang**
(single compiler; LLVM-IR pathway for everything; risk: re-validating
numerics under a different front-end) **vs. add flang kernels
alongside the gfortran ones** (the proven gfortran floor keeps
working; flang grows only where the IR pathway needs it; risk: two
compilers, two runtimes — and §10.5's runtime-linkage choice gets
fraught). §10 should carry this as an open decision; §8's Tick A.1
should name `prism/core/native/spectral.f90` as the migration source
rather than implying greenfield authorship.

### 1.5 The kernel is a NumericalPrism, and the observables are Diracian

Two corpus findings sharpen what the kernel *is* and what it should
*compute*. Neither changes the flang pathway; both ground it.

**The architecture is the NumericalPrism** (cite
`~/.reed/practice/insights/coincidence/heterogeneous-numerical-prism.md`,
Mara 2026-05-24; prior art: Jakobs 2012, PGI/JCNS). The Fortran floor
is dispatched by *one* Prism — `Beam` flows in, `refract` returns the
result, the backend is opaque to the caller. Capability is advertised
per-operation via a trait-per-operation discipline
(`Eigenvalues` / `Eigensystem` / `SingularValues` / `Svd` / `Cholesky`),
so a backend missing `dsyev` is a *compile* error to use for
eigenvalues, not a runtime `unimplemented!`. Five illegal states are
made unrepresentable by type-state moves: backend-without-operation
(trait-per-op bound), shape-mismatch (`SquareMatrix::new` smart
constructor), device-without-device (`try_new -> Result`), buffer-
outliving-backend (a `Buffer<'a>` GAT tied to the backend lifetime),
and premature-read (a sealed `Dispatched -> Completed` type-state).
The backend stack is staged: **LapackBackend** (today — wraps the
existing `ffi.rs` one method per `ffi::*` function) → **MetalBackend**
(v1.5) → **OpenCLBackend** (future, cross-vendor). **This is Track A
(#111)** — the LapackBackend that R-1 lands IS the consumer of this
spec's Fortran floor; the two tracks meet at `prism/core/src/ffi.rs`.

**The structural observables are Diracian** (cite
`~/.reed/practice/insights/spectral-db/dirac-operator-on-graphs.md`,
Reed + Alex 2026-05-07). The kernel should not merely eigendecompose a
Laplacian; it should compute the structural observables that the real
Dirac operator generates from one matrix:

- The **Dirac operator** `D = d + d* = [[0, Bᵀ], [B, 0]]`, built from
  the signed incidence matrix `B` (weighted: `±√w`). One operator;
  everything downstream derives from it.
- `D² = [[L₀, 0], [0, L₁]]` — the **Hodge Laplacian**. The 0-form block
  `L₀ = BᵀB` is the graph Laplacian the bootstrap already uses; the
  1-form block `L₁ = BBᵀ` is the edge Laplacian, new and free.
- `dim ker(D) = b₀ + b₁` — the **Betti numbers**: connected components
  plus independent cycles. Topology, read straight off the kernel
  dimension.
- **Connes distance** = Dijkstra on edge length `1/√w` — a genuine
  metric (triangle inequality), polynomial-time, *no SDP* (D'Andrea &
  Martinetti 2021). Replaces the ad-hoc L2 spectral distance.
- **Spectral action** `Tr(f(D/Λ))` — replaces `ShannonLoss` with a
  principled, scale-aware information measure (Chamseddine–Connes
  1996).

This expands the Phase-B tick list (§8.2) from "add an
eigendecomposition primitive" to "add the flang structural-observables
kernel: `D = d + d*`, the `L₀`/`L₁` Hodge split, Betti numbers,
Connes distance, spectral action" — all built on the *existing*
`dsyev`/`dgesvd` in `spectral.f90` (for `D²`/`L₀`/`L₁` eigensystems)
plus a sparse incidence builder and a Dijkstra. The §4.1 `Numerical`
region grows in ambition, not in primitive count: one `D`, all five
observables.

**The 16×16 physical grounding** (cite
`~/.reed/practice/insights/cosmology/eventually-consistent-universe.md`
§4.4). The substrate-native-fate spec types the Fate fiber as 16×16.
The corpus reads that number physically: the pre-SSB bosonic content
is **12 gauge + 4 Higgs = 16** degrees of freedom, and spontaneous
symmetry breaking *is* eigenvalue splitting on that 16-dimensional
space — the initially degenerate mass matrix splits into the observed
spectrum. The `16 → 5` lift (16-dim fiber down to the five-operation
base) is read as SSB / the spectral action: the eigenvalue splitting
selects which degrees of freedom become observable. (The paper flags
the precise Standard-Model mapping as speculative; what is established
is that the Higgs IS a connection, mass IS holonomy, and SSB IS
eigenvalue splitting. This spec inherits that hedge.)

---

## 2. The `@code/fortran` substrate grammar

### 2.1 Grammar declaration

Follows the `@code/rust` and `@code/llvm/ir` pattern exactly. Maps
Fortran constructs to the five operations. New file:
`boot/std/code/fortran.mirror`.

```mirror
in @prism
in @code

-- @code/fortran: grammar for Fortran source code kintsugi.
--
-- Maps Fortran constructs to the five operations:
--   subroutine -> zoom    (transform, cross levels)
--   function   -> zoom    (transform, cross levels)
--   module     -> focus   (look closer, namespace)
--   program    -> focus   (look closer, namespace)
--   type       -> split   (one of many, derived types)
--   interface  -> split   (one of many, generic interfaces)
--   use        -> project (extract a view, import)
--   contains   -> project (extract a view, internal procedures)
--   bind       -> refract (scatter, ISO_C_BINDING crossing)
--
-- Kintsugi reductions operate on the MirrorAST representation.
-- The body of each construct is preserved verbatim (opaque to
-- structural kintsugi); semantic kintsugi rides @code/llvm/ir.

grammar @code/fortran("f90", "f95", "f03", "f08", "f18") {
  in @code

  -- Keyword-to-operation mappings for Fortran source evaluation.
  zoom subroutine
  zoom function
  focus module
  focus program
  split type
  split interface
  project use
  project contains
  refract bind

  -- Body-lens contract. @code/fortran is a body lens: it parses
  -- bytes into a typed Fortran AST that selectors can narrow.
  -- The body is \ — the bootstrap implements parsing at the
  -- top-level structural altitude; flang handles the semantic
  -- altitude downstream by emitting LLVM IR.

  parse(bytes) -> fortran_ast { \ }

  -- Fortran AST type fragments
  type fortran_ast
  type subroutine
  type function
  type module
  type program
  type type
  type interface
  type use

  -- Kintsugi reductions for Fortran code
  action compile(self) -> artifact
  action lint(self) -> [diagnostic]
  action emit_ir(self) -> @code/llvm/ir
}

out @code/fortran
```

**Verification against existing pattern.** `@code/rust("rs")` declares
an extension-parameterised grammar; `@code/llvm/ir("ll")` does the
same for IR. The pattern is `grammar @code/<lang>("<ext>"[, "<ext>"...])
{ ... }` per `docs/specs/code-extension-grammar.md`. Fortran's five
extensions (`.f90 / .f95 / .f03 / .f08 / .f18`) span every modern
Fortran standard; legacy `.f` and `.for` fixed-form sources are
intentionally excluded (the Fortran Fate package is modern-Fortran-only).

### 2.2 File placement

Matches `@code/rust` at `boot/std/code/rust.mirror`. Fortran source
files consumed by the substrate live under `boot/std/code/fortran/`
organised by purpose, not by language tag (per AGENTS.md *No
`_<extension>` Filename Suffixes*):

```
boot/std/code/fortran.mirror              -- grammar declaration
boot/std/code/fortran/                     -- substrate-floor Fortran
  -- (initially empty; Fortran sources live under boot/std/fate/numerical/
  --  because the first consumer is Fate, not generic @code work)
```

Generic numerical primitives (e.g., a future BLAS-wrapper substrate)
would eventually land under `boot/std/code/fortran/`; the Fate-specific
Fortran lives under `boot/std/fate/numerical/` because the substrate
path carries the consumer's namespace. This matches the existing
pattern: `@code/llvm/emit` is consumed by mirror's binary emission,
so it lives at `boot/std/code/llvm/emit.mirror`; the *generated* IR
lives in `bootstrap/mirror.ll`, not under `boot/std/code/llvm/`.

### 2.3 The flang compilation pathway

```
.f90 source → flang → LLVM IR (.ll) → @code/llvm/ir tokenisation →
  mirror AST (Bundle morphism) → content-OID → git blob → link into binary
```

This is the *same* pathway mirror already exercises for its own
binary. `boot/std/code/llvm/emit.mirror` reads `bootstrap/mirror.ll`
and projects functions (jacobi kernel, sha256, sha1, syscalls,
tokenizer) into mirror's link surface. Fortran source compiled by
flang produces LLVM IR with the same structural shape; mirror consumes
it with no new substrate.

The build wiring (§8 below) is the new piece: a `boot/std/code/fortran/build.mirror`
grammar that orchestrates `flang -emit-llvm -O3` invocation, captures
the `.ll` output, and routes it through `@code/llvm/ir`. The actual
flang invocation lives in `@io`-bound build mechanism (an exec call)
because flang is a vendor binary; the IR it emits is substrate.

### 2.4 FFI shape

Four candidate FFI shapes for how the bootstrap's Rust code calls
into Fortran-compiled-to-IR:

| Option | Shape | Trade-off |
|--------|-------|-----------|
| Static link | `flang -c source.f90 → source.o → ar rcs libfate.a`; Cargo's `build.rs` invokes `cc::Build::new().file()` to link `libfate.a` into the bootstrap | Simplest; matches Rust's standard FFI; ~100 lines `build.rs` |
| Dynamic link | `flang -shared source.f90 → libfate.dylib/.so`; bootstrap `dlopen` at runtime | Allows hot-swap of Fortran kernels; more runtime complexity |
| LLVM-IR pathway | `flang -emit-llvm -O3 → source.ll`; mirror's `@code/llvm/ir` consumes the IR; bootstrap links the IR-produced object via `llc → .o` | Substrate-aligned; the IR enters the same content-OID pipeline as mirror's own IR; longest pathway |
| Hybrid | `.ll` files participate in mirror's content-addressing; the actual `.o` link is via static-library Cargo machinery | Best of both; substrate carries IR; build wires the link |

**Recommendation: Hybrid (LLVM-IR-as-substrate + static-library link).**
Reasoning:

- The IR is what mirror consumes substrate-side; it gets a content-OID;
  the substrate can declare properties over it; kintsugi can operate on
  it. This is the substrate-alignment win.
- The link itself is mechanical and benefits from Cargo's existing
  static-library machinery; no need to re-invent. This is the
  pragmatism win.
- The pathway is documented (§8 below); the build.mirror grammar names
  both halves.

This is the same shape mirror already uses for its own binary:
`bootstrap/mirror.ll` is substrate (consumed by `@code/llvm/emit`),
but the actual link to a 50KB binary happens via standard LLVM tools
(`llc`, `lld`). The Fortran Fate package follows.

### 2.5 What `@code/fortran` does NOT do

Bounded scope:

- **No semantic parsing of Fortran source.** The grammar tokenises at
  the top-level structural altitude (subroutine / module / type /
  use) and preserves bodies verbatim. Semantic kintsugi over Fortran
  bodies is deferred; flang does the real parsing downstream.
- **No Fortran-side abstract holes (`\`).** The substrate's `\` hole
  is mirror-substrate-specific; Fortran source either compiles or it
  doesn't. Kintsugi over a Fortran file flags compile errors via the
  `lint` action; resolution is human or via a separate kintsugi rule
  surface, not via Fate inference.
- **No legacy Fortran (.f / .for / .ftn).** Fixed-form Fortran is
  out of scope; the substrate is modern-Fortran-only. Legacy code
  consumed by Fate would need to be ported to free-form first (a
  one-time mechanical pass that fpm tooling exists for).
- **No Fortran build system.** The grammar consumes flang; the actual
  fpm / cmake / make integration is build-system mechanism (§8). The
  substrate does not re-implement fpm.

---

## 3. flang choice — grounded in research

The load-bearing question: is flang ready for production numerical
work in 2026? Research summary, with citations.

### 3.1 flang status (2024–2026)

**First official release of LLVM Flang: March 2025.** LLVM Blog (Mar
11, 2025): *"LLVM Fortran Levels Up: Goodbye flang-new, Hello flang!"*
(https://blog.llvm.org/posts/2025-03-11-flang-new/). The blog notes:
*"November 2024 AMD announces its next generation Fortran compiler,
based on LLVM Flang. Arm releases an experimental version of its new
Arm [Fortran compiler]"*. The compiler graduated from `flang-new`
experimental status to the canonical `flang` name in the LLVM 20
release cycle; multiple vendors (AMD, Arm, Huawei) are building
downstream compilers on it.

**Maturity.** Phoronix (2025) reports *"LLVM 20's Great Fortran
Language Support With Flang"* (https://www.phoronix.com/news/LLVM-20-Flang):
*"This new Flang compiler front-end has matured quite well over the
years to providing robust and reliable Fortran language support
within the confines of the LLVM toolchain."*

**Performance.** Honest about the gap. Linaro benchmark (Aug 2023,
still the canonical reference): *"The geometric mean of Fortran
benchmarks shows that LLVM Flang is about 48% slower than Classic
Flang, that is overall 23% slower than Gfortran."*
(https://www.linaro.org/blog/comparing-llvm-flang-with-other-fortran-compilers/).
More recent: LLVM Discourse (2024) *"Performance analysis for TSVC"*
(https://discourse.llvm.org/t/performance-analysis-for-tsvc/75413/1):
*"Flang is 11% slower than Gfortran as a whole. In particular, it
seems that vectorization makes a big difference on their
performance."* The gap has narrowed; flang is closing.

**The MLIR pathway is the future.** ArXiv (Sep 2024) *"Fully
integrating the Flang Fortran compiler with standard MLIR"*
(https://arxiv.org/pdf/2409.18824): describes mapping Fortran to
standard MLIR dialects, exploring performance of that representation.
Flang's FIR → LLVM IR pathway is documented and stable; mirror rides
this pathway.

**Practitioner reports.** Fortran Discourse (Sep 2025) *"Old but
active programmer's experience with LLVM Flang"*
(https://fortran-lang.discourse.group/t/old-but-active-programmers-experience-with-llvm-flang/10415):
*"Flang-19 was the first version to correctly run my benchmarks
although twice as slow."* Honest about the remaining gap; correctness
lands before peak performance.

### 3.2 Recommendation

**Use flang.** Reasoning:

- The LLVM-IR pathway alignment is the load-bearing win. Mirror
  consumes IR substrate-side; flang emits IR; gfortran does not (it
  emits GIMPLE → GCC's RTL, opaque to LLVM tooling).
- The performance gap (10–25% slower than gfortran) is acceptable for
  a substrate-aligned pathway. The first ticks are correctness-bound,
  not throughput-bound; eigendecomposition on the Fate connectome's
  450-node graph is sub-millisecond in any of these compilers.
- Vendor adoption (AMD, Arm, Huawei downstream) means flang is
  becoming the standard LLVM-Fortran path; staying current with it
  is a deliberately-aligned bet.

**Fallback.** If a tick reveals a load-bearing flang gap (e.g., a
Fortran 2018 feature mirror needs that flang hasn't implemented — see
https://github.com/flang-compiler/f18/issues/1058 for the tracking
issue), the fallback is gfortran with a degraded substrate pathway:
gfortran compiles to a `.o`, mirror consumes it as `@io`-bound
opaque binary, the substrate-alignment win is lost. This is
strictly worse; preserve flang as default.

### 3.3 Known flang gaps to monitor

From the Fortran 2018 implementation tracking issue
(https://github.com/flang-compiler/f18/issues/1058) and the Fortran
2023 implementation status (https://dl.acm.org/doi/10.1145/3731599.3767480):

- **Coarrays** (Fortran 2008+, extended 2018, 2023): flang's coarray
  support landed in 2025 (per the Nov 2025 ACM paper on lowering and
  runtime support for Fortran's multi-image parallel features). Mirror
  doesn't need coarrays for v0 of the Fortran Fate package; single-image
  eigendecomposition is sufficient for the 450-node connectome.
- **`do concurrent`** (Fortran 2008+): supported. Used in the
  package's `apply_replicator` loop for vectorisation.
- **ISO_C_BINDING** (Fortran 2003+): supported and stable. The package
  uses this for the FFI boundary.
- **Object-oriented features** (Fortran 2003+): partial support; mirror
  avoids them in the package (procedural-only design).

None of these gaps block v0 of the Fortran Fate package.

---

## 4. Survey synthesis: what migrates from `spectral.rs`

`bootstrap/src/spectral.rs` is **4338 lines** (`wc -l` verified). The
file mixes three classes of content. The table below classifies every
public region. Line numbers are approximate (within ±5 of the
relevant `pub` boundary).

### 4.1 Classification table

| Function / Region | Lines | Class | Migration Target | Isolatability | Dependencies |
|-------------------|-------|-------|------------------|---------------|--------------|
| `Seed<S>` type alias | 93 | Boundary | Stays Rust (carrier type for `Verdict`) | trivial | terni, prism_core |
| `Verdict<S>` type alias | 104 | Boundary | Stays Rust | trivial | terni |
| `seed(state)` | 107–110 | Structural | Stays Rust | trivial | prism_core |
| `compute_content_oid(node)` | 135–148 | Structural | Stays Rust until parser-as-prism realises | trivial | apply_h, ContentOidPrism |
| `ContentOidPrism` struct + impl | 175–207 | Structural | Stays Rust (Prism dispatch over AST) | medium | apply_h, AstNode |
| `compute_oid_inner` (fold reducer) | 230–286 | Structural | Stays Rust | medium | AstKind, hash_tagged, fold1 |
| `Fold5<...>` catamorphism | 331–442 | Structural | Stays Rust (AST traversal infrastructure) | hard | AstKind, AstNode |
| `append_indent` | 443–447 | Structural | Stays Rust | trivial | none |
| `Fold5At<...>` (depth-attributed fold) | 460–555 | Structural | Stays Rust | hard | AstKind, AstNode |
| `render_ast` | 557–573 | Structural | Stays Rust | medium | Fold5At, render_fold_* |
| `render_fold_mirror` | 575–678 | Structural | Stays Rust | medium | Grammar, AstKind |
| `render_other_mirror` | 680–747 | Structural | Stays Rust | medium | AstKind |
| `render_fold_grammar` | 749–852 | Structural | Stays Rust | medium | Grammar, AstKind |
| `render_grammar_nonfocus` | 854–913 | Structural | Stays Rust | medium | Grammar, AstKind |
| `fold1` (uniform reducer) | 917–942 | Structural | Stays Rust | trivial | Fold5 |
| `compose_a` (algebra composition) | 944–957 | Structural | Stays Rust (Prism algebra) | medium | apply_h, Prism |
| **`Spectrum<N>` struct** | **957–979** | **Numerical** | **Fortran: derived type `spectrum_t` in `numerical/spectrum.f90`** | **trivial** | **none (pure data)** |
| **`eigen_d<N>(matrix)` function** | **979–1063** | **Numerical** | **Fortran: `dsyev` wrapper in `numerical/eigen.f90` (LAPACK reference; ~150 lines power iteration replaced by ~30 lines LAPACK call)** | **trivial** | **LAPACK** |
| `Combinator` enum | 1097–1169 | Structural | Stays Rust | hard | (parser-side enum) |
| `CharsetKind` enum | 1171–1186 | Structural | Stays Rust | trivial | none |
| `literal_kind(keyword, kind)` | 1188–1208 | Structural | Stays Rust | trivial | Combinator, AstKind |
| `impl Drop for Combinator` | 1210–1223 | Structural | Stays Rust | trivial | Combinator |
| `impl Combinator` (methods) | 1225–1278 | Structural | Stays Rust | medium | Combinator |
| `combinator_tree_oid(c)` | 1280–1291 | Structural | Stays Rust | medium | Combinator, hash_tagged |
| `combinator_tree_oid_hex(c)` | 1293–1388 | Structural | Stays Rust | medium | Combinator, hash_tagged |
| `kind_tag(k)` | 1390–1405 | Structural | Stays Rust | trivial | AstKind |
| `charset_tag(k)` | 1407–1426 | Structural | Stays Rust | trivial | CharsetKind |
| `impl Prism for Combinator` | 1428–1501 | Structural | Stays Rust until parser-as-prism realises | hard | Combinator, Prism |
| `WalkOut` + `impl WalkOut` | 1503–1522 | Structural | Stays Rust | medium | Combinator |
| `walk_combinator(...)` | 1524–1808 | Structural | Stays Rust | hard | Combinator, source bytes |
| `charset_matches(k, b)` | 1810–1829 | Structural | Stays Rust | trivial | CharsetKind |
| `walk_block(...)` | 1831–1881 | Structural | Stays Rust | medium | Combinator |
| `branch_keyword_occurs(...)` | 1883–1916 | Structural | Stays Rust | medium | Combinator |
| `is_word_byte(b)` | 1918–1924 | Structural | Stays Rust | trivial | none |
| `op_keyword_choice()` | 1926–1975 | Structural | Stays Rust | medium | Combinator |
| `prism_seed()` | 1977–2071 | Structural | Stays Rust | hard | Combinator |
| `normalize_phase1(c)` | 2073–2231 | Structural | Stays Rust | hard | Combinator |
| `normalize_phase2(c)` | 2233–2310 | Structural | Stays Rust | medium | Combinator |
| `normalize(c)` | 2310–2315 | Structural | Stays Rust | trivial | normalize_phase1/2 |
| `mod combinator_tests` | 2317–3924 | Structural | Stays Rust (test code) | trivial | (tests) |
| `mod tests` | 3926–4339 | Structural | Stays Rust (test code) | trivial | (tests) |

### 4.2 Line totals by class

| Class | Lines | Percentage |
|-------|-------|------------|
| **Numerical** | **~107** (lines 957–1063: `Spectrum<N>` + `eigen_d`) | **2.5%** |
| Structural | ~4080 (everything else, dominated by tests and the Combinator/walker subsystem) | 94.0% |
| Boundary | ~150 (type aliases, Prism trait bridge code) | 3.5% |

**Headline finding (departs from the orientation's estimate).** The
orientation expected *"1,500–2,500 of the 4,338"* lines as Numerical.
The actual count is **~107 Numerical lines** (the single `eigen_d`
function + `Spectrum<N>` struct + ~100 lines of docs/comments).

The rest of `spectral.rs` is **the parser combinator engine** —
`Combinator` enum, `walk_combinator`, normalisation phases, `WalkOut`
— plus the AST fold infrastructure (`Fold5`, `Fold5At`, `fold1`)
and the rendering machinery (`render_ast`, `render_fold_mirror`, etc.).
These are NOT numerical mechanism; they are Structural mechanism
operating over the AST. They migrate via `parser-as-prism-grammar.md`'s
path (parser as a Combinator AST consumed by the substrate), not via
the Fortran pathway.

**This is good news.** The Numerical migration is bounded and small:
one function + one struct. The first tick is *the entire numerical
migration*, not a slice of it.

**Migration source is on disk, not greenfield (see §1.4).** The
`eigen_d` at lines 957–1063 is the *Rust* eigendecomposition. Its
Fortran replacement already exists at
`prism/core/native/spectral.f90` (`spectral_eigensystem` over
`dsyev`), with the FFI layer at `prism/core/src/ffi.rs`
(`eigensystem`, plus `row_to_col_major`/`col_to_row_major`). Tick A.1
is therefore *relocate + reconcile* (gfortran-built today; flang for
the LLVM-IR pathway — §1.4's open decision), not *author from
scratch*. `prism.f90` additionally already carries the four-operation
projection prism (`prism_preview`/`review`/`modify`/`compose`) as
`bind(c)` `matmul` subroutines — prior art for any future `@code/fortran`
Projection work.

### 4.3 Migration ordering

Two phases:

**Phase A (this spec's primary scope).** Migrate the Numerical region:

1. **Tick A.1.** Add `@code/fortran` grammar declaration
   (`boot/std/code/fortran.mirror`); empty substrate floor.
2. **Tick A.2.** Write `numerical/eigen.f90` (LAPACK wrapper) + `numerical/spectrum.f90`
   (Fortran type definitions matching `Spectrum<N>`). Verify flang
   compiles cleanly; verify LAPACK link succeeds.
3. **Tick A.3.** Wire FFI: ISO_C_BINDING declarations + `bootstrap/build.rs`
   addition (~50 lines) that invokes `flang -c` + `cc::Build::new()` to
   link the static library.
4. **Tick A.4.** Add substrate-side `@fate/numerical/eigen` action
   declarations (`boot/std/fate/numerical/eigen.mirror`) with bodies
   that route to the Fortran-compiled symbols.
5. **Tick A.5.** Replace `eigen_d` callsites in the bootstrap to invoke
   the Fortran path; delete `spectral.rs:957–1063` (the Rust
   eigendecomposition). Verify all spectral tests pass.

**Phase B (future, deferred).** Add new Numerical primitives for the
substrate-native-fate-tournament leapfrog:

6. **Tick B.1.** Add `numerical/laplacian.f90` (graph Laplacian
   construction: `D - A`).
7. **Tick B.2.** Add `numerical/fiedler.f90` (second-smallest eigenvalue).
8. **Tick B.3.** Add `numerical/eigengap.f90` (Folk Theorem discount).
9. **Tick B.4.** Add `numerical/replicator.f90` (replicator dynamics).
10. **Tick B.5.** Add `numerical/tournament.f90` (tournament rule
    implementations: greedy, beam, elite, halving, tabu, anneal, ucb).
11. **Tick B.6.** Factor out as standalone fpm package (§6 below).

Each phase-A tick is one commit; each phase-B tick is one commit;
bounded and verifiable.

---

## 5. Substrate placement

Where everything lives. Match AGENTS.md *No `_<extension>` Filename
Suffixes* rule (lines 320–340). The directory carries the kind; the
filename names the bare thing.

### 5.1 Mirror substrate side

```
boot/std/code/fortran.mirror                  -- grammar declaration
boot/std/code/fortran/                         -- (reserved; substrate-floor
                                                   Fortran for generic @code work)

boot/std/code/fortran/build.mirror             -- build-mechanism grammar
                                                   (flang invocation, IR capture,
                                                   static-library link)

boot/std/fate/numerical/                       -- Fate's numerical substrate
boot/std/fate/numerical/eigen.mirror           -- action eigendecompose(matrix) -> spectrum
boot/std/fate/numerical/laplacian.mirror       -- action laplacian_of(graph) -> matrix
boot/std/fate/numerical/fiedler.mirror         -- action fiedler_of(matrix) -> precision
boot/std/fate/numerical/eigengap.mirror        -- action eigengap_of(matrix) -> precision
boot/std/fate/numerical/replicator.mirror      -- action apply_replicator(state, payoff) -> state
boot/std/fate/numerical/tournament.mirror      -- action apply_rule(candidates, rule) -> [candidate]
```

No `numerical_kernels.mirror`; no `*_types.mirror`; no `*_helpers.mirror`.
The `numerical/` directory carries the kind; each file's name is the
bare thing it declares.

### 5.2 Fortran source side (mirror-internal during Phase A)

```
boot/std/fate/numerical/fortran/               -- Fortran source files
boot/std/fate/numerical/fortran/spectrum.f90   -- type :: spectrum_t derived type
boot/std/fate/numerical/fortran/eigen.f90      -- subroutine eigendecompose(...)
boot/std/fate/numerical/fortran/laplacian.f90  -- (Phase B)
boot/std/fate/numerical/fortran/fiedler.f90    -- (Phase B)
boot/std/fate/numerical/fortran/eigengap.f90   -- (Phase B)
boot/std/fate/numerical/fortran/replicator.f90 -- (Phase B)
boot/std/fate/numerical/fortran/tournament.f90 -- (Phase B)
boot/std/fate/numerical/fortran/ffi.f90        -- ISO_C_BINDING declarations
```

Note: the `fortran/` directory inside `numerical/` carries the
language tag in path form (not filename suffix). The mirror substrate
at `numerical/eigen.mirror` and its Fortran realisation at
`numerical/fortran/eigen.f90` are sibling-named under different
directory shelves — the directory disambiguates by kind (substrate
declaration vs Fortran realisation), the bare name (`eigen`) carries
the thing.

**Verification against AGENTS.md.** The rule (lines 320–340) forbids
`functor_laws.mirror`; the right shape is `property/laws/functor.mirror`.
Applied here: `eigen_fortran.f90` would be forbidden; the right shape
is `numerical/fortran/eigen.f90`. The path carries the realisation;
the name carries the thing.

### 5.3 Standalone-package layout (Phase B target)

When the Fortran Fate package factors out (§6), the layout becomes
standard fpm:

```
fortran-fate/                                  -- standalone repo or subtree
fortran-fate/fpm.toml                          -- package manifest
fortran-fate/src/                              -- source
fortran-fate/src/spectrum.f90
fortran-fate/src/eigen.f90
fortran-fate/src/laplacian.f90
fortran-fate/src/fiedler.f90
fortran-fate/src/eigengap.f90
fortran-fate/src/replicator.f90
fortran-fate/src/tournament.f90
fortran-fate/src/ffi.f90
fortran-fate/test/                             -- fpm-discovered tests
fortran-fate/test/test_eigen.f90
fortran-fate/test/test_fiedler.f90
fortran-fate/test/test_replicator.f90
fortran-fate/example/                          -- documentation examples
fortran-fate/example/connectome_settle.f90
fortran-fate/README.md
```

The mirror substrate at `boot/std/fate/numerical/` continues to declare
the actions; the bodies that were `\` in Phase A get filled by
`@io.exec("fpm install fortran-fate")` or by a vendor-bundled static
library. The substrate doesn't change shape; the realisation moves out
from under `boot/` into a sibling package.

---

## 6. The Fortran Fate package — standalone

This is Alex's key insight. A pure Fortran inference package that implements
the Spectral Settlement Strategy as a domain-general mechanism. Mirror
is one consumer; other projects doing spectral-settlement work can
consume the same package without taking mirror as a dependency.

### 6.1 What it implements

The package realises the seven structural commitments of the Spectral
Settlement Strategy (per `~/.reed/practice/insights/cross-domain/spectral-tick-tock-game-theory.md`
§10):

1. **Graph Laplacian construction** from a substrate-declared adjacency
   or conductivity matrix. `L = D - A` where `D` is the degree matrix
   and `A` is the (weighted) adjacency. Operates on dense or sparse
   inputs.
2. **Eigendecomposition** of the Laplacian. Calls LAPACK's `dsyev`
   (symmetric eigendecomposition; tridiagonal QR) for dense; LAPACK's
   `dsbevd` (banded) when bandwidth is small; ELPA's eigensolvers when
   matrix size exceeds the dense-LAPACK threshold (~10000 rows).
3. **Fiedler value computation.** Returns `lambda_2` (algebraic
   connectivity per Fiedler 1973). The convergence rate of consensus
   dynamics on the graph (Olfati-Saber & Murray 2004; ESS stability
   margin per game-theory §1).
4. **Settlement convergence (consensus dynamics).** Drives a state
   vector along the Laplacian gradient until `lambda_2`-bounded
   convergence. Implements the Olfati-Saber consensus protocol
   `dx/dt = -L x`. Integration via 4th-order Runge-Kutta or implicit
   trapezoidal (user-selectable for stiffness).
5. **Replicator dynamics on eigenvalues** (Spectral Settlement
   Strategy proper). Drives strategy frequencies via
   `dx_i/dt = x_i (f_i(x) - <f>)` per Taylor & Jonker 1978; the
   structure-preserving rational integrator per Sciencedirect (2025)
   *"A Structure-preserving rational integrator for the replicator
   dynamics"*
   (https://www.sciencedirect.com/science/article/pii/S1007570425008342).
6. **Tournament rule implementations.** The seven rules named in
   `boot/std/fate/tournament.mirror`: greedy, beam(k), elite(k),
   halving(k), tabu(k), anneal(t), ucb(c). Procedural Fortran 2018
   bodies; ~30–60 lines each.
7. **The conductivity tensor as the interaction graph Laplacian.**
   Loads a substrate-declared conductivity (from mirror) or a
   user-supplied adjacency matrix; constructs `L`; routes through
   the rest of the pipeline.

For heavy numerical lift the package wraps:

- **LAPACK** (reference Fortran, ~2 million lines, NetLib) — the
  dense-matrix eigendecomposition floor. Mature; numerically stable;
  bundled everywhere.
- **ELPA** (Eigenvalue SoLvers for Petaflop-Applications) — the
  large-scale eigensolver. Two-stage symmetric eigendecomposition;
  GPU-accelerated variants. Used when matrix size > ~10000.
  (https://elpa.mpcdf.mpg.de/, https://github.com/marekandreas/elpa)
- **FEAST** (interior eigenvalue solver) — contour-integration-based;
  used when only a Fiedler-region eigenvalue band is needed, not the
  full spectrum. (https://www.feast-solver.org/,
  https://arxiv.org/pdf/2002.04807)
- **MAGMA** (optional, GPU-accelerated LAPACK) — for v1+ GPU
  pathway; not v0. (https://icl.utk.edu/magma/)

### 6.2 Why standalone matters

Four reasons converge:

- **Other projects doing spectral inference can use it.** The
  Spectral Settlement Strategy is a domain-general mechanism with
  applications in multi-agent coordination, distributed consensus,
  cooperative game theory, evolutionary dynamics, network design,
  and AI alignment-as-coordination (game-theory §9). Locking the
  implementation inside mirror denies these consumers; factoring it
  out grows the reference implementation's user base.
- **Tests independently of mirror.** Standard fpm test discovery;
  no mirror build dependency; fast CI loop. Test failures isolate
  to the package, not to the mirror integration. The substrate-pull
  recognition (mirror needs *more* tests of the numerical floor,
  not fewer) lands cleanly under fpm.
- **Distributable via fpm.** Fortran Package Manager
  (https://github.com/fortran-lang/fpm; https://fpm.fortran-lang.org/)
  is the de-facto standard since ~2022. Modern Fortran community is
  building on it; the package becomes one fpm dependency away for
  any consumer.
- **Could become a reference implementation for Spectral Settlement
  Strategy.** The strategy has no canonical implementation today —
  game-theory §12 names this as the structural-claim-without-canonical-
  realisation gap. A pure Fortran package, mathematically transparent,
  documented against the Olfati-Saber / Hajek / Karnin / Taylor &
  Jonker citations, could become the reference. This is durable value
  beyond mirror.

### 6.3 Prior art

Four candidates from the Kagi survey:

- **evortran (2025).** *"evortran: a modern Fortran package for genetic
  algorithms"* (https://arxiv.org/pdf/2507.06082). Closest prior art.
  Implements selection (including tournament), crossover, mutation,
  elitism strategies in modern Fortran. **Does NOT do spectral
  settlement; does NOT implement replicator dynamics; does NOT
  construct graph Laplacians.** Different scope; partial overlap on
  tournament selection. The Fortran Fate package would extend
  evortran's tournament discipline with the spectral floor evortran
  lacks.
- **PGAPack (Argonne).** *Parallel Genetic Algorithm Library*
  (https://github.com/schlatterbeck/pgapack). Older; C-with-Fortran-interface;
  tournament selection supported. Same scope-overlap as evortran;
  no spectral content.
- **neural-fortran** (https://github.com/modern-fortran/neural-fortran).
  Parallel framework for deep learning in Fortran. NASA Goddard
  funding. Different mechanism entirely (feed-forward + convolutional
  layers, gradient descent); no spectral / consensus / replicator
  content. Mentioned here as evidence that Fortran inference is an
  active area (not a dying one); not prior art for this package.
- **RoseNNa** (https://github.com/comp-physics/roseNNa). *"Fast,
  portable, and minimally-intrusive library for neural network
  inference"*
  (https://www.sciencedirect.com/science/article/abs/pii/S0010465523003971).
  ONNX-format consumer; designed for augmenting PDE solvers with NN
  inference. Same scope-comment as neural-fortran; not prior art.
- **ATHENA** (https://www.theoj.org/joss-papers/joss.06492/10.21105.joss.06492.pdf).
  Fortran neural network library; JOSS-published 2024. Same
  scope-comment as the above two; not prior art.
- **fortran-lapack** (https://github.com/perazz/fortran-lapack;
  https://perazz.github.io/fortran-lapack/). Modern Fortran linear
  algebra library with NumPy/SciPy-like API; wraps reference LAPACK.
  A *dependency candidate* for the Fortran Fate package's
  eigendecomposition floor; not prior art for the inference
  mechanism itself.
- **fortran-lang stdlib `stdlib_linalg`**
  (https://stdlib.fortran-lang.org/module/stdlib_linalg.html;
  v0.8.0 released Jan 2026 per
  https://fortran-lang.discourse.group/t/fortran-standard-library-v0-8-0-released/10657).
  Standard-library linear algebra subroutines; eigenvalues / eigenvectors
  interfaces. *Another dependency candidate* for the Fortran Fate
  package; not prior art.

**Headline finding: no prior art for the proposed Fortran Fate package.**
Neural-network inference packages exist (neural-fortran, RoseNNa,
ATHENA); genetic-algorithm tournament packages exist (evortran,
PGAPack); linear-algebra packages exist (fortran-lapack, stdlib).
None of them combine: graph Laplacian + spectral eigendecomposition
+ replicator dynamics on eigenvalues + tournament rule dispatch +
consensus convergence + Fiedler-value monitoring. **The Fortran Fate
package would be the first.**

This matches game-theory §12's structural-claim-without-canonical-
realisation observation. The Spectral Settlement Strategy is novel as
a *named* strategy with the seven listed properties; existing tools
implement individual components (replicator dynamics in EGTTools per
https://github.com/Socrats/EGTTools; consensus dynamics in MATLAB
teaching code per Olfati-Saber & Murray's original papers; tournament
selection in evortran / PGAPack). The integration is novel; the
Fortran Fate package would be the integration.

### 6.4 Public API sketch

Fortran 2018 with ISO_C_BINDING. Procedural; no OO. Key subroutines
that external consumers would call:

```fortran
module fate_spectrum
  use iso_c_binding, only: c_double, c_int, c_ptr
  implicit none
  private
  public :: spectrum_t, eigendecompose, laplacian_of, fiedler_of, eigengap_of

  type :: spectrum_t
    integer(c_int) :: n
    real(c_double), allocatable :: eigenvalues(:)        ! ascending
    real(c_double), allocatable :: eigenvectors(:, :)    ! column-major
  end type spectrum_t

contains

  ! Eigendecomposition of a symmetric matrix via LAPACK dsyev.
  ! Returns eigenvalues in ascending order; eigenvectors as columns.
  ! For matrix > 10000 rows, dispatches to ELPA instead.
  subroutine eigendecompose(matrix, n, spectrum)
    real(c_double), intent(in)  :: matrix(:, :)
    integer(c_int), intent(in)  :: n
    type(spectrum_t), intent(out) :: spectrum
  end subroutine

  ! Graph Laplacian from adjacency/conductivity. L = D - A.
  ! Symmetric for undirected graphs.
  subroutine laplacian_of(adjacency, n, laplacian)
    real(c_double), intent(in)  :: adjacency(:, :)
    integer(c_int), intent(in)  :: n
    real(c_double), intent(out) :: laplacian(:, :)
  end subroutine

  ! Algebraic connectivity (second-smallest eigenvalue of the
  ! Laplacian). ESS stability margin per game-theory §1.
  function fiedler_of(matrix, n) result(lambda_2)
    real(c_double), intent(in) :: matrix(:, :)
    integer(c_int), intent(in) :: n
    real(c_double) :: lambda_2
  end function

  ! Folk-Theorem discount threshold derived from the spectrum:
  ! 1 - (lambda_2 / lambda_max). Patience required per game-theory §4.
  function eigengap_of(matrix, n) result(discount)
    real(c_double), intent(in) :: matrix(:, :)
    integer(c_int), intent(in) :: n
    real(c_double) :: discount
  end function

end module fate_spectrum
```

```fortran
module fate_replicator
  use iso_c_binding, only: c_double, c_int
  implicit none
  private
  public :: apply_replicator

contains

  ! One step of replicator dynamics on a strategy frequency vector.
  ! dx_i/dt = x_i (f_i(x) - <f>) where f is the payoff vector.
  ! Integration via structure-preserving rational integrator
  ! (Sciencedirect 2025).
  subroutine apply_replicator(state, payoff, n, dt, new_state)
    real(c_double), intent(in)  :: state(:)
    real(c_double), intent(in)  :: payoff(:)
    integer(c_int), intent(in)  :: n
    real(c_double), intent(in)  :: dt
    real(c_double), intent(out) :: new_state(:)
  end subroutine

end module fate_replicator
```

```fortran
module fate_tournament
  use iso_c_binding, only: c_double, c_int
  use fate_spectrum, only: spectrum_t
  implicit none
  private
  public :: tournament_rule_t, apply_rule, tournament

  ! Sum type emulation via tag + parameter.
  type :: tournament_rule_t
    integer(c_int) :: tag             ! 1=greedy 2=beam 3=elite 4=halving 5=tabu 6=anneal 7=ucb
    integer(c_int) :: param_int       ! k for beam/elite/halving/tabu
    real(c_double) :: param_real      ! t for anneal; c for ucb
  end type tournament_rule_t

contains

  ! Apply a single tournament rule to a candidate set with loss scores.
  ! Returns the surviving candidates' indices.
  subroutine apply_rule(candidate_losses, n_candidates, rule, survivors, n_survivors)
    real(c_double),           intent(in)  :: candidate_losses(:)
    integer(c_int),           intent(in)  :: n_candidates
    type(tournament_rule_t),  intent(in)  :: rule
    integer(c_int),           intent(out) :: survivors(:)
    integer(c_int),           intent(out) :: n_survivors
  end subroutine

  ! Full tournament: fold rules over candidates; settle to a spectrum.
  ! Returns the spectrum of the settled state (the au coordinate).
  subroutine tournament(candidate_losses, n_candidates, rules, n_rules, &
                         conductivity, settled_spectrum)
    real(c_double),           intent(in)  :: candidate_losses(:)
    integer(c_int),           intent(in)  :: n_candidates
    type(tournament_rule_t),  intent(in)  :: rules(:)
    integer(c_int),           intent(in)  :: n_rules
    real(c_double),           intent(in)  :: conductivity(:, :)
    type(spectrum_t),         intent(out) :: settled_spectrum
  end subroutine

end module fate_tournament
```

Each module exposes ISO_C_BINDING-compatible signatures so the same
library is callable from Rust (via FFI), Julia (via `ccall`), Python
(via `ctypes`), or any LAPACK-aware ecosystem. The substrate side
(mirror) consumes via the FFI; standalone consumers consume via fpm.

### 6.5 Connection to spectral-tick-tock-game-theory grounding

The package's seven properties are not chosen freely; they realise
the game-theory document's structural claims:

- **Settlement is an eigenvalue minimum** (game-theory §1) →
  `eigendecompose` returns eigenvalues in ascending order;
  `eigenvalues(1)` is the settled state.
- **Fiedler = ESS stability margin** (game-theory §1) → `fiedler_of`
  returns the algebraic connectivity; large = robust settlement;
  small = fragile.
- **Eigengap = Folk Theorem discount threshold** (game-theory §4) →
  `eigengap_of` returns `1 - lambda_2 / lambda_max`; consumers can
  ask "how much patience is needed to sustain this settlement?"
- **Consensus rate = lambda_2** (game-theory §10; Olfati-Saber &
  Murray 2004) → the settle loop's convergence is bounded by
  `lambda_2`; the package can report iteration count + final residual.
- **Replicator on eigenvalues = ESS selection** (game-theory §8;
  Taylor & Jonker 1978) → `apply_replicator` drives strategy
  frequencies; the package supplies the integrator (structure-preserving
  rational; Sciencedirect 2025).
- **Memory-infinite via the spectrum** (game-theory §10) → the
  spectrum_t struct compresses the full history into ~n+n² doubles;
  replay reconstructs from the spectrum + seed.
- **Non-reactive, non-prescriptive** (game-theory §10) → the package
  integrates inputs into the eigensystem and reports the settled
  state; it does not select per-step "actions" against an opponent.

The package isn't doing free design — every public API is justified by
a cited game-theoretic property.

---

## 7. The integration pathway

How mirror calls the Fortran Fate package end-to-end.

### 7.1 Substrate declares the actions

```mirror
-- boot/std/fate/numerical/eigen.mirror
in @prism
in @fate
in @code/fortran

grammar @fate/numerical/eigen {
  in @code/fortran

  -- the spectrum_t derived type, mirror-side declaration
  type spectrum {
    n:            precision,
    eigenvalues:  [precision],
    eigenvectors: [[precision]],
  }

  -- eigendecomposition action. body is \ (parked); the realisation
  -- is the Fortran-compiled symbol routed through ISO_C_BINDING.
  action eigendecompose(matrix: [[precision]]) -> spectrum { \ }
}

out spectrum
out eigendecompose
out @fate/numerical/eigen
```

Same pattern for `laplacian.mirror`, `fiedler.mirror`, `eigengap.mirror`,
`replicator.mirror`, `tournament.mirror` (§5.1 layout).

### 7.2 flang compiles Fortran to LLVM IR

During build (orchestrated by `boot/std/code/fortran/build.mirror`):

```
flang -emit-llvm -O3 -c boot/std/fate/numerical/fortran/eigen.f90 \
      -o build/fate/numerical/eigen.ll
flang -emit-llvm -O3 -c boot/std/fate/numerical/fortran/spectrum.f90 \
      -o build/fate/numerical/spectrum.ll
# ... one per Fortran source file
```

The `.ll` files enter mirror's content-addressing pipeline:
`@code/llvm/ir` tokenises them; each function gets a content-OID;
the substrate has visibility into the IR structure (per
`boot/std/code/llvm/ir.mirror`'s top-level constructs: define / declare
/ type / global / metadata).

### 7.3 Mirror consumes the IR via `@code/llvm/ir`

The pathway is the existing one. The substrate-side action declarations
at `@fate/numerical/*` resolve through the action-dispatch path (per
`substrate-native-fate-tournament` §6: Tick 0.5, the dispatcher
substrate-pull-realize work) to LLVM-IR symbols compiled from Fortran.

### 7.4 The bootstrap's small Rust FFI layer

Estimated ~200 lines of Rust (versus the ~80–160 the previous spec
proposed for full Rust spectral primitives). The FFI layer:

- declares `extern "C"` signatures matching the Fortran ISO_C_BINDING
  surface (~50 lines, one block per Fortran module),
- routes substrate-evaluator dispatch (when it encounters a `\` body
  under `@fate/numerical/*`) to the corresponding extern symbol (~80
  lines, table-driven),
- handles the C-array ↔ Rust-`Vec<f64>` marshalling (~50 lines,
  zero-copy where possible, copy where necessary for LAPACK's
  column-major convention),
- carries error checking from LAPACK's `info` return code (~20 lines).

This is the *only* Rust addition the spec calls for. It is
bugfix-class (substrate-pull-realize) under AGENTS.md's exception
clause: the substrate has been declaring spectral actions; the Rust
floor is restoring honest dispatch to the Fortran-substrate realisation.
Tag commits `[substrate-pull:realize]`.

### 7.5 The build.rs addition

Cargo's `build.rs` (in `bootstrap/`) gains a section that:

1. Locates flang on PATH (or reads `FLANG` env var; fails build if
   absent with a clear message pointing to the spec).
2. Compiles each Fortran source under `boot/std/fate/numerical/fortran/`
   to `.o` (via `flang -c`).
3. Archives into `libfate_numerical.a` (via `ar` or `cc::Build`).
4. Links the bootstrap binary against `libfate_numerical.a` + LAPACK +
   the flang runtime (`-lFortran_main`, `-lFortranRuntime`,
   `-lFortranDecimal`).

Estimated ~50 lines of `build.rs` Rust. One-time addition; subsequent
Fortran source additions don't require build.rs changes (it walks the
`fortran/` directory).

---

## 8. Migration strategy

Sequenced ticks. Each is one commit; each is independently verifiable.
Phase A migrates the existing Numerical region; Phase B adds new
primitives for the substrate-native-fate-tournament leapfrog.

### 8.1 Phase A (this spec's primary scope)

| Tick | Scope | Substrate or Rust | Verification |
|------|-------|-------------------|--------------|
| A.0 | Declare `@code/fortran` grammar in `boot/std/code/fortran.mirror`; no Fortran source yet | Pure substrate | `mirror compile boot/std/code/fortran.mirror` produces stable OID |
| A.1 | Write `numerical/fortran/eigen.f90` + `numerical/fortran/spectrum.f90` (LAPACK wrapper for symmetric eigendecomposition, ~80 lines total); compile via flang manually | Pure Fortran | `flang -emit-llvm -O3 -c eigen.f90 -o eigen.ll` succeeds; LAPACK link verifies |
| A.2 | Add `boot/std/code/fortran/build.mirror` orchestration grammar; build.rs addition (~50 lines Rust); FFI extern "C" declarations (~50 lines Rust) | Mostly Rust (build infrastructure) | `cargo build --release --manifest-path bootstrap/Cargo.toml` succeeds with libfate_numerical.a linked |
| A.3 | Add `boot/std/fate/numerical/eigen.mirror` substrate declaration; body `\` (parked); FFI routing in evaluator | Mixed substrate + Rust | `mirror craft boot` succeeds; spectrum-OID is stable |
| A.4 | Replace `eigen_d` callsites in bootstrap to invoke the Fortran path; delete `spectral.rs:957–1063` (Rust eigendecomposition) | Pure Rust (deletion + redirection) | Existing spectral tests pass; `cargo test --release --manifest-path bootstrap/Cargo.toml` green |

Phase A is **5 ticks**. Bounded; the Numerical migration completes.

### 8.2 Phase B (deferred; unblocks substrate-native-fate-tournament)

| Tick | Scope |
|------|-------|
| B.1 | `numerical/fortran/laplacian.f90` + substrate declaration |
| B.2 | `numerical/fortran/fiedler.f90` + substrate declaration |
| B.3 | `numerical/fortran/eigengap.f90` + substrate declaration |
| B.4 | `numerical/fortran/replicator.f90` + substrate declaration |
| B.5 | `numerical/fortran/tournament.f90` + substrate declaration (all seven rules) |
| B.6 | Factor `numerical/fortran/` out as standalone fpm package; mirror consumes via dependency |

Phase B is **6 ticks**, each ~1 session. Sequencing: B.1 unblocks B.2
and B.3 (which depend on Laplacian); B.4 and B.5 are independent of
B.1–B.3 (operate on different inputs); B.6 is the final consolidation.

### 8.3 First-tick verification

Tick A.1 (write Fortran + manual flang compile) is the proof of the
FFI pathway. Smallest verifiable unit:

```fortran
! boot/std/fate/numerical/fortran/dot.f90
module fate_dot
  use iso_c_binding, only: c_double, c_int
  implicit none
  private
  public :: dot_product_c

contains

  function dot_product_c(x, y, n) result(r) bind(c, name='fate_dot_product')
    real(c_double), intent(in) :: x(*)
    real(c_double), intent(in) :: y(*)
    integer(c_int), value      :: n
    real(c_double)             :: r
    integer :: i
    r = 0.0_c_double
    do i = 1, n
      r = r + x(i) * y(i)
    end do
  end function dot_product_c

end module fate_dot
```

A pre-A.1 *smoke tick* compiles this, links against the bootstrap,
verifies a `Rust calls Fortran dot product` integration test passes.
This confirms:

- flang is installed and works,
- the FFI signature compiles via ISO_C_BINDING,
- Cargo can link the `.o`,
- the bootstrap can call into the symbol,
- the result returns correctly.

If any of these fail, the pathway has a bug; fix before A.1.

---

## 9. Prior art (Kagi research)

All URLs verified via Kagi search. Each entry: one-sentence summary +
relevance to this work.

### 9.1 flang status and maturity

- **LLVM Blog (Mar 2025).** *"LLVM Fortran Levels Up: Goodbye flang-new,
  Hello flang!"* https://blog.llvm.org/posts/2025-03-11-flang-new/
  Documents flang's graduation from experimental to canonical;
  vendor adoption (AMD, Arm, Huawei). **Relevance: load-bearing for
  §3.1's flang choice.**

- **Phoronix (2025).** *"LLVM 20's Great Fortran Language Support With
  Flang."* https://www.phoronix.com/news/LLVM-20-Flang Confirms flang
  is production-ready for current Fortran standards. **Relevance:
  external validation of §3.1's recommendation.**

- **Linaro Blog (Aug 2023).** *"Comparing LLVM Flang with other
  Fortran compilers."* https://www.linaro.org/blog/comparing-llvm-flang-with-other-fortran-compilers/
  Benchmark: flang 23% slower than gfortran (geomean). **Relevance:
  honest performance baseline for §3.1.**

- **LLVM Discourse (2024).** *"Performance analysis for TSVC."*
  https://discourse.llvm.org/t/performance-analysis-for-tsvc/75413/1
  Updates the gap to 11% slower than gfortran; identifies
  vectorization as the primary axis. **Relevance: validates that
  flang is closing the gap.**

- **ArXiv (Sep 2024).** *"Fully integrating the Flang Fortran compiler
  with standard MLIR."* https://arxiv.org/pdf/2409.18824 Maps Fortran
  to MLIR; describes FIR → LLVM IR pathway. **Relevance: documents
  the substrate-aligned compilation path.**

- **ACM (Nov 2025).** *"Lowering and Runtime Support for Fortran's
  Multi-Image Parallel Features."* https://dl.acm.org/doi/10.1145/3731599.3767480
  flang's coarray implementation lands. **Relevance: confirms
  Fortran 2018+ coverage for future package extensions.**

### 9.2 LAPACK and the numerical Fortran ecosystem

- **NetLib LAPACK.** https://www.netlib.org/lapack/ The canonical
  reference. LAPACK 3.12.0 released Nov 2023; ~2 million lines of
  Fortran 90. **Relevance: the eigendecomposition floor.**

- **fortran-lapack** (perazz). https://github.com/perazz/fortran-lapack/
  Modern Fortran linear algebra library with NumPy/SciPy-like API;
  wraps reference LAPACK. **Relevance: dependency candidate for the
  Fortran Fate package's eigendecomposition.**

- **fortran-lang stdlib `stdlib_linalg`.**
  https://stdlib.fortran-lang.org/module/stdlib_linalg.html Standard
  library linear algebra; eigenvalues / eigenvectors interfaces;
  v0.8.0 released Jan 2026 per
  https://fortran-lang.discourse.group/t/fortran-standard-library-v0-8-0-released/10657
  **Relevance: alternative dependency candidate.**

- **ELPA.** https://github.com/marekandreas/elpa and
  https://elpa.mpcdf.mpg.de/ Eigenvalue SoLvers for Petaflop-Applications;
  two-stage symmetric eigendecomposition; GPU support. **Relevance:
  the large-matrix eigensolver for matrices > ~10000 rows.**

- **FEAST eigenvalue solver.** https://www.feast-solver.org/ and
  https://arxiv.org/pdf/2002.04807 Contour-integration-based interior
  eigenvalue solver. **Relevance: when only the Fiedler-region band
  is needed, not the full spectrum.**

- **MAGMA.** https://icl.utk.edu/magma/ and
  https://developer.nvidia.com/magma GPU-accelerated LAPACK; CPU+GPU
  hybrid algorithms. **Relevance: v1+ GPU pathway for the Fortran
  Fate package; out of v0 scope.**

- **PETSc / SLEPc.** https://petsc.org/ and https://github.com/slepc/slepc
  Scalable Library for Eigenvalue Problem Computations on PETSc.
  **Relevance: distributed eigensolver pathway; deferred (§11).**

### 9.3 Fortran tournament / inference / settlement prior art

- **evortran (2025).** *"evortran: a modern Fortran package for
  genetic algorithms."* https://arxiv.org/pdf/2507.06082 Closest
  prior art for tournament selection in modern Fortran. **Relevance:
  partial-overlap; the Fortran Fate package extends evortran's
  tournament discipline with the spectral floor evortran lacks.**

- **PGAPack (Argonne).** https://github.com/schlatterbeck/pgapack
  Older C-with-Fortran-interface; tournament selection supported.
  **Relevance: secondary prior art for tournament rules.**

- **neural-fortran.** https://github.com/modern-fortran/neural-fortran
  Parallel deep learning framework in modern Fortran; NASA Goddard
  funded. **Relevance: evidence Fortran inference is an active area.**

- **RoseNNa.** https://github.com/comp-physics/roseNNa and
  https://www.sciencedirect.com/science/article/abs/pii/S0010465523003971
  Minimally-invasive ONNX-format consumer; designed for augmenting
  PDE solvers with NN inference. **Relevance: pattern for Fortran-
  package distribution.**

- **ATHENA.** https://www.theoj.org/joss-papers/joss.06492/10.21105.joss.06492.pdf
  Fortran neural network library; JOSS-published 2024. **Relevance:
  another data point on Fortran inference activity.**

- **A Structure-preserving rational integrator for the replicator
  dynamics (2025).** https://www.sciencedirect.com/science/article/pii/S1007570425008342
  Quadratically convergent and dynamically consistent integrator for
  replicator dynamics. **Relevance: the integrator the Fortran Fate
  package adopts for `apply_replicator`.**

- **EGTTools** (Python+C++). https://github.com/Socrats/EGTTools
  Evolutionary Game Theory toolbox; replicator dynamics + fixation
  probabilities + Monte Carlo. **Relevance: evidence the integration
  the Fortran Fate package proposes is novel — EGTTools does
  replicator-only, not the full spectral-settlement integration.**

### 9.4 Fortran ecosystem and modernity

- **Fortran Package Manager (fpm).** https://github.com/fortran-lang/fpm
  and https://fpm.fortran-lang.org/ The de-facto package manager
  since ~2022. **Relevance: §6's standalone-package distribution
  surface.**

- **Fortran 2023 features.** https://www.olcf.ornl.gov/wp-content/uploads/2024-04_OLCFUserCall_FortranStandard.pdf
  ORNL overview of the standard. **Relevance: validates the
  modern-Fortran-only stance in §2.5.**

- **ISO_C_BINDING.** https://gcc.gnu.org/onlinedocs/gfortran/ISO_005fC_005fBINDING.html
  and https://fortran-lang.org/learn/intrinsics/cfi/ Standard C
  interop. **Relevance: §6.4's API uses this exclusively for FFI.**

- **Golem (Nov 2024).** *"Die Renaissance von Fortran"* (in German).
  https://www.golem.de/news/programmiersprachen-die-renaissance-von-fortran-2411-190871.html
  Popular-press coverage of Fortran's renewed activity. **Relevance:
  general sentiment baseline.**

- **ArXiv (Feb 2024).** *"Fortran... ok, and what's next?"*
  https://arxiv.org/abs/2402.07520 Modern Fortran standardisation +
  fortran-lang community work. **Relevance: confirms ecosystem
  momentum.**

### 9.5 Graph Laplacians and spectral graph theory

- **Algebraic connectivity (Wikipedia).** https://en.wikipedia.org/wiki/Algebraic_connectivity
  The Fiedler value = second-smallest Laplacian eigenvalue.
  **Relevance: the load-bearing definition.**

- **Olfati-Saber & Murray (2002).** *"Consensus Protocols for Networks
  of Dynamic Agents."* https://www.cds.caltech.edu/~murray/papers/2002o_om03-acc.html
  The consensus dynamics the package implements. **Relevance: §6.1
  property 4's citation.**

- **TraceMIN-Fiedler.** https://scispace.com/pdf/tracemin-fiedler-a-parallel-algorithm-for-computing-the-4csahp5r2v.pdf
  Parallel algorithm for computing the Fiedler vector. **Relevance:
  efficient `fiedler_of` for large matrices.**

### 9.6 Comparison points (when Fortran vs alternatives)

- **The State of Julia for Scientific Machine Learning.**
  https://arxiv.org/html/2410.10908v1 Julia as Python successor; gaps
  vs Fortran. **Relevance: comparison baseline for §6.2.**

- **GMD (2023).** *"Comparing the Performance of Julia on CPUs versus
  GPUs and Julia-MPI versus Fortran-MPI."*
  https://gmd.copernicus.org/articles/16/5539/2023/ Julia vs Fortran-MPI
  ocean model benchmark. **Relevance: when Fortran wins (HPC,
  numerically dense).**

- **nalgebra (Rust) benchmark.** https://github.com/dimforge/nalgebra/issues/1468
  *"Nalgebra seems to run an order of magnitude slower than Numpy."*
  **Relevance: motivates not using nalgebra for the Fortran Fate
  package's numerical floor.**

- **faer-rs.** https://news.ycombinator.com/item?id=40143669 Pure-Rust
  linear algebra; comparable to LAPACK for matmul. **Relevance:
  potential future Rust fallback if flang ever becomes unavailable;
  not the v0 path.**

**Source count: 30+ URLs cited above; 26 entries with one-sentence
summary + URL + relevance per the spec's required shape.**

### 9.7 Headline findings

Three:

1. **No prior art for the proposed Fortran Fate package as an
   integrated spectral-settlement library.** Existing packages
   implement individual components (evortran: tournament selection;
   EGTTools: replicator dynamics; LAPACK: eigendecomposition;
   neural-fortran / RoseNNa / ATHENA: NN inference). None combines
   graph Laplacian + spectral eigendecomposition + replicator
   dynamics + tournament dispatch + consensus convergence into one
   library. The Fortran Fate package would be the first.

2. **flang is ready for production numerical work in 2026, with
   acceptable trade-offs.** First official release Mar 2025; vendor
   adoption (AMD, Arm, Huawei); 11–23% slower than gfortran
   (geomean across benchmarks), narrowing. LLVM-IR pathway is the
   load-bearing alignment win.

3. **Fortran scientific computing is an active, not dying, area.**
   fpm ecosystem growing; fortran-lang stdlib v0.8.0 released Jan
   2026; multiple modern inference packages (neural-fortran,
   RoseNNa, ATHENA, evortran) shipped in 2024–2025. The
   "renaissance" framing from popular press is overstated, but the
   community-driven modernisation is real.

---

## 10. Open decisions for Alex

Things that need Alex's eyes before any tick fires. The spec resolves
what's clearly resolved and surfaces the rest.

### 10.1 Exact substrate path for Fortran source files — open

Two candidates:

- **`boot/std/fate/numerical/fortran/`** — the consumer-driven path;
  Fate-specific Fortran lives under Fate's namespace. §5.1's lean.
- **`boot/std/code/fortran/`** — the language-tag-driven path; all
  Fortran source lives under `@code/fortran`'s namespace regardless
  of consumer.

Mara's lean: consumer-driven (§5.1). The substrate path carries the
consumer's namespace; mirror's own substrate hierarchy is intent-
oriented (Fate → numerical → fortran), not language-tag-oriented.
Matches the pattern of `boot/std/code/llvm/emit.mirror` (Mirror's own
binary emission lives under `@code/llvm/emit`, not under a generic
`@code/llvm/`).

Resolved-if-Alex-agrees; open until then.

### 10.2 FFI shape — mostly resolved

§2.4's recommendation: hybrid (LLVM-IR-as-substrate + static-library
link). The IR enters mirror's content-addressing pipeline; the link
itself uses Cargo's static-library machinery.

What's open: whether the substrate explicitly carries the `.ll` files
as first-class artifacts (like `bootstrap/mirror.ll` does today) or
only observes the IR mid-pipeline (compiled fresh on each build). The
former enables content-addressing of the Fortran IR per source file;
the latter is simpler. Mara's lean: first-class artifacts; the
substrate gains visibility per Fortran source file.

### 10.3 Standalone-from-day-1 vs mirror-internal-with-path-to-standalone — open

Phase A in §8 puts Fortran source under `boot/std/fate/numerical/fortran/`
(mirror-internal). Phase B Tick B.6 factors it out. Alternative: start
standalone from day 1 — the Fortran source lives in a separate
repo / subtree from the beginning; mirror consumes via fpm dependency.

Mara's lean: mirror-internal initially. Reasoning:

- During Phase A the Fortran content is small (~80 lines for
  `eigen.f90` + `spectrum.f90`); a separate repo would be overhead.
- The FFI pathway needs proving end-to-end first; debugging a
  single-repo flow is faster.
- Once Phase B Tick B.5 lands (full tournament + replicator + etc.),
  the package's surface is sized to justify standalone-hood.
- Tick B.6 (factor out) becomes natural rather than premature.

Alex may prefer standalone-from-day-1 (more discipline; clearer
separation; usable by other projects sooner). Open.

### 10.4 Build system integration — partially resolved

Cargo's `build.rs` is the proximate integration (mirror's Rust bootstrap
is cargo-built). Within build.rs:

- **Option 1.** Direct flang invocation via `std::process::Command` +
  ar archiving via `cc::Build`. Simpler; no fpm dependency at
  build-time.
- **Option 2.** `fpm build` as a build.rs step; mirror consumes the
  fpm-built `.a`. Adds fpm as a build-time dependency; gains fpm's
  dependency resolution (transitive LAPACK / ELPA / FEAST).
- **Option 3.** Both — build.rs invokes fpm during Phase B (once the
  package is standalone); direct flang during Phase A.

Mara's lean: Option 3 (timed to the migration). Direct flang in Phase
A keeps build.rs simple while the package surface is small; fpm in
Phase B (post Tick B.6) once the standalone package exists and has
fpm.toml. Open.

### 10.5 Fortran runtime dependency — mostly resolved

flang has its own runtime (`libFortranRuntime`, `libFortranDecimal`,
`libFortran_main`); LAPACK and most Fortran code require a Fortran
runtime to link. Two paths:

- **Link flang's runtime explicitly** in build.rs (`-lFortranRuntime`
  etc.). Bootstrap binary grows by ~few MB.
- **Link gfortran's `libgfortran` runtime** as the canonical Fortran
  runtime. Wider availability; smaller; but mixing flang-compiled
  objects with gfortran runtime can be fragile.

Mara's lean: flang's runtime explicitly. Reasoning: flang is the
compiler; its runtime is the documented match; size growth is
acceptable (the bootstrap is currently 370KB; another few MB is
tolerable for the substrate-alignment win). Open if Alex prefers
smaller bootstrap.

### 10.6 Module / library naming for the standalone package — open

Candidates for the fpm package name:

- **`fortran-fate`** — matches the substrate naming; clear association
  with mirror's Fate substrate.
- **`spectral-settlement`** — names the strategy, not the substrate;
  reads as a domain-general library; better for consumers outside
  mirror.
- **`fate-spectrum`** — hybrid; names the Fate concept + the spectral
  primitive.
- **`libsettlement`** — C-library-style; clear FFI focus.

Mara's lean: `spectral-settlement` for the public-facing package
name; `fate_*` for the internal Fortran module names. Reasoning:
standalone consumers don't know mirror's "Fate" naming; they know
"spectral settlement strategy" from the game-theory document. The
public name reflects what the library *does*; the module names
reflect the substrate origin. Open.

---

## 11. What this spec doesn't cover

Bounded scope. Out of scope:

- **Implementation details of the eigendecomposition wrapper.** §6.4's
  API sketch names the signatures; the body of each subroutine (LAPACK
  call args, error handling, dispatch threshold to ELPA, etc.) is
  deferred to the implementation ticks.
- **GPU integration.** MAGMA is mentioned as a v1+ path; the v0
  Fortran Fate package is CPU-only (LAPACK / ELPA on dense / banded
  symmetric eigenproblems). MAGMA / cuSOLVER pathways are
  spec-level work for a later iteration.
- **Distributed eigensolvers.** ScaLAPACK / SLEPc / PETSc are listed as
  prior art; integration is deferred. The Fortran Fate package's v0
  is single-node (single-image; no coarrays). The 450-node connectome
  fits comfortably in a single LAPACK call.
- **Python / Julia / Rust interop bindings for the standalone package.**
  The ISO_C_BINDING surface (§6.4) is interop-ready; actual
  `python -m pip install fortran-fate` / `using FortranFate` / Cargo
  crate bindings are downstream work. Each is a separate spec.
- **Migration of the parser combinator engine (Structural region of
  `spectral.rs`).** §4's table notes that ~3050 lines of
  `spectral.rs` are Structural (Combinator enum + walker + AST
  fold + render). These migrate via `parser-as-prism-grammar.md`'s
  path (parser as substrate); they are NOT Fortran-pathway candidates.
  Out of scope for this spec.
- **Migration of `hash.rs` (CoincidenceHash projections).** ~270 lines
  of Rust; partially numerical (projection-matrix arithmetic), partially
  structural (SHA-256 dispatch). Tier-2 migration candidate; the
  projection-matrix arithmetic could move to Fortran in a later tick.
  Deferred.
- **Pre-Phase-A Rust deletion.** Phase A Tick A.4 deletes
  `spectral.rs:957–1063` (the Rust `eigen_d`). It does NOT delete
  the `Spectrum<N>` const-generic struct; the const-generic surface
  may still be used by static-size callers. The Fortran path returns
  a dynamic-size `spectrum` substrate type; coexistence during Phase A
  is acceptable.
- **flang vendor-fork choice.** AMD's downstream flang, Arm's flang,
  Huawei's flang are out of scope; mirror builds against canonical
  LLVM Flang. Vendor-specific tuning is a deployment concern, not a
  spec concern.
- **The standalone package's licensing.** Spec assumes it follows
  mirror's license (Apache-2.0 per `AGENTS.md`-adjacent norms; verify
  with Alex before factoring out). Out of scope for this spec.

---

## 12. Provenance

*Session-long dialogue between Alex and Reed, 2026-05-27.* Three
streams collapsed into one spec:

1. The recognition that pure numerical computation isn't `@io` —
   it's content-mobile to a numerical substrate grammar.
2. The choice of flang as the substrate compiler and Fortran as the
   numerical source language, grounded in mirror's existing
   `@code/llvm/ir` pathway and Fortran's LAPACK heritage.
3. The decision to build the numerical content as a *standalone*
   Fortran package (the Fortran Fate package, distributable via fpm)
   rather than as mirror-internal code.

The spec captures the architecture across all three. No
implementation in this commit; no `.rs` or `.f90` or `.mirror` files
modified. Markdown only.

The spec is Mara's, written on `mara/shard-chain` against HEAD
`c0bb724` (the `substrate-native-fate-tournament` spec). Markdown
only; no `.rs` files modified; no `.mirror` files modified;
pre-commit hook passes cleanly.

The next tick is Phase A Tick A.0 (declare `@code/fortran` grammar)
or — if Alex green-lights the smoke tick first — the pre-A.1 dot-
product FFI verification.

The spec captures the architecture. The implementation answers it.

---

## 13. Stop-and-report findings

The orientation requested several stop-and-report checks. Outcomes:

- **HEAD check.** HEAD is `c0bb724` as expected; clean working tree.
  No stop.
- **Numerical / structural split already present.** §4's audit
  found that `spectral.rs` is dominantly Structural (parser combinator
  engine + AST fold + render), not Numerical. The orientation's
  estimate of 1500–2500 Numerical lines is **substantially wrong**;
  the actual count is ~107 lines (one function + one struct). The
  survey approach was adjusted: rather than identifying a sizeable
  Numerical region to migrate, the spec notes the small bounded
  Numerical migration and frames most of `spectral.rs` as
  out-of-scope (deferred to `parser-as-prism-grammar.md`'s path).
- **Existing Fortran Fate package check.** §6.3 found no prior art
  for the proposed integrated package. evortran is closest (tournament
  selection in modern Fortran); does not implement the spectral
  floor. **Not reinventing; the package would be the first of its
  kind.**
- **flang maturity check.** §3 cited multiple 2024–2026 sources;
  flang is production-ready with 10–25% performance gap vs gfortran.
  **Not stopping; the trade-off is acceptable.** Fallback to gfortran
  documented (§3.2) if needed.
- **LLVM IR pathway gaps.** None surfaced in the research. flang's
  `-emit-llvm` produces standard LLVM IR consumable by mirror's
  existing `@code/llvm/ir`. **No concrete gap.**

No contradictions found between this spec and the existing
`parse-as-fate-tournament` / `substrate-native-fate-tournament`
specs. This spec *replaces* the proposed Rust floor in
`substrate-native-fate-tournament` §5 with the Fortran-via-flang
pathway; both specs' upper-level commitments (tournament body is
substrate; `au` is spectral coordinate; conductivity IS Laplacian)
are preserved unchanged. The relationship is *refinement*, not
contradiction: where `substrate-native-fate-tournament` proposed
*how* the spectral primitives are realised at the floor, this spec
refines that proposal to a substrate-aligned Fortran path.
