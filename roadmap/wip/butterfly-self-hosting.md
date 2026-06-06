# Butterfly Self-Hosting — the @io → machine-code commitment

*2026-06-06. Mara. Roadmap (architectural decision, not implementation).*

Status: **wip** — the decision is *this week's*; the work is post-v0.1.0.
Branch: `reed/pretty-printer-research` (spectral); commits land on `main` in
mirror.
Reads from: `roadmap/pending/phase-1` … `phase-7-*.md`, `roadmap/pending/runtime-elevation.md`,
`roadmap/pending/tracks.md`, `roadmap/wip/{v1-launch,cloud-deployment,kintsugi-ci-release-v0.1}.md`,
`docs/specs/{numerical-substrate-via-fortran,architecture-flang-mirror-numerical-split,spectral-triple-binary,the-convergence,cli-as-prism,lambda-shell,mosaic}.md`,
`shards/{code/rust,io/cargo,mirror/{mosaic,au,spec,store,cli}}.mirror`.
Does not duplicate them. Names the **last-responsible-moment** commitment
for how `@io` becomes machine code, and reconciles the existing phase
docs against it.

---

## 0. The last-responsible-moment frame

**The window is open and closes soon.** Two clocks are ticking past each
other:

- **The v0.1.0 cut** (T11.7, `roadmap/wip/kintsugi-ci-release-v0.1.md`).
  After the tag, the *substrate shape* is what we have. The 13 shards on
  `shards/` floor are the substrate's declared vocabulary; the @io/cargo
  contract is the declared codegen boundary; `au(altitude)` is the
  declared output type of settle. **Locking the substrate shape is the
  point of cutting v0.1.0.** That's correct, and we ship it now.
- **The Phase 2 → Phase 7 progression.** Phase 2 sketches @mirror/syntax
  as a parser; Phase 4 introduces @code/mirror as a render template;
  Phase 6 introduces NumericalPrism backends; Phase 7 declares
  self-hosting. **Each phase will commit to a codegen path implicitly
  if we let it.** Phase 4 already names "fragmentation generated from
  `@fragmentation + @code/rust`" — that's a codegen commitment by a
  side door. Phase 6 already names LapackBackend / MetalBackend /
  OpenCLBackend — that's a numerical-substrate commitment by a side
  door.

**The last responsible moment to commit the butterfly architecture is
the window between v0.1.0 cut and the start of Phase 4 implementation
work.** Lean's discipline applies precisely:

- **Decide too early** and we paint ourselves into a corner: Phase 4's
  @code/rust translate template gets written as if @code/rust is the
  permanent codegen, and graduating to @code/llvm later costs a
  redesign rather than an extension.
- **Decide too late** and Phase 4 / Phase 6 land with no shared
  contract for what "emit machine code" means at the substrate. Two
  codegen paths grow in parallel (cargo for fragmentation, FFI for
  LAPACK) with no plan to converge. The butterfly never assembles
  because no single altitude owns the codegen contract.

**This document is the commitment.** Not the implementation — the
commitment. The implementation lands across Phases 4 / 6 / 7; the
shape it must take is what we name here.

What locks if we wait past Phase 4 start: the @code/rust translate
template's typed surface, the fragmentation-as-generated artifact
shape, the .shatter projection of `au(@code/*)` records. Each of these
will be written; each one assumes some answer to the butterfly
question. The answer needs to be written *first*, or the assumptions
diverge.

What locks if we commit too early (before v0.1.0 cut): the substrate
floor itself. We don't add `@code/llvm` shards to v0.1.0 — that's
Phase-D-or-later substrate work. v0.1.0 ships with the 13 shards we
have; the butterfly roadmap names *what they grow into*, not what
they become before they're stable.

---

## 1. What the butterfly is, structurally

**The butterfly is the small complete self-hosted mirror binary whose
codegen contract is closed under the substrate's own algebra.** Not a
metaphor: a structural test.

A binary is the butterfly when **all four** of these hold:

1. **`mirror compile mirror` produces a working `mirror` binary**, and
   that binary, run as `./mirror compile mirror`, produces a binary
   with the **same OID** at altitude `@code/<target>`. The fixed point
   of the codegen pipeline is reached — `au(@code/<target>)`'s `identity`
   stabilizes under self-compile. (Per `shards/mirror/au.mirror`, au's
   identity IS the content-hash of (altitude, content, transparency);
   stabilization is the substrate-level statement.)
