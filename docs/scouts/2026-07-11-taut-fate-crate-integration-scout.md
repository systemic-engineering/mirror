# Taut scout — @fate crate integration into `cmd_peer_beam` (2026-07-11)

Scout: Taut (grep-first, read-only)
Scope: `/Users/alexwolf/dev/projects/fate/` + `mirror` bootstrap/shards
Trigger: Alex directive 2026-07-11 — "continue the collapse until nothing
empirically stops us from a spawn or an unresolvable ambiguity emerges"

## Executive summary

**LRM: LANDABLE-WITH-PREREQS.** The fate crate is empirically complete —
`Fate::excited()`, `Fate::select(current, features)`, `Fate::tick(features)`
compile today, produce `Decision { model, confidence, distribution }`, and
sit behind stable Rust API in `/Users/alexwolf/dev/projects/fate/src/lib.rs`.
`fate = { path = "../../fate" }` is a **3-line** `bootstrap/Cargo.toml`
addition; `cmd_peer_beam` calling `fate.tick(features).decision.model` is a
**~25-line** insert at `bootstrap/src/lib.rs:4212` (piece_6_fate_inference
slot currently marked `partial@recall`). What blocks a clean single-tick
land is not code — it's **six ambiguities requiring Alex adjudication**,
all at the `mission-text → Features [f64; 16]` seam. Grounding: Mara
`b0427fd` (@optics/lens family-root), Mara `7e5c298` (@optics/lens/diff),
Reed `be74b6a` (Blocker 2 Rust discharge, `--emit-diff` now emits real
bytes with spec_oid). @fate would replace the `partial@recall` composition-
piece with COMPUTED candidates. **Recommend Alex adjudicates §Q6
ambiguities before Reed lands; the Rust wiring itself is trivial once the
encoding contract is named.**

## §Q1 fate crate state

Evidence (`ls /Users/alexwolf/dev/projects/fate/` + `Cargo.toml`):

- Crate: `fate v0.1.0`, `edition = "2021"`, description: "Abyss |
  Introject | Cartographer | Explorer | Fate. Five models. One selector.
  Zero dependencies."
- Depends on `prism = { package = "prismqueer", path = "../prism/prismqueer" }`
  (the same `prismqueer` bootstrap uses).
- Optional feature `training` (serde + serde_json). Default: **no** train.
- 10 top-level modules: `lib.rs` (52 KB), `derive.rs`, `feature.rs`,
  `manifold.rs`, `runtime.rs` (25 KB), `strategy.rs`, `weights.rs`,
  `train.rs`, `compiled.rs`, `metal_runtime.rs` (feature-gated).
- Tests: `tests/pipeline_integration.rs` (6 tests, all against the public
  `Fate` API); benches: `benches/tournament.rs` (10 divan benches).
- **5 models confirmed** per Reed's memory `project_fate_architecture`:
  `enum Model { Abyss, Introject, Cartographer, Explorer, Fate }` at
  `src/lib.rs:29-40`.

**Weights**: `Fate::untrained()` (all zeros → uniform), `Fate::excited()`
(xorshift64-seeded random weights from system time). **No pre-trained
weight file ships**; `training/examples.json` (10 examples) exists as
scaffold for the training pipeline plan at `docs/superpowers/plans/2026-04-05-training-pipeline.md`.
The BF-backed `UniversalRuntime` (`weights::Weights::default_cycle()`)
compiles a hardcoded cycle-preference weight set.

## §Q2 candidate space

**Consumer-defined at the operation altitude; substrate-defined at the
5-model altitude.** The fate crate's `Fate::select(current: Model,
features: &Features) -> Decision` returns one of the fixed 5 `Model`
variants — that's the substrate-fixed space. The **peer_beam operation
space** (what mirror actually wants to navigate — "should this beam
observe / project / split / shift / settle?") is the 5-op `prism` surface
from `shards/mirror/peer/beam.mirror:270-274`:

```
prism @mirror/peer/beam {
  focus mirror_peer_beam_request
  project mirror_peer_beam_request
  split mirror_peer_beam_request
  shift mirror_peer_beam_request
  settle mirror_peer_beam_request
}
```

The clean semantic map is 1-1: `Abyss ↔ focus`, `Introject ↔ project`,
`Cartographer ↔ split`, `Explorer ↔ shift`, `Fate ↔ settle` — but I did
not find a substrate-decl that ratifies that mapping. `shards/fate.mirror`
declares `prism @fate` (42 KB), and `shards/fate/tournament.mirror` (41
KB) declares tournament arithmetic — neither name the Model-↔-prism-op
correspondence for the peer_beam consumer explicitly. (Ambiguity Q6.a.)

## §Q3 input/output encoding

**Input contract** (`src/lib.rs:47-53` + `src/feature.rs:12-53`):

- `pub const FEATURE_DIM: usize = 16;`
- `pub type Features = [f64; FEATURE_DIM];`
- 6 **active** dims: `TEMPORAL, PROCESSING, STABILITY, NOVELTY, CAUTION,
  COHERENCE` (indices 0..5).
- 10 **dark** dims: `CREATIVITY, CONFIDENCE, FORMALITY, OUTPUT_REGULATION,
  ABSTRACTION, DEFERENCE, CONFIDENCE_CALIBRATION, INNOVATION,
  REASONING_DEPTH, EMOTIONAL_TONE` (indices 6..15).
- No mission-text encoder ships. `feature::casimir(features)` computes the
  Casimir invariant `C₂ = Σ(λᵢ·xᵢ)²` over active dims;
  `feature::CASIMIR_EIGENVALUES` = `[4.12, 3.98, 4.05, 3.91, 4.08, 3.97]`.

**Output** (`src/lib.rs:57-73`):

- `Decision { model: Model, confidence: f64, distribution: [f64; 5] }`
- Also `FateOutput { model, decision, kernel_spec, loss, health }` from
  `Fate::tick(features)` (full pipeline).

**Mission text → Features gap**: no encoder exists in either crate. The
mirror side would need to write one (or land a substrate-decl that names
one). Options: (a) hash-modulo bytes into 16 f64s; (b) route through `@nl`
first; (c) declare a fate-specific tokenizer species. All Q6 material.

## §Q4 benchmark reality

`benches/tournament.rs` is real (10 `#[divan::bench]` functions, all
against the public API). Bench targets: `tournament_single_select`,
`tournament_single_tick`, `bf_compiled_single`, `bf_universal_single`,
`resolve_depth_5`, `resolve_depth_20`, etc. **I did not run cargo bench**
(read-only scout, time budget). The 475ns/inference and 2M inf/sec
numbers in Reed's memory (`project_fate_architecture`) are **not verified
here**; the bench harness EXISTS to produce those numbers on demand. The
`CompiledFateRuntime` (native-Rust generated from `brainfuck/fate.bf` via
`build.rs`) is the empirically-fast path per the module comment "Produces
identical output to FateRuntime, but faster." Empirical citation: Reed's
memory only. Verification would take one `cargo bench -p fate` invocation.

