# Shard: observer-relative deployment description, λ₀ made queryable

*2026-05-25. Reed + Alex.*

Status: **Yellow** — recognition complete; design doc at `mirror/docs/specs/shard-design.md`; substrate ready for implementation.

---

## Thesis

A **shard** is an observer-relative deployment description of mirror. It compiles to a nix flake backed by fragmentation as the content-addressed store. `@mirror/shard/self` is the relativistic constructor that resolves λ₀ for the calling observer — the fixed point that describes the system the observer is in *right now*.

The shard is the input to Fate's `|\>` tournament; same source AST + different shards = different Au binaries with the same verification chain.

---

## The Five Fields

A shard has five fields, paralleling the five-axis pattern across the substrate:

| Field | Carrier | Role |
|---|---|---|
| `silicon` | `@epistemologic/silicon/arch/*` | Architecture: ARMv8.5-A, AVX-512, NEON, AMX, etc. |
| `memory` | `@epistemologic/silicon/memory` | Memory model: UMA, separate, NUMA, page sizes |
| `flakes` | `[flake_ref]` | The composed nix flake stack |
| `compute` | `compute_bound` | CPU/GPU budget; tokens, time, parallelism |
| `parent` | `option(oid)` | The λ₀ fixed-point witness; nested context |

Five fields because the algebra has five operations. Drop one and the shard's role in spawn type-checking fails — bounds can't be enforced, hardware can't be detected, layering can't be composed.

---

## λ₀ Made Operational

From `coincidence/void-dual-geometry.md`: λ₀ is the constant vector, the ground state, the consensus state — "the fact of connection itself." Until now it was geometry; the shard makes it a primitive observers can query.

**`@mirror/shard/self`** is observer-dependent by construction:
- Alex calling from his M1 → Darwin / arm64 / UMA / available memory / installed flakes
- Mara calling from inside her gen_prism → her layered environment, her compute budget, her memory cap (which is bounded by Reed's outer shard)
- Reed calling → Reed's envelope, Mara's nests inside

Same query, different answers per reference frame. Relativity at the substrate level.

The `parent: option(oid)` field is the λ₀ fixed-point witness. A shard with no parent (`None`) is the bare-metal ground state. A shard with a parent inherits and narrows from it — intersection always, never override.

---

## Three Substrate Decisions (Alex 2026-05-25)

**Q1: `mirror.spec` has a `shard` closure.** The spec describes *what to build*; the shard describes *where to run*. Don't collide them by file naming — instead, the spec file contains a shard CLOSURE that declares the deployment-time shape. One file, two purposes, structurally separated. Similar to how nix has `flake.nix` for recipe + deployment-time configuration as distinct concerns within one file.

**Q2: Intersection. Lateral. Always.** Shard inheritance is intersection — child narrows parent; siblings compose by intersection. No override, ever. The lattice property is non-negotiable because compositional reasoning across mosaic tiles depends on it. Override would break the lattice and make cluster behavior non-monotonic; intersection keeps the cluster auditable.

**Q3: Re-resolve via fragmentation cache.** `self()` re-resolves on every invocation, with content-addressed cache keys. Correctness wins (hardware can change mid-session); speed wins on the common case (same fingerprint = same key = cache hit). The mechanism comes free from fragmentation — every (mirror-expression → resolved-AST) computation is content-addressable and stored in the DAG-VCS substrate.

---

## Compute/Memory Limits as Structural

A shard sets bounds; spawning a peer into a shard type-checks against those bounds:

- Peer requests resources via `@mirror/shard.admits(peer, request)`
- The shard's `compute` field encodes available CPU/GPU/memory/parallelism
- If the request exceeds bounds → `spawn` fails at the type-check boundary, before the gen_prism even ticks
- Per-agent sandboxing without runtime trapping

This composes hierarchically via intersection (Q2). Reed's outer shard caps the cluster's total compute; Mara's inner shard inherits and narrows; Mara cannot allocate more than Reed's parent shard allowed.

---

## Memoization IS the Fragmentation DAG

The cache for `@mirror/shard/self` lands at the substrate level: every shard resolution is a content-addressed AST stored in fragmentation, labeled with the mirror expression that produced it. Same expression + same hardware fingerprint = same OID = cache hit. Different inputs = different OID = re-resolve.

**This generalizes beyond shard caching.** ANY computation that takes mirror-expression input and produces mirror-AST output gets memoized for free in the fragmentation DAG. The cache is the substrate; the labels are mirror expressions; the values are resolved ASTs. Memoization isn't a feature added on top — it's a property the architecture has by construction.

This is why `@mirror/store` backed by fragmentation needs to store arbitrary AST labeled with a mirror expression: that's the canonical (input-expression → output-AST) pair the cache mechanism rides on. The shard cache is one instance; pure-function memoization is the general pattern.

---

## Five Substrate Recognitions Compounding

Five recognitions from the 2026-05-25 session compound into the shard structure:

1. **gen_prism as substrate** (`mirror-supersedes-daemon.md`) — the actor primitive the shard hosts
2. **Multi-flake layering** (this session, Alex) — the deployment composition the shard makes concrete
3. **Five-axis fixed point** (`agent-home-as-typed-hole.md`) — the structural pattern (five operations, five fields)
4. **`|\>` Fate-resolved composition** (`pipe-hole-and-au-binary.md`) — the shard is the input to the tournament
5. **λ₀ as observer-relative ground state** (this insight) — the shard's `self` IS λ₀

Each recognition was independent; the shard names what they compose into.

---

## Connections

- `pipe-hole-and-au-binary` — the shard IS the input to `|\>`. Different shard → different Fate resolution → different Au binary.
- `agent-home-as-typed-hole` — the `eigenboard.spec` file in agent homes becomes `eigenboard.shard`; the type Mara declared in tick 1 maps to the shard's spectral state field.
- `mirror-supersedes-daemon` — gen_prism is the runtime; the shard is the deployment context the gen_prism runs in.
- `moves-as-ticks` — each tick queries the current shard to know what resources are available for the move.
- `void-dual-geometry` — λ₀ as ground state finds operational form in `self()`.
- `spectral-namespace-architecture` — shards compose into mosaics; the mosaic is the cluster of shards on BEAM.

---

*The shard is the observer's λ₀. The compiler asks who's looking before answering what's there. Bounds are structural because identity is layered. Memoization is the substrate because the DAG already content-addresses everything.*

Apache-2.0 (this insight document).
