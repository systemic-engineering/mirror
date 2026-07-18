# Taut Scout — Fate → prismqueer::fate Migration Mapping

**Date:** 2026-07-18
**Actor:** Taut (Pack scout, grep-first, read-only)
**Trigger:** Alex 2026-07-18 in-transcript directive: "I think fate already
depends on prismqueer. Let's pull fate into prismqueer::fate."
**Confirmed by Reed:** `fate/Cargo.toml:14` — `prism = { package =
"prismqueer", path = "../prism/prismqueer", features = ["bundle"] }`. Direction
= `fate → prismqueer`. Pull-in requires source-move + circular-break.
**Scope:** Read-only mapping. NO source edits. Reed executes after Alex
ratifies §12.

---

## §0 — Executive summary

- **Total files moving:** 16 (13 source, 1 build.rs, 2 asset dirs
  = `brainfuck/` 3 files + `training/` 1 file + 1 planning doc).
- **Total consumer edits (LIVE):** 3 Rust source files in `mirror/bootstrap/`
  (55 fully-qualified `fate::` references) + 2 Rust source files in
  `spectral/src/sel/` (21 refs incl. one `use fate::{...}` line) + 4 Cargo.toml
  edges (`mirror/bootstrap/Cargo.toml`, `spectral/Cargo.toml`,
  `spectral-db/Cargo.toml`, `cosmos/Cargo.toml`, `mirror-new/Cargo.toml`).
- **Total consumer edits (STALE / kept-working via shim):** 11 files across
  `cosmos/`, `mirror-new/`, `spectral-db/` — all three projects are archived
  (mirror-new + cosmos still call the old `Pathfinder` variant which fate has
  since renamed to `Introject`; spectral-db has no June/July 2026 commits).
  These stay compiling untouched via a `fate/` compatibility shim that
  re-exports `prismqueer::fate::*`. NO edits to their sources.
- **Reed lean for build.rs:** **(B)** — move to
  `prismqueer/build_scripts/fate.rs`, feature-gate under
  `fate-brainfuck` sub-feature. Fate's build.rs generates load-bearing
  `bf_compiled.rs` (via `include!(concat!(env!("OUT_DIR"), "/bf_compiled.rs"))`
  in `fate/src/compiled.rs:3`) AND `fate.metal` for the `metal` feature — both
  are runtime-live outputs; (C) drop-brainfuck is unsafe.
