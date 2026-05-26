# Phase 5 — Reflection Model + the Scheduler Tower

**Goal:** The five operations as the compilation loop. Tick/tock convergence. Reflection observes and adjusts. The Scheduler Tower regulates the runtime's temperature.

## Tasks

1. Implement `\` hole dispatch including the `|\>` composition operator (pipe-with-a-hole) — route to `@fate.infer` per the pipe-hole-and-au-binary insight + kintsugi-tournament + heterogeneous-numerical-prism specs. `|\>` produces locally-optimal binaries (Au) anchored to a verified AST; same source OID, divergent binary bytes, unbroken verification chain.
2. Implement the tick loop. Reflection observes; projects; splits; zooms; refracts. Loop until convergence.
3. Implement kintsugi as Reflection. Per `docs/specs/kintsugi-formatter.md` + the discrete-Ricci-flow framing.
4. ~~Write `@peer` grammar for the four persistent models.~~ **Mostly landed** via Tasks #62/#65. Lens-altitude extension queued via the constructivism insight; deferred per LRM.
5. **Implement the Scheduler Tower** per `docs/specs/scheduler-tower.md`. Demand-contract extension to `gen_prism` (`halts` property declared #74; `reduction_budget` primitive declared). Subscription protocol. Dispatcher strategies (round_robin, partitioned, broadcast). Backpressure propagates upstream. Temperature `β` at loop boundaries (per-stage temperatures are incoherent KMS). Runtime dispatch via `@mirror/serve` is the wire-level consumer.
6. Gestalt writes from Reflection only. Enforce at the type level.

## Recent work this absorbs

- The Scheduler Tower spec (just landed).
- `gen_prism.mirror`'s existing actor abstraction (the demand contract extends it; backwards-compatible).
- Tournament merge per `docs/specs/kintsugi-tournament.md`.
- @fate.infer as the single Fate surface (config-shaped, not method-shaped).

## Dependencies

Phases 2–4.
