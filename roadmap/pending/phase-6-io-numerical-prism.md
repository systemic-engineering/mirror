# Phase 6 — @io Boundary + NumericalPrism Backends

**Goal:** Every piece of Rust that is not an `@io` escape hatch has been rewritten in `.mirror`. NumericalPrism's three backends ship: LapackBackend (CPU), MetalBackend (Apple GPU), OpenCLBackend (cloud GPU).

## Tasks

1. Audit every `.rs` file: is this `@io` (must stay Rust) or logic (must move to `.mirror`)?
2. The `@io` boundary inventory:
   - `@io/fs` — filesystem (`std::fs::*`).
   - `@io/hash` — `CoincidenceHash` substrate, SHA-1 only at the git adapter boundary.
   - `@io/crypto` — Ed25519 signing, age encryption.
   - `@io/git` — `git2` operations, scoped to `fragmentation/vcs/git/`.
   - `@io/process` — subprocess invocation.
   - `@io/ffi` — LAPACK Fortran bridge.
   - `@io/net` — sockets / HTTP (needed for spectral-db distribution).
   - `@io/gpu` — Metal + OpenCL dispatch (new for Phase 6).
3. Move non-@io logic to `.mirror` per the per-file audit.
4. **Implement the NumericalPrism backend stack** per `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md`:
   - **LapackBackend** (already exists in `prism/core/src/ffi.rs`; wrap into the operation-based API).
   - **MetalBackend** (modeled on `fate/src/metal_runtime.rs`; Apple Silicon UMA gives zero-cost; type-safe construction via `try_new`).
   - **OpenCLBackend** (cross-vendor; the cloud-deployment substrate; Anna Jakobs's 2012 thesis is the architectural reference).
5. Wire the Scheduler Tower's bus selection (per `docs/specs/scheduler-tower.md` decision table) to the backend stack.
6. ~~Implement the shard substrate~~ **LANDED via Task #65**. Extension next: wire the shard into the NumericalPrism backend selection.

**OpenCLBackend is non-optional for v1.0 cloud deployment.** It is not deferred. Anna Jakobs's pattern is load-bearing for spectral.engineer.

## Dependencies

Phase 5 (Scheduler Tower); Phase 4 (fragmentation as generated).