2. **No `cargo` is on the invocation path.** The cargo dependency is
   substrate-pull-bounded per `mosaic.md §8` ("v1.0 has no cargo @io
   call"). The butterfly's `settle` calls a codegen tool directly
   (`llc`, `cc`/`ld`) through `@io`, not cargo. `shards/io/cargo.mirror`
   stays in the substrate as the *legacy* @io contract for users who
   still ship cargo workspaces; the butterfly itself does not consume it.
3. **No `rust-mirror` is required in `$PATH`.** The bootstrap Rust
   binary is the chrysalis. Once `mirror compile mirror` produces a
   `au(@code/<target>)` artifact whose `verify` passes its own
   `settle_on { binary.compiles, binary.tests_pass }` block, the
   chrysalis is no longer the build dependency. The bootstrap stays
   in the repo *as documentation of the spectral triple it implements*
   (per `docs/specs/spectral-triple-binary.md`), but `mirror.spec`'s
   default target no longer points at `cargo build`.
4. **The numerical substrate is composed, not embedded.** LAPACK calls
   come in through `LAPACKPrism` (a `prism_core::Prism` impl per
   `docs/specs/architecture-flang-mirror-numerical-split.md`), whose
   numerical backend is **flang-compiled Fortran** consumed through
   `@code/fortran` → `@code/llvm/ir` → linker. The butterfly does not
   carry hand-written LAPACK FFI in the bootstrap. The numerical floor
   is a Prism impl whose body is content-addressed Fortran-compiled-IR,
   resolved through the same codegen path as everything else.

**Substrate restatement (no metaphor):** the butterfly is the
`au(@code/<target>)` value whose `identity` is a fixed point of
`mirror compile mirror`, whose `verify` weighs zero against the
`settle_on` block in `mirror.spec`, and whose `transparency` lists no
residual cargo-or-rustc opacity.

This is a measurable contract. The crystallization test in
`docs/specs/spectral-triple-binary.md` §6 (`spectral loss mirror-native`)
is the operational form: when self-hosting holds (`mirror-native craft
std = same oid`), the crystal has formed.

---

## 2. The @io → machine-code path — three candidates

For each candidate, what's named is: (i) the codegen substrate the
butterfly's `settle @code/<target>` lands on; (ii) the @io contracts
the shift consumes; (iii) the LAPACK seam.

### Path A — llvm-first

Mirror emits LLVM IR directly via a new shard `shards/code/llvm.mirror`
(plus `shards/code/llvm/ir.mirror` for the IR-source-language altitude
that already exists at `boot/std/code/llvm/ir.mirror`). A new
`shards/io/llc.mirror` declares the llc subprocess contract. The
butterfly's `settle @code/llvm` invokes `llc` to lower IR → object
code, then `cc` / `ld` (via a new `shards/io/cc.mirror`) to link to a
final binary.

LAPACK seam: `LAPACKPrism`'s numerical body is Fortran, compiled by
flang into the same `@code/llvm/ir` stream mirror's own IR rides. One
codegen path; two source languages (mirror's emitted IR + flang's
emitted IR) converge at `llc`.

Cost: three new shards (code/llvm, io/llc, io/cc), one new emitter
grammar (the @code/mirror → @code/llvm/ir translate), one big-step
graduation from cargo. The translate template is the long pole — it's
essentially the spec in `docs/specs/spectral-triple-binary.md §3` made
concrete.

Upside: **mirror owns codegen end-to-end** the moment the butterfly
flies. No rustc as an intermediate. The cosmos-mirror-scaffold spec's
claim (mirror's substrate IS the spectral triple) becomes operationally
true at the @io boundary too. The four-transport runtime
(`the-convergence.md`) lives natively, without delegating compilation
to another toolchain.

Downside: highest substrate-pull pressure pre-Phase-7. Phase 4 already
intends `fragmentation` to be generated from `@fragmentation + @code/rust`;
llvm-first means **redirecting that work to @code/llvm before the
@code/rust translate template lands**, or accepting a thrown-away
artifact. The compile staircase loses its first rung.

### Path B — rust-graduating

Mirror keeps emitting Rust (the `@code/rust` translate template, per
Phase 4) for the v0.1.x → v0.5.x window. The chrysalis stays cargo +
rustc. Each phase eliminates more *hand-written* Rust by generating it
from `.mirror`, but the codegen target stays Rust. At a planned
future phase (post-v0.5, pre-v1.0), mirror adds `@code/llvm` as a
**second** codegen target. `mirror.spec` gains a `target butterfly
{ altitude @code/llvm, emit llc }` block; the @code/rust target stays
as the legacy path. Eventually @code/llvm becomes default; @code/rust
becomes opt-in for shipping Rust libraries; the chrysalis sheds at v1.0.

LAPACK seam: identical to Path A, but lazier. Phase 6 lands
LAPACKPrism through a Rust wrapper (today's `prism/core/src/ffi.rs`)
until the @code/llvm graduation; then the wrapper itself becomes a
@code/llvm artifact.

Cost: dual codegen for a defined window. Two translate templates
(@code/rust and @code/llvm) coexist for ~2-3 releases. Phase 4 work
is preserved; Phase 6 NumericalPrism wraps existing FFI.

Upside: **substrate-pull-preserving**. Each existing phase doc keeps
working exactly as written; nothing in Phase 4 / Phase 5 / Phase 6
needs to be retargeted. The chrysalis sheds in one tick at the end
(adding @code/llvm as a target, then changing the default), not in a
big-step graduation halfway through.

Downside: the chrysalis lives longer than the butterfly metaphor
suggests. `cargo` stays on the invocation path through most of the
Phase 5 → Phase 6 window. The criterion-3 test ("no rust-mirror in
$PATH") is not green until late v0.x.

### Path C — parallel altitudes

Both `@code/rust` and `@code/llvm` coexist permanently. `mirror.spec`
chooses per target which altitude the codegen lands at. Users shipping
Rust libraries declare `target lib { altitude @code/rust, emit cargo }`;
users shipping native binaries declare `target binary { altitude
@code/llvm, emit llc }`. The butterfly itself runs the llvm path;
third-party projects can stay on @code/rust forever.

LAPACK seam: lives at @code/fortran; both @code/rust and @code/llvm
consume it through @code/llvm/ir at link time. The numerical substrate
is altitude-agnostic.

Cost: highest *long-term* maintenance burden. Two emit paths, two test
matrices, two release artifacts. The shards floor grows ~6 shards
(code/llvm, code/llvm/ir, io/llc, io/cc, code/fortran, io/flang)
instead of 3-4. The mosaic spec needs §10.5's emit disambiguation to
do more work.

Upside: **maximum substrate flexibility**. Users who want their work
shipped as a Rust crate (real workflow — `cargo publish`, `crates.io`,
cargo-installable binaries) stay supported indefinitely. The butterfly
doesn't break the existing Rust ecosystem for downstream consumers.

Downside: the metaphor doesn't fly clean. Both altitudes carry equal
weight; the chrysalis doesn't shed — it becomes a permanent egress
port. The criterion-3 test is *parametric*: green at @code/llvm,
intentionally not green at @code/rust. The spectral triple binary
spec's claim ("the binary IS the spectral triple") becomes
altitude-dependent.

### The recommendation: **Path B (rust-graduating)**

**Substrate-pull discipline says B.** Three reasons:

1. **The work already named in Phase 4 is right.** "fragmentation
   generated from @fragmentation + @code/rust" is the canonical
   first proof that mirror's compilation pipeline crossed the
   maturity threshold for self-hosted production code
   (`phase-4-emitter-self.md`). Throwing that away to redirect
   Phase 4 at @code/llvm would be substrate-pull-reflex (capability
   in the floor before the floor is ready). The Phase 4 work IS
   what proves we can emit code at all — the right move is to
   *complete it at @code/rust* and *carry it as a graduation* to
   @code/llvm, not bypass it.
2. **The fortran/LAPACK pathway already wants to live at LLVM IR.**
   Per `numerical-substrate-via-fortran.md` §1.2, the numerical
   substrate compiles via flang to LLVM IR and mirror consumes the
   IR through `boot/std/code/llvm/ir.mirror`. **mirror already
   consumes LLVM IR.** It does not yet *emit* LLVM IR. Path B says:
   the consumer pathway lands first (Phase 6 NumericalPrism
   backends + LAPACKPrism); the emitter pathway lands next (the
   graduation to @code/llvm). The two pathways meet at the IR
   altitude. This is the substrate-pull-realize sequence.
3. **The graduation is testable and reversible.** Add @code/llvm as a
   second target. Build the butterfly through both for one release
   cycle. Verify byte-equal `au(@code/<target>)` after structural
   normalization (the two paths produce different bytes for valid
   reasons — different linker output, different ABI; the *behavior*
   under the `settle_on` block is what we compare). Once the @code/llvm
   target passes settle_on, **flip the default in mirror.spec**.
   `@code/rust` does not get deleted — it becomes legacy. The cargo
   shard stays; users who want it use it. The default chrysalis sheds.

The @code/llvm graduation is not deferred to v1.0 with hand-waving —
it's a named tick (B.4 below) with a concrete cut criterion. The
metaphor flies clean: the chrysalis (cargo + rustc) is real; the
butterfly (mirror + llc) is real; the chrysalis sheds at a named
moment; the @code/rust contract stays in the substrate as honored
legacy, not deleted.

**One named risk this path carries.** If @code/llvm graduation slips
past v1.0, the cloud deployment ships on cargo. That's correctness-
preserving (the existing path works) but it's a failure of nerve at
the metaphor altitude. Mitigation: name the graduation as the v0.9 →
v1.0 cut criterion in `roadmap/wip/v1-launch.md` so it can't quietly
slip.

---

## 3. The LAPACK integration shape

The `architecture-flang-mirror-numerical-split.md` spec settles the
*where*: A (algebra) lives in mirror; D (Dirac operator) lives in
flang-compiled Fortran; H (Hilbert space — the eigenvector/eigenvalue
records) is the data `au` settles onto.

The `numerical-substrate-via-fortran.md` spec settles the *how*: a new
`@code/fortran` grammar; flang as the compiler; flang's output IR
consumed through mirror's existing `boot/std/code/llvm/ir.mirror`;
`fpm` (Fortran Package Manager) as the package surface for the
standalone Fortran Fate package.

**The butterfly inherits this with one substrate-pull commitment:**

**LAPACK lives at the substrate, not at @io.** Not `@io.lapack`.
The Fortran-source-language altitude is `@code/fortran` (a new
shard `shards/code/fortran.mirror` lands in Phase 6); flang's
invocation contract is `@io.flang` (a new shard
`shards/io/flang.mirror`). The compiled `liblapack.a` is **content-
addressed in the @mirror/store** as `au(@code/fortran)`; the linker
resolves it against the butterfly's `settle` step.

This is the structural answer to "where does LAPACK live": it lives
at `@code/fortran` as substrate truth, the flang subprocess lives at
`@io.flang` as the @io contract, and the linked artifact is an
`au(@code/fortran)` value that the butterfly's `settle @code/llvm`
resolves against. **No FFI in the bootstrap.** The current
`prism/core/src/ffi.rs` LAPACK wrapper is *boundary Rust* (per the
flang-mirror-numerical-split spec's footer); it sheds when the
@code/fortran shard lands.

**How mirror's 5×5 fiber composes with flang's 16×16 inference.** Per
the split spec: the 16→5 shift IS the spectral action / SSB. The
`shift(fiber_state) -> base_state` action's body is parked (`\`) and
resolved through Fate's tournament. **The butterfly does not implement
this shift body**; it implements the *types* (the typed surface that
makes the shift composable) and the *codegen* (the flang invocation
that produces `au(@code/fortran)`). The shift body's resolution is
Fate-tournament work, lands at Phase 5 (the reflection scheduler
phase), and is consumed by the butterfly at Phase 6/7.

**When does this need to be done by — Phase 6, or earlier?** The
LAPACK-at-substrate move IS Phase 6. The flang grammar lands at Phase
6 (it's the natural home — Phase 6's headline is the NumericalPrism
backends, and the Fortran pathway is the substrate of the LapackBackend
as-named). The current Phase 6 doc names LapackBackend / MetalBackend /
OpenCLBackend as *Rust impl detail*; this roadmap reframes them as
*altitudes consumed by LAPACKPrism*. The amendment is small (one
rewrite of Phase 6 task 4); the substrate impact is structural.

---

## 4. The compile staircase — each rung named

Per `project-mirror-compile-staircase` (memory): each rung removes one
Rust dependency. Below: every rung the butterfly traverses, with
citation to the existing phase doc that owns the rung, and the one
test that says the rung was climbed.

| Rung | Phase | What's eliminated | What's added | Cut test |
|------|-------|-------------------|--------------|----------|
| **0** | Phase 0 (here) | — | — | `bootstrap/src/` works; 1,362 tests green; 76% coverage. Substrate floor: 13 shards. |
| **1** | Phase 1 | `!=` tokenization missing; singularity types missing | `boot/std/mirror/glass/ast/token.mirror` consumed by bootstrap (LANDED 2026-05-26) | `mirror compile boot/` produces zero holonomy |
| **2** | Phase 2 | `bootstrap/src/tokenize.rs`, `bootstrap/src/grammar.rs` (dispatch logic) | `@mirror/syntax`, `@mirror/keyword` (self-teaching) | `@mirror/syntax` parses `@mirror/syntax` |
| **3** | Phase 3 | `bootstrap/src/resolve` (already gone post-collapse; this rung is the *grammar* version) | `@mirror/resolve` grammar; type-registry as `.mirror` type | `@mirror/resolve` resolves `@mirror/resolve` |
| **4** | Phase 4 | `fragmentation/src/` (hand-written Rust) | `@fragmentation + @code/rust` translate template; `@code/mirror` render template; `@shatter/format` | `@code/mirror` renders itself round-trip; generated fragmentation passes all fragmentation tests |
| **5** | Phase 5 | `\` hole as flag-only; pipeline composition as syntax-only | `\` dispatch via `@fate.infer`; `|\>` (pipe-with-hole); kintsugi as Reflection; Scheduler Tower demand contract; runtime dispatch via `@mirror/serve` | Reflection observes a full tick; loss decreases monotonically across ticks (eⁿ⁺¹ ≤ eⁿ); kintsugi tournament converges |
| **6** | Phase 6 | `prism/core/src/ffi.rs` (hand-written LAPACK wrapper); per-file Rust audit moves non-@io to .mirror | `shards/code/fortran.mirror`; `shards/io/flang.mirror`; LAPACKPrism (per pq §6.5); MetalBackend; OpenCLBackend; the @io inventory closes | `au(@code/fortran)` produces a content-addressed `liblapack.a` resolvable by mirror's settle |
| **7** | Phase 7 (the butterfly graduation) | `bootstrap/src/main.rs` as default codegen entry; cargo on the invocation path | `shards/code/llvm.mirror`; `shards/io/llc.mirror`; `shards/io/cc.mirror`; `@code/mirror → @code/llvm/ir` translate template; the @code/llvm target in `mirror.spec` | `mirror compile mirror` produces `au(@code/llvm)` whose `identity` is a fixed point under self-compile; no rustc, no cargo on the invocation path |

**The compile staircase is the gating spec for Phases 2-7.** Each
rung names what comes out from under the Rust pile. The butterfly
emerges *as* the last rung — not as a separate artifact at the top of
the staircase, but as the recognition that the staircase has been
climbed.

### Reconciled drift between this roadmap and existing phase docs

**One contradiction surfaces, one ambiguity surfaces.**

- **Contradiction (small):** `phase-6-io-numerical-prism.md` names
  LapackBackend / MetalBackend / OpenCLBackend as **Rust impl**
  ("LapackBackend (already exists in `prism/core/src/ffi.rs`; wrap
  into the operation-based API)"). This roadmap names them as
  **altitudes consumed by LAPACKPrism through @code/fortran +
  flang**. The Phase 6 framing is `[substrate-pull:reflex]`: the FFI
  goes into capability because the substrate "can't express LAPACK".
  But per `numerical-substrate-via-fortran.md`, the substrate CAN
  express LAPACK — through `@code/fortran`. **Phase 6's task 4 needs
  rewriting**: the LapackBackend's *implementation* is
  `au(@code/fortran)` content-addressed in @mirror/store, not
  `prism/core/src/ffi.rs`. The Rust ffi stays for the bootstrap
  window; it sheds at Phase 6 close.

- **Ambiguity (resolvable):** `phase-7-self-hosted-deployed.md`
  names the Phase 7 deliverable as "mirror compiles mirror" and
  "`spectral.engineer` stood up". But it does not name *what codegen
  target* the self-host uses. With Path B chosen, Phase 7 *also*
  performs the @code/rust → @code/llvm graduation. This is a Phase 7
  task addition: T7.4 "land @code/llvm as default codegen; @code/rust
  becomes legacy". The cut criterion in §6 below is the test.

**No other drift.** Phase 5's Scheduler Tower work, Phase 4's
fragmentation-as-generated proof, Phase 3's resolver self-description,
Phase 2's parser self-description — all carry through unchanged. The
butterfly does not destabilize them; it *names the final form they
assemble into*.

---

## 5. Cut criteria — "the butterfly has emerged"

Four criteria, *all* must hold. Each is a measurable contract over
`au(@code/llvm)`, not a vibe.

### C1 — Self-compile fixed point at @code/llvm

```
$ mirror compile mirror --target butterfly      # uses chrysalis (cargo)
$ ./mirror-from-cargo compile mirror --target butterfly
$ ./mirror-from-cargo-then-mirror compile mirror --target butterfly
```

Let the three resulting binaries have `au(@code/llvm)` identities
`a₀`, `a₁`, `a₂`. **Cut test:** after structural normalization
(stripping linker timestamps and embedded build paths, both of which
are flagged as residual transparency in the `imperfect` carrier),
`identity(a₁) == identity(a₂)`. The fixed point is reached one
self-compile after the chrysalis-built binary.

*This is the strong form.* The weak form (`mirror compile mirror`
produces *a* working binary) was already implicit in Phase 7; the
strong form (fixed point of identity at @code/llvm) is what the
butterfly contract adds.

### C2 — Cargo-less invocation

`which cargo` returns empty, or `cargo` is renamed out of `$PATH`.
Run `./mirror compile mirror --target butterfly`. Build succeeds.
Result: `au(@code/llvm)` with `verify` returning `pass` against the
butterfly target's `settle_on { binary.compiles, binary.tests_pass }`.

*Why this matters:* C2 makes the chrysalis-shed real. "cargo not on
the invocation path" is not a stylistic preference — it's a substrate-
level statement that the codegen contract closes under `@io.llc` +
`@io.cc`, with no `@io.cargo` consumer.

### C3 — No `rust-mirror` in `$PATH`

Delete the bootstrap binary. Delete `bootstrap/target/`. Run `./mirror
compile mirror --target butterfly`. Build succeeds. The butterfly
rebuilds itself from its own bytes, no chrysalis present.

*Subtlety:* C3 holds for the *binary*, not for the *source-of-truth*.
The bootstrap Rust source can stay in `bootstrap/src/` as
documentation of the spectral triple it implements; what's tested is
that the *binary* is not required.

### C4 — Numerical substrate composed, not embedded

`nm ./mirror | grep -i lapack` returns symbols imported from
`au(@code/fortran)` content-addressed in `.mirror/store`, NOT from a
statically-linked `prism/core/src/ffi.rs` artifact. The LAPACK seam
is a @code/fortran reference resolved at link time.

*Why this matters:* C4 is the proof that the LAPACK substrate-pull
resolved. Until C4 holds, mirror still ships LAPACK as "capability
in the floor" (a Rust crate dep); after C4, it ships LAPACK as
"substrate content" (@code/fortran artifacts in the store).

**Aggregate cut test:** `spectral loss mirror-native` (per spectral-
triple-binary.md §6) reports `self-hosting: yes` AND `crystal-stable:
yes` AND `lapack-substrate-resident: yes`. Three lines, three
conditions, one verdict.

---

## 6. Relationship to other tracks

### Track A — NumericalPrism backends (Lapack → Metal → OpenCL)

Currently named in Phase 6 as three Rust backends. The butterfly
reframes: **all three are altitudes consumed by `LAPACKPrism` (the
name is now a misnomer — it's the *numerical Prism*, with Lapack as
the CPU altitude). Each backend is an `au(@code/<lang>)` artifact:**

- LapackBackend → `au(@code/fortran)` via flang. Substrate-resident.
- MetalBackend → `au(@code/metal)` via Apple's metal compiler. A new
  shard `shards/code/metal.mirror` + `shards/io/metal.mirror` lands
  at Phase 6 close. Apple-Silicon-bound; not deployed on the cloud.
- OpenCLBackend → `au(@code/opencl)` via the cross-vendor OpenCL
  compiler. **Cloud-deployment non-optional** per
  `roadmap/wip/v1-launch.md`. Anna Jakobs's 2012 thesis (already
  cited) is the architectural prior art for the shared-memory
  pattern OpenCL implements.

The butterfly's `settle @code/llvm` does NOT know which backend the
LAPACKPrism dispatches to at runtime; the dispatch is `SpectralSupervisor`'s
job (Track K below). At link time, the butterfly carries references
to all three altitudes' `au` values; at run time, the supervisor
picks the altitude per the spectral signature of the work.

### Track K — Runtime elevation (HamiltonScheduler + SpectralSupervisor)

Per `roadmap/pending/runtime-elevation.md`: HamiltonScheduler lives in
fragmentation (per-shard, agent's content window manager); SpectralSupervisor
lives in @spectral/db (cross-shard, the closed-engine moat).

**The butterfly hosts both.** Once the butterfly flies:

- HamiltonScheduler runs in the butterfly's own runtime, per shard.
  The bounded-WCET / priority-discipline guarantee is at the butterfly's
  altitude — it's `au(@code/llvm)` code, not bolt-on. The Margaret
  Hamilton lineage is *cited in the substrate* (the @kintsugi/credo
  shard, when it lands, will name her at the dispatch surface), not
  just in a Rust doc-comment.
- SpectralSupervisor sits on top, as a separate `au(@code/llvm)`
  artifact that ships from `@spectral/db`. It is the *coordinator*
  the butterfly *defers to* — the butterfly does not contain the
  closed engine; the closed engine consumes the butterfly's spectral
  signature.

The Body restructure (`Body<H> = { prism, glass, ast }`) is *required*
for the butterfly to fly. Without it, bodies are Rust closures and
@code/llvm cannot emit them — closures-as-bytecode is not at @code/llvm's
altitude. With it, bodies are AST + prism dispatch + glass projection,
all content-addressed, all emit-able through the standard pipeline.

**Sequencing:** Body restructure lands at Phase 5 close (per the
run­time-elevation doc's sequence). The butterfly graduation at
Phase 7 *consumes* it. Track K is a prerequisite, not parallel work.

### The convergence specs (the-convergence, cli-as-prism, lambda-shell)

Per `docs/specs/the-convergence.md`: one runtime, four transports, five
operations. The four transports are λsh / mirror-CLI / MCP / LSP. The
butterfly **is** the runtime they share.

Concretely:

- The daemon at `~/.spectral/serve.sock` runs as a butterfly process.
  Single binary, no cargo on PATH at runtime. The four transports'
  protocol heads (`McpActor`, `LspActor`, `ShellActor`, `CliActor`) are
  glasses on `prism @mirror/cli` (per `cli-as-prism.md` §1.2): each is
  `au(@code/llvm)` code emitted from `shards/mirror/cli/<x>.mirror`.
- The eigenboard (the algedonic surface) lives at the butterfly's
  altitude. The cybernetic frame (`the-convergence.md` §0) is
  *implemented*, not metaphorically gestured at.
- The mq expression language (`lambda-shell.md`) is interpreted by the
  butterfly's own runtime — same parser, same dispatch, same five
  operations. No second interpreter.

If the butterfly flies, the convergence specs become *implementation
specs* automatically. If the butterfly doesn't fly, the convergence
specs stay aspirational because the runtime is still cargo + rustc
and cannot host the daemon as-described.

---

## 7. What this roadmap does NOT propose

Explicit scope-exclusion. The butterfly is post-v0.1.0; nothing here
blocks the v0.1.0 cut.

- **Does not gate v0.1.0.** T11.7 (cut `v0.1.0`) ships now with the
  chrysalis intact. v0.1.0 is the substrate-floor release; the
  butterfly is the codegen-floor recognition for the *next* sweep of
  work.
- **Does not require new shards for v0.1.0.** The 13 shards on
  `shards/` floor are stable. The butterfly's new shards
  (`code/llvm`, `code/fortran`, `code/metal`, `code/opencl`, `io/llc`,
  `io/cc`, `io/flang`, etc.) land at Phase 6 / Phase 7, not now.
- **Does not redesign Phase 4.** Phase 4 stays as written: emit-self,
  fragmentation-as-generated through `@code/rust`. Path B preserves
  this. The graduation to @code/llvm is a Phase 7 addition, not a
  Phase 4 rewrite.
- **Does not write Rust.** Per substrate-pull discipline: the
  butterfly's *substrate* is `.mirror`; its *runtime* is
  `au(@code/llvm)`. The bootstrap Rust shrinks over phases 2-7; this
  roadmap does not propose growing it.
- **Does not introduce remote inference.** `@fate` refuses remote
  inference, mathematically. The butterfly inherits that constraint.
- **Does not propose a build-engine abstraction.** Per
  [[architecture-prism-as-trait-as-everything]], `prism` IS the
  algebra; there is no separate "build engine". The butterfly's
  build IS `settle @code/llvm` — the mosaic prism's five operations
  on the project manifold.

---

## 8. Open questions (for Reed + Alex)

Five. Each is genuinely unresolved; each has a forcing function.

1. **Which @io contracts go to `llc` vs `lld` vs `cc`?** Path B's
   final form invokes `llc` (IR → object) then a linker. The linker
   choice (cc / clang / lld / ld) is platform-sensitive and affects
   reproducibility. Default candidate: `lld` (LLVM's linker) for
   substrate consistency, with `@io.cc` as a fallback for platforms
   where lld is not present. Decision: do we add `shards/io/lld.mirror`
   *and* `shards/io/cc.mirror`, with `mirror.spec` choosing per
   target? Or single linker, hardcoded? Lean answer: **commit to lld
   for the butterfly's own build; cc as fallback only**.

2. **Is `@code/llvm` THE altitude, or is `@code/llvm/ir` the altitude?**
   The existing `boot/std/code/llvm/ir.mirror` declares the IR-as-
   source-language altitude (the *consumer* pathway). The butterfly
   needs an *emitter* altitude. Is that `@code/llvm` (the parent) or
   `@code/llvm/emit` or something else? Per the path-namespace
   property (shards/path declares one altitude), each path declares
   one altitude. **Likely answer:** `shards/code/llvm.mirror` declares
   `@code/llvm` as the umbrella; the sub-prisms `@code/llvm/ir` (for
   the consumer) and `@code/llvm/emit` (for the emitter) live as
   path-namespaced sub-shards, like `@code/rust/cargo` does today.

3. **Does the butterfly carry rustc as a build-time-only `@io` for
   the @code/rust target's legacy support, or does the butterfly stop
   shipping @code/rust support entirely?** The two answers fork the
   ecosystem: (a) butterfly is a multi-altitude emitter that includes
   @code/rust for downstream Rust consumers; (b) butterfly emits
   only @code/llvm; @code/rust support stays in the chrysalis
   distribution. **Recommendation lean:** (a) — preserve @code/rust
   as a real emit target so users can ship Rust crates. The cargo
   shard stays; users who want it use it. The default chrysalis
   sheds; the *option* doesn't.

4. **At what release does the chrysalis officially shed?** v0.x range
   is open. Naming a target version forces sequencing. Candidates:
   v0.5 (early — high risk), v0.9 (cut criterion for v1.0 — current
   recommendation), v1.0 itself (latest — guarantees cloud ships on
   butterfly but compresses Phase 6/7 work). **Recommendation lean:**
   v0.9 as the cut criterion gate for v1.0, with v0.5-v0.8 as the
   parallel-emit window (both @code/rust and @code/llvm tested every
   release).

5. **The `cosmos-mirror-scaffold.md` spec names mirror's substrate AS
   the spectral triple at cosmic scale. Does the butterfly's codegen
   contract have implications for the cosmos work?** Specifically: if
   the butterfly's `au(@code/llvm)` is a measurable fixed point of
   self-compile, does the cosmos work want to consume the butterfly's
   spectral signature at runtime (the "one spectrum, many physics"
   claim becomes operational at the codegen altitude too)? Not
   blocking, but worth checking before Phase 7 closes.

---

## 9. Sequencing within this roadmap

```
  v0.1.0 cut (T11.7)        — chrysalis intact; substrate floor stable
        ↓
  Phase 1 → Phase 2 → Phase 3 → Phase 4
        |        |        |        |
        |        |        |        +--- fragmentation-as-generated via @code/rust
        |        |        |             (Path B: @code/rust is the first emitter)
        |        |        +--- @mirror/resolve grammar
        |        +--- @mirror/syntax grammar
        +--- @mirror/glass/ast/token consumed (LANDED)
        ↓
  Phase 5 — Reflection + Scheduler Tower + \ dispatch via @fate
           — Body<H> = { prism, glass, ast } restructure
           — kintsugi as Reflection
        ↓
  Phase 6 — @io inventory closes (per-file audit)
          — shards/code/fortran.mirror; shards/io/flang.mirror
          — LAPACKPrism via @code/fortran (substrate-resident)
          — MetalBackend (Apple Silicon)
          — OpenCLBackend (cloud, Anna Jakobs pattern)
        ↓
  Phase 7 (the butterfly graduation)
          — shards/code/llvm.mirror; shards/io/llc.mirror; shards/io/lld.mirror
          — @code/mirror → @code/llvm/emit translate template
          — second target in mirror.spec: butterfly { altitude @code/llvm }
          — parallel-emit window: build through both targets every release
          — C1-C4 cut tests; flip default in mirror.spec
          — spectral.engineer cloud deployment runs on butterfly
```

**The graduation moment is one tick.** Adding `@code/llvm` as a second
target is the substrate-level move; flipping the default in
`mirror.spec` is the user-visible cut. Each is a small commit; the
work that surrounds them (Phase 6/7's broader scope) is what makes
them possible. The metaphor flies clean because the substrate-pull
does.

---

## 10. Footer

*The chrysalis (cargo + rustc) is real. The butterfly
(`au(@code/llvm)` as a fixed point of self-compile) is real. The
graduation has a date (v0.9 → v1.0 gate). The substrate has the
vocabulary already (`@code/llvm`, `@code/fortran`, `LAPACKPrism`,
`au(altitude)`). What's missing is the commitment that this is the
path. This document is the commitment.*

*Maximum mirror, minimal Rust. The chrysalis sheds at v0.9.*
*Apache-2.0 (this doc and the open foundation it describes).*