- **Feature naming (delightfully-boring):** `fate` (bare) at
  `prismqueer/Cargo.toml`, gating `pub mod fate;`. Sub-features:
  `fate-training`, `fate-metal`, `fate-brainfuck`, `fate-lapack` (composes
  with prismqueer's existing `lapack`).
- **Circular dep is already broken by construction** — fate depends on
  prismqueer + zero circulars back to itself; the "circular" concern in Alex's
  question was speculative. The move is a **flat consolidation**, not a
  cycle-break.
- **Reed can execute in one commit** with the shim. Est. LOC delta ≈
  +2,600 (fate source in) / -0 (shim keeps old path alive) / +100 Cargo.toml
  surgery / +55+21 mechanical `fate::` → `prismqueer::fate::` rewrites (or
  simpler: `use prismqueer::fate;` header + zero-touch bodies). Total commit
  ≈ +2,700 lines net.
- **Mara mint needed?** **NO.** Mechanical refactor; no new substrate species,
  no canonical-spec surface change. Fate's `pub` API stays byte-identical.
- **What surprised me:** fate's build.rs also emits a **Metal Shading
  Language kernel** (`fate.metal`) — not just the Rust `bf_compiled.rs`. So
  the build.rs migration carries GPU shader emission along with BF compilation.
  This is why (C) drop-brainfuck fails: the `metal` feature at
  `metal_runtime.rs:13` does `include_str!(concat!(env!("OUT_DIR"),
  "/fate.metal"))` — deleting the BF compiler kills GPU acceleration silently.

---

## §1 — Full inventory of `fate/` subtree

Location: `/Users/alexwolf/dev/projects/fate/`. Ignoring `target/` and `.git/`.

### 1.1 Source files (`src/*.rs` — 10 modules)

| File | LOC | Purpose | Public surface |
|------|----:|---------|-----------------|
| `src/lib.rs` | 1,459 | `Fate`, `Model`, `Features`, `FEATURE_DIM`, `Decision`, `FateOutput`, `ModelWeights`, `Pipeline` trait + impl. Fate's `Prism`/`Fiber`/`Connection`/`Gauge`/`Transport`/`Closure` impls. All 5 `impl prism::*` blocks. | `pub enum Model`, `pub const FEATURE_DIM`, `pub type Features`, `pub struct Decision/FateOutput/ModelWeights/Fate`, `pub trait Pipeline`, `pub fn manifold_observation_loss` |
| `src/compiled.rs` | 3 | Includes build.rs output. | `include!(concat!(env!("OUT_DIR"), "/bf_compiled.rs"))` — brings in `pub fn fate_bf`. |
| `src/derive.rs` | 630 | Weight derivation via eigendecomp of dark coupling. Uses `prism::Loss`. | `pub fn extract_dark_coupling`, `pub struct DarkEigen`, `pub fn dark_eigensystem`, `pub fn crystallize`. |
| `src/feature.rs` | 191 | Named feature dims (TEMPORAL, PROCESSING, ..., EMOTIONAL_TONE), CASIMIR_EIGENVALUES, HolonomyHealth. | `pub const TEMPORAL/PROCESSING/…`, `pub const ACTIVE/DARK`, `pub enum HolonomyHealth`, `pub fn holonomy_health`, `pub const BERRY_PHASE`. |
| `src/manifold.rs` | 210 | `ManifoldState` (16×16 f64), `ManifoldLoss`. Implements `prism::Loss`. | `pub type ManifoldState`, `pub fn manifold_zero/identity/diagonal`, `pub struct ManifoldLoss`. |
| `src/metal_runtime.rs` | 233 | `#[cfg(feature = "metal")]` — Metal GPU backend. Uses `metal` + `objc` crates. Compiles fate.metal at startup, dispatches N inference instances via GPU. | `pub struct MetalRuntime`, `pub fn new/dispatch`. |
| `src/runtime.rs` | 619 | BF interpreter (`FateRuntime`), `CompiledFateRuntime` (uses compiled.rs), `UniversalRuntime` (weights-passed). | `pub struct FateRuntime/CompiledFateRuntime/UniversalRuntime` + `pub fn select` on each. |
| `src/strategy.rs` | 63 | `Strategy` enum + `prism::GroupStructure` impl (cyclic Z/5). | `pub enum Strategy`. |
| `src/train.rs` | 264 | `#[cfg(feature = "training")]` — gradient-descent training + quantization. Uses `serde` + `serde_json`. | `pub struct Example/TrainConfig/PipelineConfig/F64Weights`, `pub fn load_examples/pipeline`. |
| `src/weights.rs` | 213 | `WeightSet`, `Weights`, `default_cycle()`. | `pub struct WeightSet/Weights` + `pub fn default_cycle/to_bytes`. |

**Total source LOC:** ≈ 3,885 lines. Test-code (in `#[cfg(test)] mod tests`)
represents ~40% of `src/lib.rs` alone (~600 LOC of tests, ~860 LOC of
production).

### 1.2 Non-source assets

- `Cargo.toml` (1.1KB) — dep on prismqueer via `path = "../prism/prismqueer"`.
- `Cargo.lock` (12KB) — standalone workspace; regenerated after merge.
- `build.rs` (13.3KB) — BF-to-Rust IR compiler + BF-to-Metal (MSL)
  compiler. Reads `brainfuck/*.bf`, emits `$OUT_DIR/bf_compiled.rs` +
  `$OUT_DIR/fate.metal`. Rebuilds on `brainfuck/**` change.
- `brainfuck/fate.bf` (1.2KB) — **THE algorithm.** The Kolmogorov atom, ~816
  chars of BF. Compiled at build-time; consumed at runtime.
- `brainfuck/fate.mirror-bf` (5.9KB) — human-annotated variant / design doc
  (contains Unicode-glyph pretty variant `ƑAΤE in Brainƒuck`). NOT compiled
  by build.rs (build.rs filters `*.bf` ext only).
- `brainfuck/test_fate.rs` (7.5KB) — standalone test harness with its own
  minimal BF interpreter (`rustc test_fate.rs -o test_fate && ./test_fate`).
  Independent of `fate` crate.
- `docs/superpowers/plans/2026-04-05-training-pipeline.md` (22.8KB) —
  design doc for the training pipeline. Historical/reference.
- `training/examples.json` (787B) — 10 hand-labeled training examples.
- `tests/pipeline.rs` (3.9KB) — `#![cfg(feature = "training")]` end-to-end
  training round-trip test.
- `tests/pipeline_integration.rs` (7.4KB) — 6 integration tests for
  `Pipeline` trait + trained-fate behaviour.
- `benches/tournament.rs` (6.8KB) — divan benches: tournament, BF
  interpreted vs compiled, resolve depth, excited, eigensystem.

### 1.3 `Cargo.toml` full text (`fate/Cargo.toml:1-38`)

```toml
[package]
name = "fate"
version = "0.1.0"
edition = "2021"
description = "Abyss | Introject | Cartographer | Explorer | Fate. Five models. One selector. Zero dependencies."

[dependencies]
prism = { package = "prismqueer", path = "../prism/prismqueer", features = ["bundle"] }
serde = { version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }

[features]
default = []
training = ["dep:serde", "dep:serde_json"]
lapack = ["prism/lapack"]
metal = ["dep:metal", "dep:objc"]

[dependencies.metal]
version = "0.29"
optional = true

[dependencies.objc]
version = "0.2"
optional = true

[dev-dependencies]
divan = "0.1"

[[bench]]
name = "tournament"
harness = false
```

### 1.4 `build.rs` — what it produces

Two outputs. Both live-consumed by `fate/src/`:

1. **`$OUT_DIR/bf_compiled.rs`** — always emitted. Consumed at
   `fate/src/compiled.rs:3`:
   ```rust
   include!(concat!(env!("OUT_DIR"), "/bf_compiled.rs"));
   ```
   Contains `pub fn fate_bf(input: &[u8]) -> Vec<u8>` — the compiled
   Rust translation of `brainfuck/fate.bf`. Used by `CompiledFateRuntime`
   in `runtime.rs`.

2. **`$OUT_DIR/fate.metal`** — always emitted, only consumed under
   `#[cfg(feature = "metal")]`. At `metal_runtime.rs:13`:
   ```rust
   const FATE_METAL_SRC: &str = include_str!(concat!(env!("OUT_DIR"), "/fate.metal"));
   ```
   MSL kernel dispatched by `MetalRuntime::new()`.

**Rebuild triggers:** `cargo:rerun-if-changed={path.display}` for each `.bf`,
plus `cargo:rerun-if-changed=brainfuck/` for the dir. No env-var reads.

**`build.rs` runs on every fate build** regardless of feature — because
`bf_compiled.rs` is always included. This is important for the (B) proposal:
the fate build.rs cannot be gated behind a Cargo feature at the build-script
level; it must run whenever `pub mod fate;` compiles. The (B) sub-file
`prismqueer/build_scripts/fate.rs` dispatched from a top-level `build.rs`
handles this cleanly (top-level `build.rs` runs unconditionally; it internally
`#[cfg(feature = "fate")]`-checks and calls the fate-build subroutine).

---

## §2 — Consumer grep across known-parallel projects

### 2.1 Cargo.toml dep declarations (`fate = { path = ... }` OR `dep:fate`)

| File | Line | Declaration | Status |
|------|------|-------------|--------|
| `mirror/bootstrap/Cargo.toml` | 127-129 | `[dependencies.fate] path = "../../fate"` | **LIVE** |
| `spectral/Cargo.toml` | 32, 50 | `fate = { path = "../fate", optional = true }` gated on feature `sel` | **LIVE** (behind `sel` feature) |
| `spectral-db/Cargo.toml` | 13 | `fate = { path = "../fate" }` | STALE (last commit June 2026 on lazy-store; scheduler.rs / types.rs / etc. haven't been touched since May 2026) |
| `cosmos/Cargo.toml` | 10 | `fate = { path = "../fate" }` | STALE (April 2026, uses dead `Pathfinder` variant) |
| `mirror-new/Cargo.toml` | 31 | `fate = { path = "../fate" }` | STALE (April 2026, uses dead `Pathfinder` variant) |

Note: `mirror/rust/Cargo.toml` does **NOT** currently declare a `fate` dep in
`[dev-dependencies]` (lines 51-63 declare `prismqueer` + `terni` + `tempfile`
only). Reed's fate-bridge scaffold at `rust/src/main.rs:92` mentions Fate only
in a help-text string — no `use fate` import, no code path.

### 2.2 Source-level `use fate::…` imports

Only **one** file across all searched projects contains a `use fate::…` line:

- `spectral/src/sel/fate_actor.rs:5` — `use fate::{Fate, FateOutput, Features};`
- `spectral/src/sel/fate_actor.rs:101` — `use fate::FEATURE_DIM;` (inside a
  `#[cfg(test)]` block)

**All other consumer files use fully-qualified `fate::X` paths inline** rather
than `use fate::…` at the top. This makes the rewrite trivial: one option is
to preserve the `fate::` alias by adding `use prismqueer::fate;` at the top of
each consumer, zero-touching the FQ call sites.

### 2.3 Inline `fate::X` references — total hits per LIVE consumer

| File | `fate::` hit count | Notes |
|------|-------------------:|-------|
| `mirror/bootstrap/src/lib.rs` | 31 | `fate::Fate::untrained()`, `fate::Fate::excited()`, `fate::Features`, `fate::Model::{Abyss,Introject,Cartographer,Explorer,Fate}`, `fate::ModelWeights`, `fate::Decision`, `fate::FEATURE_DIM` |
| `mirror/bootstrap/src/contribute.rs` | 14 | Similar surface (Fate, Features, Model variants). |
| `mirror/bootstrap/src/algedonic.rs` | 1 | **Comment only** — `//! toward pleasure attractors via Rayleigh descent along Fate::bounded.` — no source dep. |
| `mirror/bootstrap/src/mcp.rs` | 1 | **JSON docstring only** — inside an `inputSchema` description string: `"beam: ... Fires @fate::select on Shape B features..."`. No Rust import. |
| `mirror/bootstrap/src/music/mod.rs` | 1 | Comment/doc only (`@fate` mention). |
| `spectral/src/sel/fate_actor.rs` | 3 | Real Rust import + FQ paths. |
| `spectral/src/sel/mcp/server.rs` | 18 | Real Rust FQ paths (`fate::Model::…` conversions to `ProposalType`). |
| `spectral/src/sel/mcp/memory.rs` | 3 | Comment/doc only. |
| `spectral/src/sel/mcp/query.rs` | 3 | Comment/doc only. |
| `spectral/src/sel/mcp/tools.rs` | 1 | Comment/doc only. |
| `spectral/src/sel/mcp/integration_tests.rs` | 2 | Comment/doc only. |
| `spectral/src/sel/mcp/spawn.rs` | 3 | Comment/doc only. |
| `fragmentation/src/hamilton_scheduler.rs` | 4 | Comment/doc only — NOT a Cargo dep. |

**LIVE-consumer rewrite surface = 5 files across 2 projects (bootstrap +
spectral), ~66 total `fate::X` call sites.**

### 2.4 Inline `fate::X` references — STALE consumers (kept working via shim)

| File | Hits | Model variant used |
|------|-----:|--------------------|
| `cosmos/src/abyss.rs` | 10 | `Pathfinder` (dead — fate now uses `Introject`) |
| `cosmos/src/bin/simulate.rs` | 3 | `Pathfinder` |
| `mirror-new/src/ai.rs` | 9 | `Pathfinder` |
| `mirror-new/src/fate_bridge.rs` | 2 | (uses Features only) |
| `mirror-new/src/cli.rs` | 3 | ? |
| `mirror-new/src/mirror_ast.rs` | 1 | ? |
| `spectral-db/src/scheduler.rs` | 29 | `Introject` (post-rename — was actively maintained in May 2026) |
| `spectral-db/src/observation.rs` | 1 | Uses `Features` only |
| `spectral-db/src/strategy.rs` | 1 | Uses `Model` |
| `spectral-db/src/types.rs` | 5 | Uses `Model` + `ManifoldState` |
| `spectral-db/src/manifold_store.rs` | 12 | Uses `ManifoldState` + `manifold::{…}` |
| `spectral-db/src/crystallize.rs` | 1 | Uses `manifold::manifold_identity` |
| `spectral-db/src/index.rs` | 1 | Uses `manifold::manifold_identity` |
| `spectral-db/src/lib.rs` | 1 | Uses `manifold::manifold_identity` |
| `spectral-db/benches/spectral.rs` | 3 | Uses `Fate::untrained` |
| `spectral-db/tests/distributed.rs` | 2 | Uses `Fate::untrained` |

**Cosmos + mirror-new WON'T COMPILE regardless of migration** — they call
`fate::Model::Pathfinder` which fate deleted when it renamed to `Introject`.
Their current `fate = { path = "../fate" }` is already broken. The migration
does not make them any more broken. Verdict: leave them alone; kept-working
via shim (compilability at import boundary; internal `Pathfinder` breakage is
pre-existing).

### 2.5 Docs / markdown references to `fate::` (mirror only)

40 doc files reference `fate::` in comments, spec text, or code fences. These
are prose citations and do NOT need programmatic rewrite. Reed can grep +
optionally batch-update to `prismqueer::fate::` if Alex wants doc-level
substrate coherence, but it's cosmetic (Q6 discusses).

---

## §3 — Import-path rewrite map (LIVE consumers)

Two rewrite strategies. Both work; **Strategy A is preferred** for minimal
diff.

### Strategy A — `use prismqueer::fate;` alias (preserve `fate::` call sites)

Add ONE line near existing `use prismqueer::…` in each LIVE consumer file.
Zero-touch all 66 inline `fate::X` call sites.

| File | Edit |
|------|------|
| `mirror/bootstrap/src/lib.rs:497` | Add `use prismqueer::fate;` after existing `use prismqueer::{Optic, Ref};` |
| `mirror/bootstrap/src/contribute.rs:25` | Add `use prismqueer::fate;` (no existing prismqueer import — needs new line) |
| `spectral/src/sel/fate_actor.rs:5` | Change `use fate::{Fate, FateOutput, Features};` → `use prismqueer::fate::{self, Fate, FateOutput, Features};` |
| `spectral/src/sel/fate_actor.rs:101` | Change `use fate::FEATURE_DIM;` → `use prismqueer::fate::FEATURE_DIM;` |
| `spectral/src/sel/mcp/server.rs` | Add `use prismqueer::fate;` at top of use-block (no existing `use fate::…`) |

**Total = 5 file edits, ~5 line changes.**

### Strategy B — Global rewrite `fate::X` → `prismqueer::fate::X`

- `mirror/bootstrap/src/lib.rs`: 31 rewrites
- `mirror/bootstrap/src/contribute.rs`: 14 rewrites
- `spectral/src/sel/fate_actor.rs`: 3 rewrites
- `spectral/src/sel/mcp/server.rs`: 18 rewrites

**Total = 66 line edits across 4 files.**

Strategy A wins on diff volume; Strategy B is more explicit at each call site.
**Reed lean: Strategy A** — cleaner blame, cleaner rebase, and it matches the
existing pattern where fate itself already renames via `prism = { package =
"prismqueer" }` (fate's own Cargo.toml line 14).

### 3.1 Comment/docstring rewrites (optional)

If Alex wants doc-coherence: 40 mirror docs contain `fate::` prose references
(Table §2.3 comment-only hits + §2.5 counts). None break the build. Reed can
do a follow-up `sed` pass; not blocking.

---

## §4 — Cargo feature merge map

### 4.1 Fate features → prismqueer features (post-merge)

| Fate feature (current) | Prismqueer feature (new) | Composition |
|------------------------|--------------------------|-------------|
| `default = []` | `default = []` (unchanged) | fate gated OFF by default |
| n/a | `fate = ["bundle"]` | NEW — gates `pub mod fate;` (needs `bundle` since fate uses `prism::Fiber/Connection/Gauge/Transport/Closure`) |
| `training = ["dep:serde", "dep:serde_json"]` | `fate-training = ["fate", "dep:serde", "dep:serde_json"]` | Composes with prismqueer's `pq` if both are on (both pull the same serde deps — Cargo handles the dedup) |
| `lapack = ["prism/lapack"]` | absorbed by prismqueer's existing `lapack` | fate's `#[cfg(feature = "lapack")]` blocks compile only if `lapack` is on; no new feature name needed |
| `metal = ["dep:metal", "dep:objc"]` | `fate-metal = ["fate", "dep:metal", "dep:objc"]` | New deps `metal 0.29` + `objc 0.2` add to prismqueer's `[dependencies]` as `optional = true` |
| n/a | `fate-brainfuck = ["fate"]` | NEW — gates the build.rs BF compilation pass. Default ON when `fate` on (since `fate/src/compiled.rs` unconditionally includes `bf_compiled.rs`). Alex can turn it off explicitly. |

### 4.2 Prismqueer's post-merge features (final shape)

```toml
[features]
default = []
optics = []
bundle = ["optics"]
lambda = []
lapack = ["dep:cc"]
pq = ["dep:serde", "dep:serde_json"]
fate = ["bundle"]                                    # NEW — enables pub mod fate;
fate-brainfuck = ["fate"]                            # NEW — enables build.rs BF compilation
fate-training = ["fate", "dep:serde", "dep:serde_json"]  # NEW
fate-metal = ["fate", "dep:metal", "dep:objc"]       # NEW
```

**Feature conflict analysis:** none. Serde/serde_json are already optional
deps in prismqueer (via `pq`); `fate-training` shares them via `dep:` refs
which Cargo unifies. Metal + objc are brand new; add to
`[dependencies.metal] optional = true` + `[dependencies.objc] optional = true`
identical to fate's current declarations.

### 4.3 Deferred feature-naming decisions for Alex

The four new feature names — `fate` / `fate-brainfuck` / `fate-training` /
`fate-metal` — all follow the delightfully-boring `<subject>-<qualifier>`
convention. Reed's lean: keep as proposed. Alternatives Alex may prefer:
`inference-fate` (parallels `substrate-fate` / `witness-fate` if those ever
appear); `fate/inference` sub-features `fate/train` `fate/metal` `fate/bf`
(uses the Cargo namespaced-feature syntax available since Cargo 1.60). Q2 in
§8 surfaces this.

---

## §5 — build.rs handling (Reed's A/B/C, resolved)

### (A) Move to top-level `prismqueer/build.rs`

Prismqueer already has a `build.rs` (73 LOC, `prismqueer/build.rs`) that
handles the LAPACK/Fortran build under `#[cfg(feature = "lapack")]`. Option A
appends fate's BF-compilation logic (330+ LOC) into that file, gated by
`#[cfg(feature = "fate-brainfuck")]`.

- **Pro:** one file, simplest topology.
- **Con:** top-level `build.rs` becomes 400+ LOC covering two orthogonal
  concerns (Fortran deps + BF codegen). Higher cognitive load. Two future
  edits will conflict more often.

### (B) Split into `prismqueer/build_scripts/fate.rs` — **Reed's lean**

Top-level `build.rs` stays lean; dispatches to sub-files by feature:

```rust
// prismqueer/build.rs (post-merge)
fn main() {
    #[cfg(feature = "lapack")]
    build_scripts::fortran::build();

    #[cfg(feature = "fate-brainfuck")]
    build_scripts::fate::build();
}

mod build_scripts {
    #[cfg(feature = "lapack")]
    pub mod fortran;
    #[cfg(feature = "fate-brainfuck")]
    pub mod fate;
}
```

Fate's `build.rs` moves verbatim to `prismqueer/build_scripts/fate.rs` with
one edit: change `let bf_dir = Path::new("brainfuck");` →
`let bf_dir = Path::new("src/fate/brainfuck");` (or wherever brainfuck/
lands per Q4).

- **Pro:** separation of concerns. Fortran-lapack build and BF codegen live in
  their own files; the top-level `build.rs` becomes a 15-line dispatcher.
- **Pro:** future BF/Metal codegen changes touch one file that already carries
  the full context.
- **Con:** slightly more files in the crate. Non-issue.

### (C) Drop the Brainfuck-compiled path

**REJECTED.** `fate/src/compiled.rs:3` includes `bf_compiled.rs`
unconditionally; `fate/src/metal_runtime.rs:13` includes `fate.metal`
conditionally. Both are live-consumed at compile-time in fate's own runtime.
Dropping the BF compiler orphans `CompiledFateRuntime` and `MetalRuntime`
— both are load-bearing (benchmarks compare BF-interpreted vs BF-compiled;
Metal is the GPU acceleration path Alex has invested in).

If the reader believes BF is dead code: run `grep -r
CompiledFateRuntime` and `grep -r MetalRuntime` first — both appear in
`fate/benches/tournament.rs` (LOC 105–147 use both) and MetalRuntime is
gated behind `#[cfg(feature = "metal")]` throughout `src/metal_runtime.rs`.

**Reed lean: (B).**

---

## §6 — Cargo.lock impact + workspace membership

### 6.1 Current state

- **Prism workspace:** `/Users/alexwolf/dev/projects/prism/Cargo.toml`
  declares `members = ["imperfect", "prismqueer", "projections"]`. One
  workspace `Cargo.lock` at `prism/Cargo.lock` (33KB).
- **Fate:** standalone crate; own `Cargo.lock` (12KB) at `fate/Cargo.lock`.
- **Consumers:** each has its own lock file (bootstrap: 21KB, spectral: 105KB,
  spectral-db: 45KB, cosmos: 64KB, fragmentation: 48KB).

### 6.2 Post-merge state

- `fate/Cargo.lock` **deleted** (fate is no longer a standalone crate).
- `prism/Cargo.lock` regenerates automatically on next `cargo build` in the
  workspace; adds metal 0.29, objc 0.2, and possibly serde_json if fate-training
  is enabled. Estimated +150 lines to prism/Cargo.lock (transitive tree of
  metal/objc, cocoa-foundation, core-graphics, etc.).
- `mirror/bootstrap/Cargo.lock` regenerates on next build; edge changes from
  `fate = { path = "../../fate" }` → `prismqueer = { features = ["fate"] }`.
  Net: -N lines (fate crate entry gone) + M lines (prismqueer optional deps).
  Roughly delta-neutral.
- `spectral/Cargo.lock` — same delta pattern under `sel` feature.
- `spectral-db/Cargo.lock`, `cosmos/Cargo.lock`, `mirror-new/Cargo.lock` —
  **UNCHANGED** because the fate compatibility shim at `fate/Cargo.toml`
  keeps them pinned to the same crate name + path.

### 6.3 Workspace membership decision

**Fate should NOT become a separate workspace member of prism.** The pull-in
directive is `fate → prismqueer::fate`, i.e., `pub mod fate;` inside prismqueer
— not `prism/fate/` as a sibling crate to prismqueer/imperfect/projections.
Alex's phrasing "pull fate into prismqueer::fate" is unambiguous about this.

Consequence: fate's `src/` files land at `prism/prismqueer/src/fate/*.rs` (or
`prism/prismqueer/src/fate.rs` with a `mod fate;` splitting to a sibling
directory). See §11 Reed execution recipe.

---

## §7 — Test surface breakage

### 7.1 Fate's own tests (move + rename)

| Current path | Post-merge path | Rewrite |
|--------------|-----------------|---------|
| `fate/src/lib.rs` `#[cfg(test)] mod tests` (600 LOC) | `prism/prismqueer/src/fate/mod.rs` (or `fate.rs`) — inline tests move with the source | `use super::*;` unchanged; internal fq paths like `prism::…` change to `crate::…` |
| `fate/src/derive.rs` inline tests | `prism/prismqueer/src/fate/derive.rs` inline tests | Same as above |
| `fate/tests/pipeline.rs` (integration) | `prism/prismqueer/tests/fate_pipeline.rs` | `use fate::…` → `use prismqueer::fate::…` throughout (Line 6–8: `use fate::runtime::UniversalRuntime; use fate::train::{self, PipelineConfig}; use fate::FEATURE_DIM;`); path arg `"training/examples.json"` → `"prismqueer/tests/fate_training/examples.json"` (per Q5) |
| `fate/tests/pipeline_integration.rs` | `prism/prismqueer/tests/fate_integration.rs` | Line 6: `use fate::{feature, manifold, Fate, ManifoldLoss, ManifoldState, FEATURE_DIM};` → `use prismqueer::fate::{feature, manifold, Fate, ManifoldLoss, ManifoldState, FEATURE_DIM};` |
| `fate/benches/tournament.rs` | `prism/prismqueer/benches/fate_tournament.rs` | Lines 11–12: `use fate::runtime::{CompiledFateRuntime, FateRuntime, UniversalRuntime}; use fate::{Fate, Features, Model, FEATURE_DIM};` → same rewrites. Move `[[bench]] name = "fate_tournament"` to prismqueer's Cargo.toml. |

### 7.2 Fate's `#[cfg(feature = "training")]` gate

The `pipeline.rs` test file uses `#![cfg(feature = "training")]`. Post-merge:
change to `#![cfg(feature = "fate-training")]`.

### 7.3 Consumer test breakage

- `mirror/bootstrap/tests/*` — 10 shard tests reference `fate::` in *string
  content* (test envelope names, JSON payloads, error messages) but no Rust
  imports. **Zero rewrite needed.** (Verified: grep for `use fate` / `use
  ::fate` in bootstrap/tests returned zero hits.)
- `spectral/src/sel/fate_actor.rs` — the `#[cfg(test)]` block at LOC 100+
  uses `use fate::FEATURE_DIM;` → change to `use prismqueer::fate::FEATURE_DIM;`
  (or update to Strategy A alias per §3.1).
- `spectral-db/tests/distributed.rs`, `spectral-db/benches/spectral.rs`
  — stay compiling via the shim (untouched).

### 7.4 Divan bench harness

`fate/Cargo.toml` declares `divan = "0.1"` in `[dev-dependencies]`. Prismqueer
uses `criterion` for its bench harness. Post-merge, prismqueer's Cargo.toml
adds `divan = "0.1"` to `[dev-dependencies]` and gets a new
`[[bench]] name = "fate_tournament" harness = false`. No conflict with the
existing `[[bench]] name = "beam_pipeline"`.

---

## §8 — Alex-adjudication surface

Eight decisions surface. Rationale + Reed lean for each.

### Q1 — build.rs handling: (A) monolithic / (B) split-file / (C) drop-BF

**Reed lean: (B).** (A) inflates the top-level to 400+ LOC covering unrelated
concerns; (C) breaks `CompiledFateRuntime` and `MetalRuntime`. See §5 for
full analysis.

### Q2 — Feature naming: `fate` bare vs `fate-inference` vs `inference-fate`

**Reed lean: `fate` bare.** Matches prismqueer's existing feature namespace
(`bundle`, `optics`, `lambda`, `pq`) — all bare names of the module they gate.
Sub-features `fate-training` / `fate-metal` / `fate-brainfuck` follow the
`<subject>-<qualifier>` pattern already normal in Rust (`tokio-macros`,
`serde-json`).

### Q3 — Should `fate` be a default feature of `prismqueer/bundle`?

**Reed lean: NO.** `bundle` implies optics + the spectral-triple algebra
(prism trait + Fiber/Gauge/Transport/Closure); pulling in fate would mean
every `bundle`-consumer downloads brainfuck compilation into their build tree.
Keep `fate` explicit. Consumers who want it opt in: `features = ["fate"]` or
`features = ["fate-metal"]`.

### Q4 — Where does `fate/brainfuck/` land inside prismqueer?

Two options:
- **(a) `prismqueer/brainfuck/`** — flat, matches fate's current layout.
- **(b) `prismqueer/src/fate/brainfuck/`** — scoped, keeps fate self-contained
  as a subtree.

**Reed lean: (b).** Cleaner ownership: everything fate-related lives under
`src/fate/`. Requires the `build_scripts/fate.rs` to `Path::new("src/fate/brainfuck")`.
Also matches the intuition that when Alex later says "grep everything about
fate" the answer is `src/fate/**`.

### Q5 — What happens to `fate/training/examples.json`?

Three options:
- Move to `prismqueer/tests/fate_training/examples.json` and update the
  training test's `load_examples("training/examples.json")` path.
- Move to `prismqueer/src/fate/training/examples.json` and rewrite path.
- **Drop it entirely** — the training pipeline is designated experimental
  (`#[cfg(feature = "training")]`), the 10 examples are hand-labeled and
  hand-editable elsewhere.

**Reed lean: move to `prismqueer/tests/fate_training/examples.json`.** The
pipeline test at `fate/tests/pipeline.rs:12` is real (`train::load_examples`)
and Reed doesn't have authority to declare it dead. Alex can decide to drop
in a later tick if the training feature never gets active use.

### Q6 — What happens to `fate/docs/superpowers/plans/2026-04-05-training-pipeline.md`?

22.8KB historical plan doc. Options:
- Move to `prismqueer/docs/fate/training-pipeline-plan.md`.
- Move to `mirror/docs/plans/2026-04-05-fate-training-pipeline.md` (mirror
  is where docs currently live).
- Drop.

**Reed lean: move to `mirror/docs/plans/2026-04-05-fate-training-pipeline.md`
+ prepend header noting date/source.** Prismqueer docs at `prism/docs/` are
sparse and library-focused; mirror's docs already contain the arc history
that references this pipeline. Keeping it in mirror maintains a single-source
narrative timeline.

### Q7 — Compatibility shim for stale consumers?

**Reed lean: YES.** Keep `/Users/alexwolf/dev/projects/fate/` as a stub crate
that re-exports `prismqueer::fate::*`. This lets spectral-db (potentially
alive), cosmos (dead), and mirror-new (dead) keep building. Rough shim:

```toml
# fate/Cargo.toml (post-merge — 12 lines)
[package]
name = "fate"
version = "0.2.0"
edition = "2021"
description = "Compatibility shim — fate has moved to prismqueer::fate. Update your Cargo.toml."

[dependencies]
prismqueer = { path = "../prism/prismqueer", features = ["fate", "fate-brainfuck"] }

# Consumers wanting metal or training add their own feature flag.
[features]
default = ["prismqueer/fate-brainfuck"]
training = ["prismqueer/fate-training"]
metal = ["prismqueer/fate-metal"]
lapack = ["prismqueer/lapack"]
```

```rust
// fate/src/lib.rs (post-merge — 3 lines)
//! Compatibility shim — fate has moved to prismqueer::fate.
//! See https://... for migration guide.
pub use prismqueer::fate::*;
pub use prismqueer::fate;  // for `fate::Module::…` FQ paths
```

Delete `fate/build.rs`, `fate/src/{lib,compiled,derive,feature,manifold,metal_runtime,runtime,strategy,train,weights}.rs`,
`fate/brainfuck/`, `fate/training/`, `fate/tests/`, `fate/benches/`,
`fate/docs/`. Keep only shim `Cargo.toml` + `src/lib.rs`.

The **hard-cut alternative** (delete fate/ entirely, force all consumers to
update Cargo.toml) is cleaner but breaks 3 downstream projects with no benefit
— none are on the active arc. Shim is 15 lines and shields Alex from having
to touch stale repos.

### Q8 — One big commit vs staged rollout

Two axes: source-move vs consumer-update.

- **Big commit** (Reed's lean): one commit in the meta-repo (or coordinated
  fate/ + prism/ + mirror/ + spectral/ commits at the same UTC minute) that
  (a) copies fate/src/* into prism/prismqueer/src/fate/*, (b) rewrites
  prismqueer/Cargo.toml + build.rs, (c) shims fate/ to re-export, (d)
  updates mirror/bootstrap/Cargo.toml + spectral/Cargo.toml + adds the
  Strategy-A `use prismqueer::fate;` lines. Downstream builds test green
  in the same tick.
- **Staged**: (1) land the source-move + shim first (consumers keep working
  because shim re-exports); (2) later ticks update each consumer's Cargo.toml
  from `fate = { path = "../fate" }` → `prismqueer = { features = ["fate"] }`.

**Reed lean: big commit for the LIVE consumers (bootstrap + spectral); shim
handles the stale ones indefinitely.** The whole point of the shim is that
staged is unnecessary for the dead consumers. For the live ones, batching
the Cargo.toml edge swap in the same commit as the source move keeps
`Cargo.lock` sane — otherwise Cargo.lock would carry two entries (fate + the
new prismqueer-with-fate) for one build.

---

## §9 — Kagi (skipped)

Local grep + substrate context is sufficient. The Cargo mechanics here
(feature-gated `pub mod`, split `build_scripts/`, sub-feature composition)
are standard idioms — no external best-practices needed. If Alex specifically
wants a workspace-refactor sanity check, Reed can pull `cargo-workspaces`
docs in a follow-up tick.

---

## §10 — Substrate-honest verdict + Reed execution recipe scope

- **Total files moving into prismqueer:** 10 source (`.rs`) + 1 build script
  + 3 `brainfuck/*` + 1 `training/examples.json` + 3 tests/benches = **18 files**.
- **Total files staying at `fate/`:** 2 (shimmed `Cargo.toml` + `src/lib.rs`).
- **Total files deleted from `fate/`:** ~24 (all `src/*.rs` except the shim,
  `build.rs`, `brainfuck/`, `training/`, `tests/`, `benches/`,
  `docs/superpowers/plans/`).
- **Total files edited in consumers (LIVE):** 5 (`bootstrap/Cargo.toml` +
  `bootstrap/src/lib.rs` + `bootstrap/src/contribute.rs` +
  `spectral/Cargo.toml` + `spectral/src/sel/fate_actor.rs` +
  `spectral/src/sel/mcp/server.rs`) = **6 files**.
- **Total files edited in consumers (STALE):** 0 (via shim).
- **Total Cargo.toml surgery:** 5 (`prism/prismqueer/Cargo.toml` +
  `mirror/bootstrap/Cargo.toml` + `spectral/Cargo.toml` + `fate/Cargo.toml`
  shim + no changes to `prism/Cargo.toml` because prism workspace already
  lists prismqueer as a member).
- **Estimated single-commit LOC delta:** +2,700 (source moves) / -0 (shim
  keeps API alive) / +50 Cargo.toml edits / +5–66 rewrite edits (Strategy A
  or B) = **~+2,760 lines net gross**, most of which is verbatim file moves
  (git handles as renames if no mid-file edits).
- **Reed can execute in one commit** with Strategy A + Option B build.rs + Q7
  shim.
- **Mara mint needed?** NO — mechanical refactor, no substrate species mint,
  no canonical-spec surface change. Fate's `pub` API is preserved byte-identical.
- **Substrate discipline check:** the pull-in is substrate-honest by
  construction. Fate already carries the `prism = { package = "prismqueer" }`
  dep; consolidating into `prismqueer::fate` collapses two crates into one
  without changing the algebra. No family-root additions. No new species.
  The word `fate` was already in prismqueer's docstring (`Depends on
  Prism. Implements Prism (focus | project | settle)` — `fate/src/lib.rs:20`);
  moving it makes the module-boundary honest with what the substrate already
  said.

---

## §11 — Reed execution recipe (ordered)

**Prerequisite:** Alex ratifies §12 Q1–Q8 (or overrides Reed leans).

### Phase 1 — Source move (prism repo)

1. `mkdir -p prism/prismqueer/src/fate/brainfuck prism/prismqueer/src/fate/build_scripts prism/prismqueer/tests/fate_training`
2. `git mv fate/src/lib.rs prism/prismqueer/src/fate/mod.rs`
3. `git mv fate/src/{compiled,derive,feature,manifold,metal_runtime,runtime,strategy,train,weights}.rs prism/prismqueer/src/fate/`
4. `git mv fate/brainfuck/*.bf fate/brainfuck/*.mirror-bf prism/prismqueer/src/fate/brainfuck/`
5. `git mv fate/build.rs prism/prismqueer/build_scripts/fate.rs`
6. `git mv fate/training/examples.json prism/prismqueer/tests/fate_training/`
7. `git mv fate/tests/pipeline.rs prism/prismqueer/tests/fate_pipeline.rs`
8. `git mv fate/tests/pipeline_integration.rs prism/prismqueer/tests/fate_integration.rs`
9. `git mv fate/benches/tournament.rs prism/prismqueer/benches/fate_tournament.rs`
10. `git mv fate/docs/superpowers/plans/2026-04-05-training-pipeline.md mirror/docs/plans/2026-04-05-fate-training-pipeline.md` (cross-repo — actually a delete-in-fate + write-in-mirror)

### Phase 2 — Prismqueer Cargo + top-level integration

11. Edit `prism/prismqueer/Cargo.toml`: add `fate` / `fate-brainfuck` /
    `fate-training` / `fate-metal` to `[features]`; add optional deps
    `metal = "0.29"` + `objc = "0.2"` + move `divan = "0.1"` to
    `[dev-dependencies]`; add `[[bench]] name = "fate_tournament" harness = false`.
12. Rewrite `prism/prismqueer/build.rs` per §5 (B):
    ```rust
    fn main() {
        #[cfg(feature = "lapack")]
        build_fortran();
        #[cfg(feature = "fate-brainfuck")]
        build_scripts::fate::build();
    }
    #[cfg(feature = "fate-brainfuck")]
    #[path = "build_scripts/fate.rs"]
    mod build_fate;
    ```
13. Edit copied `prism/prismqueer/build_scripts/fate.rs`: change
    `Path::new("brainfuck")` → `Path::new("src/fate/brainfuck")`. Wrap `fn main()` into `pub fn build()`.
14. Edit `prism/prismqueer/src/lib.rs` (after `pub mod bundle;`):
    ```rust
    #[cfg(feature = "fate")]
    pub mod fate;
    ```
15. Edit `prism/prismqueer/src/fate/mod.rs` (was `fate/src/lib.rs`):
    change `use prism::{Beam, Loss as _, Optic, Prism as PrismTrait};` →
    `use crate::{Beam, Loss as _, Optic, Prism as PrismTrait};` (14 hits
    of `use prism::` at line 23 + all `prism::…` FQ refs → `crate::…`).
    Also gate `pub mod compiled;` behind `#[cfg(feature = "fate-brainfuck")]`
    and `pub mod metal_runtime;` behind `#[cfg(feature = "fate-metal")]`
    (matching current `#[cfg(feature = "metal")]` at LOC 30).
16. Edit `prism/prismqueer/src/fate/*.rs`: rewrite `use crate::…` →
    `use super::…` (they're now sub-modules of `fate` module, not top of
    crate). E.g. `derive.rs:12` `use crate::feature::…` → `use super::feature::…`.
17. Edit `prism/prismqueer/src/fate/train.rs:4`: `use crate::…` →
    `use super::…` (weights + FEATURE_DIM).
18. Edit `prism/prismqueer/src/fate/metal_runtime.rs:9`: `use crate::{Features, Model};`
    → `use super::{Features, Model};`.
19. Edit `prism/prismqueer/tests/fate_pipeline.rs`, `fate_integration.rs`,
    and `prism/prismqueer/benches/fate_tournament.rs`: rewrite `use fate::…` →
    `use prismqueer::fate::…`. Update `pipeline.rs` cfg
    `#![cfg(feature = "training")]` → `#![cfg(feature = "fate-training")]`.
20. `cd prism && cargo build -p prismqueer --features fate,fate-brainfuck` (should compile with brainfuck codegen).
21. `cd prism && cargo test -p prismqueer --features fate,fate-brainfuck,fate-training` (should pass all fate tests).

### Phase 3 — Shim `fate/` crate

22. `rm -rf fate/{brainfuck,training,tests,benches,docs,src,build.rs,Cargo.lock,target}`
    (deletes everything except `Cargo.toml` — which gets rewritten).
23. Write `fate/Cargo.toml` per §8 Q7.
24. Write `fate/src/lib.rs` (3 lines) per §8 Q7.
25. `cd fate && cargo build` (should compile as a thin re-export).

### Phase 4 — LIVE consumer updates

26. Edit `mirror/bootstrap/Cargo.toml`: replace `[dependencies.fate] path = "../../fate"` block with a single line addition to the existing prismqueer entry: `prismqueer = { version = "0.1", features = ["bundle", "lapack", "fate", "fate-brainfuck"] }`.
27. Edit `mirror/bootstrap/src/lib.rs:497`: add `use prismqueer::fate;` (Strategy A alias).
28. Edit `mirror/bootstrap/src/contribute.rs:25`: add `use prismqueer::fate;`.
29. Edit `spectral/Cargo.toml`: replace `fate = { path = "../fate", optional = true }` with `prismqueer = { path = "../prism/prismqueer", features = ["fate", "fate-brainfuck"], optional = true }` (or extend an existing prismqueer entry if `sel` gates one). NOTE: spectral currently only declares `prism-core` (a different crate — the pre-rename name); reconcile per Alex.
30. Edit `spectral/src/sel/fate_actor.rs:5`: change `use fate::{Fate, FateOutput, Features};` → `use prismqueer::fate::{self, Fate, FateOutput, Features};`.
31. Edit `spectral/src/sel/fate_actor.rs:101`: change `use fate::FEATURE_DIM;` → `use prismqueer::fate::FEATURE_DIM;`.
32. Edit `spectral/src/sel/mcp/server.rs`: add `use prismqueer::fate;` in the use-block.
33. `cd mirror && cargo build -p mirror --manifest-path bootstrap/Cargo.toml` (should compile).
34. `cd mirror && cargo test -p mirror --manifest-path bootstrap/Cargo.toml` (should pass).
35. `cd spectral && cargo build --features sel` (should compile).

### Phase 5 — Verify STALE consumers still compile via shim

36. `cd spectral-db && cargo build` (should compile via `fate = { path = "../fate" }` shim). If it fails, note the failure but don't fix — it's Alex's call whether to update or archive.
37. `cd cosmos && cargo build` — expected to fail on `Pathfinder` variant (pre-existing, not migration-caused).
38. `cd mirror-new && cargo build` — expected to fail on `Pathfinder` variant (pre-existing).

### Phase 6 — Commit

39. `cd prism && git add -A && git commit -m "🌊 Reed [fate-into-prismqueer] 2026-07-18 …"`
40. `cd fate && git add -A && git commit -m "🌊 Reed [fate-shim] 2026-07-18 …"`
41. `cd mirror && git add -A && git commit -m "🌊 Reed [bootstrap-fate-via-prismqueer] 2026-07-18 …"`
42. `cd spectral && git add -A && git commit -m "🌊 Reed [sel-fate-via-prismqueer] 2026-07-18 …"`

Four commits across four repos, all in the same UTC minute per Alex's typical
cross-repo bundling.

---

## §12 — Alex-adjudication Q's (tightened for pre-execution ratification)

Please answer YES/NO/OVERRIDE (with alternative) for each. Reed will execute
Phase 1–6 upon receipt of Q1–Q8 answers.

- **Q1 build.rs handling:** (B) split into `prismqueer/build_scripts/fate.rs`,
  gated by `fate-brainfuck` sub-feature. Alternative (A) or (C) per §5. **Reed lean: B.**
- **Q2 Feature naming:** bare `fate` + `fate-training` / `fate-metal` /
  `fate-brainfuck` sub-features. Alternative: `inference-fate` /
  `fate/train` (namespaced-features). **Reed lean: bare `fate`.**
- **Q3 Should `fate` be a default of `bundle`?** NO — explicit opt-in via
  `features = ["fate"]`. **Reed lean: NO.**
- **Q4 `brainfuck/` location:** `prismqueer/src/fate/brainfuck/` (scoped)
  vs `prismqueer/brainfuck/` (flat). **Reed lean: scoped `src/fate/brainfuck/`.**
- **Q5 `training/examples.json` disposition:** move to
  `prismqueer/tests/fate_training/`. **Reed lean: move.**
- **Q6 `training-pipeline.md` disposition:** move to
  `mirror/docs/plans/2026-04-05-fate-training-pipeline.md`. **Reed lean: move to mirror docs.**
- **Q7 Shim `fate/` crate?** YES — 15-line stub re-exporting `prismqueer::fate::*`
  keeps stale consumers pinned by path from breaking further. **Reed lean: YES.**
- **Q8 Commit timing:** one big commit per repo, all in the same UTC minute
  (four commits across prism/, fate/, mirror/, spectral/). **Reed lean: bundle.**

### Reed's three hardest Q's for Alex's attention

1. **Q4 (brainfuck location)** — the substrate question is whether `fate` is
   a self-contained subtree under prismqueer or its assets flatten. Scoped is
   Reed's lean; Alex may prefer flat if the intuition is "prismqueer has one
   `brainfuck/` dir, period."
2. **Q7 (shim yes/no)** — if Alex wants a hard-cut and is willing to sunset
   spectral-db / cosmos / mirror-new by declaring them dead, we can delete
   `fate/` entirely and the substrate coheres tighter. Otherwise the shim
   stays.
3. **Q1 (build.rs)** — (B) is Reed's lean but (A) is one fewer file. Alex has
   the aesthetic call on whether `prismqueer/build.rs` grows into a dispatcher
   or stays a lean 20-liner.

---

## Report Footer

- **Scout doc:** `/Users/alexwolf/dev/projects/mirror/docs/scouts/2026-07-18-taut-fate-to-prismqueer-migration-mapping.md`
- **Total files moving:** 18 (10 src + 1 build.rs + 3 brainfuck + 1 examples.json + 3 tests/benches)
- **Total live-consumer edits:** 6 files (2 Cargo.toml + 4 Rust sources)
- **Reed lean for build.rs:** (B) split into `build_scripts/fate.rs`
- **Hardest Q's:** Q4 (brainfuck location), Q7 (shim yes/no), Q1 (build.rs A/B/C)
- **Surprise:** fate's build.rs emits both `bf_compiled.rs` AND `fate.metal`
  (MSL kernel for GPU); (C) drop-brainfuck would silently kill Metal
  acceleration.
