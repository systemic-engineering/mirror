# spectral-db substrate migration — the open interface, eight phases

*2026-06-12. Mara. Roadmap (substrate-pull plan; open-side only).*

Status: **wip** — the 8-phase shape is content named for the substrate work;
the engine internals stay closed.
Internal task tag: **#114** (the task-list ID system labels this work
"Track D" internally; this filename is content-named per `wip/` convention
and intentionally drops that label).
Reads from: `roadmap/wip/{butterfly-self-hosting,cloud-deployment,v1-launch,phase-0-current-state}.md`,
`roadmap/pending/phase-{1..7}-*.md`, `docs/specs/spectral-db-three-tier-architecture.md`,
`docs/specs/spectral-runtime.md`, `docs/specs/numerical-substrate-via-fortran.md`,
`shards/spectral/*.mirror`.

---

## 0. The closed/open guard (load-bearing — read first)

This doc names the SUBSTRATE work for `@spectral/db`. Two doors; one is
open, one is closed; the discipline that keeps them apart is the
architectural contract of this entire track.

- **`@mirror/store` is OPEN.** Git-backed (`fragmentation` crate);
  Apache-2.0; verification on write; no engine policy. The storage gate.
  Per memory `architecture-mirror-store-vs-spectral-db`: mirror MUST work
  without `@spectral/db`.
- **`@spectral/db` is the CLOSED engine ON TOP.** SEL-licensed; the
  navigation engine; the eigenvalue / Laplacian / settlement geometry.
  Per memory `architecture-splinter-and-spectral-db-edges`: floor data
  type is `Splinter` (content-addressed, K_n via OID-graph, no edges);
  `@spectral/db` builds the edge structure on top — the secret sauce.

This track's substrate work IS THE OPEN INTERFACE: the typed knobs
(`settlement_config`, `crystallize_policy`, `pressure_policy`) live in
mirror as substrate declarations; the engine implementing them stays
closed. Every phase below names work on the open side; the closed side
consumes what mirror declares without contributing back substrate
declarations.

This is also the business-model architecture: an open foundation (anybody
can declare `@spectral/db/*` knobs in their mirror substrate; anybody can
read what those knobs mean) with a closed engine (only the SEL-licensed
implementation can honour those knobs at production scale). Don't
conflate the two; don't paywall the substrate; don't open-source the
engine.

---

## 1. Phase 1 — Extract the kernel

Move the pure-mechanism numerics into `src/kernel/` inside the
spectral-db crate:

