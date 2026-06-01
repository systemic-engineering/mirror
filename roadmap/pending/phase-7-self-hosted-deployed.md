# Phase 7 — Self-Hosted + Deployed

**Goal:** mirror compiles mirror. Rust is the runtime substrate. `.mirror` is the source of truth. The system runs at spectral.engineer as a `@spectral/mosaic` of mirror-binary shards.

## Tasks

1. Validate the full self-host: `mirror compile mirror` produces a working mirror binary that compiles itself.
2. Land the `@spectral` namespace per `docs/insights/2026-05-25-spectral-namespace-architecture.md`:
   - `@spectral/mosaic` (open, Apache-2.0): multi-shard BEAM-cluster deployment grammar; compiles to `@code/beam/eaf`.
   - `@spectral/db` (closed, binary-only): the proprietary graph engine. Eigenvalue compute, fragmentation, kintsugi tournament, conductivity tensors. The IP moat.
   - `@spectral/db/{mnesia, sql/postgres, sql/lite}` (open adapters): wrappers between the closed engine and existing storage substrates.
3. Land spectral-db's distribution layer over fragmentation (MNESIA adapter; cross-node replication; conflict-resolution via the kintsugi tournament shape applied to data).
4. Stand up `spectral.engineer`. Production hardware. Load testing. The cloud deployment that makes v1.0 v1.0. The runtime supports autonomous AI-agent responses via webhook-routed `gen_prism.spawn`.
5. Tag `v0.1.0`. Apply the production version number; the v1.0 framing carries over into post-release work.

## Dependencies

Phases 1–6.
