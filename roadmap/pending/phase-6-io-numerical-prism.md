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
4. **Implement the NumericalPrism backend stack** per `~/dev/systemic.engineering/practice/insights/coincidence/heterogeneous-numerical-prism.md` AND per `docs/specs/numerical-substrate-via-fortran.md` / `roadmap/wip/butterfly-self-hosting.md` §6 Track A. The backends are **altitudes consumed by the NumericalPrism (formerly LAPACKPrism; renamed under the butterfly recognition)**, not three Rust impl modules. Each backend is an `au(@code/<lang>)` artifact content-addressed in `@mirror/store`; the dispatcher reads the spectral signature of the work and selects the altitude at run time. The Rust ffi (`prism/core/src/ffi.rs`) is the bootstrap-window shim that lets the substrate-pull happen incrementally; it sheds at Phase 6 close once the substrate-resident path is verified.
   - **LapackBackend → `au(@code/fortran)`** via flang. The Fortran Fate package compiles via flang to LLVM IR; the resulting `liblapack.a` is content-addressed in `@mirror/store`. New shards land at Phase 6: `shards/code/fortran.mirror`, `shards/io/flang.mirror`. The Rust `prism/core/src/ffi.rs` wrapper exists transiently for the bootstrap window and retires at Phase 6 close (cut criterion: `lapack-substrate-resident: yes` per butterfly-self-hosting.md §5).
   - **MetalBackend → `au(@code/metal)`** via Apple's metal compiler. Apple-Silicon-bound; not cloud-deployed. New shards at Phase 6 close: `shards/code/metal.mirror`, `shards/io/metal.mirror`. Modeled on `fate/src/metal_runtime.rs` as the prior-art reference; type-safe construction via the substrate's existing altitude-typing discipline.
   - **OpenCLBackend → `au(@code/opencl)`** via the cross-vendor OpenCL compiler. Cloud-deployment non-optional per `roadmap/wip/v1-launch.md`. Anna Jakobs's 2012 thesis is the architectural prior art for the shared-memory pattern.
5. Wire the Scheduler Tower's bus selection (per `docs/specs/scheduler-tower.md` decision table) to the backend stack.
6. ~~Implement the shard substrate~~ **LANDED via Task #65**. Extension next: wire the shard into the NumericalPrism backend selection.

**OpenCLBackend is non-optional for v1.0 cloud deployment.** It is not deferred. Anna Jakobs's pattern is load-bearing for spectral.engineer.

## Dependencies

Phase 5 (Scheduler Tower); Phase 4 (fragmentation as generated).