- Jacobi eigensystem (the eigendecomposition that drives
  λ₀-distance-as-cost and the Bundle Tower's connection 1-form);
- SHA-256 (content-addressing for the substrate's OID graph; the
  K_n hash that `Splinter` consumes);
- Laplacian construction (sheaf Laplacian `Δ_F` per
  `epistemologic/math/sheaf_laplacian.mirror`; graph Laplacian as
  degenerate sheaf case);
- Eigenvalue distance (the settlement-cost metric);
- Memory-budget cost (the HamiltonScheduler's per-shard cost model
  per memory `architecture-hamilton-scheduler`).

Constraint: pure mechanism, no policy. Existing tests pass. No mirror
substrate touched in this phase; this is closed-side housekeeping that
makes phase 2's open-side declarations cleanly absorbable.

Recognition #43 §12 addendum (Jacobi fixed-point) governs the kernel's
convergence guarantee: the Jacobi sweep is a contraction in the
substrate's eigenvalue ordering; the fixed-point makes phase 6's
self-hosting structurally stable.

---

## 2. Phase 2 — Mint `@spectral/db/*` at substrate altitude

Open-interface substrate declarations. Sub-shards:

- `@spectral/db/node` — typed node carrier (uuid_spectral + payload
  + per-altitude metadata);
- `@spectral/db/edge` — typed edge carrier (source + target + label +
  conductivity weight);
- `@spectral/db/spectral_hash` — the per-node spectral fingerprint
  derived from the local Laplacian's small-λ profile;
- `@spectral/db/graph_hash` — the whole-graph fingerprint from the
  eigenvalue spectrum (the K_n hash combined with the small-λ profile);
- `@spectral/db/crystal` — the crystallized subgraph carrier (sealed
  region; eigenvalue-invariant under further mutation);
- `@spectral/db/pressure` — the pressure-load carrier consumed by the
  Scheduler Tower (per memory `architecture-three-tier-stack`).

These declare the SHAPE the closed engine consumes; the engine's
internal storage layout (cache structure, sharding strategy, GPU
buffer formats) stays closed. Substrate dispatch points to substrate
declarations; the engine reads the substrate to know what's expected
of it.

Cross-reference: Task #268 (Crystal substrate-decl) sits inside this
phase as the first concrete tick.

---

## 3. Phase 3 — Generate Rust types from grammar

`@code/rust` codegen replaces hand-written Rust types in the spectral-db
crate with types generated from the phase-2 grammar. Same shape Phase 4
of the parser-self-hosting track applies to `fragmentation` (per
`roadmap/pending/phase-4-emitter-self.md`); the spectral-db crate's
type surface is downstream of `@spectral/db/*` substrate declarations.

This is the first phase where the closed engine starts reading mirror
substrate as source-of-truth. The engine's Rust source stops being
authoritative for type definitions; the substrate becomes
authoritative; the engine becomes an implementation.

---

## 4. Phase 4 — Policy through grammar evaluation

Move policy from hardcoded engine defaults to substrate-altitude
declarations:

- `settlement_config` — what counts as "settled" (eigenvalue-delta
  threshold, max iterations, convergence shape);
- `crystallize_policy` — when a subgraph seals into a crystal
  (eigenvalue-invariance window, age, query density);
- `pressure_policy` — how pressure load triggers demotion / promotion
  in the Scheduler Tower's flow;
- `optimizer_policy` — when LAPACK / Metal / OpenCL backends are
  selected at the @io seam (per memory
  `architecture-flang-mirror-numerical-split`).

Users override per-instance via grammar. The closed engine reads the
substrate at boot and honours the user's policy.

This phase realises the substrate-pull discipline at the policy
altitude: every "behaviour knob" that was a Rust constant becomes a
substrate-declared carrier with typed defaults.

---

## 5. Phase 5 — Actions through grammar dispatch

The five operations (`focus / project / split / shift / settle`)
become grammar actions on `@spectral/db/*` sub-shards. MCP and CLI
route through the action layer (per memory
`architecture-pq-as-mcp-surface`: 18 tools collapse to 3; stateless
wire; session-implicit shard). The closed engine implements the
action obligations; the substrate names them.

Settlement (the canonical fifth operation) sits at the heart of this
phase: `settle` IS the lattice join per memory
`architecture-shard-ref-as-prism`; the engine's implementation IS the
Jacobi sweep + the kintsugi loop's gradient descent.

The vocabulary cascade (zoom → shift; refract → settle) is HONORED
throughout this doc. The source manuscript was pre-cascade; this
phase's declarations use the post-cascade vocabulary.

---

## 6. Phase 6 — Self-hosting

The `@spectral/db` grammar settles in its own database instance. The
substrate's declarations of `@spectral/db/*` become content-addressed
nodes in a running `@spectral/db` instance; the engine reads its own
substrate from its own storage; the Jacobi fixed-point (recognition
#43 §12 addendum) makes this stable.

Two layers of self-hosting compose at this phase:

1. The mirror compiler self-hosting (per the butterfly track) gives
   mirror-the-language compilation independence from Rust.
2. THIS track's self-hosting gives `@spectral/db` storage
   independence from a separately-bootstrapped storage layer.

Together they close the loop: mirror compiles itself; mirror's
substrate lives in `@spectral/db`; `@spectral/db` reads its own
substrate from itself. The Connes spectral triple (per memory
`architecture-connes-spectral-triple`) runs at its own fixed point.

---

## 7. Phase 7 — flang / LAPACK lift

When Jacobi's O(n³) practical floor becomes the binding cost (graphs
beyond ~10⁵ nodes), swap eigendecomposition to LAPACK `dsyev` via
`@code/fortran` per `docs/specs/numerical-substrate-via-fortran.md`
and memory `architecture-flang-mirror-numerical-split`.

The substrate-altitude story stays the same: `@spectral/db/spectral_hash`
declares WHAT a spectral fingerprint is; the @io backend chooses
HOW to compute it (Jacobi at small scale; LAPACK at large scale;
Metal / OpenCL on GPU partitions per
`roadmap/wip/cloud-deployment.md`'s Anna-Jakobs pattern).

This phase is where the numerical-substrate split (flang for 16×16
weight inference; mirror for 5×5 fiber/eigenvalue scaling per memory
`architecture-flang-mirror-numerical-split`) realises at the
@spectral/db consumer altitude.

---

## 8. Phase 8 — Garden ratification gate

`@spectral/db` is the substrate for `@spectral/garden` (the
vetted-corpus distribution surface; currently a parentless ghost in
the substrate, with no declared family root because the storage
foundation isn't ratified yet).

Garden ratification requires:

- Phases 1-7 above complete;
- Sybil-in-eigenspace defense designed (belongs to the threat model
  spec; surfaced for Tick III; NOT in this doc's scope);
- The corpus signing chain through `@spectral/db/crystal` declared
  and instantiated (each garden-vetted corpus is a crystal whose
  eigenvalue invariance is the verification handle).

This phase is the bridge from infrastructure (this track) to
distribution (the `@spectral/garden` track). The eigenvalue spectrum
of the crystal IS the verification surface — the closed engine
computes it; the substrate names what it means; the garden
distributes the result.

---

## Cross-references

- **Task #114** — the internal Track-D ID for this work.
- **Task #268** — Crystal substrate-decl; the first concrete tick under
  phase 2.
- Memory `architecture-three-tier-stack` —
  fragmentation-mcp / mirror / @spectral/db with SpectralSupervisor;
  this track's phases align to the third tier.
- Memory `architecture-mirror-store-vs-spectral-db` — the open / closed
  partition; the load-bearing distinction phase 0 names.
- Memory `architecture-splinter-and-spectral-db-edges` — Splinter is
  the floor data type; `@spectral/db` builds the edges on top.
- Memory `architecture-flang-mirror-numerical-split` — phase 7's
  numerical backend lift.
- Memory `architecture-hamilton-scheduler` — phase 1's
  memory-budget kernel sits on this discipline.
- Recognition #43 §12 addendum (Jacobi fixed-point) — phases 1 and 6
  cite this convergence guarantee.
- `roadmap/wip/butterfly-self-hosting.md` — the parallel
  self-hosting track at the compiler altitude; phase 6 of THIS
  track depends on it.
- `roadmap/wip/cloud-deployment.md` — Anna Jakobs's OpenCL pattern;
  phase 7's GPU backend lift consumes this.

---

## Vocabulary cascade applied

Source manuscript was pre-cascade. Throughout this doc:

- `zoom` → `shift`;
- `refract` → `settle`.

These are the canonical post-cascade operation names. Any quoted
text from pre-cascade docs that uses `zoom` / `refract` is
translated; future docs should use the post-cascade vocabulary
directly.

---

## What this doc IS and IS NOT

IS: an 8-phase substrate-pull plan for the OPEN interface of
@spectral/db. Names the closed engine's contract surface; declares
nothing about the closed engine's internals.

IS NOT: an implementation plan for the closed engine; a license
declaration (SEL stays where it is at `LICENSE.md`); a threat model
(garden Sybil-in-eigenspace defense → Tick III); a release schedule
(phases compose with butterfly track per recognition cascade
ordering, not calendar).