## §Q5 mirror landability

**Estimate: 3 concrete pieces, ~30 LOC + 3 lines Cargo.toml, ONE tick if
Q6 ambiguities are pre-adjudicated; TWO ticks otherwise (one for encoder
substrate-decl, one for the wiring).**

Concrete wiring at `bootstrap/src/lib.rs:4212` (the `cmd_peer_beam`
`piece_6_fate_inference` slot currently at `"partial@recall"`):

```rust
// Add to Cargo.toml:
// fate = { path = "../../fate" }

// In cmd_peer_beam, replacing the piece_6 stub:
let features: fate::Features = encode_mission_to_features(&mission_text);
let fate_instance = fate::Fate::excited();  // OR untrained() — Q6.d
let output = fate_instance.tick(&features);
let selected_op = model_to_prism_op(output.decision.model);  // Q6.a
// Emit into envelope: model=<op>, confidence=<f64>, distribution=[5×f64]
```

Substrate-decl side (per Mara `77fe92d` §2 winding + peer/beam.mirror):
the current shard already declares `in @fate` (line 10), so the
composition binding is already substrate-honest. What's missing is the
species that names the Model↔op mapping (Q6.a) and the mission-encoder
(Q6.b/c).

**Prereqs**: Q6 items 1-4 minimum. Item 5-6 are OK-to-defer.

## §Q6 architectural ambiguities (Alex-adjudication-required)

Each one-liner. Alex, these are the six things Reed cannot decide from the
substrate alone.

- **Q6.a — Model↔prism-op mapping.** Is `Abyss↔focus, Introject↔project,
  Cartographer↔split, Explorer↔shift, Fate↔settle` the canonical binding,
  or does @fate/tournament's cascade shape imply a different assignment?
  No shard names this today.
- **Q6.b — Mission-text encoder home.** Does `mission: String →
  Features [f64;16]` live in @nl, in a new @fate/encode species, or
  in-line in the bootstrap Rust dispatch? Substrate-honest answer names
  the ownership.
- **Q6.c — Active vs dark dimension semantics for missions.** Should
  mission text populate only the 6 active dims (TEMPORAL..COHERENCE) and
  leave the 10 dark dims zero, or does the encoder produce all 16?
- **Q6.d — Untrained vs excited vs default_cycle at v0.** `Fate::excited()`
  produces different candidates every call (non-deterministic);
  `Fate::untrained()` produces uniform (all-equal); `default_cycle()` is a
  hardcoded cycle preference. For the first spawn, which fate-instance is
  the correct starting state?
- **Q6.e — Persistent state accumulator shape.** `@bauchladen.tray`
  accumulates. Does it accumulate (i) raw features per tick, (ii) fate
  decisions per tick, (iii) the resulting diff bytes, or (iv) all three?
  Q6.e drives the storage substrate on top of Reed `be74b6a`'s
  `--emit-diff` bytes.
- **Q6.f — Composition with @cyberpunk/algedonic sampling.** The pain-δ /
  pleasure-δ signal from Mara `966890b` feeds the peer's navigation loop.
  Should algedonic samples inject into the fate `Features` vector (which
  dim?), or ride as a separate `depth` parameter into
  `ModelWeights::forward(features, depth)`? The `depth: f64` axis exists;
  its consumer semantics for peer_beam are unbound.

## Scout closure

- **LRM**: LANDABLE-WITH-PREREQS.
- **Recommended next action for Reed**: PAUSE the wiring landing; escalate
  Q6.a–Q6.d to Alex as a single adjudication packet ("four questions, one
  session"). Q6.a and Q6.b are the load-bearing pair — with those two
  answered, the Rust wiring lands in one tick. Q6.e and Q6.f are follow-on
  ticks and can defer.
- **Cascade grounding cited**: Mara `b0427fd` (@optics/lens), Mara
  `7e5c298` (@optics/lens/diff), Reed `71df5de` + Reed `be74b6a` (Blocker
  2 Rust discharge — spec_oid via `--emit-diff`), `shards/mirror/peer/beam.mirror`
  (peer_beam substrate-decl, declares `in @fate`), `shards/fate.mirror`
  (@fate family-root, bilateral compile-time + runtime), `shards/fate/tournament.mirror`
  (multi-frequency tournament species).

Word count: ~1100 (within 1200 target).
